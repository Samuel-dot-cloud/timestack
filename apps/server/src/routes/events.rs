use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use chrono::{NaiveDateTime, Utc};
use db::events::Event;
use db::DBContext;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
struct EventInput {
    timestamp: Option<NaiveDateTime>,
    duration: Option<i64>,
    activity_type: String,
    app_id: i64,
    entity_id: Option<i64>,
    project_id: Option<i64>,
    branch_id: Option<i64>,
    language_id: Option<i64>,
    end_timestamp: Option<NaiveDateTime>,
}

async fn handle_event(
    State(db): State<Arc<Mutex<DBContext>>>,
    Json(payload): Json<EventInput>,
) -> Json<String> {
    let db = db.lock().await;

    let event = Event {
        id: None,
        timestamp: payload.timestamp.unwrap_or_else(|| Utc::now().naive_utc()),
        duration: payload.duration,
        activity_type: payload.activity_type,
        app_id: payload.app_id,
        entity_id: payload.entity_id,
        project_id: payload.project_id,
        branch_id: payload.branch_id,
        language_id: payload.language_id,
        end_timestamp: payload.end_timestamp,
    };

    match event.create(&*db).await {
        Ok(_) => Json("Event recorded".to_string()),
        Err(err) => Json(format!("Error: {}", err)),
    }
}

pub fn event_routes(db: Arc<Mutex<DBContext>>) -> Router {
    Router::new()
        .route("/events", post(handle_event))
        .with_state(db)
}
