use crate::extensions::{Context, Extension};

#[derive(Clone)]
pub struct Alias;

impl Extension for Alias {
    fn name(&self) -> String {
        "Alias".to_string()
    }

    fn description(&self) -> String {
        "Alias an extension to use another name. |alias, math, m|".to_string()
    }

    fn version(&self) -> String {
        "1".to_string()
    }

    fn is_safe(&self) -> bool {
        true
    }

    fn call(&self, mut ctx: Context) -> Option<String> {
        if ctx.arguments.len() != 2 {
            self.add_error(
                &format!(
                    "Expected two arguments, got {}. Like this: |alias, math, m|",
                    ctx.arguments.len()
                ),
                &mut ctx,
            );
        }
        let original = ctx.arguments.get(0)?.trim();
        let new = ctx.arguments.get(1)?.trim();

        match ctx.document.extensions.get(original) {
            Some(extension) => {
                let extension = extension.clone();
                ctx.document.extensions.insert(new.into(), extension)?;
            }
            None => self.add_error(
                &format!("No extension found with the name of {}", original),
                &mut ctx,
            ),
        };
        None
    }

    fn supports_block(&self) -> bool {
        false
    }

    fn supports_inline(&self) -> bool {
        true
    }

    fn interests(&self) -> Vec<String> {
        vec![]
    }
}
