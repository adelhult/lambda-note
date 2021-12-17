use crate::extensions::{Extension, ExtensionVariant};
use crate::translator::{DocumentState, OutputFormat};
use crate::Origin;
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
        "Format and syntax highlight code ".to_string()
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
        origin: &Origin,
    ) -> Option<String> {
        match fmt {
            OutputFormat::Html => html(args, variant),
            OutputFormat::Latex => latex(args, variant, state),
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

fn html(args: Vec<String>, variant: ExtensionVariant) -> Option<String> {
    lazy_static! {
        static ref PS: SyntaxSet = SyntaxSet::load_defaults_newlines();
        static ref TS: ThemeSet = ThemeSet::load_defaults();
    }

    let theme = &TS.themes["InspiredGitHub"];

    // get the syntax based on the given input
    // otherwise fallback to using plain text
    let syntax = if let Some(language) = args.get(1) {
        match PS.find_syntax_by_token(language.trim()) {
            Some(syntax) => syntax,
            None => PS.find_syntax_plain_text(),
        }
    } else {
        PS.find_syntax_plain_text()
    };

    let code = match args.get(0) {
        Some(value) => value.to_string(),
        None => "".into(),
    };

    match variant {
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

fn latex(
    args: Vec<String>,
    variant: ExtensionVariant,
    state: &mut DocumentState,
) -> Option<String> {
    let code = match args.get(0) {
        Some(value) => value.to_string(),
        None => "".into(),
    };

    let language = format!(
        "{{{}}}",
        match args.get(1) {
            Some(language) => language,
            None => "text",
        }
    );

    state.import("\\usepackage{minted}");

    Some(match variant {
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
