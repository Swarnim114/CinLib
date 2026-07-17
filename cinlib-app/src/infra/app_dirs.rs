use std::path::PathBuf;

pub fn data_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("cinlib")
}

pub fn db_path() -> PathBuf {
    data_dir().join("cinlib.db")
}

pub fn cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from(".cache"))
        .join("cinlib")
}

pub fn poster_cache_dir() -> PathBuf {
    cache_dir().join("posters")
}

pub fn backdrop_cache_dir() -> PathBuf {
    cache_dir().join("backdrops")
}
