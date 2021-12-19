use crate::extensions::{Extension, ExtensionVariant, Context};
use crate::translator::OutputFormat;

/// **Native extension**: add an image
#[derive(Clone)]
pub struct Link;

// TODO: make the html variant take the same args as the latex variant
impl Extension for Link {
    fn name(&self) -> String {
        "Link".to_string()
    }

    fn description(&self) -> String {
        "Add a hyperlink.\n\
        \n\
        Usage:\n\
        |link, url, [label]|\n\
        ...or as a block:\n\
        ------- link, https://eli.nu --------\n\
        All this **text** and other ==content==\n
        is part of the link.\n
        -------------------------------------\n\
        Note: If no label is provided, the url will just be displayed.\n\
        Provide the metadata field linkcolor to choose the color of the link\n
        for example, :: link_color = red, "
            .to_string()
    }

    fn version(&self) -> String {
        "1".to_string()
    }

    fn call(
        &self,
        mut ctx: Context,
    ) -> Option<String> {
        let url: Option<&String>;
        let label: Option<&String>;

        match ctx.variant {
            ExtensionVariant::Block => {
                url = ctx.arguments.get(1);
                label = ctx.arguments.get(0);
            }
            ExtensionVariant::Inline => {
                url = ctx.arguments.get(0);
                label = ctx.arguments.get(1);
            }
        }

        if url.is_none() {
            self.add_error("Link extensions need to be provided an url", &mut ctx);
            return None;
        }
        let url_text = url.unwrap();

        let color = ctx.document.metadata.get("link_color").cloned();

        match ctx.output_format {
            OutputFormat::LambdaNote => todo!(),
            // html output
            OutputFormat::Html => {
                Some(format!(
                    "<a href={url}{style}>{label}</a>",
                    url = url_text,
                    label = match label {
                        Some(text) => ctx.document.translate_no_boilerplate(text, "Link extension"),
                        None => url_text.to_string(),
                    },
                    style = match color {
                        None => format!(""),
                        Some(color) => format!(" style=\"color:{}\"", color)
                    }
                ))
            }
            // latex output
            OutputFormat::Latex => {
                ctx.document.import("\\usepackage{hyperref}");
                
                // add color options if given any
                if let Some(color) = color {
                    ctx.document.import(&format!(
                        "\\hypersetup{{colorlinks=true, linkcolor={color},urlcolor={color}}}",
                        color = color
                    ));
                }

                Some(match label {
                    Some(text) => format!("\\href{{{}}}{{{}}}", url_text, ctx.document.translate_no_boilerplate(text, "Link extension")),
                    None => format!("\\url{{{}}}", url_text),
                })
            }
        }
    }

    fn supports_block(&self) -> bool {
        true
    }

    fn supports_inline(&self) -> bool {
        true
    }

    fn interests(&self) -> Vec<String> {
        vec!["link_color".to_string()]
    }
}
