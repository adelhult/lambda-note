use crate::{Translator, Html, DocumentState, Block, Inline, OutputFormat};
use regex::{Regex, Captures};
use std::collections::{HashSet, HashMap};

/// A Html translator that allows you to specify your own template
/// The string template is just an ordinary html document where
/// you can inject the 'top', 'content', 'bottom' and 'import' blocks as well
/// as any metadata fields using this syntax:
/// {{content}}
/// {{foo}} will add the value of the foo metadata field, or if not found an empty string
/// {{foo| no value found }} You can add a vertical bar and speciify the fallback value yourself.  
pub struct HtmlTemplate {
    html_translator: Html,
    template: String,
    preview: bool, // if true, js code to update the document
                   // via a message will be added 
}

impl HtmlTemplate {
    pub fn new(template: &str, include_preview_code: bool) -> Self {
        Self {
            html_translator: Html,
            template: template.to_string(),
            preview: include_preview_code,
        }
    }
}



impl Translator for HtmlTemplate {
    fn block(&self, state: &mut DocumentState, block: Block) -> Option<String> {
        self.html_translator.block(state, block)
    }

    fn inline(&self, inline: &Inline) -> String {
        self.html_translator.inline(inline)
    }

    fn template(
        &self,
        content: &str,
        top: &str,
        bottom: &str,
        imports: &HashSet<String>,
        metadata: &HashMap<String, String>,
    ) -> String {
        // a regex to match something on the form {{foo}} or {{foo|default}}
        let re = Regex::new(r"\{\{\s?(\w+)\s?(?:\|(.*))?\}\}").unwrap();

        // generate a string from the imports
        let mut imports_str = set_to_str(imports);
        if self.preview {
            imports_str.push_str(PREVIEW_MSG_LISTENER);
        }

        // replace all "template-tags"
        re.replace_all(&self.template, |caps: &Captures| {
            caps.get(1)
                .and_then(|m| match m.as_str() {
                    "top" => Some(top.to_string()),
                    "content" => Some(content.to_string()),
                    "bottom" => Some(bottom.to_string()),
                    "imports" =>Some(imports_str.clone()),
                    field => metadata.get(field).cloned(),
                })
                // falback to the specified default
                .or_else(|| caps.get(2).map(|m| m.as_str().to_string()))
                // or in the worst case replace the tag with an empty string
                .unwrap_or_else(|| String::from(""))
        }).to_string()
    }

    fn escape_str(&self, raw: &str) -> String {
        self.html_translator.escape_str(raw)
    }

    fn output_format(&self) -> OutputFormat {
        OutputFormat::Html
    }
}

/// helper to convert the import hashset to a String
fn set_to_str(set: &HashSet<String>) -> String {
    let mut s = String::new();
    for x in set {
        s.push_str(x);
    }
    s
}

const PREVIEW_MSG_LISTENER: &str = r#"
<script>
// any references to local files will be broken due to
// the browsers origin policy. To fix this, we host a server at
// localhost:5432, and use the following code updates all href and src
// tags in the page to point to the local server instead of directly to the local file.
document.addEventListener("DOMContentLoaded", function () {
    document.body.querySelectorAll('[href], [src]').forEach(node => {
        if (typeof node.href !== 'undefined') {
            if (node.href.startsWith('http')) return;
            node.href = 'http://localhost:5432/' + node.href.replace("file:///", "");
        }

        if (typeof node.src !== 'undefined') {
            if (node.src.startsWith('http')) return;
            node.src = 'http://localhost:5432/' + node.src.replace("file:///", "");
        }
    });
});
</script>
"#;
