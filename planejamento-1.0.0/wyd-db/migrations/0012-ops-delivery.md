> Pergunta: "como aplico o `0012_ops_delivery` e como um pedido da web chega ao jogo sem o portal escrever estado vivo?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3; `ddl-confirmado.md` §5.16 (confirmado); `padrao-banco.md` §1/§5/§8/§9/§11; `lacunas.md` (`M17`, `M25`); `rascunho-accounts.md` `R9`
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-23)

Décimo segundo arquivo da série. Cria a **fila única** pela qual a web pede algo ao jogo: item, ouro ou
nível para uma conta ou personagem. O portal enfileira, o servidor drena e aplica; o portal nunca
escreve em `game` (`LAC-POR-01`).

!!! note "Aprovado em 2026-11-20"
    O dono aprovou o lote na rodada de perguntas de 2026-11-20, na opção "Sim, aprovar como está",
    depois de fechar `M25`: o dreno usa `SELECT … FOR UPDATE SKIP LOCKED` em transação curta e marca
    `claimed` antes de aplicar; `applied`/`failed` são expurgadas 30 dias após `closed_at` e `pending`
    nunca é expurgada. O DDL abaixo é o confirmado em `ddl-confirmado.md` §5.16, a tabela entra nas
    contadas com DDL fechado (`ddl-confirmado.md` §1) e este arquivo está `concluída` no checklist.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0012` |
| Arquivo | `crates/wyd-db/migrations/0012_ops_delivery.sql` |
| Schema | `ops` |
| Posição | 12 de 34 (ponte Cold → Hot, depois do estado de jogador de leitura simples) |
| Depende de | `0001` (schema `ops`, `ops.fn_set_updated_at`, default privileges), `0003` (`portal.person`, autor), `0005` (`game.account`, destino de `gold`) e `0006` (`game.character`, destino de `item`/`level`) |
| É dependência de | `0011` (entrega do item comprado) e `0029` (leitura operacional da fila, quando existir) |
| Tabelas criadas | 1 (`delivery_queue`) |
| Objetos criados | 1 tabela, 7 índices (5 parciais), 1 trigger, 2 `GRANT` |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca de série (`plano-migrations.md` §6) |

## 2. Objetivo

Persistir, de forma durável e idempotente, o pedido que o portal faz ao jogo — quem pediu, para quem,
o que entregar e em que ponto do ciclo a linha está —, sem que o portal toque em estado vivo e sem que
um reenvio do navegador vire duas entregas.

## 3. Pré-requisitos

1. `0001_bootstrap` aplicado: `ops`, `ops.fn_set_updated_at()` e default privileges existem; sem o
   `GRANT USAGE` em `ops` desta série, o `GRANT` daqui falha com `42501` de schema.
2. `0003_portal_person` aplicado: `portal.person` existe (FK do autor).
3. `0005_game_account` e `0006_game_character` aplicados: os destinos das FKs existem.
4. Conexão como `wyd_migrator`.

## 4. DDL do `up`

Texto de `ddl-confirmado.md` §5.16, copiado sem alteração.

```sql
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
```

## 5. DDL do `down`

Ordem inversa: trigger, índices e tabela. Os índices saem qualificados porque o runner fecha o
`search_path` da transação (`src/migrate/spec.md` §6).

```sql
-- migrate:down
DROP TRIGGER trg_delivery_queue_set_updated_at ON ops.delivery_queue;
DROP INDEX ops.ix_delivery_queue_status;
DROP INDEX ops.ix_delivery_queue_fechadas;
DROP INDEX ops.ix_delivery_queue_autor;
DROP INDEX ops.ix_delivery_queue_personagem;
DROP INDEX ops.ix_delivery_queue_conta;
DROP INDEX ops.ix_delivery_queue_pendentes;
DROP TABLE ops.delivery_queue;
```

## 6. Tabela de colunas

| Coluna | Tipo | Nulo | Default | O que é / origem |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | Identificador da entrega e ordem da fila. |
| `idempotency_key` | `UUID` | não | — | Chave do pedido gerada pelo BFF (`M17`); única na tabela. |
| `kind` | `TEXT` | não | — | `item`, `gold` ou `level` (`H`, glossário de grant). |
| `account_id` | `BIGINT` | sim | — | Conta destino de `gold`; FK `RESTRICT`. |
| `character_id` | `BIGINT` | sim | — | Personagem destino de `item` e `level`; FK `RESTRICT`. |
| `item_index` | `INTEGER` | sim | — | Índice do item no cliente (`H`, `STRUCT_ITEM.sIndex`); catálogo em `0014`, sem FK. |
| `qty` | `INTEGER` | sim | — | Quantidade da concessão de item, no mínimo 1. |
| `gold_amount` | `BIGINT` | sim | — | Ouro creditado na carteira da conta, sempre positivo. |
| `level_delta` | `INTEGER` | sim | — | Níveis somados ao personagem, sempre positivo. |
| `status` | `TEXT` | não | `'pending'` | `pending` → `claimed` → `applied`/`failed`. |
| `attempt_count` | `SMALLINT` | não | `0` | Quantas vezes o dreno reivindicou a linha. |
| `claimed_at` | `TIMESTAMPTZ` | sim | — | Quando o dreno tomou a linha; não nulo fora de `pending`. |
| `closed_at` | `TIMESTAMPTZ` | sim | — | Quando a linha saiu da fila; não nulo em `applied`/`failed`. |
| `failure_reason` | `TEXT` | sim | — | Motivo curto da falha; só existe em `failed`. |
| `created_by_person_id` | `BIGINT` | sim | — | Staff autor pela web; FK `RESTRICT` para `portal.person`. |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | Chegada do pedido; é a ordem FIFO junto com `id`. |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | Atualização com trigger (`A9`). |

## 7. Contratos e invariantes

| # | Objeto | Estado que impede | SQLSTATE |
|---:|---|---|---|
| 1 | `pk_delivery_queue` | duas entregas com o mesmo `id` | `23505` |
| 2 | `uq_delivery_queue_idem` | duas entregas para a mesma chave de pedido (duplo clique, retry) | `23505` |
| 3 | `ck_delivery_queue_tipo` | tipo fora de `item`/`gold`/`level` | `23514` |
| 4 | `ck_delivery_queue_estado` | estado fora dos quatro do ciclo | `23514` |
| 5 | `ck_delivery_queue_campos` | concessão de item sem personagem ou sem `qty`, de ouro sem conta, de nível sem personagem, ou com campo de outro tipo preenchido | `23514` |
| 6 | `ck_delivery_queue_reivindicacao` | `pending` com `claimed_at`, ou linha reivindicada sem carimbo | `23514` |
| 7 | `ck_delivery_queue_fechamento` | `applied`/`failed` sem `closed_at`, ou linha aberta com carimbo de fechamento | `23514` |
| 8 | `ck_delivery_queue_motivo` | `failed` sem motivo, ou motivo em linha que não falhou | `23514` |
| 9 | `ck_delivery_queue_ordem` | fechamento anterior à reivindicação | `23514` |
| 10 | `ck_delivery_queue_tentativas` | contador de tentativas negativo | `23514` |
| 11 | `fk_delivery_queue_account` / `_character` / `_autor` | entrega para conta, personagem ou pessoa inexistente | `23503` |
| 12 | `ON DELETE RESTRICT` | apagar conta ou personagem com concessão pendente | `23503` |
| 13 | permissão de `wyd_portal` | portal tentando `UPDATE`/`DELETE` na fila | `42501` |
| 14 | permissão de `wyd_game` | jogo tentando `INSERT` ou `DELETE` na fila | `42501` |

**O que o banco não expressa.** Que a entrega seja aplicada **uma vez** no jogo não é `CHECK`: o
banco garante a chave única e a coerência de estado, não o efeito no personagem. Quem prova é o teste
de dreno com a mesma linha reivindicada por dois processos (`M25`). Também não é do banco a política
de reenvio: `attempt_count` registra, o limite de tentativas é do dreno.

## 8. Permissões

`ops` recebe `USAGE` de `wyd_game`, `wyd_portal` e `wyd_read` no `0001`, e `SELECT` por default
privilege. Este arquivo concede o resto:

| Papel | Efeito | Origem |
|---|---|---|
| `wyd_portal` | `SELECT, INSERT` | `GRANT` explícito deste arquivo |
| `wyd_game` | `SELECT, UPDATE` | `GRANT` explícito deste arquivo |
| `wyd_read` | `SELECT` | default privilege do `0001` em `ops` |
| `PUBLIC` | nada | `0001`/provisionamento |

Sem `DELETE` em runtime: o expurgo é rotina de operação, com a retenção que `M25` fixou (30 dias após
`closed_at`). Sem `INSERT`
para o jogo: quem escreve na fila é a web; o jogo só lê e marca.

## 9. Prova de execução

PG17 efêmero, com `0001`, `0003`, `0005` e `0006` aplicados e uma pessoa, uma conta e um personagem de
prova (ids 1). A prova 12 usa `DELETE` de propósito, para medir o `RESTRICT`.

```bash
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0012_ops_delivery.sql
```

| # | Prova | Comando | Esperado |
|---:|---|---|---|
| 1 | tabela criada | `SELECT count(*) FROM information_schema.tables WHERE table_schema = 'ops' AND table_name = 'delivery_queue';` | `1` |
| 2 | item válido | `INSERT INTO ops.delivery_queue (idempotency_key, kind, character_id, item_index, qty) VALUES (gen_random_uuid(), 'item', 1, 123, 1);` | `INSERT 0 1`, `status = 'pending'` |
| 3 | reenvio da mesma chave | repetir o `INSERT` com a mesma `idempotency_key` | `23505` em `uq_delivery_queue_idem` |
| 4 | item sem personagem | `kind = 'item'` com `character_id` nulo | `23514` em `ck_delivery_queue_campos` |
| 5 | ouro na conta | `kind = 'gold', account_id = 1, gold_amount = 100` | `INSERT 0 1` |
| 6 | ouro para personagem | `kind = 'gold'` com `character_id` preenchido e `account_id` nulo | `23514` em `ck_delivery_queue_campos` |
| 7 | campo de outro tipo | `kind = 'level'` com `gold_amount` preenchido | `23514` em `ck_delivery_queue_campos` |
| 8 | tipo inventado | `kind = 'unban'` | `23514` em `ck_delivery_queue_tipo` |
| 9 | estado inventado | `status = 'done'` | `23514` em `ck_delivery_queue_estado` |
| 10 | falha sem motivo | `status = 'failed'` sem `failure_reason` | `23514` em `ck_delivery_queue_motivo` |
| 11 | reivindicação coerente | `UPDATE … SET status = 'claimed', claimed_at = now()` e depois `applied, closed_at = now()` | `UPDATE 1` nas duas |
| 12 | concessão pendente bloqueia exclusão | `DELETE FROM game.character WHERE id = 1;` com entrega pendente | `23503` em `fk_delivery_queue_character` |
| 13 | portal não drena | `SET ROLE wyd_portal; UPDATE ops.delivery_queue SET status = 'claimed';` | `42501` |
| 14 | jogo não enfileira | `SET ROLE wyd_game; INSERT INTO ops.delivery_queue (idempotency_key, kind) VALUES (gen_random_uuid(), 'gold');` | `42501` |
| 15 | `down` derruba só o lote | aplicar o bloco `-- migrate:down` e repetir 1 | `0` |

## 10. Decisões aprovadas e limites do lote

As escolhas de forma aprovadas com o lote, incluindo as duas que `M25` fechou:

| # | Escolha | O que o lote fixa |
|---|---|---|
| 1 | Destino por tipo | `gold` na carteira da conta; `item` e `level` no personagem; `ck_delivery_queue_campos` impede campo de outro tipo |
| 2 | Autoria | `created_by_person_id` com FK `RESTRICT` para `portal.person`; `0002` não tem FK só porque `0003` vem depois na série |
| 3 | Idempotência (`M17`) | `UUID` único; o reenvio devolve a linha existente, e a janela de 24 h é política do BFF |
| 4 | Ciclo de vida | `pending` → `claimed` → `applied`/`failed`, com carimbo coerente e `attempt_count` |
| 5 | Consumo do dreno (`M25`) | `SELECT … FOR UPDATE SKIP LOCKED` em transação curta: cada canal reivindica um lote, marca `claimed`/`claimed_at` e aplica fora da transação; `ix_delivery_queue_pendentes` serve a leitura em ordem de `id` |
| 6 | Retenção (`M25`) | `applied`/`failed` expurgadas 30 dias após `closed_at`; `pending` nunca é expurgada; `ix_delivery_queue_fechadas` serve o expurgo |

| Assunto | Onde é decidido |
|---|---|
| Concessão de item, ouro e nível no jogo | `wyd-admin`/`wyd-server`; aqui só a fila |
| Catálogo de item e preço | `0014` e `0011` |
| Bloqueio disciplinar e desbloqueio | `game.account_moderation` (`0005`), não a fila |
| Retenção e expurgo da fila | decidido em `M25` (2026-11-20); a rotina é de `planos/retencao-e-capacidade.md` |
| Forma do contrato HTTP que enfileira | planejamento do `wyd-admin` (`LAC-POR-01`) |

*Página de planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado.*
