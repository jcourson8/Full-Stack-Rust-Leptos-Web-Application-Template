// use crate::components::*;
use crate::server_fn::authentication::Signup;
use leptos_router::ActionForm;
use leptos::{*, ev::SubmitEvent};
use crate::views::components::password_strength_bar::PasswordStrengthBar;
use leptos::html::Button;

#[component]
pub fn Signup(
    cx: Scope,
    action: Action<Signup, Result<(), ServerFnError>>,
) -> impl IntoView {

    let (password, set_password) = create_signal(cx, String::new());
    
    let (confirm_password, set_confirm_password) = create_signal(cx, String::new());
    let (confirm_password_border_color, set_confirm_password_border_color) = create_signal(cx, "".to_string());

    let (password_visability, set_password_visability) = create_signal(cx, "password".to_string());
    let (confirm_password_visability, set_confirm_password_visability) = create_signal(cx, "password".to_string());
    
    let toggle_password = move |_| {
        if password_visability() == "password".to_string() {
            set_password_visability("text".to_string());
        } else {
            set_password_visability("password".to_string())
        }
    };

    let toggle_confirm_password = move |_| {
        if confirm_password_visability() == "password".to_string() {
            set_confirm_password_visability("text".to_string());
        } else {
            set_confirm_password_visability("password".to_string())
        }
    };


    let (password_strength, set_password_strength) = create_signal(cx, 0.0 as f64);

    let button_ref = create_node_ref::<Button>(cx);

    let on_password_input = move |ev| {
        set_password(event_target_value(&ev));
    };

    let on_confirm_password_input = move |ev| {
        set_confirm_password(event_target_value(&ev));
    };

    // for coloring the confirm box red when it doesnt match
    create_effect(cx, move |_| {
        if password.get() == confirm_password.get() {
            set_confirm_password_border_color("".to_string());
        } else {
            set_confirm_password_border_color("focus:outline-none focus:ring-2 focus:ring-red-500".to_string());
        }
    });

    let on_submit = move |ev: SubmitEvent| {
        if password_strength() < 75.0 || password.get() != confirm_password.get() {
            // ev.prevent_default() will prevent form submission
            ev.prevent_default();

        }
    };

    view! {
        cx,
        <div class="px-8 pt-10 max-w-md mx-auto mt-5 mb-5">
        <ActionForm action=action class="flex flex-col space-y-4" on:submit=on_submit >
            <h1 class="text-2xl font-bold text-center">"Sign Up"</h1>

            <label class="flex flex-col" for="password">
                "User ID:"
            </label>
            <label class="flex flex-col">
                <input type="text" placeholder="User ID" maxlength="32" name="username" class="auth-input px-3 py-2 border border-gray-300 rounded-md" required/>
            </label>

            <label class="flex flex-col" for="password">
                "Email:"
            </label>
            <label class="flex flex-col">
                <input type="text" placeholder="Email" maxlength="32" name="username" class="auth-input px-3 py-2 border border-gray-300 rounded-md" required/>
            </label>


            <label class="flex flex-col" for="password">
                "Password:"
            </label>
            <div class="relative w-full flex flex-col">
                <div class="absolute inset-y-0 right-0 flex items-center px-2">
                    <input class="hidden" id="toggle" type="checkbox" />
                    <label class="bg-gray-300 hover:bg-gray-400 rounded px-2 py-1 text-sm text-gray-600 cursor-pointer" for="toggle" on:click=toggle_password>"show"</label>
                </div>
                <input class="auth-input px-3 py-2 border border-gray-300 rounded-md" name="password" type=password_visability placeholder="Password" on:input=on_password_input required
                />
            </div>

            <label class="flex flex-col" for="password_confirm">
                "Confirm Password:"
            </label>
            <div class="relative w-full flex flex-col">
                <div class="absolute inset-y-0 right-0 flex items-center px-2">
                    <input class="hidden" id="toggle_confirm" type="checkbox" />
                    <label class="bg-gray-300 hover:bg-gray-400 rounded px-2 py-1 text-sm text-gray-600 cursor-pointer" for="toggle_confirm" on:click=toggle_confirm_password>"show"</label>
                </div>
                <input class={move || format!("auth-input px-3 py-2 border {} rounded-md", confirm_password_border_color())} name="password_confirm" type=confirm_password_visability  placeholder="Confirm Password" on:input=on_confirm_password_input required
                />
            </div>

            // <label class="flex flex-col">
            //     "Confirm Password:"
            //     <input type="password" placeholder="Password again" name="password_confirmation" class={move || format!("auth-input px-3 py-2 border {} rounded-md", confirm_password_border_color())} on:input=on_confirm_password_input />
            // </label>

            <PasswordStrengthBar password=password password_strength_signal=(password_strength, set_password_strength) />
            <label class="flex items-center">
                <input type="checkbox" name="remember" class="auth-input mr-2" />
                "Remember me?"
            </label>
            <button type="submit" class="button px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600" node_ref=button_ref>"Sign Up"</button>
        </ActionForm>
        </div>
    }
}
