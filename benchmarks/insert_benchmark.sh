#!/bin/bash

# Insert Benchmark for Memista
# Measures the performance of inserting chunks into Memista

# Default values
DATABASE_ID=${1:-"benchmark_db"}
NUM_CHUNKS=${2:-1000}
CONCURRENCY=${3:-10}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}=== Memista Insert Benchmark ===${NC}"
echo "Database ID: $DATABASE_ID"
echo "Number of chunks: $NUM_CHUNKS"
echo "Concurrency: $CONCURRENCY"
echo

# Function to generate random embedding
generate_embedding() {
  echo "[0.1, 0.2]" # Simplified for benchmarking
}

# Function to generate random text
generate_text() {
  cat /dev/urandom | env LC_CTYPE=C tr -dc 'a-zA-Z0-9' | fold -w 32 | head -n 1
}

# Function to generate random metadata
generate_metadata() {
  echo "{\"source\": \"benchmark\", \"timestamp\": $(date +%s)}"
}

# Function to create a chunk
create_chunk() {
  local i=$1
  local embedding=$(generate_embedding)
  local text=$(generate_text)
  local metadata=$(generate_metadata)
  
  echo "{\"embedding\": $embedding, \"text\": \"$text\", \"metadata\": \"$metadata\"}"
}

# Function to create chunks array
create_chunks_array() {
  local num_chunks=$1
  local chunks="["
  
  for ((i=0; i<num_chunks; i++)); do
    if [ $i -gt 0 ]; then
      chunks="$chunks,"
    fi
    chunks="$chunks$(create_chunk $i)"
  done
  
  chunks="$chunks]"
  echo "$chunks"
}

# Create test data
echo -e "${YELLOW}Preparing test data...${NC}"
CHUNKS_ARRAY=$(create_chunks_array $NUM_CHUNKS)

# Create JSON payload
JSON_PAYLOAD="{\"database_id\": \"$DATABASE_ID\", \"chunks\": $CHUNKS_ARRAY}"

# Save payload to temporary file
TEMP_FILE=$(mktemp)
echo "$JSON_PAYLOAD" > "$TEMP_FILE"

# Check if Memista is running
echo -e "${YELLOW}Checking if Memista is running...${NC}"
if ! curl -s -f -o /dev/null http://localhost:8083/openapi.json; then
  echo -e "${RED}Error: Memista is not running on http://localhost:8083${NC}"
  rm "$TEMP_FILE"
  exit 1
fi

# Drop existing database (if any)
echo -e "${YELLOW}Cleaning up existing database...${NC}"
curl -s -X DELETE -H "Content-Type: application/json" \
  -d "{\"database_id\": \"$DATABASE_ID\"}" \
  http://localhost:8083/v1/drop > /dev/null

# Run benchmark with wrk
echo -e "${YELLOW}Running insert benchmark...${NC}"
echo "$JSON_PAYLOAD" > /tmp/memista_insert_payload.json

# Use curl to measure single request time
START_TIME=$(date +%s.%N)
RESPONSE=$(curl -s -w "%{time_total}" -o /tmp/memista_response.json -X POST \
  -H "Content-Type: application/json" \
  -d @"$TEMP_FILE" \
  http://localhost:8083/v1/insert)
END_TIME=$(date +%s.%N)

# Extract time from response
REQUEST_TIME=$(echo "$RESPONSE" | tail -n1)

# Calculate requests per second
TOTAL_TIME=$(echo "$END_TIME - $START_TIME" | bc)
RPS=$(echo "scale=2; $NUM_CHUNKS / $TOTAL_TIME" | bc)

# Display results
echo
echo -e "${GREEN}=== Benchmark Results ===${NC}"
echo "Total chunks inserted: $NUM_CHUNKS"
echo "Total time: ${TOTAL_TIME}s"
echo "Requests per second: ${RPS}"
echo "Request time: ${REQUEST_TIME}s"
echo

# Cleanup
rm "$TEMP_FILE" /tmp/memista_insert_payload.json /tmp/memista_response.json

echo -e "${GREEN}Insert benchmark completed!${NC}"