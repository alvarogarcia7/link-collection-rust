#[cfg(test)]
pub mod tests {
    use rrecutils::Recfile;
    use std::env;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    pub fn read_file() {
        println!("{:?}", env::current_dir());
        let file = File::open("./data/links.rec").unwrap();
        let reader = BufReader::new(file);

        let records = Recfile::parse(reader).unwrap().records;

        println!("{:?}", records);

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].rec_type, None);
        assert_eq!(records[1].rec_type, Some("Link".to_string()));
    }
}
