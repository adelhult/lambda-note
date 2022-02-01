use crate::extensions::{Extension, ExtensionVariant, Context};
use crate::translator::{DocumentState, OutputFormat};
/// **Native extension**: generate titlepages just like in latex
#[derive(Clone)]
pub struct Maketitle;

impl Extension for Maketitle {
    fn name(&self) -> String {
        "Maketitle".to_string()
    }

    fn description(&self) -> String {
        "Generate a titlepage \n\
        \n\
        Usage:\n\
        |titlepage|\n\
        \n\
        Interests:\n\
        title, author and date.".to_string()
    }

    fn version(&self) -> String {
        "1".to_string()
    }

    fn is_safe(&self) -> bool {
        true
    }

    fn call(
        &self,
        mut ctx: Context,
    ) -> Option<String> {
        if ctx.no_arguments() {
            self.add_warning("maketitle does not take any arguments", &mut ctx);
        }

        if let ExtensionVariant::Block = ctx.variant {
            self.add_error("maketitle can not be a block extension", &mut ctx);
            return None;
        }
        match ctx.output_format {
            OutputFormat::Latex => latex(&mut ctx),
            OutputFormat::Html => html(&mut ctx),
            _ => panic!("Not implemented yet"),
        }
    }

    fn supports_block(&self) -> bool {
        false
    }

    fn supports_inline(&self) -> bool {
        true
    }

    fn interests(&self) -> Vec<String> {
        vec![
            String::from("title"),
            String::from("author"),
            String::from("date"),
        ]
    }
}

fn latex(ctx: &mut Context) -> Option<String> {
    let mut defines = String::new();

    for field in ["title", "author", "date"].iter() {
        if let Some(value) = ctx.document.metadata.get(*field) {
            // example: \field{value}
            defines.push_str(&format!("\\{}{{{}}}\n", field, value));
        }
    }
    ctx.document.import(&defines);
    Some("\\maketitle".to_string())
}

fn html(ctx: &mut Context) -> Option<String> {
    // add stylinging for the tile.
    // TODO: Should perhaps have a seperate hook for that
    ctx.document.import(
        "<style>\
    .maketitle__title {\
        position:relative;\
        margin-left:auto;\
        margin-right:auto;\
        max-width: 70%;\
        text-align:center;\
        margin-bottom:2rem;\
        margin-top:2rem;\
    }\
    .maketitle__title h1 {\
        margin-bottom:0.5rem;\
    }\
    </style>",
    );

    let (title, author, date) = get_metadata(ctx.document)?;

    Some(format!(
        "<header class=\"maketitle__title\">\
        <h1>{title}</h1>\
        <span class=\"maketitle__author\">{author}</span>\
        <br/>\
        <span class=\"maketitle__date\">{date}</span>\
    </header>",
        title = title,
        author = author,
        date = date,
    ))
}

/// Get all the relevent metadata fields, otherwise return None
fn get_metadata(state: &mut DocumentState) -> Option<(&String, &String, &String)> {
    let title = state.metadata.get("title")?;
    let author = state.metadata.get("author")?;
    let date = state.metadata.get("date")?;
    Some((title, author, date))
}
