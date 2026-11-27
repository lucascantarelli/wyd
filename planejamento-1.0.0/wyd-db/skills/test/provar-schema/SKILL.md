---
name: provar-schema
description: Prova que um arquivo da série do wyd-db faz o que a página diz, nas quatro camadas — estrutural, contrato, banco real e ciclo up/down/up — com PostgreSQL 17 local ou em container. Use antes de fechar issue de migration, ao reproduzir falha de CI com Postgres 17, e quando for preciso saber se a série inteira sobe do zero.
---

# Provar schema

## Quando usar

- Migration nova ou corrigida vai para merge.
- O CI falhou e é preciso reproduzir local com PostgreSQL 17.

Não use para testar comportamento de runtime (`game`/`portal`): ali a prova é de aplicação.

## Pré-requisitos

- Docker disponível: `make db-up` sobe o serviço `db` (PostgreSQL 17, base `wyd`, `127.0.0.1:5432`,
  `postgres`/`test`). `make db-down` derruba.
- Papéis criados por `wyd-ops/scripts/ci/db-prepare.sh` (`wyd_owner`, `wyd_migrator`, `wyd_game`,
  `wyd_portal`, `wyd_admin`, `wyd_read`) — o preflight do runner recusa outro papel.
- Série contígua em `crates/wyd-db/migrations/`.

## Passos

1. **Camada estrutural** — banco vazio, série aplicada em ordem, sem `ERROR`:

   ```bash
   make db-up
   for r in wyd_owner wyd_migrator wyd_game wyd_portal wyd_admin wyd_read; do
     docker compose exec -T db psql -U postgres -d wyd -qtAc \
       "SELECT 1 FROM pg_roles WHERE rolname='$r'" | grep -q 1 ||
       docker compose exec -T db psql -U postgres -d wyd -qc "CREATE ROLE $r NOLOGIN"
   done
   for f in crates/wyd-db/migrations/*.sql; do
     awk '{l=$0; sub(sprintf("%c",13),"",l)}
          l=="-- migrate:up"{u=1;d=1;next} l=="-- migrate:down"{d=0;next} d{print l}' "$f" |
       docker compose exec -T db psql -U postgres -d wyd -v ON_ERROR_STOP=1 -X -q || break
   done
   ```

   Com `psql` no host, use o caminho canônico:
   `PGHOST=127.0.0.1 PGPORT=5432 PGUSER=postgres PGPASSWORD=test PGDATABASE=wyd bash wyd-ops/scripts/ci/apply-series.sh`.

2. **Camada de contrato** — o que a página promete existe com o tipo prometido:
   `docker compose exec -T db psql -U postgres -d wyd -c '\d content.<tabela>'`, contagem de índices e
   constraints com o nome do padrão.
3. **Camada de banco real** — os alvos de integração. O harness (`tests/comum/mod.rs`) usa
   `psql` do PATH quando `WYD_DB_TEST_URL` existe (é o caminho do CI) e `docker exec` quando a
   variável é `WYD_DB_TEST_CONTAINER` (é o caminho de quem não tem `psql`, como esta máquina):

   ```bash
   # local, sem psql instalado: usa o container do make db-up
   WYD_DB_TEST_CONTAINER=$(docker compose ps -q db) \
   WYD_DB_TEST_USER=postgres WYD_DB_TEST_DB=wyd \
     cargo test --manifest-path crates/wyd-db/Cargo.toml --test sistema_pg -- --test-threads=1

   # CI: psql instalado e service container exposto em TCP
   WYD_DB_TEST_URL=postgres://postgres:test@127.0.0.1:5432/wyd \
   WYD_DB_TEST_USER=postgres WYD_DB_TEST_DB=wyd \
     cargo test --manifest-path crates/wyd-db/Cargo.toml --test sistema_pg -- --test-threads=1
   ```

   O alvo `runner_pg` exige banco preparado com `db-prepare.sh migrator` (papel `wyd_migrator` com
   LOGIN) e não aplica a série; `reversibilidade_pg` aplica e desfaz.
4. **Camada de ciclo** — aplicar `down` e aplicar `up` de novo devolve o mesmo estado; se sobrar
   objeto, o `down` está incompleto.
5. **Registre a evidência** na página (§ prova de execução) e no comentário da issue: comando, saída
   relevante e data.

## Como verificar

| Alegação | Prova |
|---|---|
| série sobe do zero | laço do passo 1 sem `ERROR` (28 arquivos hoje, 111 tabelas em `content`/`game`/`portal`/`ops`) |
| ordem aceita pelo runner | `runner_pg` verde, sem `NotContiguous` |
| `down` reverte | `reversibilidade_pg` verde; `to_regclass('content.<tabela>')` nulo depois do `down` |
| nada quebrou no resto | `make gate-fast` |

## Quando parar e perguntar

- A falha é de dado, não de DDL (constraint recusando valor real): pare e leve ao dono — schema não
  se ajusta ao dado sem decisão.
- A falha só reproduz no CI: registre a diferença de ambiente (imagem, papel, versão) antes de mexer.

## Armadilhas

- **Rodar `psql -f` do arquivo inteiro**: os dois blocos rodam juntos e o `down` desfaz o teste.
- **Rodar o alvo de integração sem `WYD_DB_TEST_URL` nem `WYD_DB_TEST_CONTAINER`**: o harness
  aborta com "defina WYD_DB_TEST_URL … ou WYD_DB_TEST_CONTAINER".
- **Concluir pelo `up`**: sem o ciclo, `down` quebrado passa.
- **`make gate` completo em vez de `gate-fast`** a cada passo: use o curto no ciclo e o completo ao fim.
