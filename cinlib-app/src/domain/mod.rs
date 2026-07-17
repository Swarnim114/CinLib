pub mod error;
pub mod library;
pub mod media;
pub mod metadata;

pub use error::DomainError;
pub use library::{Library, LibraryKind};
pub use media::{Episode, Movie, Series};
