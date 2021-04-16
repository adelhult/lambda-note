mod parser;

pub use parser::{parse_doc, Block, Inline, Metadata, parse_inline};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn work_in_progress() {
        let contents = std::fs::read_to_string("test.ln")
            .expect("Something went wrong reading the file");
        
        let (blocks, metadata) = parser::parse_doc(&contents);

        println!("{:?}", metadata);

        for block in blocks {
            println!("{:?}", block);
        }
    }

    #[test]
    fn inline_demo() {
        println!("{:?}", parse_inline(r"The \*/quick/ brown fox =jumps= ^over^ the +lazy+ _dog_ named*"));
    }
}


