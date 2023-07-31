
use http::request;
use::leptos::*;
use leptos::ev::pagehide;
use crate::server_fn::authentication::*;
use crate::views::components::resource_protected_route::ResourceProtectedRoute;
use leptos_meta::*;
use leptos_router::*;
use crate::views::components::header::Header;
use crate::views::{signup::Signup,login::LoginView,settings::Settings, chat::Chat};
use crate::models::context_structs::{LogoutActionContext, UserResourceContext};
use crate::models::context_structs::LoginActionContext;
use crate::models::context_structs::SignupActionContext;

use crate::views ::components::logout_button::LogoutButton;
use leptos_router::A;
// use web_sys::*;
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
    // let (page_reload, set_page_reload) = create_signal(cx, true);
    
    // use web_sys::window;

    // let window = window().expect("no global `window` exists");
    // let performance = window.performance().expect("performance should be available");

    // let navigation_entries = performance.get_entries_by_type("navigation");

    // if let Some(entry) = navigation_entries.get(0) {
    //     let navigation_timing = entry.dyn_into::<web_sys::PerformanceNavigationTiming>().expect("Failed to cast to PerformanceNavigationTiming");
    // }


    // First, let's create a resource for the 'is_authenticated' function.
    let authenticated = create_resource(
        cx,
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
                // page_reload.get(),
            )
        },  // We don't need any arguments to call 'is_authenticated'.
        move |_| {
            log::info!("[route] [App - {}] [authenticated] calling is_authenticated", log_uuid);
            is_authenticated(cx)
        }
    );

    let authenticated_loading = authenticated.loading();

    // create_effect(cx, move || {
    //     authenticated.refetch();
    // });


    // Now, let's define the condition function.
    let is_logged_in = move |cx: Scope| -> bool {
        // while is_authenticated.read(cx) == None {
        //     // nop
        // }
        authenticated.refetch();
        // loading returns a signal of a bool to indicated if the resource is still loading
        // log::info!("[route] [App - {}] [is_not_logged_in] loading - start ", log_uuid);
        // while authenticated_loading() {
        //     // log::info!("{}", authenticated_loading());
        //     // nop
        // }
        log::info!("[route] [App - {}] [is_not_logged_in] loading - done ", log_uuid);
        match authenticated.read(cx) {
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

    // let is_logged_in = move |cx: Scope| -> bool {
    //     let mut result = false; // Default to false
    
    //     authenticated.with(cx, |resource| {
    //         match resource {
    //             // If the resource is still pending, return false.
    //             None => {
    //                 log::info!("[route] [App - {}] [is_logged_in] None - returning false ", log_uuid);
    //                 result = false;
    //             }
    //             // If the resource has resolved, check the Result.
    //             Some(authenticated_result) => match authenticated_result {
    //                 // If there was an error, return false.
    //                 Err(_) => {
    //                     log::info!("[route] [App - {}] [is_logged_in] Err - returning false ", log_uuid);
    //                     result = false;
    //                 }
    //                 // If the user is authenticated, return true.
    //                 Ok(authenticated) => {
    //                     log::info!("[route] [App - {}] [is_logged_in] Ok - returning {}", log_uuid, authenticated);
    //                     result = authenticated;
    //                 }
    //             },
    //         }
    //     });
    
    //     result
    // };

    let is_not_logged_in = move |cx: Scope| -> bool {
        authenticated.refetch();
        // std::thread::sleep(std::time::Duration::from_millis(1000));
        // log::info!("[route] [App - {}] [is_not_logged_in] loading - start ", log_uuid);
        // while authenticated_loading() {
        //     // log::info!("[ro/ute] [App - {}] [is_not_logged_in] loading - waiting ", log_uuid);
        //     // nop
        //     // log::info!("{}", authenticated_loading());
        // }
        // log::info!("[route] [App - {}] [is_not_logged_in] loading - done ", log_uuid);
        match authenticated.read(cx) { //.map(|resource| match resource {

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
    // let is_not_logged_in = move |cx: Scope| -> bool {
    //     let mut result = false; // Default to false
    
    //     authenticated.with(cx, |resource| {
    //         match resource {
    //             // If the resource is still pending, return false.
    //             None => {
    //                 log::info!("[route] [App - {}] [is_logged_in] None - returning false ", log_uuid);
    //                 result = true;
    //             }
    //             // If the resource has resolved, check the Result.
    //             Some(authenticated_result) => match authenticated_result {
    //                 // If there was an error, return false.
    //                 Err(_) => {
    //                     log::info!("[route] [App - {}] [is_logged_in] Err - returning false ", log_uuid);
    //                     result = true;
    //                 }
    //                 // If the user is authenticated, return true.
    //                 Ok(authenticated) => {
    //                     log::info!("[route] [App - {}] [is_logged_in] Ok - returning {}", log_uuid, authenticated);
    //                     result = !authenticated;
    //                 }
    //             },
    //         }
    //     });
    
    //     result
    // };

    // Now, let's define the condition function.
    // let already_logged_in = move |cx: Scope| -> bool {
    //     match authenticated.read(cx) {
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
    let (view, set_view) = create_signal(cx, true);

    // create_effect(cx, move |_| {
    //     view();
    //     request_animation_frame(move|| {
    //         authenticated.refetch();
    //     });
    // });

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
                    // <ProtectedRoute 
                    //     path="/" 
                    //     redirect_path="/login" 
                    //     condition=is_logged_in
                    //     view=|cx| 
                    //         view! { cx, 
                    //             <Chat /> 
                    //         }
                    // /> //Route

                    // <Route path="/" view=move |cx| {
                    //     authenticated.refetch();
                    //     view! {
                    //         cx,
                    //         { move || authenticated.read(cx).map(|resource| match resource {
                    //                 Ok(authenticated) => {
                    //                     if authenticated == true {
                    //                         view! { cx, <Chat />}
                    //                             // .into_view(cx)
                    //                     } else {
                    //                         view! { cx, <Redirect path="/login".clone()/> }
                    //                             // .into_view(cx)
                    //                     }
                    //                 }, 
                    //                 Err(_) => {
                    //                     view! { cx, <LoginView />}
                    //                             // .into_view(cx)
                    //                 }
                    //                 // }
                    //             }).unwrap_or_else(|| {
                    //                 view! { cx,  }.into_view(cx)
                    //                         // .into_view(cx)
                    //             })
                    //         }
                    //     }}
                    // />
                    <ResourceProtectedRoute 
                    
                        path="/" 
                        resource_condition=authenticated
                        redirect_path="/login" 
                        protected_view=Box::new(|cx| 
                            view! { cx, 
                                <Chat /> 
                            }
                        )
                        fallback_view=Box::new(|cx| 
                            view! { cx,  }.into_view(cx)
                        )
                        resource_err_view=Box::new(|cx| 
                            view! { cx, 
                                <LoginView /> 
                            }
                        )
                        
                    />
                    <Route path="/signup" view=move |cx| 
                        view! {
                            cx,
                            <Signup />
                        }
                    />
                    <ResourceProtectedRoute 
                    
                        path="login" 
                        resource_condition=authenticated
                        invert_resource_condition=true
                        redirect_path="/" 
                        protected_view=Box::new(|cx| 
                            view! { cx, 
                                <LoginView /> 
                            }
                        )
                        fallback_view=Box::new(|cx| 
                            view! { cx, 
                                <LoginView /> 
                            }
                        )
                        resource_err_view=Box::new(|cx| 
                            view! { cx, 
                                <LoginView /> 
                            }
                        )
                        
                    />
                    // <ProtectedRoute 
                    //     path="/login" 
                    //     redirect_path="/" 
                    //     condition=is_not_logged_in
                    //     view= move |cx| view! { cx, 
                    //         <LoginView />                            
                    //     }
                    // /> 
                    // <Route path="/login" view=move |cx| {
                    //     // set_view(!view());
                    //     authenticated.refetch();

                    //     view! {
                    //         cx,
     
                    //             {move || authenticated.read(cx).map(|resource| match resource {

                    //                     Ok(authenticated) => {
                    //                         log::warn!(" /login Ok(authenticated)");
                    //                         if authenticated == false {
                    //                             log::warn!(" /login false");
                    //                             view! { cx, <LoginView />}
                    //                                 // .into_view(cx)
                    //                         } else {
                    //                             log::warn!(" /login redirecting to /");
                    //                             view! { cx, <Redirect path="/".clone()/> }
                    //                                 // .into_view(cx)
                    //                         }
                    //                     }, 
                    //                     Err(_) => {
                    //                         log::warn!(" / Err(_)");
                    //                         view! { cx, <LoginView />}
                    //                                 // .into_view(cx)
                    //                     }
                    //                 }).unwrap_or_else(|| {
                    //                     log::warn!(" / Err(_)");
                    //                     view! { cx, <LoginView />}
                    //                             // .into_view(cx)
                    //                 })

                    //                     // )//.into_view(cx)
                    //             }
                    //             // {move || view.get()}s
                    //         // </Suspense>
                    //     }}
                    // />

                    <ProtectedRoute 
                        path="/settings" 
                        redirect_path="/login" 
                        condition=is_logged_in
                        view=|cx| view! { cx, 
                            <Settings/> 
                        }
                    /> 
                    
                </Routes>
            </main>
            </div>
        </Router>
    }
}
