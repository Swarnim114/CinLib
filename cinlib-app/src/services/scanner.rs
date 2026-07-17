use anyhow::{Context, Result};
use sqlx::SqlitePool;
use std::path::Path;
use tracing::{info, warn};
use walkdir::WalkDir;

use crate::domain::{Library, LibraryKind, Movie};

const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv", "avi", "mov", "webm"];

pub async fn scan_library(pool: &SqlitePool, library: &Library) -> Result<()> {
    info!(library = %library.name, kind = ?library.kind, "starting scan");

    match library.kind {
        LibraryKind::Movies => scan_movies(pool, library).await?,
        LibraryKind::Series => warn!("series scanning not yet implemented"),
        LibraryKind::Mixed => warn!("mixed scanning not yet implemented"),
    }

    Ok(())
}

async fn scan_movies(pool: &SqlitePool, library: &Library) -> Result<()> {
    let mut added = 0;
    
    // In a real app we'd want to yield to the tokio runtime periodically,
    // or run this on a blocking thread pool.
    for entry in WalkDir::new(&library.path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        
        if !VIDEO_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
            continue;
        }

        let file_size = entry.metadata().map(|m| m.len()).unwrap_or(0);
        let title = path.file_stem().and_then(|s| s.to_str()).unwrap_or("Unknown Movie");

        let movie = Movie::from_file(library.id, title, path.to_path_buf(), file_size);

        let path_str = movie.file_path.to_string_lossy();
        let query = sqlx::query(
            r#"
            INSERT INTO movies (id, library_id, title, file_path, file_size, added_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(file_path) DO NOTHING
            "#)
            .bind(movie.id.to_string())
            .bind(movie.library_id.to_string())
            .bind(&movie.title)
            .bind(path_str.as_ref())
            .bind(movie.file_size as i64)
            .bind(movie.added_at.to_rfc3339())
            .bind(movie.updated_at.to_rfc3339());

        match query.execute(pool).await {
            Ok(res) if res.rows_affected() > 0 => {
                info!(title = %movie.title, "added movie");
                added += 1;
            }
            Ok(_) => {} // Already exists
            Err(e) => warn!(error = %e, path = %path.display(), "failed to insert movie"),
        }
    }

    info!(added, "completed movie scan");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::db::init_pool;
    use sqlx::Row;
    use std::fs;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_movie_scanner() {
        let dir = tempdir().expect("failed to create temp dir");
        let db_path = dir.path().join("cinlib.db");
        let pool = init_pool(&db_path).await.expect("failed to init db");

        let lib_path = dir.path().join("movies");
        fs::create_dir_all(&lib_path).expect("failed to create movies dir");
        
        fs::write(lib_path.join("Inception.mp4"), "dummy data").unwrap();
        fs::write(lib_path.join("The Matrix.mkv"), "dummy data").unwrap();
        fs::write(lib_path.join("notes.txt"), "not a video").unwrap();

        let library = Library::new("Test Movies", lib_path, LibraryKind::Movies);

        sqlx::query(
            "INSERT INTO libraries (id, name, path, kind, created_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(library.id.to_string())
        .bind(&library.name)
        .bind(library.path.to_string_lossy().as_ref())
        .bind(library.kind.as_str())
        .bind(library.created_at.to_rfc3339())
        .execute(&pool)
        .await
        .expect("failed to insert library");

        scan_library(&pool, &library).await.expect("scanner failed");

        let rows = sqlx::query("SELECT title, file_size FROM movies ORDER BY title")
            .fetch_all(&pool)
            .await
            .expect("failed to fetch movies");

        assert_eq!(rows.len(), 2, "should have found exactly 2 movies");
        
        let title1: String = rows[0].get("title");
        let title2: String = rows[1].get("title");
        
        assert_eq!(title1, "Inception");
        assert_eq!(title2, "The Matrix");
    }
}
