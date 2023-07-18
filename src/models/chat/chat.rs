use cfg_if::cfg_if;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Chat {
    pub chat_id: Uuid,
    pub user_id: Uuid,
    pub creation_time: DateTime<Utc>,
    pub chat_name: Option<String>,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        // use async_trait::async_trait;
        use sqlx::PgPool;
        use crate::models::chat::chat_message_pair::{SqlChatMessagePair,ChatMessagePair};

        impl Chat {
            pub async fn get(chat_id: Uuid, pool: &PgPool) -> Option<Self> {
                let sql_chat = sqlx::query_as::<_, SqlChat>("SELECT * FROM chats WHERE chat_id = $1")
                    .bind(chat_id)
                    .fetch_one(pool)
                    .await
                    .ok()?;

                Some(sql_chat.into_chat())
            }

            pub async fn get_all_chat_message_pairs(&self, pool: &PgPool) -> Option<Vec<ChatMessagePair>> {
                let sql_chat_message_pairs = sqlx::query_as::<_, SqlChatMessagePair>("SELECT * FROM chat_message_pairs WHERE chat_id = $1")
                    .bind(self.chat_id)
                    .fetch_all(pool)
                    .await
                    .ok()?;

                let mut chat_message_pairs = sql_chat_message_pairs
                                                .into_iter()
                                                .map(|sql_chat_message_pair| sql_chat_message_pair.into_chat_message_pair())
                                                .collect::<Vec<ChatMessagePair>>();

                chat_message_pairs.sort_by(|a, b| a.message_time.cmp(&b.message_time));
                Some(chat_message_pairs)
            }

            pub fn new_chat(user_id: Uuid, chat_name: Option<String>) -> Self {
                Self {
                    chat_id: Uuid::new_v4(),
                    user_id: user_id,
                    creation_time: Utc::now(),
                    chat_name: chat_name,
                }
            }
        }

        #[derive(sqlx::FromRow, sqlx::Decode, Clone)]
        pub struct SqlChat {
            pub chat_id: String,
            pub user_id: String,
            pub creation_time: DateTime<Utc>,
            pub chat_name: Option<String>,
        }

        impl SqlChat {
            pub fn into_chat(self) -> Chat {
                Chat {
                    chat_id: Uuid::parse_str(&self.chat_id).unwrap(),
                    user_id: Uuid::parse_str(&self.user_id).unwrap(),
                    creation_time: self.creation_time,
                    chat_name: self.chat_name,
                }
            }
        }
    }
}





// use cfg_if::cfg_if;
// use uuid::Uuid;
// use chrono::{DateTime, Utc};
// use crate::models::chat::chat_message_pair::ChatMessagePair;

// #[derive(Debug, Clone)]
// pub struct Chat {
//     pub chat_id: Uuid,
//     pub user_id: Uuid,
//     pub creation_time: DateTime<Utc>,
//     pub history: Vec<ChatMessagePair>,  // history is now a list of ChatMessagePair objects
//     pub chat_name: Option<String>,
// }

// cfg_if! {
//     if #[cfg(feature = "ssr")] {
//         use async_trait::async_trait;
//         use sqlx::PgPool;
//         use crate::models::chat::chat_message_pair::SqlChatMessagePair;
        
//         impl Chat {
//             pub async fn get(chat_id: Uuid, pool: &PgPool) -> Option<Self> {
//                 let sql_chat = sqlx::query_as::<_, SqlChat>("SELECT * FROM chats WHERE chat_id = ?")
//                     .bind(chat_id.to_string())
//                     .fetch(pool)
//                     .await
//                     .ok()?;
    
//                 Some(sql_chat.into_chat())
//             }

//             pub async fn get_all_chat_message_pairs(self, pool: &PgPool) -> Option<Vec<ChatMessagePair>> {
//                 let sql_chat_message_pairs = sqlx::query_as::<_, SqlChatMessagePair>("SELECT * FROM chat_message_pairs WHERE chat_id = ?")
//                     .bind(self.chat_id.to_string())
//                     .fetch_all(pool)
//                     .await
//                     .ok()?;
                
                
//                 let mut chat_message_pairs = sql_chat_message_pairs
//                                                 .into_iter()
//                                                 .map(|sql_chat_message_pair| sql_chat_message_pair.into_chat_message_pair())
//                                                 .collect::<Vec<ChatMessagePair>>();

//                 chat_message_pairs.sort_by(|a, b| a.message_time.cmp(&b.message_time));
//                 Some(chat_message_pairs)
//             }
    
//             pub fn new_chat(user_id: Uuid, chat_name: Option<String>) -> Self {
//                 Self {
//                     chat_id: Uuid::new_v4(),
//                     user_id: user_id,
//                     creation_time: Utc::now(),
//                     history: Vec::<ChatMessagePair>::new(),
//                     chat_name: chat_name,
//             }


//             }
//         }
        
    
//         #[derive(sqlx::FromRow, sqlx::Decode, Clone)]
//         pub struct SqlChat {
//             pub chat_id: String,
//             pub user_id: Option<String>,
//             pub creation_time: Option<DateTime<Utc>>,
//             pub history: Vec<ChatMessagePair>,
//             pub chat_name: Option<String>,
//         }
        
//         impl SqlChat {
//             pub fn into_chat(self) -> Chat {
//                 Chat {
//                     chat_id: Uuid::parse_str(&self.chat_id).unwrap(),
//                     user_id: match self.user_id {
//                         Some(user_id) => Some(Uuid::parse_str(&user_id).unwrap()),
//                         None => None,
//                     },
//                     creation_time: self.creation_time,
//                     history: self.history,
//                     chat_name: self.chat_name,
//                 }
//             }
//         }
    
//     }
//     }
    
