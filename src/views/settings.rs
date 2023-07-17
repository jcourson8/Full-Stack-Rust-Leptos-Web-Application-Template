use leptos::*;
use crate::views::components::logout_button::LogoutButton;

#[component]
pub fn Settings(
    cx: Scope,
) -> impl IntoView {

    view! {
        cx,
        <h1>"Settings"</h1>
        <LogoutButton />
    }
}
