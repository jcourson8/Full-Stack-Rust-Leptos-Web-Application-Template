use leptos::*;
use crate::models::chat::chat_message_pair::ChatMessagePair;
use crate::views::components::display_uploads::DisplayUploads;

#[component]
pub fn ChatMessagePair(
    cx: Scope,

    chat_message_pair: ReadSignal<ChatMessagePair>,
    // documents_upload: Option<Vec<String>>,
) -> impl IntoView {


    let display_uploads = move || {
        let documents_upload: Option<Vec<String>> = chat_message_pair.get().documents_upload.clone();
    
        match documents_upload {
            Some(documents) => view!{cx, <DisplayUploads uploads=documents />}.into_view(cx),
            None => view!{cx, }.into_view(cx),
        }
    };
    
    let chat_message_pair_user = move|| { 
        chat_message_pair.get().user_message.unwrap_or("ERROR".to_string())
    };
    let chat_message_pair_assistant = move|| { 
        chat_message_pair.get().assistant_message.unwrap_or("THINKING".to_string())
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
                {chat_message_pair_user()}
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
                {chat_message_pair_assistant()}
            </p>
                
            
        </div>
    }
}