-- Add migration script here
CREATE TABLE api_keys (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used_at DATETIME,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    deleted_at DATETIME -- For soft delete support
);

-- Add index for fast key lookups
CREATE INDEX idx_api_keys_key ON api_keys(key) WHERE deleted_at IS NULL AND is_active = TRUE; 