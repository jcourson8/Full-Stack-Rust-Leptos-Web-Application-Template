
use::leptos::*;
use crate::server_fn::authentication::*;
use leptos_meta::*;
use leptos_router::*;
use crate::views::components::header::Header;
use crate::views::{signup::Signup,login::LoginView,settings::Settings, chat::Chat};
use crate::models::context_structs::LogoutActionContext;
use crate::models::context_structs::LoginActionContext;
use crate::models::context_structs::SignupActionContext;

use crate::views ::components::logout_button::LogoutButton;
use leptos_router::A;
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

    let user = create_resource(
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




    // First, let's create a resource for the 'is_authenticated' function.
    let is_authenticated = create_resource(
        cx,
        || (),  // We don't need any arguments to call 'is_authenticated'.
        move |_| is_authenticated(cx),
    );

    // Now, let's define the condition function.
    let is_logged_in = move |cx: Scope| -> bool {
        match is_authenticated.read(cx) {
            // If the resource is still pending, return false.
            None => false,
            // If the resource has resolved, check the Result.
            Some(result) => match result {
                // If there was an error, return false.
                Err(_) => false,
                // If the user is authenticated, return true.
                Ok(authenticated) => authenticated,
            },
        }
    };

    let is_not_logged_in = move |cx: Scope| -> bool {
        match is_authenticated.read(cx) {
            // If the resource is still pending, return false.
            None => true,
            // If the resource has resolved, check the Result.
            Some(result) => match result {
                // If there was an error, return false.
                Err(_) => true,
                // If the user is authenticated, return true.
                Ok(authenticated) => !authenticated,
            },
        }
    };

    // Now, let's define the condition function.
    // let already_logged_in = move |cx: Scope| -> bool {
    //     match is_authenticated.read(cx) {
    //         // If the resource is still pending, return false.
    //         None => false,
    //         // If the resource has resolved, check the Result.
    //         Some(result) => match result {
    //             // If there was an error, return false.
    //             Err(_) => false,
    //             // If the user is authenticated, return true.
    //             Ok(authenticated) => authenticated,
    //         },
    //     }
    // };


    view! {
        cx,
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/session_auth_axum.css"/>
        <Router>
            <div class="flex flex-col h-screen overflow-hidden">
            <Header user/>
            <hr/>
            <main class="flex-1 overflow-y-scroll py-2 mx-2">
                
                <Routes>
                    <ProtectedRoute 
                        path="" 
                        redirect_path="login" 
                        condition=is_logged_in
                        view=|cx| 
                            view! { cx, 
                                <Chat /> 
                            }
                    /> //Route
                    <Route path="signup" view=move |cx| 
                        view! {
                            cx,
                            <Signup />
                        }
                    />
                    // <Route path="logout" view=move |cx| view! {
                    //     cx,
                    //     <Login action=login />
                    // }/>
                    <ProtectedRoute 
                        path="login" 
                        redirect_path="/" 
                        condition=is_not_logged_in
                        view= move |cx| view! { cx, 
                            <LoginView />                            
                        }
                    /> 
                    <ProtectedRoute 
                        path="settings" 
                        redirect_path="login" 
                        condition=is_logged_in
                        view=|cx| view! { cx, 
                            <Settings/> 
                        }
                    /> 
                    // <Route path="/*any" view=move |cx| view! {
                    //     cx,
                    //     <Login action=login />
                    // }/>
                    
                </Routes>
            </main>
            </div>
        </Router>
    }
}
