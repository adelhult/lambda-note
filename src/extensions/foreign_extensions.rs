use crate::extensions::{Extension, ExtensionVariant};
use crate::translator::{DocumentState, OutputFormat};
pub struct ForeignExtension {
    definition: String,
    name: String,
    version: String,
    description: String,
    interests: Vec<String>,
    block_support: bool,
    inline_support: bool,
}

impl Extension for ForeignExtension {
    fn call(
        &self,
        args: Vec<String>,
        output_format: OutputFormat,
        variant: ExtensionVariant,
        state: &mut DocumentState
    ) -> Option<String> {
        // todo use self.definition to
        todo!()
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn version(&self) -> String {
        self.version.clone()
    }
    fn description(&self) -> String {
        self.description.clone()
    }
    fn interests(&self) -> Vec<String> {
        self.interests.clone()
    }
    fn supports_block(&self) -> bool {
        self.block_support.clone()
    }
    fn supports_inline(&self) -> bool {
        self.inline_support.clone()
    }
}

// // kanske bättre med
// impl ForeignExtension {
//     fn new(definition: &str ) -> Self {

//     }
// }
