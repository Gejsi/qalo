use std::{collections::HashMap, rc::Rc};

use crate::object::{EvalError, Object};

#[derive(Debug, Clone)]
pub struct Environment {
    pub store: HashMap<String, Object>,
    pub outer: Option<Rc<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn get(&self, name: &str) -> Result<Object, EvalError> {
        match self.store.get(name) {
            Some(lit) => Ok(lit.clone()),
            None => {
                if let Some(outer) = &self.outer {
                    Ok(outer.get(name)?)
                } else {
                    Err(EvalError::VariableNotFound(name.to_string()))
                }
            }
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name.to_string(), value);
    }
}
