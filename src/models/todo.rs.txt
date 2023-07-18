use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::user::User;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub user: Option<User>,
    pub title: String,
    pub created_at: String,
    pub completed: bool,
    pub is_guest: bool,
}

cfg_if! {
if #[cfg(feature = "ssr")] {

    use sqlx::PgPool;

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlTodo {
        id: String,
        user_id: String,
        title: String,
        created_at: String,
        completed: bool,
        is_guest:bool,
    }

    impl SqlTodo {
        pub async fn into_todo(self, pool: &PgPool) -> Todo {
            Todo {
                // id: self.id,
                // user: User::get(self.user_id, pool).await,
                id: Uuid::parse_str(&self.id).expect("Failed to parse UUID"),
                user: User::get(Uuid::parse_str(&self.user_id).expect("Failed to parse UUID"), pool).await,
                title: self.title,
                created_at: self.created_at,
                completed: self.completed,
                is_guest: self.is_guest,

            }
        }
    }
}
}