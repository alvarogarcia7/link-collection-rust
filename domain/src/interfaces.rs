pub mod database {
    use crate::Record;

    pub trait DatabaseReadAccess {
        fn read_all(self) -> Vec<Record>;
    }

    pub trait DatabaseWriteAccess {
        fn write(&self, record: Record);
    }
}

#[derive(Debug)]
pub enum RecordProviderError {
    NoRecordFound,
    ErrorFetchingRecord,
}
pub mod record {
    use crate::interfaces::RecordProviderError;
    use crate::Record;
    pub trait RecordProvider {
        fn fetch(&mut self) -> Result<(Record, Vec<String>), RecordProviderError>;
    }
}
