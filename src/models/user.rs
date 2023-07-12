use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub permissions: HashSet<String>,
    pub is_guest: bool,
}


impl Default for User {
    fn default() -> Self {
        let permissions = HashSet::new();

        Self {
            id: Uuid::new_v4(), 
            username: "Guest".into(),
            password: "".into(),
            permissions,
            is_guest: true,
        }
    }

}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use async_trait::async_trait;
    use sqlx::SqlitePool;
    use axum_session_auth::{SessionSqlitePool, Authentication, HasPermission};
    pub type AuthSession = axum_session_auth::AuthSession<User, Uuid, SessionSqlitePool, SqlitePool>;
    
    impl User {
        pub async fn get(id: Uuid, pool: &SqlitePool) -> Option<Self> {
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = ?")
                .bind(id.to_string())
                .fetch_one(pool)
                .await
                .ok()?;

            //lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
            let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
                "SELECT token FROM user_permissions WHERE user_id = ?;",
            )
            .bind(id.to_string())
            .fetch_all(pool)
            .await
            .ok()?;

            Some(sqluser.into_user(Some(sql_user_perms)))
        }

        pub async fn get_from_username(name: String, pool: &SqlitePool) -> Option<Self> {
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE username = ?")
                .bind(name)
                .fetch_one(pool)
                .await
                .ok()?;
        
            //lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
            let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
                "SELECT token FROM user_permissions WHERE user_id = ?;",
            )
            // .bind(sqluser.id) // If sqluser.id is a String
            .bind(sqluser.id.to_string()) // If sqluser.id is a Uuid
            .fetch_all(pool)
            .await
            .ok()?;
        
            Some(sqluser.into_user(Some(sql_user_perms)))
        }

        pub fn new_guest(id: Uuid) -> Self {
            let permissions = HashSet::new();
    
            Self {
                id,
                username: "Guest".into(),
                password: "".into(),
                permissions,
                is_guest: true,
            }
        }
    }
    
    #[async_trait]
    impl Authentication<User, Uuid, SqlitePool> for User {
        async fn load_user(userid: Uuid, pool: Option<&SqlitePool>) -> Result<User, anyhow::Error> {
            let pool = pool.unwrap();

            User::get(userid, pool)
                .await
                .ok_or_else(|| anyhow::anyhow!("Cannot get user"))
        }

        fn is_authenticated(&self) -> bool {
            true
        }

        fn is_active(&self) -> bool {
            true
        }

        fn is_anonymous(&self) -> bool {
            false
        }
    }

    #[async_trait]
    impl HasPermission<SqlitePool> for User {
        async fn has(&self, perm: &str, _pool: &Option<&SqlitePool>) -> bool {
            self.permissions.contains(perm)
        }
    }


    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlPermissionTokens {
        pub token: String,
    }

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlUser {
        pub id: String,
        pub username: String,
        pub password: String,
    }
    
    impl SqlUser {
        pub fn into_user(self, sql_user_perms: Option<Vec<SqlPermissionTokens>>) -> User {
            User {
                id: Uuid::parse_str(&self.id).expect("Failed to parse UUID"),
                username: self.username,
                password: self.password,
                permissions: if let Some(user_perms) = sql_user_perms {
                    user_perms
                        .into_iter()
                        .map(|x| x.token)
                        .collect::<HashSet<String>>()
                } else {
                    HashSet::<String>::new()
                },
                is_guest: false,
            }
        }
    }

}
}