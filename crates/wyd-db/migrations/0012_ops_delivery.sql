-- migrate:up
CREATE TABLE ops.delivery_queue (
    id                   BIGINT GENERATED ALWAYS AS IDENTITY,
    -- Chave do pedido gerada pelo BFF: reenvio devolve a mesma entrega, nunca enfileira duas (M17).
    idempotency_key      UUID NOT NULL,
    -- `gold` credita a carteira da conta; `item` e `level` exigem personagem.
    kind                 TEXT NOT NULL,
    account_id           BIGINT,
    character_id         BIGINT,
    item_index           INTEGER,
    qty                  INTEGER,
    gold_amount          BIGINT,
    level_delta          INTEGER,
    status               TEXT NOT NULL DEFAULT 'pending',
    attempt_count        SMALLINT NOT NULL DEFAULT 0,
    -- Instante em que o dreno tomou a linha para si.
    claimed_at           TIMESTAMPTZ,
    -- Instante em que a linha deixou a fila, por sucesso ou por falha.
    closed_at            TIMESTAMPTZ,
    failure_reason       TEXT,
    -- Staff que pediu a concessão pela web. Pessoa nunca é apagada (A12), então RESTRICT é seguro.
    created_by_person_id BIGINT,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_delivery_queue PRIMARY KEY (id),
    CONSTRAINT uq_delivery_queue_idem UNIQUE (idempotency_key),
    CONSTRAINT fk_delivery_queue_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT fk_delivery_queue_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT fk_delivery_queue_autor FOREIGN KEY (created_by_person_id)
        REFERENCES portal.person (id) ON DELETE RESTRICT ON UPDATE NO ACTION,
    CONSTRAINT ck_delivery_queue_tipo CHECK (kind IN ('item', 'gold', 'level')),
    CONSTRAINT ck_delivery_queue_estado CHECK (
        status IN ('pending', 'claimed', 'applied', 'failed')
    ),
    CONSTRAINT ck_delivery_queue_tentativas CHECK (attempt_count >= 0),
    -- Cada tipo carrega só os campos dele, com o destino que o jogo sabe aplicar.
    CONSTRAINT ck_delivery_queue_campos CHECK (
        (kind = 'item' AND character_id IS NOT NULL AND item_index IS NOT NULL AND qty >= 1
            AND gold_amount IS NULL AND level_delta IS NULL)
        OR (kind = 'gold' AND account_id IS NOT NULL AND gold_amount > 0
            AND item_index IS NULL AND qty IS NULL AND level_delta IS NULL)
        OR (kind = 'level' AND character_id IS NOT NULL AND level_delta > 0
            AND item_index IS NULL AND qty IS NULL AND gold_amount IS NULL)
    ),
    -- Pendente não tem reivindicação; qualquer outro estado foi reivindicado antes.
    CONSTRAINT ck_delivery_queue_reivindicacao CHECK (
        (status <> 'pending') = (claimed_at IS NOT NULL)
    ),
    CONSTRAINT ck_delivery_queue_fechamento CHECK (
        (status IN ('applied', 'failed')) = (closed_at IS NOT NULL)
    ),
    CONSTRAINT ck_delivery_queue_motivo CHECK (
        (status = 'failed') = (failure_reason IS NOT NULL)
    ),
    CONSTRAINT ck_delivery_queue_motivo_tamanho CHECK (
        failure_reason IS NULL OR char_length(failure_reason) BETWEEN 1 AND 200
    ),
    CONSTRAINT ck_delivery_queue_ordem CHECK (closed_at IS NULL OR closed_at >= claimed_at)
);

-- O dreno lê pendentes em ordem de `id`, sem `Sort` na consulta da fila.
CREATE INDEX ix_delivery_queue_pendentes ON ops.delivery_queue (id) WHERE status = 'pending';
-- Toda FK tem índice no lado filho (`P-15`); nulo não entra no índice.
CREATE INDEX ix_delivery_queue_conta ON ops.delivery_queue (account_id)
    WHERE account_id IS NOT NULL;
CREATE INDEX ix_delivery_queue_personagem ON ops.delivery_queue (character_id)
    WHERE character_id IS NOT NULL;
CREATE INDEX ix_delivery_queue_autor ON ops.delivery_queue (created_by_person_id)
    WHERE created_by_person_id IS NOT NULL;
-- Expurgo por retenção varre só o que já fechou.
CREATE INDEX ix_delivery_queue_fechadas ON ops.delivery_queue (closed_at)
    WHERE closed_at IS NOT NULL;
CREATE INDEX ix_delivery_queue_status ON ops.delivery_queue (status, created_at DESC);

CREATE TRIGGER trg_delivery_queue_set_updated_at
    BEFORE UPDATE ON ops.delivery_queue
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

-- O portal enfileira e acompanha; o jogo drena e marca. Ninguém apaga em runtime (P-16).
GRANT SELECT, INSERT ON ops.delivery_queue TO wyd_portal;
GRANT SELECT, UPDATE ON ops.delivery_queue TO wyd_game;
-- migrate:down
-- Reversão derivada do `up` deste arquivo; roda em ordem decrescente pela série.
-- Descarta o dado destas tabelas: o `up` recria a estrutura, não o conteúdo (`serie-e-regras.md` §1.3).
DROP TABLE ops.delivery_queue;
