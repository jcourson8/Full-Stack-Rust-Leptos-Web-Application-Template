use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;
// use crate::models::errors::GeneralError;


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
    use sqlx::PgPool;
    use axum_session_auth::{SessionPgPool, Authentication, HasPermission};
    pub type AuthSession = axum_session_auth::AuthSession<User, Uuid, SessionPgPool, PgPool>;
    use crate::models::errors::PoolError;
    use leptos::ServerFnError;
    
    impl User {
        pub async fn get(id: Uuid, pool: &PgPool) -> Result<Option<Self>, ServerFnError> {
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = $1")
                .bind(id)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    log::warn!("Error getting user with id \"{}\" from database: {}", id, e);
                    // Server func error should be more generic and not leak database errors
                    ServerFnError::ServerError(format!("Error getting user with id \"{}\" from database.", id))
                })?;

            //lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
            let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
                "SELECT token FROM user_permissions WHERE user_id = $1;",
            )
            .bind(id)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                log::warn!("Error getting user permissions for user with id \"{}\" from database: {}", id, e);
                ServerFnError::ServerError(format!("Error getting user permissions for user with id \"{}\" from database", id))
            })?;

            Ok(Some(sqluser.into_user(Some(sql_user_perms))))
        }

        pub async fn get_from_username(name: String, pool: &PgPool) -> Result<Option<Self>, ServerFnError>{
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE username = $1")
                .bind(name.clone())
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    log::warn!("Error getting user with username \"{}\" from database, error: {}", name.clone(), e);
                    ServerFnError::ServerError(format!("Error getting user with username \"{}\" from database", name))
                })?;
        
            //lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
            let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
                "SELECT token FROM user_permissions WHERE user_id = $1;",
            )
            // .bind(sqluser.id) // If sqluser.id is a String
            .bind(sqluser.id.clone()) // If sqluser.id is a Uuid
            .fetch_all(pool)
            .await
            .map_err(|e| {
                log::warn!("Error getting user permissions for user with username \"{}\" from database: {}", name.clone(), e);
                ServerFnError::ServerError(format!("Error getting user permissions for user with username \"{}\" from database", name.clone()))
            })?;
        
            Ok(Some(sqluser.into_user(Some(sql_user_perms))))
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
    impl Authentication<User, Uuid, PgPool> for User {
        async fn load_user(userid: Uuid, pool: Option<&PgPool>) -> Result<User, anyhow::Error> {
            let log_uuid = Uuid::new_v4();
            log::info!("[load_user - {}] Loading user with id: {}", log_uuid.clone(), userid);
            let pool = pool.ok_or(PoolError::ConnectionError)?;
            
            let user = User::get(userid, pool)
                .await
                .map_err(|e| {
                    log::info!("[load_user - {}] User {} not found.", log_uuid.clone(), userid);
                    anyhow::anyhow!("{:?}",e.to_string()) // TODO fix
                })?
                .ok_or_else(|| {
                    log::warn!("User was None");
                    anyhow::anyhow!("User was None")
                })?;

            Ok(user)
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
    impl HasPermission<PgPool> for User {
        async fn has(&self, perm: &str, _pool: &Option<&PgPool>) -> bool {
            self.permissions.contains(perm)
        }
    }


    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlPermissionTokens {
        pub token: String,
    }

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlUser {
        pub id: Uuid,
        pub username: String,
        pub password: String,
    }
    
    impl SqlUser {
        pub fn into_user(self, sql_user_perms: Option<Vec<SqlPermissionTokens>>) -> User {
            User {
                id: self.id,
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
