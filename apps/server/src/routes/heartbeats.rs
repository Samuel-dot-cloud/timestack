use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use chrono::Utc;
use db::heartbeats::Heartbeat;
use db::DBContext;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
struct HeartbeatInput {
    project_id: Option<i64>,
    entity_id: Option<i64>,
    branch_id: Option<i64>,
    language_id: Option<i64>,
    app_id: i64,
    is_write: bool,
    lines: Option<i64>,
    cursorpos: Option<i64>,
}

async fn handle_heartbeat(
    State(db): State<Arc<Mutex<DBContext>>>,
    Json(payload): Json<HeartbeatInput>,
) -> Json<String> {
    let db = db.lock().await;

    let heartbeat = Heartbeat {
        id: None,
        project_id: payload.project_id,
        entity_id: payload.entity_id,
        branch_id: payload.branch_id,
        language_id: payload.language_id,
        app_id: Some(payload.app_id),
        timestamp: Utc::now(),
        is_write: Some(payload.is_write),
        lines: payload.lines,
        cursorpos: payload.cursorpos,
    };

    match heartbeat.create(&*db).await {
        Ok(_) => Json("Heartbeat recorded".to_string()),
        Err(err) => Json(format!("Error: {}", err)),
    }
}

pub fn heartbeat_routes(db: Arc<Mutex<DBContext>>) -> Router {
    Router::new()
        .route("/heartbeats", post(handle_heartbeat))
        .with_state(db)
}
