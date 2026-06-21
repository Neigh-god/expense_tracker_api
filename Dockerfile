# Build stage
FROM rust:1.96-slim-bookworm AS builder

WORKDIR /app

# Install dependencies for SQLx compile-time checks
RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

# Copy manifests first for better caching
COPY Cargo.toml Cargo.lock ./
COPY .sqlx ./.sqlx

# Copy migrations (needed for SQLx compile-time verification)
COPY migrations ./migrations

# Copy source code
COPY src ./src

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/expense_tracker_api /app/expense_tracker_api

# Expose port
EXPOSE 3000

# Run the binary
CMD ["/app/expense_tracker_api"]
