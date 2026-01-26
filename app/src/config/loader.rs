use super::structs::{ConfigError, ConfigFile};
use log::debug;
use std::fs;
use std::path::{Path, PathBuf};

/// Expand tilde (~) in paths to user's home directory
fn expand_home_dir(path: &str) -> PathBuf {
    if let Some(path_without_tilde) = path.strip_prefix("~") {
        if let Some(home_dir) = dirs::home_dir() {
            return home_dir.join(path_without_tilde);
        }
    }
    PathBuf::from(path)
}

/// Read TOML configuration from a file path
pub fn read_toml_file(path: &Path) -> Result<ConfigFile, ConfigError> {
    debug!("Reading TOML config from: {}", path.display());
    let contents =
        fs::read_to_string(path).map_err(|_| ConfigError::FileNotFound(path.to_path_buf()))?;
    let config: ConfigFile = toml::from_str(&contents)?;
    debug!("Successfully parsed TOML config");
    Ok(config)
}

/// Read configuration from default home directory location: ~/.config/lc/config.toml
pub fn read_from_home_config() -> Result<ConfigFile, ConfigError> {
    if let Some(config_dir) = dirs::config_dir() {
        let config_path = config_dir.join("lc").join("config.toml");
        if config_path.exists() {
            return read_toml_file(&config_path);
        }
    }
    Err(ConfigError::FileNotFound(PathBuf::from(
        "~/.config/lc/config.toml",
    )))
}

/// Parse .env.local file and extract LC_* variables into a ConfigFile
pub fn read_from_env_local() -> Result<ConfigFile, ConfigError> {
    let env_local_path = Path::new(".env.local");

    if !env_local_path.exists() {
        return Err(ConfigError::FileNotFound(env_local_path.to_path_buf()));
    }

    debug!("Reading .env.local configuration");
    dotenvy::from_filename(".env.local").ok();

    let mut config = ConfigFile::default();
    let mut any_found = false;

    // Check for LC_* environment variables that were loaded from .env.local
    if let Ok(db) = std::env::var("LC_DATABASE_FILE") {
        if !db.is_empty() {
            config.database_file = db;
            any_found = true;
        }
    }

    if let Ok(level) = std::env::var("LC_LOG_LEVEL") {
        if !level.is_empty() {
            config.log_level = level;
            any_found = true;
        }
    }

    if let Ok(tmpl) = std::env::var("LC_TEMPLATE_FILE") {
        if !tmpl.is_empty() {
            config.template_file = tmpl;
            any_found = true;
        }
    }

    if let Ok(url) = std::env::var("LC_HACKER_NEWS_URL") {
        if !url.is_empty() {
            config.hacker_news_url = url;
            any_found = true;
        }
    }

    if any_found {
        debug!("Successfully read .env.local configuration");
        Ok(config)
    } else {
        Err(ConfigError::FileNotFound(env_local_path.to_path_buf()))
    }
}

/// Read configuration from environment variables: LC_DATABASE_FILE, LC_LOG_LEVEL, etc.
pub fn read_from_env_vars() -> ConfigFile {
    let mut config = ConfigFile::default();
    let mut any_found = false;

    if let Ok(db) = std::env::var("LC_DATABASE_FILE") {
        if !db.is_empty() {
            config.database_file = db;
            any_found = true;
        }
    }

    if let Ok(level) = std::env::var("LC_LOG_LEVEL") {
        if !level.is_empty() {
            config.log_level = level;
            any_found = true;
        }
    }

    if let Ok(tmpl) = std::env::var("LC_TEMPLATE_FILE") {
        if !tmpl.is_empty() {
            config.template_file = tmpl;
            any_found = true;
        }
    }

    if let Ok(url) = std::env::var("LC_HACKER_NEWS_URL") {
        if !url.is_empty() {
            config.hacker_news_url = url;
            any_found = true;
        }
    }

    if any_found {
        debug!("Found environment variables: LC_*");
    }

    config
}

/// Load configuration following the hierarchy:
/// CLI args > LC_CONFIG_FILE env var > .env.local > ~/.config/lc/config.toml > defaults
pub fn load_configuration(
    config_file_cli: Option<&str>,
    database_file_cli: Option<&str>,
    log_level_cli: Option<&str>,
    template_file_cli: Option<&str>,
    hacker_news_url_cli: Option<&str>,
) -> Result<ConfigFile, ConfigError> {
    let mut config = ConfigFile::default();
    debug!("Starting configuration load with hierarchy");

    // 1. Try to load ~/.config/lc/config.toml
    if let Ok(file_config) = read_from_home_config() {
        debug!("Loaded config from ~/.config/lc/config.toml");
        config.merge_with(&file_config);
    }

    // 2. Try to load ./.env.local
    if let Ok(env_local_config) = read_from_env_local() {
        debug!("Loaded config from .env.local");
        config.merge_with(&env_local_config);
    }

    // 3. Try to load from LC_CONFIG_FILE environment variable
    if let Ok(env_config_file) = std::env::var("LC_CONFIG_FILE") {
        if !env_config_file.is_empty() {
            let expanded_path = expand_home_dir(&env_config_file);
            match read_toml_file(&expanded_path) {
                Ok(custom_config) => {
                    debug!(
                        "Loaded config from LC_CONFIG_FILE env var: {}",
                        env_config_file
                    );
                    config.merge_with(&custom_config);
                }
                Err(e) => {
                    debug!("Failed to load from LC_CONFIG_FILE: {:?}", e);
                }
            }
        }
    }

    // 4. Try to load from CLI --config-file argument
    if let Some(config_path) = config_file_cli {
        if !config_path.is_empty() {
            let expanded_path = expand_home_dir(config_path);
            let custom_config = read_toml_file(&expanded_path)?;
            debug!(
                "Loaded config from CLI --config-file argument: {}",
                config_path
            );
            config.merge_with(&custom_config);
        }
    }

    // 5. Merge individual environment variables
    let env_config = read_from_env_vars();
    config.merge_with(&env_config);

    // 6. Merge CLI arguments (highest priority)
    if let Some(db) = database_file_cli {
        if !db.is_empty() {
            config.database_file = db.to_string();
            debug!("Using database_file from CLI: {}", db);
        }
    }

    if let Some(log) = log_level_cli {
        if !log.is_empty() {
            config.log_level = log.to_string();
            debug!("Using log_level from CLI: {}", log);
        }
    }

    if let Some(tmpl) = template_file_cli {
        if !tmpl.is_empty() {
            config.template_file = tmpl.to_string();
            debug!("Using template_file from CLI: {}", tmpl);
        }
    }

    if let Some(url) = hacker_news_url_cli {
        if !url.is_empty() {
            config.hacker_news_url = url.to_string();
            debug!("Using hacker_news_url from CLI: {}", url);
        }
    }

    debug!(
        "Final config: db={}, log_level={}, template={}, hn_url={}",
        config.database_file, config.log_level, config.template_file, config.hacker_news_url
    );

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_expand_home_dir() {
        let expanded = expand_home_dir("~/test/path");
        assert!(expanded.to_string_lossy().contains("test/path"));

        let not_tilde = expand_home_dir("/absolute/path");
        assert_eq!(not_tilde, PathBuf::from("/absolute/path"));
    }

    #[test]
    fn test_read_toml_file() {
        let dir = TempDir::new().unwrap();
        let config_path = dir.path().join("config.toml");

        fs::write(
            &config_path,
            r#"
database_file = "/custom/db.rec"
log_level = "debug"
template_file = "/custom/template/"
hacker_news_url = "https://custom.url"
"#,
        )
        .unwrap();

        let config = read_toml_file(&config_path).unwrap();
        assert_eq!(config.database_file, "/custom/db.rec");
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.template_file, "/custom/template/");
        assert_eq!(config.hacker_news_url, "https://custom.url");
    }

    #[test]
    fn test_read_toml_file_not_found() {
        let result = read_toml_file(Path::new("/nonexistent/path.toml"));
        assert!(matches!(result, Err(ConfigError::FileNotFound(_))));
    }

    #[test]
    fn test_read_toml_file_invalid_syntax() {
        let dir = TempDir::new().unwrap();
        let config_path = dir.path().join("config.toml");

        fs::write(&config_path, "invalid [toml syntax").unwrap();

        let result = read_toml_file(&config_path);
        assert!(matches!(result, Err(ConfigError::ParseError(_))));
    }

    #[test]
    fn test_read_from_env_local() {
        let dir = TempDir::new().unwrap();
        let current_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        let env_local_path = dir.path().join(".env.local");
        fs::write(
            &env_local_path,
            r#"
# Comment
LC_DATABASE_FILE=/custom/db.rec
LC_LOG_LEVEL=debug
LC_TEMPLATE_FILE=/custom/template/
"#,
        )
        .unwrap();

        let result = read_from_env_local();
        std::env::set_current_dir(current_dir).unwrap();

        let config = result.unwrap();
        assert_eq!(config.database_file, "/custom/db.rec");
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.template_file, "/custom/template/");
        // hacker_news_url should remain default
        assert_eq!(config.hacker_news_url, "http://0.0.0.0:8181");
    }

    #[test]
    fn test_load_configuration_with_cli_args() {
        let config = load_configuration(
            None,
            Some("/cli/db.rec"),
            Some("trace"),
            None,
            Some("https://cli.url"),
        )
        .unwrap();

        assert_eq!(config.database_file, "/cli/db.rec");
        assert_eq!(config.log_level, "trace");
        assert_eq!(config.template_file, "./data/template/");
        assert_eq!(config.hacker_news_url, "https://cli.url");
    }

    #[test]
    fn test_load_configuration_defaults() {
        let config = load_configuration(None, None, None, None, None).unwrap();

        assert_eq!(config.database_file, "./data/database/links.rec");
        assert_eq!(config.log_level, "info");
        assert_eq!(config.template_file, "./data/template/");
        assert_eq!(config.hacker_news_url, "http://0.0.0.0:8181");
    }
}
