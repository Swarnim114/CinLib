-- UUIDs → TEXT, booleans → INTEGER (0/1), datetimes → ISO-8601 TEXT

-- ── Libraries ──────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS libraries (
    id          TEXT    NOT NULL PRIMARY KEY,
    name        TEXT    NOT NULL,
    path        TEXT    NOT NULL UNIQUE,
    kind        TEXT    NOT NULL CHECK(kind IN ('movies', 'series', 'mixed')),
    created_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- ── Movies ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS movies (
    id               TEXT     NOT NULL PRIMARY KEY,
    library_id       TEXT     NOT NULL REFERENCES libraries(id) ON DELETE CASCADE,
    title            TEXT     NOT NULL,
    year             INTEGER,
    file_path        TEXT     NOT NULL UNIQUE,
    file_size        INTEGER  NOT NULL DEFAULT 0,
    duration_secs    INTEGER,
    tmdb_id          INTEGER,
    overview         TEXT,
    poster_path      TEXT,
    backdrop_path    TEXT,
    rating           REAL,
    genres           TEXT     NOT NULL DEFAULT '[]',  -- JSON array of strings
    is_favorite      INTEGER  NOT NULL DEFAULT 0,
    watch_position   INTEGER  NOT NULL DEFAULT 0,     -- seconds watched
    watch_completed  INTEGER  NOT NULL DEFAULT 0,
    added_at         TEXT     NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at       TEXT     NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- ── Series ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS series (
    id               TEXT     NOT NULL PRIMARY KEY,
    library_id       TEXT     NOT NULL REFERENCES libraries(id) ON DELETE CASCADE,
    title            TEXT     NOT NULL,
    year             INTEGER,
    folder_path      TEXT     NOT NULL UNIQUE,
    tmdb_id          INTEGER,
    overview         TEXT,
    poster_path      TEXT,
    backdrop_path    TEXT,
    rating           REAL,
    genres           TEXT     NOT NULL DEFAULT '[]',
    status           TEXT,   -- e.g. 'Ended', 'Returning Series'
    total_episodes   INTEGER  NOT NULL DEFAULT 0,
    is_favorite      INTEGER  NOT NULL DEFAULT 0,
    added_at         TEXT     NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at       TEXT     NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- ── Episodes ───────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS episodes (
    id               TEXT     NOT NULL PRIMARY KEY,
    series_id        TEXT     NOT NULL REFERENCES series(id) ON DELETE CASCADE,
    title            TEXT,
    season_number    INTEGER  NOT NULL,
    episode_number   INTEGER  NOT NULL,
    file_path        TEXT     NOT NULL UNIQUE,
    file_size        INTEGER  NOT NULL DEFAULT 0,
    duration_secs    INTEGER,
    overview         TEXT,
    still_path       TEXT,
    watch_position   INTEGER  NOT NULL DEFAULT 0,
    watch_completed  INTEGER  NOT NULL DEFAULT 0,
    air_date         TEXT,
    added_at         TEXT     NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(series_id, season_number, episode_number)
);

-- ── Indices ────────────────────────────────────────────────────────────────
-- By library — primary list queries.
CREATE INDEX IF NOT EXISTS idx_movies_library  ON movies(library_id);
CREATE INDEX IF NOT EXISTS idx_series_library  ON series(library_id);
CREATE INDEX IF NOT EXISTS idx_episodes_series ON episodes(series_id);

-- Playback order for episodes.
CREATE INDEX IF NOT EXISTS idx_episodes_order
    ON episodes(series_id, season_number, episode_number);

-- Partial indexes — only index rows that match the predicate.
-- Favourites shelf.
CREATE INDEX IF NOT EXISTS idx_movies_fav
    ON movies(library_id, added_at)
    WHERE is_favorite = 1;

CREATE INDEX IF NOT EXISTS idx_series_fav
    ON series(library_id, added_at)
    WHERE is_favorite = 1;

-- Continue Watching shelf — in-progress, not finished.
CREATE INDEX IF NOT EXISTS idx_movies_progress
    ON movies(updated_at)
    WHERE watch_position > 0 AND watch_completed = 0;

CREATE INDEX IF NOT EXISTS idx_episodes_progress
    ON episodes(series_id, season_number, episode_number)
    WHERE watch_position > 0 AND watch_completed = 0;
