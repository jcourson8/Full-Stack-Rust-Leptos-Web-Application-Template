use leptos::*;
use crate::models::chat::chat_history::ChatHistory;
use crate::views::components::display_uploads::DisplayUploads;

#[component]
pub fn ChatMessagePair(
    cx: Scope,

    chat_history: ReadSignal<ChatHistory>,
    // documents_upload: Option<Vec<String>>,
) -> impl IntoView {


    let display_uploads = move || {
        let documents_upload: Option<Vec<String>> = chat_history.get().documents_upload.clone();
    
        match documents_upload {
            Some(documents) => view!{cx, <DisplayUploads uploads=documents />}.into_view(cx),
            None => view!{cx, }.into_view(cx),
        }
    };
    
    let chat_history_user = move|| { 
        chat_history.get().user_message.unwrap_or("ERROR".to_string())
    };
    let chat_history_assistant = move|| { 
        chat_history.get().assistant.unwrap_or("THINKING".to_string())
    };


    view! { cx,
        <div 
            class="py-3 px-3 md:px-6 w-full flex flex-col max-w-4xl overflow-hidden scroll-pb-32" 
            style="whitespace: \"pre-line\""
        >
            <div class="flex mb-2">
                <div>
                    <span class="capitalize text-xs bg-sky-200 rounded-xl p-1 px-2 w-fit"> // dark:bg-sky-200
                    "User"
                    </span>
                </div>
                {display_uploads()}
            </div>
            // Render the text content with markdown
            <p class="prose ml-[6px] mt-1"> //dark:prose-invert
                {chat_history_user()}
            </p>
                
            
        </div>
        <div class="py-3 px-3 md:px-6 w-full flex flex-col max-w-4xl overflow-hidden scroll-pb-32 bg-gray-200 bg-opacity-60 py-8">
            <div class="flex mb-2">
                <div>
                    <span class="capitalize text-xs bg-sky-200 rounded-xl p-1 px-2 w-fit"> // dark:bg-sky-200
                    "Assistant"
                    </span>
                </div>
            </div>
            // Render the text content with markdown
            <p class="prose ml-[6px] mt-1"> //dark:prose-invert
                {chat_history_assistant()}
            </p>
                
            
        </div>
    }
}