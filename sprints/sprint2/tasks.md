# Sprint 2 Tasks

## API Development ✅
- ✅ Implement POST /nodes endpoint
  - ✅ Validate request body
  - ✅ Handle duplicate blockchain types
  - ✅ Return created node
- ✅ Implement PUT /nodes/{id} endpoint
  - ✅ Validate request body
  - ✅ Handle non-existent nodes
  - ✅ Return updated node
- ✅ Implement DELETE /nodes/{id} endpoint
  - ✅ Handle cascade deletion with price history (now implemented with ON DELETE CASCADE)
  - ✅ Return success status

## Price History Integration 🟡
- ✅ Create price fetcher service
  - ✅ Implement GCP price fetching
    - ✅ Basic structure implemented
    - ✅ Price calculation logic
    - ✅ Pagination handling for SKUs
      - ✅ Implement memory-efficient pagination
      - ✅ Filter results across all pages
      - ✅ Add retry mechanism for failed requests
        - ✅ Implement basic retry with fixed delay
  - 🔴 Implement AWS price fetching (placeholder created)
- 🟡 Add scheduled price updates
  - 🟡 Set up background task system
    - 🔴 Create task scheduler
    - 🔴 Implement graceful shutdown
    - 🔴 Add error handling for tasks
  - 🔴 Configure update intervals
    - 🔴 Make interval configurable via env vars
    - 🔴 Add jitter to prevent thundering herd
- ✅ Create price history endpoints
  - ✅ GET /nodes/{id}/prices
  - ✅ GET /nodes/{id}/prices/latest
  - ✅ POST /nodes/{id}/prices

## Database ✅
- ✅ Create price history table migration
  - ✅ Define table schema
  - ✅ Add foreign key relationship
- 🔴 Create API keys table migration
- 🟡 Implement database models
  - ✅ Price history models
  - 🔴 API key models
- ✅ Add database connection pooling

## Testing & Documentation ✅
- ✅ Write tests for POST /nodes
- ✅ Write tests for PUT /nodes
- ✅ Write tests for DELETE /nodes
- ✅ Write tests for price history endpoints
  - ✅ POST /nodes/{id}/prices
  - ✅ GET /nodes/{id}/prices
  - ✅ GET /nodes/{id}/prices/latest
- ✅ Organize test structure
  - ✅ Move integration tests to /tests
  - ✅ Create common test utilities
- 🔴 Create Insomnia collection for API testing
- 🔴 Document API endpoints
- 🔴 Add example requests and responses

## Authentication & Security 🔴
- 🔴 Create API keys table
- 🔴 Implement API key middleware
- 🔴 Add rate limiting
- 🔴 Create key management endpoints
  - 🔴 POST /api-keys
  - 🔴 DELETE /api-keys/{id}

## Error Handling 🟡
- ✅ Create custom error types
- ✅ Implement error middleware
- ✅ Add request validation
- 🔴 Improve error responses 