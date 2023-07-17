use passwords::analyzer;
use leptos::*;
// use leptos::html::Button;
use passwords::scorer;

// const SOME_THRESHOLD: f64 = 80.0;

#[component]
pub fn PasswordStrengthBar(
    cx: Scope,
    password: ReadSignal<std::string::String>,
    password_strength_signal: (ReadSignal<f64>, WriteSignal<f64>),
    // set_strength_pass: WriteSignal<bool>,

) -> impl IntoView {

    let (password_strength, set_password_strength) = password_strength_signal;

    // let (password_strength_note, set_password_strength_note) = create_signal(cx, String::new());
    
    create_effect(cx, move |_| {
        let password = password.get();
        // let analyzed = analyzer::analyze(&password);
        let strength_score =  scorer::score(&analyzer::analyze(password));
        
        set_password_strength.set(strength_score);
    });
    
    view! {
        cx,
        <div class="flex -mx-1">
        {(0..5).map(|i| {
            // let strength = strength.clone();
            view! {
                cx,
                <div class="w-1/5 px-1">
                    <div class={move || {
                        let strength_score: f64 = password_strength.get();
                        if (i as f64) < (strength_score / 20.0) {
                            if strength_score <= 40.0 {
                                // set_strength_pass(false);
                                // set_password_strength_note("very weak".to_string());
                                "h-2 rounded-xl transition-colors bg-red-400"
                                
                            } else if strength_score <= 80.0 {
                                // set_strength_pass(false);
                                // set_password_strength_note("good".to_string());
                                "h-2 rounded-xl transition-colors bg-yellow-400"
                            } else {
                                // set_strength_pass(true);
                                // set_password_strength_note("very good".to_string());
                                "h-2 rounded-xl transition-colors bg-green-500"
                            }
                        } else {
                            // set_strength_pass(false);
                            "h-2 rounded-xl transition-colors bg-gray-200"
                        }
                    }} />
                </div>
            }
        }).collect::<Vec<_>>()}
        </div>
        // <div class="password-strength-bar">
        //     //  style={move || format!("width: {}%", strength())}
        //     {password_strength_note}
        // </div>
    }
}

