CREATE TABLE IF NOT EXISTS stickers
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    latitude REAL NOT NULL,
    longitude REAL NOT NULL,
    place_name TEXT NOT NULL,
    pictures TEXT NOT NULL DEFAULT '[]', -- JSON array of picture URLs
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_stickers_name ON stickers (name);
CREATE INDEX IF NOT EXISTS idx_stickers_created_at ON stickers (created_at);
