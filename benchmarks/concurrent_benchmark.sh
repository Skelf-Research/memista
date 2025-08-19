#!/bin/bash

# Concurrent Benchmark for Memista
# Measures the performance of concurrent insert and search operations

# Default values
DATABASE_ID=${1:-"concurrent_benchmark_db"}
NUM_OPERATIONS=${2:-1000}
CONCURRENCY=${3:-10}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}=== Memista Concurrent Benchmark ===${NC}"
echo "Database ID: $DATABASE_ID"
echo "Number of operations: $NUM_OPERATIONS"
echo "Concurrency: $CONCURRENCY"
echo

# Function to generate test chunk
generate_test_chunk() {
  echo "{\"embedding\": [0.1, 0.2], \"text\": \"Concurrent test text\", \"metadata\": \"{\\\"source\\\": \\\"concurrent_benchmark\\\"}\"}"
}

# Function to generate search embedding
generate_search_embedding() {
  echo "[0.1, 0.2]"
}

# Check if Memista is running
echo -e "${YELLOW}Checking if Memista is running...${NC}"
if ! curl -s -f -o /dev/null http://localhost:8083/openapi.json; then
  echo -e "${RED}Error: Memista is not running on http://localhost:8083${NC}"
  exit 1
fi

# Prepare by dropping existing database
echo -e "${YELLOW}Cleaning up existing database...${NC}"
curl -s -X DELETE -H "Content-Type: application/json" \
  -d "{\"database_id\": \"$DATABASE_ID\"}" \
  http://localhost:8083/v1/drop > /dev/null

# Create test data
echo -e "${YELLOW}Preparing test data...${NC}"
TEST_CHUNK=$(generate_test_chunk)
SEARCH_EMBEDDING=$(generate_search_embedding)

# Save payloads to files
INSERT_PAYLOAD="{\"database_id\": \"$DATABASE_ID\", \"chunks\": [$TEST_CHUNK]}"
echo "$INSERT_PAYLOAD" > /tmp/memista_concurrent_insert.json

SEARCH_PAYLOAD="{\"database_id\": \"$DATABASE_ID\", \"embeddings\": [$SEARCH_EMBEDDING], \"num_results\": 5}"
echo "$SEARCH_PAYLOAD" > /tmp/memista_concurrent_search.json

# Function to perform insert operation
perform_insert() {
  curl -s -X POST -H "Content-Type: application/json" \
    -d @/tmp/memista_concurrent_insert.json \
    http://localhost:8083/v1/insert > /dev/null
}

# Function to perform search operation
perform_search() {
  curl -s -X POST -H "Content-Type: application/json" \
    -d @/tmp/memista_concurrent_search.json \
    http://localhost:8083/v1/search > /dev/null
}

# Function to perform mixed operations
perform_operation() {
  local op_type=$((RANDOM % 2))
  if [ $op_type -eq 0 ]; then
    perform_insert
  else
    perform_search
  fi
}

# Export functions for GNU parallel
export -f perform_insert perform_search perform_operation
export DATABASE_ID

# Run concurrent benchmark
echo -e "${YELLOW}Running concurrent benchmark...${NC}"

# Measure concurrent performance using background processes
START_TIME=$(date +%s.%N)

# Launch concurrent operations
pids=()
for ((i=0; i<NUM_OPERATIONS; i++)); do
  perform_operation &
  pids+=($!)
  
  # Limit concurrency
  if [ ${#pids[@]} -ge $CONCURRENCY ]; then
    # Wait for all background processes to complete
    for pid in ${pids[@]}; do
      wait $pid
    done
    pids=()
  fi
done

# Wait for remaining processes
for pid in ${pids[@]}; do
  wait $pid
done

END_TIME=$(date +%s.%N)

# Calculate results
TOTAL_TIME=$(echo "$END_TIME - $START_TIME" | bc)
RPS=$(echo "scale=2; $NUM_OPERATIONS / $TOTAL_TIME" | bc)

# Display results
echo
echo -e "${GREEN}=== Benchmark Results ===${NC}"
echo "Total operations: $NUM_OPERATIONS"
echo "Concurrency level: $CONCURRENCY"
echo "Total time: ${TOTAL_TIME}s"
echo "Requests per second: ${RPS}"
echo

# Cleanup
rm /tmp/memista_concurrent_insert.json /tmp/memista_concurrent_search.json

echo -e "${GREEN}Concurrent benchmark completed!${NC}"