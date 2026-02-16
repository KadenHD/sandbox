pub mod chat;
pub mod config;
pub mod db;
pub mod logging;
pub mod server;

/// Starts the application
pub fn run() {
    let cfg = config::config();
    logging::init(&cfg.log_level, None);

    log::info!("Logging in '{}' level", cfg.log_level);
    log::info!("Running app '{}' in '{}' mode", cfg.app_name, cfg.env_name);
    log::info!("Listening on '{}:{}'", cfg.app_host, cfg.app_port);
}
