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
