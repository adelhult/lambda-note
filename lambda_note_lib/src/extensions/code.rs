use std::collections::HashMap;
use crate::extensions::{Extension, ExtensionVariant};
use crate::translator::{DocumentState, OutputFormat};

/// **Native extension**: hides the content from the final output
#[derive(Clone)]
pub struct Code;

impl Extension for Code {
    fn name(&self) -> String {
        "Code".to_string()
    }

    fn description(&self) -> String {
        "Format and syntax highlight code ".to_string()
    }

    fn version(&self) -> String {
        "1".to_string()
    }

    fn call(
        &self,
        args: Vec<String>,
        fmt: OutputFormat,
        variant: ExtensionVariant,
        state: &mut DocumentState
    ) -> Option<String> {
        match fmt {
            OutputFormat::Html => html(state, variant),
            OutputFormat::Latex => latex(state, variant),
            _ => panic!("Output format {:?} not supported by code extension", fmt)
        }
    }

    fn supports_block(&self) -> bool {
        true
    }

    fn supports_inline(&self) -> bool {
        true
    }

    fn interests(&self) -> Vec<String> {
        vec![]
    }
}

fn html(state: &mut DocumentState, variant: ExtensionVariant) -> Option<String> {
    todo!()
}


fn latex(state: &mut DocumentState, variant: ExtensionVariant) -> Option<String>{
    todo!()
}
