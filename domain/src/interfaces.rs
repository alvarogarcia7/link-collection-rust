pub mod database {
    use crate::Record;

    pub trait DatabaseReadAccess {
        fn read_all(self) -> Vec<Record>;
    }
}
