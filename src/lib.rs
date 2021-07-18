mod extension_manager;
mod parser;
mod translator;

use std::io::{Read, Write};
use std::process::{Command, Stdio};

pub use extension_manager::greet;
pub use parser::{parse_doc, Block, EscapeChar, Inline, Metadata, Tag};
pub use translator::{translate, OutputFormat};

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn work_in_progress() {
        let content =
            std::fs::read_to_string("test.ln").expect("Something went wrong reading the file");

        let (blocks, metadata) = parse_doc(&content);

        println!("{:?}", metadata);

        for block in blocks {
            println!("{}", block);
        }
    }
    #[test]
    fn translation_test() {
        let content = fs::read_to_string("test.ln").expect("Something went wrong reading the file");

        let (blocks, _) = parse_doc(&content);
        let output = translate(blocks).unwrap();
        fs::write("test.html", output).expect("Unable to write file");
    }

    #[test]
    fn extension_test() {
        let process = Command::new("python")
            .arg("extension_test.py")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn extension");

        // Write a string to the `stdin` of `wc`.
        //
        // `stdin` has type `Option<ChildStdin>`, but since we know this instance
        // must have one, we can directly `unwrap` it.
        match process.stdin.unwrap().write_all(greet().as_bytes()) {
            Err(why) => panic!("couldn't write to extension stdin: {}", why),
            Ok(_) => println!("sent message"),
        }

        // Because `stdin` does not live after the above calls, it is `drop`ed,
        // and the pipe is closed.
        //
        // This is very important, otherwise `wc` wouldn't start processing the
        // input we just sent.

        // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
        let mut s = String::new();
        match process.stdout.unwrap().read_to_string(&mut s) {
            Err(why) => panic!("couldn't read wc stdout: {}", why),
            Ok(_) => print!("wc responded with:\n{}", s),
        }
    }

    #[test]
    fn escape_chars() {
        let (dashes, _) = parse_doc(r#"\endash\emdash"#);
        assert_eq!(
            dashes[0],
            Block::Paragraph(vec![
                Inline::Escaped(EscapeChar::EnDash),
                Inline::Escaped(EscapeChar::EmDash)
            ]),
            "Testing en dashes and em dashes"
        );

        let (tag_symbols, _) = parse_doc(r#"\*\^\_\/\\\=\~\|"#);
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
                Inline::Escaped(EscapeChar::Bar)
            ]),
            "Testing tag symbols"
        );

        let (greek_letters, _) = parse_doc(
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
