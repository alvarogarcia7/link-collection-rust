use crate::tags::{lowercase_separated_by_dash, split_tags};
use domain::interfaces::record::RecordProvider;
use domain::Record;
use std::collections::HashMap;

#[derive(Default)]
pub struct HardcodedRecordProvider {}

impl HardcodedRecordProvider {}

impl RecordProvider for HardcodedRecordProvider {
    fn fetch(&mut self) -> Record {
        let fields_dto = vec![
            (
                "Id".to_string(),
                "a1a6925a-7958-11e8-a87f-0242ac110002".to_string(),
            ),
            (
                "Date".to_string(),
                "Tue, 26 Jun 2018 15:50:21 +0000".to_string(),
            ),
            ("Category".to_string(), "category1".to_string()),
            ("Title".to_string(), "Good title".to_string()),
            ("Body".to_string(), ["Body Line1", "Body line 2"].join("\n")),
            (
                "Tags".to_string(),
                lowercase_separated_by_dash(split_tags(vec![
                    "Tag 1, tag 2".to_string(),
                    "Another TAg".to_string(),
                ]))
                .join(", "),
            ),
        ];
        let mut fields: HashMap<String, String> = HashMap::with_capacity(fields_dto.len());

        for (key, value) in fields_dto.iter() {
            fields.insert(key.clone(), value.clone());
        }

        Record {
            record_type: "Link".to_string(),
            fields,
            fields_dto,
        }
    }
}
