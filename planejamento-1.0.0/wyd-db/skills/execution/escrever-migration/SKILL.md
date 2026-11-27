---
name: escrever-migration
description: Escreve ou corrige um arquivo da série de 34 migrations do wyd-db, com a página de planejamento correspondente e o dicionário regenerado. Use ao pegar issue de migration, ao corrigir DDL de arquivo ainda não publicado, ou quando o runner falhar com NotContiguous e for preciso entender qual buraco da série está aberto.
---

# Escrever migration

## Quando usar

- A issue é de um arquivo da série (`0001`–`0034`) e a página dele em
  `planejamento-1.0.0/wyd-db/migrations/` está `aceita`.
- O DDL já fechado em `ddl-confirmado.md` precisa virar arquivo.
- O runner recusa a série porque falta arquivo ou porque um marcador está fora do lugar.

Não use para inventar DDL novo, mudar constraint para caber dado, nem para seed: seed tem regra
própria (§4 de `plano-migrations.md`) e o arquivo não nasce sem medição.

## Pré-requisitos

- `plano-migrations.md` §2 (formato), §3 (arquivo por arquivo) e §4 (seeds) lidos.
- `padrao-banco.md` — o DDL nasce dele; `ddl-confirmado.md` §5.x é a fonte literal do bloco.
- PostgreSQL 17 local: `make db-up` (`docker compose --profile db up -d --wait db`).
- Árvore limpa em `develop` e issue com DoD acordado.

## Passos

1. **Confirme a posição.** `ls crates/wyd-db/migrations/` tem de estar contíguo de `0001` ao último
   arquivo existente; a série inteira fecha em **34** arquivos. Buraco no meio quebra o runner antes
   de qualquer SQL rodar.
2. **Copie o DDL do `ddl-confirmado.md` sem alterar.** Se precisar mudar algo, a mudança vai primeiro
   para a página e para o `ddl-confirmado.md`; o arquivo não é lugar de decidir.
3. **Escreva `up` e `down` no mesmo arquivo**, com os marcadores `-- migrate:up` e `-- migrate:down`
   **sozinhos na linha** — é o que `apply-series.sh` e o runner procuram.
4. **`down` é a reversão exata na ordem inversa**: trigger antes da tabela, FK antes da tabela alvo,
   função por último. `down` que só “não faz nada” reprova o ciclo.
5. **Sem `ALTER` em arquivo já publicado.** Enquanto a série não entrou em produção, o DDL é
   consolidado no arquivo dono; depois da publicação, correção é arquivo novo.
6. **Atualize o dicionário** quando tabela ou coluna muda: `docs/dicionario-dados/gerado/<tabela>.md`
   e o censo de `wyd-db-1.0.0.md`.
7. **Escreva/atualize a página** `planejamento-1.0.0/wyd-db/migrations/NNNN_slug.md` com as seções
   obrigatórias (`§1` identificação a `§10` o que não decide), incluindo a prova de execução.
8. **Prove na série real**, não em SQL solto:

   ```bash
   make db-up
   for f in crates/wyd-db/migrations/*.sql; do
     awk '/^-- migrate:up/{on=1;next} /^-- migrate:down/{on=0} on' "$f" \
       | PGPASSWORD=test psql -h 127.0.0.1 -U postgres -d wyd -v ON_ERROR_STOP=1 -X -q
   done
   ```

## Como verificar

| Passo | Comando | Espera |
|---|---|---|
| série inteira aplica do zero | laço acima em banco vazio | nenhum `ERROR` |
| runner aceita a ordem | `cargo test -p wyd-db` | sem `NotContiguous` |
| ciclo | aplicar `up`, depois `down`, depois `up` | volta ao mesmo estado |
| página e árvore | `make gate-fast` | verde |

## Quando parar e perguntar

- O DDL do `ddl-confirmado.md` não cabe no dado real: **constraint não se afrouxa sozinha** — é
  decisão do dono, registrada na página e no `ddl-confirmado.md`.
- A correção exige arquivo **novo** na série (muda a contagem de 34 e o `SCHEMA_VERSION`).
- Aparece value de conteúdo para semear: pare. Seed exige medição na fonte, não linha inventada nem
  cópia de `deprecated/`.

## Armadilhas

- **Trazer DDL ou dado do `deprecated/`.** Nível 4 inspira, nível 1 decide; o banco novo foi
  modelado do zero e nada do legado entra por script, dump ou cópia.
- **Relaxar `CHECK`/FK para o dataset herdado passar.** Isso inverte a ordem: primeiro o dado é
  medido, depois o schema é decidido.
- **Marcador com texto na mesma linha** (`-- migrate:up -- nota`) — o bloco não é encontrado e o
  arquivo aplica vazio sem erro.
- **`down` invertendo o `up` na mesma ordem** — `DROP TABLE` antes do `DROP TRIGGER` falha.
- **Teste novo no root** ou em pasta não prevista: teste de série/CI mora em
  `wyd-ops/tests/unitario/`, teste do crate em `crates/wyd-db/tests/`.
