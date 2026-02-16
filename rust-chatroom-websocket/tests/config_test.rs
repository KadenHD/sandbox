use rust_chatroom_websocket::config::{config, Config};
use std::{env, fs};

/// Helper to clear all relevant env vars before each test
fn clear_env() {
    unsafe {
        env::remove_var("APP_ENV");
        env::remove_var("APP_NAME");
        env::remove_var("APP_HOST");
        env::remove_var("APP_PORT");
        env::remove_var("LOG_LEVEL");
        env::remove_var("DATABASE_URL");
    }
}

#[test]
fn test_config_from_env_all_values_set() {
    clear_env();

    unsafe {
        env::set_var("APP_ENV", "test");
        env::set_var("APP_NAME", "test-app");
        env::set_var("APP_HOST", "0.0.0.0");
        env::set_var("APP_PORT", "3001");
        env::set_var("LOG_LEVEL", "debug");
        env::set_var("DATABASE_URL", "sqlite::memory:");
    }

    let config = Config::from_env();

    assert_eq!(config.env_name, "test");
    assert_eq!(config.app_name, "test-app");
    assert_eq!(config.app_host, "0.0.0.0");
    assert_eq!(config.app_port, 3001);
    assert_eq!(config.log_level, "debug");
    assert_eq!(config.database_url, "sqlite::memory:");
}

#[test]
fn test_defaults_are_used() {
    clear_env();

    unsafe {
        env::set_var("APP_ENV", "test");
        env::set_var("DATABASE_URL", "sqlite::memory:");
    }

    let config = Config::from_env();

    assert_eq!(config.app_name, "rust-chatroom-websocket");
    assert_eq!(config.app_host, "127.0.0.1");
    assert_eq!(config.app_port, 8080);
    assert_eq!(config.log_level, "info");
}

#[test]
#[should_panic(expected = "APP_PORT must be a valid u16")]
fn test_invalid_port_panics() {
    clear_env();

    unsafe {
        env::set_var("APP_ENV", "test");
        env::set_var("APP_PORT", "invalid");
        env::set_var("DATABASE_URL", "sqlite::memory:");
    }

    Config::from_env();
}

#[test]
#[should_panic(expected = "DATABASE_URL must be set")]
fn test_missing_database_url_panics() {
    clear_env();

    unsafe {
        env::set_var("APP_ENV", "test");
    }

    Config::from_env();
}

#[test]
fn test_env_file_loading_success() {
    clear_env();

    // Create temporary .env.temp file
    fs::write(
        ".env.temp",
        r#"
APP_NAME=env-file-app
APP_HOST=192.168.1.1
APP_PORT=4000
LOG_LEVEL=trace
DATABASE_URL=sqlite::memory:
"#,
    )
    .unwrap();

    unsafe {
        env::set_var("APP_ENV", "temp");
    }

    let config = Config::from_env();

    assert_eq!(config.env_name, "temp");
    assert_eq!(config.app_name, "env-file-app");
    assert_eq!(config.app_host, "192.168.1.1");
    assert_eq!(config.app_port, 4000);
    assert_eq!(config.log_level, "trace");
    assert_eq!(config.database_url, "sqlite::memory:");

    fs::remove_file(".env.temp").unwrap();
}

#[test]
#[should_panic(expected = "Missing .env.missing")]
fn test_missing_env_file_panics() {
    clear_env();

    unsafe {
        env::set_var("APP_ENV", "missing");
    }

    Config::from_env();
}

#[test]
fn test_static_config_accessor() {
    clear_env();

    unsafe {
        env::set_var("APP_ENV", "test");
        env::set_var("DATABASE_URL", "sqlite::memory:");
    }

    let cfg = config();

    assert_eq!(cfg.env_name, "test");
    assert_eq!(cfg.database_url, "sqlite::memory:");
}
