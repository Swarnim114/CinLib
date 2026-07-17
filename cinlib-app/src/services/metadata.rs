use anyhow::{Context, Result};
use serde::Deserialize;
use sqlx::SqlitePool;
use tracing::{info, warn};
use std::env;

use crate::domain::Movie;
use crate::infra::{cache, http};

const TMDB_API_BASE: &str = "https://api.themoviedb.org/3";
const TMDB_IMAGE_BASE: &str = "https://image.tmdb.org/t/p";

#[derive(Deserialize)]
struct TmdbSearchResponse {
    results: Vec<TmdbSearchResult>,
}

#[derive(Deserialize)]
struct TmdbSearchResult {
    id: i64,
    overview: Option<String>,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
}

pub async fn fetch_movie_metadata(pool: &SqlitePool, movie: &Movie) -> Result<()> {
    info!(title = %movie.title, "fetching metadata");

    let api_key = match env::var("TMDB_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            warn!("TMDB_API_KEY not set. Skipping metadata fetch.");
            return Ok(());
        }
    };

    let url = format!("{}/search/movie", TMDB_API_BASE);
    let res = http::HTTP_CLIENT
        .get(&url)
        .query(&[("api_key", &api_key), ("query", &movie.title)])
        .send()
        .await
        .context("Failed to send TMDB search request")?;

    if !res.status().is_success() {
        warn!(status = %res.status(), "TMDB search failed");
        return Ok(());
    }

    let search_res: TmdbSearchResponse = res.json().await.context("Failed to parse TMDB response")?;

    let first_result = match search_res.results.into_iter().next() {
        Some(res) => res,
        None => {
            info!(title = %movie.title, "no results found on TMDB");
            return Ok(());
        }
    };

    let mut local_poster_path = None;
    if let Some(ref path) = first_result.poster_path {
        let poster_url = format!("{}/w500{}", TMDB_IMAGE_BASE, path);
        if let Ok(bytes) = http::get_bytes(&poster_url).await {
            if let Ok(saved_path) = cache::save_poster(path, bytes).await {
                local_poster_path = Some(saved_path.to_string_lossy().to_string());
            }
        }
    }

    let mut local_backdrop_path = None;
    if let Some(ref path) = first_result.backdrop_path {
        let backdrop_url = format!("{}/w1280{}", TMDB_IMAGE_BASE, path);
        if let Ok(bytes) = http::get_bytes(&backdrop_url).await {
            if let Ok(saved_path) = cache::save_backdrop(path, bytes).await {
                local_backdrop_path = Some(saved_path.to_string_lossy().to_string());
            }
        }
    }

    sqlx::query(
        r#"
        UPDATE movies
        SET tmdb_id = ?, overview = ?, poster_path = ?, backdrop_path = ?
        WHERE id = ?
        "#,
    )
    .bind(first_result.id)
    .bind(first_result.overview)
    .bind(local_poster_path)
    .bind(local_backdrop_path)
    .bind(movie.id.to_string())
    .execute(pool)
    .await
    .context("Failed to update movie metadata in database")?;

    info!(title = %movie.title, tmdb_id = first_result.id, "metadata updated successfully");
    Ok(())
}
