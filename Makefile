# Fachada de comandos. Nenhuma lógica de negócio mora aqui: o gate é
# `wyd-docs/gate.sh`, os scripts de CI são `wyd-ops/scripts/ci/*.sh` e o Compose é a
# definição do ambiente. `make` só evita decorar caminhos.

# Todos os crates são Rust portátil, inclusive o núcleo do `wyd-launcher` (sem
# Tauri nem `cfg(windows)`); o empacotamento NSIS, quando existir, é que exige
# Windows e terá alvo próprio. Aqui o launcher entra como qualquer outro crate.
CRATES := wyd-db wyd-ops-signer wyd-launcher wyd-protocol wyd-net wyd-core wyd-sim wyd-server wyd-admin wyd-bots
# Crates com teste que entram no `make test`; o `wyd-db` roda a `lib` e os
# contratos `contrato_migrations`/`contrato_api_migrate`/`contrato_api_provision`
# (os alvos PostgreSQL têm `ci-db.yml`).
TEST_CRATES := wyd-ops-signer wyd-launcher wyd-protocol wyd-net wyd-core wyd-sim wyd-server wyd-admin wyd-bots
ifeq ($(OS),Windows_NT)
PYTHON ?= py -3
else
PYTHON ?= python
endif

.PHONY: gate gate-fast layout test-gate build fmt lint test docs docs-up docs-down db-up db-down rollback

gate:
	docker compose --profile gate run --rm gate

# Checks nativos: não precisa de Docker nem de MkDocs; o build e os links ficam
# no `make gate`/CI. Use no ciclo curto do agente.
gate-fast:
	cd wyd-docs && $(PYTHON) -m gate fast --root ..

# Só o layout: compara a árvore real com a página autoritativa e lista cada
# caminho fora do padrão. O `gate-fast` e o `gate` já o executam.
layout:
	cd wyd-docs && $(PYTHON) -m gate check layout --root .. --details

test-gate:
	cd wyd-docs && $(PYTHON) -m unittest discover -s tests -v

build:
	@for c in $(CRATES); do \
		cargo build --manifest-path crates/$$c/Cargo.toml || exit 1; \
	done

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
	cargo test --manifest-path crates/wyd-db/Cargo.toml --test contrato_migrations --locked
	cargo test --manifest-path crates/wyd-db/Cargo.toml --test contrato_api_migrate --locked
	cargo test --manifest-path crates/wyd-db/Cargo.toml --test contrato_api_provision --locked
	@for c in $(TEST_CRATES); do \
		cargo test --manifest-path crates/$$c/Cargo.toml --locked || exit 1; \
	done

docs:
	docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d docs --build

docs-up:
	docker compose pull docs
	docker compose up -d --no-build --wait docs

docs-down:
	docker compose down

db-up:
	docker compose --profile db up -d --wait db

db-down:
	docker compose --profile db down

rollback:
	@test -n "$(TAG)" || { echo "uso: make rollback TAG=<sha>"; exit 1; }
	WYD_DOCS_TAG=$(TAG) docker compose pull docs
	WYD_DOCS_TAG=$(TAG) docker compose up -d --no-build --wait docs
