// use http::request;
use::leptos::*;
// use leptos::ev::pagehide;
use crate::server_fn::authentication::*;
use leptos_meta::*;
// use leptos_router::*;
// use crate::views::components::header::Header;
// use crate::views::{signup::Signup,login::LoginView,settings::Settings, chat::Chat};
use crate::models::context_structs::{LogoutActionContext, UserResourceContext};
use crate::models::context_structs::LoginActionContext;
use crate::models::context_structs::SignupActionContext;
use crate::views::app::App;

// use crate::views ::components::logout_button::LogoutButton;
// use leptos_router::A;

#[component]
pub fn Main(cx: Scope) -> impl IntoView {
    // let app_state: RwSignal<AppState> = create_rw_signal(cx, AppState { auth: None });
    // provide_context(cx, app_state);

    // let auth = create_resource(
    //     cx,
    //     || (),
    //     move |_| async move {
    //         match fetch_token().await {
    //             Ok(token) => {
    //                 app_state.set(AppState { token: Some(token) });
    //                 Some(token)
    //             }
    //             Err(_) => None,
    //         }
    //     },
    // );
    let log_uuid = uuid::Uuid::new_v4();
    log::info!("[route] [Main - {}]", log_uuid);

    let login: Action<Login, Result<(), ServerFnError>> = create_server_action::<Login>(cx);
    let logout = create_server_action::<Logout>(cx);
    let signup = create_server_action::<Signup>(cx);
    // let authenticate = create_server_action::<IsAuthenticated>(cx);

    let user: Resource<(usize, usize, usize), Result<Option<crate::models::user::User>, ServerFnError>> = create_resource(
        cx,
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
            )
        },
        move |_| get_user(cx),
    );
    provide_meta_context(cx);
    provide_context(cx, LogoutActionContext(logout));
    provide_context(cx, LoginActionContext(login));
    provide_context(cx, SignupActionContext(signup));
    provide_context(cx, UserResourceContext(user));


    let auth: Resource<(usize, usize, usize), Result<bool, ServerFnError>> = create_resource(
        cx,
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
                // page_reload.get(),
            )
        },  // We don't need any arguments to call 'is_authenticated'.
        move |_| is_authenticated(cx),
    );


    view! { cx,
      <Suspense fallback=move || {
          view! { cx, <p>"Loading..."</p> }
      }>{move || { auth.read(cx).map(|auth| match auth {
        Ok(auth) => {
            view! { cx, <App is_authenticated=auth user=user /> }
        },
        Err(_) => {
            view! { cx, <p>"Error"</p> }.into_view(cx)
        }
        // view! { cx, <App is_authenticated=auth user=user /> }) }}</Suspense>
    }) }}</Suspense>
    }
}
