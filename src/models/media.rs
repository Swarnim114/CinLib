// Media is one video file we found, with info pulled out of its filename.
// just data here, no logic. parsing lives in services::filename_parser,
// grouping lives in services::grouping.

use std::path::PathBuf;

#[derive(Debug)]
pub struct Media {
    pub path: PathBuf,
    pub title: String,
    pub year: Option<u32>,
    pub resolution: Option<String>,
    pub codec: Option<String>,
    pub source: Option<String>,
    pub extension: String,
    pub season: Option<u32>,
    pub episode: Option<u32>,
}

impl Media {
    // turns year (Option<u32>) into Option<String> so its easy to print
    pub fn year_as_text(&self) -> Option<String> {
        if self.year.is_none() {
            return None;
        }
        Some(self.year.unwrap().to_string())
    }
}

// Series is a show made up of multiple episodes, grouped by title
#[derive(Debug)]
pub struct Series {
    pub title: String,
    pub episodes: Vec<Media>,
}
