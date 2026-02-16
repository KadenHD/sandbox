use crate::chat::message::ChatMessage;
use sqlx::{PgPool, Row};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};

#[derive(Clone)]
pub struct ChatRoom {
    pub name: String,
    pub sender: broadcast::Sender<String>, // broadcast channel
    pub history: Arc<RwLock<Vec<ChatMessage>>>, // in-memory history
    pub users: Arc<RwLock<HashSet<String>>>, // active users
    pub db_pool: Arc<PgPool>,
}

impl ChatRoom {
    pub fn new(name: String, db_pool: Arc<PgPool>) -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self {
            name,
            sender,
            history: Arc::new(RwLock::new(Vec::new())),
            users: Arc::new(RwLock::new(HashSet::new())),
            db_pool,
        }
    }

    pub async fn join(&self, username: String) -> bool {
        let mut users = self.users.write().await;
        if users.contains(&username) {
            return false;
        }
        users.insert(username.clone());

        let join_msg = ChatMessage::system(format!("{} joined the room", username));
        self.add_message(join_msg).await; // push to history, broadcast, DB

        true
    }

    pub async fn leave(&self, username: String) {
        let mut users = self.users.write().await;
        if users.remove(&username) {
            let leave_msg = ChatMessage::system(format!("{} left the room", username));
            self.add_message(leave_msg).await;
        }
    }

    pub async fn add_message(&self, msg: ChatMessage) {
        // Push to in-memory history
        self.history.write().await.push(msg.clone());

        // Send to broadcast channel (live updates)
        let _ = self.sender.send(msg.to_json());

        // Insert into database
        let _ = sqlx::query(
            "INSERT INTO chat_messages (id, room_name, sender, content, timestamp)
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(msg.id)
        .bind(&self.name)
        .bind(&msg.sender)
        .bind(&msg.content)
        .bind(msg.timestamp)
        .execute(&*self.db_pool)
        .await;
    }

    pub async fn load_history(&self, limit: i64) -> Vec<ChatMessage> {
        // Load from DB
        let rows = sqlx::query(
            "SELECT id, sender, content, timestamp
             FROM chat_messages
             WHERE room_name = $1
             ORDER BY timestamp ASC
             LIMIT $2",
        )
        .bind(&self.name)
        .bind(limit)
        .fetch_all(&*self.db_pool)
        .await
        .unwrap_or_default();

        rows.into_iter()
            .map(|r| ChatMessage {
                id: r.try_get("id").unwrap(),
                sender: r.try_get("sender").unwrap(),
                content: r.try_get("content").unwrap(),
                timestamp: r.try_get("timestamp").unwrap(),
            })
            .collect()
    }

    pub async fn active_users(&self) -> Vec<String> {
        self.users.read().await.iter().cloned().collect()
    }
}
