use::leptos::*;
use crate::server_fn::authentication::Logout;

#[derive(Copy, Clone)]
pub struct LogoutActionContext(pub Action<Logout, Result<(), ServerFnError>>);