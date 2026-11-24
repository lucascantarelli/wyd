> Pergunta: "que tabelas o banco cria para o mercado entre jogadores e a custódia do item, e com que invariante?"
> Público: Desenvolvedor novo, arquitetura e quem opera
> Dono: equipe do `wyd-db`
> Base: `lacunas.md` `M31` (decidida em 2026-11-22); `ddl-confirmado.md` §5.34 e §5.32 (emenda); `armaria-e-mercado.md` §3 e §6; `../wyd-admin/docs/arquitetura/contrato-c4.md` §4.9–§4.13
> Revisado em: 2026-11-22

**Estado:** aceito (2026-11-24)

Trigésimo quarto arquivo da série: **onze tabelas** — dez em `portal` (o mercado) e uma em `game` (a
custódia do item). Fecha `LAC-ADM-07` do `wyd-admin` e completa o `0032` com os dois `GRANT` que a
custódia exige.

!!! note "Decidido em 2026-11-22 (`M31`)"
    A série passa de 33 para **34 arquivos** e `Tab.` de 98 para **109**; `SCHEMA_VERSION = 34` na troca.
    A forma vem de `armaria-e-mercado.md` §3, que continua dona da **razão**; aqui mora o texto.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0034` (decidido em 2026-11-22; próximo livre depois de `0033`) |
| Arquivo | `crates/wyd-db/migrations/0034_portal_market.sql` |
| Schema | `portal` (10 tabelas) + `game` (1 tabela) |
| Posição | 34 de 34 |
| Depende de | `0001` (schemas, `ops.fn_set_updated_at`, default privileges), `0006` (`game.character`), `0009` (`game.character_item`) e `0032` (emendado nesta rodada) |
| É dependência de | nada na série; as views de `0029` não a leem |
| Tabelas criadas | 11 |
| Objetos criados | 11 tabelas, 1 trigger por tabela com `updated_at` (4), 15 índices, 2 `REVOKE` e 1 `GRANT` |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca; na troca, vale 34 |

**Por que um arquivo e não dois.** A custódia (`game.market_escrow`) e a liquidação (`portal.market_*`)
são a mesma transação: o item sai de `game.character_item`, nasce na custódia e o crédito se move. Dois
arquivos por schema separariam o que o contrato promete atômico e nenhum seria dono da fronteira.

## 2. Por que tabela nova

| Alternativa | Por que não |
|---|---|
| `ops.delivery_queue` para entregar o item | a fila concede item **novo** a uma conta, de forma assíncrona; não transfere instância existente entre personagens (`armaria-e-mercado.md` §5) |
| views de leitura apenas | view não guarda custódia: sem tabela aberta, o item não teria onde estar fora do inventário |
| `game.account_wallet` como carteira do mercado | a moeda do mercado é o crédito do portal, não o ouro da conta (`M16`); misturar apagaria a fronteira entre dois saldos |
| adiar o lote | `ADM-10` do C4 ficaria sem forma e a armaria do portal sem schema |

## 3. DDL

```sql
-- 0034_portal_market: mercado do portal (10 tabelas em `portal`) e custódia do item (1 em `game`).
-- Depende de `0009` (`game.character_item`, `game.character`) e de `0001` (`ops.fn_set_updated_at`).

-- migrate:up
CREATE TABLE portal.market_wallet (
    account_id BIGINT NOT NULL,
    currency   TEXT NOT NULL DEFAULT 'portal_credit',
    balance    BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_wallet PRIMARY KEY (account_id),
    CONSTRAINT ck_market_wallet_moeda CHECK (currency IN ('portal_credit')),
    CONSTRAINT ck_market_wallet_saldo CHECK (balance >= 0)
);
CREATE TRIGGER trg_market_wallet_set_updated_at
    BEFORE UPDATE ON portal.market_wallet
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.market_ledger (
    id              BIGINT GENERATED ALWAYS AS IDENTITY,
    account_id      BIGINT NOT NULL,
    kind            TEXT NOT NULL,
    amount          BIGINT NOT NULL,
    balance_after   BIGINT NOT NULL,
    ref_kind        TEXT,
    ref_id          BIGINT,
    idempotency_key TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_ledger PRIMARY KEY (id),
    CONSTRAINT ck_market_ledger_tipo CHECK (kind IN (
        'recarga', 'compra', 'venda', 'taxa', 'retencao_lance', 'devolucao_lance', 'estorno'
    )),
    CONSTRAINT ck_market_ledger_valor CHECK (amount <> 0),
    CONSTRAINT ck_market_ledger_saldo CHECK (balance_after >= 0),
    CONSTRAINT ck_market_ledger_ref CHECK ((ref_kind IS NULL) = (ref_id IS NULL)),
    CONSTRAINT ck_market_ledger_ref_tipo CHECK (
        ref_kind IS NULL OR ref_kind IN ('anuncio', 'transacao', 'disputa', 'recarga')
    ),
    CONSTRAINT uq_market_ledger_idempotency UNIQUE (idempotency_key)
);
CREATE INDEX ix_market_ledger_conta_criado ON portal.market_ledger (account_id, created_at DESC);

CREATE TABLE portal.market_fee_rule (
    id                   BIGINT GENERATED ALWAYS AS IDENTITY,
    fee_bps              INTEGER NOT NULL,
    effective_from       TIMESTAMPTZ NOT NULL,
    created_by_person_id BIGINT,
    note                 TEXT NOT NULL DEFAULT '',
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_fee_rule PRIMARY KEY (id),
    CONSTRAINT uq_market_fee_rule_vigencia UNIQUE (effective_from),
    CONSTRAINT ck_market_fee_rule_bps CHECK (fee_bps BETWEEN 0 AND 10000)
);

CREATE TABLE portal.market_listing (
    id                BIGINT GENERATED ALWAYS AS IDENTITY,
    seller_account_id BIGINT NOT NULL,
    character_id      BIGINT NOT NULL,
    item_index        INTEGER NOT NULL,
    item_kind_slug    TEXT,
    qty               INTEGER NOT NULL DEFAULT 1,
    refine            SMALLINT NOT NULL DEFAULT 0,
    ancient           BOOLEAN NOT NULL DEFAULT false,
    gem_effect        SMALLINT NOT NULL DEFAULT 0,
    gem_value         SMALLINT NOT NULL DEFAULT 0,
    flags             INTEGER NOT NULL DEFAULT 0,
    kind              TEXT NOT NULL,
    price             BIGINT NOT NULL,
    buyout_price      BIGINT,
    fee_bps           INTEGER NOT NULL,
    ends_at           TIMESTAMPTZ,
    status            TEXT NOT NULL DEFAULT 'rascunho',
    escrow_ref        BIGINT,
    current_bid_id    BIGINT,
    bid_count         INTEGER NOT NULL DEFAULT 0,
    view_count        INTEGER NOT NULL DEFAULT 0,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_listing PRIMARY KEY (id),
    CONSTRAINT ck_market_listing_tipo CHECK (kind IN ('venda', 'leilao')),
    CONSTRAINT ck_market_listing_estado CHECK (status IN (
        'rascunho', 'ativo', 'vendido', 'expirado', 'cancelado', 'disputa'
    )),
    CONSTRAINT ck_market_listing_preco CHECK (price > 0),
    CONSTRAINT ck_market_listing_compra_ja CHECK (buyout_price IS NULL OR buyout_price >= price),
    CONSTRAINT ck_market_listing_prazo CHECK ((kind = 'leilao') = (ends_at IS NOT NULL)),
    CONSTRAINT ck_market_listing_custodia CHECK (status <> 'ativo' OR escrow_ref IS NOT NULL),
    CONSTRAINT ck_market_listing_qtd CHECK (qty >= 1),
    CONSTRAINT ck_market_listing_refino CHECK (refine BETWEEN 0 AND 15),
    CONSTRAINT ck_market_listing_taxa CHECK (fee_bps BETWEEN 0 AND 10000),
    CONSTRAINT ck_market_listing_popularidade CHECK (bid_count >= 0 AND view_count >= 0),
    CONSTRAINT ck_market_listing_categoria CHECK (
        item_kind_slug IS NULL OR char_length(item_kind_slug) BETWEEN 1 AND 64
    )
);
CREATE INDEX ix_market_listing_item_status_preco
    ON portal.market_listing (item_index, status, price);
CREATE INDEX ix_market_listing_kind_status_preco
    ON portal.market_listing (item_kind_slug, status, price);
CREATE INDEX ix_market_listing_status_popularidade
    ON portal.market_listing (status, bid_count DESC, view_count DESC);
CREATE INDEX ix_market_listing_status_prazo
    ON portal.market_listing (status, ends_at) WHERE status = 'ativo';
CREATE TRIGGER trg_market_listing_set_updated_at
    BEFORE UPDATE ON portal.market_listing
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.market_bid (
    id                BIGINT GENERATED ALWAYS AS IDENTITY,
    listing_id        BIGINT NOT NULL,
    bidder_account_id BIGINT NOT NULL,
    amount            BIGINT NOT NULL,
    status            TEXT NOT NULL DEFAULT 'ativo',
    hold_ledger_id    BIGINT,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_bid PRIMARY KEY (id),
    CONSTRAINT fk_market_bid_listing FOREIGN KEY (listing_id)
        REFERENCES portal.market_listing (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT fk_market_bid_ledger FOREIGN KEY (hold_ledger_id)
        REFERENCES portal.market_ledger (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT ck_market_bid_valor CHECK (amount > 0),
    CONSTRAINT ck_market_bid_estado CHECK (status IN (
        'ativo', 'superado', 'vencedor', 'perdido', 'cancelado'
    ))
);
CREATE UNIQUE INDEX ux_market_bid_comprador_ativo
    ON portal.market_bid (listing_id, bidder_account_id) WHERE status = 'ativo';
CREATE INDEX ix_market_bid_listing_criado ON portal.market_bid (listing_id, created_at DESC);
CREATE INDEX ix_market_bid_ledger ON portal.market_bid (hold_ledger_id)
    WHERE hold_ledger_id IS NOT NULL;

CREATE TABLE portal.market_transaction (
    id                BIGINT GENERATED ALWAYS AS IDENTITY,
    listing_id        BIGINT NOT NULL,
    buyer_account_id  BIGINT NOT NULL,
    seller_account_id BIGINT NOT NULL,
    price_gross       BIGINT NOT NULL,
    fee_amount        BIGINT NOT NULL,
    net_amount        BIGINT NOT NULL,
    fee_bps           INTEGER NOT NULL,
    escrow_ref        BIGINT,
    settled_at        TIMESTAMPTZ NOT NULL,
    idempotency_key   TEXT NOT NULL,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_transaction PRIMARY KEY (id),
    CONSTRAINT fk_market_transaction_listing FOREIGN KEY (listing_id)
        REFERENCES portal.market_listing (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT uq_market_transaction_listing UNIQUE (listing_id),
    CONSTRAINT uq_market_transaction_idempotency UNIQUE (idempotency_key),
    CONSTRAINT ck_market_transaction_partes CHECK (buyer_account_id <> seller_account_id),
    CONSTRAINT ck_market_transaction_soma CHECK (fee_amount + net_amount = price_gross),
    CONSTRAINT ck_market_transaction_valores CHECK (
        price_gross > 0 AND fee_amount >= 0 AND net_amount >= 0
    )
);
CREATE INDEX ix_market_transaction_comprador
    ON portal.market_transaction (buyer_account_id, settled_at DESC);
CREATE INDEX ix_market_transaction_vendedor
    ON portal.market_transaction (seller_account_id, settled_at DESC);

CREATE TABLE portal.market_review (
    id                  BIGINT GENERATED ALWAYS AS IDENTITY,
    transaction_id      BIGINT NOT NULL,
    reviewer_account_id BIGINT NOT NULL,
    seller_account_id   BIGINT NOT NULL,
    rating              SMALLINT NOT NULL,
    comment             TEXT,
    state               TEXT NOT NULL DEFAULT 'visivel',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_review PRIMARY KEY (id),
    CONSTRAINT fk_market_review_transaction FOREIGN KEY (transaction_id)
        REFERENCES portal.market_transaction (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT uq_market_review_transaction UNIQUE (transaction_id),
    CONSTRAINT ck_market_review_nota CHECK (rating BETWEEN 1 AND 5),
    CONSTRAINT ck_market_review_comentario CHECK (
        comment IS NULL OR char_length(comment) <= 500
    ),
    CONSTRAINT ck_market_review_estado CHECK (state IN ('visivel', 'oculto'))
);

CREATE TABLE portal.market_reputation (
    account_id      BIGINT NOT NULL,
    sales_completed INTEGER NOT NULL DEFAULT 0,
    sales_disputed  INTEGER NOT NULL DEFAULT 0,
    disputes_lost   INTEGER NOT NULL DEFAULT 0,
    rating_sum      BIGINT NOT NULL DEFAULT 0,
    rating_count    INTEGER NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_reputation PRIMARY KEY (account_id),
    CONSTRAINT ck_market_reputation_contadores CHECK (
        sales_completed >= 0 AND sales_disputed >= 0 AND disputes_lost >= 0 AND rating_count >= 0
    ),
    CONSTRAINT ck_market_reputation_notas CHECK ((rating_count = 0) = (rating_sum = 0))
);
CREATE TRIGGER trg_market_reputation_set_updated_at
    BEFORE UPDATE ON portal.market_reputation
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE portal.market_notification (
    id         BIGINT GENERATED ALWAYS AS IDENTITY,
    account_id BIGINT NOT NULL,
    kind       TEXT NOT NULL,
    ref_kind   TEXT NOT NULL,
    ref_id     BIGINT NOT NULL,
    read_at    TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_notification PRIMARY KEY (id),
    CONSTRAINT ck_market_notification_tipo CHECK (kind IN (
        'lance_recebido', 'lance_superado', 'arrematado', 'vendido', 'expirado',
        'disputa_aberta', 'disputa_decidida', 'avaliacao_recebida'
    )),
    CONSTRAINT ck_market_notification_ref CHECK (ref_kind IN ('anuncio', 'transacao', 'disputa'))
);
CREATE INDEX ix_market_notification_conta_nao_lida
    ON portal.market_notification (account_id) WHERE read_at IS NULL;

CREATE TABLE portal.market_dispute (
    id                    BIGINT GENERATED ALWAYS AS IDENTITY,
    transaction_id        BIGINT NOT NULL,
    opened_by_account_id  BIGINT NOT NULL,
    reason_code           TEXT NOT NULL,
    description           TEXT NOT NULL,
    state                 TEXT NOT NULL DEFAULT 'aberta',
    resolution            TEXT,
    decided_by_person_id  BIGINT,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_market_dispute PRIMARY KEY (id),
    CONSTRAINT fk_market_dispute_transaction FOREIGN KEY (transaction_id)
        REFERENCES portal.market_transaction (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT ck_market_dispute_motivo CHECK (reason_code IN (
        'nao_recebido', 'divergente', 'cobranca_indevida', 'outro'
    )),
    CONSTRAINT ck_market_dispute_estado CHECK (state IN (
        'aberta', 'em_analise', 'resolvida_comprador', 'resolvida_vendedor', 'cancelada'
    )),
    CONSTRAINT ck_market_dispute_desfecho CHECK (
        (state IN ('resolvida_comprador', 'resolvida_vendedor'))
            = (resolution IS NOT NULL AND decided_by_person_id IS NOT NULL)
    ),
    CONSTRAINT ck_market_dispute_descricao CHECK (char_length(description) BETWEEN 1 AND 2000)
);
CREATE UNIQUE INDEX ux_market_dispute_aberta
    ON portal.market_dispute (transaction_id) WHERE state IN ('aberta', 'em_analise');
CREATE INDEX ix_market_dispute_estado_aberta
    ON portal.market_dispute (state, created_at) WHERE state IN ('aberta', 'em_analise');
CREATE INDEX ix_market_dispute_transaction ON portal.market_dispute (transaction_id);
CREATE TRIGGER trg_market_dispute_set_updated_at
    BEFORE UPDATE ON portal.market_dispute
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.market_escrow (
    id              BIGINT GENERATED ALWAYS AS IDENTITY,
    character_id    BIGINT NOT NULL,
    item_index      INTEGER NOT NULL,
    qty             INTEGER NOT NULL,
    refine          SMALLINT NOT NULL,
    ancient         BOOLEAN NOT NULL,
    gem_effect      SMALLINT NOT NULL,
    gem_value       SMALLINT NOT NULL,
    flags           INTEGER NOT NULL,
    state           TEXT NOT NULL DEFAULT 'retido',
    listing_ref     BIGINT,
    transaction_ref BIGINT,
    idempotency_key TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at       TIMESTAMPTZ,
    CONSTRAINT pk_market_escrow PRIMARY KEY (id),
    CONSTRAINT fk_market_escrow_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT uq_market_escrow_idempotency UNIQUE (idempotency_key),
    CONSTRAINT ck_market_escrow_estado CHECK (state IN ('retido', 'entregue', 'devolvido')),
    CONSTRAINT ck_market_escrow_qtd CHECK (qty >= 1),
    CONSTRAINT ck_market_escrow_refino CHECK (refine BETWEEN 0 AND 15),
    CONSTRAINT ck_market_escrow_fechamento CHECK ((state = 'retido') = (closed_at IS NULL))
);
CREATE INDEX ix_market_escrow_estado ON game.market_escrow (state, created_at);
CREATE INDEX ix_market_escrow_character ON game.market_escrow (character_id);

REVOKE UPDATE, DELETE ON portal.market_ledger FROM wyd_portal;
REVOKE INSERT, UPDATE, DELETE ON game.market_escrow FROM wyd_game;
GRANT SELECT, INSERT, UPDATE ON game.market_escrow TO wyd_admin;
```

```sql
-- migrate:down
REVOKE SELECT, INSERT, UPDATE ON game.market_escrow FROM wyd_admin;
DROP INDEX game.ix_market_escrow_character;
DROP INDEX game.ix_market_escrow_estado;
DROP TABLE game.market_escrow;
DROP TRIGGER trg_market_dispute_set_updated_at ON portal.market_dispute;
DROP INDEX portal.ix_market_dispute_transaction;
DROP INDEX portal.ix_market_dispute_estado_aberta;
DROP INDEX portal.ux_market_dispute_aberta;
DROP TABLE portal.market_dispute;
DROP INDEX portal.ix_market_notification_conta_nao_lida;
DROP TABLE portal.market_notification;
DROP TRIGGER trg_market_reputation_set_updated_at ON portal.market_reputation;
DROP TABLE portal.market_reputation;
DROP TABLE portal.market_review;
DROP INDEX portal.ix_market_transaction_vendedor;
DROP INDEX portal.ix_market_transaction_comprador;
DROP TABLE portal.market_transaction;
DROP INDEX portal.ix_market_bid_ledger;
DROP INDEX portal.ix_market_bid_listing_criado;
DROP INDEX portal.ux_market_bid_comprador_ativo;
DROP TABLE portal.market_bid;
DROP TRIGGER trg_market_listing_set_updated_at ON portal.market_listing;
DROP INDEX portal.ix_market_listing_status_prazo;
DROP INDEX portal.ix_market_listing_status_popularidade;
DROP INDEX portal.ix_market_listing_kind_status_preco;
DROP INDEX portal.ix_market_listing_item_status_preco;
DROP TABLE portal.market_listing;
DROP TABLE portal.market_fee_rule;
DROP INDEX portal.ix_market_ledger_conta_criado;
DROP TABLE portal.market_ledger;
DROP TRIGGER trg_market_wallet_set_updated_at ON portal.market_wallet;
DROP TABLE portal.market_wallet;
```

## 4. Colunas

### 4.1 `portal.market_wallet`

| Coluna | Tipo | Nulo | Default | Significa |
|---|---|---|---|---|
| `account_id` | `BIGINT` | não | — | PK; a conta dona do saldo de mercado |
| `currency` | `TEXT` | não | `'portal_credit'` | unidade do saldo; uma só na 1.0.0 |
| `balance` | `BIGINT` | não | `0` | saldo disponível; nunca negativo |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | criação e último `UPDATE` (por trigger) |

### 4.2 `portal.market_ledger`

| Coluna | Tipo | Nulo | Default | Significa |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `account_id` | `BIGINT` | não | — | conta do extrato |
| `kind` | `TEXT` | não | — | motivo: recarga, compra, venda, taxa, retenção/devolução de lance, estorno |
| `amount` | `BIGINT` | não | — | valor com sinal; nunca zero |
| `balance_after` | `BIGINT` | não | — | saldo depois do lançamento |
| `ref_kind` / `ref_id` | `TEXT` / `BIGINT` | sim | — | origem: anúncio, transação, disputa ou recarga; os dois juntos ou nenhum |
| `idempotency_key` | `TEXT` | não | — | chave única do lançamento (`M17`) |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | instante do lançamento |

### 4.3 `portal.market_fee_rule`

| Coluna | Tipo | Nulo | Default | Significa |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `fee_bps` | `INTEGER` | não | — | taxa da casa em bps, 0–10000 |
| `effective_from` | `TIMESTAMPTZ` | não | — | início da vigência; única, para não haver duas regras no mesmo instante |
| `created_by_person_id` | `BIGINT` | sim | — | pessoa do portal que criou; sem FK (não há FK de `portal` para `game`) |
| `note` | `TEXT` | não | `''` | motivo da mudança |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | registro da linha |

### 4.4 `portal.market_listing`

| Coluna | Tipo | Nulo | Default | Significa |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `seller_account_id` | `BIGINT` | não | — | conta vendedora |
| `character_id` | `BIGINT` | não | — | personagem de onde o item saiu |
| `item_index` | `INTEGER` | não | — | item do catálogo (`0014`), sem FK |
| `item_kind_slug` | `TEXT` | sim | — | cópia da categoria para filtro; anulável |
| `qty` | `INTEGER` | não | `1` | quantidade anunciada |
| `refine` | `SMALLINT` | não | `0` | refino da instância, 0–15 |
| `ancient` / `gem_effect` / `gem_value` / `flags` | `BOOLEAN` / `SMALLINT` / `SMALLINT` / `INTEGER` | não | `false`/`0`/`0`/`0` | retrato da instância, igual a `0009` |
| `kind` | `TEXT` | não | — | `venda` ou `leilao` |
| `price` | `BIGINT` | não | — | preço de saída, > 0 |
| `buyout_price` | `BIGINT` | sim | — | compra imediata do leilão, ≥ `price` |
| `fee_bps` | `INTEGER` | não | — | taxa congelada no ato do anúncio |
| `ends_at` | `TIMESTAMPTZ` | sim | — | fim do leilão; obrigatório em `leilao`, proibido em `venda` |
| `status` | `TEXT` | não | `'rascunho'` | rascunho, ativo, vendido, expirado, cancelado, disputa |
| `escrow_ref` | `BIGINT` | sim | — | id da custódia; obrigatório com `status = 'ativo'` |
| `current_bid_id` | `BIGINT` | sim | — | lance líder |
| `bid_count` / `view_count` | `INTEGER` | não | `0` | contadores para a vitrine |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | criação e último `UPDATE` (por trigger) |

### 4.5 `portal.market_bid`

| Coluna | Tipo | Nulo | Default | Significa |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `listing_id` | `BIGINT` | não | — | FK `RESTRICT` para o anúncio |
| `bidder_account_id` | `BIGINT` | não | — | conta que deu o lance |
| `amount` | `BIGINT` | não | — | valor, > 0 |
| `status` | `TEXT` | não | `'ativo'` | ativo, superado, vencedor, perdido, cancelado |
| `hold_ledger_id` | `BIGINT` | sim | — | FK `RESTRICT` para o lançamento de retenção |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | instante do lance |

### 4.6 `portal.market_transaction`

| Coluna | Tipo | Nulo | Default | Significa |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `listing_id` | `BIGINT` | não | — | FK `RESTRICT`, único: um anúncio liquida uma vez |
| `buyer_account_id` / `seller_account_id` | `BIGINT` | não | — | partes, obrigatoriamente diferentes |
| `price_gross` | `BIGINT` | não | — | valor pago pelo comprador |
| `fee_amount` / `net_amount` | `BIGINT` | não | — | taxa da casa e líquido do vendedor; somam o bruto |
| `fee_bps` | `INTEGER` | não | — | taxa aplicada |
| `escrow_ref` | `BIGINT` | sim | — | custódia liquidada |
| `settled_at` | `TIMESTAMPTZ` | não | — | instante da liquidação |
| `idempotency_key` | `TEXT` | não | — | chave única da liquidação |

### 4.7 `portal.market_review`

| Coluna | Tipo | Nulo | Default | Significa |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `transaction_id` | `BIGINT` | não | — | FK `RESTRICT`, única: uma avaliação por transação |
| `reviewer_account_id` / `seller_account_id` | `BIGINT` | não | — | quem avaliou e quem foi avaliado |
| `rating` | `SMALLINT` | não | — | nota 1–5 |
| `comment` | `TEXT` | sim | — | até 500 caracteres |
| `state` | `TEXT` | não | `'visivel'` | `visivel` ou `oculto` por moderação |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | instante da avaliação |

### 4.8 `portal.market_reputation`

| Coluna | Tipo | Nulo | Default | Significa |
|---|---|---|---|---|
| `account_id` | `BIGINT` | não | — | PK |
| `sales_completed` / `sales_disputed` / `disputes_lost` | `INTEGER` | não | `0` | contadores de venda e disputa |
| `rating_sum` / `rating_count` | `BIGINT` / `INTEGER` | não | `0` | soma e número de notas; zero nos dois ou em nenhum |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | criação e último `UPDATE` (por trigger) |

### 4.9 `portal.market_notification`

| Coluna | Tipo | Nulo | Default | Significa |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `account_id` | `BIGINT` | não | — | destinatário |
| `kind` | `TEXT` | não | — | tipo do aviso, em lista fechada de oito |
| `ref_kind` / `ref_id` | `TEXT` / `BIGINT` | não | — | anúncio, transação ou disputa que motivou |
| `read_at` | `TIMESTAMPTZ` | sim | — | quando a conta leu; nulo é o badge |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | instante do evento |

### 4.10 `portal.market_dispute`

| Coluna | Tipo | Nulo | Default | Significa |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `transaction_id` | `BIGINT` | não | — | FK `RESTRICT` para a transação disputada |
| `opened_by_account_id` | `BIGINT` | não | — | quem abriu |
| `reason_code` | `TEXT` | não | — | não recebido, divergente, cobrança indevida, outro |
| `description` | `TEXT` | não | — | 1 a 2000 caracteres |
| `state` | `TEXT` | não | `'aberta'` | aberta, em análise, resolvida para um lado, cancelada |
| `resolution` | `TEXT` | sim | — | desfecho escrito; obrigatório nas duas resoluções |
| `decided_by_person_id` | `BIGINT` | sim | — | autor da decisão; sem FK, obrigatório junto com `resolution` |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | criação e último `UPDATE` (por trigger) |

### 4.11 `game.market_escrow`

| Coluna | Tipo | Nulo | Default | Significa |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | PK |
| `character_id` | `BIGINT` | não | — | FK `RESTRICT` para `game.character`: de quem é o item |
| `item_index` / `qty` | `INTEGER` | não | — | item e quantidade custodiados |
| `refine` / `ancient` / `gem_effect` / `gem_value` / `flags` | `SMALLINT` / `BOOLEAN` / `SMALLINT` / `SMALLINT` / `INTEGER` | não | — | retrato da instância, copiado da mochila |
| `state` | `TEXT` | não | `'retido'` | retido, entregue, devolvido |
| `listing_ref` / `transaction_ref` | `BIGINT` | sim | — | referência ao anúncio e à liquidação; sem FK, por fronteira |
| `idempotency_key` | `TEXT` | não | — | chave única da operação de custódia |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | instante da retenção |
| `closed_at` | `TIMESTAMPTZ` | sim | — | instante da entrega ou devolução; nulo só em `retido` |

## 5. Contratos e invariantes

| Constraint | Estado que proíbe | Código |
|---|---|---|
| `ck_market_wallet_saldo`, `ck_market_ledger_saldo` | saldo negativo | `23514` |
| `ck_market_ledger_valor` | lançamento de zero | `23514` |
| `uq_market_ledger_idempotency` | o mesmo lançamento gravado duas vezes | `23505` |
| `ck_market_ledger_tipo` | motivo fora da lista de sete | `23514` |
| `ck_market_listing_custodia` | anúncio `ativo` sem custódia | `23514` |
| `ck_market_listing_prazo` | leilão sem `ends_at`, ou venda com ele | `23514` |
| `ck_market_listing_compra_ja` | compra imediata abaixo do preço de saída | `23514` |
| `ux_market_bid_comprador_ativo` | dois lances ativos do mesmo comprador no mesmo anúncio | `23505` |
| `ck_market_transaction_soma` | taxa + líquido ≠ bruto | `23514` |
| `ck_market_transaction_partes` | comprador igual ao vendedor | `23514` |
| `uq_market_transaction_listing` | duas liquidações do mesmo anúncio | `23505` |
| `uq_market_review_transaction` | duas avaliações da mesma transação | `23505` |
| `ck_market_reputation_notas` | média sem notas ou soma sem contagem | `23514` |
| `ux_market_dispute_aberta` | duas disputas vivas na mesma transação | `23505` |
| `ck_market_dispute_desfecho` | resolução sem autor, ou autor sem resolução | `23514` |
| `ck_market_escrow_fechamento` | custódia `retido` com carimbo, ou fechada sem ele | `23514` |
| `uq_market_escrow_idempotency` | a mesma custódia executada duas vezes | `23505` |

## 6. Permissões

| Papel | Efeito |
|---|---|
| `wyd_portal` | `USAGE` em `portal` e `SELECT`/`INSERT`/`UPDATE`/`DELETE` nas dez tabelas, por default privilege do `0001` — **exceto** `UPDATE`/`DELETE` em `market_ledger`, revogados aqui: o extrato é append-only (`M16`) |
| `wyd_admin` | `USAGE` em `game` e `SELECT`/`INSERT`/`UPDATE` em `game.market_escrow` (este arquivo), mais `SELECT, INSERT, DELETE` em `game.character_item` e `SELECT` em `game.character` (emenda do `0032`) |
| `wyd_game` | `SELECT` em `game.market_escrow`, por default privilege; **sem** `INSERT`/`UPDATE`/`DELETE`, revogados aqui — quem move item é o executor (`LAC-POR-01`) |
| `wyd_read` | `SELECT` nas onze, por default privilege |
| ninguém | escrita em `game.market_escrow` por `wyd_portal`: sem `USAGE` em `game` (`P-17`), a tentativa falha com `42501` |

## 7. Provas

| # | Invariante | Comando | Esperado |
|---:|---|---|---|
| 1 | onze tabelas | contar as tabelas novas por schema | `10` em `portal`, `1` em `game` |
| 2 | anúncio ativo sem custódia | `INSERT` com `status = 'ativo'` e `escrow_ref` nulo | `23514` em `ck_market_listing_custodia` |
| 3 | leilão sem prazo | `kind = 'leilao'` com `ends_at` nulo | `23514` em `ck_market_listing_prazo` |
| 4 | soma da transação | `fee_amount + net_amount <> price_gross` | `23514` em `ck_market_transaction_soma` |
| 5 | dois lances ativos | duas linhas ativas do mesmo comprador no mesmo anúncio | segundo `23505` em `ux_market_bid_comprador_ativo` |
| 6 | duas disputas vivas | duas com `state = 'aberta'` na mesma transação | segundo `23505` em `ux_market_dispute_aberta` |
| 7 | resolução sem autor | `state = 'resolvida_comprador'` com `decided_by_person_id` nulo | `23514` em `ck_market_dispute_desfecho` |
| 8 | extrato append-only | `SET ROLE wyd_portal; UPDATE portal.market_ledger …` | `42501` |
| 9 | jogo não move custódia | `SET ROLE wyd_game; INSERT INTO game.market_escrow …` | `42501` |
| 10 | admin move custódia | `SET ROLE wyd_admin; INSERT INTO game.market_escrow …` e `DELETE FROM game.character_item …` | `INSERT 0 1` nas duas |
| 11 | chave de custódia repetida | duas linhas com o mesmo `idempotency_key` | segundo `23505` em `uq_market_escrow_idempotency` |
| 12 | `down` limpo | `make migrations-apply` (up + down) | 0 tabela de mercado ao fim |

## 8. O que este arquivo não decide

- **Retenção do mercado.** As tabelas nascem como estado de jogador; expurgo de notificação e de anúncio
  expirado é decisão própria, com medição (`M22` cobre sessão e auditoria, não mercado).
- **Estatísticas de arma** (`P-POR-08`): sem coluna e sem captura; a ficha publica só o provado desde
  `LAC-POR-08` (fechada em 2026-11-22).
- **Recarga do crédito.** `market_ledger.kind = 'recarga'` existe, mas a origem do crédito é do portal.
- **Separação de `ADM-10`.** A custódia é o contrato do `wyd-admin` (`LAC-ADM-07`, fechada em `M31`); aqui
  está só a forma que ele escreve.
