use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ChatHistory {
    pub message_id: Uuid,
    pub chat_id: Uuid,
    pub user_message: Option<String>,
    pub assistant: Option<String>,
    pub message_time: Option<DateTime<Utc>>,
    pub documents_upload: Option<Vec<String>>,
}