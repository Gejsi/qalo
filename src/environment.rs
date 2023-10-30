use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::{EvalError, Object};

#[derive(Debug)]
pub struct Environment {
    pub store: HashMap<String, Object>,
    pub outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn get(&self, name: &str) -> Result<Object, EvalError> {
        if let Some(lit) = self.store.get(name) {
            Ok(lit.clone())
        } else if let Some(outer) = &self.outer {
            Ok(outer.borrow().get(name)?)
        } else {
            Err(EvalError::VariableNotFound(name.to_string()))
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name.to_string(), value);
    }
}
