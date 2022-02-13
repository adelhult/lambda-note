use crate::extensions::{Extension, Context};

#[derive(Clone)]
pub struct Id;

impl Extension for Id {
    fn name(&self) -> String {
        "Identity".to_string()
    }

    fn description(&self) -> String {
        "Identity function, does nothing but output the same text".to_string()
    }

    fn version(&self) -> String {
        "1".to_string()
    }

    fn is_safe(&self) -> bool {
        true
    }

    fn call(&self, ctx: Context) -> Option<String> {
        let text = ctx.arguments.get(0)?;
        // FIME: create and use a translate inline function if
        // this is called inline.
        Some(ctx.document.translate_no_template(text, "identity"))
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
