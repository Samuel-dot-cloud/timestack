use crate::db::open_database;
use chrono::Utc;
use rusqlite::params;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

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

/// Tracks the start time of typing
static mut TYPING_START: Option<Instant> = None;
/// Tracks if the user is currently typing
static mut IS_TYPING: bool = false;

/// Log a file event (open, save, focus, typing) with relevant metadata
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

/// Detect typing activity and log typing time
pub fn track_typing(file: &str, language: &str, project: &str, editor: &str) {
    let file = file.to_string();
    let language = language.to_string();
    let project = project.to_string();
    let editor = editor.to_string();

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));

            unsafe {
                if IS_TYPING {
                    let elapsed = TYPING_START.unwrap().elapsed().as_secs();

                    // If no typing for 3+ seconds, log typing time and reset
                    if elapsed > 3 {
                        let event = Event {
                            file: &file,
                            activity: "typing",
                            language: &language,
                            project: &project,
                            editor: &editor,
                            metadata: Some("User was actively typing"),
                            duration: Some(elapsed as i64),
                        };

                        log_event(&event);

                        IS_TYPING = false;
                    }
                }
            }
        }
    });
}

/// Reset typing timer when the user starts typing
pub fn start_typing() {
    unsafe {
        if !IS_TYPING {
            TYPING_START = Some(Instant::now());
            IS_TYPING = true;
        }
    }
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
