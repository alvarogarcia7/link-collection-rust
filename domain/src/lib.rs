pub mod interfaces;
pub mod tags;

#[derive(Debug)]
pub struct Record {
    pub record_type: String,
    pub fields: Vec<RecordGrain>,
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

pub struct Tags {
    pub values: Vec<String>,
}

impl Tags {}
