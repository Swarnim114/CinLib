use anyhow::{Context, Result};
use bytes::Bytes;
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::infra::app_dirs;

pub async fn save_poster(tmdb_path: &str, data: Bytes) -> Result<PathBuf> {
    let file_name = tmdb_path.trim_start_matches('/');
    let dir = app_dirs::poster_cache_dir();
    fs::create_dir_all(&dir)
        .await
        .context("create poster cache dir")?;

    let path = dir.join(file_name);
    fs::write(&path, data).await.context("write poster file")?;
    Ok(path)
}

pub async fn save_backdrop(tmdb_path: &str, data: Bytes) -> Result<PathBuf> {
    let file_name = tmdb_path.trim_start_matches('/');
    let dir = app_dirs::backdrop_cache_dir();
    fs::create_dir_all(&dir)
        .await
        .context("create backdrop cache dir")?;

    let path = dir.join(file_name);
    fs::write(&path, data).await.context("write backdrop file")?;
    Ok(path)
}
