CREATE TABLE IF NOT EXISTS sources
(
    id
        INTEGER
        PRIMARY
            KEY
        AUTOINCREMENT,
    NAME
        TEXT
        NOT
            NULL
        UNIQUE,
    COUNT
        INTEGER
        NOT
            NULL
        DEFAULT
            0,
    created_at
        DATETIME
        DEFAULT
            CURRENT_TIMESTAMP,
    updated_at
        DATETIME
        DEFAULT
            CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_sources_name ON sources (NAME);