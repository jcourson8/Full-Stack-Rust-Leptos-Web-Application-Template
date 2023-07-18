use leptos::{*, html::Input, ev::SubmitEvent};
use uuid::Uuid;
use crate::models::chat::chat_message_pair::ChatMessagePair;
use chrono::{DateTime, Utc};

#[component]
pub fn ChatInput(
    cx: Scope,
    input_element: NodeRef<Input>,
    histories_signal: (ReadSignal<Vec<(ReadSignal<ChatMessagePair>, WriteSignal<ChatMessagePair>)>>, WriteSignal<Vec<(ReadSignal<ChatMessagePair>, WriteSignal<ChatMessagePair>)>>),

) -> impl IntoView {
    let (_current_history, set_current_history) = histories_signal;

    let (user_msg, _set_user_msg) = create_signal(cx, String::new());

    let add_chat_message_pair = move |ev: SubmitEvent| {
        ev.prevent_default();
        

        // here, we'll extract the value from the input
        let usr_msg = input_element()
            .expect("<input> to exist")
            .value();

        let chat_message_pair = ChatMessagePair {
            message_id: Uuid::new_v4(),
            chat_id: Uuid::new_v4(),
            user_message: Some(String::from(usr_msg)),
            assistant_message: None,
            message_time: Some(Utc::now()),
            documents_upload: Some(vec![String::from("doc1.pdf"), String::from("doc2.pdf")]),
        };
        // sig is (chat_message_pair, set_chat_message_pairtory)
        let sig = create_signal(cx, chat_message_pair);

        // add the user msg while we wait for response 
        set_current_history.update(move |current_history| {
            current_history.push(sig);
        });

        // sleeps for 2 seconds

        set_current_history.update(move |current_history| {

            if let Some((last_chat_message_pair, last_chat_message_pair_setter)) = current_history.last_mut() {

                let mut cloned_last_chat_message_pair = last_chat_message_pair.get().clone();
                cloned_last_chat_message_pair.assistant_message = Some(String::from("Assistant's response"));
                let sig: (ReadSignal<ChatMessagePair>, WriteSignal<ChatMessagePair>) = create_signal(cx, cloned_last_chat_message_pair);
                current_history.pop();
                current_history.push(sig);
        
            }
        });
        
    };

    view! { cx,
        <form 
            on:submit=add_chat_message_pair
            class="sticky bottom-0 p-5 bg-whiterounded-t-md border border-black/10 border-b-0 w-full flex items-center justify-center gap-2 z-20" //dark:bg-black dark:border-white/25 
        >
            <input
                autoFocus
                required

                type="Text"
                value=user_msg
                node_ref=input_element
                class="w-full p-2 border border-gray-300 outline-none rounded " //dark:border-gray-500 dark:bg-gray-800
                placeholder="Begin conversation here..."
            />
            <div class="m-2">
            <button 
                class="bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded" //sm:px-4 sm:py-2
                type="submit" 
            >
            "Submit"
            </button>
            </div>

        </form>
}
}