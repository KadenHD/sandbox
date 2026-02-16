use crate::server::{AppState, websocket::websocket_handler};
use axum::{Router, extract::Path, http::header, response::Response, routing::get};
use include_dir::{Dir, include_dir};
use std::sync::Arc;

// Embed static folder
static STATIC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/static");

// Dynamic file serving with proper MIME type
async fn serve_file(Path(file_path): Path<String>) -> Response {
    if let Some(file) = STATIC_DIR.get_file(&file_path) {
        let content = file.contents_utf8().unwrap_or_default();
        // Determine MIME type
        let mime = if file_path.ends_with(".css") {
            "text/css"
        } else if file_path.ends_with(".js") {
            "application/javascript"
        } else if file_path.ends_with(".html") {
            "text/html"
        } else {
            "text/plain"
        };

        Response::builder()
            .header(header::CONTENT_TYPE, mime)
            .body(content.into())
            .unwrap()
    } else {
        Response::builder()
            .status(404)
            .header(header::CONTENT_TYPE, "text/html")
            .body("<h1>File not found</h1>".into())
            .unwrap()
    }
}

pub fn create_router(state: AppState) -> Router {
    let chat_manager = state.chat_manager.clone();

    Router::new()
        // Root page
        .route(
            "/",
            get(|| async { serve_file(Path("index.html".to_string())).await }),
        )
        // Serve all other static files dynamically
        .route("/static/{*file_path}", get(serve_file))
        // WebSocket route
        .route(
            "/ws/{room}/{username}",
            get(
                move |ws: axum::extract::WebSocketUpgrade,
                      Path((room_name, username)): Path<(String, String)>| {
                    let chat_manager = chat_manager.clone();
                    async move {
                        let room = chat_manager.get_or_create_room(&room_name).await;
                        websocket_handler(ws, Arc::new(room), username).await
                    }
                },
            ),
        )
}
