use std::borrow::Borrow;

use indexmap::IndexMap;

use super::{descriptor::Descriptor, primary::Value};

#[derive(Debug, Eq, PartialEq)]
pub struct DataDict {
    dict: IndexMap<String, Value>,
}

impl PartialOrd for DataDict {
    // Not used, needed to satisfy compiler
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

impl Ord for DataDict {
    // Not used, needed to satisfy compiler
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

impl DataDict {
    pub fn default() -> Self {
        let mut dict = IndexMap::new();
        dict.insert("none".to_string(), Value::none());
        DataDict { dict }
    }

    pub fn insert(&mut self, name: impl Into<String>, value: Value) {
        self.dict.insert(name.into(), value);
    }

    pub fn data_descriptors(&self) -> Vec<Descriptor> {
        self.dict
            .iter()
            .filter(|(_k, v)| v != &&Value::none())
            .map(|(name, _)| Descriptor::new(name))
            .collect()
    }

    pub fn get_data(&self, desc: &Descriptor) -> &Value {
        match self.dict.get(&desc.name) {
            Some(val) => val,
            None => panic!(),
        }
    }

    pub fn get_data_from_key(&self, key: impl Into<String>) -> &Value {
        match self.dict.get(&key.into()) {
            Some(val) => val,
            None => self.dict.get("none").unwrap(),
        }
    }
}
