use domain::interfaces::record::RecordProvider;
use domain::interfaces::RecordProviderError;
use domain::{Record, RecordGrain, Tags};

#[derive(Default)]
pub struct HardcodedRecordProvider {}

impl HardcodedRecordProvider {}

impl RecordProvider for HardcodedRecordProvider {
    fn fetch(&mut self) -> Result<(Record, Vec<String>), RecordProviderError> {
        let field_values = vec![
            (
                "Id".to_string(),
                "a1a6925a-7958-11e8-a87f-0242ac110002".to_string(),
            ),
            (
                "Date".to_string(),
                "Tue, 12 Jun 2024 10:50:21 +0000".to_string(),
            ),
            (
                "Link".to_string(),
                "http://example.com/article/blog-1".to_string(),
            ),
            ("Category".to_string(), "category1".to_string()),
            ("Title".to_string(), "Good title".to_string()),
            (
                "Body".to_string(),
                ["Body Line1", "Body line 2", "LONG LONG LONG "].join("\n+ "),
            ),
            (
                "Tags".to_string(),
                Tags::import(vec!["Tag 1, tag 2".to_string(), "Another TAg".to_string()])
                    .values
                    .join(", "),
            ),
        ];
        let mut fields: Vec<RecordGrain> = vec![];

        for (key, value) in field_values.iter() {
            fields.push(RecordGrain::new(key.clone(), value.clone()));
        }

        Ok((
            Record {
                record_type: "Link".to_string(),
                fields,
            },
            vec![],
        ))
    }
}
