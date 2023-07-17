use::leptos::*;

#[component]
pub fn Card(
    cx: Scope,
    #[prop(into)]
    class: String,
    children: Children
) -> impl IntoView {

    let cardStyle = format!{"{}{}","shadow-md hover:shadow-xl transition-shadow rounded-xl overflow-hidden bg-white border-black/10", class};

    view! { cx,
        <div
            // ref={ref as LegacyRef<HTMLDivElement>}
            class=cardStyle// dark:shadow-primary/25 dark:bg-black border dark:border-white/25
            // {...props}
        >
            {children(cx)}
        </div>
    }
}
