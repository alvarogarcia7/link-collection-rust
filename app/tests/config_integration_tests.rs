use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_config_hierarchy_defaults_only() {
    // Test that defaults are used when no config files exist
    let config = select::config::load_configuration(None, None, None, None, None)
        .expect("Should load default configuration");

    assert_eq!(config.database_file, "./data/database/links.rec");
    assert_eq!(config.log_level, "info");
    assert_eq!(config.template_file, "./data/template/");
    assert_eq!(config.hacker_news_url, "http://0.0.0.0:8181");
}

#[test]
fn test_config_hierarchy_cli_overrides_everything() {
    let config = select::config::load_configuration(
        None,
        Some("/cli/database.rec"),
        Some("debug"),
        Some("/cli/templates/"),
        Some("https://custom.url"),
    )
    .expect("Should load configuration");

    // CLI args should override defaults
    assert_eq!(config.database_file, "/cli/database.rec");
    assert_eq!(config.log_level, "debug");
    assert_eq!(config.template_file, "/cli/templates/");
    assert_eq!(config.hacker_news_url, "https://custom.url");
}

#[test]
fn test_config_hierarchy_partial_cli_override() {
    // Only override some values via CLI
    let config = select::config::load_configuration(
        None,
        Some("/cli/database.rec"),
        None, // Don't override log_level
        None,
        None,
    )
    .expect("Should load configuration");

    // Database should be from CLI
    assert_eq!(config.database_file, "/cli/database.rec");
    // Others should be defaults
    assert_eq!(config.log_level, "info");
    assert_eq!(config.template_file, "./data/template/");
    assert_eq!(config.hacker_news_url, "http://0.0.0.0:8181");
}

#[test]
fn test_config_parse_log_level() {
    let mut config = select::config::ConfigFile::default();

    // Test valid levels
    let valid_levels = vec!["trace", "debug", "info", "warn", "error", "off"];
    for level in valid_levels {
        config.log_level = level.to_string();
        assert!(
            config.parse_log_level().is_ok(),
            "Failed to parse valid log level: {}",
            level
        );
    }

    // Test case insensitivity
    config.log_level = "DEBUG".to_string();
    assert!(config.parse_log_level().is_ok());

    config.log_level = "Info".to_string();
    assert!(config.parse_log_level().is_ok());

    // Test invalid level
    config.log_level = "invalid".to_string();
    assert!(config.parse_log_level().is_err());
}

#[test]
fn test_config_merge() {
    let mut config1 = select::config::ConfigFile {
        database_file: "/original/db.rec".to_string(),
        log_level: "info".to_string(),
        template_file: "/original/templates/".to_string(),
        hacker_news_url: "https://original.url".to_string(),
    };

    let config2 = select::config::ConfigFile {
        database_file: "/override/db.rec".to_string(),
        log_level: "debug".to_string(),
        template_file: "/original/templates/".to_string(), // Same as original
        hacker_news_url: String::new(),                    // Empty - should not override
    };

    config1.merge_with(&config2);

    // Database and log level should be overridden
    assert_eq!(config1.database_file, "/override/db.rec");
    assert_eq!(config1.log_level, "debug");
    // Template should remain (unchanged)
    assert_eq!(config1.template_file, "/original/templates/");
    // URL should remain (empty not merged)
    assert_eq!(config1.hacker_news_url, "https://original.url");
}

#[test]
fn test_config_error_display() {
    let err1 = select::config::ConfigError::FileNotFound(std::path::PathBuf::from("/missing.toml"));
    assert!(err1.to_string().contains("not found"));

    let err2 = select::config::ConfigError::ParseError("invalid syntax".to_string());
    assert!(err2.to_string().contains("parse"));

    let err3 = select::config::ConfigError::InvalidLogLevel("invalid".to_string());
    assert!(err3.to_string().contains("invalid"));
}

#[test]
fn test_toml_file_read_basic() {
    let dir = TempDir::new().unwrap();
    let config_path = dir.path().join("config.toml");

    fs::write(
        &config_path,
        r#"
database_file = "/test/db.rec"
log_level = "debug"
template_file = "/test/templates/"
hacker_news_url = "https://test.url"
"#,
    )
    .unwrap();

    let config =
        select::config::loader::read_toml_file(&config_path).expect("Should read TOML file");

    assert_eq!(config.database_file, "/test/db.rec");
    assert_eq!(config.log_level, "debug");
    assert_eq!(config.template_file, "/test/templates/");
    assert_eq!(config.hacker_news_url, "https://test.url");
}

#[test]
fn test_toml_file_read_partial() {
    // Test TOML file with only some fields
    let dir = TempDir::new().unwrap();
    let config_path = dir.path().join("config.toml");

    fs::write(
        &config_path,
        r#"
database_file = "/test/db.rec"
log_level = "warn"
"#,
    )
    .unwrap();

    let config =
        select::config::loader::read_toml_file(&config_path).expect("Should read TOML file");

    assert_eq!(config.database_file, "/test/db.rec");
    assert_eq!(config.log_level, "warn");
    // Other fields should be from TOML defaults or Serde defaults
}

#[test]
fn test_toml_file_not_found() {
    let result = select::config::loader::read_toml_file(Path::new("/nonexistent/config.toml"));
    assert!(result.is_err());
}

#[test]
fn test_toml_file_invalid_syntax() {
    let dir = TempDir::new().unwrap();
    let config_path = dir.path().join("config.toml");

    fs::write(&config_path, "invalid [toml syntax").unwrap();

    let result = select::config::loader::read_toml_file(&config_path);
    assert!(result.is_err());
}
