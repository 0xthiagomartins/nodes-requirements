-- Add migration script here
CREATE TABLE IF NOT EXISTS nodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    blockchain_type TEXT NOT NULL,
    cpu_cores INTEGER NOT NULL,
    ram_gb INTEGER NOT NULL,
    storage_gb INTEGER NOT NULL,
    network_mbps INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create price_history table
CREATE TABLE IF NOT EXISTS price_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    provider TEXT NOT NULL,
    node_id INTEGER NOT NULL,
    price_usd DECIMAL(10,2) NOT NULL,
    recorded_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(node_id) REFERENCES nodes(id)
); 