.PHONY: up
up:
	docker compose up -d

.PHONY: down
down:
	docker compose down

.PHONY: dev
dev:
	sqlx migrate run
	cargo watch -x run

.PHONY: test
test:
	cargo nextest run
