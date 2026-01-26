extern crate core;

pub mod commands;
pub mod common;
pub mod config;
pub mod configuration;
pub mod print;

#[cfg(test)]
pub mod tests {
    use log::{debug, error, info, warn};

    #[test]
    fn test_log_levels() {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init()
            .ok();

        debug!("This is a debug message");
        info!("This is an info message");
        warn!("This is a warning message");
        error!("This is an error message");
    }

    #[test]
    fn test_logging_with_formatting() {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .is_test(true)
            .try_init()
            .ok();

        let context = "test_context";
        let value = 42;
        info!("Context: {}, Value: {}", context, value);
        warn!("Warning with debug values: {:?}", vec![1, 2, 3]);
    }

    #[test]
    fn test_logging_initialization() {
        let result = std::panic::catch_unwind(|| {
            env_logger::Builder::from_default_env()
                .filter_level(log::LevelFilter::Info)
                .is_test(true)
                .try_init()
        });

        assert!(result.is_ok(), "Logger initialization failed");
    }
}
