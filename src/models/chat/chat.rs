
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::chat::chat_history::ChatHistory;

#[derive(Debug, Clone)]
pub struct Chat {
    pub chat_id: Uuid,
    pub user_id: Option<Uuid>,
    pub creation_time: Option<DateTime<Utc>>,
    pub history: Vec<ChatHistory>,  // history is now a list of ChatHistory objects
    pub chat_name: Option<String>,
}
