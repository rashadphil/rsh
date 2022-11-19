use crate::types::primary::Value;
use derive_new::new;

#[derive(Debug, new)]
pub struct InStream {
    pub values: Value,
}
