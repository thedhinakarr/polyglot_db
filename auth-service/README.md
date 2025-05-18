# Auth Service

This service handles user authentication and management for the Polyglot Persistence System.

## Features

- User registration and login
- JWT-based authentication
- Session management with Redis
- User profile management

## Technologies

- Node.js/Express
- PostgreSQL for user data
- Redis for session management
- JWT for authentication

## Setup

1. Install dependencies:
   ```bash
   npm install
   ```

2. Create `.env` file (use `.env.example` as a template):
   ```bash
   cp .env.example .env
   ```

3. Start development server:
   ```bash
   npm run dev
   ```

## Testing

```bash
npm test
```

## API Documentation

See [Auth Service API Documentation](../docs/api/auth-service.md) for detailed API information.
