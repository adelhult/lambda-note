use crate::extensions::{Context, Extension, ExtensionVariant};
use crate::translator::OutputFormat;
use latex2mathml::{latex_to_mathml, DisplayStyle};

/// **Native extension**: make math equations
/// TODO: handle errors
#[derive(Clone)]
pub struct Math;

impl Extension for Math {
    fn name(&self) -> String {
        "Math".to_string()
    }

    fn description(&self) -> String {
        "Format equations and math using LaTeX syntax.".to_string()
    }

    fn version(&self) -> String {
        "1".to_string()
    }

    fn is_safe(&self) -> bool {
        true
    }

    fn call(&self, mut ctx: Context) -> Option<String> {
        match ctx.output_format {
            OutputFormat::Latex => latex(&ctx),
            OutputFormat::Html => html(&mut ctx),
            _ => panic!("Not implemented yet"),
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

fn html(ctx: &mut Context) -> Option<String> {
    ctx.document.import(r#"<script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/mml-chtml.js"></script>"#);

    let value = ctx.arguments.get(0)?;
    latex_to_mathml(
        value,
        match ctx.variant {
            ExtensionVariant::Block => DisplayStyle::Block,
            ExtensionVariant::Inline => DisplayStyle::Inline,
        },
    )
    .ok()
}

fn latex(ctx: &Context) -> Option<String> {
    let value = ctx.arguments.get(0).map_or_else(|| "", |content| content);
    Some(match ctx.variant {
        ExtensionVariant::Block => format!("\\begin{{equation}}\n{}\n\\end{{equation}}", value),
        ExtensionVariant::Inline => format!("${}$", value),
    })
}
