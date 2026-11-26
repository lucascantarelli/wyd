# Fachada de comandos. Nenhuma lógica de negócio mora aqui: o gate é
# `wyd-docs/gate.sh`, os scripts de CI são `scripts/ci/*.sh` e o Compose é a
# definição do ambiente. `make` só evita decorar caminhos.

CRATES := wyd-db wyd-ops-signer wyd-launcher

.PHONY: gate fmt lint test docs docs-up docs-down db-up db-down rollback

gate:
	docker compose --profile gate run --rm gate

fmt:
	@for c in $(CRATES); do \
		cargo fmt --manifest-path crates/$$c/Cargo.toml -- --check || exit 1; \
	done

lint:
	@for c in $(CRATES); do \
		cargo clippy --manifest-path crates/$$c/Cargo.toml --all-targets -- -D warnings || exit 1; \
	done

test:
	cargo test --manifest-path crates/wyd-db/Cargo.toml --lib
	cargo test --manifest-path crates/wyd-ops-signer/Cargo.toml --offline
	cargo test --manifest-path crates/wyd-launcher/Cargo.toml --offline

docs:
	docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d docs --build

docs-up:
	docker compose pull docs
	docker compose up -d --wait docs

docs-down:
	docker compose down

db-up:
	docker compose --profile db up -d --wait db

db-down:
	docker compose --profile db down

rollback:
	@test -n "$(TAG)" || { echo "uso: make rollback TAG=<sha>"; exit 1; }
	WYD_DOCS_TAG=$(TAG) docker compose up -d --wait docs
