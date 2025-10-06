CREATE TABLE IF NOT EXISTS stickers
(
    id         SERIAL PRIMARY KEY,
    name       TEXT             NOT NULL,
    latitude   DOUBLE PRECISION NOT NULL,
    longitude  DOUBLE PRECISION NOT NULL,
    place_name TEXT             NOT NULL,
    pictures   JSONB            NOT NULL DEFAULT '[]', -- JSON array of picture URLs
    created_at TIMESTAMP                 DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP                 DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_stickers_name ON stickers (name);
CREATE INDEX IF NOT EXISTS idx_stickers_created_at ON stickers (created_at);

-- Trigger to update the updated_at column on row update (function already created in base_table migration)
CREATE TRIGGER update_stickers_updated_at
    BEFORE UPDATE
    ON stickers
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();