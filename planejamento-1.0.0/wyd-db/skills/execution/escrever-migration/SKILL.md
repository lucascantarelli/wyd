---
name: escrever-migration
description: Escreve ou corrige um arquivo da série de 34 migrations do wyd-db, com a página de planejamento correspondente e o dicionário regenerado. Use ao pegar issue de migration, ao corrigir DDL de arquivo ainda não publicado, ou quando o runner falhar com NotContiguous e for preciso entender qual buraco da série está aberto.
---

# Escrever migration

## Quando usar

- A issue é de um arquivo da série (`0001`–`0034`) e a página dele em
  `planejamento-1.0.0/wyd-db/migrations/` está `aceita`.
- O DDL já fechado em `planejamento-1.0.0/wyd-db/ddl-confirmado.md` precisa virar arquivo.
- O runner recusa a série porque falta arquivo ou porque um marcador está fora do lugar.

Não use para inventar DDL novo, mudar constraint para caber dado, nem para seed: seed tem regra
própria (§4 de `plano-migrations.md`) e o arquivo não nasce sem medição.

## Pré-requisitos

- `planejamento-1.0.0/wyd-db/plano-migrations.md` §2 (formato), §3 (arquivo por arquivo) e §4 (seeds).
- `planejamento-1.0.0/wyd-db/docs/arquitetura/padroes-postgres.md` (o DDL nasce dele);
  `ddl-confirmado.md` §5.x é a fonte literal do bloco.
- Docker disponível: `make db-up` sobe o serviço `db` (PostgreSQL 17, base `wyd`, em
  `127.0.0.1:5432`, usuário `postgres`, senha `test`).
- Árvore limpa em `develop` e issue com DoD acordado.

## Passos

1. **Confirme a posição.** `ls crates/wyd-db/migrations/` tem de estar contíguo de `0001` ao último
   arquivo existente; a série inteira fecha em **34** arquivos. Buraco no meio quebra o runner antes
   de qualquer SQL rodar.
2. **Copie o DDL do `ddl-confirmado.md` sem alterar.** Mudança de DDL vai primeiro para a página e
   para o `ddl-confirmado.md`; o arquivo não é lugar de decidir.
3. **Escreva `up` e `down` no mesmo arquivo**, com os marcadores `-- migrate:up` e `-- migrate:down`
   **sozinhos na linha**. `wyd-ops/scripts/ci/apply-series.sh` valida isso e falha com código 4 (dois
   `up`), 5 (ordem errada) ou 6 (marcador faltando).
4. **`down` é a reversão exata na ordem inversa**: trigger antes da tabela, FK antes da tabela alvo,
   função por último. `down` que só “não faz nada” reprova o ciclo.
5. **Sem `ALTER` em arquivo já publicado.** Enquanto a série não entrou em produção, o DDL é
   consolidado no arquivo dono; depois da publicação, correção é arquivo novo.
6. **Atualize o dicionário** quando tabela ou coluna muda:
   `planejamento-1.0.0/wyd-db/docs/dicionario-dados/gerado/<tabela>.md` e o censo do dashboard.
7. **Escreva/atualize a página** `planejamento-1.0.0/wyd-db/migrations/NNNN_slug.md` com as seções
   obrigatórias (§1 identificação a §10 o que não decide), incluindo a prova de execução.
8. **Prove na série real**, do zero:

   ```bash
   make db-up

   # papéis que o preflight do runner exige (idempotente)
   for r in wyd_owner wyd_migrator wyd_game wyd_portal wyd_admin wyd_read; do
     docker compose exec -T db psql -U postgres -d wyd -qtAc \
       "SELECT 1 FROM pg_roles WHERE rolname='$r'" | grep -q 1 ||
       docker compose exec -T db psql -U postgres -d wyd -qc "CREATE ROLE $r NOLOGIN"
   done

   # aplica só o bloco `-- migrate:up` de cada arquivo, em ordem de nome
   for f in crates/wyd-db/migrations/*.sql; do
     awk '{l=$0; sub(sprintf("%c",13),"",l)}
          l=="-- migrate:up"{u=1;d=1;next} l=="-- migrate:down"{d=0;next} d{print l}' "$f" |
       docker compose exec -T db psql -U postgres -d wyd -v ON_ERROR_STOP=1 -X -q || break
   done
   ```

   Com `psql` instalado no host (é o caso do CI, que instala `postgresql-client`), o caminho
   canônico é o script da operação:

   ```bash
   PGHOST=127.0.0.1 PGPORT=5432 PGUSER=postgres PGPASSWORD=test PGDATABASE=wyd \
     bash wyd-ops/scripts/ci/apply-series.sh
   ```

## Como verificar

| Passo | Comando | Espera |
|---|---|---|
| série inteira aplica do zero | laço acima em banco vazio | nenhum `ERROR`; 28 arquivos hoje |
| série contígua (o runner recusa buraco com `NotContiguous`) | `WYD_DB_TEST_CONTAINER=$(docker compose ps -q db) cargo test --manifest-path crates/wyd-db/Cargo.toml --test sistema_pg -- --test-threads=1` | verde, sem `NotContiguous` |
| ciclo | `WYD_DB_TEST_CONTAINER=$(docker compose ps -q db) cargo test --manifest-path crates/wyd-db/Cargo.toml --test reversibilidade_pg -- --test-threads=1` | `3 passed`: 1 ciclo real em PG17 + 2 conferências textuais |
| página e árvore | `make gate-fast` | verde |

## Quando parar e perguntar

- O DDL do `ddl-confirmado.md` não cabe no dado real: **constraint não se afrouxa sozinha** — é
  decisão do dono, registrada na página e no `ddl-confirmado.md`.
- A correção exige arquivo **novo** na série: a contagem de 34 arquivos muda e a série deixa de ser a
  mesma que `plano-migrations.md` §3 descreve (`plano-migrations.md` §6 trata da troca de série).
- Aparece valor de conteúdo para semear: pare. Seed exige medição na fonte, não linha inventada nem
  cópia de `deprecated/`.

## Armadilhas

- **Trazer DDL ou dado do `deprecated/`.** Nível 4 inspira, nível 1 decide; o banco novo foi modelado
  do zero e nada do legado entra por script, dump ou cópia.
- **Relaxar `CHECK`/FK para o dataset herdado passar.** Primeiro o dado é medido, depois o schema é
  decidido.
- **Marcador com texto na mesma linha** (`-- migrate:up -- nota`) — o bloco não é encontrado e o
  arquivo aplica vazio sem erro.
- **`down` invertendo o `up` na mesma ordem** — `DROP TABLE` antes do `DROP TRIGGER` falha.
- **Teste novo no root**: teste do crate vai em `crates/wyd-db/tests/` (`sistema/`, `contrato/`,
  `comum/`, `resources/`); teste de série/CI/script vai em `wyd-ops/tests/unitario/`.
