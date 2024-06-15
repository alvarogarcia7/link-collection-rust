use std::fs::File;
use std::io::BufReader;

use rrecutils::Recfile;

use domain::interfaces::database::DatabaseReadAccess;
use domain::{Record, RecordGrain};

pub struct RecutilsDatabaseAccess<'a> {
    // reader: &'a Recfile,
    path: &'a str,
    record_type: String,
}

impl<'a> RecutilsDatabaseAccess<'a> {
    pub fn new(path: &'a str, record_type: String) -> Self {
        Self { path, record_type }
    }
}

impl<'a> DatabaseReadAccess for RecutilsDatabaseAccess<'a> {
    fn read_all(self) -> Vec<Record> {
        let reader = Recfile::parse(BufReader::new(File::open(self.path).unwrap())).unwrap();

        let mapped_records = reader
            .iter_by_type(&self.record_type)
            //             .filter_map(|p| match p.last_name {
            //    Some(_) => Some(p.age),
            //    None => None
            // })
            // .iter()
            // .filter_map(
            //     move |foreign| {
            //         if self.record_type.is_none() {
            //             self.record_type = foreign.rec_type.clone();
            //             None
            //         } else {
            //             // foreign.fields.iter().map(|x|)
            //             let mut fields: HashMap<String, String> = HashMap::new();
            //             for (key, value) in foreign.fields.iter() {
            //                 fields.insert(key.to_string(), value.to_string());
            //             }
            //             Some(Record { record_type: <Option<String> as Clone>::clone(&self.record_type).unwrap(), fields })
            //         }
            //     }
            // ).collect::<Vec<Record>>();
            .map(|foreign| {
                // foreign.fields.iter().map(|x|)
                let mut fields: Vec<RecordGrain> = vec![];
                let mut foreign_fields = vec![];
                for (key, value) in foreign.fields.iter() {
                    fields.push(RecordGrain::new(key.to_string(), value.to_string()));
                    foreign_fields.push((key.clone(), value.clone()));
                }
                Record {
                    record_type: self.record_type.clone(),
                    fields,
                }
            })
            .collect::<Vec<Record>>();
        println!("Mapped records: {:?}", mapped_records.len());
        mapped_records
    }
}
