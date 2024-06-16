# Backlog
## Legend
* ✅ - Done
* 🚧 - In progress
* 🚀 - Ready to start
### EPIC - ID:1 - Read existing database files
#### ✅STORY - ID:1.1 - Dump the contents into the console
#### ✅STORY - ID:1.2 - Use a fixed template to print to console
#### 🚀STORY - ID:1.3 - Use any template to print to console
#### ✅STORY - ID:1.4 - Read the existing date formats
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
### ✅STORY - ID:6.3 - Add a configuration so the values are not repeated
### STORY - ID:6.4 - Read the configuration from a file
### EPIC - ID:2 - Add a new record to the database
### ✅STORY - ID:2.1 - From the CLI
### 🚧STORY - ID:2.2 - From a (temporary) file
### ✅DEFECT - ID:2.3 - Print the record in the same order as it was provided
### EPIC - ID:3 - Retrieve items
### EPIC - ID:4 - Download items from HackerNews
### ✅STORY - ID:4.1 - Create a new stubbed HTTP server
### ✅STORY - ID:4.2 - Download the items from a URL
### EPIC - ID:5 - Create a new wrapper on top of recutils library
#### STORY - ID:5.1 - Create it
#### STORY - ID:5.2 - Publish it to crates.io

