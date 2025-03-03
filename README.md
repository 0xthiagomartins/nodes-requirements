# Miner Node Hardware Comparison App

## Introduction
A comprehensive tool for comparing hardware requirements across different blockchain miner nodes (Ethereum, Solana, Cardano, and Bitcoin). This application helps users evaluate and compare the hardware specifications needed to run various blockchain nodes, along with associated cloud hosting costs.

## Project Structure

```
├── backend/ # Rust backend application
│ ├── src/ # Source code
│ ├── migrations/ # Database migrations
│ └── tests/ # Backend tests
├── frontend/ # Next.js frontend application
│ ├── src/ # Source code
│ ├── components/ # Reusable UI components
│ └── tests/ # Frontend tests
└── sprints/ # Sprint planning and tracking
├── sprint1/ # Current sprint
├── sprint2/ # Next sprint
└── docs/ # Sprint documentation
```

## Directory Overview
- **backend/**: Rust-based backend using Actix Web and SQLite, handling data management and API endpoints
- **frontend/**: Next.js application with Tailwind CSS and Material Design, providing the user interface
- **sprints/**: Project management directory containing sprint plans, tasks, and progress tracking

---

# Development Roadmap

## **Phase 1: Planning & Research**
### **1. Define Data Structure**
- Identify key attributes for each miner node (Ethereum, Solana, Cardano, Bitcoin).
- Design the SQLite database schema.
- Determine how to retrieve hardware requirement updates (manual entry, API, web scraping).

### **2. Choose Tech Stack**
- **Backend**: Rust (e.g., Actix Web, Axum, or Rocket) with SQLite.
- **Frontend**: Next.js with Tailwind CSS and Material Design (e.g., MUI or shadcn/ui).
- **API Layer**: REST or GraphQL.
- **Cloud Pricing Data**: API scraping or manual data entry.

---

## **Phase 2: Backend Development**
### **1. Setup Rust Project**
- Initialize Rust project.
- Set up Actix Web (or another Rust web framework).
- Integrate SQLite with Diesel or SQLx.

### **2. Build API Endpoints**
- **CRUD for miner node data**:
  - `GET /nodes` – Fetch all nodes.
  - `GET /nodes/{id}` – Fetch details of a specific node.
  - `POST /nodes` – Add a new node.
  - `PUT /nodes/{id}` – Update a node.
  - `DELETE /nodes/{id}` – Remove a node.

- **Price Comparison API**:
  - Fetch hardware costs from GCP & Hetzner.
  - Store historical price data for tracking.

- **Authentication (Optional)**
  - API key-based access control or JWT.

---

## **Phase 3: Frontend Development**
### **1. Setup Next.js & Tailwind**
- Initialize Next.js project.
- Configure Tailwind CSS & Material Design (e.g., MUI or shadcn/ui).
- Create global styles & layout.

### **2. Build UI Components**
- **Dashboard Page**: List all miner nodes.
- **Node Details Page**: Show hardware requirements, cost comparisons.
- **Price Tracker Page**: Historical pricing for different cloud providers.
- **Admin Panel (Optional)**: Manage nodes and pricing data.

### **3. API Integration**
- Connect frontend with Rust backend.
- Implement dynamic data fetching with React hooks.

---

## **Phase 4: Testing & Deployment**
### **1. Backend Testing**
- Unit tests for Rust API.
- Database integration tests.
- API testing with Insomnia for endpoint validation and documentation.

### **2. Frontend Testing**
- Jest & React Testing Library for UI tests.
- Cypress for end-to-end testing.

### **3. Deployment**
- Deploy backend on a cloud provider (e.g., Hetzner, GCP, or Fly.io).
- Deploy frontend on Vercel (or GCP Cloud Run).

---

## **Phase 5: Enhancements & Future Features**
- **Live Syncing**: Automatically update hardware requirements.
- **User Accounts**: Allow users to save preferred configurations.
- **Custom Pricing Models**: Users can input their own cloud provider costs.

