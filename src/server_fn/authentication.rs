use cfg_if::cfg_if;
use leptos::*;
use uuid::Uuid;
use crate::models::user::*;


cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::SqlitePool;
        use crate::server_fn::{pool, auth};
        use axum_session_auth::SessionSqlitePool;
        use bcrypt::{hash, verify, DEFAULT_COST};
        pub type AuthSession = axum_session_auth::AuthSession<User, Uuid, SessionSqlitePool, SqlitePool>;
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
    let auth = auth(cx)?;

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
