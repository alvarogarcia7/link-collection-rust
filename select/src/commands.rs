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
