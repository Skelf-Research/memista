#!/bin/bash

# Benchmark Runner for Memista
# Runs all benchmark scripts sequentially

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}==========================================${NC}"
echo -e "${BLUE}    Memista Complete Benchmark Suite      ${NC}"
echo -e "${BLUE}==========================================${NC}"
echo

# Check if required tools are installed
echo -e "${YELLOW}Checking prerequisites...${NC}"

# Check if bc is installed
if ! command -v bc &> /dev/null; then
  echo -e "${RED}Error: bc is not installed. Please install bc.${NC}"
  exit 1
fi

# Check if Memista is running
echo -e "${YELLOW}Checking if Memista is running...${NC}"
if ! curl -s -f -o /dev/null http://localhost:8083/openapi.json; then
  echo -e "${RED}Error: Memista is not running on http://localhost:8083${NC}"
  echo -e "${YELLOW}Please start Memista with: cargo run${NC}"
  exit 1
fi

echo -e "${GREEN}All prerequisites satisfied!${NC}"
echo

# Run insert benchmark
echo -e "${BLUE}==========================================${NC}"
echo -e "${BLUE}        Running Insert Benchmark          ${NC}"
echo -e "${BLUE}==========================================${NC}"
./insert_benchmark.sh
echo

# Run search benchmark
echo -e "${BLUE}==========================================${NC}"
echo -e "${BLUE}        Running Search Benchmark          ${NC}"
echo -e "${BLUE}==========================================${NC}"
./search_benchmark.sh
echo

# Run concurrent benchmark
echo -e "${BLUE}==========================================${NC}"
echo -e "${BLUE}      Running Concurrent Benchmark        ${NC}"
echo -e "${BLUE}==========================================${NC}"
./concurrent_benchmark.sh
echo

echo -e "${GREEN}==========================================${NC}"
echo -e "${GREEN}      All benchmarks completed!           ${NC}"
echo -e "${GREEN}==========================================${NC}"