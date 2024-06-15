use domain::interfaces::record::RecordProvider;
use domain::{Record, RecordGrain};

use crate::tags::{lowercase_separated_by_dash, split_tags};

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
                "Tue, 12 Jun 2024 10:50:21 +0000".to_string(),
            ),
            ("Category".to_string(), "category1".to_string()),
            ("Title".to_string(), "Good title".to_string()),
            (
                "Body".to_string(),
                ["Body Line1", "Body line 2", "LONG LONG LONG "].join("\n+"),
            ),
            (
                "Tags".to_string(),
                lowercase_separated_by_dash(split_tags(vec![
                    "Tag 1, tag 2".to_string(),
                    "Another TAg".to_string(),
                ]))
                .join(", "),
            ),
        ];
        let mut fields: Vec<RecordGrain> = vec![];

        for (key, value) in fields_dto.iter() {
            fields.push(RecordGrain::new(key.clone(), value.clone()));
        }

        Record {
            record_type: "Link".to_string(),
            fields,
            fields_dto,
        }
    }
}
