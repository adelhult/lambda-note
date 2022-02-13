mod code;
mod conditional;
mod hidden;
mod img;
mod link;
mod maketitle;
mod math;
mod calc;
mod escape;
mod raw;
mod alias;
mod define;
mod id;

use crate::parser::{Origin, OriginName};
use crate::translator::{DocumentState, OutputFormat};
use code::Code;
use conditional::Conditional;
use hidden::Hidden;
use img::Img;
use link::Link;
use maketitle::Maketitle;
use math::Math;
use calc::Calc;
use escape::Escape;
use define::Define;
use id::Id;
use raw::Raw;
use alias::Alias;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ExtensionVariant {
    Block,
    Inline,
}

pub struct Context<'a> {
    document: &'a mut DocumentState,
    origin: Origin,
    variant: ExtensionVariant,
    output_format: OutputFormat,
    arguments: Vec<String>,
}

impl<'a> Context<'a> {
    pub fn new(
        args: Vec<String>,
        variant: ExtensionVariant,
        document: &'a mut DocumentState,
        origin: Origin,
    ) -> Self {
        Context {
            arguments: args,
            output_format: document.get_output_format(),
            document,
            origin,
            variant,
        }
    }

    pub fn no_arguments(&self) -> bool {
        self.arguments.is_empty()
    }
}

pub trait Extension {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn version(&self) -> String;
    fn is_safe(&self) -> bool;

    fn call(&self, context: Context) -> Option<String>;

    fn supports_block(&self) -> bool;
    fn supports_inline(&self) -> bool;
    fn interests(&self) -> Vec<String>;

    /// Add an error to the current document state
    fn add_error(&self, description: &str, ctx: &mut Context) {
        let document_name = match &ctx.origin.name {
            OriginName::Filename(name) => name,
            _ => "MACRO EXPANSION", // todo, should display the entire expansion
        };

        ctx.document.errors.push(format!(
            "Error from {name} expression: {description}.\n\
                (Line {line_number} of {document_name}",
            name = self.name(),
            description = description,
            line_number = ctx.origin.line_number,
            document_name = document_name,
        ));
    }

    /// Add a warning to the current document state
    fn add_warning(&self, description: &str, ctx: &mut Context) {
        let document_name = match &ctx.origin.name {
            OriginName::Filename(name) => name,
            _ => "MACRO EXPANSION", // todo, should display the entire expansion
        };

        ctx.document.warnings.push(format!(
            "Warning from {name} expression: {description}.\n\
                (Line {line_number} of {document_name}",
            name = self.name(),
            description = description,
            line_number = ctx.origin.line_number,
            document_name = document_name,
        ));
    }
}

/// Returns a hashmap of all the native extensions
pub fn get_native_extensions() -> HashMap<String, Rc<dyn Extension>> {
    let mut map: HashMap<String, Rc<dyn Extension>> = HashMap::new();
    map.insert("hidden".to_string(), Rc::new(Hidden));
    map.insert("img".to_string(), Rc::new(Img));
    map.insert("math".to_string(), Rc::new(Math));
    map.insert("maketitle".to_string(), Rc::new(Maketitle));
    map.insert("code".to_string(), Rc::new(Code));
    map.insert("conditional".to_string(), Rc::new(Conditional));
    map.insert("link".to_string(), Rc::new(Link));
    map.insert("calc".to_string(), Rc::new(Calc));
    map.insert("escape".to_string(), Rc::new(Escape));
    map.insert("raw".to_string(), Rc::new(Raw));
    map.insert("alias".to_string(), Rc::new(Alias));
    map.insert("define".to_string(), Rc::new(Define));
    map.insert("id".to_string(), Rc::new(Id));
    map
}
