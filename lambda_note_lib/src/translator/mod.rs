mod html;
mod latex;
mod utils;
mod web_preview;

use crate::extensions::{get_native_extensions, Context, Extension, ExtensionVariant};
use crate::{parse_doc, Block, Inline, Origin, Tag};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub use html::Html;
pub use latex::Latex;
pub use web_preview::WebPreview;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    LambdaNote,
    Html,
    Latex,
}

pub trait Translator {
    /// Translate a block, returns None if the block does not produce any output
    fn block(&self, state: &mut DocumentState, block: Block) -> Option<String>;

    /// Translate an inline element
    fn inline(&self, inline: &Inline) -> String;

    /// generate a boilerplate document given the content and the rest of the document state
    fn template(
        &self,
        content: &str,
        top: &str,
        bottom: &str,
        imports: &HashSet<String>,
        metadata: &HashMap<String, String>,
    ) -> String;

    /// escape a str to avoid conflicting with the output format
    fn escape_str(&self, raw: &str) -> String;

    fn output_format(&self) -> OutputFormat;
}
pub struct DocumentState {
    pub metadata: HashMap<String, String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    extensions: HashMap<String, Rc<dyn Extension>>,
    translator: Rc<dyn Translator>,
    imports: HashSet<String>,
    top: String,
    bottom: String,
}

impl<'a> DocumentState {
    /// create a new document state that
    /// only contains the prelude of native extensions
    pub fn new<T: 'static + Translator>(translator: T) -> Self {
        DocumentState {
            metadata: HashMap::new(),
            imports: HashSet::new(),
            top: String::new(),
            bottom: String::new(),
            extensions: get_native_extensions(),
            translator: Rc::new(translator),
            warnings: vec![],
            errors: vec![],
        }
    }

    pub fn import(&mut self, import: &str) {
        self.imports.insert(import.to_string());
    }

    /// Given the current document state translate the source text
    /// and mutate the state if a new extensions or metadata fields
    /// are found
    pub fn translate(&mut self, source: &str, doc_name: &str) -> String {
        let result = self.translate_no_template(source, doc_name);
        // TODO: the translator should not be cloned,
        // there is def. a better way to do this.
        self.translator.template(
            &result,
            &self.top,
            &self.bottom,
            &self.imports,
            &self.metadata,
        )
    }

    pub fn translate_no_template(&mut self, source: &str, doc_name: &str) -> String {
        let mut output = String::new();

        for block in parse_doc(source, doc_name) {
            if let Some(s) = self.translate_block(block) {
                output.push_str(&s);
                output.push('\n');
            }
        }

        output
    }

    /// translate an extension
    fn translate_extension(
        &mut self,
        symbol: &str,
        args: Vec<String>,
        variant: ExtensionVariant,
        origin: &Origin,
    ) -> Option<String> {
        match self.extensions.get(symbol).cloned() {
            None => {
                self.errors
                    .push(format!("No extension found with the name of {}", symbol));
                None
            }
            Some(extension) => extension.call(Context::new(args, variant, self, origin.clone())),
        }
    }

    /// Add a new metadata field to the document state
    fn add_metadata(&mut self, symbol: String, value: String) {
        self.metadata.insert(symbol, value);
    }

    /// Translate a block and return the translated text as an option
    fn translate_block(&mut self, block: Block) -> Option<String> {
        match block {
            Block::Extension(symbol, args, origin) => {
                self.translate_extension(&symbol, args, ExtensionVariant::Block, &origin)
            }
            Block::Metadata(symbol, value, _) => {
                self.add_metadata(symbol, value);
                None
            }
            // The translation of all other blocks will be delegated
            // to the translator for the current output format
            _ => self.translator.clone().block(self, block),
        }
    }

    fn translate_content(&mut self, block: &Block) -> String {
        let (text, origin) = match block {
            Block::Heading(text, _, origin) => (text, origin),
            Block::Paragraph(text, origin) => (text, origin),
            _ => panic!("Can not translate blocks without inline elements"),
        };
        text.into_iter()
            .map(|i| self.translate_inline(i, origin))
            .collect()
    }

    fn translate_inline(&mut self, inline: &Inline, origin: &Origin) -> String {
        if let Inline::Extension(symbol, args) = inline {
            return self
                .translate_extension(symbol, args.clone(), ExtensionVariant::Inline, origin)
                .unwrap_or_else(|| "".to_string());
        }

        self.translator.inline(inline)
    }

    pub fn get_output_format(&self) -> OutputFormat {
        self.translator.output_format()
    }
}
