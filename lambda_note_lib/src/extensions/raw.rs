use crate::extensions::{Context, Extension};

/// Input gets treated as raw text and won't be escaped or translated
#[derive(Clone)]
pub struct Raw;

impl Extension for Raw {
    fn name(&self) -> String {
        "Raw".to_string()
    }

    fn description(&self) -> String {
        "Input gets treated as raw text and won't be escaped or translated".to_string()
    }

    fn version(&self) -> String {
        "1".to_string()
    }

    fn call(&self, mut ctx: Context) -> Option<String> {
        let nr_args = ctx.arguments.len();
        if nr_args != 1 {
            self.add_warning(&format!("Got {} arguments, expected 1.", nr_args), &mut ctx);
        }

        Some(ctx.arguments.get(0)?)
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
