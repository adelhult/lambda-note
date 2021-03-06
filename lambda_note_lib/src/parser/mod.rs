use std::{
    fmt,
    iter::{self, Zip},
    ops::RangeFrom,
    str,
};

pub mod block;
pub mod inline;

use block::next_block;
use inline::parse_inline;

type LineNumber = usize;

/// Describes the origin of a block
/// i.e the line number and the document name or the name of
/// the extension macro that created it
#[derive(Debug, PartialEq, Clone)]
pub struct Origin {
    pub line_number: usize,
    pub name: OriginName,
}

/// source texts can, apart from files, come from macro expansions inside
/// of extensions, so we would need to keep track of who spawned them
#[derive(Debug, PartialEq, Clone)]
pub enum OriginName {
    Filename(String),
    Expansion(Box<OriginName>),
}

impl Origin {
    pub fn new(line_number: usize, document_name: &str) -> Self {
        Origin {
            line_number,
            name: OriginName::Filename(document_name.to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Block {
    Heading(Vec<Inline>, u8, Origin),
    Paragraph(Vec<Inline>, Origin),
    Metadata(String, String, Origin),
    List(Vec<Inline>, Origin), // TODO, support ordered lists
    Divider(Origin),           // a section divider, i.e a new page
    Extension(String, Vec<String>, Origin),
}

impl Block {
    pub fn get_line_number(&self) -> LineNumber {
        match self {
            Block::Heading(_, _, origin) => origin,
            Block::Paragraph(_, origin) => origin,
            Block::Metadata(_, _, origin) => origin,
            Block::List(_, origin) => origin,
            Block::Divider(origin) => origin,
            Block::Extension(_, _, origin) => origin,
        }
        .line_number
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Block::Heading(text, level, _) => format!(
                    "{} (lvl {})",
                    text.iter().map(|i| i.to_string()).collect::<String>(),
                    level
                ),
                Block::Paragraph(text, _) => format!(
                    "{}\n",
                    text.iter().map(|i| i.to_string()).collect::<String>()
                ),
                Block::Divider(_) => "</divider>".to_string(),
                Block::Extension(name, args, _) =>
                    format!("<{name}, {args:?}>\n</{name}>", name = name, args = args),
                _ => "".to_string(), // TODO: implement for the rest
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Inline {
    Text(String),
    Escaped(EscapeChar),
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
                Inline::Extension(name, args) => format!("<{}, {:?}/>", name, args),
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
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
pub enum EscapeChar {
    // greek letters:
    Alpha,
    Beta,
    GammaLower,
    GammaUpper,
    DeltaLower,
    DeltaUpper,
    Epsilon,
    EpsilonVar,
    Zeta,
    Eta,
    ThetaLower,
    ThetaUpper,
    ThetaVar,
    Iota,
    Kappa,
    LambdaLower,
    LambdaUpper,
    Mu,
    Nu,
    XiLower,
    XiUpper,
    PiLower,
    PiUpper,
    Rho,
    RhoVar,
    SigmaLower,
    SigmaUpper,
    Tau,
    UpsilonLower,
    UpsilonUpper,
    PhiLower,
    PhiUpper,
    PhiVar,
    Chi,
    PsiLower,
    PsiUpper,
    OmegaLower,
    OmegaUpper,
    // dashes:
    EmDash,
    EnDash,
    //arrows:
    LeftThin,
    LeftBold,
    RightThin,
    RightBold,
    UpThin,
    UpBold,
    DownThin,
    DownBold,
    // escaping lambda note syntax
    Asterisk,
    Caret,
    Underscore,
    ForwardSlash,
    BackSlash,
    Equal,
    Tilde,
    Bar,
    Colon,
    TableFlip,
}

// unicode mappings of the special escape characters
impl fmt::Display for EscapeChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // greek letters:
                EscapeChar::Alpha => "??",
                EscapeChar::Beta => "??",
                EscapeChar::GammaLower => "??",
                EscapeChar::GammaUpper => "??",
                EscapeChar::DeltaLower => "??",
                EscapeChar::DeltaUpper => "??",
                EscapeChar::Epsilon => "??",
                EscapeChar::EpsilonVar => "??",
                EscapeChar::Zeta => "??",
                EscapeChar::Eta => "??",
                EscapeChar::ThetaLower => "??",
                EscapeChar::ThetaUpper => "??",
                EscapeChar::ThetaVar => "??",
                EscapeChar::Iota => "??",
                EscapeChar::Kappa => "??",
                EscapeChar::LambdaLower => "??",
                EscapeChar::LambdaUpper => "??",
                EscapeChar::Mu => "??",
                EscapeChar::Nu => "??",
                EscapeChar::XiLower => "??",
                EscapeChar::XiUpper => "??",
                EscapeChar::PiLower => "??",
                EscapeChar::PiUpper => "??",
                EscapeChar::Rho => "??",
                EscapeChar::RhoVar => "??",
                EscapeChar::SigmaLower => "??",
                EscapeChar::SigmaUpper => "??",
                EscapeChar::Tau => "??",
                EscapeChar::UpsilonLower => "??",
                EscapeChar::UpsilonUpper => "??",
                EscapeChar::PhiLower => "??",
                EscapeChar::PhiUpper => "??",
                EscapeChar::PhiVar => "??",
                EscapeChar::Chi => "??",
                EscapeChar::PsiLower => "??",
                EscapeChar::PsiUpper => "??",
                EscapeChar::OmegaLower => "??",
                EscapeChar::OmegaUpper => "??",
                // dashes:
                EscapeChar::EmDash => "???",
                EscapeChar::EnDash => "???",
                //arrows:
                EscapeChar::RightThin => "???",
                EscapeChar::RightBold => "???",
                EscapeChar::LeftThin => "???",
                EscapeChar::LeftBold => "???",
                EscapeChar::UpThin => "???",
                EscapeChar::UpBold => "???",
                EscapeChar::DownThin => "???",
                EscapeChar::DownBold => "???",
                // escaping lambda note syntax
                EscapeChar::Asterisk => "*",
                EscapeChar::Caret => "^",
                EscapeChar::Underscore => "_",
                EscapeChar::ForwardSlash => "/",
                EscapeChar::BackSlash => "\\",
                EscapeChar::Equal => "=",
                EscapeChar::Tilde => "~",
                EscapeChar::Bar => "|",
                EscapeChar::Colon => ":",
                EscapeChar::TableFlip => "(??????????????????? ?????????",
            }
        )
    }
}

type Lines<'a> = iter::Peekable<Zip<str::Lines<'a>, RangeFrom<LineNumber>>>;

pub fn parse_doc(source: &str, doc_name: &str) -> Vec<Block> {
    let mut lines = source.lines().zip(1..).peekable();
    let mut text: Vec<(String, LineNumber)> = vec![];
    let mut blocks = vec![];

    loop {
        if let Some(block) = next_block(&mut lines, doc_name) {
            // start of a new block -> empty the buffer
            if !text.is_empty() {
                blocks.append(&mut consume_text_buffer(&mut text, doc_name));
            }

            // append the new block as well
            blocks.push(block);
            continue;
        }

        // if there are still lines left,
        // we assume its a normal line of text
        // and push it to the text buffer
        if let Some((line, line_number)) = lines.next() {
            text.push((line.into(), line_number));
            continue;
        }

        // finally, if the document is fully exhausted,
        // append the last paragraph and break the loop
        if !text.is_empty() {
            blocks.append(&mut consume_text_buffer(&mut text, doc_name));
        }

        break blocks;
    }
}

/// Consumes the text buffer and returns a list of paragraph blocks
fn consume_text_buffer(text: &mut Vec<(String, LineNumber)>, doc_name: &str) -> Vec<Block> {
    let paragraphs = text
        .split(|(s, _)| s.trim().is_empty()) // get each paragraph
        .filter_map(|lines| {
            // get the line number of the start of the paragraph
            let (_, line_number) = lines.get(0)?;

            // remove the line numbers join into a string
            let text = lines
                .iter()
                .map(|(l, _)| l.clone())
                .collect::<Vec<String>>()
                .join("\n");

            let block = Block::Paragraph(parse_inline(&text), Origin::new(*line_number, doc_name));
            // remove empty paragraph blocks
            if is_empty(&block) {
                None
            } else {
                Some(block)
            }
        })
        .collect();

    // empty the buffer
    text.clear();

    paragraphs
}

/// returns true if the paragraph contains any elements
/// otherwise false.
fn is_empty(block: &Block) -> bool {
    if let Block::Paragraph(text, _) = block {
        text.is_empty()
    } else {
        false
    }
}
