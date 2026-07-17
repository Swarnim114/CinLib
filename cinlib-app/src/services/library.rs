use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};
use std::{path::PathBuf, str::FromStr};
use uuid::Uuid;

use crate::domain::{Library, LibraryKind};

pub async fn get_all_libraries(pool: &SqlitePool) -> Result<Vec<Library>> {
    let rows = sqlx::query(
        "SELECT id, name, path, kind, created_at FROM libraries ORDER BY created_at ASC",
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch libraries")?;

    let mut libraries = Vec::new();
    for row in rows {
        let id_str: String = row.get("id");
        let name: String = row.get("name");
        let path_str: String = row.get("path");
        let kind_str: String = row.get("kind");
        let created_at_str: String = row.get("created_at");

        if let (Ok(id), Ok(kind), Ok(created_at)) = (
            Uuid::from_str(&id_str),
            LibraryKind::from_str(&kind_str),
            DateTime::parse_from_rfc3339(&created_at_str),
        ) {
            libraries.push(Library {
                id,
                name,
                path: PathBuf::from(path_str),
                kind,
                created_at: created_at.with_timezone(&Utc),
            });
        }
    }

    Ok(libraries)
}

pub async fn add_library(
    pool: &SqlitePool,
    name: &str,
    path: PathBuf,
    kind: LibraryKind,
) -> Result<Library> {
    let library = Library::new(name, path.clone(), kind);

    sqlx::query(
        "INSERT INTO libraries (id, name, path, kind, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(library.id.to_string())
    .bind(&library.name)
    .bind(library.path.to_string_lossy().as_ref())
    .bind(library.kind.as_str())
    .bind(library.created_at.to_rfc3339())
    .execute(pool)
    .await
    .context("Failed to insert new library")?;

    Ok(library)
}

pub async fn get_movies(pool: &SqlitePool) -> Result<Vec<crate::domain::Movie>> {
    let rows = sqlx::query(
        "SELECT * FROM movies ORDER BY title ASC",
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch movies")?;

    let mut movies = Vec::new();
    for row in rows {
        let id_str: String = row.get("id");
        let library_id_str: String = row.get("library_id");
        
        if let (Ok(id), Ok(library_id)) = (Uuid::from_str(&id_str), Uuid::from_str(&library_id_str)) {
            let path_str: String = row.get("file_path");
            let mut movie = crate::domain::Movie::from_file(
                library_id,
                row.get::<String, _>("title"),
                PathBuf::from(path_str),
                row.get::<i64, _>("file_size") as u64,
            );
            movie.id = id;
            movie.tmdb_id = row.get("tmdb_id");
            movie.overview = row.get("overview");
            movie.poster_path = row.get::<Option<String>, _>("poster_path").map(PathBuf::from);
            movie.backdrop_path = row.get::<Option<String>, _>("backdrop_path").map(PathBuf::from);
            movies.push(movie);
        }
    }

    Ok(movies)
}
