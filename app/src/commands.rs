use data_access::recutils_database::RecutilsDatabaseWriter;
use domain::interfaces::database::DatabaseWriteAccess;
use domain::interfaces::record::RecordProvider;

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
    database_write_access: RecutilsDatabaseWriter<'a>,
}

pub enum NewRecordUseCaseError {
    None,
}

impl<'a> NewRecordUseCase<'a> {
    pub fn new(database_write_access: RecutilsDatabaseWriter<'a>) -> Self {
        Self {
            database_write_access,
        }
    }

    pub fn run(
        &self,
        _record_provider: &mut dyn RecordProvider,
    ) -> Result<(), NewRecordUseCaseError> {
        let domain_record = _record_provider.fetch();

        if domain_record.is_err() {
            //TODO AGB: how to stack the errors?
            return Err(NewRecordUseCaseError::None);
        }

        let record = domain_record.unwrap();

        self.database_write_access.write(record.0);

        self.database_write_access.commit(record.1);

        Ok(())
    }
}
