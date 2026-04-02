# Build stage
FROM rust:1.85-slim AS builder

WORKDIR /app

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

# Copy built binaries from builder
COPY --from=builder /app/target/release/stream_tx_discord_blackjack .
COPY --from=builder /app/target/release/stream_tx_discord_deleteaccount .

# Create temp directory for SQLite database
RUN mkdir -p ./temp

# Default command (can be overridden by docker-compose)
CMD ["./stream_tx_discord_blackjack"]
