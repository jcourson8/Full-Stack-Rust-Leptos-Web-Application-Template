use cfg_if::cfg_if;
use leptos::*;
use uuid::Uuid;
use crate::models::user::*;


cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::PgPool;
        use crate::server_fn::{pool, auth};
        use axum_session_auth::SessionPgPool;
        use bcrypt::{hash, verify, DEFAULT_COST};
        pub type AuthSession = axum_session_auth::AuthSession<User, Uuid, SessionPgPool, PgPool>;
        use crate::models::errors::PoolError;
}}


#[server(IsAuthenticated, "/api")]
pub async fn is_authenticated(
    cx: Scope,
) -> Result<bool, ServerFnError> {
    let auth = auth(cx).map_err(|e| {
        log::warn!("Error getting auth from context: {}", e);
        // Server func error should be more generic and not leak database errors
        ServerFnError::ServerError(format!("TODO BETTER ERROR"))
    })?;
    Ok(auth.is_authenticated())
}

#[server(GetUser, "/api")]
pub async fn get_user(cx: Scope) -> Result<Option<User>, ServerFnError> {
    log::info!("-- GetUser Server Fn --");
    let auth = auth(cx).map_err(|e| {
        log::warn!("Error getting auth from context: {}", e);
        // Server func error should be more generic and not leak database errors
        ServerFnError::ServerError(format!("TODO BETTER ERROR"))
    })?;

    let current_user = match auth.current_user {
        Some(user) => Some(user),
        None => {
            let id = Uuid::new_v4();
            log::info!("Creating guest user with UUID: {}", id.to_string());

            // so that a guest can be more persistant and if they navigate to signup we can associate their data
            auth.login_user(id);
            auth.remember_user(true);

        
            Some(User::new_guest(id))

        },
    };

    Ok(current_user)
    
}

#[server(Login, "/api")]
pub async fn login(
    cx: Scope,
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    log::info!("-- Login Server Fn --");
    let pool = pool(cx).map_err(|e| {
        log::warn!("Error getting pool from context: {}", e);
        // Server func error should be more generic and not leak database errors
        ServerFnError::ServerError(format!("TODO BETTER ERROR"))
    })?;
    let auth = auth(cx).map_err(|e| {
        log::warn!("Error getting auth from context: {}", e);
        // Server func error should be more generic and not leak database errors
        ServerFnError::ServerError(format!("TODO BETTER ERROR"))
    })?;

    let user: User = User::get_from_username(username.clone(), &pool)
        .await
        .map_err(|e| e)?
        .ok_or_else(|| {
            log::warn!("User \"{}\" returned None from database.", username);
            // message that doesnt leak database info
            ServerFnError::ServerError("Username or password is incorrect.".to_string())
        })?;

    match verify(password, &user.password)? {
        true => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());
            leptos_axum::redirect(cx, "/");
            Ok(())
        }
        false => Err(ServerFnError::ServerError(
            "Username or password is incorrect.".to_string(),
        )),
    }
}

#[server(TransferDataToUser, "/api")]
async fn transfer_data_to_user(
    cx: Scope,
    guest_id: Uuid,
) -> Result<(), ServerFnError> {
    log::info!("-- TransferDataToUser Server Fn --");
    let pool = pool(cx).map_err(|e| {
        log::warn!("Error getting pool from context: {}", e);
        // Server func error should be more generic and not leak database errors
        ServerFnError::ServerError(format!("TODO BETTER ERROR"))
    })?;
    // let auth = auth(cx)?;
    let current_user = get_user(cx)
                        .await
                        .map_err(|e| {
                            log::warn!("Error getting current user from context: {}", e);
                            ServerFnError::ServerError(format!("TODO BETTER ERROR"))
                        })?
                        .ok_or_else(|| {
                            log::warn!("User in context was None:");
                            ServerFnError::ServerError(format!("TODO BETTER ERROR"))
                        })?;
                        
    // println!{"{} ----- {}", guest_id.to_string(), current_user.id.to_string() };

    let current_user = sqlx::query("UPDATE todos SET user_id = $1, is_guest = false WHERE user_id = $2 AND is_guest = true")
                .bind(current_user.id)
                .bind(guest_id)
                .execute(&pool)
                .await
                .map_err(|e| {
                    log::warn!("Error transfering data from guest user to new user \"{}\", error: {}", current_user.id.to_string(),e);
                    ServerFnError::ServerError(format!("TODO BETTER ERROR"))
                })?;
                // .ok_or_else(|| {
                //     log::warn!("Data Transfer Failed: There was no current user (this should not be possible)");
                //     ServerFnError::ServerError("TODO BETTER ERROR".to_string())
                // })?;


    // match current_user {
    //     Some(current_user) => {
    //         sqlx::query("UPDATE todos SET user_id = $1, is_guest = false WHERE user_id = $2 AND is_guest = true")
    //             .bind(current_user.id.to_string())
    //             .bind(guest_id.to_string())
    //             .execute(&pool)
    //             .await
    //             .map_err(|e| {
    //                 log::warn!("Error transfering data from guest user to new user \"{}\", error: {}", current_user.id.to_string()),e);
    //                 // Server func error should be more generic and not leak database errors
    //                 ServerFnError::ServerError(format!("TODO BETTER ERROR"))
    //             })?;
    //     },
    //     None => {
    //         log::warn!("Data Transfer Failed: There was no current user (this should not be possible)");
    //         ServerFnError::ServerError("TODO BETTER ERROR".to_string())
    //     },
    // };

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
    log::info!("-- Signup Server Fn --");
    let pool = pool(cx).map_err(|e| {
        log::warn!("Error getting pool from context: {}", e);
        ServerFnError::ServerError(format!("TODO BETTER ERROR"))
    })?;
    let auth = auth(cx).map_err(|e| {
        log::warn!("Error getting auth from context: {}", e);
        // Server func error should be more generic and not leak database errors
        ServerFnError::ServerError(format!("TODO BETTER ERROR"))
    })?;
    let guest_user = get_user(cx).await?;


    if password != password_confirmation {
        ServerFnError::ServerError("Passwords did not match.".into());
    }

    let password_hashed = hash(password, DEFAULT_COST)
            .map_err(|e| {
                log::warn!("Error hashing password for username: {}, error: {}", username, e);
                 ServerFnError::ServerError(format!("Error hashing password for username: {}, error: {}", username, e))
                })?;

    sqlx::query("INSERT INTO users (username, password, id) VALUES ($1,$2,$3)")
        .bind(username.clone())
        .bind(password_hashed)
        .bind(Uuid::new_v4())
        .execute(&pool)
        .await
        .map_err(|e| {
            log::warn!("Error inserting new user into database: {}, error: {}", username, e);
            ServerFnError::ServerError(format!("Error inserting new user into database: {}", e))
        })?;

    let user =
        User::get_from_username(username.clone(), &pool)
            .await
            .map_err(|e| {
                log::warn!("Error fetching new user \"{}\" from database, error:{ }", username, e);
                ServerFnError::ServerError(
                    "Signup failed: User was not created.".to_string(),
                )
            })
            .map_err(|e| e)?
            .ok_or_else(|| {
                log::warn!("User \"{}\" failed for into_user.", username);
                ServerFnError::ServerError(
                    "Signup failed: User was not created.".to_string(),
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
    log::info!("Signup successful for username: {}", username);
    Ok(())
}

#[server(Logout, "/api")]
pub async fn logout(cx: Scope) -> Result<(), ServerFnError> {
    log::info!("-- Logout Server Fn --");
    let auth = auth(cx).map_err(|e| {
        log::warn!("Error getting auth from context: {}", e);
        // Server func error should be more generic and not leak database errors
        ServerFnError::ServerError(format!("TODO BETTER ERROR"))
    })?;

    auth.logout_user();
    leptos_axum::redirect(cx, "/");

    Ok(())
}
