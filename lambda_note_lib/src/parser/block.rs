use super::{inline::parse_inline, Block, Lines, Origin};
use lazy_static::lazy_static;
use regex::Regex;

/// Returns the next block and consumes the corresponding lines
/// Note: this function does not parse normal paragraph blocks,
/// that is done in the `parse_doc` function.
pub fn next_block(lines: &mut Lines, doc_name: &str) -> Option<Block> {
    parse_extension(lines, doc_name)
        .or_else(|| parse_metadata(lines, doc_name))
        .or_else(|| parse_heading(lines, doc_name))
        .or_else(|| parse_divider(lines, doc_name))
        .or_else(|| parse_list(lines, doc_name))
}

fn parse_metadata(lines: &mut Lines, doc_name: &str) -> Option<Block> {
    let (line, _) = lines.peek()?;

    lazy_static! {
        static ref METADATA_RULE: Regex = Regex::new(r"^\s*?::\s*(\w+)\s*=\s*(.+)$").unwrap();
    }

    let captures = METADATA_RULE.captures(line)?;
    let key = captures.get(1)?.as_str().trim();
    let value = captures.get(2)?.as_str().trim();

    // consume the line and return the block
    let (_, line_number) = lines.next()?;
    Some(Block::Metadata(
        key.into(),
        value.into(),
        Origin::new(line_number, doc_name),
    ))
}

fn parse_divider(lines: &mut Lines, doc_name: &str) -> Option<Block> {
    let (line, line_number) = lines.peek()?;
    let line_number = *line_number;

    line.trim_start().starts_with("===").then(|| {
        lines.next(); // consume the line
        Block::Divider(Origin::new(line_number, doc_name))
    })
}

// TODO
fn parse_list(_lines: &mut Lines, _: &str) -> Option<Block> {
    None
}

fn parse_extension(lines: &mut Lines, doc_name: &str) -> Option<Block> {
    lazy_static! {
        static ref RULE: Regex =
            Regex::new(r"^\s*(?P<div>-{3,})\s*(?P<ident>\w+)\s*(?:,(?P<args>[^-]+))?-*\s*$")
                .unwrap();
    }
    let (line, line_number) = lines.peek()?;
    let line_number = *line_number;

    let captures = RULE.captures(line)?;
    let divider_length = captures.name("div")?.as_str().chars().count();
    let ident = captures.name("ident")?.as_str().trim().to_string();

    // collect the arguments into a Vec<String>
    let mut arguments = captures.name("args").map_or_else(Vec::new, |s| {
        s.as_str()
            .split(',')
            .map(|arg| arg.trim().to_string())
            .collect()
    });

    // consume the first line of the block
    lines.next();

    // and than consume everything up until a line
    // that starts with n number of "-" in succession
    let end_prefix = "-".repeat(divider_length);
    let contents = lines
        .map(|(l, _)| l) // remove the line numbers
        .take_while(|line| !line.trim_start().starts_with(&end_prefix))
        .collect::<Vec<&str>>()
        .join("\n");

    // the first argument will be the main content of ,the block
    arguments.insert(0, contents);

    Some(Block::Extension(ident, arguments, Origin::new(line_number, doc_name)))
}

fn parse_heading(lines: &mut Lines, doc_name: &str) -> Option<Block> {
    let (line, _) = lines.peek()?;
    let line = line.trim_start();
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
    let (_, line_number) = lines.next()?;
    Some(Block::Heading(
        parse_inline(&title),
        level as u8,
        Origin::new(line_number, doc_name),
    ))
}
