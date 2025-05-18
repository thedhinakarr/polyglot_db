# Development Setup Guide

This guide provides instructions for setting up the development environment for the Polyglot Persistence System.

## Prerequisites

- Node.js (v14+)
- Rust (latest stable)
- Python (3.8+)
- Docker and Docker Compose
- Git

## Getting Started

1. Clone the repository:
   ```bash
   git clone [repository-url]
   cd polyglot_db
   ```

2. Start the database containers:
   ```bash
   docker-compose up -d
   ```

3. Set up and run each service:

### Auth Service (JavaScript/Node.js)

```bash
cd auth-service
npm install
cp .env.example .env  # Update with your configuration
npm run dev
```

### Business Service (Rust)

```bash
cd business-service
cargo build
cargo run
```

### Analytics Service (Python)

```bash
cd analytics-service
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt
python app.py
```

## Testing

Each service has its own testing framework:

### Auth Service
```bash
cd auth-service
npm test
```

### Business Service
```bash
cd business-service
cargo test
```

### Analytics Service
```bash
cd analytics-service
python -m pytest
```

## API Documentation

For API documentation, see the [API Documentation](../api) directory.
