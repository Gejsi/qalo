use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::{EvalError, Object};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Environment {
    pub store: HashMap<String, Object>,
    pub outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn get(&self, name: &str) -> Result<Object, EvalError> {
        if let Some(obj) = self.store.get(name) {
            Ok(obj.clone())
        } else if let Some(outer) = &self.outer {
            Ok(outer.borrow().get(name)?)
        } else {
            Err(EvalError::IdentifierNotFound(name.to_string()))
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }
}
