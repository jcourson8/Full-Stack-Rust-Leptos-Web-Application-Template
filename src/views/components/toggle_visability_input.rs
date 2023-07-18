use::leptos::*;
use leptos::ev::{Event, MouseEvent};


#[component]
pub fn ToggleVisabilityInput<F>(
    cx: Scope,
    /// The label for the input box
    #[prop(into)]
    label: String,

    #[prop(into, default = "password".to_string())]
    input_name: String,
    /// The phrase in the toggle button
    #[prop(into, default = "show".to_string())]
    toggle_button_phrase: String,
    /// Input box placeholder. Defaults to the label.
    #[prop(into, default = label.clone())]
    placeholder: String,
    /// on:input function for the input box
    on_input: F,
    /// closue that will update the style of the input box reactivly
    #[prop(into,default = None)]
    reactive_input_style: Option<leptos::ReadSignal<std::string::String>>,
) -> impl IntoView 
where
    F: Fn(Event) + 'static, 
{

    let reactive_input_style = reactive_input_style.unwrap_or_else(|| create_signal(cx, "".to_string()).0);


    let (input_visability, set_input_visability) = create_signal(cx, "password".to_string());

    let toggle_visability = move |_:MouseEvent| {
        if input_visability() == "password".to_string() {
            set_input_visability("text".to_string());
        } else {
            set_input_visability("password".to_string())
        }
    };

    view! { cx,
        <label class="flex flex-col" for=input_name.clone()>
            {label}
        </label>
        <div class="relative w-full flex flex-col">
            <div class="absolute inset-y-0 right-0 flex items-center px-2">
                <input 
                    class="hidden" 
                    id=format!("{}{}","toggle_",input_name.clone()) 
                    type="checkbox" 
                />
                <label 
                    class="bg-gray-300 hover:bg-gray-400 rounded px-2 py-1 text-sm text-gray-600 cursor-pointer" 
                    for=format!("{}{}","toggle_",input_name.clone())  
                    on:click=toggle_visability>{toggle_button_phrase}
                </label>
            </div>
            <input 
                class={move || format!("auth-input px-3 py-2 border {} rounded-md", reactive_input_style())} 
                name=input_name.clone() 
                type=input_visability  
                placeholder=placeholder 
                on:input=on_input
                required
            />
        </div>
    }
}
