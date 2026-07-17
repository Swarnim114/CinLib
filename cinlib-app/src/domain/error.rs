use std::path::PathBuf;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("library not found: {0}")]
    LibraryNotFound(Uuid),

    #[error("movie not found: {0}")]
    MovieNotFound(Uuid),

    #[error("series not found: {0}")]
    SeriesNotFound(Uuid),

    #[error("episode not found: {0}")]
    EpisodeNotFound(Uuid),

    #[error("unknown library kind: '{0}'")]
    InvalidLibraryKind(String),

    #[error("library already exists at: {0:?}")]
    DuplicatePath(PathBuf),

    #[error("not a directory: {0:?}")]
    NotADirectory(PathBuf),
}
