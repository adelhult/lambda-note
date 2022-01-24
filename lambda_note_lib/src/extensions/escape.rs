use crate::extensions::{Context, Extension};

/// Escapes the entire input as plain text
#[derive(Clone)]
pub struct Escape;

impl Extension for Escape {
    fn name(&self) -> String {
        "Escape".to_string()
    }

    fn description(&self) -> String {
        "Escapes the entire input as plain text to avoid it being parsed as markup".to_string()
    }

    fn version(&self) -> String {
        "1".to_string()
    }

    fn call(&self, mut ctx: Context) -> Option<String> {
        let nr_args = ctx.arguments.len();
        if nr_args != 1 {
            self.add_warning(&format!("Got {} arguments, expected 1.", nr_args), &mut ctx);
        }

        let input = ctx.arguments.get(0)?;
        Some(ctx.document.escape_str(input))
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
