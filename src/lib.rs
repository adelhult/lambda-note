mod parser;

pub use parser::{parse_doc, Block, Inline, Metadata};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_in_progress() {
        let contents =
            std::fs::read_to_string("test.ln").expect("Something went wrong reading the file");

        let (blocks, metadata) = parse_doc(&contents);

        println!("{:?}", metadata);

        for block in blocks {
            println!("{}", block);
        }
    }

    #[test]
    fn escape_chars() {
        // TODO: add a more complete test later.
        let (blocks, _) = parse_doc("\\alpha\\beta\\Lambda\\lambda\\Upsilon\\up!");
        assert_eq!(blocks[0].to_string().trim(), "αβΛλϒ↑!".to_string());
    }
}
