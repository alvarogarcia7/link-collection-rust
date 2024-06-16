use rrecutils::Record;

pub fn to_dto(records: Vec<domain::Record>) -> Vec<Record> {
    let record_type = records[0].record_type.clone();
    let mut result = Vec::with_capacity(records.len());
    for record in records {
        let mut fields = vec![];
        for grain in record.fields {
            fields.push((grain.key, grain.value));
        }
        result.push(Record {
            rec_type: Some(record_type.clone()),
            fields,
        });
    }
    result
}

#[cfg(test)]

pub mod tests {
    use crate::dto::to_dto;
    use domain::{Record, RecordGrain};
    use std::vec;

    #[test]
    pub fn keep_the_name_and_value() {
        let domain = vec![Record {
            record_type: "Link".to_string(),
            fields: vec![
                RecordGrain::new("Title".to_string(), "The title".to_string()),
                RecordGrain::new("url".to_string(), "http://example.com".to_string()),
            ],
        }];

        let actual = to_dto(domain);

        let expected = vec![rrecutils::Record {
            rec_type: Some("Link".to_string()),
            fields: vec![
                ("Title".to_string(), "The title".to_string()),
                ("url".to_string(), "http://example.com".to_string()),
            ],
        }];

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn process_all_records_present() {
        let domain_records = vec![
            Record {
                record_type: "Link".to_string(),
                fields: vec![RecordGrain::new("Title".to_string(), "Title 1".to_string())],
            },
            Record {
                record_type: "Link".to_string(),
                fields: vec![RecordGrain::new("Title".to_string(), "Title 2".to_string())],
            },
        ];

        let actual = to_dto(domain_records);

        let expected = vec![
            rrecutils::Record {
                rec_type: Some("Link".to_string()),
                fields: vec![("Title".to_string(), "Title 1".to_string())],
            },
            rrecutils::Record {
                rec_type: Some("Link".to_string()),
                fields: vec![("Title".to_string(), "Title 2".to_string())],
            },
        ];

        assert_eq!(actual, expected);
    }
}
