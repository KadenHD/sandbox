pub mod router;
pub mod websocket;

use crate::{chat::manager::ChatManager, config, logging};
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub chat_manager: Arc<ChatManager>,
}

pub async fn run() -> anyhow::Result<()> {
    let cfg = config::config();
    logging::init(&cfg.log_level, None);

    log::info!("Connecting to PostgreSQL...");
    let db_pool = PgPool::connect(&cfg.database_url).await?;
    let db_pool = Arc::new(db_pool);
    log::info!("Connected to PostgreSQL");

    let chat_manager = Arc::new(ChatManager::new(db_pool.clone()));
    let state = AppState { chat_manager };

    let app: Router = router::create_router(state);

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", cfg.app_host, cfg.app_port)).await?;
    log::info!("Server running on {}:{}", cfg.app_host, cfg.app_port);

    axum::serve(listener, app).await?;

    Ok(())
}
