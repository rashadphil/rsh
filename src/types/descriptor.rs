use super::primary::{Any, Type};

#[derive(Debug)]
pub struct Descriptor {
    pub name: String,
    desc_type: Box<dyn Type>,
}

impl Descriptor {
    pub fn new(name: impl Into<String>) -> Self {
        Descriptor {
            name: name.into(),
            desc_type: Box::new(Any),
        }
    }
}
