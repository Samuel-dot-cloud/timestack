use crate::utils::{error_response, to_naive_datetime};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use chrono::{DateTime, NaiveDateTime, Utc};
use db::apps::App;
use db::branches::Branch;
use db::entities::Entity;
use db::events::Event;
use db::languages::Language;
use db::projects::Project;
use db::DBContext;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
struct EventInput {
    timestamp: Option<DateTime<Utc>>,
    duration: Option<i64>,
    activity_type: String,
    app_name: String,
    entity_name: String,
    entity_type: String,
    project_name: String,
    project_path: String,
    branch_name: String,
    language_name: String,
    end_timestamp: Option<DateTime<Utc>>,
}

async fn handle_event(
    State(db): State<Arc<Mutex<DBContext>>>,
    Json(payload): Json<EventInput>,
) -> Result<Json<String>, (StatusCode, Json<String>)> {
    let db = db.lock().await;

    let app_id = App::find_or_insert(&*db, &payload.app_name)
        .await
        .map_err(error_response)?;
    let project_id = Project::find_or_insert(&*db, &payload.project_name, &payload.project_path)
        .await
        .map_err(error_response)?;
    let branch_id = Branch::find_or_insert(&*db, project_id, &payload.branch_name)
        .await
        .map_err(error_response)?;
    let entity_id =
        Entity::find_or_insert(&*db, project_id, &payload.entity_name, &payload.entity_type)
            .await
            .map_err(error_response)?;
    let language_id = Language::find_or_insert(&*db, &payload.language_name)
        .await
        .map_err(error_response)?;

    let event = Event {
        id: None,
        timestamp: to_naive_datetime(payload.timestamp).unwrap_or_else(|| Utc::now().naive_utc()),
        duration: payload.duration,
        activity_type: payload.activity_type,
        app_id,
        entity_id: Some(entity_id),
        project_id: Some(project_id),
        branch_id: Some(branch_id),
        language_id: Some(language_id),
        end_timestamp: to_naive_datetime(payload.end_timestamp),
    };

    event.create(&*db).await.map_err(error_response)?;

    Ok(Json("Event recorded".to_string()))
}

pub fn event_routes(db: Arc<Mutex<DBContext>>) -> Router {
    Router::new()
        .route("/events", post(handle_event))
        .with_state(db)
}
