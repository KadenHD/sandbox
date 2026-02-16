pub mod chat;
pub mod config;
pub mod db;
pub mod logging;
pub mod server;

pub fn run() {
    let cfg = config::config();

    println!("App: {}", cfg.app_name);
    println!("Env: {}", cfg.env_name);
    println!("Listening on {}:{}", cfg.app_host, cfg.app_port);
    println!("Log Level: {}", cfg.log_level);
}
