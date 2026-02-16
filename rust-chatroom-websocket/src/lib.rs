pub mod chat;
pub mod config;
pub mod db;
pub mod logging;
pub mod server;

pub fn run() {
    logging::init();
    let cfg = config::config();

    log::info!("Logging in {} level", cfg.log_level);
    log::info!("Running app '{}' in {} mode", cfg.app_name, cfg.env_name);
    log::info!("Listening on {}:{}", cfg.app_host, cfg.app_port);
}
