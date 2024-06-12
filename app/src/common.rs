#![allow(dead_code)]

use std::{fs, io};

/// This can be changed to modify all the tool metadata all at once
pub const VERSION: &str = "0.0";
pub const AUTHOR: &str = "Getty Ritter <rrecutils@infinitenegativeutility.com>";

/// If this doesn't name a path, or if the path is `"-"`, then return
/// a buffered reader from stdin; otherwise, attempt to open the file
/// named by the path and return a buffered reader around it
pub fn input_from_spec(spec: Option<&str>) -> io::Result<io::BufReader<Box<dyn io::Read>>> {
    match spec.unwrap_or("-") {
        "-" => Ok(io::BufReader::new(Box::new(io::stdin()))),
        path => {
            let f = fs::File::open(path)?;
            Ok(io::BufReader::new(Box::new(f)))
        }
    }
}

/// If this doesn't name a path, or if the path is `"-"`, then return
/// a buffered writer to stdout; otherwise, attempt to open the file
/// named by the path and return a writer around it
pub fn output_from_spec(spec: Option<&str>) -> io::Result<Box<dyn io::Write>> {
    match spec.unwrap_or("-") {
        "-" => Ok(Box::new(io::stdout())),
        path => Ok(Box::new(fs::File::open(path)?)),
    }
}
