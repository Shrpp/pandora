.PHONY: help dev down logs migrate cli test-oidc

help:
	@echo ""
	@echo "  make dev        — build and start postgres + ovtl-core (Docker)"
	@echo "  make down       — stop and remove containers"
	@echo "  make logs       — tail server logs"
	@echo "  make migrate    — run pending migrations (server must be stopped)"
	@echo "  make cli        — launch the Ratatui admin TUI"
	@echo "  make test-oidc  — run the full OIDC flow end-to-end (needs server up)"
	@echo ""

dev:
	docker compose up --build

down:
	docker compose down

logs:
	docker compose logs -f ovtl-core

migrate:
	docker compose run --rm ovtl-core ./ovtl-core --migrate

cli:
	cargo run -p ovtl-cli

test-oidc:
	@bash scripts/test-oidc.sh
