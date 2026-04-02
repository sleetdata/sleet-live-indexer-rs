# Cargo chef stage for dependency caching
FROM rust:1.94-slim AS chef
WORKDIR /app
RUN cargo install cargo-chef --locked

# Planner stage
FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Cook dependencies (cached layer)
RUN cargo chef cook --release --recipe-path recipe.json

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build in release mode
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies (for rusqlite)
RUN apt-get update && apt-get install -y --no-install-recommends \
    libsqlite3-0 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy all built binaries from builder
COPY --from=builder /app/target/release/ ./bin/

# Create temp directory for SQLite database
RUN mkdir -p ./temp

# Default command (can be overridden by docker-compose)
CMD ["./bin/stream_tx_discord_blackjack"]
