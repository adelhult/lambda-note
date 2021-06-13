use std::{collections::HashMap, str};
use lazy_static::lazy_static;
use regex::Regex;
use super::{Metadata, Lines, Block, Key};

use super::inline::{parse_inline};


/// Returns the next block and consumes the corresponding lines
/// Note: this function does not parse normal paragraph blocks,
/// that is done in the `parse_doc` function.
pub fn next_block(lines: &mut Lines, metadata: &mut Metadata) -> Option<Block> {
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
    let arguments = parse_arguments(&captures)?;
    let ident = captures.name("ident")?.as_str();

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

/// Given a regex capture, collect all the arguments
/// into a hashmap or return None if we fail to parse the input
fn parse_arguments(captures: &regex::Captures) -> Option<HashMap<Key, String>> {
    let mut arguments = HashMap::new();
    let args = captures.name("args")?;

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
    Some(arguments)
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