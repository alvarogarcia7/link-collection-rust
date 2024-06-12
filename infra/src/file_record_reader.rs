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
            ("Tags".to_string(), "tag1, tag_2, name-surname".to_string()),
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
