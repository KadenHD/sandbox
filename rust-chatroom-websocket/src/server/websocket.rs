use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Path, State},
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use crate::chat::ChatMessage;
use super::AppState;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Path((room_name, username)): Path<(String, String)>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, room_name, username))
}

async fn handle_socket(
    socket: WebSocket,
    state: AppState,
    room_name: String,
    username: String,
) {
    let room = state.chat_manager.get_or_create_room(&room_name).await;
    let mut rx = room.sender.subscribe();
    let tx = room.sender.clone();

    let (mut sender, mut receiver) = socket.split();

    // Send recent history
    let history = room.get_history().await;
    for msg in history {
        if sender.send(Message::Text(msg.to_json().into())).await.is_err() {
            break;
        }
    }

    // Notify join
    let join_msg = ChatMessage::new("system", format!("{} joined the room", username));
    room.add_message(join_msg.clone()).await;
    let _ = tx.send(join_msg.to_json());

    // Task 1: send broadcast messages to this client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg_json) = rx.recv().await {
            if sender.send(Message::Text(msg_json.into())).await.is_err() {
                break;
            }
        }
    });

    // Task 2: receive messages from this client and broadcast
    let tx_clone = tx.clone();
    let username_clone = username.clone();
    let room_clone = room.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text_bytes) = msg {
                let text: String = text_bytes.to_string();
                let chat_msg = ChatMessage::new(username_clone.clone(), text);
                room_clone.add_message(chat_msg.clone()).await;
                let _ = tx_clone.send(chat_msg.to_json());
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    // Notify leave
    let leave_msg = ChatMessage::new("system", format!("{} left the room", username));
    room.add_message(leave_msg.clone()).await;
    let _ = tx.send(leave_msg.to_json());
}
