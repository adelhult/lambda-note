mod extension_manager;
mod parser;
mod translator;
mod extensions;

pub use extension_manager::greet;
pub use parser::{parse_doc, Block, EscapeChar, Inline, Metadata, Tag};
pub use translator::{DocumentState, OutputFormat};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_chars() {
        let dashes = parse_doc(r#"\endash\emdash"#);
        assert_eq!(
            dashes[0],
            Block::Paragraph(vec![
                Inline::Escaped(EscapeChar::EnDash),
                Inline::Escaped(EscapeChar::EmDash)
            ]),
            "Testing en dashes and em dashes"
        );

        let tag_symbols = parse_doc(r#"\*\^\_\/\\\=\~\|\:"#);
        assert_eq!(
            tag_symbols[0],
            Block::Paragraph(vec![
                Inline::Escaped(EscapeChar::Asterisk),
                Inline::Escaped(EscapeChar::Caret),
                Inline::Escaped(EscapeChar::Underscore),
                Inline::Escaped(EscapeChar::ForwardSlash),
                Inline::Escaped(EscapeChar::BackSlash),
                Inline::Escaped(EscapeChar::Equal),
                Inline::Escaped(EscapeChar::Tilde),
                Inline::Escaped(EscapeChar::Bar),
                Inline::Escaped(EscapeChar::Colon)
            ]),
            "Testing tag symbols"
        );

        let greek_letters = parse_doc(
            r#"
            \alpha\beta\gamma\Gamma\delta\Delta\epsilon\varepsilon\zeta\eta\theta\Theta\vartheta
            \iota\kappa\lambda\Lambda\mu\nu\xi\Xi\pi\Pi\rho\varrho\sigma\Sigma\tau\upsilon\Upsilon
            \phi\Phi\varphi\chi\psi\Psi\omega\Omega
        "#,
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
