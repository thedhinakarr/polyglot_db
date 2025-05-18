# Auth Service

## Overview

The Auth Service is responsible for user authentication and management. It is implemented in JavaScript/Node.js and uses PostgreSQL for user data and Redis for session management.

## Key Features

- User registration
- User authentication with JWT
- Session management
- User profile management

## Implementation Details

### Repository Pattern

The service implements the repository pattern to abstract database operations:

- `BaseRepository`: Abstract class defining standard CRUD operations
- `UserRepository`: Implementation for PostgreSQL user data
- `SessionRepository`: Implementation for Redis session data

### Database Clients

Database access is abstracted through client classes:

- `PostgresClient`: Client for PostgreSQL operations
- `RedisClient`: Client for Redis operations

### Service Layer

Business logic is contained in service classes:

- `UserService`: Handles user registration, authentication, and profile management

### API Layer

The API is implemented using Express:

- `AuthController`: Handles HTTP requests
- `auth.routes.js`: Defines API endpoints
- `auth.middleware.js`: JWT verification middleware

## Deployment

The service can be deployed as a Docker container. See the Docker Compose configuration in the project root.