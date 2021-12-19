use crate::extensions::{Extension, Context};

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

    fn call(&self, _: Context) -> Option<String> {
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
