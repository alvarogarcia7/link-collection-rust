extern crate core;

use core::str;
use std::env;
use std::path::Path;
use std::process::exit;

// use clap::{Args, Parser, Subcommand, ValueEnum};
use clap::{arg, Parser, Subcommand};
// use clap::arg;
use clap::builder::Str;
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::{DefaultEditor, EditMode, Editor};

use select::print::run;

// Source: https://docs.rs/clap/latest/clap/_derive/_cookbook/git_derive/index.html

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "git")]
#[command(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

struct MyEditor {
    rl: Editor<(), FileHistory>,
}

impl Default for MyEditor {
    fn default() -> Self {
        let mut result = DefaultEditor::new().unwrap();
        result.set_edit_mode(EditMode::Vi);
        Self { rl: result }
    }
}

trait MyReadline {
    fn read_until_ctrl_d(&mut self, query: String, prompt: &str) -> Vec<String>;
}

impl MyReadline for MyEditor {
    fn read_until_ctrl_d(&mut self, query: String, prompt: &str) -> Vec<String> {
        let mut lines = vec![];
        loop {
            println!("Type '{}' (CTRL-D to finish)", query);
            let readline = self.rl.readline(prompt);
            match readline {
                Ok(line) => {
                    lines.push(line.clone());
                    println!("Line: {}", line);
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        lines
    }
}

impl<'a> App<'a> {
    fn new(global_configuration: GlobalConfiguration<'a>) -> Self {
        Self {
            global_configuration,
        }
    }
    fn run(self, commands: Commands) -> Result<(), ()> {
        match commands {
            Commands::List { file, target } => {
                let destination_path = Path::new(&target);
                run(
                    GlobalConfiguration::verify_path(&file).unwrap(),
                    self.global_configuration.template_path,
                    self.global_configuration.template_name,
                    destination_path,
                )
                .unwrap();
                println!("Formatted file written to: {:?}", destination_path);
                Ok(())
            }
            Commands::NewRecord { from } => {
                if "cli_line_reader" == from {
                    // let mut rl = DefaultEditor::my_default().unwrap();
                    let mut read_line = MyEditor::default();
                    let _body = read_line.read_until_ctrl_d("Body".to_string(), ">> ");
                    return Ok(());
                }
                let record_file = GlobalConfiguration::verify_path(&from);
                match record_file {
                    None => {
                        return Err(());
                    }
                    Some(record_file) => {
                        println!(
                            "Faking reading the record file with the new record: {:?}",
                            record_file
                        );
                        println!(
                            "Faking writing the database file with the new record: {:?}",
                            self.global_configuration.database_path
                        )
                    }
                };
                Ok(())
            }
        }
    }
}

#[derive(Debug, Subcommand, PartialEq)]
enum Commands {
    #[command(alias = "ls", arg_required_else_help = true)]
    List {
        file: String,
        #[arg(default_missing_value = "-", default_value = "-")]
        target: String,
    },
    #[command(alias = "n")]
    NewRecord {
        #[arg(
            default_missing_value = "cli_line_reader",
            default_value = "cli_line_reader"
        )]
        from: String,
    },
    // Compare two commits
    // Diff {
    //     #[arg(value_name = "COMMIT")]
    //     base: Option<OsString>,
    //     #[arg(value_name = "COMMIT")]
    //     head: Option<OsString>,
    //     #[arg(last = true)]
    //     path: Option<OsString>,
    //     #[arg(
    //         long,
    //         require_equals = true,
    //         value_name = "WHEN",
    //         num_args = 0..=1,
    //         default_value_t = ColorWhen::Auto,
    //         default_missing_value = "always",
    //         value_enum
    //     )]
    //     color: ColorWhen,
    // },
    // /// pushes things
    // #[command(arg_required_else_help = true)]
    // Push {
    //     /// The remote to target
    //     remote: String,
    // },
    // /// adds things
    // #[command(arg_required_else_help = true)]
    // Add {
    //     /// Stuff to add
    //     #[arg(required = true)]
    //     path: Vec<PathBuf>,
    // },
    // Stash(StashArgs),
    // #[command(external_subcommand)]
    // External(Vec<OsString>),
}
//
// #[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
// enum ColorWhen {
//     Always,
//     Auto,
//     Never,
// }
//
// impl std::fmt::Display for ColorWhen {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.to_possible_value()
//             .expect("no values are skipped")
//             .get_name()
//             .fmt(f)
//     }
// }
//
// #[derive(Debug, Args, PartialEq)]
// #[command(args_conflicts_with_subcommands = true)]
// #[command(flatten_help = true)]
// struct StashArgs {
//     #[command(subcommand)]
//     command: Option<StashCommands>,
//
//     #[command(flatten)]
//     push: StashPushArgs,
// }
//
// #[derive(Debug, Subcommand, PartialEq)]
// enum StashCommands {
//     Push(StashPushArgs),
//     Pop { stash: Option<String> },
//     Apply { stash: Option<String> },
// }
//
// #[derive(Debug, Args, PartialEq)]
// struct StashPushArgs {
//     #[arg(short, long)]
//     message: Option<String>,
// }

struct GlobalConfiguration<'a> {
    database_path: &'a Path,
    template_path: &'a Path,
    template_name: String,
}

impl<'a> GlobalConfiguration<'a> {
    fn verify_path(raw_value: &str) -> Option<&Path> {
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
    ) -> Self {
        Self {
            database_path: GlobalConfiguration::verify_path(database_path).unwrap(),
            template_path: GlobalConfiguration::verify_path(template_path).unwrap(),
            template_name,
        }
    }
}

struct App<'a> {
    global_configuration: GlobalConfiguration<'a>,
}

fn main() {
    let args = Cli::parse();

    let global_configuration = GlobalConfiguration::in_memory(
        "./data/database/links.rec",
        "./data/template/",
        "cli-short.mustache".to_string(),
    );

    App::new(global_configuration)
        .run(args.command)
        .map(|_| exit(0))
        .map_err(|_| exit(1))
        .unwrap();
    // match args.command {
    //     Commands::List { file } => {
    //         println!("Cloning {file}");
} //     Commands::Diff {
  //         mut base,
  //         mut head,
  //         mut path,
  //         color,
  //     } => {
  //         if path.is_none() {
  //             path = head;
  //             head = None;
  //             if path.is_none() {
  //                 path = base;
  //                 base = None;
  //             }
  //         }
  //         let base = base
  //             .as_deref()
  //             .map(|s| s.to_str().unwrap())
  //             .unwrap_or("stage");
  //         let head = head
  //             .as_deref()
  //             .map(|s| s.to_str().unwrap())
  //             .unwrap_or("worktree");
  //         let path = path.as_deref().unwrap_or_else(|| OsStr::new(""));
  //         println!(
  //             "Diffing {}..{} {} (color={})",
  //             base,
  //             head,
  //             path.to_string_lossy(),
  //             color
  //         );
  //     }
  //     Commands::Push { remote } => {
  //         println!("Pushing to {remote}");
  //     }
  //     Commands::Add { path } => {
  //         println!("Adding {path:?}");
  //     }
  //     Commands::Stash(stash) => {
  //         let stash_cmd = stash.command.unwrap_or(StashCommands::Push(stash.push));
  //         match stash_cmd {
  //             StashCommands::Push(push) => {
  //                 println!("Pushing {push:?}");
  //             }
  //             StashCommands::Pop { stash } => {
  //                 println!("Popping {stash:?}");
  //             }
  //             StashCommands::Apply { stash } => {
  //                 println!("Applying {stash:?}");
  //             }
  //         }
  //     }
  //     Commands::External(args) => {
  //         println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
  //     }
  // }
  // }

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

// fn run() -> Result<Vec<Record>, MyError> {
//     let matches = Cli::parse().get_matches();
//
//     match matches
//         .subcommand()
//         .map(|(f, rest)| (SubcommandType::from(f), rest))
//     {
//         Some((SubcommandType::List, arg_matches)) => {
//             let path = arg_matches.get_one::<String>("FILE");
//             println!("Reading database file at: {:?}", path);
//             let access = RecutilsDatabaseAccess::new(path.unwrap(), "Link".to_string());
//             list_command::run(access)
//         }
//         _ => todo!(),
//     }
// }

#[cfg(test)]
pub mod test_parsing_commands {
    use clap::Parser;

    use crate::{Cli, Commands};

    fn data_provider_list() -> Vec<&'static str> {
        vec!["list", "ls"]
    }

    #[test]
    fn parse_the_list_subcommand_with_any_variant() {
        for subcommand in data_provider_list() {
            let arg_vec = ["", subcommand, "$FILE"];

            let actual = Cli::parse_from(arg_vec.iter());

            assert_eq!(
                actual.command,
                Commands::List {
                    file: "$FILE".to_string(),
                    target: "-".to_string()
                }
            );
        }
    }

    #[test]
    fn x_parse_the_new_subcommand_with_any_variant() {
        for subcommand in ["new-record", "n"] {
            let arg_vec = ["", subcommand];

            let actual = Cli::parse_from(arg_vec.iter());

            assert_eq!(
                actual.command,
                Commands::NewRecord {
                    from: "cli_line_reader".to_string()
                }
            );
        }
    }
    #[test]
    fn parse_the_new_subcommand_with_the_default_from() {
        let arg_vec = ["", "new-record"];

        let actual = Cli::parse_from(arg_vec.iter());

        assert_eq!(
            actual.command,
            Commands::NewRecord {
                from: "cli_line_reader".to_string()
            }
        );
    }
    #[test]
    fn parse_the_new_subcommand_with_an_overridden_from() {
        let arg_vec = ["", "new-record", "file.txt"];

        let actual = Cli::parse_from(arg_vec.iter());

        assert_eq!(
            actual.command,
            Commands::NewRecord {
                from: "file.txt".to_string()
            }
        );
    }
}

#[cfg(test)]
pub mod test_executing_commands {
    use super::*;

    fn global_configuration_test<'a>() -> GlobalConfiguration<'a> {
        GlobalConfiguration::in_memory(
            "./tests/data/links.rec",
            "./template/",
            "cli-short.mustache".to_string(),
        )
    }

    #[test]
    #[ignore] // This test uses the filesystem
    fn run_the_list_subcommand() {
        App::new(global_configuration_test())
            .run(Commands::List {
                file: "tests/data/links.rec".to_string(),
                target: "/dev/null".to_string(),
            })
            .unwrap();
    }

    // #[test]
    // fn run_the_newrecord_subcommand_from_the_cli_reader() {
    //     assert_eq!(
    //         App::new(global_configuration_test()).run(Commands::NewRecord {
    //             from: "cli_line_reader".to_string(),
    //         }),
    //         Ok(())
    //     );
    // }

    #[test]
    #[ignore] // This test uses the filesystem
    fn run_the_newrecord_subcommand_from_file() {
        assert_eq!(
            App::new(global_configuration_test()).run(Commands::NewRecord {
                from: "./tests/data/new-record-1.txt".to_string(),
            }),
            Ok(())
        );
    }
}
