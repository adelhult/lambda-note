use super::{Block, DocumentState, Html, Inline, OutputFormat, Translator};
use std::collections::{HashMap, HashSet};

/// A translator that transpiles into HTML code
/// but also adds a message listener to reload the page as well as
/// an anchor tag corresponding to each line in the source file.
/// This makes it possible to "jump" from any given line in the source code and see how it will
/// render as output.
pub struct WebPreview {
    translator: Html,
}

impl WebPreview {
    pub fn new() -> Self {
        WebPreview {
            translator: Html,
        }
    }
}

impl Default for WebPreview {
    fn default() -> Self {
        Self::new()
    }
}

impl Translator for WebPreview {
    fn output_format(&self) -> OutputFormat {
        self.translator.output_format()
    }

    fn block(&self, state: &mut DocumentState, block: Block) -> Option<String> {
        let line_number = block.get_line_number();
        let html = self.translator.block(state, block)?;
        Some(format!("<a name=\"{}\"></a>\n{}", line_number, html))
    }

    fn inline(&self, inline: &Inline) -> String {
        self.translator.inline(inline)
    }

    fn template(
        &self,
        content: &str,
        top: &str,
        bottom: &str,
        imports: &HashSet<String>,
        metadata: &HashMap<String, String>,
    ) -> String {
        let mut import_str = String::new();
        for import in imports {
            import_str.push_str(import);
            import_str.push('\n');
        }

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
    <link href="https://fonts.googleapis.com/css2?family=Fira+Code&family=PT+Serif:ital,wght@0,400;0,700;1,400;1,700&family=Poppins:wght@700&display=swap" rel="stylesheet">
    <style>
        html {{
            scroll-behavior: smooth;
        }}
        
        *::-moz-selection, *::selection {{
            background: #E2705B;
            color:white;
        }}

        h1, h2, h3, h4, h5, h6 {{
            font-family: 'Poppins', sans-serif;
        }}

        h1 {{
            font-size: 1.7rem;
        }}

        h2 {{
            font-size: 1.5rem;
        }}

        body, p {{
            font-family: 'PT Serif', serif;
        }}

        .content {{
            box-sizing: border-box;
            padding:0.8rem;
            margin-top:2rem;
            margin-bottom:4rem;
            margin-left:auto;
            margin-right:auto;
            max-width:750px;
        }}

        img {{
            max-width:100%;
        }}

        pre {{
            border-radius:0.3rem;
            padding:0.8rem;
            font-family: 'Fira Code', monospace;
            box-sizing:border-box;
            font-size:0.9rem;
            overflow-x:auto;
        }}

        hr {{
            margin-top:2rem;
            margin-bottom:2rem;
        }}

        @media print {{
            hr {{ 
                page-break-after: always;
                visibility: hidden;
                margin:0;
                padding:0;
            }}

            .content {{
                padding:0;
            }}
        }}
    </style>
    <script>
        // listen for an event to reload the window
        window.addEventListener('message', event => {{ 
                // Data sent with postMessage is stored in event.data:
                console.log(event.data); 
                if (event.data === "reload")
                    window.location.reload()
        }}); 
    </script>
    <title>{title}</title>
</head>
<body>
    <div class="content">
{top}
{content}
{bottom}
</div>
</body>
</html>"#,
            imports = import_str,
            top = top,
            bottom = bottom,
            language = metadata.get("language").unwrap_or(&"en".to_string()),
            title = metadata.get("title").unwrap_or(&"Document".to_string()),
            content = content,
        )
    }

    fn escape_str(&self, raw: &str) -> String {
        self.translator.escape_str(raw)
    }
}
