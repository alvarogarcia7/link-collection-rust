// Source: https://github.com/aisamanra/rrecutils/blob/master/src/tools/format.rs
extern crate clap;
extern crate rrecutils;
extern crate rustache;

use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::{env, io};

use rustache::Render;

struct R {
    rec: rrecutils::Record,
}

impl Render for R {
    fn render<W: io::Write>(
        &self,
        template: &str,
        writer: &mut W,
    ) -> Result<(), rustache::RustacheError> {
        use rustache::HashBuilder;
        let mut hb = HashBuilder::new();
        if let Some(ref t) = self.rec.rec_type {
            hb = hb.insert("%rec", t.clone());
        }
        for field in self.rec.fields.iter() {
            hb = hb.insert(&field.0, field.1.clone());
        }
        hb.render(template, writer)
    }
}

fn render_to_single_file(
    mut output: Box<dyn std::io::Write>,
    joiner: Option<&str>,
    recfile: rrecutils::Recfile,
    template: String,
) -> Result<(), ()> {
    let mut first = true;
    for r in recfile.records.into_iter() {
        if first {
            first = false;
        } else if let Some(j) = joiner {
            output.write_all(j.as_bytes()).unwrap();
            output.write_all(&[b'\n']).unwrap();
        }
        R { rec: r }
            .render(&template, &mut output.as_mut())
            .map_err(|e| panic!("Rustache error: {:?}", e))?;
    }

    Ok(())
}

fn run(
    database_path: &Path,
    template_path: &Path,
    template_name: String,
    destination: &Path,
) -> Result<(), ()> {
    println!("PWD: {:?}", env::current_dir());

    let file = File::open(database_path).unwrap();

    let mut recfile = rrecutils::Recfile::parse(BufReader::new(file)).unwrap();
    recfile.filter_by_type("Link");

    let buf1 = template_path.join(template_name);
    let template: String = match File::open::<PathBuf>(buf1) {
        Ok(mut path) => {
            use io::Read;
            let mut buf = Vec::new();
            path.read_to_end(&mut buf).unwrap();
            String::from_utf8(buf).unwrap()
        }
        Err(_) => panic!("No template specified!"),
    };

    render_to_single_file(
        Box::new(File::create(destination).unwrap()),
        None,
        recfile,
        template,
    )?;

    Ok(())
}

pub fn main() {
    run(
        Path::new("./data_access/data/links.rec"),
        Path::new("./select/template/"),
        "cli-short.mustache".to_string(),
        Path::new("./target/formatted.txt"),
    )
    .unwrap();
}

#[cfg(test)]
pub mod tests {
    use crate::run;
    use std::path::Path;

    #[test]
    #[ignore]
    fn main() {
        match run(
            Path::new("../data_access/data/links.rec"),
            Path::new("./template/"),
            "cli-short.mustache".to_string(),
            Path::new("../target/formatted.txt"),
        ) {
            Ok(()) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}
