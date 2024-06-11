extern crate core;

use core::str;

use clap::builder::Str;
use clap::{arg, Command};

use data_access::recutils_database::RecutilsDatabaseAccess;
use domain::Record;
use select::commands::list_command;
use select::commands::list_command::MyError;

#[derive(PartialEq, Copy, Clone)]
pub enum SubcommandType {
    List,
}

const COMMANDS_AND_NAMES: [(&str, SubcommandType); 1] = [("ls", SubcommandType::List)];

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
            Command::new(SubcommandType::List)
                .about("Reads a file")
                .arg(
                    arg!(<FILE> "The database file to read")
                        // .action(ArgAction::Set)
                        .required(true),
                )
                .arg_required_else_help(true),
        )
}

fn run() -> Result<Vec<Record>, MyError> {
    let matches = cli().get_matches();

    match matches
        .subcommand()
        .map(|(f, rest)| (SubcommandType::from(f), rest))
    {
        Some((SubcommandType::List, arg_matches)) => {
            let path = arg_matches.get_one::<String>("FILE");
            println!("Reading database file at: {:?}", path);
            let access = RecutilsDatabaseAccess::new(path.unwrap(), "Link".to_string());
            list_command::run(access)
        }
        _ => todo!(),
    }
}

pub fn main() {
    run().unwrap();
    // match
    // {
    //     Ok(_) => (),
    //     Err(e) => println!("{:?}", e),
    // }
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

        let subcommand_args = actual.subcommand().unwrap().1;
        assert_eq!(subcommand_args.ids().len(), 1);
        assert!(subcommand_args.contains_id("FILE"));
        assert_eq!(
            subcommand_args.get_one::<String>("FILE"),
            Some(&"$FILE".to_string())
        );
    }
}
