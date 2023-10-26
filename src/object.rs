// TODO: i don't really like this approach of handling objects.

#[derive(Debug)]
pub enum ObjectKind {
    Integer,
    Boolean,
}

trait Object {
    fn kind(&self) -> ObjectKind;
    fn inspect(&self) -> String;
}

#[derive(Debug)]
pub struct Integer(pub i32);

impl Object for Integer {
    fn kind(&self) -> ObjectKind {
        ObjectKind::Integer
    }

    fn inspect(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug)]
pub struct Boolean(pub bool);

impl Object for Boolean {
    fn kind(&self) -> ObjectKind {
        ObjectKind::Boolean
    }

    fn inspect(&self) -> String {
        self.0.to_string()
    }
}
