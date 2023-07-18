use cfg_if::cfg_if;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ChatMessagePair {
    pub message_id: Uuid,
    pub chat_id: Uuid,
    pub user_message: Option<String>,
    pub assistant_message: Option<String>,
    pub message_time: Option<DateTime<Utc>>,
    pub documents_upload: Option<Vec<String>>,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use async_trait::async_trait;
    use sqlx::SqlitePool;
    use axum_session_auth::{SessionSqlitePool, Authentication, HasPermission};
    
    impl ChatMessagePair {
        pub async fn get(message_id: Uuid, pool: &SqlitePool) -> Option<Self> {
            let sql_chat_message_pair = sqlx::query_as::<_, SqlChatMessagePair>("SELECT * FROM chat_message_pairs WHERE message_id = ?")
                .bind(message_id.to_string())
                .fetch(pool)
                .await
                .ok()?;

            Some(sql_chat_message_pair.into_chat_message_pair())
        }

        pub async fn update_chat_message_pair(chat_id:Uuid, user_message: String, assistant_message: String, pool: &SqlitePool) -> Option<Self> {

            sqlx::query_as::<_, SqlChatMessagePair>("UPDATE chat_message_pairs SET user_message = ?, assistant_message = ? WHERE chat_id = ?")
                .bind(user_message)
                .bind(assistant_message)
                .bind(chat_id.to_string())
                .execute(pool)
                .await
                .ok()?;
        }

        pub fn new_chat_message_pair(chat_id: Uuid, user_message: String, assistant_message: String, documents_upload: Vec<String>) -> Self {
            Self {
                message_id: Uuid::new_v4(),
                chat_id: chat_id,
                user_message: Some(user_message),
                assistant_message: Some(assistant_message),
                message_time: Some(Utc::now()),
                documents_upload: Some(documents_upload),
        }
        }
    }
    

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlChatMessagePair {
        pub message_id: String,
        pub chat_id: String,
        pub user_message: Option<String>,
        pub assistant_message: Option<String>,
        pub message_time: Option<DateTime<Utc>>,
    }
    
    impl SqlChatMessagePair {
        pub fn into_chat_message_pair(self) -> ChatMessagePair {
            ChatMessagePair {
                message_id: Uuid::parse_str(&self.message_id).expect("Failed to parse UUID"),
                chat_id: Uuid::parse_str(&self.chat_id).expect("Failed to parse UUID"),
                user_message: Some(self.user_message),
                assistant_message: Some(self.assistant_message),
                message_time: Some(self.message_time),
                documents_upload: Some(self.documents_upload),
            }
        }
    }

}
}


    // #[async_trait]
    // impl Authentication<User, Uuid, SqlitePool> for User {
    //     async fn load_chat_message_pair(message_id: Uuid, pool: Option<&SqlitePool>) -> Result<User, anyhow::Error> {
    //         let pool = pool.unwrap();

    //         ChatMessagePair::get(message_id, pool)
    //             .await
    //             .ok_or_else(|| anyhow::anyhow!("Cannot get user"))
    //     }

    //     fn is_authenticated(&self) -> bool {
    //         true
    //     }

    //     fn is_active(&self) -> bool {
    //         true
    //     }

    //     fn is_anonymous(&self) -> bool {
    //         false
    //     }
    // }