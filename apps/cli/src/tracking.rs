use std::path::Path;
use std::process::Command;
use chrono::Utc;
use rusqlite::params;
use crate::db::open_database;

/// Struct to store file event details
pub struct Event<'a> {
    pub file: &'a str,
    pub activity: &'a str,
    pub language: &'a str,
    pub project: &'a str,
    pub editor: &'a str,
    pub metadata: Option<&'a str>,
    pub duration: Option<i64>,
}

pub fn log_event(event: &Event) {
    let conn = open_database();
    let timestamp = Utc::now();
    let branch_name = get_git_branch(event.file);
    let metadata_value = event.metadata.unwrap_or("None");
    let duration_value = event.duration.unwrap_or(0);

    // TODO: Use match instead of expect
    conn.execute(
        "INSERT INTO events (file, activity, branch_name, language, project, editor, metadata, timestamp, duration)\
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            event.file,
            event.activity,
            branch_name,
            event.language,
            event.project,
            event.editor,
            metadata_value,
            timestamp.to_rfc3339(),
            duration_value
        ],
    ).expect("Failed to insert event");
}

/// Get the current Git branch name (if applicable)
fn get_git_branch(file: &str) -> String {
    let path = Path::new(file).parent();
    if let Some(dir) = path {
        let output = Command::new("git")
            .arg("-C")
            .arg(dir)
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                return String::from_utf8_lossy(&output.stdout).trim().to_string();
            }
        }
    }
    "Unknown".to_string()
}
