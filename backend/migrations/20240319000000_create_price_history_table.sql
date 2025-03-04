-- Add migration script here
CREATE TABLE IF NOT EXISTS price_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    node_id INTEGER NOT NULL,
    provider VARCHAR(50) NOT NULL,  -- 'gcp' or 'hetzner'
    price_per_hour DECIMAL(10, 4) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    fetched_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (node_id) REFERENCES nodes(id) ON DELETE CASCADE
);

-- Add indexes for common queries
CREATE INDEX idx_price_history_node_id ON price_history(node_id);
CREATE INDEX idx_price_history_fetched_at ON price_history(fetched_at);
CREATE INDEX idx_price_history_provider ON price_history(provider); 