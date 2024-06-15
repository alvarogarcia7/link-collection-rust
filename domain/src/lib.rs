pub mod interfaces;

#[derive(Debug)]
pub struct Record {
    pub record_type: String,
    pub fields: Vec<RecordGrain>,
    pub fields_dto: Vec<(String, String)>,
}

impl Record {}

#[derive(Debug)]
pub struct RecordGrain {
    pub key: String,
    pub value: String,
}

impl RecordGrain {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}
