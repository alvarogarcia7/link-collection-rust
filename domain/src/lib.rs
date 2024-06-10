use std::collections::HashMap;

pub mod interfaces;

#[derive(Debug)]
pub struct Record {
    pub record_type: String,
    pub fields: HashMap<String, String>,
}

impl Record {}
