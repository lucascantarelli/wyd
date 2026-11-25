> Pergunta: "o que `content.item_effect` guarda e como o catálogo garante que só existe efeito conhecido?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.17; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Efeito de item — taxonomia que traduz a constante `EF_*` do cliente em `slug` estável
(`EF_HUMAN` → `ef_human`). Os 55 símbolos estão previstos no seed `0017`, que segue `bloqueada` até as
contagens serem reconferidas na fonte (`plano-migrations.md` §4). É referenciada diretamente por
`content.refine_gem_effect` (`0021`) e, por trigger, pelo array `effect_names` de `item_catalog`
(`0014`). Não guarda magnitude, duração nem alvo: só o símbolo.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `slug` | `TEXT` | não | — | efeito em `snake_case`, derivado de `EF_*`; 2–32; PK |
| `name` | `TEXT` | não | — | nome exibido, 1–64 caracteres após aparar |
| `enabled` | `BOOLEAN` | não | `true` | `false` aposenta o efeito sem `DELETE` |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da linha |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

O cliente escreve a constante em maiúsculas (`EF_*`); esta tabela guarda a forma dobrada em minúsculas.
A conversão é regra de seed, não coluna.

## Chave primária

`pk_item_effect (slug)` — o símbolo é a identidade (`P-04`). Não há coluna numérica do efeito: o número
do cliente, se existir, não foi trazido para o schema.

## FKs de saída e de entrada

- **Saída:** nenhuma; o schema `content` não aponta para `game`/`portal`.
- **Entrada, `RESTRICT` (1 FK):** `fk_refine_gem_effect_codigo` em
  `content.refine_gem_effect.effect_code` (`0021`), `ON UPDATE RESTRICT`, `ON DELETE RESTRICT`.
- **Entrada sem FK — `item_catalog.effect_names`:** o array `TEXT[]` não referencia tabela; quem recusa
  símbolo fora desta tabela é o trigger `trg_item_catalog_effects`, com `23503` nomeando
  `fk_item_catalog_efeito` no erro. Esse nome **não** é constraint: é rótulo da exceção levantada pelo
  trigger (`gerado/item_catalog.md` §"`CHECK`s").

## Unicidade

Só a PK — nenhum único adicional.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_item_effect_slug` | `slug` fora de `^[a-z][a-z0-9_]{1,31}$` |
| `ck_item_effect_nome` | `name` vazio, só com espaço, ou acima de 64 caracteres |

## Índices

Nenhum além do índice da PK. Trigger `trg_item_effect_set_updated_at` (`BEFORE UPDATE`, por linha).

## Armadilha de leitura

`enabled = false` **não** invalida efeito já gravado em `item_catalog.effect_names`: o trigger de
`0014` confere existência do `slug`, não aposentadoria. Efeito aposentado continua referenciável, e a
decisão de o runtime aplicá-lo ou não não está no banco. Renomear `slug` depois do seed falha com
`23503` enquanto houver linha de `refine_gem_effect` apontando: `ON UPDATE RESTRICT` impede o `UPDATE`
na origem, não propaga a mudança.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed (`0017`), que está `bloqueada`.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001-bootstrap.md` §7). Dono da permissão: `migrations/0013-content-taxonomy.md` §8.
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032-admin-grants.md` §5).

## Migration

Forma (dona): `migrations/0013-content-taxonomy.md` — arquivo `0013-content-taxonomy.sql`.
