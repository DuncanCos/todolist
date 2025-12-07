use crate::models::todo_model::{ToDo, ToDoCreateBody, ToDoUpdateBody};

use axum::http::StatusCode;
use axum::{Extension, Json, extract, extract::Path, response::IntoResponse};

use clerk_rs::validators::authorizer::ClerkJwt;
use sqlx::postgres::PgPool;
use uuid::Uuid;

pub async fn get_todo(
    Extension(pool): Extension<PgPool>,
    claim: Extension<ClerkJwt>,
) -> impl IntoResponse {
    eprintln!("User JWT: {:?}", claim.sub);

    match sqlx::query_as::<_, ToDo>("SELECT * FROM todos WHERE user_id=$1")
        .bind(claim.sub.clone())
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

pub async fn create_todo(
    Extension(pool): Extension<PgPool>,
    claim: Extension<ClerkJwt>,
    // Path(id): extract::Path<i32>,
    extract::Json(body): extract::Json<ToDoCreateBody>,
) -> impl IntoResponse {
    let uuid = Uuid::new_v4();
    match sqlx::query(
        "
    INSERT INTO todos (uuid, title, description, status, created_at, user_id) 
    VALUES ($3,$1,$2,'todo',NOW(),$4)
    ",
    )
    .bind(body.title)
    .bind(body.description)
    .bind(uuid)
    .bind(claim.sub.clone())
    .fetch_all(&pool)
    .await
    {
        Ok(_todo) => (StatusCode::OK, "done".to_string()).into_response(),
        Err(err) => {
            eprintln!("Database query failed: {:?}", err);
            let message = "Unable to fetch users".to_string();
            (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
        }
    }
}

pub async fn delete_todo(
    Extension(pool): Extension<PgPool>,
    Path(id): extract::Path<i32>,
    claim: Extension<ClerkJwt>,
    // extract::Json(body): extract::Json<Body>,
) -> impl IntoResponse {
    match sqlx::query("DELETE FROM todos WHERE id=$1 AND user_id=$2")
        .bind(id)
        .bind(claim.sub.clone())
        .fetch_all(&pool)
        .await
    {
        Ok(_todo) => (StatusCode::OK, "done".to_string()).into_response(),
        Err(err) => {
            eprintln!("Database query failed: {:?}", err);
            let message = "Unable to fetch users".to_string();
            (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
        }
    }
}

pub async fn update_todo(
    Extension(pool): Extension<PgPool>,
    claim: Extension<ClerkJwt>,
    Path(id): extract::Path<i32>,
    extract::Json(body): extract::Json<ToDoUpdateBody>,
) -> impl IntoResponse {
    match sqlx::query("UPDATE todos SET title = $1, description=$2 WHERE id=$3 AND user_id=$4")
        .bind(body.title)
        .bind(body.description)
        .bind(id)
        .bind(claim.sub.clone())
        .fetch_all(&pool)
        .await
    {
        Ok(_todo) => (StatusCode::OK, "done".to_string()).into_response(),
        Err(err) => {
            eprintln!("Database query failed: {:?}", err);
            let message = "Unable to fetch users".to_string();
            (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
        }
    }
}

pub async fn done_todo(
    Extension(pool): Extension<PgPool>,
    Path(id): extract::Path<i32>,
    claim: Extension<ClerkJwt>,
    // extract::Json(body): extract::Json<Body>,
) -> impl IntoResponse {
    match sqlx::query("UPDATE todos SET status='done' WHERE id = $1 AND user_id=$2")
        .bind(id)
        .bind(claim.sub.clone())
        .execute(&pool)
        .await
    {
        Ok(_result) => (StatusCode::OK, "done".to_string()).into_response(),
        Err(err) => {
            eprintln!("Database query failed: {:?}", err);
            let message = "Unable to fetch users".to_string();
            (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
        }
    }
}

pub async fn undone_todo(
    Extension(pool): Extension<PgPool>,
    Path(id): extract::Path<i32>,
    claim: Extension<ClerkJwt>,
    // extract::Json(body): extract::Json<Body>,
) -> impl IntoResponse {
    match sqlx::query("UPDATE todos SET status='todo' WHERE id = $1 AND user_id=$2")
        .bind(id)
        .bind(claim.sub.clone())
        .execute(&pool)
        .await
    {
        Ok(_result) => (StatusCode::OK, "done".to_string()).into_response(),
        Err(err) => {
            eprintln!("Database query failed: {:?}", err);
            let message = "Unable to fetch users".to_string();
            (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
        }
    }
}
