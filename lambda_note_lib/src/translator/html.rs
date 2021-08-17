use super::{Block, DocumentState, Inline, Metadata, Tag};

pub fn translate_block(state: &mut DocumentState, block: Block) -> Option<String> {
    match block {
        Block::Heading(text, lvl) => {
            let level = if lvl > 6 { 6 } else { lvl };
            Some(format!(
                "<h{level}>{text}</h{level}>",
                level = level,
                text = state.translate_text(text)
            ))
        }
        Block::Divider => Some("<hr/>".to_string()),
        Block::Paragraph(text) => Some(format!("<p>{}</p>", state.translate_text(text))),
        _ => None,
    }
}

pub fn translate_inline(inline: Inline) -> String {
    match inline {
        Inline::Begin(tag) => format!("<{}>", tag_to_string(&tag)),
        Inline::End(tag) => format!("</{}>", tag_to_string(&tag)),
        Inline::Escaped(escaped) => escaped.to_string(),
        Inline::Text(content) => escape_str(&content),
        _ => panic!("Failed to translate inline element {:?}", inline),
    }
}

pub fn boilerplate(state: &mut DocumentState, content: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html lang="{language}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    {imports}
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;700&display=swap" rel="stylesheet"> 
    <style>
        html, body, h1,h2,h3,h4,h5,h6 {{
            font-family: 'Inter', sans-serif;
        }}
        .content {{
            margin-left:auto;
            margin-right:auto;
            max-width:1000px;
        }}
        
        img {{
            max-width:100%;
            text-align:center;
        }}
    </style>
    <title>{title}</title>
</head>
<body>
    <div class="content">
        {top}
        {content}
        {bottom}
    </div>
</body>
</html>
"#,
        imports = state
            .imports
            .iter()
            .fold(String::new(), |acc, s| acc + s + "\n"),
        top = state.top,
        bottom = state.bottom,
        language = state.metadata.get("language").unwrap_or(&"en".to_string()),
        title = state
            .metadata
            .get("title")
            .unwrap_or(&"Document".to_string()),
        content = content
    )
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
