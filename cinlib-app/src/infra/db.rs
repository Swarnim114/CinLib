use anyhow::{Context, Result};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use std::path::Path;
use std::str::FromStr;
use tracing::info;

pub type DbPool = sqlx::SqlitePool;

pub async fn init_pool(db_path: &Path) -> Result<DbPool> {
    if let Some(parent) = db_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .with_context(|| format!("create data dir: {}", parent.display()))?;
    }

    let options = SqliteConnectOptions::from_str(&format!("sqlite://{}", db_path.display()))
        .with_context(|| format!("invalid sqlite path: {}", db_path.display()))?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(8)
        .connect_with(options)
        .await
        .with_context(|| format!("open pool at: {}", db_path.display()))?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("run migrations")?;

    info!(path = %db_path.display(), "db ready");
    Ok(pool)
}
