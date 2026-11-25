> Pergunta: "o que `game.character_chaos` guarda e qual faixa `cp` aceita?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.11; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Contador de pontos de caos (`cp`) do personagem: no máximo uma linha por personagem.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | PK e FK para `character` |
| `cp` | `SMALLINT` | não | `0` | pontos de caos, de −75 a 75 |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da linha |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_character_chaos (character_id)` — o personagem é a chave; não há id próprio.

## FKs de saída e de entrada

- **Saída:** `fk_character_chaos_character`: `character_id → game.character (id)`,
  `CASCADE`/`NO ACTION`.
- **Entrada:** nenhuma.

## Unicidade

Só a PK.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_character_chaos_cp` | `cp` fora de −75 a 75 |

O `CHECK` fecha só a faixa: o efeito de jogo em cada limite não é definido nesta página.

## Índices

Nenhum além da PK. Trigger `trg_character_chaos_set_updated_at`.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal` (sem `USAGE` em `game`).

## Migration

Forma (dona): `migrations/0008-game-skills.md` — arquivo `0008-game-skills.sql`.
