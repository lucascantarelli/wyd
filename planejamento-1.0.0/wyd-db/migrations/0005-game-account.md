> Pergunta: "como aplico o `0005_game_account` e que conta, carteira, ledger, moderação e reset de PIN passam a existir?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3; `ddl-confirmado.md` §5.2; `padrao-banco.md` §2/§5/§8/§11; `lacunas.md` (`A17`, `M1`–`M3`, `M16`)
> Revisado em: 2026-11-20

Quinto arquivo da série. Cria a conta de jogo ligada à pessoa do portal, sua carteira, o ledger
append-only de moedas (`M16`), o estado de moderação e os tokens de reset de PIN. Pessoa e conta são
identidades distintas; conta fechada não se apaga e devolve sua vaga para a pessoa.

!!! warning "Nada disto rodou ainda"
    O DDL abaixo é **planejado** (`P`): nenhum Postgres foi executado contra ele e
    `crates/wyd-db/migrations/0005_game_account.sql` ainda não existe. O texto é o de
    `ddl-confirmado.md` §5.2, dono do DDL; esta página o transcreve sem redesenhá-lo.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0005` |
| Arquivo | `crates/wyd-db/migrations/0005_game_account.sql` |
| Schema | `game` (com FK de autor/identidade para `portal.person`) |
| Posição | 5 de 34 (conta e acesso do jogo) |
| Depende de | `0001_bootstrap` (schemas/permissões), `0003_portal_person` (pessoa e autor de moderação), `0004_portal_access` não cria FK usada aqui |
| É dependência de | `0006_game_character` (`game.character.account_id`), `0007_game_session`, `0011_game_economy` |
| Tabelas criadas | 5 (`account`, `account_wallet`, `account_ledger`, `account_moderation`, `pin_reset`) |
| Objetos criados | 5 tabelas, 8 índices, 1 trigger |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca de série (`plano-migrations.md` §6) |

## 2. Objetivo

Criar a identidade de login do cliente, limitar a cinco contas ativas por pessoa, manter saldo e
histórico de moedas, registrar bloqueio disciplinar com autor e permitir reset de PIN sem armazenar
o token em claro.

## 3. Pré-requisitos

1. `0001_bootstrap` aplicado e papéis do cluster provisionados.
2. `0003_portal_person` aplicado: `portal.person` existe e a FK `account.person_id` pode ser criada.
3. Conexão como `wyd_migrator`; `game` concede defaults de escrita apenas a `wyd_game` e leitura a
   `wyd_read`.
4. `portal.person` titular deve existir antes de inserir uma conta. A pessoa não é removida em operação.

## 4. DDL do `up`

Texto confirmado em `ddl-confirmado.md` §5.2.

```sql
-- migrate:up
CREATE TABLE game.account (
    id                  BIGINT GENERATED ALWAYS AS IDENTITY,
    person_id           BIGINT NOT NULL,
    slot                SMALLINT NOT NULL,
    login               TEXT NOT NULL,
    login_fold          TEXT GENERATED ALWAYS AS (lower(btrim(login))) STORED,
    password_hash       TEXT NOT NULL,
    password_changed_at TIMESTAMPTZ,
    pin_hash            TEXT,
    pin_failure_count   SMALLINT NOT NULL DEFAULT 0,
    pin_locked_until    TIMESTAMPTZ,
    login_failure_count SMALLINT NOT NULL DEFAULT 0,
    login_locked_until  TIMESTAMPTZ,
    role                TEXT NOT NULL DEFAULT 'player',
    status              TEXT NOT NULL DEFAULT 'active',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_login_at       TIMESTAMPTZ,
    closed_at           TIMESTAMPTZ,
    CONSTRAINT pk_account PRIMARY KEY (id),
    CONSTRAINT fk_account_person FOREIGN KEY (person_id)
        REFERENCES portal.person (id) ON DELETE RESTRICT,
    CONSTRAINT ck_account_slot_range CHECK (slot BETWEEN 1 AND 5),
    CONSTRAINT ck_account_login_len CHECK (char_length(login) BETWEEN 1 AND 16),
    CONSTRAINT ck_account_login_forma CHECK (login = btrim(login) AND char_length(login) > 0),
    CONSTRAINT ck_account_role CHECK (role IN ('player', 'moderator', 'admin')),
    CONSTRAINT ck_account_status CHECK (status IN ('active', 'closed')),
    CONSTRAINT ck_account_senha_hasheada CHECK (password_hash LIKE '$argon2id$%'),
    CONSTRAINT ck_account_pin_hasheado CHECK (pin_hash IS NULL OR pin_hash LIKE '$argon2id$%'),
    CONSTRAINT ck_account_pin_falhas CHECK (pin_failure_count >= 0),
    CONSTRAINT ck_account_login_falhas CHECK (login_failure_count >= 0),
    CONSTRAINT ck_account_senha_trocada CHECK (
        password_changed_at IS NULL OR password_changed_at >= created_at
    ),
    CONSTRAINT ck_account_fechamento_coerente CHECK ((status = 'closed') = (closed_at IS NOT NULL))
);
CREATE UNIQUE INDEX ux_account_login ON game.account (login_fold);
CREATE UNIQUE INDEX ux_account_slot ON game.account (person_id, slot) WHERE status <> 'closed';
CREATE INDEX ix_account_person ON game.account (person_id);
CREATE TRIGGER trg_account_set_updated_at
    BEFORE UPDATE ON game.account
    FOR EACH ROW
    EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.account_wallet (
    account_id BIGINT NOT NULL,
    coin       BIGINT NOT NULL DEFAULT 0,
    CONSTRAINT pk_account_wallet PRIMARY KEY (account_id),
    CONSTRAINT fk_account_wallet_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE RESTRICT,
    CONSTRAINT ck_account_wallet_coin CHECK (coin >= 0 AND coin <= 2147483647)
);

CREATE TABLE game.account_ledger (
    id            BIGINT GENERATED ALWAYS AS IDENTITY,
    account_id    BIGINT NOT NULL,
    delta         BIGINT NOT NULL,
    balance_after BIGINT NOT NULL,
    reason        TEXT NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_account_ledger PRIMARY KEY (id),
    CONSTRAINT fk_account_ledger_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE RESTRICT,
    CONSTRAINT ck_account_ledger_delta CHECK (delta <> 0),
    CONSTRAINT ck_account_ledger_balance CHECK (
        balance_after >= 0 AND balance_after <= 2147483647
    ),
    CONSTRAINT ck_account_ledger_reason CHECK (char_length(reason) BETWEEN 1 AND 64)
);
CREATE INDEX ix_account_ledger_conta ON game.account_ledger (account_id, created_at DESC);

CREATE TABLE game.account_moderation (
    account_id BIGINT NOT NULL,
    blocked    BOOLEAN NOT NULL DEFAULT false,
    reason     TEXT NOT NULL DEFAULT '',
    blocked_by BIGINT,
    blocked_at TIMESTAMPTZ,
    CONSTRAINT pk_account_moderation PRIMARY KEY (account_id),
    CONSTRAINT fk_account_moderation_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE RESTRICT,
    CONSTRAINT fk_account_moderation_autor FOREIGN KEY (blocked_by)
        REFERENCES portal.person (id) ON DELETE RESTRICT,
    CONSTRAINT ck_account_moderation_motivo CHECK (
        NOT blocked OR char_length(btrim(reason)) > 0
    ),
    CONSTRAINT ck_account_moderation_instante CHECK (
        (blocked AND blocked_at IS NOT NULL) OR (NOT blocked AND blocked_at IS NULL)
    ),
    CONSTRAINT ck_account_moderation_autor CHECK (NOT blocked OR blocked_by IS NOT NULL)
);
CREATE INDEX ix_account_moderation_autor ON game.account_moderation (blocked_by);
CREATE INDEX ix_account_moderation_bloqueadas
    ON game.account_moderation (account_id) WHERE blocked;

CREATE TABLE game.pin_reset (
    id           BIGINT GENERATED ALWAYS AS IDENTITY,
    account_id   BIGINT NOT NULL,
    token_hash   TEXT NOT NULL,
    requested_ip INET,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at   TIMESTAMPTZ NOT NULL,
    used_at      TIMESTAMPTZ,
    CONSTRAINT pk_pin_reset PRIMARY KEY (id),
    CONSTRAINT fk_pin_reset_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE RESTRICT,
    CONSTRAINT uq_pin_reset_token UNIQUE (token_hash),
    CONSTRAINT ck_pin_reset_token_forma CHECK (token_hash ~ '^[0-9a-f]{64}$'),
    CONSTRAINT ck_pin_reset_validade CHECK (expires_at > created_at),
    CONSTRAINT ck_pin_reset_consumo CHECK (used_at IS NULL OR used_at >= created_at)
);
CREATE INDEX ix_pin_reset_account ON game.pin_reset (account_id, created_at DESC);
```

## 5. DDL do `down`

Ordem inversa, filhos antes do pai. As FKs `RESTRICT` protegem o histórico financeiro e a identidade;
o `down` remove os objetos do arquivo como unidade declarada.

```sql
-- migrate:down
DROP INDEX ix_pin_reset_account;
DROP TABLE game.pin_reset;
DROP INDEX ix_account_moderation_bloqueadas;
DROP INDEX ix_account_moderation_autor;
DROP TABLE game.account_moderation;
DROP INDEX ix_account_ledger_conta;
DROP TABLE game.account_ledger;
DROP TABLE game.account_wallet;
DROP TRIGGER trg_account_set_updated_at ON game.account;
DROP INDEX ix_account_person;
DROP INDEX ux_account_slot;
DROP INDEX ux_account_login;
DROP TABLE game.account;
```

## 6. Tabelas de colunas

### 6.1 `game.account`

| Coluna | Tipo | Nulo | Default | Significado / origem |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | Identificador interno da conta. |
| `person_id` | `BIGINT` | não | — | Pessoa titular do portal; FK `RESTRICT` (`A18`). |
| `slot` | `SMALLINT` | não | — | Vaga da pessoa, 1–5 (`H`, limite aprovado). |
| `login` | `TEXT` | não | — | Login do cliente, até 16 caracteres (`H`). |
| `login_fold` | `TEXT` gerada STORED | não | `lower(btrim(login))` | Chave de login sem caixa/espaço. |
| `password_hash` | `TEXT` | não | — | Hash Argon2id, nunca senha em claro (`M9`). |
| `password_changed_at` | `TIMESTAMPTZ` | sim | — | Invalidar sessões emitidas antes da troca (`M3`). |
| `pin_hash` | `TEXT` | sim | — | PIN opcional como Argon2id hash (`B3`). |
| `pin_failure_count` | `SMALLINT` | não | `0` | Falhas de PIN; lockout é política de código. |
| `pin_locked_until` | `TIMESTAMPTZ` | sim | — | Fim do bloqueio do PIN (`B3`). |
| `login_failure_count` | `SMALLINT` | não | `0` | Falhas no login do jogo (`M3`, `A8`). |
| `login_locked_until` | `TIMESTAMPTZ` | sim | — | Fim do bloqueio de login. |
| `role` | `TEXT` | não | `'player'` | `player`, `moderator`, `admin` (`A15`). |
| `status` | `TEXT` | não | `'active'` | `active`/`closed`; bloqueio disciplinar fica na tabela filha (`M1`). |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Marcas; trigger atualiza `updated_at` (`M3`). |
| `last_login_at` | `TIMESTAMPTZ` | sim | — | Último login concluído. |
| `closed_at` | `TIMESTAMPTZ` | sim | — | Tombstone da conta; fechamento libera vaga. |

### 6.2 `game.account_wallet` e `game.account_ledger`

| Tabela.coluna | Tipo | Nulo | Default | Significado / origem |
|---|---|---|---|---|
| `account_wallet.account_id` | `BIGINT` | não | — | PK/FK `RESTRICT`, uma carteira por conta. |
| `account_wallet.coin` | `BIGINT` | não | `0` | Saldo corrente, limitado ao inteiro de 32 bits do fio (`H`). |
| `account_ledger.id` | `BIGINT` identity | não | identity | ID do lançamento. |
| `account_ledger.account_id` | `BIGINT` | não | — | Conta dona, FK `RESTRICT`. |
| `account_ledger.delta` | `BIGINT` | não | — | Variação não nula do saldo (`M16`). |
| `account_ledger.balance_after` | `BIGINT` | não | — | Saldo após o lançamento, dentro da faixa de `coin`. |
| `account_ledger.reason` | `TEXT` | não | — | Motivo operacional, 1–64 chars; vocabulário é contrato de aplicação. |
| `account_ledger.created_at` | `TIMESTAMPTZ` | não | `now()` | Instante do lançamento (`P-06`). |

### 6.3 `game.account_moderation`

| Coluna | Tipo | Nulo | Default | Significado / origem |
|---|---|---|---|---|
| `account_id` | `BIGINT` | não | — | PK/FK da conta, `RESTRICT`. |
| `blocked` | `BOOLEAN` | não | `false` | Estado disciplinar (`M1`). |
| `reason` | `TEXT` | não | `''` | Motivo obrigatório quando bloqueada. |
| `blocked_by` | `BIGINT` | sim | — | Pessoa staff autora; FK `portal.person` `RESTRICT`. |
| `blocked_at` | `TIMESTAMPTZ` | sim | — | Instante do bloqueio; nulo se liberada. |

### 6.4 `game.pin_reset`

| Coluna | Tipo | Nulo | Default | Significado / origem |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | ID interno do reset. |
| `account_id` | `BIGINT` | não | — | Conta alvo, FK `RESTRICT`. |
| `token_hash` | `TEXT` | não | — | SHA-256 hex do token; token em claro nunca vai ao banco (`M2`). |
| `requested_ip` | `INET` | sim | — | IP de origem, sem destinatário PII (`P-22`). |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | Emissão. |
| `expires_at` | `TIMESTAMPTZ` | não | — | Expiração; TTL fica na política (`B1`). |
| `used_at` | `TIMESTAMPTZ` | sim | — | Consumo único; linha retida para auditoria. |

## 7. Contratos e invariantes

| Objeto | Estado que impede | SQLSTATE |
|---|---|---|
| `ux_account_login` | login duplicado sem caixa | `23505` |
| `ux_account_slot` | mais de uma conta viva na mesma vaga ou mais de cinco vagas por pessoa | `23505` |
| `fk_account_person` | conta sem pessoa ou pessoa apagada | `23503` |
| `ck_account_*` | vaga, login, role/status, hash, contadores ou fechamento fora da regra | `23514` |
| `fk_account_wallet_account` / `fk_account_ledger_account` | carteira/lançamento sem conta; deleção da conta bloqueada | `23503` |
| `ck_account_wallet_coin` / `ck_account_ledger_*` | saldo fora da faixa, delta zero ou motivo inválido | `23514` |
| `fk_account_moderation_autor` + `ck_account_moderation_*` | bloqueio sem motivo, autor ou instante coerente | `23503` / `23514` |
| `fk_pin_reset_account`, `uq_pin_reset_token`, `ck_pin_reset_*` | reset órfão, hash repetido/malformado, expiração ou consumo anterior à criação | `23503` / `23505` / `23514` |
| NOT NULL | campo obrigatório ausente | `23502` |

## 8. Permissões

Default privileges de `0001`:

| Papel | Efeito | Origem |
|---|---|---|
| `wyd_game` | `SELECT, INSERT, UPDATE, DELETE` nas cinco tabelas | default privilege `game` |
| `wyd_read` | `SELECT` | default privilege `game` |
| `wyd_portal` | sem `USAGE` no schema `game`; não acessa tabelas cruas | fronteira `P-17` |
| `PUBLIC` | nenhum acesso | `0001`/provisionamento |

## 9. Prova de execução

PG17 efêmero, com `0001` e `0003` aplicados, como `wyd_migrator`. Deve existir pessoa de teste id 1.

```bash
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0005_game_account.sql
```

| Prova | Comando/consulta | Esperado |
|---|---|---|
| cinco tabelas | contar as cinco tabelas no schema `game` | `5` |
| login dobrado duplicado | inserir `Foo` e `foo` | segundo `23505` em `ux_account_login` |
| sexta vaga | criar slots 1–5 e tentar outra conta ativa | `23505` em `ux_account_slot` |
| conta sem pessoa | inserir `person_id=999` | `23503` em `fk_account_person` |
| carteira fora da faixa | inserir `coin=-1` ou `2147483648` | `23514` em `ck_account_wallet_coin` |
| ledger sem variação | inserir `delta=0` | `23514` em `ck_account_ledger_delta` |
| moderação sem autor | `blocked=true` sem `blocked_by` | `23514` em `ck_account_moderation_autor` |
| PIN em claro | inserir token não-hex/curto | `23514` em `ck_pin_reset_token_forma` |
| conta não se apaga | `DELETE FROM game.account WHERE id=1` com carteira | `23503` (`RESTRICT`) |
| portal não cruza fronteira | `SET ROLE wyd_portal; SELECT * FROM game.account;` | `42501 permission denied for schema game` |
| `down` remove lote | executar `down` e contar tabelas | `0` tabelas criadas pelo lote |

## 10. O que este arquivo não decide

| Assunto | Fonte dona | Efeito |
|---|---|---|
| Conta transferível/vendável | `lacunas.md` `A17` (fechada: não) | nenhuma coluna/tabela de titularidade |
| Ledger de moedas | `lacunas.md` `M16` (fechada: sim, ledger) | `account_ledger` append-only por contrato de código |
| Loja/cash | `lacunas.md` `A19` | tabelas do lote `0011`, não aqui |
| Regra do PIN/lockout | `lacunas.md` `B3`, código de segurança | o schema armazena hash/contadores, não fixa fluxo |

*Página de planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado.*