use std::collections::HashMap;
use crate::extensions::{Extension, ExtensionVariant};
use crate::translator::{DocumentState, OutputFormat};

/// **Native extension**: hides the content from the final output
#[derive(Clone)]
pub struct Hidden;

impl Extension for Hidden {
    fn name(&self) -> String {
        "Hidden".to_string()
    }

    fn description(&self) -> String {
        "Hides the content from the final output".to_string()
    }

    fn version(&self) -> String {
        "1".to_string()
    }

    fn call(
        &self,
        _: Vec<String>,
        _: OutputFormat,
        _: ExtensionVariant,
        _: &mut DocumentState
    ) -> Option<String> {
        None
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
