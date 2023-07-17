use crate::server_fn::authentication::*;
use::leptos::*;
// use leptos::html::Button;
use leptos_router::ActionForm;


#[component]
pub fn Login(
    cx: Scope,
    action: Action<Login, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        cx,
        <div class="pt-10">
            <ActionForm action=action class="flex flex-col items-center space-y-4">
                <h1 class="text-2xl font-bold">"Log In"</h1>
                <label class="flex flex-col space-y-2">
                    <span class="text-sm font-medium">"User ID:"</span>
                    <input type="text" placeholder="User ID" maxlength="32" name="username" class="auth-input px-2 py-1 border border-gray-300 rounded-md" />
                </label>
                <label class="flex flex-col space-y-2">
                    <span class="text-sm font-medium">"Password:"</span>
                    <input type="password" placeholder="Password" name="password" class="auth-input px-2 py-1 border border-gray-300 rounded-md" />
                </label>
                <label class="flex items-center space-x-2">
                    <input type="checkbox" name="remember" class="auth-input" />
                    <span class="text-sm">"Remember me?"</span>
                </label>
                <button type="submit" class="button px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600">"Log In"</button>
            </ActionForm>
        </div>
    }
}

// // ChatGPT
// #[component]
// pub fn Login(
//     cx: Scope,
//     action: Action<Login, Result<(), ServerFnError>>,
// ) -> impl IntoView {
//     let (password, set_password) = create_signal(cx, String::new());
//     let button_ref = create_node_ref::<Button>(cx);

//     let on_input = move |ev| {
//         set_password(event_target_value(&ev));
//     };
    
//     view! {
//         cx,
//         <div class="pt-10">
//             <ActionForm action=action class="flex flex-col items-center space-y-4">
//                 <h1 class="text-2xl font-bold">"Log In"</h1>
//                 <label class="flex flex-col space-y-2">
//                     <span class="text-sm font-medium">"User ID:"</span>
//                     <input type="text" placeholder="User ID" maxlength="32" name="username" class="auth-input px-2 py-1 border border-gray-300 rounded-md" />
//                 </label>
//                 <label class="flex flex-col space-y-2">
//                     <span class="text-sm font-medium">"Password:"</span>
//                     <input type="password" placeholder="Password" name="password" class="auth-input px-2 py-1 border border-gray-300 rounded-md"
//                            on:input=on_input
//                     />
//                 </label>
//                 <PasswordStrengthBar password=password submit_button=button_ref />
//                 <label class="flex items-center space-x-2">
//                     <input type="checkbox" name="remember" class="auth-input" />
//                     <span class="text-sm">"Remember me?"</span>
//                 </label>
//                 <button type="submit" class="button px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600" node_ref=button_ref>"Log In"</button>
//             </ActionForm>
//         </div>
//     }
// }
