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

impl Tags {
    pub fn import(values: Vec<String>) -> Self {
        let tags = tags::split_tags(values);
        let tags = tags::lowercase_separated_by_dash(tags);
        Tags { values: tags }
    }
}
