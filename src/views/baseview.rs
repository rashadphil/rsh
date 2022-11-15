use derive_new::new;

use crate::types::primary::Value;

use super::{table::TableView, RenderView};

#[derive(new)]
pub struct BaseView<'a> {
    value: &'a Value,
}

impl RenderView for BaseView<'_> {
    fn render(&self) -> Vec<String> {
        match self.value {
            Value::List(l) => {
                let view = TableView::from_values(l);
                view.render()
            }
            Value::Object(o) => {
                vec![]
            }
            Value::Primitive(p) => {
                vec![]
            }
        }
    }
}
