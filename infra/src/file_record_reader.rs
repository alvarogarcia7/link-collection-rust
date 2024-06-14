use domain::interfaces::record::RecordProvider;
use domain::Record;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct FileReaderRecordProvider {
    path: PathBuf,
}

impl FileReaderRecordProvider {
    // TODO AGB: This should be a Path, need to handle the lifetime
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }
}

impl RecordProvider for FileReaderRecordProvider {
    fn fetch(&mut self) -> Record {
        println!(
            "Faking reading the record file with the new record: {:?}",
            self.path
        );

        let body = ["Body Line1", "Body line 2"].join("\n+");
        let fields_dto: HashMap<&str, &str> = HashMap::from([
            ("Id", "a1a6925a-7958-11e8-a87f-0242ac110002"),
            ("Date", "Tue, 26 Jun 2018 15:50:21 +0000"),
            ("Category", "category1"),
            ("Title", "Good title"),
            ("Body", body.as_str()),
            ("Tags", "tag1, tag_2, name-surname"),
        ]);

        let fields = fields_dto
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        let fields_dto = fields_dto
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        Record {
            record_type: "Link".to_string(),
            fields,
            fields_dto,
        }
    }
}
