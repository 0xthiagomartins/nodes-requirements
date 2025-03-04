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