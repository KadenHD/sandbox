use axum::{routing::get, Router};
use super::{websocket::websocket_handler, AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/ws/{room}/{username}", get(websocket_handler))
        .with_state(state)
}
