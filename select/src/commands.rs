use crate::configuration::GlobalConfiguration;
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

    pub fn run(&self, _record_provider: &dyn RecordProvider) -> Result<(), NewRecordUseCaseError> {
        println!(
            "Running NewRecordUseCase with configuration: {:?}",
            self.global_configuration
        );
        Ok(())
    }
}
