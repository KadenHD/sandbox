use crate::chat::ChatMessage;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

#[derive(Clone)]
pub struct ChatRoom {
    pub name: String,
    pub sender: broadcast::Sender<String>,
    pub history: Arc<RwLock<Vec<ChatMessage>>>,
}

impl ChatRoom {
    pub fn new(name: String) -> Self {
        let (sender, _) = broadcast::channel(100);
        Self {
            name,
            sender,
            history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn add_message(&self, msg: ChatMessage) {
        let mut hist = self.history.write().await;
        hist.push(msg);

        // Limit history to last 100 messages
        if hist.len() > 100 {
            let drain_count = hist.len() - 100; // calculate first
            hist.drain(0..drain_count);
        }
    }

    pub async fn get_history(&self) -> Vec<ChatMessage> {
        self.history.read().await.clone()
    }
}
