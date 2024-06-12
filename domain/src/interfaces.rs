pub mod database {
    use crate::Record;

    pub trait DatabaseReadAccess {
        fn read_all(self) -> Vec<Record>;
    }
}

pub mod record {
    use crate::Record;

    pub trait RecordProvider {
        fn fetch(&mut self) -> Record;
    }
}
