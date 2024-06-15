pub mod recutils_database;

pub mod dto;

#[cfg(test)]
pub mod tests {
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::io::{BufReader, BufWriter};

    use rrecutils::{Recfile, Record};

    use domain::RecordGrain;

    #[test]
    #[ignore]
    pub fn read_file() {
        println!("{:?}", env::current_dir());
        let file = File::open("../../data/database/links.rec").unwrap();
        let reader = BufReader::new(file);

        let records = Recfile::parse(reader).unwrap().records;

        // println!("{:?}", records);

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].rec_type, None);
        assert_eq!(records[1].rec_type, Some("Link".to_string()));
    }

    #[test]
    pub fn keep_the_order_of_the_fields() {
        let field_values = vec![
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
                ["Body Line1", "Body line 2", "LONG LONG LONG"].join("\n+"),
            ),
            ("Tags".to_string(), "tag1, tag_2, name-surname".to_string()),
        ];
        let mut fields: Vec<RecordGrain> = vec![];

        for (key, value) in field_values.iter() {
            fields.push(RecordGrain::new(key.clone(), value.clone()));
        }

        let record = domain::Record {
            record_type: "Link".to_string(),
            fields,
        };

        // to dto

        let vec1 = record
            .fields
            .iter()
            .map(|grain| (grain.key.clone(), grain.value.clone()))
            .collect();
        let dto_record = Record {
            rec_type: Some("Link".to_string()),
            fields: vec1,
        };

        let rec_file = Recfile {
            records: vec![dto_record],
        };

        let mut buffer = Vec::new();
        {
            let mut writer = std::io::BufWriter::new(&mut buffer);
            rec_file.write(&mut writer).unwrap();
            writer.flush().unwrap()
        }

        let actual = String::from_utf8(buffer).unwrap();

        let expected = "Id: a1a6925a-7958-11e8-a87f-0242ac110002
Date: Tue, 12 Jun 2024 10:50:21 +0000
Category: category1
Title: Good title
Body: Body Line1
+Body line 2
+LONG LONG LONG
Tags: tag1, tag_2, name-surname

";

        assert_eq!(actual, expected);
    }

    #[test]
    #[ignore]
    pub fn write_file() {
        {
            println!("{:?}", env::current_dir());
            let file = File::create("./data/links_mod.rec").unwrap();
            let writer = &mut BufWriter::new(file);

            let record = Record {
                rec_type: Some("Link".to_string()),
                fields: vec![
                    (
                        "Id".to_string(),
                        "a1a6925a-7958-11e8-a87f-0242ac110002".to_string(),
                    ),
                    (
                        "Date".to_string(),
                        "Tue, 26 Jun 2018 15:50:21 +0000".to_string(),
                    ),
                    ("Category".to_string(), "category1".to_string()),
                    ("Title".to_string(), "Title".to_string()),
                    ("Body".to_string(), "A body with multiple lines".to_string()),
                    ("Tags".to_string(), "tag1, tag_2, name-surname".to_string()),
                ],
            };

            let recfile = Recfile {
                records: vec![record],
            };
            recfile.write(writer).unwrap();
        }

        {
            let file = File::open("./data/links_mod.rec").unwrap();
            let reader = BufReader::new(file);

            let records = Recfile::parse(reader).unwrap().records;

            assert_eq!(records.len(), 1);
            assert_eq!(
                records[0].fields[0],
                (
                    "Id".to_string(),
                    "a1a6925a-7958-11e8-a87f-0242ac110002".to_string()
                )
            );
        }
    }
}
