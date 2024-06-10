#[cfg(test)]
pub mod tests {
    use rrecutils::{Recfile, Record};
    use std::env;
    use std::fs::File;
    use std::io::{BufReader, BufWriter};

    #[test]
    #[ignore]
    pub fn read_file() {
        println!("{:?}", env::current_dir());
        let file = File::open("./data/links.rec").unwrap();
        let reader = BufReader::new(file);

        let records = Recfile::parse(reader).unwrap().records;

        // println!("{:?}", records);

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].rec_type, None);
        assert_eq!(records[1].rec_type, Some("Link".to_string()));
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
