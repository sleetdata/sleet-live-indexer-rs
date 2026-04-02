# DOCKER

## Quick Start

```sh
# 1. Clone and enter project
git clone <repo-url> sleet-live-indexer-rs
cd sleet-live-indexer-rs

# 2a. Option A: For Docker (plain KEY=value format)
cp .env.docker .env
# Edit .env and set your webhook URLs

# 2b. Option B: For shell sourcing (export format)
cp .env.example .env
# Edit .env and set your webhook URLs
# For running binaries locally
source .env
# Docker Compose automatically reads `.env` from the project directory.

# 3. Build and start both services
docker compose up -d --build

# 4. View logs
docker compose logs -f

# 5. Stop services
docker compose down

# 6. Stop and remove volumes (clears SQLite DB)
docker compose down -v
```

## Individual Service Control

```sh
# Start both
docker compose up -d

# Start specific service
docker compose up -d blackjack
docker compose up -d deleteaccount

# Stop specific service
docker compose stop blackjack
docker compose stop deleteaccount

# Restart specific service
docker compose restart blackjack

# View logs for specific service
docker compose logs -f blackjack
docker compose logs -f deleteaccount
```

## Notes

- **Auto-restart**: Services restart automatically unless stopped (`restart: unless-stopped`)
- **SQLite database**: Stored in `./temp/indexer.db` (mounted volume)
- **NEAR Stream URL**: Defaults to `http://host.docker.internal:8080` (your local machine)
- **Discord webhooks**: Set in `.env` file

---

copyright 2026 by sleet.near
