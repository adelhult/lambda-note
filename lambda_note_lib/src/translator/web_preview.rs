use std::collections::{HashMap, HashSet};

use super::{Block, DocumentState, Html, Inline, OutputFormat, Translator};

/// A translator that transpiles into HTML code
/// but also adds an anchor tag corresponding to each line in the source file.
/// This makes it possible to "jump" from any given line in the source code and see how it will
/// render as output.
pub struct WebPreview {
    next_line: usize,
    translator: Html,
}

impl WebPreview {
    pub fn new() -> Self {
        WebPreview {
            next_line: 1,
            translator: Html,
        }
    }
}

impl Translator for WebPreview {
    fn output_format(&self) -> OutputFormat {
        self.translator.output_format()
    }

    fn block(&self, state: &mut DocumentState, block: Block) -> Option<String> {
        // use the underlying html translator, but prepend anchor tags to keep
        // track of what output the different lines in the source file(s) produced.
        let mut anchor_tags = String::new();

        for i in self.next_line..=block.get_line_number() {
            // FIXME: the document name needs to be included as well to avoid collisions
            anchor_tags.push_str(&format!("<a name=\"line-{}\">\n", i));
        }

        if let Some(html) = self.translator.block(state, block) {
            Some(format!(
                "{anchor_tags}{content}",
                anchor_tags = anchor_tags,
                content = html
            ))
        } else {
            None
        }
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
       self.translator.template(content, top, bottom, imports, metadata)
    }

    fn escape_str(&self, raw: &str) -> String {
        self.translator.escape_str(raw)
    }
}
