use leptos::*;

#[component]
pub fn DisplayUploads(
    cx: Scope,
    /// the list of sha1's received from the server corresponding to the files uploaded
    uploads: Vec<String>,
) -> impl IntoView {
    
    view! {cx,
        <div class="flex space-x-2 ml-4 overflow-x-scroll">
            <div class="text-xs p-1 w-fit pb-1">"Uploads:"</div>
            {uploads.into_iter()
                .map(|upload| view! { cx, 
                        <div class="text-xs bg-gray-300 rounded-xl p-1 px-2 w-fit">
                            {upload}
                        </div>
                }).collect::<Vec<_>>()
            }
        </div>

    }
}