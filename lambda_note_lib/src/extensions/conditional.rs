use crate::extensions::{Extension, ExtensionVariant};
use crate::translator::{DocumentState, OutputFormat};
use lazy_static::lazy_static;
use regex::Regex;

//notes:
//----- if, platform = mac, output = html ------
//includes body if all args checks out

//possible expressions:
//      platform/os = macos/mac/windows/win/linux/unix/web/wasm (unix includes mac)
//      target/output/file/type/extension = html/latex/tex/lambdanote/λnote
//      conditional_XXX = str

//boolean expressions are key/value pairs separated by =, ==, !=, is, is not, isn't

//you can use keywords "all" and "or"/"one of" to set the behaviour of multiple expressions,
//by default using "boolean and"
//      "all" means booleAND
//      "one of" or "or" means boORlean

//author: CMDJojo

#[derive(Clone)]
pub struct Conditional;

impl Extension for Conditional {
    fn name(&self) -> String {
        "Conditional".to_string()
    }

    fn description(&self) -> String {
        "Includes a block if conditions are met.\n\
        \n\
        Usage:\n\
        |conditional, You are not using HTML, file != html|\n\
        \n\
        or as a block...\n\
        ---- conditional, one of, os = mac, type = latex ----\n\
        you are probably not viewing this in Internet Explorer...\n\
        ----"
            .to_string()
    }

    fn version(&self) -> String {
        "1".to_string()
    }

    fn call(
        &self,
        args: Vec<String>,
        fmt: OutputFormat,
        _: ExtensionVariant,
        state: &mut DocumentState,
    ) -> Option<String> {
        if args.is_empty() {
            return None;
        }
        let body_index = 0;
        let body = args.get(body_index).unwrap();
        let mut and = true;
        let mut failed = false;
        let mut exprs: Vec<Expression> = vec![];
        for (pos, arg) in args.iter().enumerate() {
            if pos == body_index {
                continue;
            }
            match parse(arg.as_str(), state) {
                Some(ParseResult::Expr(expr)) => exprs.push(expr),
                Some(ParseResult::Rule(rule)) => and = rule == ExpressionRule::BooleanAnd,
                None => failed = true,
            }
        }
        if failed {
            return None;
        }
        let platform = get_platform(state);
        if and {
            for expr in exprs {
                if !expr.check(fmt, platform, state) {
                    return None;
                }
            }
            Some(state.translate_no_boilerplate(&body.to_string()))
        } else {
            for expr in exprs {
                if expr.check(fmt, platform, state) {
                    return Some(state.translate_no_boilerplate(&body.to_string()));
                }
            }
            None
        }
    }

    fn supports_block(&self) -> bool {
        true
    }

    fn supports_inline(&self) -> bool {
        true
    }

    fn interests(&self) -> Vec<String> {
        vec!["conditional_*".to_string()]
    }
}

fn parse(text: &str, state: &mut DocumentState) -> Option<ParseResult> {
    if let Some(rule) = parse_special(text) {
        Some(ParseResult::Rule(rule))
    } else {
        parse_expr(text, state).map(ParseResult::Expr)
    }

    
}

fn parse_special(text: &str) -> Option<ExpressionRule> {
    lazy_static! {
        static ref AND_PATTERN: Regex = Regex::new(r"^[ \t]*all[ \t]*$").unwrap();
        static ref OR_PATTERN: Regex = Regex::new(r"^[ \t]*((one *of)|(or))[ \t]*$").unwrap();
    }
    if AND_PATTERN.is_match(text) {
        Some(ExpressionRule::BooleanAnd)
    } else if OR_PATTERN.is_match(text) {
        Some(ExpressionRule::BooleanOr)
    } else {
        None
    }
}

fn parse_expr(text: &str, state: &mut DocumentState) -> Option<Expression> {
    lazy_static! {
        static ref MAIN_PATTERN: Regex =
            Regex::new(r"^[ \t]*([\w_]+?)[ \t]*(?:(is not|isnt|isn't|!=)|(is|==|=))[ \t]*([\w_]+)[ \t]*$").unwrap();
        static ref CONDITIONAL_PATTERN: Regex =
            Regex::new(r"^[ \t]*conditional_[\w_]*[ \t]*(?:(?:is not|isnt|isn't|!=)|(?:is|==|=))[ \t]*(.*?)[ \t]*$").unwrap();
    }

    if let Some(captures) = MAIN_PATTERN.captures(text) {
        let mut key = captures.get(1)?.as_str().to_string();
        let xnor = captures.get(3).is_some();
        let mut val = captures.get(4)?.as_str().to_string();
        if key.starts_with("conditional_") {
            if let Some(val_capture) = CONDITIONAL_PATTERN.captures(text) {
                if let Some(search_val) = val_capture.get(1) {
                    Some(Expression::StringEquality(
                        key,
                        search_val.as_str().to_string(),
                        xnor,
                    ))
                } else {
                    state.add_error(&format!(
                        "Could not find value to search for with key {}",
                        key
                    ));
                    None
                }
            } else {
                state.add_error(&format!(
                    "Could not find value to search for with key {}",
                    key
                ));
                None
            }
        } else {
            key = key.to_lowercase();
            val = val.to_lowercase();
            if key == "platform" || key == "os" {
                match val.as_str() {
                    "macos" | "mac" => Some(Expression::PlatformEquality(
                        Platform::Unix(UnixVariant::MacOs),
                        xnor,
                    )),
                    "windows" | "win" => {
                        Some(Expression::PlatformEquality(Platform::Windows, xnor))
                    }
                    "linux" => Some(Expression::PlatformEquality(
                        Platform::Unix(UnixVariant::Linux),
                        xnor,
                    )),
                    "unix" => Some(Expression::PlatformEquality(
                        Platform::Unix(UnixVariant::Other),
                        xnor,
                    )),
                    "web" | "wasm" => Some(Expression::PlatformEquality(Platform::Web, xnor)),
                    _ => {
                        state.add_error(&format!(
                            "Unknown platform: {}\nAllowed values: Windows/Win, MacOS/Mac, Linux, Unix, Web/Wasm",
                            val
                        ));
                        None
                    }
                }
            } else if ["target", "output", "file", "type", "extension", "format"]
                .contains(&key.as_str())
            {
                match val.as_str() {
                    "html" => Some(Expression::OutputEquality(OutputFormat::Html, xnor)),
                    "latex" | "tex" => Some(Expression::OutputEquality(OutputFormat::Latex, xnor)),
                    "lambdanote" | "λnote" => {
                        Some(Expression::OutputEquality(OutputFormat::LambdaNote, xnor))
                    }
                    _ => {
                        state.add_error(&format!(
                            "Unknown file type: {}\nAllowed values: HTML, LaTeX/TeX, Lambdanote/λnote",
                            val
                        ));
                        None
                    }
                }
            } else {
                state.add_error(&format!("Unknown key: {}\n\
                Valid keys: platform/os, target/output/file/type/extension/format, and metadata fields starting with conditional_, \
                and keywords 'all', 'one of'/'or'", key));
                None
            }
        }
    } else {
        state.add_error(&format!("Invalid expression: {}", text));
        None
    }
}

enum ParseResult {
    Expr(Expression),
    Rule(ExpressionRule),
}

#[derive(PartialEq, Eq)]
enum ExpressionRule {
    BooleanAnd,
    BooleanOr,
}
enum Expression {
    PlatformEquality(Platform, bool),
    OutputEquality(OutputFormat, bool),
    StringEquality(String, String, bool),
}

impl Expression {
    fn check(
        &self,
        document_format: OutputFormat,
        document_platform: Platform,
        document_state: &mut DocumentState,
    ) -> bool {
        match self {
            Self::PlatformEquality(platform, xnor) => {
                if platform == &Platform::Unix(UnixVariant::Other) {
                    match document_platform {
                        Platform::Unix(_) => *xnor,
                        _ => !*xnor,
                    }
                } else {
                    (*platform == document_platform) ^ (!xnor)
                }
            }
            Self::OutputEquality(format, xnor) => (*format == document_format) ^ (!xnor),
            Self::StringEquality(key, value, xnor) => {
                if let Some(val) = document_state.metadata.get(key) {
                    (val == value) ^ (!xnor)
                } else {
                    document_state.add_warning(&format!("Key {} doesn't exist!", key));
                    !xnor
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Platform {
    Windows,
    Web,
    Unix(UnixVariant),
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UnixVariant {
    Linux,
    MacOs,
    Other,
}

fn get_platform(state: &mut DocumentState) -> Platform {
    if cfg!(target_os = "macos") {
        Platform::Unix(UnixVariant::MacOs)
    } else if cfg!(target_family = "windows") {
        Platform::Windows
    } else if cfg!(target_os = "linux") {
        Platform::Unix(UnixVariant::Linux)
    } else if cfg!(target_family = "unix") {
        Platform::Unix(UnixVariant::Other)
    } else if cfg!(target_family = "wasm") {
        Platform::Web
    } else {
        state.add_warning("Could not infer platform");
        Platform::Unknown
    }
}
