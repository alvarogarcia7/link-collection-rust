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
#[derive(Debug, Parser, PartialEq)] // requires `derive` feature
#[command(name = "lc")]
struct Cli {
    #[arg(short, long)]
    file: Option<String>,
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, default_value = "dev")]
    environment: String,
}

#[derive(Debug)]
pub struct AppOptions<'a> {
    pub commands: Commands,
    pub database_path: &'a Path,
    pub template_path: &'a Path,
    pub template_name: String,
    pub hackernews_api_path: String,
}

impl<'a> App<'a> {
    fn new(app_options: AppOptions<'a>) -> Self {
        Self { app_options }
    }
    fn run(self) -> Result<(), ()> {
        let commands = self.app_options.commands;
        match commands {
            Commands::List(list_args) => {
                let string = list_args.target.unwrap_or("stdout".to_string());
                let destination_path = Path::new(&string);
                run(
                    self.app_options.database_path,
                    self.app_options.template_path,
                    self.app_options.template_name,
                    destination_path,
                )
                .unwrap();
                println!("Formatted file written to: {:?}", destination_path);
                Ok(())
            }
            Commands::NewRecord { from } => {
                let record_provider =
                    Self::decide_which_provider(self.app_options.hackernews_api_path, &from);
                // AGB: alternative: ok_or_else
                let mut record_provider = record_provider.ok_or(())?;
                NewRecordUseCase::new(RecutilsDatabaseWriter::new(self.app_options.database_path))
                    .run(&mut *record_provider)
                    .map_err(|_| ())
            }
        }
    }

    fn decide_which_provider(
        hacker_news_path: String,
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
                FirebaseHackerNewsDownloader::new(hacker_news_path),
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
// #[command(args_conflicts_with_subcommands = true)]
// #[command(flatten_help = true)]
pub struct ListArgs {
    // #[command(subcommand)]
    // command: Option<StashCommands>,
    //
    // #[command(flatten)]
    // push: StashPushArgs,
    // #[arg(short, long)]
    // file: Option<String>,
    #[arg(short, long)]
    target: Option<String>,
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
pub enum Commands {
    #[command(alias = "ls")]
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
    app_options: AppOptions<'a>,
}

impl<'a> App<'a> {
    pub(crate) fn parse_args<'b>(
        commands: Commands,
        global_configuration: GlobalConfiguration<'b>,
        database_path: &'b Path,
    ) -> AppOptions<'b> {
        AppOptions {
            commands,
            database_path,
            template_path: global_configuration.template_path,
            template_name: global_configuration.template_name,
            hackernews_api_path: global_configuration.hackernews_api_path,
        }
    }
}

fn main() {
    let args = Cli::parse();

    let (download_path, database_path) = if args.environment == "pro" {
        (
            "http://0.0.0.0:8181".to_string(),
            "../link-collection/data/links.rec".to_string(),
        )
    } else {
        (
            "https://hacker-news.firebaseio.com".to_string(),
            "./data/database/links.rec".to_string(),
        )
    };

    let global_configuration = GlobalConfiguration::in_memory(
        &database_path,
        "./data/template/",
        "cli-short.mustache".to_string(),
        download_path,
    );

    let string = args.file.unwrap_or(
        global_configuration
            .database_path
            .to_str()
            .unwrap()
            .to_string(),
    );
    let value = string.as_str();
    let path = GlobalConfiguration::verify_path(value).unwrap();

    App::new(App::parse_args(args.command, global_configuration, path))
        .run()
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
            let arg_vec = ["", "--file", "$FILE", subcommand, "--target", "stdout"];

            let actual = Cli::parse_from(arg_vec.iter());

            assert_eq!(
                actual,
                Cli {
                    environment: "dev".to_string(),
                    file: Some("$FILE".to_string()),
                    command: Commands::List(ListArgs {
                        target: Some("stdout".to_string())
                    })
                }
            );
        }
    }

    #[test]
    fn parse_the_list_subcommand_without_file_args() {
        let arg_vec = ["", "list", "--target", "stdout"];

        let actual = Cli::parse_from(arg_vec.iter());

        assert_eq!(
            actual,
            Cli {
                environment: "dev".to_string(),
                file: None,
                command: Commands::List(ListArgs {
                    target: Some("stdout".to_string())
                })
            }
        );
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

    #[test]
    fn parse_the_list_without_file() {
        let arg_vec = ["", "list", "--target", "stdout"];

        let actual = Cli::parse_from(arg_vec.iter());

        assert_eq!(
            actual.command,
            Commands::List(ListArgs {
                target: Some("stdout".to_string())
            })
        );
    }
    #[test]
    fn parse_the_list_without_both() {
        let arg_vec = ["", "list"];

        let actual = Cli::parse_from(arg_vec.iter());

        assert_eq!(
            actual,
            Cli {
                file: None,
                command: Commands::List(ListArgs { target: None }),
                environment: "dev".to_string()
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

    // #[test]
    // #[ignore] // This test uses the filesystem
    // fn run_the_list_subcommand() {
    //     App::new(global_configuration_test())
    //         .run(Commands::List(ListArgs {
    //             file: "tests/data/links.rec".to_string(),
    //             target: "/dev/null".to_string(),
    //         }))
    //         .unwrap();
    // }

    #[test]
    #[ignore]
    fn run_the_newrecord_subcommand_from_hardcoded() {
        assert_eq!(
            App::new(App::parse_args(
                Commands::NewRecord {
                    from: "hardcoded".to_string(),
                },
                global_configuration_test(),
                global_configuration_test().database_path
            ))
            .run(),
            Ok(())
        );
    }

    #[test]
    #[ignore] // This test uses the filesystem
    fn run_the_newrecord_subcommand_from_file() {
        assert_eq!(
            App::new(App::parse_args(
                Commands::NewRecord {
                    from: "./tests/data/new-record-1.txt".to_string(),
                },
                global_configuration_test(),
                global_configuration_test().database_path
            ))
            .run(),
            Ok(())
        );
    }
}
