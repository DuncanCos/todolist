use axum::{
    Extension, Router,
};
use sqlx::postgres::PgPool;
use tower_http::cors::CorsLayer;

use http::HeaderValue;

mod todo_route;


use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};

use tracing::Level;




pub fn routing(pool: PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://localhost:5173"),
            HeaderValue::from_static("http://127.0.0.1:5173"),
        ])
        .allow_methods(vec![
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
            http::Method::OPTIONS,
        ])
        .allow_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
            http::header::SET_COOKIE,
            http::header::COOKIE,
        ])
        .allow_credentials(true);

    let app = Router::new()
        .nest("/todo", todo_route::todo_routing())
        .layer(Extension(pool))
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO)) // Log des requêtes entrantes
                .on_response(DefaultOnResponse::new().level(Level::INFO)), // Log des réponses
        );
    app
}