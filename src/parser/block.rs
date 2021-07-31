use super::{inline::parse_inline, Block, Lines, Metadata};
use lazy_static::lazy_static;
use regex::Regex;

/// Returns the next block and consumes the corresponding lines
/// Note: this function does not parse normal paragraph blocks,
/// that is done in the `parse_doc` function.
pub fn next_block(lines: &mut Lines) -> Option<Block> {
    parse_extension(lines)
        .or_else(|| parse_metadata(lines))
        .or_else(|| parse_heading(lines))
        .or_else(|| parse_divider(lines))
        .or_else(|| parse_list(lines))
}

fn parse_metadata(lines: &mut Lines) -> Option<Block> {
    let line = lines.peek()?;

    lazy_static! {
        static ref METADATA_RULE: Regex = Regex::new(r"^\s*?::\s*(\w+)\s*=\s*(.+)$").unwrap();
    }

    let captures = METADATA_RULE.captures(line)?;
    let key = captures.get(1)?.as_str().trim();
    let value = captures.get(2)?.as_str().trim();

    // consume the line
    lines.next();
    Some(Block::Metadata(key.into(), value.into()))
}

fn parse_divider(lines: &mut Lines) -> Option<Block> {
    lines.peek()?.trim_start().starts_with("===").then(|| {
        lines.next(); // consume the line
        Block::Divider
    })
}

// TODO
fn parse_list(_lines: &mut Lines) -> Option<Block> {
    None
}

fn parse_extension(lines: &mut Lines) -> Option<Block> {
    lazy_static! {
        static ref RULE: Regex =
            Regex::new(r"^\s*(?P<div>-{3,})\s*(?P<ident>\w+)\s*(?:,(?P<args>[^-]+))?-*\s*$")
                .unwrap();
    }
    let line = lines.peek()?;
    let captures = RULE.captures(line)?;
    let divider_length = captures.name("div")?.as_str().chars().count();
    let ident = captures.name("ident")?.as_str().to_string();

    // collect the arguments into a Vec<String>
    let mut arguments = captures.name("args").map_or_else(
        || Vec::new(),
        |s| {
            s.as_str()
                .split(",")
                .map(|arg| arg.trim().to_string())
                .collect()
        }
    );

    // consume the first line of the block
    lines.next();

    // and than consume everything up until a line
    // that starts with n number of "-" in succession
    let end_prefix = "-".repeat(divider_length);
    let contents = lines
        .take_while(|line| !line.trim_start().starts_with(&end_prefix))
        .collect::<Vec<&str>>()
        .join("\n");   
    
    // the first argument will be the main content of the block
    arguments.insert(0, contents);

    Some(Block::Extension(ident, arguments))
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
