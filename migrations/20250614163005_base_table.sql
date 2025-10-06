CREATE TABLE IF NOT EXISTS sources
(
    id
        SERIAL
        PRIMARY
            KEY,
    name
        TEXT
        NOT
            NULL
        UNIQUE,
    count
        INTEGER
        NOT
            NULL
        DEFAULT
            0,
    created_at
        TIMESTAMP
        DEFAULT
            CURRENT_TIMESTAMP,
    updated_at
        TIMESTAMP
        DEFAULT
            CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_sources_name ON sources (name);
CREATE INDEX IF NOT EXISTS idx_sources_created_at ON sources (created_at);

-- Trigger to update the updated_at column on row update
CREATE OR REPLACE FUNCTION update_updated_at_column()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_sources_updated_at
    BEFORE UPDATE
    ON sources
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();