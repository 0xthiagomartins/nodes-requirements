#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Setting up development environment...${NC}"

# Create database directory if it doesn't exist
if [ ! -d "../db" ]; then
    echo -e "${YELLOW}Creating database directory...${NC}"
    mkdir -p ../db
    echo -e "${GREEN}Database directory created!${NC}"
fi

# Create .env file if it doesn't exist
if [ ! -f "../.env" ]; then
    echo -e "${YELLOW}Creating .env file...${NC}"
    echo "DATABASE_URL=sqlite:db/app.db" > ../.env
    echo -e "${GREEN}.env file created!${NC}"
fi

# Check if sqlx-cli is installed
if ! command -v sqlx &> /dev/null; then
    echo -e "${YELLOW}Installing sqlx-cli...${NC}"
    cargo install sqlx-cli
    echo -e "${GREEN}sqlx-cli installed!${NC}"
fi

# Create database if it doesn't exist
if [ ! -f "db/app.db" ]; then
    echo -e "${YELLOW}Creating database...${NC}"
    sqlx database create
    echo -e "${GREEN}Database created!${NC}"

    echo -e "${YELLOW}Running database migrations...${NC}"
    sqlx migrate run
    echo -e "${GREEN}Migrations completed!${NC}"
else
    echo -e "${YELLOW}Checking for pending migrations...${NC}"
    sqlx migrate run
    echo -e "${GREEN}Migrations up to date!${NC}"
fi

# Seed database with test data if requested
if [ "$1" = "--with-test-data" ]; then
    echo -e "${YELLOW}Seeding database with test data...${NC}"
    sqlite3 db/app.db < scripts/seed.sql
    echo -e "${GREEN}Database seeded!${NC}"
fi

echo -e "${GREEN}Setup completed successfully!${NC}" 