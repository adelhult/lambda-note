use std::{collections::HashMap, fmt, iter, str};

pub mod block;
pub mod inline;

use block::next_block;
use inline::parse_inline;

#[derive(Debug)]
pub enum Block {
    Heading(Vec<Inline>, u8),
    Paragraph(Vec<Inline>),
    Metadata(String, String),
    List(Vec<Inline>), // TODO, support ordered lists
    Divider,           // a section divider, i.e a new page
    Extension(String, Vec<String>, String),
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Block::Heading(text, level) => format!(
                    "{} (lvl {})",
                    text.iter().map(|i| i.to_string()).collect::<String>(),
                    level
                ),
                Block::Paragraph(text) => format!(
                    "{}\n",
                    text.iter().map(|i| i.to_string()).collect::<String>()
                ),
                Block::Divider => "</divider>".to_string(),
                Block::Extension(name, args, contents) => format!(
                    "<{name}, {args:?}>\n{contents}\n</{name}>",
                    name = name,
                    args = args,
                    contents = contents
                ),
                _ => "".to_string(), // TODO: implement for the rest
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum Inline {
    Text(String),
    Escaped(EscapeChars),
    Begin(Tag),
    End(Tag),
    Extension(String, Vec<String>),
}

impl fmt::Display for Inline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Inline::Text(contents) => contents.clone(),
                Inline::Escaped(character) => character.to_string(),
                Inline::Begin(tag) => format!("<{}>", tag.to_string()),
                Inline::End(tag) => format!("</{}>", tag.to_string()),
                Inline::Extension(name, args) => format!("<{}, {:?}", name, args),
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum Tag {
    Italic,
    Bold,
    Underline,
    Superscript,
    Subscript,
    Strikethrough,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tag::Italic => "italic",
                Tag::Bold => "bold",
                Tag::Underline => "underline",
                Tag::Superscript => "super",
                Tag::Subscript => "sub",
                Tag::Strikethrough => "strikethrough",
            }
        )
    }
}

// special escape chars like greek letters
#[derive(Debug, PartialEq, Clone)]
pub enum EscapeChars {
    SmallLambda,
    BigLambda,
    SmallAlpha,
}

impl fmt::Display for EscapeChars {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EscapeChars::SmallLambda => "λ",
                EscapeChars::BigLambda => "Λ",
                EscapeChars::SmallAlpha => "α",
            }
        )
    }
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
