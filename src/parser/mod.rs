use std::{iter, str, collections::HashMap};

pub mod inline;
pub mod block;
use block::{next_block};
use inline::{parse_inline};



#[derive(Debug)]
pub enum Block {
    Heading(Vec<Inline>, u8),
    Paragraph(Vec<Inline>),
    Metadata(String, String),
    List(Vec<Inline>), // TODO, support ordered lists
    Divider,           // a section divider, i.e a new page
    Extension(String, HashMap<Key, String>, String),
}

#[derive(Debug)]
pub enum Inline {
    Text(String),
    Escaped(String),
    Italic(Vec<Inline>),
    Bold(Vec<Inline>),
    Underline(Vec<Inline>),
    Superscript(Vec<Inline>),
    Subscript(Vec<Inline>),
    Strikethrough(Vec<Inline>),
    Extension(String, HashMap<Key, String>)
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Ordered(u8),
    Named(String),
}

pub type Metadata = HashMap<String, String>;
type Lines<'a> = iter::Peekable<str::Lines<'a>>;

pub fn parse_doc(source: &str) -> (Vec<Block>, Metadata) {
    let mut metadata = HashMap::new();
    let mut lines = source.lines().peekable();
    let mut text: Vec<String> = vec![];
    let mut blocks = vec![];

    loop {
        if let Some(block) = next_block(&mut lines, &mut metadata) {
            // start of a new block -> empty the buffer
            if !text.is_empty() {
                blocks.append(&mut consume_text_buffer(&mut text));
            }

            // append the new block as well
            blocks.push(block);
            continue;
        }

        // if there are still lines left,
        // we assume its a normal line of text
        // and push it to the text buffer
        if let Some(line) = lines.next() {
            text.push(line.into());
            continue;
        }

        // finally, if the document is fully exhausted,
        // append the last paragraph and break the loop
        if !text.is_empty() {
            blocks.append(&mut consume_text_buffer(&mut text));
        }

        break (blocks, metadata);
    }
}

/// Consumes the text buffer and returns a list of paragraph blocks
fn consume_text_buffer(text: &mut Vec<String>) -> Vec<Block> {
    let paragraphs = text
        .join("\n")
        .split("\n\n")
        .map(|p| Block::Paragraph(parse_inline(p)))
        .collect();

    text.clear();

    paragraphs
}