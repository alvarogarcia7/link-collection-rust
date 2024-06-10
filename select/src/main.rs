extern crate core;

use core::str;

use clap::builder::Str;
use clap::{arg, Command};

mod common;

#[derive(PartialEq, Copy, Clone)]
pub enum SubcommandType {
    LIST,
}

const COMMANDS_AND_NAMES: [(&str, SubcommandType); 1] = [("ls", SubcommandType::LIST)];

// // #[repr(String)]
// trait SubcommandType {
//     fn to_string(self) -> String;
// }
//
// pub enum SubCommandSynonymType {
//     LIST
// }
//
//
// struct Subcommand {
//     name: String,
//     Item: SubCommandSynonymType,
// }
//
// impl SubcommandType for Subcommand {
//     fn to_string(self) -> String {
//         match self.Item {
//             LIST => "ls",
//             _ => todo!()
//         }.to_string()
//     }
// }
//
// impl From<&str> for SubcommandType {
//     fn from(value: &str) -> Self {
//         SubcommandType::LIST
//     }
// }
//
// impl From<SubCommandSynonymType> for Str {
//     fn from(value: SubCommandSynonymType) -> Self {
//         value.
//         let x: String = SubcommandType::into(value);
//         // SubcommandType::type_id(&value)
//         Self::from("ls")
//     }
// }
//
// impl TryInto<SubcommandType> for str {
//     fn from(value: SubcommandType) -> Self {
//         "a".
//     }
// }

impl From<SubcommandType> for Str {
    fn from(value: SubcommandType) -> Self {
        let (x, _) = COMMANDS_AND_NAMES
            .iter()
            .find(|(_, x)| x == &value)
            .unwrap();
        Self::from(*x)
    }
}

impl From<&str> for SubcommandType {
    fn from(value: &str) -> Self {
        let (_, x) = COMMANDS_AND_NAMES
            .iter()
            .find(|(x, _)| x == &value)
            .unwrap();
        *x
    }
}

fn cli() -> Command {
    Command::new("link-collection")
        .about("A tool to manage your links")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // .allow_external_subcommands(true)
        .subcommand(
            Command::new(SubcommandType::LIST)
                .about("Reads a file")
                .arg(arg!(<FILE> "The database file to read"))
                .arg_required_else_help(true),
        )
}

fn run() -> Result<(), ()> {
    let matches = cli().get_matches();

    match matches
        .subcommand()
        .map(|(f, rest)| (SubcommandType::from(f), rest))
    {
        Some((SubcommandType::LIST, _)) => {}
        _ => todo!(),
    }

    println!("{:?}", matches);

    Ok(())
}

pub fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => println!("{:?}", e),
    }
}

#[cfg(test)]
pub mod tests {
    use crate::cli;

    #[test]
    pub fn parse_the_ls_subcommand() {
        let arg_vec = vec!["", "ls", "$FILE"];

        let actual = cli().get_matches_from(arg_vec);
        println!("{:?}", actual);

        assert_eq!(actual.subcommand().unwrap().0, "ls");
        assert_eq!(actual.subcommand().unwrap().1.ids().len(), 1);
        assert!(actual.subcommand().unwrap().1.contains_id("FILE"));
    }
}
