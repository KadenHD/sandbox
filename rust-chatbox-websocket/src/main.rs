use rust_chatroom_websocket::config::config;

fn main() {
    println!("App: {}", config().app_name);
    println!("Env: {}", config().env_name);
    println!("Listening on {}:{}", config().app_host, config().app_port);
    println!("Log Level: {}", config().log_level);
}
