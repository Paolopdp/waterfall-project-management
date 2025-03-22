# Todo Checklist for SOTA Open Source Resource Management Tool

This checklist covers all the major phases of the project—from initial setup to final deployment and maintenance. Each section is broken down into actionable tasks that can be checked off as you progress.

---

## 1. Project Setup & Configuration ✓

- [x] **Initialize Repository & Structure**
  - [x] Initialize a Git repository
  - [x] Create a `.gitignore` for Rust, Node.js, and Docker files
  - [x] Create a project structure:
    - [x] `/backend` (Rust project)
    - [x] `/frontend` (Next.js project)
  - [x] Create a basic `README.md` outlining the project overview and structure

---

## 2. Backend (Rust) Setup

### A. Base Setup & Health Check ✓

- [x] Initialize a new Rust project with Cargo in `/backend`
- [x] Set up a minimal web server using Actix-Web
- [x] Implement a basic health check endpoint
- [x] Write an initial test for the health check endpoint

### B. Database Integration & Data Models ✓

- [x] Integrate PostgreSQL using SQLx
  - [x] Define the **Project** data model with necessary fields
  - [x] Write database migrations to create corresponding tables
  - [x] Implement validation for Project model:
    - [x] Name validation (non-empty, max length)
    - [x] Date validation (end after start)
    - [x] Budget validation (non-negative)
- [x] Implement basic CRUD endpoints for the Project model:
  - [x] Create endpoint with validation
  - [x] Read endpoint (get by ID and get all)
  - [x] Update endpoint
  - [x] Delete endpoint
  - [x] Write unit tests for each CRUD operation
- [x] Define the **User** data model with necessary fields
  - [x] Create User model with validation
  - [x] Add user roles enum
  - [x] Create database migration
  - [x] Implement user service with password hashing

### C. Authentication & Role-Based Access Control (Completed)

- [x] Implement JWT-based authentication:
  - [x] Create a user registration endpoint
  - [x] Create a login endpoint that validates credentials and returns a JWT token
  - [x] Write tests for registration/login flows
- [x] Add role-based access control:
  - [x] Define roles (Already done in UserRole enum)
  - [x] Implement middleware for JWT validation and role checking
  - [x] Write tests for protected endpoints

### D. Project Lifecycle Management Endpoints (In Progress)

- [x] Develop endpoints for each project lifecycle phase:
  - [x] Tasks CRUD endpoints
  - [x] Requirements Analysis
  - [x] System & Software Design
  - [x] Implementation/Development
  - [x] Integration & Testing
  - [x] Deployment/Release
  - [x] Maintenance & Project Closure
- [x] Write integration tests for complete project lifecycle flow

---

## 3. Frontend Setup (In Progress)

### A. Base Setup & Landing Page

- [ ] Bootstrap a new Next.js project in `/frontend`.
- [ ] Create a basic landing page that displays:
  - "Welcome to the IT Waterfall Resource Manager"

### B. Core UI Components & API Integration

- [x] Develop a project dashboard that:
  - [x] Fetches and displays a list of tasks from the backend
- [ ] Create a form for submitting new project proposals
- [x] Implement API integration:
  - [x] Write services to call backend endpoints
  - [ ] Manage JWT tokens for authentication
  - [ ] Test the integration with CRUD endpoints
- [ ] Write unit tests for UI components
- [ ] Ensure smooth navigation between pages

---

## 4. Advanced Features & Collaboration Tools (Not Started)

...

## 5. Integration & Finalization (Not Started)

...

## 6. Deployment & CI/CD Pipeline (Partially Complete)

- [x] Create Dockerfiles for:
  - [x] Rust backend
  - [x] Next.js frontend
- [x] Write a Docker Compose file to run both containers together
- [ ] Set up a CI/CD pipeline:
  - [ ] Automate running tests on each commit
  - [ ] Automate builds and deployments
- [ ] Update documentation with deployment instructions

---

## 7. Documentation & Final Testing (In Progress)

- [x] Update developer documentation:
  - [ ] Create API documentation (Swagger/OpenAPI)
  - [x] Add code comments and update the README
- [ ] Create user documentation
- [ ] Conduct user acceptance testing
- [ ] Perform final integration testing

---

## 8. Post-Deployment & Maintenance (Not Started)

- [ ] Schedule regular reviews and maintenance tasks.
