// use crate::components::*;
use crate::server_fn::authentication::*;
use leptos_router::ActionForm;
use leptos::*;

#[component]
pub fn Logout(
    cx: Scope,
    action: Action<Logout, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        cx,
        <div id="loginbox" class="flex items-center justify-center h-screen">
            <ActionForm action=action class="flex items-center">
                <button type="submit" class="button px-4 py-2 bg-red-500 text-white rounded-md hover:bg-red-600">"Log Out"</button>
            </ActionForm>
        </div>
    }
}