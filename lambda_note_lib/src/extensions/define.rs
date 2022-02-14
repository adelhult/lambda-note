use crate::extensions::{Context, Extension};
use crate::translator::OutputFormat;
use crate::Origin;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::time::Duration;
use wait_timeout::ChildExt;
use super::ExtensionVariant;

#[derive(Clone)]
pub struct Define;

impl Extension for Define {
    fn name(&self) -> String {
        "Define".to_string()
    }

    fn description(&self) -> String {
        "Define a new extension".to_string()
    }

    fn version(&self) -> String {
        "1".to_string()
    }

    fn is_safe(&self) -> bool {
        false
    }

    fn call(&self, mut ctx: Context) -> Option<String> {
        if ctx.arguments.get(0).is_none() {
            self.add_error("No name provided as first argument", &mut ctx);
        }

        if ctx.arguments.get(1).is_none() {
            self.add_error(
                "Second argument missing. No way of invoking the extension was provided",
                &mut ctx,
            );
        }

        let name = ctx.arguments.get(0)?.trim().to_string();
        let command = ctx.arguments.get(1)?;
        let timeout = get_timeout(&ctx);

        match get_extension(command, timeout) {
            Ok(extension) => {
                // log all the provided errors and warnings
                for error in &extension.errors {
                    self.add_error(
                        &format!(
                            "{} had the following error when being defined: {}",
                            &name, error
                        ),
                        &mut ctx,
                    );
                }

                for error in &extension.warnings {
                    self.add_warning(
                        &format!(
                            "{} had the following warning when being defined: {}",
                            &name, &error
                        ),
                        &mut ctx,
                    );
                }
                // add the extension to the document
                ctx.document.extensions.insert(name, Rc::new(extension));
            }
            Err(error) => {
                self.add_error(
                    &format!("Failed to add the extension {} due to {}", name, error),
                    &mut ctx,
                );
            }
        }
        None
    }

    fn supports_block(&self) -> bool {
        false
    }

    fn supports_inline(&self) -> bool {
        true
    }

    fn interests(&self) -> Vec<String> {
        vec![String::from("timeout")]
    }
}

/// Get the timeout value from the document metadata
/// or set a predefined default value.
fn get_timeout(ctx: &Context) -> f32 {
    ctx.document
        .metadata
        .get("timeout")
        .map(|x| x.replace("second", "").replace("s", "").parse::<f32>().ok())
        .flatten()
        .unwrap_or(2.0)
}

/// Given a request struct, send a message to a child process and await
/// a response string
fn send<T: Serialize>(command: &str, timeout: f32, req: T) -> Result<String, Error> {
    let shell = if cfg!(target_os = "windows") {
        "cmd"
    } else {
        "sh"
    };
    let shell_arg = if cfg!(target_os = "windows") {
        "/C"
    } else {
        "-c"
    };

    let mut child = Command::new(shell)
        .arg(shell_arg)
        .arg(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let contents = serde_json::to_string(&req).unwrap();

    // write the request object to the childs stdin
    if child
        .stdin
        .as_mut()
        .ok_or(Error::ProcessFailure)?
        .write_all(contents.as_bytes())
        .is_err()
    {
        return Err(Error::ProcessFailure);
    }

    let secs = Duration::from_secs_f32(timeout);

    // kill the child process if we timeout
    if child.wait_timeout(secs).ok().flatten().is_none() {
        child.kill().expect("Failed to kill process");
        return Err(Error::Timeout);
    };

    match child.stdout.take() {
        Some(mut stdout) => {
            let mut result = String::new();
            stdout.read_to_string(&mut result).unwrap();
            Ok(result)
        }
        None => Err(Error::ProcessFailure),
    }
}

/// Get a extension struct given a shell command and timeout
fn get_extension(command: &str, timeout: f32) -> Result<ForeignExtension, Error> {
    let response = send(command, timeout, InfoRequest::new())?;
    serde_json::from_str(&response)
        .map_err(|_| Error::JsonParsingFailure(response))
        .map(|mut extension: ForeignExtension| {
            // set the command field
            extension.command = command.to_string();
            extension
        })
}

#[derive(Debug, PartialEq)]
enum Error {
    JsonParsingFailure(String),
    ProcessFailure,
    Timeout,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Error::JsonParsingFailure(msg) => format!("failing to parse the json response \"{}\"", &msg),
            Error::ProcessFailure => "failing to spawn and communicate with child process".into(),
            Error::Timeout => "timeout. If you want to give the the process more time, specify the \"timeout\" metadata field".into()
        };
        write!(f, "{}", text)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InfoRequest {
    #[serde(rename = "type")]
    request_type: String,
    version: String,
}

impl InfoRequest {
    fn new() -> Self {
        InfoRequest {
            request_type: "info".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActionRequest {
    #[serde(rename = "type")]
    request_type: String,
    version: String,
    output_format: OutputFormat,
    arguments: Vec<String>,
    metadata: HashMap<String, String>,
}

impl ActionRequest {
    /// create a request from based on a given extension and call context
    fn from(extension: &ForeignExtension, ctx: &Context) -> Self {
        // collect the metadata field that this extension is interested in
        let metadata = extension
            .interests
            .iter()
            .filter_map(|key| {
                ctx.document
                    .metadata
                    .get(key)
                    .cloned()
                    .map(|value| (key.clone(), value))
            })
            .collect();

        ActionRequest {
            request_type: "action".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            output_format: ctx.output_format,
            arguments: ctx.arguments.clone(),
            metadata,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActionResponse {
    #[serde(default)]
    imports: Vec<String>,
    #[serde(default)]
    top: String,
    #[serde(default)]
    bottom: String,
    #[serde(default)]
    errors: Vec<String>,
    #[serde(default)]
    warnings: Vec<String>,
    content: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Action {
    Raw(String),
    Extension {
        name: String,
        arguments: Vec<String>,
        block: bool,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ForeignExtension {
    name: String,
    version: String,
    #[serde(skip)]
    command: String,
    description: String,
    #[serde(default)]
    errors: Vec<String>,
    #[serde(default)]
    warnings: Vec<String>,
    supported_formats: Vec<OutputFormat>,
    #[serde(default)]
    interests: Vec<String>,
    block_support: bool,
    inline_support: bool,
}

impl Extension for ForeignExtension {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn version(&self) -> String {
        self.version.clone()
    }

    fn is_safe(&self) -> bool {
        false
    }

    fn call(&self, mut ctx: Context) -> Option<String> {
        let timeout = get_timeout(&ctx);
        let req = ActionRequest::from(self, &ctx);

        let raw_response = send(&self.command, timeout, req)
            .map_err(|error| {
                self.add_error(&error.to_string(), &mut ctx);
                error
            })
            .ok()?;

        let response: ActionResponse = serde_json::from_str(&raw_response)
            .map_err(|_| {
                self.add_error(
                    &Error::JsonParsingFailure(raw_response).to_string(),
                    &mut ctx,
                );
                Error::JsonParsingFailure
            })
            .ok()?;

        ctx.document.top.push_str(&response.top);
        ctx.document.bottom.push_str(&response.bottom);
        response.imports.iter().for_each(|i| ctx.document.import(i));
        response
            .errors
            .iter()
            .for_each(|e| self.add_error(e, &mut ctx));
        response
            .warnings
            .iter()
            .for_each(|e| self.add_warning(e, &mut ctx));

        let mut content = String::new();

        for action in response.content {
            content.push_str(&match action {
                Action::Raw(s) => s,

                Action::Extension {
                    name,
                    arguments,
                    block,
                } => {
                    let variant = if block {
                        ExtensionVariant::Block
                    } else {
                        ExtensionVariant::Inline
                    };

                    ctx.document
                        .translate_extension(
                            &name,
                            arguments,
                            variant,
                            &Origin::new(0, &self.name()),
                        )
                        .unwrap_or_else(|| String::from(""))
                }
            });
        }

        Some(content)
    }

    fn supports_block(&self) -> bool {
        self.block_support
    }

    fn supports_inline(&self) -> bool {
        self.inline_support
    }

    fn interests(&self) -> Vec<String> {
        self.interests.clone()
    }
}
