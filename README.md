# Polyglot Persistence System

A microservices-based system implementing polyglot persistence principles, where each service uses the most appropriate programming language and database technology for its specific problem domain.

## System Overview

This system consists of three main services:

- **Auth Service** (JavaScript/Node.js): User authentication and management
- **Business Service** (Rust): Core business logic and transactions
- **Analytics Service** (Python): Data processing and analysis

Each service implements a problem-first approach to database selection, using specialized databases optimized for its specific data requirements.

## Getting Started

See the [Development Setup Guide](docs/development/setup.md) for instructions on how to set up and run the system locally.

## Architecture

For detailed architecture information, see the [Architecture Documentation](docs/architecture/overview.md).

## Database Selection

Our database selection follows a problem-first approach as described in the [Database Selection Strategy](docs/database/selection-strategy.md).

## API Documentation

API documentation is available for each service:
- [Auth Service API](docs/api/auth-service.md)
- [Business Service API](docs/api/business-service.md)
- [Analytics Service API](docs/api/analytics-service.md)