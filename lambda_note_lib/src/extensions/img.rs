use crate::extensions::{Context, Extension, ExtensionVariant};
use crate::translator::OutputFormat;

/// **Native extension**: add an image
#[derive(Clone)]
pub struct Img;

// TODO: make the html variant take the same args as the latex variant
impl Extension for Img {
    fn name(&self) -> String {
        "Img".to_string()
    }

    fn description(&self) -> String {
        "Add an image.\n\
        \n\
        Usage:\n\
        |img, filepath, [alt, width, label]|\n\
        \n\
        or as a block...\n\
        ---- img, filepath [width, label] ----\n\
        alt text\n\
        ----"
            .to_string()
    }

    fn version(&self) -> String {
        "1".to_string()
    }

    fn is_safe(&self) -> bool {
        true
    }

    fn call(&self, mut ctx: Context) -> Option<String> {
        match ctx.output_format {
            OutputFormat::LambdaNote => todo!(),
            OutputFormat::Html => self.html(&mut ctx),
            OutputFormat::Latex => self.latex(&mut ctx),
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
impl Img {
    fn html(&self, ctx: &mut Context) -> Option<String> {
        let alt: Option<&String>;
        let src: Option<&String>;

        match ctx.variant {
            ExtensionVariant::Block => {
                alt = ctx.arguments.get(0);
                src = ctx.arguments.get(1);
            }

            ExtensionVariant::Inline => {
                src = ctx.arguments.get(0);
                alt = ctx.arguments.get(1);
            }
        }
        if src.is_none() {
            self.add_error("Img: no path to the image was given", ctx);
            return None;
        }

        Some(format!(
            "<img src=\"{filename}\" {alt} style=\"max-width:{width}%\">",
            filename = src.map_or_else(|| String::from(""), |s| s.to_owned()),
            width = ctx.arguments.get(2).unwrap_or(&String::from("100")),
            alt = alt.map_or_else(|| String::from(""), |s| format!("alt=\"{}\"", s)),
        ))
    }

    // | src, [alt, width, label] |
    fn latex(&self, ctx: &mut Context) -> Option<String> {
        ctx.document.import("\\usepackage{graphicx}");

        let alt: Option<&String>;
        let src: Option<&String>;

        match ctx.variant {
            ExtensionVariant::Block => {
                alt = ctx.arguments.get(0);
                src = ctx.arguments.get(1);
            }

            ExtensionVariant::Inline => {
                src = ctx.arguments.get(0);
                alt = ctx.arguments.get(1);
            }
        }

        Some(format!(
            "\\begin{{figure}}[h]
{caption}
{label}
\\centering
\\includegraphics[width={width}\\textwidth]{{{src}}}
\\end{{figure}}",
            src = src.unwrap_or(&String::from("")),
            caption = alt.map_or_else(
                || String::from(""),
                |text| format!("\\caption{{{}}}", text.trim().replace("\n", r#"\\"#))
            ),
            width = ctx.arguments.get(2).unwrap_or(&String::from("1")),
            label = ctx.arguments.get(3).map_or_else(
                || String::from(""),
                |label| format!("\\label{{{}}}", label.trim())
            )
        ))
    }
}
