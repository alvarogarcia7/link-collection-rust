pub mod loader;
pub mod structs;

pub use loader::load_configuration;
pub use structs::{ConfigError, ConfigFile};
