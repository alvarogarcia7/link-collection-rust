use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

use rrecutils::{Recfile, Record};

use domain::interfaces::record::RecordProvider;

use crate::configuration::GlobalConfiguration;

pub mod list_command {
    use domain::interfaces::database::DatabaseReadAccess;
    use domain::Record;

    #[derive(Debug)]
    pub enum MyError {
        Xxx,
    }

    pub fn run(database: impl DatabaseReadAccess) -> Result<Vec<Record>, MyError> {
        // for x in database.read_all() {
        //     println!("{:?}", x)
        // }
        Ok(database.read_all())
    }
}

pub struct NewRecordUseCase<'a> {
    global_configuration: GlobalConfiguration<'a>,
}

pub enum NewRecordUseCaseError {
    None,
}

impl<'a> NewRecordUseCase<'a> {
    pub fn new(global_configuration: GlobalConfiguration<'a>) -> Self {
        Self {
            global_configuration,
        }
    }

    pub fn run(
        &self,
        _record_provider: &mut dyn RecordProvider,
    ) -> Result<(), NewRecordUseCaseError> {
        let domain_record = _record_provider.fetch();

        let vec1 = domain_record
            .fields
            .iter()
            .map(|grain| (grain.key.clone(), grain.key.clone()))
            .collect();
        let dto_record = Record {
            rec_type: Some("Link".to_string()),
            fields: vec1,
        };

        let recfile = Recfile {
            records: vec![dto_record],
        };
        assert_eq!(recfile.records.len(), 1);

        let path = self.global_configuration.database_path;
        // let file = OpenOptions().append(path).unwrap();
        let file = OpenOptions::new().append(true).open(path).unwrap();
        let mut writer = BufWriter::new(file);
        recfile.write(&mut writer).unwrap();
        writer.flush().unwrap();
        println!("Wrote record to database file: {:?}", path);
        Ok(())
    }
}
