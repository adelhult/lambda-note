use crate::EscapeChar;

use super::{Block, DocumentState, Inline, Tag};

pub fn translate_block(state: &mut DocumentState, block: Block) -> Option<String> {
    match block {
        Block::Heading(text, lvl) => Some(heading(state.translate_text(text), lvl)),
        Block::Divider => Some("\\newpage".to_string()),
        Block::Paragraph(text) => Some(format!("{}\n\n", state.translate_text(text))),
        _ => None,
    }
}

fn heading(text: String, level: u8) -> String {
    match level {
        1 => format!("\\section{{{}}}", text),
        2 => format!("\\subsection{{{}}}", text),
        3 => format!("\\subsubsection{{{}}}", text),
        4 => format!("\\paragraph{{{}}}", text),
        _ => format!("\\subparagraph{{{}}}", text),
    }
}

pub fn translate_inline(inline: Inline) -> String {
    match inline {
        Inline::Begin(tag) => format!("\\{}{{", tag_to_string(&tag)),
        Inline::End(_) => "}".to_string(),
        Inline::Escaped(escaped) => escape_char(escaped),
        Inline::Text(content) => escape_str(&content),
        _ => panic!("Failed to translate inline element {:?}", inline),
    }
}

pub fn boilerplate(state: &mut DocumentState, content: &str) -> String {
    format!(
        r#"
\documentclass[12pt]{{{class}}}
{title}
{author}
\begin{{document}}
{content}
\end{{document}}
        "#,
        class = state
            .metadata
            .get("documentclass")
            .unwrap_or(&"article".to_string()),
        title = state
            .metadata
            .get("title")
            .map_or_else(|| "".to_string(), |title| format!("\\title{{{}}}", title)),
        author = state
            .metadata
            .get("author")
            .map_or_else(|| "".to_string(), |author| format!("\\author{{{}}}", author)),
        content = content
    )
}

fn tag_to_string(tag: &Tag) -> String {
    match tag {
        &Tag::Bold => "textbf",
        &Tag::Italic => "textit",
        &Tag::Underline => "underline",
        &Tag::Superscript => "^",
        &Tag::Subscript => "_",
        &Tag::Strikethrough => "sout", // todo: needs package!
    }
    .to_string()
}

fn escape_str(raw: &str) -> String {
    raw.chars()
        .map(|c| match c {
            '&' => "\\&".to_string(),
            '%' => "\\%".to_string(),
            '$' => "\\$".to_string(),
            '#' => "\\#".to_string(),
            '_' => "\\_".to_string(),
            '{' => "\\{".to_string(),
            '}' => "\\}".to_string(),
            '~' => "\\textasciitilde".to_string(),
            '^' => "\\textasciicircum".to_string(),
            '\\' => "\\textbackslash".to_string(),
            c => c.to_string(),
        })
        .collect()
}

fn escape_char(c: EscapeChar) -> String {
    match c {
        EscapeChar::Alpha => "\\alpha", 
        EscapeChar::Beta => "\\beta",
        EscapeChar::GammaLower => "\\gamma",
        EscapeChar::GammaUpper => "\\Gamma",
        EscapeChar::DeltaLower => "\\delta",
        EscapeChar::DeltaUpper => "\\Delta",
        EscapeChar::Epsilon => "\\epsilon",
        EscapeChar::EpsilonVar => "\\varepsilon",
        EscapeChar::Zeta => "\\zeta",
        EscapeChar::Eta => "\\eta",
        EscapeChar::ThetaLower => "\\theta",
        EscapeChar::ThetaUpper => "\\Theta",
        EscapeChar::ThetaVar => "\\vartheta",
        EscapeChar::Iota => "\\iota",
        EscapeChar::Kappa => "\\kapa",
        EscapeChar::LambdaLower => "\\lambda",
        EscapeChar::LambdaUpper => "\\Lambda",
        EscapeChar::Mu => "\\mu",
        EscapeChar::Nu => "\\nu",
        EscapeChar::XiLower => "\\xi",
        EscapeChar::XiUpper => "\\Xi",
        EscapeChar::PiLower => "\\pi",
        EscapeChar::PiUpper => "\\Pi",
        EscapeChar::Rho => "\\Rho",
        EscapeChar::RhoVar => "\\varrho",
        EscapeChar::SigmaLower => "\\sigma",
        EscapeChar::SigmaUpper => "\\Sigma",
        EscapeChar::Tau => "\\tau",
        EscapeChar::UpsilonLower => "\\upsilon",
        EscapeChar::UpsilonUpper => "\\Upsilon",
        EscapeChar::PhiLower => "\\phi",
        EscapeChar::PhiUpper => "\\Phi",
        EscapeChar::PhiVar => "\\varphi",
        EscapeChar::Chi => "\\chi",
        EscapeChar::PsiLower => "\\psi",
        EscapeChar::PsiUpper => "\\Psi",
        EscapeChar::OmegaLower => "\\omega",
        EscapeChar::OmegaUpper => "\\Omega",
        // dashes:
        EscapeChar::EmDash => "---",
        EscapeChar::EnDash => "--",
        //arrows:
        EscapeChar::LeftThin => "\\left",
        EscapeChar::LeftBold => "\\Left",
        EscapeChar::RightThin => "\\right",
        EscapeChar::RightBold => "\\Right",
        EscapeChar::UpThin => "\\up",
        EscapeChar::UpBold => "\\Up",
        EscapeChar::DownThin => "\\down",
        EscapeChar::DownBold => "\\Down",
        // escaping lambda note syntax
        EscapeChar::Asterisk => "*",
        EscapeChar::Caret => "\\textasciicircum",
        EscapeChar::Underscore => "\\_",
        EscapeChar::ForwardSlash => "/",
        EscapeChar::BackSlash => "\\textbackslash",
        EscapeChar::Equal => "=",
        EscapeChar::Tilde => "\\textasciitilde",
        EscapeChar::Bar => "|",
        EscapeChar::Colon => ":",
        EscapeChar::TableFlip => "(╯°□°）╯︵ ┻━┻",
    }.to_string()
}   
