use core::fmt::Debug;

pub trait RshObject: Debug {}

#[derive(Debug)]
pub enum Value {
    Object(Box<dyn RshObject>),
    List(Vec<Value>),
    String(String),
}

impl Value {
    pub fn object(value: impl RshObject + 'static) -> Self {
        Value::Object(Box::new(value))
    }

    pub fn list(values: impl Into<Vec<Value>>) -> Self {
        Value::List(values.into())
    }

    pub fn string(string: impl Into<String>) -> Self {
        Value::String(string.into())
    }
}
