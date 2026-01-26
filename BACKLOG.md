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
#### ✅STORY - ID:1.5 - Add the optional field 'Link'
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
#### Acceptance Criteria:
* The configuration file is in TOML format
* The configuration file is read from the user's home directory: ~/.config/lc/config.toml
* The configuration file can be overridden by the .env.local file in the current working directory
* The configuration file can be overridden by an environment variable: LC_CONFIG_FILE
* The configuration file can be overridden by a command line argument: --config-file <path>
* The configuration file contains the following values:
  * database_file: path to the database file
  * log_level: log level for the logger
  * template_file: path to the template file
  * hacker_news_url: URL to download the hackernews items from

For this item to be complete, the following tasks need to be done:
* [ ] Create a struct to hold the configuration values
* [ ] Implement the logic to read the configuration file from the user's home directory
* [ ] Implement the logic to read the configuration file from the .env.local file
* [ ] Implement the logic to read the configuration file from the environment variable
* [ ] Implement the logic to read the configuration file from the command line argument
* [ ] Implement the logic to merge the configuration values from the different sources
* [ ] Write tests to verify the configuration reading logic
* [ ] Update the documentation to explain how to use the configuration file
* [ ] Update the README file to include information about the configuration file
* [ ] Create example configuration files for users to reference (.env.sample and config.toml.sample)
* [ ] Remove the logic to read from --env-file as it is no longer needed
* [ ] Update existing code to use the configuration values from the struct instead of hardcoded values
* [ ] Ensure proper error handling and logging for configuration reading issues
* [ ] Perform integration testing to ensure the application works correctly with the configuration file
* [ ] Update the build scripts to include the new configuration file if necessary
* [ ] Update the CI/CD pipeline to include tests for the configuration reading logic[ ] 
* [ ] Update the deployment scripts to ensure the configuration file is included
* [ ] Update the logging setup to use the log_level from the configuration file
* [ ] Update the database connection logic to use the database_file from the configuration file
* [ ] Update the template rendering logic to use the template_file from the configuration file
* [ ] Update the hackernews download logic to use the hacker_news_url from the configuration file
* [ ] Perform a code review to ensure the configuration reading logic is implemented correctly
### STORY - ID:6.5 - List the tags from the database
### EPIC - ID:2 - Add a new record to the database
### ✅STORY - ID:2.1 - From the CLI
### 🚧STORY - ID:2.2 - From a (temporary) file
### ✅DEFECT - ID:2.3 - Print the record in the same order as it was provided
### EPIC - ID:3 - Retrieve items
### EPIC - ID:4 - Download items from HackerNews
### ✅STORY - ID:4.1 - Create a new stubbed HTTP server
### ✅STORY - ID:4.2 - Download the items from a URL
### ✅STORY - ID:4.3 - Commit to git using the date of publishing of the hackernews item
### EPIC - ID:5 - Create a new wrapper on top of recutils library
#### STORY - ID:5.1 - Create it
#### STORY - ID:5.2 - Publish it to crates.io
### EPIC - ID:7 - Replace the Go Stubs with Rust Stubs
#### STORY - ID:7.1 - Create Rust Stubs for existing Go Stubs
##### Acceptance Criteria:
* For each existing Go Stub (see docker-compose.yml), create a corresponding Rust Stub that replicates the functionality.
* Remove the dependency to the Go Stubs from the project and to docker.
* Ensure that the Rust Stubs can be easily started and stopped, similar to the Go Stubs.
* In the Makefile, replace the commands that start and stop the Go Stubs with commands to start and stop the Rust Stubs.
#### STORY - ID:7.2 - Replace Go Stubs with Rust Stubs in the project
### EPIC - ID:8 - Migrate existing codebase from Go to Rust

