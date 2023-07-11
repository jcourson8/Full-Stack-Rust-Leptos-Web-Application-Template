use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use sqlx::SqlitePool;
    use axum_session_auth::{SessionSqlitePool, Authentication, HasPermission};
    use bcrypt::{hash, verify, DEFAULT_COST};
    use crate::todo::{pool, auth};
    pub type AuthSession = axum_session_auth::AuthSession<User, Uuid, SessionSqlitePool, SqlitePool>;
}}

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
        // use uuid::Uuid;
    
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
    


    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlPermissionTokens {
        pub token: String,
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


#[server(Foo, "/api")]
pub async fn foo() -> Result<String, ServerFnError> {
    Ok(String::from("Bar!"))
}

#[server(GetUser, "/api")]
pub async fn get_user(cx: Scope) -> Result<Option<User>, ServerFnError> {
    let auth = auth(cx)?;

    let current_user = match auth.current_user {
        Some(user) => Some(user),
        None => {
            let id = auth.session.get("user_auth_session_id").unwrap_or(Uuid::new_v4());
            println!{"GETUSER:{}",id}

            // so that a guest can be more persistant and if they navigate to signup we can associate their data
            auth.login_user(id);
            auth.remember_user(true);

        
            Some(User::new_guest(id))

        },
    };

    Ok(current_user)
    
}
// #[server(GetUser, "/api")]
// pub async fn get_user(cx: Scope) -> Result<Option<User>, ServerFnError> {
//     let auth = auth(cx)?;
    
//     let user = match auth.current_user {
//         Some(user) => user.id,
//         None => Uuid::new(),
//     };

    
// }

#[server(Login, "/api")]
pub async fn login(
    cx: Scope,
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;
    let auth = auth(cx)?;

    let user: User = User::get_from_username(username, &pool)
        .await
        .ok_or_else(|| {
            ServerFnError::ServerError("User does not exist.".into())
        })?;

    match verify(password, &user.password)? {
        true => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());
            leptos_axum::redirect(cx, "/");
            Ok(())
        }
        false => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}

#[server(TransferDataToUser, "/api")]
async fn transfer_data_to_user(
    cx: Scope,
    guest_id: Uuid,
) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;
    // let auth = auth(cx)?;
    let current_user = get_user(cx).await?;
    // println!{"{} ----- {}", guest_id.to_string(), current_user.id.to_string() };

    match current_user {
        Some(current_user) => {
            sqlx::query("UPDATE todos SET user_id = ?, is_guest = 0 WHERE user_id = ? AND is_guest = 1")
                .bind(current_user.id.to_string())
                .bind(guest_id.to_string())
                .execute(&pool)
                .await?
        },
        None => {
            return Err(
                ServerFnError::ServerError(
                    "Data Transfer Failed: There was no current user (this should not be possible)".into()
                ))
        },
    };

    Ok(())

}

#[server(Signup, "/api")]
pub async fn signup(
    cx: Scope,
    username: String,
    password: String,
    password_confirmation: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;
    let auth = auth(cx)?;
    let guest_user = get_user(cx).await?;


    if password != password_confirmation {
        return Err(ServerFnError::ServerError(
            "Passwords did not match.".into(),
        ));
    }

    let password_hashed = hash(password, DEFAULT_COST).unwrap();

    sqlx::query("INSERT INTO users (username, password, id) VALUES (?,?,?)")
        .bind(username.clone())
        .bind(password_hashed)
        .bind(Uuid::new_v4().to_string())
        .execute(&pool)
        .await?;

    let user =
        User::get_from_username(username, &pool)
            .await
            .ok_or_else(|| {
                ServerFnError::ServerError(
                    "Signup failed: User does not exist.".to_string(),
                )
            })?;

    
    auth.login_user(user.id);
    auth.remember_user(remember.is_some());
    // if successful and Some(guest_id) then try to tranfser data over to new user
    if let Some(guest) = guest_user.as_ref() {
        transfer_data_to_user(cx, guest.id).await?;
    }
        // .ok_or_else(|| {
        //     ServerFnError::ServerError(
        //         "Signup failed: User does not exist.".into(),
        //     )
        // })?;


    leptos_axum::redirect(cx, "/");

    Ok(())
}

#[server(Logout, "/api")]
pub async fn logout(cx: Scope) -> Result<(), ServerFnError> {
    let auth = auth(cx)?;

    auth.logout_user();
    leptos_axum::redirect(cx, "/");

    Ok(())
}
