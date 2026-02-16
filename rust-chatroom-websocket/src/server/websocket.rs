use crate::chat::message::ChatMessage;
use crate::chat::room::ChatRoom;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    room: Arc<ChatRoom>,
    username: String,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, room, username))
}

async fn handle_socket(socket: WebSocket, room: Arc<ChatRoom>, username: String) {
    log::trace!("User attempting to join: {}", username);

    if !room.join(username.clone()).await {
        log::trace!("User already in room: {}", username);
        return;
    }

    log::trace!("User joined: {}", username);

    let (mut sender, mut receiver) = socket.split();

    // Send history
    let history = room.load_history(1000).await;
    for msg in history {
        if sender
            .send(Message::Text(msg.to_json().into()))
            .await
            .is_err()
        {
            room.leave(username.clone()).await;
            return;
        }
    }

    let mut broadcast_rx = room.sender.subscribe();
    let room_clone = room.clone();
    let username_clone = username.clone();

    // Create two async tasks
    let send_task = tokio::spawn(async move {
        while let Ok(msg_json) = broadcast_rx.recv().await {
            if sender.send(Message::Text(msg_json.into())).await.is_err() {
                break;
            }
        }
    });

    let receive_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    let chat_msg = ChatMessage::new(username.clone(), text.to_string());
                    room.add_message(chat_msg).await;
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    });

    // Wait until one of them finishes
    tokio::select! {
        _ = send_task => {},
        _ = receive_task => {},
    }

    log::trace!("User disconnecting: {}", username_clone);

    room_clone.leave(username_clone).await;

    log::trace!("User left cleanly");
}
