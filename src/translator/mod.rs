mod html;
mod latex;

use super::{parse_doc, Block, Inline, Metadata, Tag};
use crate::extensions;
use extensions::{get_native_extensions, Extension, ExtensionVariant};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, rc::Rc};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum OutputFormat {
    LambdaNote,
    Html,
    Latex,
}

pub trait Translator {
    fn translate_block(state: &mut DocumentState, block: Block) -> Option<String>;
    fn translate_inline(inline: Inline) -> String;
    fn boilerplate(state: &mut DocumentState, content: &str) -> String;
    fn escape_str(raw: &str) -> String;
}

// TODO: maybe rename
pub struct DocumentState {
    metadata: HashMap<String, String>,
    extensions: HashMap<String, Rc<dyn Extension>>,
    warnings: Vec<String>,
    errors: Vec<String>,
    output_format: OutputFormat,
}

impl DocumentState {
    /// create a new empty document state
    pub fn new(output_format: OutputFormat) -> Self {
        // todo add some default extensions;
        DocumentState {
            metadata: HashMap::new(),
            extensions: get_native_extensions(),
            output_format,
            warnings: vec![],
            errors: vec![],
        }
    }

    /// Given the current document state translate the source text
    /// and mutate the state if a new extensions or metadata fields
    /// are found
    pub fn translate(&mut self, source: &str) -> String {
        let mut output = String::new();

        for block in parse_doc(source) {
            if let Some(s) = self.translate_block(block) {
                output.push_str(&format!("{}\n", s))
            }
        }

        match self.output_format {
            OutputFormat::Latex => latex::boilerplate(self, &output),
            OutputFormat::Html => html::boilerplate(self, &output),
            _ => panic!("Output format not supported"),
        }
    }

    /// translate an extension
    fn translate_extension(
        &mut self,
        symbol: &str,
        args: Vec<String>,
        variant: ExtensionVariant,
    ) -> Option<String> {
        let extension = self.extensions.get(symbol)?.clone();
        extension.call(args, self.output_format, variant, self) 
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
            _ => match self.output_format {
                // TODO: these should be compiled based on if a crate feature is enabled
                // we might not want to bloat the binary with a latex translator if we are
                // building a web viewer for instance
                OutputFormat::Html => html::translate_block(self, block),
                OutputFormat::Latex => latex::translate_block(self, block),
                _ => panic!(
                    "The output format {:?} is not supported",
                    self.output_format
                ),
            },
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

        return match self.output_format {
            OutputFormat::Html => html::translate_inline(inline),
            OutputFormat::Latex => latex::translate_inline(inline),
            _ => panic!(
                "The output format {:?} is not supported",
                self.output_format
            ),
        };
    }
}
