use super::{Context, Extension, ExtensionVariant};
use evalexpr::*;

/// Calculation expressions
/// Evaluate expressions and small scripts
/// based on evalexpr crate.
pub struct Calc;

impl Extension for Calc {
    fn name(&self) -> String {
        String::from("Calc")
    }

    fn description(&self) -> String {
        "Evaluate arithmetic and logic expressions directly \n\
        in your document as it gets rendered.\n\n\
        Usage:\n\
        ```\n\
        ---- calc, [prefix, display]
        a = 20;
        a + 10
        ---- 
        ```\n\
        Or inline `|calc, 2 * 3 * math::sin(2), [prefix, display]|`.\n\
        **Prefix** is the text that simply will be prefixed before the output.\n\
        The third argument **display** is only allowed to be the string 'display'.\n\
        If it is included the entire expression will be outputed as a code block.\n\
        \n\n\
        Variables can be assigned values using the '=' operator. However,\n\
        these variables are only local to expression they are defined in.\n\
        If you want to have a global variable that you can use in multiple expressions,\n\
        do so by defining a metadata field prefixed with 'calc_'. Example:\n\
        ```\n\
        :: calc_coolness factor = 10\n\
        \n\
        Alanzo is this cool: |calc, coolness_factor * 3|. While\n\
        Alan has |calc, coolness_factor * 3.5| coolness points.\n\
        ```\n\
        **Note: variables defined as metadata fields are read only. If you reassign\n\
        their values it will only change the value in that expression.**\n\n\
        Calc uses the Rust library [evalexpr](https://github.com/ISibboI/evalexpr)\n\
        and you can learn more about the syntax and the available functions.\n\
        \n".to_string()
    }

    fn version(&self) -> String {
        String::from("1")
    }

    fn is_safe(&self) -> bool {
        true
    }

    fn call(&self, mut context: Context) -> Option<String> {
        let prefix = context.arguments.get(1).cloned();
        let expression = match context.arguments.get(0) {
            None => {
                self.add_error("No expression to evaluate was provided", &mut context);
                return None;
            }
            Some(text) => text,
        };

        // if a third argument is given and is "display" the entire
        // expression will be outputed as code block
        let display = if let Some(arg) = context.arguments.get(2) {
            arg.trim() == "display"
        } else {
            false
        };

        let mut output = String::new();

        if display {
            output.push_str(&context.document.translate_no_template(
                &format!("--------- code\n{}\n---------", expression),
                "calc expression",
            ))
        }

        let mut eval_context = HashMapContext::new();

        // first we get all the relevent metadata entries.
        // This is not really optimal since metadata is stored in a hashmap and
        // we will need to loop through the entire one to search for keys matching the
        // prefix "calc-".
        for (k, v) in &context.document.metadata {
            if !k.starts_with("calc_") {
                continue;
            }

            let (_, variable_name) = k.split_once("calc_").unwrap();

            // evaluate the value or abort if we can't
            let value = match eval(v) {
                Ok(value) => value,
                Err(_) => {
                    // FIXME: use self.add_error when it has been improved
                    // (should not take the entire context, only &mut document)
                    context.document.warnings.push(format!(
                        "Failed to parse the value in the meta data entry:\n :: {} = {}",
                        k, v
                    ));
                    continue;
                }
            };

            if eval_context.set_value(variable_name.into(), value).is_err() {
                context.document.warnings.push(format!(
                    "Failed to parse the value in the meta data entry:\n :: {} = {}",
                    k, v
                ));
            }
        }

        // actually evaluate the expression
        match eval_with_context_mut(expression, &mut eval_context) {
            Ok(value) => {
                let value_str = match value {
                    Value::Empty => "".into(),
                    v => v.to_string(),
                };

                let mut result = if let Some(prefix) = prefix {
                    format!("{} {}", prefix, value_str)
                } else {
                    value_str
                };
                
                // if we have a block extension we can translate the contents of the
                // prefix and result without causing issues too.
                if context.variant == ExtensionVariant::Block {
                    result = context.document.translate_no_template(&result, "calc extension");
                }

                output.push_str(&result);
            },
            Err(error) => {
                self.add_error(&error.to_string(), &mut context);
                return None;
            }
        };
        Some(output)
    }

    fn supports_block(&self) -> bool {
        true
    }

    fn supports_inline(&self) -> bool {
        true
    }

    fn interests(&self) -> Vec<String> {
        vec![String::from("calc_*")]
    }
}
