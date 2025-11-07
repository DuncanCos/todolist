use crate::models::todo_model::ToDo;

use axum::http::StatusCode;
use axum::{
    // extract,  extract::Path, 
    response::IntoResponse,
    Extension, Json,
};


use sqlx::postgres::PgPool;





pub async fn get_todo(
    Extension(pool): Extension<PgPool>,
    // Path(id): extract::Path<i32>,
    // extract::Json(body): extract::Json<Body>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, ToDo>("SELECT * FROM todos")
        .fetch_all(&pool)
        .await
    {
        Ok(todo) => (StatusCode::OK, Json(todo)).into_response(),
        Err(err) => {
            eprintln!("Database query failed: {:?}", err);
            let message = "Unable to fetch users".to_string();
            (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
        }
    }
}