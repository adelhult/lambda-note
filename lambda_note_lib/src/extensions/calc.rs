use super::{Context, Extension};
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
        // TODO: write a desc.
        String::from("Calc - calculation expressions")
    }

    fn version(&self) -> String {
        String::from("1")
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
            Ok(value) => output.push_str(&context.document.translate_no_template(
                {
                    let value_str = match value {
                        Value::Empty => "".into(),
                        v => v.to_string(),
                    };

                    &if let Some(prefix) = prefix {
                        format!("{} {}", prefix, value_str)
                    } else {
                        value_str
                    }
                },
                "calc expression",
            )),
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
        vec![String::from("calc-*")]
    }
}
