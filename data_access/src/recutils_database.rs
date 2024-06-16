use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use std::process::Command;

use rrecutils::Recfile;

use crate::dto::to_dto;
use domain::interfaces::database::{DatabaseReadAccess, DatabaseWriteAccess};
use domain::{Record, RecordGrain};

pub struct RecutilsDatabaseAccess<'a> {
    // reader: &'a Recfile,
    path: &'a str,
    record_type: String,
}

impl<'a> RecutilsDatabaseAccess<'a> {
    pub fn new(path: &'a str, record_type: String) -> Self {
        Self { path, record_type }
    }
}

impl<'a> DatabaseReadAccess for RecutilsDatabaseAccess<'a> {
    fn read_all(self) -> Vec<Record> {
        let reader = Recfile::parse(BufReader::new(File::open(self.path).unwrap())).unwrap();

        let mapped_records = reader
            .iter_by_type(&self.record_type)
            //             .filter_map(|p| match p.last_name {
            //    Some(_) => Some(p.age),
            //    None => None
            // })
            // .iter()
            // .filter_map(
            //     move |foreign| {
            //         if self.record_type.is_none() {
            //             self.record_type = foreign.rec_type.clone();
            //             None
            //         } else {
            //             // foreign.fields.iter().map(|x|)
            //             let mut fields: HashMap<String, String> = HashMap::new();
            //             for (key, value) in foreign.fields.iter() {
            //                 fields.insert(key.to_string(), value.to_string());
            //             }
            //             Some(Record { record_type: <Option<String> as Clone>::clone(&self.record_type).unwrap(), fields })
            //         }
            //     }
            // ).collect::<Vec<Record>>();
            .map(|foreign| {
                // foreign.fields.iter().map(|x|)
                let mut fields: Vec<RecordGrain> = vec![];
                let mut foreign_fields = vec![];
                for (key, value) in foreign.fields.iter() {
                    fields.push(RecordGrain::new(key.to_string(), value.to_string()));
                    foreign_fields.push((key.clone(), value.clone()));
                }
                Record {
                    record_type: self.record_type.clone(),
                    fields,
                }
            })
            .collect::<Vec<Record>>();
        println!("Mapped records: {:?}", mapped_records.len());
        mapped_records
    }
}

pub struct RecutilsDatabaseWriter<'a> {
    path: &'a Path,
}

impl<'a> RecutilsDatabaseWriter<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self { path }
    }
    pub fn commit(&self, variables: Vec<String>) {
        let mut envs = HashMap::<&str, &str>::new();
        variables.iter().for_each(|tag| {
            let mut split = tag.split(';');
            if let Some("VARIABLE") = split.next() {
                let variable_name = split.next();
                if variable_name.unwrap() == "DATE" {
                    let variable_value = split.next();

                    envs.insert("GIT_COMMITTER_DATE", variable_value.unwrap());
                    envs.insert("GIT_AUTHOR_DATE", variable_value.unwrap());
                }
                // println!(
                //     "Variable: {} Value: {}",
                //     variable_name.unwrap(),
                //     variable_value.unwrap()
                // );

                // let output = if cfg!(target_os = "windows") {
                //     Command::new("cmd")
                //         .args(["/C", "echo hello"])
                //         .output()
                //         .expect("failed to execute process")
                // } else {
                // };
            }
        });
        // use std::process::Command;
        //
        // let mut list_dir = Command::new("ls");
        //
        // // Execute `ls` in the current directory of the program.
        // list_dir.status().expect("process failed to execute");
        //
        // println!();
        //
        // // Change `ls` to execute in the root directory.
        // list_dir.current_dir("/");
        //
        // // And then execute `ls` again but in the root directory.
        // list_dir.status().expect("process failed to execute");

        let command = Command::new("git")
            .current_dir(self.path.parent().unwrap())
            .arg("add")
            .arg(self.path.file_name().unwrap())
            .output()
            .expect("Git add went wrong");
        println!("{:?}", String::from_utf8(command.stdout));

        let command2 = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("save")
            .arg("-n")
            .envs(envs.iter())
            .current_dir(self.path.parent().unwrap())
            .output()
            .expect("failed to execute process");
        println!("{:?}", String::from_utf8(command2.stdout));
    }
}

impl<'a> DatabaseWriteAccess for RecutilsDatabaseWriter<'a> {
    fn write(&self, record: Record) {
        let recfile = Recfile {
            records: to_dto(vec![record]),
        };
        let file = std::fs::OpenOptions::new()
            .append(true)
            .open(self.path)
            .unwrap();
        let mut writer = BufWriter::new(file);
        recfile.write(&mut writer).unwrap();
        writer.flush().unwrap();
        println!("Wrote record to database file: {:?}", self.path);
    }
}
