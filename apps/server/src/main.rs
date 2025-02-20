use std::path::PathBuf;
use std::sync::Arc;
use db::DBContext;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing_subscriber::{fmt, EnvFilter};
use crate::app::create_app;

mod routes;
mod utils;
mod app;

#[tokio::main]
async fn main() {
    fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let db_path = utils::get_application_support_path();
    let db_url = format!("sqlite://{}", db_path.to_str().unwrap());

    let db = Arc::new(Mutex::new(DBContext::new(&db_url).await.expect("Failed to connect to DB")));

    let app = create_app(db.clone()).await;

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
