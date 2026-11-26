> Pergunta: "o que `game.guild` guarda e como o nome único sem caixa é garantido?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.13; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Guilda como entidade: nome exibido e nome dobrado para comparação. Não guarda líder, nível nem fama —
liderança é papel em `guild_member`, e o resto não tem semântica medida.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `name` | `TEXT` | não | — | nome de 3–16 caracteres após aparar |
| `name_fold` | `TEXT` gerada | sim | `lower(btrim(name))` | `STORED`, alvo do índice único |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | fundação |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_guild (id)` — a guilda é entidade; o nome é único, não chave.

## FKs de saída e de entrada

- **Saída:** nenhuma.
- **Entrada, `CASCADE`:** `guild_member`, `guild_relation` (origem e alvo).
- **Entrada, `SET NULL`:** `guild_tower`, `event_participation`, `kefra_state` e `castle_state` — a
  guilda some e o estado perde a referência, sem levar o histórico.
- **Sem FK para conteúdo:** zona de guilda e guerra são conteúdo (`0019`), não coluna daqui.

## Unicidade

`ux_guild_nome (name_fold)` — índice único **total**, sem cláusula parcial: guilda que acaba é apagada,
não fica como linha morta segurando o nome.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_guild_nome` | nome com menos de 3 ou mais de 16 caracteres após `btrim` |

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ux_guild_nome (name_fold)` | nome livre para fundar guilda |

Trigger `trg_guild_set_updated_at`.

## Papéis

- **Escreve:** `wyd_game`.
- **Lê:** `wyd_game`, `wyd_read`.
- **Não lê:** `wyd_portal` (sem `USAGE` em `game`); leitura de ranking pelo portal é leitura planejada,
  sem contrato de view aprovado (`P`).

## Migration

Forma (dona): `migrations/0010_game_guild.md` — arquivo `0010_game_guild.sql`.
