use std::collections::{HashMap, HashSet};

use parser::components::TemplateElement;
use serde_json::{json, Map, Value};

pub mod parser;

/// The Template state structure keeps track of the template file
/// and possible executions.
#[derive(Clone, Debug)]
pub struct Template {
    element: TemplateElement,
    arguments: Value,
    redirects: HashMap<String, Vec<String>>,
}

impl Template {
    /// Create a new empty and undeclared template file. This function
    /// is mostly useless.
    pub fn new() -> Self {
        Self {
            element: TemplateElement::Ignored,
            arguments: Value::Null,
            redirects: HashMap::new(),
        }
    }

    /// Generate a Template from a template file.
    pub fn from(template: &str) -> Option<Self> {
        Some(Self {
            element: TemplateElement::from_str(template)?,
            arguments: Value::Null,
            redirects: HashMap::new(),
        })
    }

    /// Set the template context.
    pub fn set_template(&mut self, value: TemplateElement) {
        self.element = value;
    }

    /// Set the argument context.
    pub fn set_context(&mut self, value: Value) {
        self.arguments = value;
    }

    /// Creates a new variable and adds it to the argument context, or if None,
    /// deletes a variable and all sub-variables from the tree.
    pub fn set_argument(&mut self, key: Vec<String>, value: Option<String>) -> bool {
        todo!()
    }

    /// Index the argument context and search for the variable.
    pub fn get_argument(&self, mut key: Vec<String>) -> Option<String> {
        if key.len() == 0 {
            return None;
        }

        let mut proof_cycles = HashSet::new();

        loop {
            let root = key.get(0).unwrap();

            if proof_cycles.contains(root) {
                return None;
            }

            if let Some(append) = self.redirects.get(key.get(0).unwrap()) {
                proof_cycles.insert(root.to_owned());

                let mut tmp = append.clone();
                tmp.append(&mut key);
                key = tmp;

                continue;
            }

            break;
        }

        let mut value = &self.arguments;

        for index in key.into_iter() {
            match value {
                Value::Null => return Some("".to_owned()),
                Value::String(s) => return Some(s.clone()),
                Value::Bool(b) => return if *b { Some("true".to_owned()) } else { None },
                Value::Number(i) => return Some(i.to_string()),
                Value::Object(map) => {
                    value = map.get(index.as_str())?;
                }
                Value::Array(vec) => {
                    let idx = index.parse::<usize>().ok()?;
                    value = vec.get(idx)?;
                }
            }
        }

        return None;
    }

    /// Overrides some variables
    pub fn override_arguments(&mut self) {
        if !self.arguments.is_object() {
            self.arguments = Value::Object(Map::new());
        }

        let arguments = self.arguments.as_object_mut().unwrap();
        
        // Set true and false as default boolean arguments.
        arguments.insert("true".to_owned(), Value::String("true".to_owned()));
        arguments.remove("false");

        // Sets the meta arguments: Version and version support flag.
        arguments.insert("meta".to_owned(), json!({
            "version": "v1",
            "v1": "true",
        }));
    }

    /// Create a string from the current template.
    pub fn generate(&self) -> Result<String, String> {
        match &self.element {
            TemplateElement::Ignored => Ok(String::from("")),
            TemplateElement::Variable(value) => {
                if let Some(v) = self.get_argument(value.clone()) {
                    Ok(v)
                } else {
                    Ok("".to_owned())
                }
            }
            TemplateElement::Requires(value) => {
                if self.get_argument(value.clone()).is_some() {
                    Ok("".to_owned())
                } else {
                    Err(format!(
                        "The (requiring) variable {} killed the scope.",
                        value.join(".")
                    ))
                }
            }
            TemplateElement::StringLiteral(string) => Ok(string.clone()),
            TemplateElement::Scope(scope) => {
                let mut output = String::from("");

                for element in scope {
                    let mut context = self.clone();
                    context.set_template(element.clone());

                    if let Ok(s) = context.generate() {
                        output += s.as_str();
                    } else {
                        break;
                    }
                }

                Ok(output)
            }
            TemplateElement::Foreach(foreach) => {
                let mut context = self.clone();
                context.redirects.insert(foreach.value.clone(), foreach.variable.clone());
                context.set_template(TemplateElement::Scope(foreach.scope.clone()));
                Ok(context.generate()?)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_template() {
        let template = Template::from("\"Hello!\"").unwrap();
        assert_eq!("Hello!", template.generate().unwrap().as_str());
    }
}
