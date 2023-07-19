use leptos::*;
use crate::views::components::logout_button::LogoutButton;

#[component]
pub fn Settings(
    cx: Scope,
) -> impl IntoView {
    let log_uuid = uuid::Uuid::new_v4();
    log::info!("[route] [Settings - {}]", log_uuid);

    view! {
        cx,
        <h1>"Settings"</h1>
        <LogoutButton />
    }
}
