use std::collections::{HashMap, HashSet};

use crate::{Block, DocumentState, EscapeChar, Inline, OutputFormat, Tag, Translator};
/// A translator that transpiles into LaTeX code.
pub struct Latex;

impl Translator for Latex {
    fn output_format(&self) -> OutputFormat {
        OutputFormat::Latex
    }

    fn block(&self, state: &mut DocumentState, block: Block) -> Option<String> {
        match block {
            Block::Heading(_, lvl, _) => Some(heading(state.translate_content(&block), lvl)),
            Block::Divider(_) => Some("\\newpage".to_string()),
            Block::Paragraph(_, _) => Some(format!("{}\n\n", state.translate_content(&block))),
            _ => None,
        }
    }

    fn inline(&self, inline: &Inline) -> String {
        match inline {
            Inline::Begin(tag) => format!("\\{}{{", tag_to_string(tag)),
            Inline::End(_) => "}".to_string(),
            Inline::Escaped(escaped) => escape_char(escaped),
            Inline::Text(content) => self.escape_str(content),
            _ => panic!("Failed to translate inline element {:?}", inline),
        }
    }

    fn template(
        &self,
        content: &str,
        top: &str,
        bottom: &str,
        imports: &HashSet<String>,
        metadata: &HashMap<String, String>,
    ) -> String {
        format!(
            r#"
\documentclass[12pt]{{{class}}}
\usepackage[utf8]{{inputenc}}
{imports}
\begin{{document}}
{top}
{content}
{bottom}
\end{{document}}
            "#,
            imports = imports.iter().fold(String::new(), |acc, s| acc + s + "\n"),
            top = top,
            bottom = bottom,
            class = metadata
                .get("documentclass")
                .unwrap_or(&"article".to_string()),
            content = content,
        )
    }

    fn escape_str(&self, raw: &str) -> String {
        raw.chars()
            .map(|c| match c {
                '&' => "\\&".to_string(),
                '%' => "\\%".to_string(),
                '$' => "\\$".to_string(),
                '#' => "\\#".to_string(),
                '_' => "\\_".to_string(),
                '{' => "\\{".to_string(),
                '}' => "\\}".to_string(),
                '>' => "\\textgreater".to_string(),
                '<' => "\\textless".to_string(),
                '~' => "\\textasciitilde".to_string(),
                '^' => "\\textasciicircum".to_string(),
                '\\' => "\\textbackslash".to_string(),
                c => c.to_string(),
            })
            .collect()
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

fn tag_to_string(tag: &Tag) -> String {
    match *tag {
        Tag::Bold => "textbf",
        Tag::Italic => "textit",
        Tag::Underline => "underline",
        Tag::Superscript => "^",
        Tag::Subscript => "textsubscript",
        Tag::Strikethrough => "sout", // todo: needs package!
    }
    .to_string()
}

fn escape_char(c: &EscapeChar) -> String {
    match c {
        EscapeChar::Alpha => "$\\alpha$",
        EscapeChar::Beta => "$\\beta$",
        EscapeChar::GammaLower => "$\\gamma$",
        EscapeChar::GammaUpper => "$\\Gamma$",
        EscapeChar::DeltaLower => "$\\delta$",
        EscapeChar::DeltaUpper => "$\\Delta$",
        EscapeChar::Epsilon => "$\\epsilon$",
        EscapeChar::EpsilonVar => "$\\varepsilon$",
        EscapeChar::Zeta => "$\\zeta$",
        EscapeChar::Eta => "$\\eta$",
        EscapeChar::ThetaLower => "$\\theta$",
        EscapeChar::ThetaUpper => "$\\Theta$",
        EscapeChar::ThetaVar => "$\\vartheta$",
        EscapeChar::Iota => "$\\iota$",
        EscapeChar::Kappa => "$\\kapa$",
        EscapeChar::LambdaLower => "$\\lambda$",
        EscapeChar::LambdaUpper => "$\\Lambda$",
        EscapeChar::Mu => "$\\mu$",
        EscapeChar::Nu => "$\\nu$",
        EscapeChar::XiLower => "$\\xi$",
        EscapeChar::XiUpper => "$\\Xi$",
        EscapeChar::PiLower => "$\\pi$",
        EscapeChar::PiUpper => "$\\Pi$",
        EscapeChar::Rho => "$\\Rho$",
        EscapeChar::RhoVar => "$\\varrho$",
        EscapeChar::SigmaLower => "$\\sigma$",
        EscapeChar::SigmaUpper => "$\\Sigma$",
        EscapeChar::Tau => "$\\tau$",
        EscapeChar::UpsilonLower => "$\\upsilon$",
        EscapeChar::UpsilonUpper => "$\\Upsilon$",
        EscapeChar::PhiLower => "$\\phi$",
        EscapeChar::PhiUpper => "$\\Phi$",
        EscapeChar::PhiVar => "$\\varphi$",
        EscapeChar::Chi => "$\\chi$",
        EscapeChar::PsiLower => "$\\psi$",
        EscapeChar::PsiUpper => "$\\Psi$",
        EscapeChar::OmegaLower => "$\\omega$",
        EscapeChar::OmegaUpper => "$\\Omega$",
        // dashes:
        EscapeChar::EmDash => "---",
        EscapeChar::EnDash => "--",
        //arrows:
        EscapeChar::LeftThin => "$\\leftarrow$",
        EscapeChar::LeftBold => "$\\Leftarrow$",
        EscapeChar::RightThin => "$\\rightarrow$",
        EscapeChar::RightBold => "$\\Rightarrow$",
        EscapeChar::UpThin => "$\\uparrow$",
        EscapeChar::UpBold => "$\\Uparrow$",
        EscapeChar::DownThin => "$\\downarrow$",
        EscapeChar::DownBold => "$\\Downarrow$",
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
    }
    .to_string()
}
