use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Goal {
    pub id: i64,
    pub name: String,
    pub target_duration: i64,
    pub frequency: String,
    pub exclude_days: Option<String>,
    pub progress: i64,
    pub metadata: Option<String>,
}

impl Goal {
    /// Insert a new goal
    pub async fn insert(pool: &SqlitePool, goal: Goal) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO goals (name, target_duration, frequency, exclude_days, progress, metadata)
            VALUES (?, ?, ?, ?, ?, ?)",
            goal.name,
            goal.target_duration,
            goal.frequency,
            goal.exclude_days,
            goal.progress,
            goal.metadata
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Update a goal progress by a specified amount
    pub async fn update_progress(pool: &SqlitePool, goal_id: i64, progress_inc: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE goals SET progress = progress + ? WHERE id = ?",
            progress_inc,
            goal_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Fetch all goals.
    pub async fn fetch_all(pool: &SqlitePool) -> Result<Vec<Goal>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, target_duration, frequency, exclude_days, progress, metadata
            FROM goals
            "#
        )
        .fetch_all(pool)
        .await?;

        let goals: Vec<Goal> = rows.into_iter().map(|row| Goal {
            id: row.id,
            name: row.name,
            target_duration: row.target_duration,
            frequency: row.frequency,
            exclude_days: row.exclude_days,
            progress: row.progress.unwrap_or_default(),
            metadata: row.metadata,
        }).collect();

        Ok(goals)
    }

    /// Fetch a specific goal by its name.
    pub async fn fetch_by_name(pool: &SqlitePool, goal_name: &str) -> Result<Option<Goal>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, target_duration, frequency, exclude_days, progress, metadata
            FROM goals WHERE name = ?
            "#,
            goal_name
        )
        .fetch_optional(pool)
        .await?;

        let goal = row.map(|row| Goal {
            id: row.id,
            name: row.name,
            target_duration: row.target_duration,
            frequency: row.frequency,
            exclude_days: row.exclude_days,
            progress: row.progress.unwrap_or_default(),
            metadata: row.metadata,
        });

        Ok(goal)
    }
}
