use serde::{Deserialize, Serialize};
use super::{Block, Inline, Tag};

#[derive(Serialize, Deserialize)]
pub enum OutputFormat {
    LambdaNote,
    Html,
    Latex,
}

pub fn translate(blocks: Vec<Block>) -> Option<String> {
    Some(
        blocks
            .into_iter()
            .map(|block| {
                let mut s = translate_block(block);
                s.push('\n');
                s
            })
            .collect(),
    )
}

fn translate_block(block: Block) -> String {
    match block {
        Block::Heading(inline, lvl) => {
            let level = if lvl > 6 { 6 } else { lvl };
            format!(
                "<h{level}>{text}</h{level}>",
                level = level,
                text = translate_inline(inline)
            )
        }
        Block::Divider => "<hr/>".to_string(),
        Block::Extension(ident, _, _) => format!("<extension {}>", ident), //Todo
        Block::Paragraph(content) => format!("<p>{}</p>", translate_inline(content)),
        _ => "".to_string(),
    }
}

fn translate_inline(text: Vec<Inline>) -> String {
    text.iter()
        .map(|el| {
            match el {
                Inline::Begin(tag) => format!("<{}>", tag_to_string(&tag)),
                Inline::End(tag) => format!("</{}>", tag_to_string(&tag)),
                Inline::Escaped(escaped) => escaped.to_string(),
                Inline::Text(content) => escape_str(content),
                // TODO: how do we handle extensions?
                Inline::Extension(ident, _) => format!("<extension {}", ident),
            }
        })
        .collect()
}

fn tag_to_string(tag: &Tag) -> String {
    match tag {
        &Tag::Bold => "strong",
        &Tag::Italic => "em",
        &Tag::Underline => "ins",
        &Tag::Superscript => "sup",
        &Tag::Subscript => "sub",
        &Tag::Strikethrough => "del",
    }
    .to_string()
}

fn escape_str(raw: &str) -> String {
    raw.chars()
        .map(|c| match c {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#39;".to_string(),
            c => c.to_string(),
        })
        .collect()
}
