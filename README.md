@'
# Expense Tracker API

Production-style REST API in Rust for personal expense tracking with JWT authentication, input validation, and cloud PostgreSQL.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust 1.96.0 |
| Web Framework | Axum 0.8 |
| Async Runtime | Tokio |
| Database | SQLx 0.8 + PostgreSQL (Supabase) |
| Auth | JWT (jsonwebtoken) + Argon2 |
| Validation | validator crate |
| Serialization | serde / serde_json |

## Features

- ✅ **User Authentication** — Register/login with Argon2 password hashing + JWT tokens
- ✅ **Protected Expense CRUD** — Create, list, get, update, delete expenses (user-scoped)
- ✅ **Input Validation** — Email format, password length, amount format, category constraints
- ✅ **Error Handling** — Structured JSON errors with appropriate HTTP status codes
- ✅ **Cloud Database** — Supabase PostgreSQL with connection pooling

## API Endpoints

| Method | Endpoint | Auth Required | Description |
|--------|----------|--------------|-------------|
| POST | `/auth/register` | No | Create new account |
| POST | `/auth/login` | No | Get JWT token |
| GET | `/health` | No | Health check |
| POST | `/expenses` | Yes (Bearer token) | Create expense |
| GET | `/expenses` | Yes (Bearer token) | List my expenses |
| GET | `/expenses/{id}` | Yes (Bearer token) | Get single expense |
| PUT | `/expenses/{id}` | Yes (Bearer token) | Update expense |
| DELETE | `/expenses/{id}` | Yes (Bearer token) | Delete expense |

## Authentication

All protected endpoints require a Bearer token in the Authorization header:

```http
Authorization: Bearer <jwt_token>
