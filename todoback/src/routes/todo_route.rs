use axum::{
    routing::{get},
    Router,
};

use crate::controllers::todo_controller;

pub fn todo_routing() -> Router {
    let app = Router::new()
    .route("/todo", get(todo_controller::get_todo));
   
    app
}