use core::str;
use std::env;
use std::path::Path;

#[derive(Debug)]
pub struct GlobalConfiguration<'a> {
    pub database_path: &'a Path,
    pub template_path: &'a Path,
    pub template_name: String,
    pub hackernews_api_path: String,
}

impl<'a> GlobalConfiguration<'a> {
    pub fn verify_path(raw_value: &str) -> Option<&Path> {
        let path = Path::new(raw_value);
        if !path.exists() {
            eprintln!("PWD: {:?}", env::current_dir());
            eprintln!("This path does not exist: {:?}", path);
            return None;
        }
        Some(path)
    }
    pub fn in_memory(
        database_path: &'a str,
        template_path: &'a str,
        template_name: String,
        hackernews_api_path: String,
    ) -> Self {
        Self {
            database_path: GlobalConfiguration::verify_path(database_path).unwrap(),
            template_path: GlobalConfiguration::verify_path(template_path).unwrap(),
            template_name,
            hackernews_api_path,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum SubcommandType {
    List,
}
