use crate::models::context_structs::LogoutActionContext;
use leptos_router::ActionForm;
use leptos::*;

#[component]
pub fn LogoutButton(
    cx: Scope,
    #[prop(into, default="button px-4 py-2 bg-red-500 text-white rounded-md hover:bg-red-600".to_string())]
    style: String,
) -> impl IntoView {
    let logout: Action<crate::server_fn::authentication::Logout, Result<(), ServerFnError>> = use_context::<LogoutActionContext>(cx).unwrap().0;


    view! {
        cx,
        // <div id="logoutbox" class="flex items-center justify-center h-screen">
            <ActionForm action=logout class="flex">
                <button type="submit" class=style>"Log Out"</button>
            </ActionForm>
        // </div>
    }
}