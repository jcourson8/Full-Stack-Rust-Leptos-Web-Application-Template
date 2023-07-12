// use crate::server_fn::auth::*;
// use crate::views::{todos::Todos, login::Login, settings::Logout, signup::Signup};
// use crate::views::components::header::Header;
// use crate::models::user::User;

use cfg_if::cfg_if;
use leptos::*;
// use leptos_meta::*;
// use leptos_router::*;
// use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::todo::Todo;



cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::server_fn::{pool};
    use crate::models::todo::SqlTodo;
    use crate::server_fn::authentication::get_user;
}
}

#[server(GetTodos, "/api")]
pub async fn get_todos(cx: Scope) -> Result<Vec<Todo>, ServerFnError> {
    use futures::TryStreamExt;

    let pool = pool(cx)?;

    let mut todos = Vec::new();
    let mut rows =
        sqlx::query_as::<_, SqlTodo>("SELECT * FROM todos").fetch(&pool);

    while let Some(row) = rows.try_next().await? {
        todos.push(row);
    }

    // why can't we just have async closures?
    // let mut rows: Vec<Todo> = rows.iter().map(|t| async { t }).collect();

    let mut converted_todos = Vec::with_capacity(todos.len());

    for t in todos {
        let todo = t.into_todo(&pool).await;
        converted_todos.push(todo);
    }

    let todos: Vec<Todo> = converted_todos;

    Ok(todos)
}

#[server(AddTodo, "/api")]
pub async fn add_todo(cx: Scope, title: String) -> Result<(), ServerFnError> {
    let user = get_user(cx).await?;
    let pool = pool(cx)?;



    let (id, is_guest) = match user {
        Some(user) => {
            println!{"ADDTODO:{}", user.id}
            (user.id, user.is_guest)
        },
        None => (Uuid::parse_str("0000000-0000-0000-0000-000000000000").expect("Failed to parse UUID"), false), // if no user use auth session id
    };

    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    match sqlx::query(
        "INSERT INTO todos (title, user_id, completed, id, is_guest) VALUES (?, ?, false, ?, ?)",
    )
    .bind(title)
    .bind(id.to_string())
    .bind(Uuid::new_v4().to_string())
    .bind(is_guest)
    .execute(&pool)
    .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(DeleteTodo, "/api")]
pub async fn delete_todo(cx: Scope, id: Uuid) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;

    Ok(sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id.to_string())
        .execute(&pool)
        .await
        .map(|_| ())?)
}

