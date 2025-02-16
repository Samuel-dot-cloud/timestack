use std::ops::Deref;
use clap::{Args, Subcommand};
use crate::db::{get_database_path, set_database_path};
use crate::sync::sync_data;
use crate::tracking::{log_event, track_typing};

#[derive(Subcommand)]
pub enum Commands {
    /// Log a file event (open, save, focus, typing)
    Heartbeat(HeartbeatArgs),

    /// Start tracking typing time
    Typing(TypingArgs),

    /// Sync tracked data to server
    Sync,

    /// Set the database path (used by VSCode)
    SetDBPath {
        path: String,
    },

    /// Get the current database path
    GetDBPath
}

/// Arguments for `Heartbeat` (logging file events)
#[derive(Args)]
pub struct HeartbeatArgs {
    pub file: String,
    pub activity: String,
    pub langauge: String,
    pub project: String,
    pub editor: String,
    pub metadata: Option<String>,
}

/// Arguments for `Typing` (tracking typing activity)
#[derive(Args)]
pub struct TypingArgs {
    pub file: String,
    pub language: String,
    pub project: String,
    pub editor: String,
}

pub fn execute_command(command: Commands) {
    match command {
        Commands::Heartbeat(args) => {
            let event = crate::tracking::Event {
                file: &args.file,
                activity: &args.activity,
                language: &args.langauge,
                project: &args.project,
                editor: &args.editor,
                metadata: args.metadata.as_deref(),
                duration: None
            };
            log_event(&event);
        }

        Commands::Typing(args) => {
            track_typing(&args.file, &args.language, &args.project, &args.editor);
        }

        Commands::Sync => {
            sync_data();
        }

        Commands::SetDBPath { path} => {
            set_database_path(&path);
            println!("Database path set to {}", path);
        }

        Commands::GetDBPath => {
            println!("{}", get_database_path().display());
        }
    }
}