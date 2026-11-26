> Pergunta: "o que `content.guard_spawn_reward` guarda de cada exigência do guarda e o que o CHECK realmente garante?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.21; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel
**Sem arquivo no repositório:** `Guard.txt` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

Exigência do guarda por slot de 0 a 7, com oito números posicionais. A tabela não nomeia as oito
posições: o bloco do `Guard.txt` não tem semântica provada, e inventar nome seria promover hipótese a
fato.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `guard_index` | `SMALLINT` | não | — | guarda dono da exigência (FK) |
| `req_slot` | `SMALLINT` | não | — | posição da exigência no bloco |
| `req_values` | `SMALLINT[]` | não | — | oito números posicionais, sem nome por posição |

## Chave primária

`pk_guard_spawn_reward (guard_index, req_slot)` — a posição da exigência é a chave; cada guarda tem no
máximo uma linha por slot.

## FKs de saída e de entrada

- **Saída, `CASCADE`:** `fk_guard_spawn_reward_guarda`:
  `guard_index → content.guard_spawn (guard_index)`, `ON UPDATE RESTRICT`, `ON DELETE CASCADE` — apagar
  o guarda leva as exigências junto.
- **Entrada:** nenhuma.

## Unicidade

Só a PK.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_guard_spawn_reward_slot` | `req_slot` fora de 0–7 |
| `ck_guard_spawn_reward_valores` | array com tamanho diferente de 8, ou com `NULL` em alguma posição |

## Índices

Nenhum além da PK — a leitura é sempre pelo guarda. Sem carimbos e sem trigger: import puro.

## Armadilha de leitura

O tamanho fixo de oito **não** é garantido para o array vazio. A expressão é
`array_length(req_values, 1) = 8 AND array_position(req_values, NULL) IS NULL`; com `'{}'`,
`array_length` devolve `NULL`, o predicado inteiro avalia nulo, e `CHECK` só recusa `false`. É a mesma
armadilha registrada em `game_matrix`: array sem nenhum elemento passa.

Os oito números são posicionais: o significado de cada posição é `H`, declarado em
`migrations/0019_content_world.md` §9.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed (`0020`); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` (default privilege de `content`, `0001_bootstrap.md` §7).
- **Não lê:** `wyd_admin` (`0032` não concede `content`). Dono da permissão:
  `migrations/0019_content_world.md` §8.

## Migration

Forma (dona): `migrations/0019_content_world.md` — arquivo `0019_content_world.sql`.
