#!/bin/bash

# Memista Development Setup Script
# This script sets up the development environment for Memista

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}=== Memista Development Setup ===${NC}"
echo

# Check for Rust
echo -e "${YELLOW}Checking Rust installation...${NC}"
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo -e "${GREEN}✓ Rust is installed: $RUST_VERSION${NC}"
else
    echo -e "${RED}✗ Rust is not installed${NC}"
    echo -e "${YELLOW}Install Rust from https://rustup.rs/${NC}"
    exit 1
fi

# Check for Cargo
echo -e "${YELLOW}Checking Cargo installation...${NC}"
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    echo -e "${GREEN}✓ Cargo is installed: $CARGO_VERSION${NC}"
else
    echo -e "${RED}✗ Cargo is not installed${NC}"
    exit 1
fi

# Check for CMake (required for usearch)
echo -e "${YELLOW}Checking CMake installation...${NC}"
if command -v cmake &> /dev/null; then
    CMAKE_VERSION=$(cmake --version | head -n1)
    echo -e "${GREEN}✓ CMake is installed: $CMAKE_VERSION${NC}"
else
    echo -e "${RED}✗ CMake is not installed${NC}"
    echo -e "${YELLOW}Install CMake:${NC}"
    echo "  - macOS: brew install cmake"
    echo "  - Ubuntu/Debian: sudo apt-get install cmake"
    exit 1
fi

# Navigate to project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

echo
echo -e "${YELLOW}Setting up project...${NC}"

# Create .env file if it doesn't exist
if [ ! -f ".env" ]; then
    echo -e "${YELLOW}Creating .env file from config/sample.env...${NC}"
    cp config/sample.env .env
    echo -e "${GREEN}✓ Created .env file${NC}"
else
    echo -e "${GREEN}✓ .env file already exists${NC}"
fi

# Build the project
echo
echo -e "${YELLOW}Building project...${NC}"
cargo build

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build failed${NC}"
    exit 1
fi

# Run tests
echo
echo -e "${YELLOW}Running tests...${NC}"
cargo test

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Tests passed${NC}"
else
    echo -e "${RED}✗ Tests failed${NC}"
    exit 1
fi

echo
echo -e "${GREEN}=== Setup Complete ===${NC}"
echo
echo "Next steps:"
echo "  1. Start the server: cargo run"
echo "  2. Test the API: curl http://localhost:8083/openapi.json"
echo "  3. View docs: cd documentation && mkdocs serve"
echo
