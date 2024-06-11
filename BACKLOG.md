# Backlog
## Legend
* ✅ - Done
* 🚧 - In progress
* 🚀 - Ready to start
### EPIC - ID:1 - Read existing database files
#### ✅STORY - ID:1.1 - Dump the contents into the console
#### 🚧STORY - ID:1.2 - Use a template to print to console
### EPIC - ID:6 - Create a cli application
#### 🚧STORY - ID:6.1 - Use clap to for top command and multiple subcommands
* TODO: Read this cookbook: https://docs.rs/clap/latest/clap/_derive/_cookbook/git_derive/index.html
* TODO: Read https://docs.rs/clap/latest/clap/ and implement it with:
```rust
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
```
### STORY - ID:6.2 - Add a logger
### EPIC - ID:2 - Add a new record to the database
### EPIC - ID:3 - Retrieve items
### EPIC - ID:4 - Download items from HackerNews
### EPIC - ID:5 - Create a new wrapper on top of recutils library
#### STORY - ID:5.1 - Create it
#### STORY - ID:5.2 - Publish it to crates.io

