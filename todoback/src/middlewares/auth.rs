use axum::{Extension, extract::Request, http::StatusCode, middleware::Next, response::Response};
use clerk_rs::validators::authorizer::ClerkJwt;

pub async fn auth_middleware(
    claim: Option<Extension<ClerkJwt>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
}
