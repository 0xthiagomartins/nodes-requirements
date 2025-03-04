# Insomnia API Collection

This folder contains the Insomnia collection for testing both our backend API and the GCP Pricing API.

## Setup Instructions

1. Install Insomnia
   - Download and install [Insomnia](https://insomnia.rest/download)

2. Import Collection
   - Open Insomnia
   - Click on `Create` -> `Import from File`
   - Select `api_collection.json` from this folder

3. Configure Environment Variables
   - Click on `Manage Environments` (gear icon)
   - Create a new environment (e.g., "Development")
   - Add the following variables:
   ```json
   {
     "backend_url": "http://localhost:8080",
     "gcp_url": "https://cloudbilling.googleapis.com",
     "gcp_api_key": "your_gcp_api_key",
     "gcp_project_id": "your_gcp_project_id",
     "gcp_billing_account": "your_billing_account_id"
   }
   ```

## Collection Structure

The collection is organized into several folders:

### Backend API
- **Nodes**: CRUD operations for node management
  - GET /nodes
  - GET /nodes/{id}
  - POST /nodes
  - PUT /nodes/{id}
  - DELETE /nodes/{id}

- **Price History**: Price tracking endpoints
  - GET /nodes/{id}/prices
  - GET /nodes/{id}/prices/latest
  - POST /nodes/{id}/prices

### GCP Pricing API
- **Public Pricing API (v2beta)**
  - List Services
  - Get Service Details
  - List SKUs
  
- **Billing Account API (v1beta)**
  - List Billing Account SKUs
  - Get SKU Price
  - List SKU Groups
  - List SKUs in Group

## Usage Tips

1. **Environment Variables**
   - Make sure to replace placeholder values with your actual API keys
   - Keep sensitive data in environment variables, not in the requests

2. **Testing Flow**
   - Start with creating a node
   - Use the returned node ID for price-related operations
   - Test GCP pricing API endpoints to fetch current prices

3. **Common Parameters**
   - `pageSize`: Control number of results (default: 50)
   - `filter`: Filter results (e.g., by service ID)
   - `currencyCode`: Specify currency for prices (default: USD)

## Troubleshooting

1. **401 Unauthorized**
   - Check if your GCP API key is correct
   - Verify the API key is properly set in environment variables

2. **404 Not Found**
   - For backend: Ensure the server is running at the correct URL
   - For GCP: Verify service IDs and SKU IDs exist

3. **400 Bad Request**
   - Check request body format for POST/PUT requests
   - Verify required parameters are included

## Updating the Collection

When making changes to the API:
1. Update the collection in Insomnia
2. Export the collection
3. Replace `api_collection.json` in this folder
4. Update this README if needed
5. Commit changes to the repository

## Related Documentation

- [GCP Pricing API Documentation](https://cloud.google.com/billing/docs/reference/pricing-api/rest)
- [GCP Getting Started with Pricing API](https://cloud.google.com/billing/docs/how-to/get-pricing-information-api)
- [Backend API Documentation](../api-docs.yaml) 