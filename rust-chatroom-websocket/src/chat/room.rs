use tokio::sync::broadcast;

#[derive(Clone)]
pub struct ChatRoom {
    pub name: String,
    pub sender: broadcast::Sender<String>,
}

impl ChatRoom {
    pub fn new(name: String) -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { name, sender }
    }
}
