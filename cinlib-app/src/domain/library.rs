use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

use crate::domain::error::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LibraryKind {
    Movies,
    Series,
    Mixed,
}

impl LibraryKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Movies => "movies",
            Self::Series => "series",
            Self::Mixed  => "mixed",
        }
    }
}

impl std::fmt::Display for LibraryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for LibraryKind {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "movies" => Ok(Self::Movies),
            "series" => Ok(Self::Series),
            "mixed"  => Ok(Self::Mixed),
            other    => Err(DomainError::InvalidLibraryKind(other.to_owned())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub id:         Uuid,
    pub name:       String,
    pub path:       PathBuf,
    pub kind:       LibraryKind,
    pub created_at: DateTime<Utc>,
}

impl Library {
    pub fn new(name: impl Into<String>, path: PathBuf, kind: LibraryKind) -> Self {
        Self {
            id:         Uuid::new_v4(),
            name:       name.into(),
            path,
            kind,
            created_at: Utc::now(),
        }
    }
}
