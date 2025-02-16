use std::time::Duration;
use reqwest::blocking::Client;
use serde_json::json;
use crate::db::open_database;

const SERVER_URL: &str = "";

/// Sync data to the server
pub fn sync_data() {
    let conn = open_database();
    let client = Client::new();

    // Fetch unsynced events
    let mut stmt = conn.prepare(
        "SELECT id, file, activity, branch_name, language, project, editor, metadata, timestamp, duration\
             FROM events where synced = 0"
    ).expect("Failed to prepare query");

    let events: Vec<_> = stmt
        .query_map([], |row| {
            let id: i32 = row.get(0).unwrap_or(0);
            let file: String = row.get(1).unwrap_or_default();
            let activity: String = row.get(2).unwrap_or_default();
            let branch_name: String = row.get(3).unwrap_or("unknown".to_string());
            let language: String = row.get(4).unwrap_or_default();
            let project: String = row.get(5).unwrap_or_default();
            let editor: String = row.get(6).unwrap_or_default();
            let metadata: String = row.get(7).unwrap_or("None".to_string());
            let timestamp: String = row.get(8).unwrap_or_default();
            let duration: Option<i64> = row.get(9).ok();

            Ok(json!({
                "id": id,
                "file": file,
                "activity": activity,
                "branch_name": branch_name,
                "language": language,
                "project": project,
                "editor": editor,
                "metadata": metadata,
                "timestamp": timestamp,
                "duration": duration,
            }))
        })
        .expect("Failed to fetch unsynced events")
        .filter_map(Result::ok)
        .collect();

    if events.is_empty() {
        println!("No unsynced events to send.");
        return;
    }

    let payload = json!({ "events": events });

    match client.post(SERVER_URL)
        .json(&payload)
        .timeout(Duration::from_secs(10))
        .send()
    {
        Ok(response) if response.status().is_success() => {
            println!("Successfully synced {} events.", events.len());

            // Mark events as synced
            let event_ids: Vec<i32> = events.iter()
                .map(|e| e["id"].as_i64().unwrap_or(0) as i32)
                .collect();

            if !event_ids.is_empty() {
                let placeholders = event_ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
                let sql = format!("UPDATE events SET synced = 1 WHERE id IN ({})", placeholders);

                let mut stmt = conn.prepare(&sql).expect("Failed to prepare update query");
                let params: Vec<&dyn rusqlite::ToSql> = event_ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect();

                stmt.execute(&params).expect("Failed to update sync status");
            }
        }

        Ok(response) => {
            eprintln!("Server responded with error: {}", response.status());
        }

        Err(err) => {
            eprintln!("Failed to sync data: {}", err);
        }
    }
}