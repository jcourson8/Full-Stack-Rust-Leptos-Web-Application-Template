use leptos::*;
use crate::models::user::User;
use crate::views ::components::logout_button::LogoutButton;
use leptos_router::A;

#[component]
pub fn Header(
    cx: Scope, 
    user: leptos::Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>> , 
) -> impl IntoView {

    view! { cx, 
        <header class="bg-gray-900 p-4 text-white fixed top-0 w-full z-50">
            <A href="/" class="text-2xl font-bold"><h1>"Home"</h1></A>
            <Transition
                fallback=move || view! {cx, <span class="text-gray-300">"Loading..."</span>}
            >
            {move || {
                user.read(cx).map(|user| match user {
                    Err(e) => view! {cx,
                        <A href="/signup" class="text-white">"Signup"</A>", "
                        <A href="/login" class="text-white">"Login"</A>", "
                        <span class="text-red-300">{format!("Login error: {}", e)}</span>
                    }.into_view(cx),
                    Ok(None) => view! {cx,
                        <A href="/signup" class="text-white">"Signup"</A>", "
                        <A href="/login" class="text-wihte">"Login"</A>", "
                        <span class="text-green-500">"Logged out."</span>
                    }.into_view(cx),
                    Ok(Some(user)) => match user.is_guest {
                        true => view! {cx,
                            <A href="/signup" class="text-white">"Signup"</A>", "
                            <A href="/login" class="text-white">"Login"</A>", "
                            <span class="text-yellow-300">"Guest user."</span>
                        }.into_view(cx),
                        false => view! {cx,
                            <A href="/settings" class="text-white">"Settings"</A>", "
                            // <A href="/logout" class="text-white">"Logout"</A>", " // New Logout link here
                            <span class="text-white-500">{format!("Logged in as: {} ({})", user.username, user.id.to_string())}</span>
                            <LogoutButton style="text-white"/>
                        }.into_view(cx),
                    }
                })
            }}
            </Transition>
        </header>
    }
}