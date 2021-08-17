use crate::extensions::{Extension, ExtensionVariant};
use crate::translator::{DocumentState, OutputFormat};

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

    fn call(
        &self,
        args: Vec<String>,
        fmt: OutputFormat,
        variant: ExtensionVariant,
        state: &mut DocumentState,
    ) -> Option<String> {
        match fmt {
            OutputFormat::LambdaNote => todo!(),
            OutputFormat::Html => html(args, variant, self.interests(), state),
            OutputFormat::Latex => latex(args, variant, self.interests(), state),
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
    interests: Vec<String>,
    state: &mut DocumentState,
) -> Option<String> {
    let mut alt: Option<&String>;
    let mut src: Option<&String>;

    match variant {
        ExtensionVariant::Block => {
            alt = args.get(0);
            src = args.get(1);
        }

        ExtensionVariant::Inline => {
            src = args.get(0);
            alt = args.get(1);
        }
    }
    if src.is_none() {
        state.add_error("Img: no path to the image was given");
        return None;
    }

    Some(format!(
        "<img src=\"{filename}\" {alt}>",
        filename = src.map_or_else(|| String::from(""), |s| s.to_owned()),
        alt = alt.map_or_else(|| String::from(""), |s| format!("alt=\"{}\"", s))
    ))
}

// | src, [alt, width, label] |
fn latex(
    args: Vec<String>,
    variant: ExtensionVariant,
    interests: Vec<String>,
    state: &mut DocumentState,
) -> Option<String> {
    state.import("\\usepackage{graphicx}");

    let mut alt: Option<&String>;
    let mut src: Option<&String>;

    match variant {
        ExtensionVariant::Block => {
            alt = args.get(0);
            src = args.get(1);
        }

        ExtensionVariant::Inline => {
            src = args.get(0);
            alt = args.get(1);
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
        width = args.get(2).unwrap_or(&String::from("1")),
        label = args.get(3).map_or_else(
            || String::from(""),
            |label| format!("\\label{{{}}}", label.trim())
        )
    ))
}
