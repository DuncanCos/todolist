use axum::{Extension, Router};
use sqlx::postgres::PgPool;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

use http::HeaderValue;

mod todo_route;

use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};

use tracing::Level;

// use crate::middlewares::auth::auth_middleware;
use clerk_rs::{
    ClerkConfiguration,
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
};

pub fn routing(pool: PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://localhost:5173"),
            HeaderValue::from_static("http://localhost:3000"),
            HeaderValue::from_static("http://127.0.0.1:5173"),
            HeaderValue::from_static("https://todo.dunkan.xyz"),
        ])
        .allow_methods(vec![
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
            http::Method::OPTIONS,
        ])
        .allow_headers(Any)
        .allow_credentials(true);

    let clerk_key = std::env::var("CLERK_SECRET_KEY").expect("CLERK_SECRET_KEY must be set");

    let config = ClerkConfiguration::new(None, None, Some(clerk_key), None);
    let clerk = Clerk::new(config);

    let app = Router::new()
        .nest("/todo", todo_route::todo_routing())
        .layer(Extension(pool))
        // .layer(from_fn(auth_middleware))
        .layer(ClerkLayer::new(
            MemoryCacheJwksProvider::new(clerk),
            None,
            true,
        ))
        .layer(cors)
        // .layer(from_fn(auth_middleware))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO)) // Log des requêtes entrantes
                .on_response(DefaultOnResponse::new().level(Level::INFO)), // Log des réponses
        );
    app
}
