use std::fs;

mod parser;
mod translator;

pub use parser::{parse_doc, Block, Inline, Metadata, Tag};
pub use translator::translate;

#[cfg(test)]
mod tests {
    use super::*;

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
    fn escape_chars() {
        // TODO: add a more complete test later.
        let (blocks, _) = parse_doc("\\alpha\\beta\\Lambda\\lambda\\Upsilon\\up!");
        assert_eq!(blocks[0].to_string().trim(), "αβΛλϒ↑!".to_string());
    }
}
