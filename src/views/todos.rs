// use crate::components::*;

use leptos::*;
use leptos_router::{MultiActionForm, ActionForm};

// use crate::server_fn::todo::get_todos;

use crate::server_fn::todo::{get_todos, AddTodo, DeleteTodo};
use crate::models::todo::Todo;
use crate::error_template::ErrorTemplate;



#[component]
pub fn Todos(cx: Scope) -> impl IntoView {
    let add_todo = create_server_multi_action::<AddTodo>(cx);
    let delete_todo: Action<DeleteTodo, Result<(), ServerFnError>> = create_server_action::<DeleteTodo>(cx);
    let submissions: ReadSignal<Vec<leptos_server::Submission<AddTodo, Result<(), ServerFnError>>>> = add_todo.submissions();

    // list of todos is loaded from the server in reaction to changes
    let todos: Resource<(usize, usize), Result<Vec<Todo>, ServerFnError>> = create_resource(
        cx,
        move || (add_todo.version().get(), delete_todo.version().get()),
        move |_| get_todos(cx),
    );

//     view! {
//         cx,
//         <div class="flex flex-col items-center justify-center space-y-4">
//             <MultiActionForm action=add_todo class="space-y-2">
//                 <label class="flex items-center space-x-2">
//                     <span>"Add a Todo"</span>
//                     <input type="text" name="title" class="border border-gray-300 p-2 rounded-md"/>
//                 </label>
//                 <input type="submit" value="Add" class="px-4 py-2 bg-blue-600 text-white rounded-md cursor-pointer"/>
//             </MultiActionForm>
//             <Transition fallback=move || view! {cx, <p class="text-gray-500">"Loading..."</p> }>
//                 <ErrorBoundary fallback=|cx, errors| view!{ cx, <ErrorTemplate errors=errors/>}>
//                     {make_todo_list(cx, todos, delete_todo, submissions)}
//                 </ErrorBoundary>
//             </Transition>
//         </div>
//     }
// }

// fn make_todo_list(cx: Scope, todos: Resource<(usize, usize), Result<Vec<Todo>, ServerFnError>>, delete_todo: Action<DeleteTodo, Result<(), ServerFnError>>, submissions: ReadSignal<Vec<leptos_server::Submission<AddTodo, Result<(), ServerFnError>>>> ) -> impl IntoView {
//     move || {
//         let display_existing_todos = move || todos.read(cx).unwrap_or_default();
//         let map_todos_to_views = move |todos| todos.into_iter().map(|todo| todo_view(cx, todo, delete_todo)).collect_view(cx);
//         let handle_existing_todos = move |todos: Result<Vec<Todo>, ServerFnError>| match todos {
//             Err(e) => view! { cx, <pre class="error text-red-500">"Server Error: " {e.to_string()}</pre> }.into_view(cx),
//             Ok(todos) => if todos.is_empty() {
//                 view! { cx, <p class="text-gray-500">"No tasks were found."</p> }.into_view(cx)
//             } else {
//                 map_todos_to_views(todos)
//             }
//         };
        
//         let existing_todos = display_existing_todos().map(handle_existing_todos);

//         let pending_todos = move || {
//             submissions
//             .get()
//             .into_iter()
//             .filter(|submission| submission.pending().get())
//             .map(|submission| {
//                 view! {
//                     cx,
//                     <li class="pending text-gray-500">{move || submission.input.get().map(|data| data.title) }</li>
//                 }
//             })
//             .collect_view(cx)
//         };

//         view! {
//             cx,
//             <ul class="space-y-2">
//                 {existing_todos}
//                 {pending_todos}
//             </ul>
//         }
//     }
// }

// fn todo_view(cx: Scope, todo: Todo, delete_todo: Action<DeleteTodo, Result<(), ServerFnError>>) -> impl IntoView {
//     view! {
//         cx,
//         <li class="flex items-center space-x-2">
//             {todo.title}
//             ": Created at "
//             {todo.created_at}
//             " by "
//             {
//                 match todo.is_guest {
//                     true => "GUEST".to_string(),
//                     false => todo.user.unwrap_or_default().username
//                 }
//             }
//             <ActionForm action=delete_todo class="flex items-center space-x-2">
//                 <input type="hidden" name="id" value={todo.id.to_string()}/>
//                 <input type="submit" value="X" class="px-2 py-1 bg-red-600 text-white rounded-md cursor-pointer"/>
//             </ActionForm>
//         </li>
//     }
// }

    view! {
        cx,
        <div class="container mx-auto my-8">
            <MultiActionForm action=add_todo class="flex space-x-2 mb-4">
                <label class="flex flex-col px-8 w-full">
                    "Add a Todo:"
                    <div class="flex space-x-2 pt-1">
                        <input type="text" name="title" class="py-2 auth-input border border-gray-300 rounded-md flex-grow"/>
                        <input type="submit" value="Add" class="py-2 px-1 bg-blue-500 text-white rounded-md hover:bg-blue-600 flex-grow"/>
                    </div>
                </label>
                
            </MultiActionForm>
            <Transition fallback=move || view! {cx, <p class="text-center">"Loading..."</p> }>
                <ErrorBoundary fallback=|cx, errors| view!{ cx, <ErrorTemplate errors=errors/>}>
                    {{move || {
                        let existing_todos = {
                            move || {
                                todos.read(cx)
                                    .map(move |todos| match todos {
                                        Err(e) => {
                                            view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view(cx)
                                        }
                                        Ok(todos) => {
                                            if todos.is_empty() {
                                                view! { cx, <p>"No tasks were found."</p> }.into_view(cx)
                                            } else {
                                                todos
                                                    .into_iter()
                                                    .map(move |todo| {
                                                        view! {
                                                            cx,
                                                            <TodoView todo delete_todo />
                                                        }
                                                    })
                                                    .collect_view(cx)
                                            }
                                        }
                                    })
                                    .unwrap_or_default()
                            }
                        };

                        let pending_todos = move || {
                            submissions
                            .get()
                            .into_iter()
                            .filter(|submission| submission.pending().get())
                            .map(|submission| {
                                view! {
                                    cx,
                                    <li class="pending">{move || submission.input.get().map(|data| data.title) }</li>
                                }
                            })
                            .collect_view(cx)
                        };

                        view! {
                            cx,
                            <ul class="space-y-2">
                                {existing_todos}
                                {pending_todos}
                            </ul>
                        }
                    }
                }}
                </ErrorBoundary>
            </Transition>
        </div>
    }
}


#[component]
pub fn TodoView(
    cx: Scope,
    todo: Todo,
    delete_todo: Action<DeleteTodo, Result<(), ServerFnError>>
) -> impl IntoView {
    view! { cx,
        <li class="flex items-center justify-between p-4 mb-2 bg-white shadow rounded">
            <div class="flex items-center">
                <p class="mr-2 text-lg font-medium">{todo.title}</p>
                <p class="text-sm text-gray-500">"Created at "{todo.created_at}" by "{
                    match todo.is_guest {
                        true => "GUEST".to_string(),
                        false => todo.user.unwrap_or_default().username
                    }
                }</p>
            </div>
            <ActionForm action=delete_todo class="flex items-center">
                <input type="hidden" name="id" value={todo.id.to_string()}/>
                <button type="submit" class="px-2 py-1 bg-red-500 text-sm text-white rounded hover:bg-red-600">"Delete"</button>
            </ActionForm>
        </li>
    }
}