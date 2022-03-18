//! A simple, highly extendable, markup language
//!
//! λnote is a markup language with a familiar syntax to Markdown, but with the
//! the addition of many more inline text styles and
//! a powerful built-in extensions system.
//!
//! # Quick Start
//! ```
//! use lambda_note_lib::{DocumentState, Html};
//!  
//! let mut document = DocumentState::new(Html);
//! let result = document.translate("# Hello\n \\lambdanote!", "test");
//! ```

mod extensions;
mod parser;
mod translator;

pub use parser::{parse_doc, Block, EscapeChar, Inline, Origin, Tag};
pub use translator::{DocumentState, Html, Latex, OutputFormat, Translator, WebPreview, HtmlTemplate};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_chars() {
        let dashes = parse_doc(r#"\endash\emdash"#, "test");
        assert_eq!(
            dashes[0],
            Block::Paragraph(
                vec![
                    Inline::Escaped(EscapeChar::EnDash),
                    Inline::Escaped(EscapeChar::EmDash)
                ],
                Origin::new(1, "test")
            ),
            "Testing en dashes and em dashes"
        );

        let tag_symbols = parse_doc(r#"\*\^\_\/\\\=\~\|\:"#, "test");
        assert_eq!(
            tag_symbols[0],
            Block::Paragraph(
                vec![
                    Inline::Escaped(EscapeChar::Asterisk),
                    Inline::Escaped(EscapeChar::Caret),
                    Inline::Escaped(EscapeChar::Underscore),
                    Inline::Escaped(EscapeChar::ForwardSlash),
                    Inline::Escaped(EscapeChar::BackSlash),
                    Inline::Escaped(EscapeChar::Equal),
                    Inline::Escaped(EscapeChar::Tilde),
                    Inline::Escaped(EscapeChar::Bar),
                    Inline::Escaped(EscapeChar::Colon)
                ],
                Origin::new(1, "test")
            ),
            "Testing tag symbols"
        );

        let greek_letters = parse_doc(
            r#"
            \alpha\beta\gamma\Gamma\delta\Delta\epsilon\varepsilon\zeta\eta\theta\Theta\vartheta
            \iota\kappa\lambda\Lambda\mu\nu\xi\Xi\pi\Pi\rho\varrho\sigma\Sigma\tau\upsilon\Upsilon
            \phi\Phi\varphi\chi\psi\Psi\omega\Omega
        "#,
            "test",
        );

        assert_eq!(
            greek_letters[0]
                .to_string()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>(),
            "αβγΓδΔϵεζηθΘϑικλΛμνξΞπΠρϱσΣτυϒϕΦφχψΨωΩ".to_string(),
            "Testing greek letters"
        );
    }
}
