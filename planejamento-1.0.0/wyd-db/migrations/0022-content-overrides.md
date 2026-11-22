> Pergunta: "como o staff edita, e desfaz, as regras de refino, SANC e composição sem tocar no seed?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3 (linha 22), `ddl-confirmado.md` §5.23 (confirmado em 2026-11-21), fichas legadas de `0037_config_editavel.sql`
> Revisado em: 2026-11-21

Vigésimo segundo arquivo da série: 3 tabelas no schema `content`. As fichas de coluna vêm das fontes
históricas de configuração editável (nível 4); são **hipótese `H`** até validação própria, não colunas
provadas por aparecerem no legado.

!!! note "Aprovado em 2026-11-21"
    O dono aprovou o conjunto em 2026-11-21 ("Sim, aprovar como está"), sobre as decisões de forma de
    §8. O lote está `concluída` em `migrations-checklist.md` §2 e as três tabelas contam entre as 109 com
    DDL fechado.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0022` (checklist, linha 106) |
| Arquivo | `crates/wyd-db/migrations/0022_content_overrides.sql` |
| Schema | `content` (3 tabelas) |
| Posição | 22 de 32 |
| Depende de | `0001` (schema, privilégios); `0021` (`content.refine_rate`, `content.sanc_rate`, `content.combine_value`) |
| É dependência de | `0023` (seed das regras) |
| Tabelas criadas | `refine_rate_override`, `sanc_rate_override`, `combine_value_override` |
| Objetos criados | 3 tabelas, 3 FKs, 0 índices secundários, 3 `GRANT` |
| `SCHEMA_VERSION` | Continua `53` até a troca da série (`plano-migrations.md` §6) |

## 2. Objetivo

Guardar a edição do staff sobre três famílias de regras sem reescrever o seed: o loader lê
`COALESCE(override, base)`, e apagar a linha de override devolve o valor base. As bases são seed-only
e read-only em runtime (`0021`); esta é a única camada de `content.rules` que o portal escreve.

## 3. Pré-requisitos

1. `0001_bootstrap`: schema `content` e default privileges (só `SELECT` para `wyd_game`, `wyd_portal`, `wyd_read`).
2. `0021_content_rules`: `content.refine_rate(kind, step)`, `content.sanc_rate(powder, refine_index)`, `content.combine_value(family, key)`.
3. `ops.config_revision`/`ops.config_change` (`0002`) para autoria e antes/depois da edição — não são FK deste lote.
4. Aplicação posterior de `0023` respeita a ordem da série; nenhuma tabela posterior é FK deste lote.

## 4. DDL do `up`

Texto confirmado em `ddl-confirmado.md` §5.23, copiado sem alteração.

```sql
-- 0022_content_overrides: edição do staff sobre refino, SANC e composição.
-- Forma proposta a partir das fichas do legado em deprecated/.agents/prompts/análises/wyd-db/database/tabelas/;
-- toda semântica histórica não explicitada no planejamento novo permanece H, nunca fato por cópia.
-- Não usa COMMENT ON: a série documenta semântica nas páginas de migration.
-- Override sem linha base é impossível; apagar a linha de override é o reset para o valor da base.

-- migrate:up
CREATE TABLE content.refine_rate_override (
    kind       TEXT NOT NULL,
    step       SMALLINT NOT NULL,
    rate_bps   INTEGER NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_refine_rate_override PRIMARY KEY (kind, step),
    CONSTRAINT fk_refine_rate_override_base FOREIGN KEY (kind, step)
        REFERENCES content.refine_rate (kind, step) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT ck_refine_rate_override_rate CHECK (rate_bps BETWEEN 0 AND 10000)
);

CREATE TABLE content.sanc_rate_override (
    powder       TEXT NOT NULL,
    refine_index SMALLINT NOT NULL,
    rate_bps     INTEGER NOT NULL,
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_sanc_rate_override PRIMARY KEY (powder, refine_index),
    CONSTRAINT fk_sanc_rate_override_base FOREIGN KEY (powder, refine_index)
        REFERENCES content.sanc_rate (powder, refine_index) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT ck_sanc_rate_override_rate CHECK (rate_bps BETWEEN 0 AND 10000)
);

-- Same typed value as the base table: exactly one of rate_bps/coin, checked here too (FK does not
-- constrain values). Per-row replacement: a row here replaces the whole base row for its key.
CREATE TABLE content.combine_value_override (
    family     TEXT NOT NULL,
    key        TEXT NOT NULL,
    rate_bps   INTEGER,
    coin       BIGINT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_combine_value_override PRIMARY KEY (family, key),
    CONSTRAINT fk_combine_value_override_base FOREIGN KEY (family, key)
        REFERENCES content.combine_value (family, key) ON UPDATE RESTRICT ON DELETE CASCADE,
    CONSTRAINT ck_combine_value_override_one_unit CHECK (
        (rate_bps IS NOT NULL AND coin IS NULL AND rate_bps BETWEEN 0 AND 10000)
        OR (rate_bps IS NULL AND coin IS NOT NULL AND coin >= 0)
    )
);

-- 0001 grants only SELECT on `content`. The portal admin area is the only runtime that edits
-- configuration (M21, §5.9): DELETE here is the reset to the base value.
GRANT SELECT, INSERT, UPDATE, DELETE ON content.refine_rate_override TO wyd_portal;
GRANT SELECT, INSERT, UPDATE, DELETE ON content.sanc_rate_override TO wyd_portal;
GRANT SELECT, INSERT, UPDATE, DELETE ON content.combine_value_override TO wyd_portal;
```

## 5. DDL do `down`

```sql
-- migrate:down
REVOKE SELECT, INSERT, UPDATE, DELETE ON content.combine_value_override FROM wyd_portal;
REVOKE SELECT, INSERT, UPDATE, DELETE ON content.sanc_rate_override FROM wyd_portal;
REVOKE SELECT, INSERT, UPDATE, DELETE ON content.refine_rate_override FROM wyd_portal;
DROP TABLE content.combine_value_override;
DROP TABLE content.sanc_rate_override;
DROP TABLE content.refine_rate_override;
```

## 6. Colunas

Origem `H` = hipótese derivada da ficha do legado; `P` = provada por fonte do planejamento novo.
Nenhuma coluna deste lote é `P`: a base de comparação (`0021`) também é `H`.

### `content.refine_rate_override`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `kind` | `TEXT` | não | `H` | Espelha `refine_rate.kind`; a PK da base inclui a família, então o override também precisa dela. |
| `step` | `SMALLINT` | não | `H` | Espelha `refine_rate.step`; faixa 1–15 vem do CHECK/FK da base, não se repete aqui. |
| `rate_bps` | `INTEGER` | não | `H` | Chance editada em bps, mesma unidade da base (§5.22). |
| `updated_at` | `TIMESTAMPTZ` | não | `H` | Marcador barato para o loader; autoria e histórico ficam em `ops.config_change`. |

### `content.sanc_rate_override`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `powder` | `TEXT` | não | `H` | Espelha `sanc_rate.powder`; o conjunto `PO`/`PL`/`AMAGO` é garantido pela FK à base. |
| `refine_index` | `SMALLINT` | não | `H` | Espelha `sanc_rate.refine_index`; faixa 0–15 vem da base. |
| `rate_bps` | `INTEGER` | não | `H` | Chance de SANC editada em bps; o `percent` do legado não é copiado. |
| `updated_at` | `TIMESTAMPTZ` | não | `H` | Mesmo papel do carimbo nas outras duas. |

### `content.combine_value_override`

| Coluna | Tipo | Nulo | Origem | Por quê |
|---|---|---|---|---|
| `family` | `TEXT` | não | `H` | Espelha `combine_value.family`. |
| `key` | `TEXT` | não | `H` | Espelha `combine_value.key`. |
| `rate_bps` | `INTEGER` | sim | `H` | Valor em bps quando a edição é de taxa; exatamente uma das duas colunas de valor. |
| `coin` | `BIGINT` | sim | `H` | Custo em unidade menor quando a edição é de moeda; exatamente uma das duas. |
| `updated_at` | `TIMESTAMPTZ` | não | `H` | Mesmo papel do carimbo nas outras duas. |

Não há `updated_by`: atribuição de staff é `ops.config_revision`/`config_change` (`0002`, §5.9), e a
série proíbe FK de `content` para `portal`. Não há `content_source`/`notes` por linha: proveniência de
dataset fica em `0019` e `H` fica nesta página, como decidido em §5.22.

## 7. Constraints e índices

- PKs naturais iguais às da base: `(kind, step)`, `(powder, refine_index)`, `(family, key)`.
- FKs compostas para a base com `ON DELETE CASCADE` e `ON UPDATE RESTRICT`: override sem base é
  impossível; remover a base limpa a edição; alterar a chave da base não é operação prevista.
- CHECKs só de valor, porque a FK já garante o domínio da chave: `rate_bps` 0–10000 nas duas tabelas
  de taxa e o “exatamente um” de `rate_bps`/`coin` em `combine_value_override`.
- Índices: nenhum. As PKs cobrem as FKs; não existe coluna de busca secundária.
- `GRANT SELECT, INSERT, UPDATE, DELETE` em cada tabela para `wyd_portal`; nada novo para
  `wyd_game`/`wyd_read`.

## 8. Decisões respondidas em 2026-11-20

1. Carimbo de edição: só `updated_at`; autoria em `ops.config_change`, sem `updated_by`.
2. FK composta para a base, `ON DELETE CASCADE`, em vez de tabela raiz sem FK.
3. `sanc_rate_override` em `rate_bps` 0–10000, igual à base; não `percent` 0–100 do legado.
4. `combine_value_override` com `rate_bps`/`coin` e CHECK de exatamente um, espelhando a base.
5. Escrita: `GRANT SELECT, INSERT, UPDATE, DELETE` a `wyd_portal` (área administrativa, `M21`).
6. Reset: `DELETE` da linha, sem tombstone; histórico em `ops`.

## 9. Prova prevista (não executada)

1. PK duplicada em cada tabela → `23505`.
2. Override com chave ausente na base (`step = 99`, `powder = 'XX'`, família inexistente) → `23503`.
3. `DELETE` na linha base referenciada → linha de override vai junto, sem erro.
4. `rate_bps` < 0 ou > 10000 → `23514`.
5. Ambas ou nenhuma das colunas de valor em `combine_value_override` → `23514`.
6. `wyd_game`/`wyd_read` tenta escrever → `42501`; `wyd_portal` escreve → sucesso.
7. `down` revoga os privilégios e derruba as tabelas sem erro de dependência.

## 10. O que não decide

| Assunto | Dono/fechamento |
|---|---|
| Quais linhas o staff edita em produção e com que frequência | operação do portal; sem fonte no planejamento |
| Convenção de `target_key` para chave composta em `ops.config_change` | `portal`/`ops`; este lote não a fixa |
| `COALESCE(override, base)` e invalidação de cache no runtime | contrato do `wyd-sim`/`wyd-server` |
| Valores dos seeds das regras base | `0023`, só com fontes verificadas |
| Reuso deste padrão em `0025` (`drop_rate_override`, `content_rate_override`) | lote próprio, decisão própria |

*Página de planejamento. Nenhum SQL foi executado nem arquivo `.sql` criado.*
