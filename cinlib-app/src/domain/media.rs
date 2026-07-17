use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub id:              Uuid,
    pub library_id:      Uuid,
    pub title:           String,
    pub year:            Option<i32>,
    pub file_path:       PathBuf,
    pub file_size:       u64,
    pub duration_secs:   Option<u32>,
    pub tmdb_id:         Option<i64>,
    pub overview:        Option<String>,
    pub poster_path:     Option<PathBuf>,
    pub backdrop_path:   Option<PathBuf>,
    pub rating:          Option<f64>,
    pub genres:          Vec<String>,
    pub is_favorite:     bool,
    pub watch_position:  u32,
    pub watch_completed: bool,
    pub added_at:        DateTime<Utc>,
    pub updated_at:      DateTime<Utc>,
}

impl Movie {
    pub fn from_file(
        library_id: Uuid,
        title: impl Into<String>,
        file_path: PathBuf,
        file_size: u64,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            library_id,
            title: title.into(),
            year: None,
            file_path,
            file_size,
            duration_secs: None,
            tmdb_id: None,
            overview: None,
            poster_path: None,
            backdrop_path: None,
            rating: None,
            genres: Vec::new(),
            is_favorite: false,
            watch_position: 0,
            watch_completed: false,
            added_at: now,
            updated_at: now,
        }
    }

    // Returns progress as a fraction in [0.0, 1.0].
    pub fn progress(&self) -> Option<f64> {
        let dur = self.duration_secs? as f64;
        if dur == 0.0 {
            return None;
        }
        Some((self.watch_position as f64 / dur).clamp(0.0, 1.0))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    pub id:             Uuid,
    pub library_id:     Uuid,
    pub title:          String,
    pub year:           Option<i32>,
    pub folder_path:    PathBuf,
    pub tmdb_id:        Option<i64>,
    pub overview:       Option<String>,
    pub poster_path:    Option<PathBuf>,
    pub backdrop_path:  Option<PathBuf>,
    pub rating:         Option<f64>,
    pub genres:         Vec<String>,
    pub status:         Option<String>,
    pub total_episodes: u32,
    pub is_favorite:    bool,
    pub added_at:       DateTime<Utc>,
    pub updated_at:     DateTime<Utc>,
}

impl Series {
    pub fn from_folder(
        library_id: Uuid,
        title: impl Into<String>,
        folder_path: PathBuf,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            library_id,
            title: title.into(),
            year: None,
            folder_path,
            tmdb_id: None,
            overview: None,
            poster_path: None,
            backdrop_path: None,
            rating: None,
            genres: Vec::new(),
            status: None,
            total_episodes: 0,
            is_favorite: false,
            added_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub id:              Uuid,
    pub series_id:       Uuid,
    pub title:           Option<String>,
    pub season_number:   u32,
    pub episode_number:  u32,
    pub file_path:       PathBuf,
    pub file_size:       u64,
    pub duration_secs:   Option<u32>,
    pub overview:        Option<String>,
    pub still_path:      Option<PathBuf>,
    pub watch_position:  u32,
    pub watch_completed: bool,
    pub air_date:        Option<NaiveDate>,
    pub added_at:        DateTime<Utc>,
}

impl Episode {
    pub fn from_file(
        series_id: Uuid,
        season_number: u32,
        episode_number: u32,
        file_path: PathBuf,
        file_size: u64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            series_id,
            title: None,
            season_number,
            episode_number,
            file_path,
            file_size,
            duration_secs: None,
            overview: None,
            still_path: None,
            watch_position: 0,
            watch_completed: false,
            air_date: None,
            added_at: Utc::now(),
        }
    }

    pub fn label(&self) -> String {
        format!("S{:02}E{:02}", self.season_number, self.episode_number)
    }

    pub fn progress(&self) -> Option<f64> {
        let dur = self.duration_secs? as f64;
        if dur == 0.0 {
            return None;
        }
        Some((self.watch_position as f64 / dur).clamp(0.0, 1.0))
    }
}
