use crate::extensions::{Extension, ExtensionVariant};
use crate::translator::{DocumentState, OutputFormat};
use crate::Origin;
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
        "Format equations and math".to_string()
    }

    fn version(&self) -> String {
        "1".to_string()
    }

    fn call(
        &self,
        args: Vec<String>,
        output_format: OutputFormat,
        variant: ExtensionVariant,
        state: &mut DocumentState,
        origin: &Origin,
    ) -> Option<String> {
        match output_format {
            OutputFormat::Latex => latex(args, variant, state),
            OutputFormat::Html => html(args, variant, state),
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

fn html(
    args: Vec<String>,
    variant: ExtensionVariant,
    state: &mut DocumentState,
) -> Option<String> {
    state.import(r#"<script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/mml-chtml.js"></script>"#);

    let value = args.get(0)?;
    latex_to_mathml(
        value,
        match variant {
            ExtensionVariant::Block => DisplayStyle::Block,
            ExtensionVariant::Inline => DisplayStyle::Inline,
        },
    )
    .ok()
}

fn latex(
    args: Vec<String>,
    variant: ExtensionVariant,
    _: &mut DocumentState,
) -> Option<String> {
    let value = args.get(0).map_or_else(|| "", |content| content);
    Some(match variant {
        ExtensionVariant::Block => format!("\\begin{{equation}}\n{}\n\\end{{equation}}", value),
        ExtensionVariant::Inline => format!("${}$", value),
    })
}
