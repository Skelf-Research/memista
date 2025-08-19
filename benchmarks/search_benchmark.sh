#!/bin/bash

# Search Benchmark for Memista
# Measures the performance of searching for chunks in Memista

# Default values
DATABASE_ID=${1:-"benchmark_db"}
NUM_QUERIES=${2:-1000}
CONCURRENCY=${3:-10}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}=== Memista Search Benchmark ===${NC}"
echo "Database ID: $DATABASE_ID"
echo "Number of queries: $NUM_QUERIES"
echo "Concurrency: $CONCURRENCY"
echo

# Function to generate random embedding for search
generate_search_embedding() {
  echo "[0.1, 0.2]" # Simplified for benchmarking
}

# Function to create search payload
create_search_payload() {
  local num_queries=$1
  local embeddings="["
  
  for ((i=0; i<num_queries; i++)); do
    if [ $i -gt 0 ]; then
      embeddings="$embeddings,"
    fi
    embeddings="$embeddings$(generate_search_embedding)"
  done
  
  embeddings="$embeddings]"
  echo "{\"database_id\": \"$DATABASE_ID\", \"embeddings\": $embeddings, \"num_results\": 5}"
}

# Check if Memista is running
echo -e "${YELLOW}Checking if Memista is running...${NC}"
if ! curl -s -f -o /dev/null http://localhost:8083/openapi.json; then
  echo -e "${RED}Error: Memista is not running on http://localhost:8083${NC}"
  exit 1
fi

# Prepare test data by inserting some chunks first
echo -e "${YELLOW}Preparing test data (inserting chunks)...${NC}"

# Create a smaller dataset for search testing
NUM_CHUNKS=100
CHUNKS_ARRAY="["
for ((i=0; i<NUM_CHUNKS; i++)); do
  if [ $i -gt 0 ]; then
    CHUNKS_ARRAY="$CHUNKS_ARRAY,"
  fi
  CHUNKS_ARRAY="$CHUNKS_ARRAY{\"embedding\": [0.1, 0.2], \"text\": \"Sample text $i\", \"metadata\": \"{\\\"source\\\": \\\"benchmark\\\"}\"}"
done
CHUNKS_ARRAY="$CHUNKS_ARRAY]"

# Drop existing database
curl -s -X DELETE -H "Content-Type: application/json" \
  -d "{\"database_id\": \"$DATABASE_ID\"}" \
  http://localhost:8083/v1/drop > /dev/null

# Insert test chunks
INSERT_PAYLOAD="{\"database_id\": \"$DATABASE_ID\", \"chunks\": $CHUNKS_ARRAY}"
echo "$INSERT_PAYLOAD" > /tmp/memista_insert_payload.json
curl -s -X POST -H "Content-Type: application/json" \
  -d @/tmp/memista_insert_payload.json \
  http://localhost:8083/v1/insert > /dev/null

echo -e "${YELLOW}Running search benchmark...${NC}"

# Create search payload
SEARCH_PAYLOAD=$(create_search_payload $NUM_QUERIES)
echo "$SEARCH_PAYLOAD" > /tmp/memista_search_payload.json

# Measure search performance
START_TIME=$(date +%s.%N)
RESPONSE=$(curl -s -w "%{time_total}" -o /tmp/memista_search_response.json -X POST \
  -H "Content-Type: application/json" \
  -d @/tmp/memista_search_payload.json \
  http://localhost:8083/v1/search)
END_TIME=$(date +%s.%N)

# Extract time from response
REQUEST_TIME=$(echo "$RESPONSE" | tail -n1)

# Calculate requests per second
TOTAL_TIME=$(echo "$END_TIME - $START_TIME" | bc)
RPS=$(echo "scale=2; $NUM_QUERIES / $TOTAL_TIME" | bc)

# Display results
echo
echo -e "${GREEN}=== Benchmark Results ===${NC}"
echo "Total queries: $NUM_QUERIES"
echo "Total time: ${TOTAL_TIME}s"
echo "Requests per second: ${RPS}"
echo "Request time: ${REQUEST_TIME}s"
echo

# Cleanup
rm /tmp/memista_insert_payload.json /tmp/memista_search_payload.json /tmp/memista_search_response.json

echo -e "${GREEN}Search benchmark completed!${NC}"