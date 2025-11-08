use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::controllers::todo_controller;

pub fn todo_routing() -> Router {
    let app = Router::new()
    .route("/todo", get(todo_controller::get_todo))
    .route("/todo", post(todo_controller::create_todo))
    .route("/todo/{:id}", put(todo_controller::update_todo))
    .route("/todo/{:id}", delete(todo_controller::delete_todo))
    .route("/todo/done/{:id}", post(todo_controller::done_todo))
     .route("/todo/undone/{:id}", post(todo_controller::undone_todo))
    ;

    app
}
