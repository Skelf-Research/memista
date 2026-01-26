#!/bin/bash

# Simple test script for Memista
# Verifies basic functionality

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}=== Memista Functionality Test ===${NC}"
echo

# Check if Memista is running
echo -e "${YELLOW}Checking if Memista is running...${NC}"
if curl -s -f -o /dev/null http://localhost:8083/openapi.json; then
  echo -e "${GREEN}✓ Memista is running${NC}"
else
  echo -e "${RED}✗ Memista is not running on http://localhost:8083${NC}"
  echo -e "${YELLOW}Please start Memista with: cargo run${NC}"
  exit 1
fi

# Test database ID
TEST_DB="test_db_$(date +%s)"

# Test 1: Drop any existing test database
echo -e "${YELLOW}Test 1: Cleaning up existing test database...${NC}"
RESPONSE=$(curl -s -w "%{http_code}" -o /tmp/test_response.json -X DELETE \
  -H "Content-Type: application/json" \
  -d "{\"database_id\": \"$TEST_DB\"}" \
  http://localhost:8083/v1/drop)

if [ "$RESPONSE" == "200" ]; then
  echo -e "${GREEN}✓ Database cleanup successful${NC}"
else
  echo -e "${RED}✗ Database cleanup failed with HTTP $RESPONSE${NC}"
fi

# Test 2: Insert chunks
echo -e "${YELLOW}Test 2: Inserting test chunks...${NC}"
INSERT_DATA="{\"database_id\": \"$TEST_DB\", \"chunks\": [{\"embedding\": [0.1, 0.2], \"text\": \"Test chunk 1\", \"metadata\": \"{\\\"source\\\": \\\"test\\\"}\"}, {\"embedding\": [0.3, 0.4], \"text\": \"Test chunk 2\", \"metadata\": \"{\\\"source\\\": \\\"test\\\"}\"}]}"
echo "$INSERT_DATA" > /tmp/insert_data.json

RESPONSE=$(curl -s -w "%{http_code}" -o /tmp/insert_response.json -X POST \
  -H "Content-Type: application/json" \
  -d @/tmp/insert_data.json \
  http://localhost:8083/v1/insert)

if [ "$RESPONSE" == "200" ]; then
  echo -e "${GREEN}✓ Chunk insertion successful${NC}"
  echo -e "${YELLOW}Insert response: $(cat /tmp/insert_response.json)${NC}"
else
  echo -e "${RED}✗ Chunk insertion failed with HTTP $RESPONSE${NC}"
  cat /tmp/insert_response.json
fi

# Test 3: Search chunks
echo -e "${YELLOW}Test 3: Searching for chunks...${NC}"
SEARCH_DATA="{\"database_id\": \"$TEST_DB\", \"embeddings\": [[0.1, 0.2]], \"num_results\": 5}"
echo "$SEARCH_DATA" > /tmp/search_data.json

RESPONSE=$(curl -s -w "%{http_code}" -o /tmp/search_response.json -X POST \
  -H "Content-Type: application/json" \
  -d @/tmp/search_data.json \
  http://localhost:8083/v1/search)

if [ "$RESPONSE" == "200" ]; then
  echo -e "${GREEN}✓ Chunk search successful${NC}"
  echo -e "${YELLOW}Search response: $(cat /tmp/search_response.json)${NC}"
else
  echo -e "${RED}✗ Chunk search failed with HTTP $RESPONSE${NC}"
  cat /tmp/search_response.json
fi

# Test 4: Drop database
echo -e "${YELLOW}Test 4: Dropping test database...${NC}"
RESPONSE=$(curl -s -w "%{http_code}" -o /tmp/drop_response.json -X DELETE \
  -H "Content-Type: application/json" \
  -d "{\"database_id\": \"$TEST_DB\"}" \
  http://localhost:8083/v1/drop)

if [ "$RESPONSE" == "200" ]; then
  echo -e "${GREEN}✓ Database drop successful${NC}"
  echo -e "${YELLOW}Drop response: $(cat /tmp/drop_response.json)${NC}"
else
  echo -e "${RED}✗ Database drop failed with HTTP $RESPONSE${NC}"
  cat /tmp/drop_response.json
fi

# Cleanup
rm -f /tmp/test_response.json /tmp/insert_data.json /tmp/insert_response.json \
      /tmp/search_data.json /tmp/search_response.json /tmp/drop_response.json

echo
echo -e "${GREEN}=== Test completed ===${NC}"