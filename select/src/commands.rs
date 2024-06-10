pub mod list_command {
    use domain::interfaces::database::DatabaseReadAccess;

    pub enum MyError {
        Xxx,
    }

    pub fn run(database: impl DatabaseReadAccess) -> Result<MyError, MyError> {
        let x = database.read_all();
        for x in x {
            println!("{:?}", x)
        }
        Ok(MyError::Xxx)
    }
}
