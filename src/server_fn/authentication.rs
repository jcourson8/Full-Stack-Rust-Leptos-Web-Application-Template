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
    let auth = auth(cx)?;
    Ok(auth.is_authenticated())
}

#[server(GetUser, "/api")]
pub async fn get_user(cx: Scope) -> Result<Option<User>, ServerFnError> {
    let log_uuid = Uuid::new_v4();
    log::info!("[GetUser - {}]", log_uuid.clone());

    let auth = auth(cx)?;

    let current_user = match auth.current_user {
        Some(user) => {
            log::info!("[GetUser - {}] there was Some(user)", log_uuid.clone());
            Some(user)
        },
        None => {
            // let id = Uuid::new_v4();
            // log::info!("[GetUser - {}] Creating guest user with UUID: {}", log_uuid.clone(), id.to_string());

            // // so that a guest can be more persistant and if they navigate to signup we can associate their data
            // auth.login_user(id);
            // auth.remember_user(true);

        
            // Some(User::new_guest(id))
            None

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
    let log_uuid = Uuid::new_v4();
    log::info!("[Login - {}] username: {}, password: {}, remember: {:?}", log_uuid.clone(), username.clone(), password.clone(), remember.clone());
    log::info!("[Login - {}] Getting pool from context...", log_uuid.clone());
    let pool = pool(cx)?;
    log::info!("[Login - {}] Got pool getting auth from context...", log_uuid.clone());
    let auth = auth(cx)?;
    log::info!("[Login - {}] Got auth from context...", log_uuid.clone());

    let user: User = User::get_from_username(username.clone(), &pool)
        .await
        .map_err(|e| e)?
        .ok_or_else(|| {
            log::warn!("[Login - {}] User \"{}\" returned None from database.", log_uuid.clone(), username);
            // message that doesnt leak database info
            ServerFnError::ServerError("Username or password is incorrect.".to_string())
        })?;

    match verify(password, &user.password)? {
        true => {
            log::error!("[Login - {}] auth.login_user: {}", log_uuid.clone(), user.id.to_string());
            auth.login_user(user.id);
            // auth.remember_user(remember.is_some());
            log::error!("[Login - {}] redirect to /", log_uuid);
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
    let log_uuid = Uuid::new_v4();
    log::info!("[TransferDataToUser - {}] guest_id: {}", log_uuid.clone(), guest_id.clone().to_string());
    
    let pool = pool(cx)?;

    let current_user = get_user(cx)
                        .await
                        .map_err(|e| {
                            log::warn!("[TransferDataToUser - {}] Error getting current user from context: {}", log_uuid.clone(), e);
                            ServerFnError::ServerError(format!("TODO BETTER ERROR"))
                        })?
                        .ok_or_else(|| {
                            log::warn!("[TransferDataToUser - {}] User in context was None:", log_uuid.clone());
                            ServerFnError::ServerError(format!("TODO BETTER ERROR"))
                        })?;
                        
    let current_user = sqlx::query("UPDATE todos SET user_id = $1, is_guest = false WHERE user_id = $2 AND is_guest = true")
                .bind(current_user.id)
                .bind(guest_id)
                .execute(&pool)
                .await
                .map_err(|e| {
                    log::warn!("[TransferDataToUser - {}] Error transfering data from guest user to new user \"{}\", error: {}",  log_uuid.clone(), current_user.id.to_string(),e);
                    ServerFnError::ServerError(format!("TODO BETTER ERROR"))
                })?;


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
    let log_uuid = Uuid::new_v4();
    log::info!("[Signup - {}] username: {}, password: {}, password_confirmation: {}, remember: {:?}", log_uuid.clone(), username.clone(), password.clone(), password_confirmation.clone(), remember.clone()); 
    
    let pool = pool(cx)?;

    let auth = auth(cx)?;

    // let guest_user = get_user(cx).await?;


    if password != password_confirmation {
        ServerFnError::ServerError("Passwords did not match.".into());
    }

    let password_hashed = hash(password, DEFAULT_COST)
                            .map_err(|e| {
                                log::warn!("[Signup - {}] Error hashing password for username: {}, error: {}", log_uuid.clone(), username, e);
                                ServerFnError::ServerError(format!("Error hashing password for username: {}, error: {}", username, e))
                            })?;

    sqlx::query("INSERT INTO users (username, password, id) VALUES ($1,$2,$3)")
        .bind(username.clone())
        .bind(password_hashed)
        .bind(Uuid::new_v4())
        .execute(&pool)
        .await
        .map_err(|e| {
            log::warn!("[Signup - {}] Error inserting new user into database: {}, error: {}", log_uuid.clone(), username, e);
            ServerFnError::ServerError(format!("Error inserting new user into database: {}", e))
        })?;

    let user = User::get_from_username(username.clone(), &pool)
                .await
                .map_err(|e| {
                    ServerFnError::ServerError(
                        format!("Signup failed: {}", e.to_string()),
                    )
                })?
                .ok_or_else(|| {
                    log::warn!("[Signup - {}] User \"{}\" failed for into_user.", log_uuid.clone(), username);
                    ServerFnError::ServerError(
                        format!("Signup failed"),
                    )
                })?;

    
    log::error!("[Signup - {}] auth.login_user: {}", log_uuid.clone(), user.id.to_string());
    auth.login_user(user.id);
    // auth.remember_user(remember.is_some());
    // if successful and Some(guest_id) then try to tranfser data over to new user
    // if let Some(guest) = guest_user.as_ref() {
    //     transfer_data_to_user(cx, guest.id).await?            
    // }

    leptos_axum::redirect(cx, "/");
    log::info!("[Signup - {}] Signup successful for username: {}", log_uuid.clone(), username);
    Ok(())
}

#[server(Logout, "/api")]
pub async fn logout(cx: Scope) -> Result<(), ServerFnError> {
    let log_uuid = Uuid::new_v4();
    log::info!("[Logout - {}]", log_uuid);
    let auth = auth(cx)?;

    auth.remember_user(false);
    auth.logout_user();
    
    log::error!("[Logout - {}] redirect to login", log_uuid);
    leptos_axum::redirect(cx, "/login");

    Ok(())
}
