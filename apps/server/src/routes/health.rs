use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Server is running 🚀")
}

pub fn health_routes() -> Router {
    Router::new().route("/health", get(health_check))
}
