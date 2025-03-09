-- Drop the rate limiting columns
DROP INDEX IF EXISTS idx_api_keys_rate_limit;

-- SQLite doesn't support DROP COLUMN in a single statement
CREATE TABLE api_keys_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used_at DATETIME,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    deleted_at DATETIME
);

INSERT INTO api_keys_new 
SELECT id, key, name, created_at, last_used_at, is_active, deleted_at
FROM api_keys;

DROP TABLE api_keys;
ALTER TABLE api_keys_new RENAME TO api_keys;

-- Recreate the original index
CREATE INDEX idx_api_keys_key ON api_keys(key) 
WHERE deleted_at IS NULL AND is_active = TRUE; 