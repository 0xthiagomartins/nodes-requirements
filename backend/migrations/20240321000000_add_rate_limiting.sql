-- Add migration script here
ALTER TABLE api_keys
ADD COLUMN requests_per_minute INTEGER NOT NULL DEFAULT 60;

ALTER TABLE api_keys
ADD COLUMN requests_this_minute INTEGER NOT NULL DEFAULT 0;

ALTER TABLE api_keys
ADD COLUMN last_request_time DATETIME;

-- Add index for rate limiting queries
CREATE INDEX idx_api_keys_rate_limit ON api_keys(key, requests_this_minute, last_request_time)
WHERE deleted_at IS NULL AND is_active = TRUE; 