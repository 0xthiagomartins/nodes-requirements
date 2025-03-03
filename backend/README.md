# Miner Node Comparison - Backend

Backend service for the Miner Node Hardware Comparison application, built with Rust, Actix Web, and SQLite.

## Prerequisites

- Rust (latest stable version)
- SQLite
- pkg-config
- OpenSSL development files

On Ubuntu/Debian:
```bash
sudo apt update
sudo apt install pkg-config libssl-dev sqlite3 libsqlite3-dev
```

## Setup

1. Install SQLx CLI:
```bash
cargo install sqlx-cli
```

2. Set up environment:
```bash
# Run the setup script from project root
../scripts/setup.sh

# Or manually:
mkdir -p db
echo "DATABASE_URL=sqlite:db/app.db" > .env
sqlx database create
sqlx migrate run
```

## Development

### Running the Server

```bash
# Run in development mode
cargo run

# Run with hot reload (requires cargo-watch)
cargo install cargo-watch
cargo watch -x run
```

### Database Management

```bash
# Create a new migration
sqlx migrate add <migration_name>

# Run pending migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

## Project Structure

```
backend/
├── src/
│   ├── main.rs          # Application entry point
│   ├── routes/          # API route handlers
│   ├── models/          # Data models and schemas
│   ├── db/              # Database interactions
│   └── error/           # Error handling
├── migrations/          # Database migrations
├── db/                  # SQLite database files
├── Cargo.toml          # Dependencies and project metadata
└── .env                # Environment variables
```

## API Endpoints

- `GET /nodes` - List all nodes
- `GET /nodes/{id}` - Get specific node
- `POST /nodes` - Create new node
- `PUT /nodes/{id}` - Update node
- `DELETE /nodes/{id}` - Delete node

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test <test_name>

# Run with logging
RUST_LOG=debug cargo test
```

## Environment Variables

- `DATABASE_URL`: SQLite database URL (default: `sqlite:db/app.db`)
- `RUST_LOG`: Logging level (e.g., `debug`, `info`, `warn`)

## Contributing

1. Ensure you have run `cargo fmt` and `cargo clippy`
2. Update tests if needed
3. Follow the existing code style
4. Update documentation for any new features

## Common Issues

### SQLx CLI Installation Fails
Make sure you have pkg-config and OpenSSL development files installed:
```bash
sudo apt install pkg-config libssl-dev
```

### Database Connection Fails
Verify that:
1. The `db` directory exists
2. The `.env` file is present with correct DATABASE_URL
3. You have run `sqlx migrate run` 