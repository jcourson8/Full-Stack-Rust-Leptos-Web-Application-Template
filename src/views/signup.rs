// use crate::components::*;
use crate::server_fn::authentication::Signup;
use leptos_router::ActionForm;
use leptos::*;

#[component]
pub fn Signup(
    cx: Scope,
    action: Action<Signup, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        cx,
        <div class="px-8 pt-10">
        <ActionForm action=action class="flex flex-col space-y-4">
            <h1 class="text-2xl font-bold text-center">"Sign Up"</h1>
            <label class="flex flex-col">
                "User ID:"
                <input type="text" placeholder="User ID" maxlength="32" name="username" class="auth-input px-3 py-2 border border-gray-300 rounded-md" />
            </label>
            <label class="flex flex-col">
                "Password:"
                <input type="password" placeholder="Password" name="password" class="auth-input px-3 py-2 border border-gray-300 rounded-md" />
            </label>
            <label class="flex flex-col">
                "Confirm Password:"
                <input type="password" placeholder="Password again" name="password_confirmation" class="auth-input px-3 py-2 border border-gray-300 rounded-md" />
            </label>
            <label class="flex items-center">
                <input type="checkbox" name="remember" class="auth-input mr-2" />
                "Remember me?"
            </label>
            <button type="submit" class="button px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600">"Sign Up"</button>
        </ActionForm>
        </div>
    }
}

