> Pergunta: "como aplico o `0007_game_session` e como cadastro a topologia de canais e a sessão viva?"
> Público: Desenvolvedor novo e operação do servidor
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3; `ddl-confirmado.md` §5.5; `padrao-banco.md` §2/§5/§8/§9; `lacunas.md` (`A10`, `A14`, `M23`)
> Revisado em: 2026-11-20

**Estado:** aceito (2026-11-24)

Sétimo arquivo da série. Cria a topologia `server_channel` e as sessões de jogo. A configuração de cada
instalação (grupo, `host`, URL de status e limites) é dado operacional, inserido fora da migration; uma
sessão só pode referenciar canal cadastrado. O arquivo mantém sessão encerrada para auditoria e
reaper.

!!! note "Aplicado em 2026-11-24"
    O arquivo `crates/wyd-db/migrations/0007-game-session.sql` existe e a série aplicou do zero em PostgreSQL 17
    nessa data, como `wyd_migrator` (`planejamento-1.0.0/wyd-db/pendencias-de-fechamento.md` §6.6.30). O
    DDL continua sendo o de `ddl-confirmado.md` §5.5, dono do texto; as provas de §9 não foram executadas uma a uma
    nesta leva.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0007` |
| Arquivo | `crates/wyd-db/migrations/0007-game-session.sql` |
| Schema | `game` |
| Posição | 7 de 34 (topologia de canais e sessão viva) |
| Depende de | `0001` (schemas/função), `0005` (`game.account`), `0006` (`game.character`) |
| É dependência de | `0008`+ usam `game.character`; sessão referencia topologia criada aqui |
| Tabelas criadas | 2 (`game.server_channel`, `game.session`) |
| Objetos criados | 2 tabelas, 6 índices, 1 trigger |
| `SCHEMA_VERSION` | não muda aqui: a troca para `34` acontece no procedimento de `plano-migrations.md` §6 |

## 2. Objetivo

Fixar a topologia de canal por chave composta `(group_id, channel)` e registrar uma sessão por conta e
personagem, com sinais de vida, liberação, motivo e índices para ocupação, auditoria e reaper.

## 3. Pré-requisitos

1. `0001`, `0005` e `0006` aplicados; `game.account` e `game.character` existem.
2. Canais de instalação criados operacionalmente em `game.server_channel` antes do login; este arquivo
   **não** insere host ou IP de ambiente.
3. Conexão como `wyd_migrator`.

## 4. DDL do `up`

Texto confirmado em `ddl-confirmado.md` §5.5.

```sql
-- migrate:up
CREATE TABLE game.server_channel (
    group_id    SMALLINT NOT NULL,
    channel     SMALLINT NOT NULL,
    host        TEXT NOT NULL,
    status_url  TEXT NOT NULL DEFAULT '',
    enabled     BOOLEAN NOT NULL DEFAULT true,
    max_players INTEGER,
    maintenance BOOLEAN,
    role        TEXT,
    note        TEXT NOT NULL DEFAULT '',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_server_channel PRIMARY KEY (group_id, channel),
    CONSTRAINT ck_server_channel_grupo CHECK (group_id BETWEEN 0 AND 9),
    CONSTRAINT ck_server_channel_canal CHECK (channel BETWEEN 0 AND 10),
    CONSTRAINT ck_server_channel_host CHECK (host <> ''),
    CONSTRAINT ck_server_channel_status_url CHECK (char_length(status_url) <= 32),
    CONSTRAINT ck_server_channel_jogavel CHECK (
        (channel = 0 AND max_players IS NULL AND maintenance IS NULL AND role IS NULL)
     OR (channel > 0 AND max_players IS NOT NULL AND maintenance IS NOT NULL AND role IS NOT NULL)
    ),
    CONSTRAINT ck_server_channel_max CHECK (max_players >= 0),
    CONSTRAINT ck_server_channel_role CHECK (role IN ('normal', 'verde'))
);
CREATE TRIGGER trg_server_channel_set_updated_at
    BEFORE UPDATE ON game.server_channel
    FOR EACH ROW
    EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.session (
    id             BIGINT GENERATED ALWAYS AS IDENTITY,
    account_id     BIGINT NOT NULL,
    character_id   BIGINT,
    group_id       SMALLINT NOT NULL,
    channel        SMALLINT NOT NULL,
    client_version INTEGER NOT NULL,
    ip             INET,
    opened_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    renewed_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    released_at    TIMESTAMPTZ,
    released_reason TEXT,
    CONSTRAINT pk_session PRIMARY KEY (id),
    CONSTRAINT fk_session_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE RESTRICT,
    CONSTRAINT fk_session_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE RESTRICT,
    CONSTRAINT fk_session_canal FOREIGN KEY (group_id, channel)
        REFERENCES game.server_channel (group_id, channel) ON DELETE RESTRICT,
    CONSTRAINT ck_session_canal CHECK (channel BETWEEN 1 AND 10),
    CONSTRAINT ck_session_client_version CHECK (client_version > 0),
    CONSTRAINT ck_session_sinal CHECK (renewed_at >= opened_at),
    CONSTRAINT ck_session_liberacao CHECK (released_at >= opened_at),
    CONSTRAINT ck_session_motivo CHECK (
        (released_at IS NULL AND released_reason IS NULL)
     OR (released_at IS NOT NULL AND released_reason IN ('disconnect', 'reaper', 'character_deleted', 'admin'))
    )
);
CREATE UNIQUE INDEX ux_session_viva_por_conta ON game.session (account_id)
    WHERE released_at IS NULL;
CREATE UNIQUE INDEX ux_session_viva_por_personagem ON game.session (character_id)
    WHERE released_at IS NULL AND character_id IS NOT NULL;
CREATE INDEX ix_session_canal ON game.session (group_id, channel, opened_at DESC);
CREATE INDEX ix_session_reaper ON game.session (renewed_at) WHERE released_at IS NULL;
CREATE INDEX ix_session_ip ON game.session (ip, opened_at DESC) WHERE ip IS NOT NULL;
```

## 5. DDL do `down`

Índices, constraints e triggers são dependentes das tabelas; o down remove primeiro `session`, depois
`server_channel`. A função do trigger pertence a `0001` e fica.

```sql
-- migrate:down
DROP TABLE game.session;
DROP TRIGGER trg_server_channel_set_updated_at ON game.server_channel;
DROP TABLE game.server_channel;
```

## 6. Tabelas de colunas

### 6.1 `game.server_channel`

| Coluna | Tipo | Nulo | Default | Significado / origem |
|---|---|---|---|---|
| `group_id` | `SMALLINT` | não | — | Identificador do grupo/instalação, 0–9 (protocolo/legado `H`). |
| `channel` | `SMALLINT` | não | — | 0 status, 1–10 jogáveis (`H`). |
| `host` | `TEXT` | não | — | Host da instalação; fornecido por operação, não seed. |
| `status_url` | `TEXT` | não | `''` | Endpoint de status do canal. |
| `enabled` | `BOOLEAN` | não | `true` | Canal habilitado para conexão. |
| `max_players` | `INTEGER` | sim | — | Capacidade do canal jogável; nulo na linha status; `1000` é valor de instalação (`H`). |
| `maintenance` | `BOOLEAN` | sim | — | Manutenção do canal jogável; nulo na linha status. |
| `role` | `TEXT` | sim | — | `normal`/`verde`; nulo na linha status (`M23`). |
| `note` | `TEXT` | não | `''` | Nota operacional. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Marcas; trigger `A9`. |

### 6.2 `game.session`

| Coluna | Tipo | Nulo | Default | Significado / origem |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | Identificador da sessão. |
| `account_id` | `BIGINT` | não | — | Conta autenticada; FK `RESTRICT`. |
| `character_id` | `BIGINT` | sim | — | Personagem selecionado; nulo antes da seleção, FK `RESTRICT`. |
| `group_id` / `channel` | `SMALLINT` | não | — | Chave composta de canal cadastrado. |
| `client_version` | `INTEGER` | não | — | Versão inteira recebida no handshake 7662 (`H`). |
| `ip` | `INET` | sim | — | IP para auditoria/multibox, não identidade (`B5`, `P-10`). |
| `opened_at` / `renewed_at` | `TIMESTAMPTZ` | não | `now()` | Abertura e heartbeat. |
| `released_at` | `TIMESTAMPTZ` | sim | — | Momento de desconexão/reaper/admin. |
| `released_reason` | `TEXT` | sim | — | Motivo obrigatório após liberação (`A14`). |

## 7. Contratos e invariantes

| Objeto | Estado que impede | SQLSTATE |
|---|---|---|
| `pk_server_channel` | duas linhas para o mesmo grupo/canal | `23505` |
| `ck_server_channel_*` | grupo/canal inválido, host vazio, URL longa ou capacidade/role incoerentes com canal 0 | `23514` |
| `fk_session_account` / `fk_session_character` / `fk_session_canal` | sessão sem conta/personagem/canal existente | `23503` |
| `ux_session_viva_por_conta` | segunda sessão viva da mesma conta | `23505` |
| `ux_session_viva_por_personagem` | segunda sessão viva do mesmo personagem | `23505` |
| `ck_session_*` | canal 0, versão inválida, timestamps invertidos ou motivo ausente/inválido | `23514` |
| índices parciais de `released_at IS NULL` | mantêm unicidade somente para sessões ativas | — |

## 8. Permissões

Default privileges do `0001`:

| Papel | Efeito | Origem |
|---|---|---|
| `wyd_game` | `SELECT, INSERT, UPDATE, DELETE` nas duas tabelas | default privilege `game` |
| `wyd_read` | `SELECT` | default privilege `game` |
| `wyd_portal` | sem `USAGE` em `game`; sem leitura crua | fronteira `P-17` |
| `PUBLIC` | nenhum acesso | provisionamento/`0001` |

## 9. Prova de execução

PG17 efêmero, com `0001`, `0005`, `0006` aplicados. A fixture abaixo é de teste, não parte da
migration; dados de host de produção nunca são versionados.

```bash
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0007-game-session.sql
```

| Prova | Comando/consulta | Esperado |
|---|---|---|
| tabelas criadas | contar `server_channel` e `session` em `game` | `2` |
| canal 0 com capacidade | inserir status channel 0 com `max_players=1000` | `23514` em `ck_server_channel_jogavel` |
| canal jogável sem role | inserir channel 1 com `role=NULL` | `23514` em `ck_server_channel_jogavel` |
| fixture de canal | inserir `channel=1`, `host='127.0.0.1'`, `max_players=1000`, `maintenance=false`, `role='normal'` | sucesso |
| sessão sem canal | inserir sessão em par inexistente | `23503` em `fk_session_canal` |
| segunda sessão da conta | inserir duas sessões sem `released_at` da mesma conta | segunda `23505` em `ux_session_viva_por_conta` |
| sessão liberada sem motivo | `released_at=now(), released_reason=NULL` | `23514` em `ck_session_motivo` |
| liberação antes da abertura | `released_at < opened_at` | `23514` em `ck_session_liberacao` |
| IP indexado só quando presente | consultar definição de `ix_session_ip` | índice parcial `WHERE ip IS NOT NULL` |
| `down` remove o lote | executar down, contar as duas tabelas | `0` |

## 10. O que este arquivo não decide

| Assunto | Fonte dona | Efeito |
|---|---|---|
| Quem libera a sessão e reaper | `lacunas.md` `A10` (fechada/revisada por `A14`) | canal no disconnect, reaper 60 s/stale 180 s |
| Personagem apagado com sessão viva | `lacunas.md` `A14` | `released_reason='character_deleted'`; personagem soft-delete |
| Topologia e fusão de capacidade/papel | `M23`, decisão refletida em §5.5 | chave composta e campos fundidos |
| Endpoints/hosts por ambiente | operação, não migration | inserir `server_channel` fora da série; sem IP/host real aqui |
| Contagem máxima de canais por instalação | planejamento de operação | banco aplica faixas absolutas; deployment define instâncias |

*Página de planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado.*