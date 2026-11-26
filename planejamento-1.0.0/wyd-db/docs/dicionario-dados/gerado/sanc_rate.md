> Pergunta: "o que `content.sanc_rate` guarda por pó e índice de refino e o que significa uma célula ausente?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.22; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Chance de SANC por tipo de pó (`PO`, `PL`, `AMAGO`) e índice de refino, em pontos-base
(`rate_bps`, 0–10000). Tabela **seed-only e read-only em runtime**: só recebe célula com fonte explícita
— default compilado do cliente não vira seed. A ausência de linha é ausência de taxa publicada, e
operação muda o valor por `content.sanc_rate_override` (`0022`), não por `UPDATE` aqui.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `powder` | `TEXT` | não | — | tipo de pó, `PO`/`PL`/`AMAGO`; PK com `refine_index` |
| `refine_index` | `SMALLINT` | não | — | índice do refino, 0–15 |
| `rate_bps` | `INTEGER` | não | — | chance em pontos-base, 0–10000 |

Sem `created_at`/`updated_at` e sem trigger: base de seed, sem escrita do staff em runtime
(`migrations/0021_content_rules.md` §7).

## Chave primária

`pk_sanc_rate (powder, refine_index)` — o par identifica a célula; não há `id` próprio (`P-04`).

## FKs de saída e de entrada

- **Saída:** nenhuma; o schema `content` não aponta para `game`/`portal`.
- **Entrada (1 FK):** `fk_sanc_rate_override_base` em `content.sanc_rate_override` (`0022`),
  `ON UPDATE RESTRICT`, `ON DELETE CASCADE`. Apagar a base apaga o override, como nas outras duas bases
  de `0021`; as FKs de catálogo é que são `RESTRICT`.

## Unicidade

Só a PK: o mesmo pó não repete índice, e pós diferentes usam o mesmo índice sem colidir.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_sanc_rate_powder` | `powder` fora de `('PO', 'PL', 'AMAGO')` |
| `ck_sanc_rate_index` | `refine_index` fora de 0–15 |
| `ck_sanc_rate_rate` | `rate_bps` fora de 0–10000 |

`rate_bps = 0` é aceito e significa **0% medido**; não existir linha é outra coisa. Nada obriga os três
pós a cobrirem os 16 índices.

## Índices

Nenhum além do índice da PK, que cobre a busca por `(powder, refine_index)` e a FK do override.

## Armadilha de leitura

O intervalo daqui é **0–15**, enquanto `content.refine_rate.step` vai de **1 a 15**: os dois "passos" não
são o mesmo domínio, e comparar tabela a tabela pelo número cru erra a linha. `powder` é conjunto
fechado **proposto**: os três valores vêm da ficha do legado (`H`), não de enum verificado no cliente.
E o override de `0022` tem precedência: ler só esta tabela ignora a operação.

## Papéis

- **Escreve:** `wyd_migrator`, no seed (`0023`), que está `bloqueada`; nenhum `GRANT` de escrita em
  runtime (`migrations/0021_content_rules.md` §7).
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001_bootstrap.md` §7).
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032_admin_grants.md` §5).

## Migration

Forma (dona): `migrations/0021_content_rules.md` — arquivo `0021_content_rules.sql`.
