use::leptos::*;


#[component]
pub fn Chat(
    cx: Scope
) -> impl IntoView {

    view! { cx,
        <div>"HELLO WORLD"</div>
    }
}

// use::leptos::*;
// use leptos::html::Input;
// use crate::models::chat::chat_message_pair::ChatMessagePair;
// use crate::views::components::chat_messages::ChatMessages;
// use crate::views::components::chat_input::ChatInput;



// #[component]
// pub fn Chat(
//     cx: Scope
// ) -> impl IntoView {

//     let input_element: NodeRef<Input> = create_node_ref(cx);

    
//     // simulates getting a conversation stored on db
//     let initial_histories: Vec<(ReadSignal<ChatMessagePair>, WriteSignal<ChatMessagePair>)> = Vec::new();
//     let histories_signal = create_signal(cx, initial_histories);
//     let (current_history, _set_current_history) = histories_signal;

    
//     view! { cx,

    
//         // // component that displays the chat history
//         // <main className="relative flex flex-col h-screen">
//         // <ChatMessages current_history=current_history/>
//         // // component that allows the user to send a message
//         // <ChatInput input_element histories_signal />
//         <main class="flex flex-col">

//             <section class="overflow-hidden flex-grow fixed top-0">
//                 <ChatMessages current_history={current_history} />
//             </section>

//             <section class="fixed bottom-0 w-full p-3 bg-white">
//                 <ChatInput input_element={input_element} histories_signal={histories_signal} />
//             </section>

//         </main>



        
//     }
// }










// #[component]
// pub fn Chat(
//     cx: Scope
// ) -> impl IntoView {

//     let input_element: NodeRef<Input> = create_node_ref(cx);

//     a
//     // simulates getting a conversation stored on db
//     let initial_histories: Vec<(ReadSignal<ChatMessagePair>, WriteSignal<ChatMessagePair>)> = Vec::new();
//     let histories_signal = create_signal(cx, initial_histories);
//     let (current_history, _set_current_history) = histories_signal;

    
//     view! { cx,
//         <main className="flex flex-col w-full pt-10">
//             <section className="flex flex-col flex-1 items-center w-full h-full min-h-screen">
//                 <div className="relative h-full w-full flex flex-col flex-1 items-center">
//                     <div className="h-full flex-1 w-full flex flex-col items-center">
//                         <ChatMessages current_history=current_history/>
//                     </div>                    
//                 </div>
//                 <ChatInput input_element histories_signal />
//             </section>
//         </main>
//     }
// }