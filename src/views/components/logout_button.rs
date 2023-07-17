use crate::models::context_structs::LogoutActionContext;
use leptos_router::ActionForm;
use leptos::*;

#[component]
pub fn LogoutButton(
    cx: Scope,
) -> impl IntoView {
    let logout = use_context::<LogoutActionContext>(cx).unwrap().0;
    
    view! {
        cx,
        <div id="logoutbox" class="flex items-center justify-center h-screen">
            <ActionForm action=logout class="flex items-center">
                <button type="submit" class="button px-4 py-2 bg-red-500 text-white rounded-md hover:bg-red-600">"Log Out"</button>
            </ActionForm>
        </div>
    }
}