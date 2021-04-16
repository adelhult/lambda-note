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

        break (blocks, metadata);
    }
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

/// Parse a string slice into a vector of
/// all the inline elements found inside.
pub fn parse_inline<'a>(source: &'a str) -> Vec<Inline> {
    if source.is_empty() {
        return vec![];
    }

    lazy_static! {
        static ref INLINE_RULE: Regex = Regex::new(
            r"(?x)
            # Literals and escape characters
            (?P<escape>\\(?:
                /|\*|=|\^|\+|_|
                Lambda|lambda
                |Alpha|alpha
            ))
            
            # Typography tags
            | /(?P<italic>\S|\S.*?\S)/
            | \*(?P<bold>\S|\S.*?\S)\*
            | =(?P<underline>\S|\S.*?\S)=
            | \^(?P<superscript>\S|\S.*?\S)\^
            | _(?P<subscript>\S|\S.*?\S)_
            | \+(?P<strikethrough>\S|\S.*?\S)\+
            
            # Extension
            | \|\s*(P?<ident>\w+?)\s*(?:,(?P<args>[^|]+))*\|
        ",
        )
        .unwrap();
        static ref INLINE_EXTENSION: Regex =
            Regex::new(r"\|\s*(P?<ident>\w+?)\s*(?:,(?P<args>[^|]+))*\|").unwrap();
    }

    if let Some(m) = INLINE_RULE.find(source) {
        let mut result = vec![];

        let preceding_text = &source[..m.start()];
        let symbol = &source[m.start()..m.start() + 1];

        // add the preceding text
        if !preceding_text.is_empty() {
            result.push(Inline::Text(preceding_text.into()));
        }

        // calc the remainder slice
        let remainder = if m.end() >= source.len() {
            ""
        } else if symbol == r"\" {
            // escaped items does not have a end tag
            &source[m.end()..]
        } else {
            &source[(m.end() + 1)..]
        };

        // helper function: trim the ends of matches
        let trim = |s: &'a str, m: regex::Match, left, right| -> &'a str {
            &s[(m.start() + left)..(m.end() - right)]
        };

        // avoids boilerplate code when returning tags
        macro_rules! tag {
            ($i:ident) => {
                vec![Inline::$i(parse_inline(trim(source, m, 1, 1)))]
            };
        }

        // parse the children and add the node to the result vector
        result.append(&mut match symbol {
            "*" => tag!(Bold),
            "/" => tag!(Italic),
            "=" => tag!(Underline),
            "^" => tag!(Superscript),
            "_" => tag!(Subscript),
            "+" => tag!(Strikethrough),
            r"\" => vec![Inline::Escaped(trim(source, m, 1, 0).into())],
            "|" => {
                // This is an extension. Not  the cleanest code,
                // but we will just capture the groups with a repeated regex match
                let contents = trim(source, m, 1, 1);
                vec![] // todo
            }
            _ => panic!("inline regex error, unknown tag symbol!"),
        });

        // parse the remainder
        result.append(&mut parse_inline(remainder));

        result
    } else {
        vec![Inline::Text(source.into())]
    }
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