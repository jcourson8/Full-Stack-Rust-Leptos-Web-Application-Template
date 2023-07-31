
pub mod authentication;
// pub mod todo;


use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub use self::authentication::AuthSession;
        use sqlx::PgPool;
        use leptos::Scope;
        use leptos::ServerFnError;
        use leptos::use_context;

        pub fn pool(cx: Scope) -> Result<PgPool, ServerFnError> {
            let log_uuid = uuid::Uuid::new_v4();
            log::info!("[Pool - {}]", log_uuid.clone());
        use_context::<PgPool>(cx)
                .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
        }

        pub fn auth(cx: Scope) -> Result<AuthSession, ServerFnError> {
            let log_uuid = uuid::Uuid::new_v4();
            log::info!("[Auth - {}]", log_uuid.clone());
            use_context::<AuthSession>(cx)
                .ok_or_else(|| ServerFnError::ServerError("Auth session missing.".into()))

        // pub fn log_and_create_error<E>(function_name: &str, err: E) -> ServerFnError
        // where
        //     E: std::fmt::Display,
        // {
        //     log::warn!("Error getting auth from function {}: {}", function_name, err);
        //     ServerFnError::ServerError(format!("TODO BETTER ERROR"))
        // }
    }
    }
}