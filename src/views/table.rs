use tabled::{builder::Builder, Style};

use crate::types::primary::Value;

use super::RenderView;

pub struct TableView {
    headers: Vec<String>,
    records: Vec<Vec<String>>,
}

impl TableView {
    pub fn from_values(values: &Vec<Value>) -> Self {
        let descriptors = &values[0].data_descriptors();

        let headers = descriptors.iter().map(|desc| desc.name.clone()).collect();
        let mut records = vec![];

        for value in values {
            let row: Vec<String> = descriptors
                .iter()
                .map(|desc| value.get_data(desc).format())
                .collect();
            records.push(row);
        }

        TableView { headers, records }
    }
}

impl RenderView for TableView {
    fn render(&self) -> Vec<String> {
        let mut builder = Builder::default();
        builder.set_columns(&self.headers);

        for record in &self.records {
            builder.add_record(record);
        }

        let mut table = builder.build();
        table.with(Style::modern());

        vec![table.to_string()]
    }
}
