use crate::infra::db::DbPool;
use std::fmt;

#[derive(Clone)]
pub enum DbStatus {
    Initialising,
    Ready(DbPool),
    Failed(String),
}

impl DbStatus {
    pub fn pool(&self) -> Option<&DbPool> {
        match self {
            Self::Ready(p) => Some(p),
            _ => None,
        }
    }

    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Ready(_))
    }
}

impl fmt::Debug for DbStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Initialising => write!(f, "Initialising"),
            Self::Ready(_)     => write!(f, "Ready"),
            Self::Failed(e)    => write!(f, "Failed({e})"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: DbStatus,
}

impl AppState {
    pub fn new() -> Self {
        Self { db: DbStatus::Initialising }
    }
}
