use super::{Block, DocumentState, Inline, OutputFormat, Tag, Translator};

/// A translator that transpiles into HTML code
pub struct Html;

impl Translator for Html {
    fn output_format(&self) -> OutputFormat {
        OutputFormat::Html
    }

    fn block(&self, state: &mut DocumentState, block: Block) -> Option<String> {
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

    fn inline(&self, inline: Inline) -> String {
        match inline {
            Inline::Begin(tag) => format!("<{}>", tag_to_string(&tag)),
            Inline::End(tag) => format!("</{}>", tag_to_string(&tag)),
            Inline::Escaped(escaped) => escaped.to_string(),
            Inline::Text(content) => self.escape_str(&content),
            _ => panic!("Failed to translate inline element {:?}", inline),
        }
    }

    fn boilerplate(&self, state: &mut DocumentState, content: &str) -> String {
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
        <link href="https://fonts.googleapis.com/css2?family=Open+Sans:ital,wght@0,400;0,700;1,400&display=swap" rel="stylesheet"> 
        <link href="https://fonts.googleapis.com/css2?family=Fira+Code&display=swap" rel="stylesheet"> 
            <style>
            html, body, h1, h2, h3, h4, h5, h6 {{
                font-family: 'Open Sans', sans-serif;
            }}

            .content {{
                margin-top:2rem;
                margin-bottom:4rem;
                margin-left:auto;
                margin-right:auto;
                max-width:700px;
            }}

            img {{
                max-width:100%;
                text-align:center;
            }}


            pre {{
                padding:0.8rem;
                font-family: 'Fira Code', monospace;
                box-sizing:border-box;
                font-size:0.9rem;
                border-radius:0.2rem;
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

    fn escape_str(&self, raw: &str) -> String {
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
