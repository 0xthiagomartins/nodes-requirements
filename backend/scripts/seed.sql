-- Clear existing data
DELETE FROM price_history;
DELETE FROM nodes;

-- Reset auto-increment
DELETE FROM sqlite_sequence WHERE name='nodes';
DELETE FROM sqlite_sequence WHERE name='price_history';

-- Insert test nodes
INSERT INTO nodes (blockchain_type, cpu_cores, ram_gb, storage_gb, network_mbps) VALUES
    ('ethereum', 8, 32, 2000, 1000),
    ('bitcoin', 4, 16, 1000, 500),
    ('solana', 16, 128, 4000, 2000),
    ('cardano', 8, 64, 1500, 1000);

-- Insert test price history
INSERT INTO price_history (provider, node_id, price_usd, recorded_at) VALUES
    ('aws', 1, 150.00, datetime('now', '-7 days')),
    ('aws', 1, 145.00, datetime('now')),
    ('gcp', 1, 155.00, datetime('now')),
    ('azure', 1, 160.00, datetime('now')),
    
    ('aws', 2, 80.00, datetime('now', '-7 days')),
    ('aws', 2, 85.00, datetime('now')),
    ('gcp', 2, 82.00, datetime('now')),
    ('azure', 2, 88.00, datetime('now')),
    
    ('aws', 3, 300.00, datetime('now', '-7 days')),
    ('aws', 3, 295.00, datetime('now')),
    ('gcp', 3, 310.00, datetime('now')),
    ('azure', 3, 305.00, datetime('now')),
    
    ('aws', 4, 200.00, datetime('now', '-7 days')),
    ('aws', 4, 195.00, datetime('now')),
    ('gcp', 4, 205.00, datetime('now')),
    ('azure', 4, 198.00, datetime('now')); 