# ==============================================================================
# Environment

POSTGRES := postgres:16.4
GENI := ghcr.io/emilpriver/geni:v1.1.5
DATABASE_URL := postgres://postgres:password@localhost:5432/newsletter

# ==============================================================================
# Dependencies

dev-docker:
	docker pull $(ALPINE) & \
	docker pull $(POSTGRES) & \
	docker pull $(GENI) & \
	wait;

# ==============================================================================
# Docker Compose

compose-db-up:
	docker compose --profile db up

compose-db-down:
	docker compose --profile db down

# ==============================================================================
# Admin

pgcli:
	pgcli postgresql://postgres:postgres@localhost:5432/reminders_dev

health:
	curl -il http://localhost:8000/health

# ==============================================================================
# Cargo

lint:
	cargo run clippy

test:
	cargo test

clean:
	cargo clean
