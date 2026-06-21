# Expense Tracker API

A production-grade REST API for tracking personal expenses, built with Rust, Axum, and PostgreSQL. Features JWT authentication, input validation, Docker containerization, and GitHub Actions CI/CD.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust 1.96 |
| Web Framework | Axum 0.8 |
| Async Runtime | Tokio |
| Database | PostgreSQL (via SQLx) |
| Auth | JWT (jsonwebtoken) + Argon2 |
| Validation | validator |
| Containerization | Docker + Docker Compose |
| CI/CD | GitHub Actions |

## Features

- **User Authentication**: Register/Login with JWT tokens, Argon2 password hashing
- **Expense CRUD**: Create, list, get, update, delete expenses (user-scoped)
- **Input Validation**: Email format, password length, required fields
- **Protected Routes**: Bearer token middleware on all expense endpoints
- **Structured Error Handling**: Consistent JSON error responses
- **Docker Support**: Multi-stage Dockerfile + docker-compose
- **Integration Tests**: 4 tests covering auth and expense flows
- **CI/CD**: Automated testing on every push/PR

## API Endpoints

### Public
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| POST | `/auth/register` | Create new account |
| POST | `/auth/login` | Login, receive JWT |

### Protected (requires Bearer token)
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/expenses` | List all expenses (paginated) |
| POST | `/expenses` | Create new expense |
| GET | `/expenses/{id}` | Get single expense |
| PUT | `/expenses/{id}` | Update expense |
| DELETE | `/expenses/{id}` | Delete expense |

## Quick Start

### Prerequisites
- Rust 1.96+
- PostgreSQL 16+
- `sqlx-cli`: `cargo install sqlx-cli --no-default-features --features native-tls,postgres`

### 1. Environment Setup

Create `.env`:
```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/expense_tracker
JWT_SECRET=your-secret-key-min-32-chars-long
JWT_EXPIRATION=604800
PORT=3000
RUST_LOG=debug﻿
