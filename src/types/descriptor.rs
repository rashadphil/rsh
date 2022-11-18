#[derive(Debug)]
pub struct Descriptor {
    pub name: String,
}

impl Descriptor {
    pub fn new(name: impl Into<String>) -> Self {
        Descriptor { name: name.into() }
    }
}
