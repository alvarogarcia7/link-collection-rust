use rrecutils::Record;

pub fn to_dto(records: Vec<domain::Record>) -> Record {
    let mut fields = vec![];
    let record_type = records[0].record_type.clone();
    for record in records {
        for grain in record.fields {
            fields.push((grain.key, grain.value));
        }
    }
    Record {
        rec_type: Some(record_type),
        fields,
    }
}

#[cfg(test)]

pub mod tests {
    use crate::dto::to_dto;
    use domain::{Record, RecordGrain};

    #[test]
    pub fn keep_the_name_and_value() {
        let domain = Record {
            record_type: "Link".to_string(),
            fields: vec![
                RecordGrain::new("title".to_string(), "The title".to_string()),
                RecordGrain::new("url".to_string(), "http://example.com".to_string()),
            ],
        };

        let actual = to_dto(vec![domain]);

        let expected = rrecutils::Record {
            rec_type: Some("Link".to_string()),
            fields: vec![
                ("title".to_string(), "The title".to_string()),
                ("url".to_string(), "http://example.com".to_string()),
            ],
        };

        assert_eq!(actual, expected);
    }
}
