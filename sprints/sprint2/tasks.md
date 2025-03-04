# Sprint 2 Tasks

## API Development
- ✅ Implement POST /nodes endpoint
  - ✅ Validate request body
  - ✅ Handle duplicate blockchain types
  - ✅ Return created node
- ✅ Implement PUT /nodes/{id} endpoint
  - ✅ Validate request body
  - ✅ Handle non-existent nodes
  - ✅ Return updated node
- 🟡 Implement DELETE /nodes/{id} endpoint
  - 🟡 Handle cascade deletion with price history (pending price history table)
  - ✅ Return success status

## Price History Integration
- 🔴 Create price fetcher service
  - Implement GCP price fetching
  - Implement Hetzner price fetching
- 🔴 Add scheduled price updates
  - Set up background task system
  - Configure update intervals
- 🔴 Create price history endpoints
  - GET /nodes/{id}/prices
  - GET /nodes/{id}/prices/latest

## Authentication & Security
- 🔴 Create API keys table
- 🔴 Implement API key middleware
- 🔴 Add rate limiting
- 🔴 Create key management endpoints
  - POST /api-keys
  - DELETE /api-keys/{id}

## Error Handling
- 🟢 Create custom error types
- 🟢 Implement error middleware
- 🟡 Add request validation
- 🔴 Improve error responses

## Database
- 🔴 Create price history table migration
- 🔴 Create API keys table migration
- 🔴 Implement database models
- 🔴 Add database connection pooling

## Testing & Documentation
- ✅ Write tests for POST /nodes
- ✅ Write tests for PUT /nodes
- ✅ Write tests for DELETE /nodes
- 🟢 Organize test structure
  - ✅ Move integration tests to /tests
  - ✅ Create common test utilities
- 🔴 Create Insomnia collection for API testing
- 🔴 Document API endpoints
- 🔴 Add example requests and responses 