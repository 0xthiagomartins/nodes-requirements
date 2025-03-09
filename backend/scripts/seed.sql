-- Clear existing data
DELETE FROM price_history;
DELETE FROM nodes;

-- Reset auto-increment
DELETE FROM sqlite_sequence WHERE name='nodes';
DELETE FROM sqlite_sequence WHERE name='price_history';

-- Insert test nodes
INSERT INTO nodes (blockchain_type, cpu_cores, ram_gb, storage_gb, network_mbps)
VALUES 
    ('ethereum', 4, 8, 500, 1000),
    ('bitcoin', 8, 16, 1000, 2000),
    ('polkadot', 2, 4, 250, 500);

-- Insert test price history
INSERT INTO price_history (node_id, provider, price_per_hour, currency, fetched_at)
VALUES 
    (1, 'gcp', 1.25, 'USD', datetime('now')),
    (1, 'aws', 1.50, 'USD', datetime('now')),
    (2, 'gcp', 2.50, 'USD', datetime('now')),
    (2, 'aws', 2.75, 'USD', datetime('now')),
    (3, 'gcp', 0.75, 'USD', datetime('now')),
    (3, 'aws', 0.85, 'USD', datetime('now'));

-- Insert test API keys
INSERT INTO api_keys (key, name, is_active)
VALUES 
    ('test-key-1', 'Development Key', TRUE),
    ('test-key-2', 'Testing Key', TRUE);

-- Add comments about Taraxa node types
-- Note: These comments are for documentation purposes
/*
  Taraxa Consensus Nodes:
  
  Lite Consensus Node:
  - Prunes historical state upon node restart
  - Most common type on the network
  - Community nodes often restart periodically
  - Functionally identical to full nodes except for state pruning
  
  Full Consensus Node:
  - Maintains complete historical state
  - Storage requirements grow continuously
  - Recommended when storage isn't a constraint
  - State DB grows rapidly during high transaction loads
*/ 