use leptos::*;
use crate::models::chat::chat_history::ChatHistory;
use crate::views::components::card::Card;
use crate::views::components::chat_message_pair::ChatMessagePair;

#[component]
pub fn ChatMessages(
    cx: Scope,
    current_history: ReadSignal<Vec<(ReadSignal<ChatHistory>, WriteSignal<ChatHistory>)>>
) -> impl IntoView {


    view! { cx,
// The <For/> component is central here
            // This allows for efficient, key list rendering
            <Card class="p-5 max-w-3xl w-full flex flex-col h-full mb-8">
                <div class="flex-1">
                        
                    <For
                        // `each` takes any function that returns an iterator
                        // this should usually be a signal or derived signal
                        // if it's not reactive, just render a Vec<_> instead of <For/>
                        each=current_history
                        // the key should be unique and stable for each row
                        // using an index is usually a bad idea, unless your list
                        // can only grow, because moving items around inside the list
                        // means their indices will change and they will all rerender
                        key=move |(chat_history,_)| chat_history.get().message_id
                        // the view function receives each item from your `each` iterator
                        // and returns a view
                        view=move |cx, (chat_history, _)| {
                            view! { cx,
                            
                                    <ChatMessagePair
                                        chat_history
                                    />
                            
                            }
                        }
                    />
                </div>
            </Card>
    }
}