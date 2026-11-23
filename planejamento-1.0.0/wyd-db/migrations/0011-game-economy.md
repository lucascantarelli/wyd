> Pergunta: "como aplico o `0011_game_economy` e como cidades, preços, loja pessoal, imposto e o cash do portal passam a existir?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3; `ddl-confirmado.md` §5.14 (confirmado); `padrao-banco.md` §1/§5/§9/§11; `lacunas.md` (`A19`, `M17`)
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-23)

Décimo primeiro arquivo. Cria a economia: quatro tabelas em `game` (cidade, preço por mercador, loja
pessoal e livro do imposto) e seis em `portal` (loja de donate, recompensa diária, resgate diário,
preço da capa de reino e os dois razões de crédito/débito). `A19` fixou essa divisão: dinheiro real e
cash ficam no `portal`; preço e imposto de jogo, em `game`.

!!! note "Aprovado em 2026-11-20"
    O dono aprovou o lote na rodada de perguntas de 2026-11-20, na opção "Sim, aprovar como está",
    depois de escolher as quatro formas de §10. O DDL abaixo é o confirmado em `ddl-confirmado.md`
    §5.14, as dez tabelas entram nas que têm DDL fechado (`ddl-confirmado.md` §1) e este arquivo está
    `concluída` no checklist.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0011` |
| Arquivo | `crates/wyd-db/migrations/0011_game_economy.sql` |
| Schema | `game` (4 tabelas) + `portal` (6 tabelas) |
| Posição | 11 de 34 (economia e cash) |
| Depende de | `0001` (schemas, `ops.fn_set_updated_at`, default privileges), `0005` (conta citada por valor), `0006` (`game.character`, dono da loja e do imposto) e `0010` concluída não é pré-requisito |
| É dependência de | `0012_ops_delivery` (entrega de item comprado), `0029_views` (leitura administrativa) |
| Tabelas criadas | 10 (`city`, `item_price`, `personal_shop`, `city_tax_ledger` em `game`; `donate_shop_item`, `daily_reward_item`, `daily_reward_claim`, `kingdom_cape_price`, `donate_credit`, `donate_debit` em `portal`) |
| Objetos criados | 10 tabelas, 8 índices (2 parciais), 6 triggers |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca de série (`plano-migrations.md` §6) |

## 2. Objetivo

Guardar a configuração econômica por cidade e por mercador, a loja pessoal com seu imposto e o livro
do que foi recolhido, além das tabelas de cash do portal: o que a loja de donate vende, a recompensa
diária e seu resgate único, o preço da capa por reino e os razões idempotentes de crédito e débito.

## 3. Pré-requisitos

1. `0001_bootstrap` aplicado: schemas, `ops.fn_set_updated_at()` e default privileges.
2. `0006_game_character` aplicado: `game.character` existe (FK da loja e do livro de imposto).
3. Personagem de prova criado; sem ele, a prova 6 falha com `23503` no lugar errado.
4. Conexão como `wyd_migrator`.

## 4. DDL do `up`

Texto de `ddl-confirmado.md` §5.14, copiado sem alteração.

```sql
-- migrate:up
CREATE TABLE game.city (
    code          TEXT NOT NULL,
    name          TEXT NOT NULL,
    -- Mapa da cidade não é publicado pelo cliente 7662 (`H`); fica nulo até prova de trace.
    map_id        SMALLINT,
    is_capital    BOOLEAN NOT NULL DEFAULT false,
    tax_rate_bps  INTEGER NOT NULL DEFAULT 1000,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_city PRIMARY KEY (code),
    CONSTRAINT ck_city_code CHECK (code ~ '^[a-z][a-z0-9_]{2,15}$'),
    CONSTRAINT ck_city_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 32),
    CONSTRAINT ck_city_mapa CHECK (map_id IS NULL OR map_id BETWEEN 0 AND 255),
    CONSTRAINT ck_city_imposto CHECK (tax_rate_bps BETWEEN 0 AND 10000)
);
CREATE TRIGGER trg_city_set_updated_at
    BEFORE UPDATE ON game.city
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.item_price (
    item_index INTEGER NOT NULL,
    npc_id     INTEGER NOT NULL,
    buy_price  BIGINT NOT NULL DEFAULT 0,
    sell_price BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_item_price PRIMARY KEY (item_index, npc_id),
    CONSTRAINT ck_item_price_compra CHECK (buy_price >= 0),
    CONSTRAINT ck_item_price_venda CHECK (sell_price >= 0)
);

-- Sem FK para `content.item_catalog`/`content.npc_definition` (0014/0019): conteúdo é posterior.
CREATE INDEX ix_item_price_npc ON game.item_price (npc_id);
CREATE TRIGGER trg_item_price_set_updated_at
    BEFORE UPDATE ON game.item_price
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.personal_shop (
    id           BIGINT GENERATED ALWAYS AS IDENTITY,
    character_id BIGINT NOT NULL,
    city_code    TEXT NOT NULL,
    name         TEXT NOT NULL,
    is_open      BOOLEAN NOT NULL DEFAULT true,
    tax_bps      INTEGER NOT NULL DEFAULT 1000,
    tax_paid     BIGINT NOT NULL DEFAULT 0,
    opened_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at    TIMESTAMPTZ,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_personal_shop PRIMARY KEY (id),
    CONSTRAINT fk_personal_shop_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT fk_personal_shop_city FOREIGN KEY (city_code)
        REFERENCES game.city (code) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT ck_personal_shop_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 24),
    CONSTRAINT ck_personal_shop_imposto CHECK (tax_bps BETWEEN 0 AND 10000),
    CONSTRAINT ck_personal_shop_pago CHECK (tax_paid >= 0),
    CONSTRAINT ck_personal_shop_fechamento CHECK (closed_at IS NULL OR closed_at >= opened_at),
    -- Aberta não tem data de fechamento; fechada tem.
    CONSTRAINT ck_personal_shop_estado CHECK (is_open = (closed_at IS NULL))
);

-- Uma loja aberta por personagem e a busca por cidade são os dois acessos quentes.
CREATE UNIQUE INDEX ux_personal_shop_aberta ON game.personal_shop (character_id) WHERE is_open;
CREATE INDEX ix_personal_shop_cidade ON game.personal_shop (city_code) WHERE is_open;
CREATE TRIGGER trg_personal_shop_set_updated_at
    BEFORE UPDATE ON game.personal_shop
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.city_tax_ledger (
    id          BIGINT GENERATED ALWAYS AS IDENTITY,
    city_code   TEXT NOT NULL,
    shop_id     BIGINT,
    amount      BIGINT NOT NULL,
    reason      TEXT NOT NULL,
    collected_by BIGINT,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_city_tax_ledger PRIMARY KEY (id),
    CONSTRAINT fk_city_tax_ledger_city FOREIGN KEY (city_code)
        REFERENCES game.city (code) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT fk_city_tax_ledger_shop FOREIGN KEY (shop_id)
        REFERENCES game.personal_shop (id) ON DELETE SET NULL ON UPDATE NO ACTION,
    CONSTRAINT fk_city_tax_ledger_quem FOREIGN KEY (collected_by)
        REFERENCES game.character (id) ON DELETE SET NULL ON UPDATE NO ACTION,
    CONSTRAINT ck_city_tax_ledger_valor CHECK (amount >= 0),
    CONSTRAINT ck_city_tax_ledger_motivo CHECK (
        reason IN ('shop_sale', 'npc_sale', 'portal', 'collection')
    )
);

CREATE INDEX ix_city_tax_ledger_cidade ON game.city_tax_ledger (city_code, occurred_at DESC);

CREATE TABLE portal.donate_shop_item (
    id            BIGINT GENERATED ALWAYS AS IDENTITY,
    item_index    INTEGER NOT NULL,
    price_credits INTEGER NOT NULL,
    duration_days SMALLINT,
    enabled       BOOLEAN NOT NULL DEFAULT true,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_donate_shop_item PRIMARY KEY (id),
    CONSTRAINT ck_donate_shop_item_preco CHECK (price_credits > 0),
    CONSTRAINT ck_donate_shop_item_duracao CHECK (duration_days IS NULL OR duration_days > 0)
);
CREATE TRIGGER trg_donate_shop_item_set_updated_at
    BEFORE UPDATE ON portal.donate_shop_item
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.daily_reward_item (
    id           BIGINT GENERATED ALWAYS AS IDENTITY,
    day          SMALLINT,
    item_index   INTEGER,
    gold         BIGINT NOT NULL DEFAULT 0,
    qty          SMALLINT NOT NULL DEFAULT 1,
    account_wide BOOLEAN NOT NULL DEFAULT true,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_daily_reward_item PRIMARY KEY (id),
    CONSTRAINT ck_daily_reward_item_dia CHECK (day IS NULL OR day BETWEEN 1 AND 7),
    CONSTRAINT ck_daily_reward_item_ouro CHECK (gold >= 0),
    CONSTRAINT ck_daily_reward_item_qtd CHECK (qty > 0),
    -- Recompensa sem item e sem ouro não entrega nada.
    CONSTRAINT ck_daily_reward_item_conteudo CHECK (item_index IS NOT NULL OR gold > 0)
);
CREATE TRIGGER trg_daily_reward_item_set_updated_at
    BEFORE UPDATE ON portal.daily_reward_item
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.daily_reward_claim (
    id             BIGINT GENERATED ALWAYS AS IDENTITY,
    account_id     BIGINT NOT NULL,
    reward_item_id BIGINT,
    claim_date     DATE NOT NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_daily_reward_claim PRIMARY KEY (id),
    -- Um resgate por conta e dia UTC: o `23505` é o sinal de "já resgatou".
    CONSTRAINT uq_daily_reward_claim_dia UNIQUE (account_id, claim_date),
    CONSTRAINT fk_daily_reward_claim_reward FOREIGN KEY (reward_item_id)
        REFERENCES portal.daily_reward_item (id) ON DELETE RESTRICT ON UPDATE NO ACTION
);

CREATE TABLE portal.kingdom_cape_price (
    id            SMALLINT NOT NULL DEFAULT 1,
    version       BIGINT NOT NULL DEFAULT 1,
    hekalotia_cost INTEGER NOT NULL DEFAULT 8,
    akelonia_cost  INTEGER NOT NULL DEFAULT 8,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_kingdom_cape_price PRIMARY KEY (id),
    CONSTRAINT ck_kingdom_cape_price_singleton CHECK (id = 1),
    CONSTRAINT ck_kingdom_cape_price_versao CHECK (version >= 1),
    CONSTRAINT ck_kingdom_cape_price_hekalotia CHECK (hekalotia_cost BETWEEN 4 AND 32),
    CONSTRAINT ck_kingdom_cape_price_akelonia CHECK (akelonia_cost BETWEEN 4 AND 32)
);
CREATE TRIGGER trg_kingdom_cape_price_set_updated_at
    BEFORE UPDATE ON portal.kingdom_cape_price
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.donate_credit (
    id              BIGINT GENERATED ALWAYS AS IDENTITY,
    idempotency_key UUID NOT NULL,
    account_id      BIGINT NOT NULL,
    credits         BIGINT NOT NULL,
    created_by      BIGINT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_donate_credit PRIMARY KEY (id),
    CONSTRAINT uq_donate_credit_idem UNIQUE (idempotency_key),
    CONSTRAINT ck_donate_credit_valor CHECK (credits > 0)
);

CREATE TABLE portal.donate_debit (
    id              BIGINT GENERATED ALWAYS AS IDENTITY,
    idempotency_key UUID NOT NULL,
    account_id      BIGINT NOT NULL,
    credits         BIGINT NOT NULL,
    reason          TEXT NOT NULL DEFAULT 'portal',
    created_by      BIGINT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_donate_debit PRIMARY KEY (id),
    CONSTRAINT uq_donate_debit_idem UNIQUE (idempotency_key),
    CONSTRAINT ck_donate_debit_valor CHECK (credits > 0),
    CONSTRAINT ck_donate_debit_motivo CHECK (reason IN ('portal', 'shop_purchase'))
);

CREATE INDEX ix_donate_debit_conta ON portal.donate_debit (account_id, created_at DESC);
```

## 5. DDL do `down`

Ordem inversa: cash do portal primeiro, depois as tabelas de `game`, com o livro do imposto antes da
loja e da cidade.

```sql
-- migrate:down
DROP INDEX portal.ix_donate_debit_conta;
DROP TABLE portal.donate_debit;
DROP TABLE portal.donate_credit;
DROP TRIGGER trg_kingdom_cape_price_set_updated_at ON portal.kingdom_cape_price;
DROP TABLE portal.kingdom_cape_price;
DROP TABLE portal.daily_reward_claim;
DROP TRIGGER trg_daily_reward_item_set_updated_at ON portal.daily_reward_item;
DROP TABLE portal.daily_reward_item;
DROP TRIGGER trg_donate_shop_item_set_updated_at ON portal.donate_shop_item;
DROP TABLE portal.donate_shop_item;
DROP INDEX game.ix_city_tax_ledger_cidade;
DROP TABLE game.city_tax_ledger;
DROP TRIGGER trg_personal_shop_set_updated_at ON game.personal_shop;
DROP INDEX game.ix_personal_shop_cidade;
DROP INDEX game.ux_personal_shop_aberta;
DROP TABLE game.personal_shop;
DROP TRIGGER trg_item_price_set_updated_at ON game.item_price;
DROP INDEX game.ix_item_price_npc;
DROP TABLE game.item_price;
DROP TRIGGER trg_city_set_updated_at ON game.city;
DROP TABLE game.city;
```

## 6. Tabelas de colunas

### 6.1 `game.city`

| Coluna | Tipo | Nulo | Default | O que é / origem |
|---|---|---|---|---|
| `code` | `TEXT` | não | — | PK legível (`noatun`, `hekalotia`), `H` do legado `city.code`. |
| `name` | `TEXT` | não | — | Nome exibido, 1–32 (`H`). |
| `map_id` | `SMALLINT` | sim | — | Mapa da cidade; nulo até prova de trace (`H`, legado marcava UNVERIFIED). |
| `is_capital` | `BOOLEAN` | não | `false` | Capital de reino (`H`). |
| `tax_rate_bps` | `INTEGER` | não | `1000` | Imposto da loja pessoal em bps, 0–10000 (`H`, 1000 = 10%). |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Marcas com trigger (`A9`). |

### 6.2 `game.item_price`

| Coluna | Tipo | Nulo | Default | O que é / origem |
|---|---|---|---|---|
| `item_index` | `INTEGER` | não | — | Item do cliente (`H`); catálogo em `0014`. |
| `npc_id` | `INTEGER` | não | — | Mercador (`H`); definição em `0019`. |
| `buy_price` | `BIGINT` | não | `0` | Preço de compra, não negativo (`H`). |
| `sell_price` | `BIGINT` | não | `0` | Preço de venda, não negativo (`H`). |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Marcas com trigger. |

### 6.3 `game.personal_shop`

| Coluna | Tipo | Nulo | Default | O que é / origem |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | Identificador da loja. |
| `character_id` | `BIGINT` | não | — | Dono; FK `CASCADE`. |
| `city_code` | `TEXT` | não | — | Cidade onde a loja está; FK `RESTRICT`. |
| `name` | `TEXT` | não | — | Nome da loja, 1–24. |
| `is_open` | `BOOLEAN` | não | `true` | Loja aberta; coerente com `closed_at`. |
| `tax_bps` | `INTEGER` | não | `1000` | Imposto aplicado, 0–10000 (`H`). |
| `tax_paid` | `BIGINT` | não | `0` | Imposto já recolhido, não negativo (`H`). |
| `opened_at` / `closed_at` | `TIMESTAMPTZ` | não / sim | `now()` / — | Abertura e fechamento (`H`). |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Marcas com trigger. |

### 6.4 `game.city_tax_ledger`

| Coluna | Tipo | Nulo | Default | O que é / origem |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | Lançamento. |
| `city_code` | `TEXT` | não | — | Cidade que recebeu; FK `RESTRICT`. |
| `shop_id` | `BIGINT` | sim | — | Loja de origem; FK `SET NULL` (histórico sobrevive). |
| `amount` | `BIGINT` | não | — | Valor recolhido, não negativo (`H`). |
| `reason` | `TEXT` | não | — | `shop_sale`, `npc_sale`, `portal` ou `collection` (`H`). |
| `collected_by` | `BIGINT` | sim | — | Personagem que recolheu; FK `SET NULL`. |
| `occurred_at` | `TIMESTAMPTZ` | não | `now()` | Instante do lançamento (`H`, legado `at`). |

### 6.5 `portal.donate_shop_item`

| Coluna | Tipo | Nulo | Default | O que é / origem |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | Oferta da loja. |
| `item_index` | `INTEGER` | não | — | Item vendido (`H`); catálogo em `0014`. |
| `price_credits` | `INTEGER` | não | — | Preço em créditos, > 0 (`H`). |
| `duration_days` | `SMALLINT` | sim | — | Duração do item, positiva ou nula (`H`). |
| `enabled` | `BOOLEAN` | não | `true` | Oferta ativa (`H`). |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Marcas com trigger. |

### 6.6 `portal.daily_reward_item`

| Coluna | Tipo | Nulo | Default | O que é / origem |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | Linha de recompensa. |
| `day` | `SMALLINT` | sim | — | Dia 1–7 ou nulo para "todo dia" (`H`). |
| `item_index` | `INTEGER` | sim | — | Item entregue (`H`); catálogo em `0014`. |
| `gold` | `BIGINT` | não | `0` | Ouro entregue, não negativo (`H`). |
| `qty` | `SMALLINT` | não | `1` | Quantidade do item, > 0 (`H`). |
| `account_wide` | `BOOLEAN` | não | `true` | Recompensa por conta, não por personagem (`H`). |

### 6.7 `portal.daily_reward_claim`, `portal.kingdom_cape_price` e razões

| Tabela.coluna | Tipo | Nulo | Default | O que é / origem |
|---|---|---|---|---|
| `daily_reward_claim.account_id` | `BIGINT` | não | — | Conta que resgatou (`H`); sem FK por fronteira. |
| `daily_reward_claim.reward_item_id` | `BIGINT` | sim | — | Linha da recompensa; FK `RESTRICT`. |
| `daily_reward_claim.claim_date` | `DATE` | não | — | Dia UTC do resgate; único com a conta (`H`). |
| `kingdom_cape_price.id` | `SMALLINT` | não | `1` | Linha única. |
| `kingdom_cape_price.version` | `BIGINT` | não | `1` | Versão para concorrência otimista (`H`). |
| `kingdom_cape_price.hekalotia_cost` / `akelonia_cost` | `INTEGER` | não | `8` | Custo por reino, 4–32 (`H`). |
| `donate_credit.idempotency_key` / `donate_debit.idempotency_key` | `UUID` | não | — | Chave única do BFF (`M17`). |
| `donate_credit.account_id` / `donate_debit.account_id` | `BIGINT` | não | — | Conta citada por valor, sem FK (`P-17`). |
| `donate_credit.credits` / `donate_debit.credits` | `BIGINT` | não | — | Crédito/débito > 0 (`H`). |
| `donate_credit.created_by` / `donate_debit.created_by` | `BIGINT` | sim | — | Autor staff; valor de pessoa/`portal`, sem FK. |
| `donate_debit.reason` | `TEXT` | não | `'portal'` | `portal` ou `shop_purchase` (`H`). |

## 7. Contratos e invariantes

| # | Objeto | Estado que impede | SQLSTATE |
|---:|---|---|---|
| 1 | `pk_city` / `ck_city_code` | código repetido ou fora de `snake_case` | `23505` / `23514` |
| 2 | `ck_city_imposto` / `ck_city_mapa` | imposto fora de 0–10000 ou mapa fora de 0–255 | `23514` |
| 3 | `pk_item_price` | dois preços para o mesmo item e mercador | `23505` |
| 4 | `ck_item_price_*` | preço negativo | `23514` |
| 5 | `ux_personal_shop_aberta` | segunda loja aberta do mesmo personagem | `23505` |
| 6 | `ck_personal_shop_estado` / `_fechamento` | `is_open` divergente de `closed_at`, ou fechamento antes da abertura | `23514` |
| 7 | `fk_personal_shop_*` | loja sem personagem ou sem cidade | `23503` |
| 8 | `ck_city_tax_ledger_valor` / `_motivo` | valor negativo ou motivo fora dos quatro | `23514` |
| 9 | `fk_city_tax_ledger_city` | lançamento em cidade inexistente | `23503` |
| 10 | `ck_donate_shop_item_*` | preço ≤ 0 ou duração ≤ 0 | `23514` |
| 11 | `ck_daily_reward_item_*` | dia fora de 1–7, quantidade ≤ 0 ou recompensa vazia | `23514` |
| 12 | `uq_daily_reward_claim_dia` | segundo resgate da mesma conta no mesmo dia | `23505` |
| 13 | `ck_kingdom_cape_price_*` | segunda linha, versão < 1 ou custo fora de 4–32 | `23514` |
| 14 | `uq_donate_credit_idem` / `uq_donate_debit_idem` | repetir a mesma chave de idempotência | `23505` |
| 15 | `ck_donate_*_valor` / `ck_donate_debit_motivo` | crédito/débito ≤ 0 ou motivo inventado | `23514` |

## 8. Permissões

Nenhum `GRANT` explícito. Default privileges do `0001`:

| Papel | Efeito | Origem |
|---|---|---|
| `wyd_game` | escreve e lê as 4 tabelas de `game`; **nada** em `portal` | default privilege + `P-17` |
| `wyd_portal` | escreve e lê as 6 tabelas de `portal`; **nada** em `game` | default privilege + `P-17` |
| `wyd_read` | `SELECT` nas dez tabelas | default privilege |
| `PUBLIC` | nada | `0001` |

## 9. Prova de execução

PG17 efêmero, com `0001` e `0006` aplicados e um personagem de prova (`game.character` id 1).

```bash
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0011_game_economy.sql
```

| # | Prova | Comando | Esperado |
|---:|---|---|---|
| 1 | dez tabelas | contar as dez em `game` e `portal` | `4` + `6` |
| 2 | código de cidade inválido | `INSERT INTO game.city (code, name) VALUES ('Noatun', 'Noatun');` | `23514` em `ck_city_code` |
| 3 | imposto acima do teto | `… tax_rate_bps = 20000` | `23514` em `ck_city_imposto` |
| 4 | preço negativo | `INSERT INTO game.item_price (item_index, npc_id, buy_price) VALUES (1, 1, -1);` | `23514` em `ck_item_price_compra` |
| 5 | preço repetido | dois `INSERT` em `(1, 1)` | `23505` em `pk_item_price` |
| 6 | loja de personagem inexistente | `INSERT INTO game.personal_shop (character_id, city_code, name) VALUES (99, 'noatun', 'Loja');` | `23503` em `fk_personal_shop_character` |
| 7 | segunda loja aberta | duas lojas abertas do personagem 1 | segundo `23505` em `ux_personal_shop_aberta` |
| 8 | aberta com fechamento | `is_open = true, closed_at = now()` | `23514` em `ck_personal_shop_estado` |
| 9 | imposto com motivo inventado | `reason = 'tax'` no livro | `23514` em `ck_city_tax_ledger_motivo` |
| 10 | recompensa vazia | `INSERT INTO portal.daily_reward_item DEFAULT VALUES;` | `23514` em `ck_daily_reward_item_conteudo` |
| 11 | dois resgates no dia | duas linhas com a mesma conta e data | segundo `23505` em `uq_daily_reward_claim_dia` |
| 12 | crédito repetido | duas linhas com o mesmo `idempotency_key` | segundo `23505` em `uq_donate_credit_idem` |
| 13 | crédito zero | `credits = 0` | `23514` em `ck_donate_credit_valor` |
| 14 | segunda linha de capa | dois `INSERT` em `kingdom_cape_price` | segundo `23505` em `pk_kingdom_cape_price` |
| 15 | fronteira | `SET ROLE wyd_game; SELECT count(*) FROM portal.donate_credit;` | `42501` |
| 16 | `down` derruba só o lote | aplicar o bloco `-- migrate:down` e repetir 1 | `0` + `0` |

## 10. O que este arquivo não decide

As quatro escolhas de forma foram decididas com o lote em 2026-11-20:

| # | Escolha | O que o lote fixa |
|---|---|---|
| 1 | Fronteira das seis de cash | `portal` conforme `A19`, com `account_id` **sem FK** para `game.account` e validação na borda |
| 2 | Idempotência (`M17`) | `UUID` nativo único por tabela; janela de 24 h é política do BFF |
| 3 | Preço por mercador | PK `(item_index, npc_id)` com `npc_id` obrigatório; preço de catálogo em `0014` |
| 4 | Loja pessoal | uma aberta por personagem (índice parcial) e `CASCADE` do personagem |

| Assunto | Onde é decidido |
|---|---|
| Catálogo de item, de NPC e de região | `0014`, `0019`, `0024` |
| Saldo de moeda | `0006` (`character.coin`) e `0005` (`account_wallet.coin`) |
| Entrega do item comprado | `0012_ops_delivery` |
| Cálculo do imposto e repartição entre cidades | `docs/wyd/economia.md` e código de simulação |

*Página de planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado.*