> Pergunta: "o que o `0002_ops_audit` cria no banco, e por que a auditoria e o histórico de configuração moram no mesmo arquivo?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3; `ddl-confirmado.md` §5.3, §5.4 e §5.9; `padrao-banco.md` §1/§5/§9
> Revisado em: 2026-11-20

**Estado:** aceito (2026-11-20)

Segundo arquivo da série. Ele cria a **trilha do que aconteceu** — auditoria administrativa e tentativa
de autenticação — e a **trilha do que foi reconfigurado** — revisão de configuração e as mudanças que
ela aplicou. As quatro tabelas são append-only por permissão: nenhum papel de runtime recebe `UPDATE`
ou `DELETE`.

!!! note "Aprovado em 2026-11-20"
    O lote foi aprovado pelo dono na rodada de perguntas de 2026-11-20, na opção "Aprovar com as
    mudanças que marquei nas perguntas acima". As quatro escolhas marcadas eram exatamente as já
    escritas no desenho, então nenhuma mudança foi aplicada e a condição da aprovação ficou satisfeita.
    O DDL abaixo é o confirmado em `ddl-confirmado.md` §5.3, §5.4 e §5.9, e este arquivo é `concluída`
    no checklist.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0002` |
| Arquivo | `crates/wyd-db/migrations/0002-ops-audit.sql` |
| Schema | `ops` |
| Posição | 2 de 34 (primeiro arquivo do bloco operacional) |
| Depende de | `0001_bootstrap` — schemas, papéis e `ALTER DEFAULT PRIVILEGES`; sem ele o `GRANT` daqui falha com `42501` |
| É dependência de | nenhum arquivo da série: nenhuma tabela de outro arquivo referencia `ops.config_*`, e as duas tabelas de auditoria não têm FK por desenho |
| Tabelas criadas | 4 (`ops.audit_log`, `ops.login_attempt`, `ops.config_revision`, `ops.config_change`) |
| Objetos criados | 4 tabelas, 9 índices, 4 `GRANT` (um por tabela) |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca de série (`plano-migrations.md` §6) |

## 2. Objetivo

Registrar, sem que nenhum runtime possa reescrever ou apagar, **quem fez o quê** (`audit_log`), **quem
tentou autenticar e como falhou** (`login_attempt`) e **qual configuração passou a valer, com valores
antes e depois** (`config_revision` + `config_change`).

## 3. Pré-requisitos
**Sem arquivo no repositório:** `db/roles.sql` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

1. `db/roles.sql` aplicado por `wyd-db provision apply` — os papéis do cluster existem antes do `0001`.
2. `0001_bootstrap` aplicado: `ops` existe, pertence a `wyd_owner`, e `wyd_game`/`wyd_portal` têm
   `USAGE` nele (é o que faz o `GRANT INSERT` valer; sem `USAGE` o erro seria `42501` de schema).
3. Conexão como `wyd_migrator` — os default privileges do `0001` só alcançam objeto **criado por** ele.

## 4. DDL do `up`

O texto de `audit_log` e `login_attempt` é o de `ddl-confirmado.md` §5.3/§5.4. O de `config_revision`
e `config_change` é o de §5.9, confirmado em 2026-11-20.

```sql
-- 0002_ops_audit: auditoria, tentativa de login e histórico de configuração.
-- Depende de `0001` (schemas, papéis, default privileges). Nenhuma FK para arquivo posterior.

-- migrate:up
CREATE TABLE ops.audit_log (
    id          BIGINT GENERATED ALWAYS AS IDENTITY,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- Ator tripartido (M18-A). Sem FK, por desenho: auditoria sobrevive ao ator.
    actor_kind  TEXT NOT NULL,
    actor_id    BIGINT,
    actor_ref   TEXT,
    action      TEXT NOT NULL,
    -- Herdados do legado (TEXT). Virar JSONB é decisão que ninguém tomou; não entra por gosto.
    target      TEXT NOT NULL DEFAULT '',
    detail      TEXT NOT NULL DEFAULT '',
    CONSTRAINT pk_audit_log PRIMARY KEY (id),
    CONSTRAINT ck_audit_log_actor_kind CHECK (actor_kind IN ('person', 'account', 'system')),
    CONSTRAINT ck_audit_log_actor_sistema CHECK ((actor_kind = 'system') = (actor_id IS NULL)),
    CONSTRAINT ck_audit_log_actor_ref CHECK ((actor_kind <> 'system') = (actor_ref IS NULL)),
    CONSTRAINT ck_audit_log_action CHECK (char_length(action) > 0)
);

CREATE INDEX ix_audit_log_occurred ON ops.audit_log (occurred_at DESC);
-- "o que o staff 42 fez?" é igualdade + faixa, nessa ordem: kind, id, depois o tempo.
CREATE INDEX ix_audit_log_actor ON ops.audit_log (actor_kind, actor_id, occurred_at DESC);

GRANT INSERT ON ops.audit_log TO wyd_game, wyd_portal;

CREATE TABLE ops.login_attempt (
    id              BIGINT GENERATED ALWAYS AS IDENTITY,
    occurred_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- De onde veio a tentativa: login do portal (e-mail) ou handshake do cliente 7662 (login).
    surface         TEXT NOT NULL,
    -- sha256 hex do `email_fold` (portal) ou do `login_fold` (jogo); nunca o valor em claro.
    identifier_hash TEXT NOT NULL,
    ip              INET,
    result          TEXT NOT NULL,
    CONSTRAINT pk_login_attempt PRIMARY KEY (id),
    CONSTRAINT ck_login_attempt_surface CHECK (surface IN ('portal', 'game')),
    CONSTRAINT ck_login_attempt_identifier CHECK (identifier_hash ~ '^[0-9a-f]{64}$'),
    CONSTRAINT ck_login_attempt_result CHECK (
        result IN ('ok', 'bad_secret', 'bad_second_factor', 'blocked', 'locked', 'unknown_identifier')
    )
);

CREATE INDEX ix_login_attempt_identifier ON ops.login_attempt (identifier_hash, occurred_at DESC);
CREATE INDEX ix_login_attempt_ip ON ops.login_attempt (ip, occurred_at DESC) WHERE ip IS NOT NULL;
CREATE INDEX ix_login_attempt_occurred ON ops.login_attempt (occurred_at DESC);

GRANT INSERT ON ops.login_attempt TO wyd_game, wyd_portal;

CREATE TABLE ops.config_revision (
    id               BIGINT GENERATED ALWAYS AS IDENTITY,
    -- Quem editou. Mesmo ator tripartido de `audit_log` (M18-A), sem FK: `portal.person` nasce em
    -- `0003`, arquivo posterior — FK para frente é proibida (`padrao-banco.md` §9, item 8).
    author_kind      TEXT NOT NULL,
    author_id        BIGINT,
    author_ref       TEXT,
    -- Por que a configuração mudou. Texto curto de operação, nunca dado de jogo.
    note             TEXT NOT NULL DEFAULT '',
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- Instante em que a revisão passou a valer. Uma revisão nasce aplicada: não há rascunho na 1.0.0.
    applied_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_config_revision PRIMARY KEY (id),
    CONSTRAINT ck_config_revision_autor CHECK (
        (author_kind = 'person' AND author_id IS NOT NULL AND author_ref IS NULL)
        OR (author_kind = 'system' AND author_id IS NULL AND author_ref IS NOT NULL)
    ),
    CONSTRAINT ck_config_revision_nota CHECK (char_length(note) <= 500),
    CONSTRAINT ck_config_revision_aplicacao CHECK (applied_at >= created_at)
);

CREATE INDEX ix_config_revision_aplicada ON ops.config_revision (applied_at DESC);
CREATE INDEX ix_config_revision_autor ON ops.config_revision (author_kind, author_id, applied_at DESC);

CREATE TABLE ops.config_change (
    id            BIGINT GENERATED ALWAYS AS IDENTITY,
    revision_id   BIGINT NOT NULL,
    -- Onde a mudança incide: `schema.tabela` + a chave da linha.
    target_schema TEXT NOT NULL,
    target_table  TEXT NOT NULL,
    target_key    TEXT NOT NULL,
    -- Valor antes e depois, em texto canônico do tipo da coluna alvo. Sem JSONB para estado de jogo.
    -- `old_value` nulo = a chave não existia antes; `new_value` nulo = a chave deixou de existir.
    old_value     TEXT,
    new_value     TEXT,
    CONSTRAINT pk_config_change PRIMARY KEY (id),
    CONSTRAINT fk_config_change_revision FOREIGN KEY (revision_id)
        REFERENCES ops.config_revision (id) ON DELETE RESTRICT,
    -- Uma chave muda uma vez por revisão; mudar de novo exige outra revisão.
    CONSTRAINT uq_config_change_alvo UNIQUE (revision_id, target_schema, target_table, target_key),
    CONSTRAINT ck_config_change_schema CHECK (target_schema IN ('content', 'game', 'portal', 'ops')),
    CONSTRAINT ck_config_change_tabela CHECK (target_table ~ '^[a-z][a-z0-9_]*$'),
    CONSTRAINT ck_config_change_chave CHECK (char_length(target_key) BETWEEN 1 AND 200),
    CONSTRAINT ck_config_change_muda CHECK (old_value IS DISTINCT FROM new_value),
    CONSTRAINT ck_config_change_tem_valor CHECK (old_value IS NOT NULL OR new_value IS NOT NULL),
    CONSTRAINT ck_config_change_tamanho CHECK (
        (old_value IS NULL OR char_length(old_value) <= 4000)
        AND (new_value IS NULL OR char_length(new_value) <= 4000)
    )
);

CREATE INDEX ix_config_change_revision ON ops.config_change (revision_id);
-- Histórico de uma chave, do mais novo para o mais antigo.
CREATE INDEX ix_config_change_alvo
    ON ops.config_change (target_schema, target_table, target_key, revision_id DESC);

-- O portal é o único runtime que escreve configuração (área administrativa). Sem UPDATE/DELETE.
GRANT SELECT, INSERT ON ops.config_revision TO wyd_portal;
GRANT SELECT, INSERT ON ops.config_change TO wyd_portal;
```

**Por que as duas tabelas de configuração ficam em `ops` e não em `content`.** `ops` é o schema do
que a operação fez, não do que o jogo usa: o jogo lê a configuração efetiva em `content.game_setting` e
nas tabelas de taxa, não o histórico de quem a mudou. O histórico é lido pela área administrativa do
portal (`wyd_portal`) e por `wyd_read`, e nunca pelo `wyd_game`.

**Por que não há FK para `portal.person`.** `portal` nasce em `0003`, depois deste arquivo, e o padrão
proíbe FK para tabela criada em arquivo posterior (`padrao-banco.md` §9, item 8). O autor usa o mesmo
ator tripartido de `audit_log` (`M18-A`), com o `CHECK` que exige identificação em qualquer dos dois
casos: revisão sem autor é estado impossível.

## 5. DDL do `down`

Ordem inversa exata, com os índices qualificados pelo mesmo motivo de `0004`: o runner fecha o
`search_path` da transação (`src/migrate/spec.md` §6).

```sql
-- migrate:down
REVOKE SELECT, INSERT ON ops.config_change FROM wyd_portal;
REVOKE SELECT, INSERT ON ops.config_revision FROM wyd_portal;
DROP INDEX ops.ix_config_change_alvo;
DROP INDEX ops.ix_config_change_revision;
DROP TABLE ops.config_change;
DROP INDEX ops.ix_config_revision_autor;
DROP INDEX ops.ix_config_revision_aplicada;
DROP TABLE ops.config_revision;
REVOKE INSERT ON ops.login_attempt FROM wyd_game, wyd_portal;
DROP INDEX ops.ix_login_attempt_occurred;
DROP INDEX ops.ix_login_attempt_ip;
DROP INDEX ops.ix_login_attempt_identifier;
DROP TABLE ops.login_attempt;
REVOKE INSERT ON ops.audit_log FROM wyd_game, wyd_portal;
DROP INDEX ops.ix_audit_log_actor;
DROP INDEX ops.ix_audit_log_occurred;
DROP TABLE ops.audit_log;
```

Depois deste `down`, `ops` continua existindo (criado em `0001`) e vazio de tabela.

## 6. Tabelas de colunas

### 6.1 `ops.audit_log`

| Coluna | Tipo | Nulo | Default | O que é |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | Chave interna da linha de auditoria. |
| `occurred_at` | `TIMESTAMPTZ` | não | `now()` | Instante do evento, sempre do servidor (`P-06`). |
| `actor_kind` | `TEXT` | não | — | Origem do ator: `person`, `account` ou `system` (`M18-A`). |
| `actor_id` | `BIGINT` | sim | — | `portal.person.id` ou `game.account.id`, conforme o tipo; sem FK, por desenho. |
| `actor_ref` | `TEXT` | sim | — | Nome do processo quando `actor_kind = 'system'`. |
| `action` | `TEXT` | não | — | Verbo auditado (ex.: `config.apply`, `person.suspend`). |
| `target` | `TEXT` | não | `''` | Alvo legível do evento, herdado do legado como texto. |
| `detail` | `TEXT` | não | `''` | Detalhe livre do evento; nunca segredo. |

### 6.2 `ops.login_attempt`

| Coluna | Tipo | Nulo | Default | O que é |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | Chave interna. |
| `occurred_at` | `TIMESTAMPTZ` | não | `now()` | Instante da tentativa. |
| `surface` | `TEXT` | não | — | `portal` (login web) ou `game` (handshake 7662). |
| `identifier_hash` | `TEXT` | não | — | sha256 hex do identificador dobrado; nunca o valor em claro (`P-22`). |
| `ip` | `INET` | sim | — | Origem da tentativa, para auditoria de abuso. |
| `result` | `TEXT` | não | — | Resultado observado, incluindo `unknown_identifier` (sinal de enumeração). |

### 6.3 `ops.config_revision`

| Coluna | Tipo | Nulo | Default | O que é | Origem |
|---|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | Número da revisão. Não há segundo contador: o id **é** o número. | decisão de 2026-11-20 (§5.9) |
| `author_kind` | `TEXT` | não | — | `person` (staff do portal) ou `system` (rotina). | `M18-A`, aplicado por decisão de 2026-11-20 |
| `author_id` | `BIGINT` | sim | — | `portal.person.id` quando `author_kind = 'person'`; sem FK (arquivo posterior). | `M18-A` |
| `author_ref` | `TEXT` | sim | — | Nome do processo quando `author_kind = 'system'`. | `M18-A` |
| `note` | `TEXT` | não | `''` | Motivo curto da revisão, escrito por quem editou. | lote 0002 |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | Instante em que a revisão foi submetida. | `P-06` |
| `applied_at` | `TIMESTAMPTZ` | não | `now()` | Instante em que a revisão passou a valer; revisão nasce aplicada. | decisão de 2026-11-20 |

### 6.4 `ops.config_change`

| Coluna | Tipo | Nulo | Default | O que é | Origem |
|---|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | Chave da linha de mudança. | lote 0002 |
| `revision_id` | `BIGINT` | não | — | Revisão dona; FK `RESTRICT` — mudança órfã é impossível. | lote 0002 |
| `target_schema` | `TEXT` | não | — | Schema do alvo, restrito a `content`, `game`, `portal` ou `ops`. | lote 0002 |
| `target_table` | `TEXT` | não | — | Tabela do alvo, em `snake_case` (`P-02`). | lote 0002 |
| `target_key` | `TEXT` | não | — | Chave da linha ou da configuração dentro da tabela. | lote 0002 |
| `old_value` | `TEXT` | sim | — | Valor anterior em texto canônico; nulo = chave não existia. | decisão de 2026-11-20 |
| `new_value` | `TEXT` | sim | — | Valor novo em texto canônico; nulo = chave removida. | decisão de 2026-11-20 |

Nenhuma coluna é `JSONB`: o valor é texto canônico do tipo da coluna alvo, e quem interpreta é o
código que lê a configuração efetiva.

## 7. Contratos e invariantes

| # | Objeto | O que proíbe | SQLSTATE |
|---:|---|---|---|
| 1 | `pk_audit_log`, `pk_login_attempt`, `pk_config_revision`, `pk_config_change` | duas linhas com o mesmo `id` | `23505` |
| 2 | `ck_audit_log_actor_kind` / `ck_audit_log_actor_sistema` / `ck_audit_log_actor_ref` | ator fora dos três tipos, `system` com id, ou ator com `ref` | `23514` |
| 3 | `ck_login_attempt_surface` / `ck_login_attempt_result` | superfície ou resultado fora da lista | `23514` |
| 4 | `ck_login_attempt_identifier` | hash que não seja sha256 hex de 64 | `23514` |
| 5 | `ck_config_revision_autor` | **revisão sem autor** — `person` sem `author_id` ou `system` sem `author_ref` | `23514` |
| 6 | `ck_config_revision_aplicacao` | revisão aplicada antes de ser criada | `23514` |
| 7 | `ck_config_revision_nota` | nota acima de 500 caracteres | `23514` |
| 8 | `fk_config_change_revision` | **mudança órfã** — `revision_id` que não existe; e `RESTRICT` impede apagar revisão com mudança | `23503` / `23503` |
| 9 | `uq_config_change_alvo` | duas mudanças na mesma chave dentro da mesma revisão | `23505` |
| 10 | `ck_config_change_schema` / `ck_config_change_tabela` / `ck_config_change_chave` | alvo fora dos quatro schemas, tabela fora de `snake_case`, chave vazia ou acima de 200 | `23514` |
| 11 | `ck_config_change_muda` | mudança que não muda nada (`old_value` = `new_value`) | `23514` |
| 12 | `ck_config_change_tem_valor` | linha sem valor nenhum, nem antes nem depois | `23514` |
| 13 | `ck_config_change_tamanho` | valor acima de 4.000 caracteres | `23514` |
| 14 | `NOT NULL` das colunas marcadas | revisão sem tipo de autor, mudança sem revisão ou sem alvo | `23502` |
| 15 | permissão | `wyd_portal` ou `wyd_game` tentando `UPDATE`/`DELETE` em qualquer das quatro tabelas | `42501` |

**O que o banco não expressa.** "A configuração efetiva em `content.game_setting` é igual ao
`new_value` da última revisão daquela chave" depende de leitura cruzada e é invariante de código, não
`CHECK`. O banco garante que a mudança tem revisão, autor, alvo e valor diferente; a conferência de que
o valor aplicado corresponde ao registrado é da aplicação que escreve as duas coisas na mesma
transação.

**Reconfigurar não apaga história.** Cada reconfiguração é uma linha nova em `config_revision` e N
linhas em `config_change`; voltar atrás é uma revisão nova com os valores antigos em `new_value`, nunca
`UPDATE` ou `DELETE`. Como `wyd_portal` não tem `UPDATE`/`DELETE`, isso é garantido por permissão, não
por disciplina de código (`P-16`).

## 8. Permissões

| Papel | `audit_log` / `login_attempt` | `config_revision` / `config_change` | Como |
|---|---|---|---|
| `wyd_read` | `SELECT` | `SELECT` | default privilege do `0001` em `ops` |
| `wyd_portal` | `INSERT` | `SELECT, INSERT` | `GRANT` explícito deste arquivo |
| `wyd_game` | `INSERT` | **nada** | `GRANT` explícito só de `INSERT` nas duas de auditoria |
| `wyd_owner` / `wyd_migrator` | dono | dono | criação |

`UPDATE` e `DELETE` não são concedidos a ninguém em runtime: append-only por permissão (`P-16`). O
`SELECT` do portal nas duas tabelas de configuração existe porque a área administrativa mostra o
histórico; ele não abre `audit_log` para o portal.

## 9. Prova de execução

PG17 efêmero, sem tocar a série herdada. Supõe `provision apply` feito e conexão como `wyd_migrator`.

```bash
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0001-bootstrap.sql
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0002-ops-audit.sql
```

| # | Prova | Comando | Esperado |
|---:|---|---|---|
| 1 | quatro tabelas em `ops` | `SELECT count(*) FROM information_schema.tables WHERE table_schema = 'ops';` | `4` |
| 2 | ator `person` com referência de processo | `INSERT INTO ops.audit_log (actor_kind, actor_id, actor_ref, action) VALUES ('person', 1, 'proc', 'x');` | `23514` em `ck_audit_log_actor_ref` |
| 3 | login com identificador em claro | `INSERT INTO ops.login_attempt (surface, identifier_hash, result) VALUES ('portal', 'a@b.c', 'ok');` | `23514` em `ck_login_attempt_identifier` |
| 4 | revisão sem autor | `INSERT INTO ops.config_revision (note) VALUES ('sem autor');` | `23502` em `author_kind` |
| 5 | revisão `person` sem id | `INSERT INTO ops.config_revision (author_kind, note) VALUES ('person', 'x');` | `23514` em `ck_config_revision_autor` |
| 6 | revisão válida | `INSERT INTO ops.config_revision (author_kind, author_id, note) VALUES ('person', 1, 'ajuste de taxa') RETURNING id;` | `INSERT 0 1` |
| 7 | mudança órfã | `INSERT INTO ops.config_change (revision_id, target_schema, target_table, target_key, old_value, new_value) VALUES (999, 'content', 'game_setting', 'exp_rate', '1', '2');` | `23503` em `fk_config_change_revision` |
| 8 | mudança sem mudança | `… VALUES (1, 'content', 'game_setting', 'exp_rate', '2', '2');` | `23514` em `ck_config_change_muda` |
| 9 | mudança sem alvo válido | `… VALUES (1, 'public', 'x', 'k', NULL, '2');` | `23514` em `ck_config_change_schema` |
| 10 | mudança válida | `… VALUES (1, 'content', 'game_setting', 'exp_rate', '1', '2');` | `INSERT 0 1` |
| 11 | mesma chave duas vezes na revisão | repetir a prova 10 | `23505` em `uq_config_change_alvo` |
| 12 | portal escreve configuração | `SET ROLE wyd_portal; INSERT INTO ops.config_revision (author_kind, author_id) VALUES ('person', 1);` | `INSERT 0 1` |
| 13 | portal não reescreve | `SET ROLE wyd_portal; UPDATE ops.config_revision SET note = 'x';` | `42501` |
| 14 | portal não lê auditoria | `SET ROLE wyd_portal; SELECT count(*) FROM ops.audit_log;` | `42501` |
| 15 | jogo não lê configuração | `SET ROLE wyd_game; SELECT count(*) FROM ops.config_change;` | `42501` |
| 16 | `down` derruba só o que criou | aplicar o bloco `-- migrate:down` e repetir 1 | `0` |
| 17 | ciclo fecha | refazer o `up` após o `down` | prova 1 de novo (`4`) |

A prova 4 usa `note` de propósito: o `NOT NULL` de `author_kind` responde antes do `CHECK`, e é o que
separa "sem autor" (`23502`) de "autor malformado" (`23514`).

## 10. O que este arquivo não decide

| Assunto | Onde é decidido | Efeito aqui |
|---|---|---|
| Forma final das duas tabelas de configuração | `ddl-confirmado.md` §5.9 (confirmada em 2026-11-20) | o DDL acima é o confirmado; mudança de forma exige lote novo |
| Qual configuração é editável e com que tipo | planejamento do `wyd-portal` e `content.game_setting` (`0019`) | as tabelas guardam alvo e valor como texto; não decidem o catálogo de chaves |
| Retenção do histórico de configuração | `M22` não cobre estas tabelas; hoje **sem purge** | nenhuma coluna de expiração; mudança de política é decisão nova |
| Formato do hash de identificador | `M6` (fechada) | `sha256` hex de 64 |
| Quem grava `actor_kind`/`actor_id` em cada ação | contrato de `repo/audit` | o banco só recusa o par incoerente |

*Página de planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado.*
