use std::collections::HashMap;

use crate::object::Object;

#[derive(Debug)]
pub struct Environment {
    pub store: HashMap<String, Object>, // Object represents the evaluated values
    pub outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        self.store.get(name)
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name.to_string(), value);
    }
}
