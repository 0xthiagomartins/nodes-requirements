export interface Node {
    id: number;
    blockchain_type: string;
    cpu_cores: number;
    ram_gb: number;
    storage_gb: number;
    network_mbps: number;
    created_at: string;
    updated_at: string;
}

export interface PriceHistory {
    id: number;
    provider: string;
    node_id: number;
    price_usd: number;
    recorded_at: string;
} 