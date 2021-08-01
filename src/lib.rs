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
    use std::fs;
    #[test]
    fn work_in_progress() {
        let content =
            std::fs::read_to_string("test.ln").expect("Something went wrong reading the file");

        let blocks = parse_doc(&content);

        for block in blocks {
            println!("{}", block);
        }
    }

    #[test]
    fn translation_test() {
        let content = fs::read_to_string("test.ln").expect("Something went wrong reading the file");

        let mut doc_state = DocumentState::new(OutputFormat::Latex);
        let output = doc_state.translate(&content);

        fs::write("test.tex", output).expect("Unable to write file");
    }

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
