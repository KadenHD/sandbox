use crate::chat::room::ChatRoom;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ChatManager {
    rooms: Arc<RwLock<HashMap<String, ChatRoom>>>,
    db_pool: Arc<PgPool>,
}

impl ChatManager {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self {
            rooms: Arc::new(RwLock::new(HashMap::new())),
            db_pool,
        }
    }

    pub fn pool(&self) -> Arc<PgPool> {
        self.db_pool.clone()
    }

    pub async fn get_or_create_room(&self, room_name: &str) -> ChatRoom {
        let mut rooms = self.rooms.write().await;
        rooms
            .entry(room_name.to_string())
            .or_insert_with(|| ChatRoom::new(room_name.to_string(), self.db_pool.clone()))
            .clone()
    }
}
