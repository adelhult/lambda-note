use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, iter, str};

type Lines<'a> = iter::Peekable<str::Lines<'a>>;
pub type Metadata = HashMap<String, String>;

/// Parse a document into Blocks
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

        break;
    }

    (blocks, metadata)
}

/// Returns the next block and consumes the corresponding lines
/// Note: this function does not parse normal paragraph blocks,
/// that is done in the `parse_doc` function.
fn next_block(lines: &mut Lines, metadata: &mut Metadata) -> Option<Block> {
    parse_metadata(lines, metadata)
        .or_else(|| parse_heading(lines))
        .or_else(|| parse_divider(lines))
        .or_else(|| parse_list(lines))
        .or_else(|| parse_extension(lines))
}

fn parse_metadata(lines: &mut Lines, metadata: &mut Metadata) -> Option<Block> {
    let line = lines.peek()?;

    lazy_static! {
        static ref METADATA_RULE: Regex = Regex::new(r"^\s*?::\s*(\w+)\s*=\s*(.+)$").unwrap();
    }

    let captures = METADATA_RULE.captures(line)?;
    let key = captures.get(1)?.as_str().trim();
    let value = captures.get(2)?.as_str().trim();

    // consume the line
    lines.next();
    metadata.insert(key.into(), value.into());
    Some(Block::Metadata(key.into(), value.into()))
}

fn parse_divider(lines: &mut Lines) -> Option<Block> {
    lines.peek()?.trim_start().starts_with("===").then(|| {
        lines.next(); // consume the line
        Block::Divider
    })
}

// TODO
fn parse_list(lines: &mut Lines) -> Option<Block> {
    None
}

fn parse_extension(lines: &mut Lines) -> Option<Block> {
    lazy_static! {
        static ref EXTENSION_RULE: Regex =
            Regex::new(r"^\s*-{3,}\s*(?P<ident>\w+)\s*(?:,(?P<args>[^-]+))?-*\s*$").unwrap();
    }

    let line = lines.peek()?;
    let captures = EXTENSION_RULE.captures(line)?;
    let ident = captures.name("ident")?.as_str();

    // collect all of the potential arguments
    // in a hashmap
    let mut arguments = HashMap::new();

    if let Some(args) = captures.name("args") {
        let mut i = 0;

        for arg in args.as_str().split(",") {
            let values: Vec<&str> = arg.split("=").map(|s| s.trim()).collect();

            match values.len() {
                2 => {
                    arguments.insert(Key::Named(values[0].into()), values[1].into());
                }

                1 => {
                    arguments.insert(Key::Ordered(i), values[0].into());
                    i += 1;
                }

                _ => {
                    // TODO: maybe report an error instead of aborting the parsing,
                    // since it seems quite clear that the user actually is trying to create an
                    // extension block.
                    return None;
                }
            }
        }
    }

    // consume the first line of the block
    lines.next();

    // and than consume everything up until a line
    // that starts with "---"
    let mut contents = String::new();
    while let Some(line) = lines.next() {
        if line.trim_start().starts_with("---") {
            break;
        }
        contents.push_str(line);
    }

    Some(Block::Extension(ident.into(), arguments, contents))
}

fn parse_heading(lines: &mut Lines) -> Option<Block> {
    let line = lines.peek()?.trim_start();
    let level = line.chars().take_while(|c| *c == '#').count();

    if level < 1 {
        return None;
    }

    // TODO: replace with split_once, when it reaches stable
    let title = line
        .split("# ")
        .skip(1)
        .fold(String::new(), |acc, s| acc + s);

    if title.is_empty() {
        return None;
    }

    // consume the line
    lines.next();
    Some(Block::Heading(parse_inline(&title), level as u8))
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

/// TODO
fn parse_inline(s: &str) -> Vec<Inline> {
    vec![Inline::Text(s.into())]
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Ordered(u8),
    Named(String),
}

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
    Bold(Box<Inline>),
    Italic(Box<Inline>),
    Underline(Box<Inline>),
    Text(String),
    Extension(String, HashMap<String, String>),
    // literal / escaped,
    // should decide how i want to do this
    // i want to support stuff like \Lambda and perhaps auto-convert stuff like -- & ->, and other stuff
}
