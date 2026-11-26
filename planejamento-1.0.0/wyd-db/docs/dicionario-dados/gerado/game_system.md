> Pergunta: "o que `content.game_system` guarda e por que nenhuma tabela da série aponta para ela?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.17; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Taxonomia do sistema de jogo do cliente: código estável (`slug`), nome exibido e aposentadoria sem
`DELETE`. É uma das quatro taxonomias de look-up de `0013`; o conteúdo dela (20 linhas) está previsto
no seed `0016`, que segue `bloqueada` até as contagens serem reconferidas na fonte
(`plano-migrations.md` §4). Nenhuma tabela da série referencia esta: ela é catálogo para o runtime,
não alvo de FK.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `slug` | `TEXT` | não | — | identificador do sistema, `snake_case` de 2–32; PK |
| `name` | `TEXT` | não | — | nome exibido em pt-BR, 1–64 caracteres após aparar |
| `enabled` | `BOOLEAN` | não | `true` | `false` aposenta a linha sem `DELETE` |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da linha |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_game_system (slug)` — o código é a identidade (`P-04`: sem `id` sintético onde a chave natural já
é o que o consumidor carrega). `name` não é chave e não tem `UNIQUE`: dois sistemas podem ter o mesmo
nome exibido.

## FKs de saída e de entrada

- **Saída:** nenhuma. O schema `content` não aponta para `game`/`portal` (`plano-migrations.md` §3.1,
  item 5).
- **Entrada:** nenhuma. `grep 'REFERENCES content.game_system'` em `ddl-confirmado.md` não acha nada
  (`V`, 2026-11-22) — diferente das quatro irmãs do mesmo lote, que são alvo de `0014`, `0015`, `0021`
  e `0026`. Consumidor de runtime desta taxonomia não está provado no planejamento (`H`).

## Unicidade

Só a PK: `slug` não se repete e não há restrição de nome único.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_game_system_slug` | `slug` fora de `^[a-z][a-z0-9_]{1,31}$`: maiúscula, hífen, 1 caractere ou 33+ |
| `ck_game_system_nome` | `name` vazio, só com espaço, ou acima de 64 caracteres |

Nada exige que `enabled` comece `true` nem que exista ao menos uma linha habilitada: a taxonomia vazia
é estado aceito pelo schema.

## Índices

Nenhum além do índice da PK. Trigger `trg_game_system_set_updated_at` (`BEFORE UPDATE`, por linha)
usa `ops.fn_set_updated_at()` de `0001`.

## Armadilha de leitura

`enabled = false` é a aposentadoria: como não há FK, o schema **não** impede apagar a linha — a
integridade de quem referencia o código é do seed e do runtime, não do banco. Renomear `slug` muda a
identidade; o nome exibido é o que pode mudar sem quebrar referência.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed (`0016`), que está `bloqueada`.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001_bootstrap.md` §7).
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032_admin_grants.md` §5).
  Dono da permissão: `migrations/0013_content_taxonomy.md` §8.

## Migration

Forma (dona): `migrations/0013_content_taxonomy.md` — arquivo `0013_content_taxonomy.sql`.
