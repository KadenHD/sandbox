use rust_chatroom_websocket::config::settings::Config;
use std::env;

pub fn setup_test_env() {
    unsafe {
        env::set_var("APP_ENV", "test");
        env::set_var("APP_NAME", "test-app");
        env::set_var("APP_HOST", "0.0.0.0");
        env::set_var("APP_PORT", "3001");
        env::set_var("LOG_LEVEL", "debug");
        env::set_var("DATABASE_URL", "sqlite::memory:");
    }
}

#[test]
fn test_config_from_env() {
    setup_test_env();

    let config = Config::from_env();

    assert_eq!(config.env_name, "test");
    assert_eq!(config.app_name, "test-app");
    assert_eq!(config.app_host, "0.0.0.0");
    assert_eq!(config.app_port, 3001);
    assert_eq!(config.log_level, "debug");
    assert_eq!(config.database_url, "sqlite::memory:");
}
