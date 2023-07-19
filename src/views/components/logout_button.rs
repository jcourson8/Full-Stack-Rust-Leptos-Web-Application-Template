use crate::{models::context_structs::LogoutActionContext, server_fn::authentication::Logout};
use leptos_router::{ActionForm, FromFormData};
use leptos::*;
use leptos::ev::SubmitEvent;

#[component]
pub fn LogoutButton(
    cx: Scope,
    #[prop(into, default="button px-4 py-2 bg-red-500 text-white rounded-md hover:bg-red-600".to_string())]
    style: String,
) -> impl IntoView {
    // let logout: Action<crate::server_fn::authentication::Logout, Result<(), ServerFnError>> = use_context::<LogoutActionContext>(cx).unwrap().0;
    // handle the logout action don't unwrap like above example:
    let logout_action = use_context::<LogoutActionContext>(cx);
    let logout: Action<crate::server_fn::authentication::Logout, Result<(), ServerFnError>> = match logout_action {
        Some(context) => context.0,
        None => {
            // Handle the case when the context is not found. This might be returning a default value, 
            // or you might want to render an error message to the user. Here we just log an error message and return.
            log::error!("LogoutActionContext not found in the context tree.");
            // return view! { cx, <div>"An error occurred."</div> }
            use_context::<LogoutActionContext>(cx).unwrap().0
        }
    };

    let on_submit = move |ev: SubmitEvent| {
        let logout = Logout::from_event(&ev);
        if logout.is_err() {
            // log::warn!("Logout error: {:?}", logout.err());
            ev.prevent_default();
        }
    };

    view! {
        cx,
        // <div id="logoutbox" class="flex items-center justify-center h-screen">
            <ActionForm action=logout class="flex" on:submit=on_submit >
                <button type="submit" class=style>"Log Out"</button>
            </ActionForm>
        // </div>
    }
}
