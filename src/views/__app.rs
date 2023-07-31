
use http::request;
use::leptos::*;
use leptos::ev::pagehide;
use crate::server_fn::authentication::*;
use leptos_meta::*;
use leptos_router::*;
use crate::views::components::header::Header;
use crate::views::{signup::Signup,login::LoginView,settings::Settings, chat::Chat};
use crate::models::context_structs::{LogoutActionContext, UserResourceContext};
use crate::models::context_structs::LoginActionContext;
use crate::models::context_structs::SignupActionContext;

use crate::views ::components::logout_button::LogoutButton;
use leptos_router::A;
use web_sys::*;
// use crate::models::user::User;



#[component]
pub fn App(
    cx: Scope
) -> impl IntoView {
    let log_uuid = uuid::Uuid::new_v4();
    log::info!("[route] [App - {}]", log_uuid);

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

    let (currentUser, setCurrentUser) = create_signal(cx, None as Option<crate::models::user::User>);

    // First, let's create a resource for the 'is_authenticated' function.
    let is_authenticated = create_resource(
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




    // Now, let's define the condition function.
    let is_logged_in = move |cx: Scope| -> bool {
        // while is_authenticated.read(cx) == None {
        //     // nop
        // }

        match is_authenticated.read(cx) {
            // If the resource is still pending, return false.
            None => {
                log::info!("[route] [App - {}] [is_logged_in] None - returning false ", log_uuid);
                false
            }
            // If the resource has resolved, check the Result.
            Some(result) => match result {
                // If there was an error, return false.
                Err(_) => {
                    log::info!("[route] [App - {}] [is_logged_in] Err - returning false ", log_uuid);
                    false
                }
                // If the user is authenticated, return true.
                Ok(authenticated) => {
                    log::info!("[route] [App - {}] [is_logged_in] Ok - returning {}", log_uuid, authenticated);
                    authenticated
                }
            },
        }
    };

    let is_not_logged_in = move |cx: Scope| -> bool {
        // std::thread::sleep(std::time::Duration::from_millis(1000));

        match is_authenticated.read(cx) {

            // If the resource is still pending, return false.
            None => {
                log::info!("[route] [App - {}] [is_not_logged_in] None - returning true ", log_uuid);
                true
            }
            // If the resource has resolved, check the Result.
            Some(result) => match result {
                // If there was an error, return false.
                Err(_) => {
                    log::info!("[route] [App - {}] [is_not_logged_in] Err - returning true ", log_uuid);
                    true
                }
                // If the user is authenticated, return true.
                Ok(authenticated) => {
                    log::info!("[route] [App - {}] [is_not_logged_in] Ok - returning {}", log_uuid, !authenticated);
                    !authenticated
                }
            },
        }
    };


    view! {
        cx,
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/session_auth_axum.css"/>
        <Router>
            <div class="flex flex-col h-screen overflow-hidden">
            // <Header />
            <header class="bg-gray-900 p-4 text-white shadow-md w-full z-50"> //fixed top-0fixed top-0
                <A href="/" class="text-2xl font-bold"><h1>"Home"</h1></A>
                <Transition
                    fallback=move || view! {cx, <span class="text-gray-300">"Loading..."</span>}
                >
                {move || {
                    user.read(cx).map(|user| match user {
                        Err(e) => view! {cx,
                            <a href="/signup" class="text-white">"Signup"</a>
                            ", "
                            <a href="/login" class="text-white">"Login"</a>
                            ", "
                            <span class="text-red-300">{format!("Login error: {}", e)}</span>
                        }.into_view(cx),
                        Ok(None) => view! {cx,
                            <a href="/signup" class="text-white">"Signup"</a>", "
                            <a href="/login" class="text-wihte">"Login"</a>", "
                            <span class="text-green-500">"Logged out."</span>
                        }.into_view(cx),
                        Ok(Some(user)) => match user.is_guest {
                            true => view! {cx,
                                <a href="/signup" class="text-white">"Signup"</a>", "
                                <a href="/login" class="text-white">"Login"</a>", "
                                <span class="text-yellow-300">"Guest user."</span>
                            }.into_view(cx),
                            false => view! {cx,
                                <a href="settings" class="text-white">"Settings"</a>", "
                                // <A href="/logout" class="text-white">"Logout"</A>", " // New Logout link here
                                <span class="text-white-500">{format!("Logged in as: {} ({})", user.username, user.id.to_string())}</span>
                                <LogoutButton style="text-white"/>
                            }.into_view(cx),
                        }
                    })
                }}
                </Transition>
            </header>
            <hr/>
            <main class="flex-1 overflow-y-scroll py-2 mx-2">
                
                <Routes>
                
                </Routes>
            </main>
            </div>
        </Router>
    }
}






#[component]
fn App(cx: Scope, token: Option<Token>) -> impl IntoView {
    let (is_routing, set_is_routing) = create_signal(cx, false);

    view! { cx,
      <RoutingProgress
        is_routing
        max_time=std::time::Duration::from_millis(250)
        class="RoutingProgress"
      />
      <Router set_is_routing>
        <Routes>
          <ProtectedRoute
            path=Routes::Login::path()
            redirect_path=Routes::Authentication::Dash.path()
            condition=move |_| token.is_none()
            view=Login
          />
          <ProtectedRoute
            path="/*"
            redirect_path=Routes::Login::path()
            condition=move |_| token.is_some()
            view=Application
          >
            <Route path="" view=|cx| view! { cx, <Redirect path=Routes::Authentication::Dash.path()/> }/>
            <Route path=Routes::Authentication::App.path() view=SubRoute>
              <Route path="" view=|cx| view! { cx, <Redirect path=Routes::Authentication::Dash.path()/> }/>
            </Route>
          </ProtectedRoute>
          <Route path="/*any" view=NotFound/>
        </Routes>
      </Router>
    }
}

#[component]
fn Main(cx: Scope) -> impl IntoView {
    let app_state: RwSignal<AppState> = create_rw_signal(cx, AppState { auth: None });
    provide_context(cx, app_state);

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
    let auth = create_resource(
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
          view! { cx, <Loading/> }
      }>{move || { token.read(cx).map(|tok| view! { cx, <App token=tok/> }) }}</Suspense>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <Main/> })
}