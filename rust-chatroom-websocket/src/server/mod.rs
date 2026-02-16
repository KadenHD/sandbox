pub mod router;
pub mod websocket;

use crate::{config, logging};
use crate::chat::manager::ChatManager;

use axum::Router;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub chat_manager: Arc<ChatManager>,
}

pub async fn run() -> anyhow::Result<()> {
    let cfg = config::config();
    logging::init(&cfg.log_level, None);

    let chat_manager = Arc::new(ChatManager::new());

    let state = AppState {
        chat_manager,
    };

    let app: Router = router::create_router(state.clone());

    let listener = tokio::net::TcpListener::bind(
        format!("{}:{}", cfg.app_host, cfg.app_port)
    ).await?;

    log::info!("Logging in '{}' level", cfg.log_level);
    log::info!("Running app '{}' in '{}' mode", cfg.app_name, cfg.env_name);
    log::info!("Listening on '{}:{}'", cfg.app_host, cfg.app_port);

    axum::serve(listener, app).await?;

    Ok(())
}
