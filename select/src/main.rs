use clap::{arg, Command};

mod common;

fn cli() -> Command {
    Command::new("link-collection")
        .about("A tool to manage your links")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // .allow_external_subcommands(true)
        .subcommand(
            Command::new("ls")
                .about("Reads a file")
                .arg(arg!(<FILE> "The database file to read"))
                .arg_required_else_help(true),
        )
}

fn run() -> Result<(), ()> {
    let matches = cli().get_matches();

    println!("{:?}", matches);

    Ok(())
}

fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => println!("{:?}", e),
    }
}
