use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::room::ChatRoom;

#[derive(Clone)]
pub struct ChatManager {
    rooms: Arc<RwLock<HashMap<String, ChatRoom>>>,
}

impl ChatManager {
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_or_create_room(&self, room_name: &str) -> ChatRoom {
        let mut rooms = self.rooms.write().await;

        rooms
            .entry(room_name.to_string())
            .or_insert_with(|| ChatRoom::new(room_name.to_string()))
            .clone()
    }

    pub async fn list_rooms(&self) -> Vec<String> {
        let rooms = self.rooms.read().await;
        rooms.keys().cloned().collect()
    }
}
