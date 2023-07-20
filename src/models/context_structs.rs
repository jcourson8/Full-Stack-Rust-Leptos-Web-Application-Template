use::leptos::*;
use crate::server_fn::authentication::Logout;
use crate::server_fn::authentication::Login;
use crate::server_fn::authentication::Signup;

#[derive(Copy, Clone)]
pub struct LogoutActionContext(pub Action<Logout, Result<(), ServerFnError>>);

#[derive(Copy, Clone)]
pub struct LoginActionContext(pub Action<Login, Result<(), ServerFnError>>);

#[derive(Copy, Clone)]
pub struct SignupActionContext(pub Action<Signup, Result<(), ServerFnError>>);