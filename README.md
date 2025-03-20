# Waterfall Resource Manager

An open source tool for resource management in IT projects following the waterfall methodology.

## A small introduction

This project will be developed in my free time, in order to explore Rust language, Next.js and agentic/AI tools usage. I will try to keep everything as clean as possible. For the moment i choosed to adopt a Monorepo approach, even if probably at some point i will split the project in two, for backend and frontend.

## Features

- Waterfall methodology project management
- Resource planning and allocation
- Time, cost, and progress monitoring
- Dashboard to visualize project status
- Notification system for delays and issues

## Technologies

- **Backend**: Rust
- **Frontend**: Next.js, React, Tailwind CSS

## Installation

### Prerequisites

- Node.js (v18+)
- PostgreSQL
- Docker (optional)

### Manual Setup

```bash
# Clone the repository
# Install backend dependencies # TODO complete

# Install frontend dependencies
cd ../frontend
npm install

# Start in development mode
cd ..
docker-compose up  # Or start backend and frontend separately
```

### Setup with Docker

```bash
# Clone the repository
git clone https://github.com/yourusername/waterfall-resource-manager.git
cd waterfall-resource-manager

# Start with Docker Compose
docker-compose up
```

## Development

```bash
# Backend
cd backend
npm run dev  # Start the development server

# Frontend
cd frontend
npm run dev  # Start the Next.js development server
```

## License

MIT

## Roadmap

- Get a grasp over Rust language and Next.js features
- Set up a pipeline
- Deploy stuff somewhere
- ...?
