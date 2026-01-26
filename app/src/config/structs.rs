use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

/// Configuration file structure for TOML deserialization
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ConfigFile {
    #[serde(default)]
    pub database_file: String,
    #[serde(default)]
    pub log_level: String,
    #[serde(default)]
    pub template_file: String,
    #[serde(default)]
    pub hacker_news_url: String,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            database_file: "./data/database/links.rec".to_string(),
            log_level: "info".to_string(),
            template_file: "./data/template/".to_string(),
            hacker_news_url: "http://0.0.0.0:8181".to_string(),
        }
    }
}

impl ConfigFile {
    /// Merge other configuration into self, with other's non-empty values taking precedence
    pub fn merge_with(&mut self, other: &ConfigFile) {
        if !other.database_file.is_empty() && other.database_file != "./data/database/links.rec" {
            self.database_file = other.database_file.clone();
        }
        if !other.log_level.is_empty() && other.log_level != "info" {
            self.log_level = other.log_level.clone();
        }
        if !other.template_file.is_empty() && other.template_file != "./data/template/" {
            self.template_file = other.template_file.clone();
        }
        if !other.hacker_news_url.is_empty() && other.hacker_news_url != "http://0.0.0.0:8181" {
            self.hacker_news_url = other.hacker_news_url.clone();
        }
    }

    /// Parse log level string into a log::LevelFilter
    pub fn parse_log_level(&self) -> Result<log::LevelFilter, String> {
        match self.log_level.to_lowercase().as_str() {
            "trace" => Ok(log::LevelFilter::Trace),
            "debug" => Ok(log::LevelFilter::Debug),
            "info" => Ok(log::LevelFilter::Info),
            "warn" => Ok(log::LevelFilter::Warn),
            "error" => Ok(log::LevelFilter::Error),
            "off" => Ok(log::LevelFilter::Off),
            _ => Err(format!(
                "Invalid log level '{}'. Valid levels are: trace, debug, info, warn, error, off",
                self.log_level
            )),
        }
    }
}

/// Configuration error types
#[derive(Debug)]
pub enum ConfigError {
    FileNotFound(PathBuf),
    ParseError(String),
    InvalidLogLevel(String),
    IoError(std::io::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileNotFound(path) => {
                write!(f, "Configuration file not found: {}", path.display())
            }
            ConfigError::ParseError(msg) => write!(f, "Failed to parse configuration: {}", msg),
            ConfigError::InvalidLogLevel(level) => {
                write!(f, "Invalid log level '{}'. Valid levels are: trace, debug, info, warn, error, off", level)
            }
            ConfigError::IoError(err) => write!(f, "IO error reading configuration: {}", err),
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::IoError(err)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::ParseError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ConfigFile::default();
        assert_eq!(config.database_file, "./data/database/links.rec");
        assert_eq!(config.log_level, "info");
        assert_eq!(config.template_file, "./data/template/");
        assert_eq!(config.hacker_news_url, "http://0.0.0.0:8181");
    }

    #[test]
    fn test_parse_log_level() {
        let mut config = ConfigFile::default();

        config.log_level = "trace".to_string();
        assert_eq!(config.parse_log_level().unwrap(), log::LevelFilter::Trace);

        config.log_level = "debug".to_string();
        assert_eq!(config.parse_log_level().unwrap(), log::LevelFilter::Debug);

        config.log_level = "info".to_string();
        assert_eq!(config.parse_log_level().unwrap(), log::LevelFilter::Info);

        config.log_level = "warn".to_string();
        assert_eq!(config.parse_log_level().unwrap(), log::LevelFilter::Warn);

        config.log_level = "error".to_string();
        assert_eq!(config.parse_log_level().unwrap(), log::LevelFilter::Error);

        config.log_level = "off".to_string();
        assert_eq!(config.parse_log_level().unwrap(), log::LevelFilter::Off);
    }

    #[test]
    fn test_parse_log_level_case_insensitive() {
        let mut config = ConfigFile::default();

        config.log_level = "DEBUG".to_string();
        assert_eq!(config.parse_log_level().unwrap(), log::LevelFilter::Debug);

        config.log_level = "Info".to_string();
        assert_eq!(config.parse_log_level().unwrap(), log::LevelFilter::Info);
    }

    #[test]
    fn test_parse_log_level_invalid() {
        let mut config = ConfigFile::default();
        config.log_level = "invalid".to_string();
        assert!(config.parse_log_level().is_err());
    }

    #[test]
    fn test_merge_with() {
        let mut config = ConfigFile::default();
        let other = ConfigFile {
            database_file: "/custom/path".to_string(),
            log_level: "debug".to_string(),
            template_file: "./data/template/".to_string(),
            hacker_news_url: "https://custom.url".to_string(),
        };

        config.merge_with(&other);

        assert_eq!(config.database_file, "/custom/path");
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.template_file, "./data/template/");
        assert_eq!(config.hacker_news_url, "https://custom.url");
    }

    #[test]
    fn test_merge_with_empty_values() {
        let mut config = ConfigFile {
            database_file: "/original/path".to_string(),
            log_level: "warn".to_string(),
            template_file: "/original/template/".to_string(),
            hacker_news_url: "https://original.url".to_string(),
        };

        let other = ConfigFile {
            database_file: String::new(),
            log_level: String::new(),
            template_file: String::new(),
            hacker_news_url: String::new(),
        };

        config.merge_with(&other);

        // Empty values should not override existing ones
        assert_eq!(config.database_file, "/original/path");
        assert_eq!(config.log_level, "warn");
        assert_eq!(config.template_file, "/original/template/");
        assert_eq!(config.hacker_news_url, "https://original.url");
    }
}
