use crate::extensions::{Context, Extension, ExtensionVariant};
use crate::translator::OutputFormat;
use lazy_static::lazy_static;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::html::{
    highlighted_html_for_string, styled_line_to_highlighted_html, IncludeBackground,
};
use syntect::parsing::SyntaxSet;

/// **Native extension**: hides the content from the final output
#[derive(Clone)]
pub struct Code;

impl Extension for Code {
    fn name(&self) -> String {
        "Code".to_string()
    }

    fn description(&self) -> String {
        "Format and syntax highlight code. Usage:\n\
        ```\n
        ---- code, [language] ----\n\
        foo = bar\n\
        ----\n\
        \n\
        |code, foo = bar, [language]|\n\
        ```".to_string()
    }

    fn version(&self) -> String {
        "1".to_string()
    }

    fn is_safe(&self) -> bool {
        true
    }

    fn call(&self, mut ctx: Context) -> Option<String> {
        match ctx.output_format {
            OutputFormat::Html => html(&ctx),
            OutputFormat::Latex => latex(&mut ctx),
            _ => None,
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

fn html(ctx: &Context) -> Option<String> {
    lazy_static! {
        static ref PS: SyntaxSet = SyntaxSet::load_defaults_newlines();
        static ref TS: ThemeSet = ThemeSet::load_defaults();
    }

    let theme = &TS.themes["InspiredGitHub"];

    // get the syntax based on the given input
    // otherwise fallback to using plain text
    let syntax = if let Some(language) = ctx.arguments.get(1) {
        match PS.find_syntax_by_token(language.trim()) {
            Some(syntax) => syntax,
            None => PS.find_syntax_plain_text(),
        }
    } else {
        PS.find_syntax_plain_text()
    };

    let code = match ctx.arguments.get(0) {
        Some(value) => value.to_string(),
        None => "".into(),
    };

    match ctx.variant {
        ExtensionVariant::Block => Some(highlighted_html_for_string(&code, &PS, syntax, theme)),
        ExtensionVariant::Inline => {
            let mut h = HighlightLines::new(syntax, theme);
            let regions = h.highlight(&code, &PS);
            Some(styled_line_to_highlighted_html(
                &regions[..],
                IncludeBackground::No,
            ))
        }
    }
}

fn latex(ctx: &mut Context) -> Option<String> {
    let code = match ctx.arguments.get(0) {
        Some(value) => value.to_string(),
        None => "".into(),
    };

    let language = format!(
        "{{{}}}",
        match ctx.arguments.get(1) {
            Some(language) => language,
            None => "text",
        }
    );

    ctx.document.import("\\usepackage{minted}");

    Some(match ctx.variant {
        ExtensionVariant::Block => format!(
            "\\begin{{minted}}{lang}\n{content}\n\\end{{minted}}",
            lang = language,
            content = code
        ),
        ExtensionVariant::Inline => {
            format!("\\mint{lang}|{content}|", lang = language, content = code)
        }
    })
}
