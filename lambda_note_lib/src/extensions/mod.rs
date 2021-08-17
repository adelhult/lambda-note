mod foreign_extensions;
mod hidden;
mod img;
mod math;
mod maketitle;

use crate::translator::{DocumentState, OutputFormat};
use foreign_extensions::ForeignExtension;
use hidden::Hidden;
use img::Img;
use math::Math;
use maketitle::Maketitle;
use std::{collections::HashMap, rc::Rc};


#[derive(Debug, Clone, Copy)]
pub enum ExtensionVariant {
    Block,
    Inline,
}


pub trait Extension {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn version(&self) -> String;

    fn call(
        &self,
        args: Vec<String>,
        output_format: OutputFormat,
        variant: ExtensionVariant,
        state: &mut DocumentState
    ) -> Option<String>;
    fn supports_block(&self) -> bool;
    fn supports_inline(&self) -> bool;
    fn interests(&self) -> Vec<String>;
}

/// Returns a hashmap of all the native extensions
pub fn get_native_extensions() -> HashMap<String, Rc<dyn Extension>> {
    let mut map: HashMap<String, Rc<dyn Extension>> = HashMap::new();
    map.insert("hidden".to_string(), Rc::new(Hidden));
    map.insert("img".to_string(), Rc::new(Img));
    map.insert("math".to_string(), Rc::new(Math));
    map.insert("maketitle".to_string(), Rc::new(Maketitle));
    map
}