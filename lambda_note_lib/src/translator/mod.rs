mod html;
mod latex;

use crate::extensions::{get_native_extensions, Extension, ExtensionVariant};
use crate::{parse_doc, Block, Inline, Tag};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub use html::Html;
pub use latex::Latex;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum OutputFormat {
    LambdaNote,
    Html,
    Latex,
}

pub trait Translator {
    /// Translate a block, returns None if the block does not produce any output
    fn block(&self, state: &mut DocumentState, block: Block) -> Option<String>;

    /// Translate an inline element
    fn inline(&self, inline: Inline) -> String;

    /// generate a boilerplate document given the content and the rest of the document state
    fn boilerplate(&self, state: &mut DocumentState, content: &str) -> String;

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

    pub fn add_warning(&mut self, warning: &str) {
        self.warnings.push(warning.to_string());
    }

    pub fn add_error(&mut self, error: &str) {
        self.errors.push(error.to_string());
    }

    pub fn import(&mut self, import: &str) {
        self.imports.insert(import.to_string());
    }

    /// Given the current document state translate the source text
    /// and mutate the state if a new extensions or metadata fields
    /// are found
    pub fn translate(&mut self, source: &str) -> String {
        self.translator.clone().boilerplate(self, self.translate_no_boilerplate(source))
    }

    pub fn translate_no_boilerplate(&mut self, source &str) -> String {
        let mut output = String::new();
        
        for block in parse_doc(source) {
            if let Some(s) = self.translate_block(block) {
                output.push_str(&format!("{}\n", s))
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
    ) -> Option<String> {
        let extension = self.extensions.get(symbol)?.clone();
        extension.call(args, self.translator.output_format(), variant, self)
        // TODO: handle errors, and is rc really the right choice?
    }

    /// Add a new metadata field to the document state
    fn add_metadata(&mut self, symbol: String, value: String) {
        self.metadata.insert(symbol, value);
    }

    /// Translate a block and return the translated text as an option
    fn translate_block(&mut self, block: Block) -> Option<String> {
        match block {
            Block::Extension(symbol, args) => {
                self.translate_extension(&symbol, args, ExtensionVariant::Block)
            }
            Block::Metadata(symbol, value) => {
                self.add_metadata(symbol, value);
                None
            }
            // The translation of all other blocks will be delegated
            // to the translator for the current output format
            _ => self.translator.clone().block(self, block),
        }
    }

    fn translate_text(&mut self, text: Vec<Inline>) -> String {
        text.into_iter().map(|i| self.translate_inline(i)).collect()
    }

    fn translate_inline(&mut self, inline: Inline) -> String {
        if let Inline::Extension(symbol, args) = inline {
            return self
                .translate_extension(&symbol, args, ExtensionVariant::Inline)
                .unwrap_or("".to_string());
        }

        self.translator.inline(inline)
    }
}
