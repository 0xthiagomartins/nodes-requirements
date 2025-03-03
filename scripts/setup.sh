#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Setting up development environment...${NC}"

# Create database directory if it doesn't exist
if [ ! -d "backend/db" ]; then
    echo -e "${YELLOW}Creating database directory...${NC}"
    mkdir -p backend/db
    echo -e "${GREEN}Database directory created!${NC}"
fi

# Create .env file if it doesn't exist
if [ ! -f "backend/.env" ]; then
    echo -e "${YELLOW}Creating .env file...${NC}"
    echo "DATABASE_URL=sqlite:db/app.db" > backend/.env
    echo -e "${GREEN}.env file created!${NC}"
fi

# Check if sqlx-cli is installed
if ! command -v sqlx &> /dev/null; then
    echo -e "${YELLOW}Installing sqlx-cli...${NC}"
    cargo install sqlx-cli
    echo -e "${GREEN}sqlx-cli installed!${NC}"
fi

# Create database if it doesn't exist
if [ ! -f "backend/db/app.db" ]; then
    echo -e "${YELLOW}Creating database...${NC}"
    cd backend && sqlx database create
    echo -e "${GREEN}Database created!${NC}"
fi

echo -e "${GREEN}Setup completed successfully!${NC}" 