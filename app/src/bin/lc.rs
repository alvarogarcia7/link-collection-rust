extern crate core;

use core::str;
use std::path::Path;
use std::process::exit;

// use clap::{Args, Parser, Subcommand, ValueEnum};
use clap::{arg, Args, Parser, Subcommand};
use data_access::recutils_database::RecutilsDatabaseWriter;

use domain::interfaces::record::RecordProvider;
use downloader::downloader::FirebaseHackerNewsDownloader;
use infra::cli_line_reader::{CliReaderRecordProvider, MyEditor};
use infra::date::DateProvider;
use infra::file_record_reader::FileReaderRecordProvider;
use infra::hacker_news_importer::FirebaseHackerNewsImporterProvider;
use infra::hardcoded::HardcodedRecordProvider;
use select::commands::NewRecordUseCase;
use select::configuration::GlobalConfiguration;
use select::print::run;

// use clap::arg;

// Source: https://docs.rs/clap/latest/clap/_derive/_cookbook/git_derive/index.html

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "lc")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, default_value = "dev")]
    environment: String,
}

impl<'a> App<'a> {
    fn new(global_configuration: GlobalConfiguration<'a>) -> Self {
        Self {
            global_configuration,
        }
    }
    fn run(self, commands: Commands) -> Result<(), ()> {
        match commands {
            Commands::List(list_args) => {
                let destination_path = Path::new(&list_args.file);
                run(
                    GlobalConfiguration::verify_path(&list_args.file).unwrap(),
                    self.global_configuration.template_path,
                    self.global_configuration.template_name,
                    destination_path,
                )
                .unwrap();
                println!("Formatted file written to: {:?}", destination_path);
                Ok(())
            }
            Commands::NewRecord { from } => {
                let record_provider =
                    Self::decide_which_provider(&self.global_configuration, &from);
                // AGB: alternative: ok_or_else
                let mut record_provider = record_provider.ok_or(())?;
                NewRecordUseCase::new(RecutilsDatabaseWriter::new(
                    self.global_configuration.database_path,
                ))
                .run(&mut *record_provider)
                .map_err(|_| ())
            }
        }
    }

    fn decide_which_provider(
        global_configuration: &GlobalConfiguration,
        provider_name: &String,
    ) -> Option<Box<dyn RecordProvider + 'static>> {
        if "hardcoded" == provider_name {
            Some(Box::<HardcodedRecordProvider>::default() as Box<dyn RecordProvider>)
        } else if "cli_line_reader" == provider_name {
            Some(Box::new(CliReaderRecordProvider::new(
                MyEditor::default(),
                DateProvider::default(),
            )) as Box<dyn RecordProvider>)
        } else if provider_name.starts_with("import") {
            let maybe_id: Vec<&str> = provider_name.split(':').collect();
            let id = maybe_id[1].parse::<u64>();
            if id.is_err() {
                print!(
                    "Couldn't parse the number {:?}. Full string: {:?}",
                    maybe_id[1], provider_name
                );
                return None;
            }
            let id = id.unwrap();
            Some(Box::new(FirebaseHackerNewsImporterProvider::new(
                MyEditor::default(),
                DateProvider::default(),
                FirebaseHackerNewsDownloader::new(global_configuration.hackernews_api_path.clone()),
                // FirebaseHackerNewsDownloader::new("https://hacker-news.firebaseio.com".to_string()),
                id,
            )) as Box<dyn RecordProvider>)
        } else {
            let record_file = GlobalConfiguration::verify_path(provider_name)?;
            Some(Box::new(FileReaderRecordProvider::new(record_file)) as Box<dyn RecordProvider>)
        }
    }
}

#[derive(Debug, Args, PartialEq)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct ListArgs {
    // #[command(subcommand)]
    // command: Option<StashCommands>,
    //
    // #[command(flatten)]
    // push: StashPushArgs,
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    target: String,
}

#[derive(Debug, Subcommand, PartialEq)]
enum StashCommands {
    Push(StashPushArgs),
    Pop { stash: Option<String> },
    Apply { stash: Option<String> },
}

#[derive(Debug, Args, PartialEq)]
struct StashPushArgs {
    #[arg(short, long)]
    message: Option<String>,
}

#[derive(Debug, Subcommand, PartialEq)]
enum Commands {
    #[command(alias = "ls", arg_required_else_help = true)]
    List(ListArgs),
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

struct App<'a> {
    global_configuration: GlobalConfiguration<'a>,
}

fn main() {
    let args = Cli::parse();

    let (download_path, database_path) = if args.environment == "pro" {
        (
            "http://0.0.0.0:8181".to_string(),
            "../link-collection/data/links.rec",
        )
    } else {
        (
            "https://hacker-news.firebaseio.com".to_string(),
            "./data/database/links.rec",
        )
    };

    let global_configuration = GlobalConfiguration::in_memory(
        database_path,
        "./data/template/",
        "cli-short.mustache".to_string(),
        download_path,
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

    use crate::{Cli, Commands, ListArgs};

    fn data_provider_list() -> Vec<&'static str> {
        vec!["list", "ls"]
    }

    #[test]
    fn parse_the_list_subcommand_with_any_variant() {
        for subcommand in data_provider_list() {
            let arg_vec = ["", subcommand, "--file", "$FILE", "--target", "stdout"];

            let actual = Cli::parse_from(arg_vec.iter());

            assert_eq!(
                actual.command,
                Commands::List(ListArgs {
                    file: "$FILE".to_string(),
                    target: "stdout".to_string()
                })
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
            "./tests/template/",
            "cli-short.mustache".to_string(),
            "http://0.0.0.0:8181".to_string(),
        )
    }

    #[test]
    #[ignore] // This test uses the filesystem
    fn run_the_list_subcommand() {
        App::new(global_configuration_test())
            .run(Commands::List(ListArgs {
                file: "tests/data/links.rec".to_string(),
                target: "/dev/null".to_string(),
            }))
            .unwrap();
    }

    #[test]
    #[ignore]
    fn run_the_newrecord_subcommand_from_hardcoded() {
        assert_eq!(
            App::new(global_configuration_test()).run(Commands::NewRecord {
                from: "hardcoded".to_string(),
            }),
            Ok(())
        );
    }

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
