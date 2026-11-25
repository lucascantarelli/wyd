> Pergunta: "o que a edição do staff sobre `content.refine_rate` guarda e como o reset devolve a base?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.23; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Substituição editável da linha de `content.refine_rate` para a mesma chave `(kind, step)`. Uma linha
aqui vale **no lugar** da base, não como delta; apagar a linha é o reset para o valor base. É a única
camada de `content` que a área administrativa do portal escreve nesta família (`M21`, `0022`).

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `kind` | `TEXT` | não | — | espelha `refine_rate.kind`; PK com `step` |
| `step` | `SMALLINT` | não | — | espelha `refine_rate.step`; PK; faixa vem da FK à base |
| `rate_bps` | `INTEGER` | não | — | taxa editada em pontos-base, 0–10000 |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbo da linha; sem trigger neste lote |

`updated_at` só recebe `now()` no `INSERT`: o lote não cria trigger `BEFORE UPDATE`, então um `UPDATE`
que não informe a coluna mantém o instante anterior. Autoria e valor anterior ficam em
`ops.config_change`, não aqui.

## Chave primária

`pk_refine_rate_override (kind, step)` — exatamente a PK da base; uma edição por regra, sem `id`.

## FKs de saída e de entrada

- **Saída (1 FK):** `fk_refine_rate_override_base`: `(kind, step) → content.refine_rate (kind, step)`,
  `ON UPDATE RESTRICT`, `ON DELETE CASCADE`. Override de chave inexistente é impossível; apagar a base
  leva a edição junto.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para o override.

## Unicidade

Só a PK: a mesma chave não tem duas edições concorrentes.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_refine_rate_override_rate` | `rate_bps` fora de 0–10000 |

`kind`/`step` não têm `CHECK` próprio: a FK composta garante que a chave existe na base, e a base já
limita família e passo. O `CHECK` de valor não é herdado pela FK e precisa existir aqui.

## Índices

Nenhum além do índice da PK: a própria PK cobre a FK composta e a leitura por `(kind, step)`. Não há
busca secundária nem índice em `updated_at`.

## Armadilha de leitura

O override substitui a **linha inteira**, não coluna a coluna: não existe `COALESCE` por campo, e uma
edição de `rate_bps` mantém a chave da base, não uma composição das duas linhas. Apagar a base apaga o
override por `CASCADE`, então “sumiu a edição” pode ser consequência de um seed novo. E `updated_at`
sem trigger não é prova de autoria: quem editou está em `ops.config_change`.

## Papéis

- **Escreve:** `wyd_portal`, com `SELECT, INSERT, UPDATE, DELETE` concedidos por `0022` para a área
  administrativa (`M21`). `DELETE` é o reset para a base.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001-bootstrap.md` §7); o `GRANT` de `0022` cobre `wyd_portal`.
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032-admin-grants.md` §5).

## Migration

Forma (dona): `migrations/0022-content-overrides.md` — arquivo `0022-content-overrides.sql`.
