> Pergunta: "quem escreve as sete tabelas do C4 e o `ops.audit_log`, e com que privilégio?"
> Público: Desenvolvedor novo e segurança
> Dono: equipe do `wyd-db`
> Base: `lacunas.md` `M27` (decidida em 2026-11-21); `ddl-confirmado.md` §5.32; `docs/arquitetura/seguranca.md` §"Decisão — papel `wyd_admin`"; `wyd-admin/docs/arquitetura/contrato-c4.md` §4
> Revisado em: 2026-11-22

Trigésimo segundo arquivo da série e o único sem tabela: só `GRANT`s para o papel `wyd_admin`. O papel
**não** nasce aqui — é criado fora da série por `db/roles.sql` (`src/provision/spec.md` §2 e §4), como
os outros cinco.

!!! note "Decidido em 2026-11-21"
    O dono fechou `M27` na opção **A + S1 + C1 + D1**: `wyd_admin` com `LOGIN`, sem membership, `SELECT`
    nas sete tabelas do C4, auditoria por `INSERT` em `ops.audit_log` e `GRANT` em arquivo novo. A série
    passa de **31 para 32 arquivos**, `Tab.` continua **97** e `SCHEMA_VERSION` iria a **32** na troca.

!!! note "Emenda de 2026-11-22 (`M31`)"
    A custódia de item do mercado (`ADM-10`, C4 §4.9–§4.13) escreve `game.character_item` e lê
    `game.character`; sem esses dois privilégios o contrato aprovado não executava. O arquivo ganha
    `SELECT, INSERT, DELETE` na mochila e `SELECT` no personagem. A série segue para **34 arquivos**
    com os lotes `0033` e `0034` e `Tab.` vai a **109** — `SCHEMA_VERSION = 34` na troca.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0032` (decidido em 2026-11-21; próximo livre depois de `0031`) |
| Arquivo | `crates/wyd-db/migrations/0032_admin_grants.sql` |
| Schema | `game` + `ops` (sem tabela) |
| Posição | 32 de 34 (privilégio do papel de staff; os lotes `0033` e `0034` vêm depois) |
| Depende de | `0001` (schemas e default privileges), `0002` (`ops.audit_log`), `0005` (`account_moderation`, `account_wallet`, `account_ledger`), `0007` (`session`, `server_channel`), `0011` (`city`, `item_price`) e do papel `wyd_admin` já criado por `db/roles.sql` |
| É dependência de | nada na série: nenhuma view de `0029` e nenhuma tabela o leem |
| Tabelas criadas | 0 |
| Objetos criados | 0 — 13 `GRANT`s (2 de schema, 10 de tabela e 1 de coluna) |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca de série (`plano-migrations.md` §6); na troca, o valor acompanha o total de arquivos |

**Por que no fim da série:** privilégio é o último passo — ele precisa das tabelas e do papel. Nenhum
arquivo anterior depende dele, então entrar depois de `0031` não reordena nada.

## 2. Por que arquivo novo (`D1`)

O `0001` concede `USAGE` e default privileges por schema; reabri-lo para um sexto papel obrigaria a
reescrever DDL aplicado, que o runner trata como imutável (`plano-migrations.md` §2). O `db/roles.sql`
do provision não é alternativa: `src/provision/spec.md` §1 exclui privilégio de schema da série dali —
ele só cria papel e mexe em `public`. Sobra um arquivo na série, sem tabela e sem objeto.

## 3. DDL

`USAGE` no schema é o direito de **atravessar**; a escrita vem dos privilégios por tabela. Os dois são
necessários — sem o `USAGE`, o `GRANT` de tabela responderia `42501 permission denied for schema game`
(`0001` §7).

```sql
-- migrate:up
-- Papel de staff do `wyd-admin` (`M27`): criado fora da série por `db/roles.sql`.
GRANT USAGE ON SCHEMA game TO wyd_admin;
GRANT USAGE ON SCHEMA ops TO wyd_admin;

GRANT SELECT, INSERT, UPDATE ON game.account_moderation TO wyd_admin;
GRANT SELECT, UPDATE ON game.account_wallet TO wyd_admin;
GRANT SELECT, INSERT ON game.account_ledger TO wyd_admin;
GRANT SELECT, UPDATE ON game.server_channel TO wyd_admin;
GRANT SELECT, UPDATE ON game.session TO wyd_admin;
GRANT SELECT, UPDATE ON game.city TO wyd_admin;
GRANT SELECT, UPDATE ON game.item_price TO wyd_admin;
-- Custódia de item do mercado (`M31`, C4 §4.9–§4.13): o item sai da mochila e volta para ela na mesma
-- transação. `DELETE` existe só aqui, e só nesta tabela.
GRANT SELECT, INSERT, DELETE ON game.character_item TO wyd_admin;
GRANT SELECT ON game.character TO wyd_admin;

GRANT INSERT ON ops.audit_log TO wyd_admin;
GRANT SELECT (id) ON ops.audit_log TO wyd_admin;
```

```sql
-- migrate:down
REVOKE SELECT (id) ON ops.audit_log FROM wyd_admin;
REVOKE INSERT ON ops.audit_log FROM wyd_admin;

REVOKE SELECT ON game.character FROM wyd_admin;
REVOKE SELECT, INSERT, DELETE ON game.character_item FROM wyd_admin;
REVOKE SELECT, UPDATE ON game.item_price FROM wyd_admin;
REVOKE SELECT, UPDATE ON game.city FROM wyd_admin;
REVOKE SELECT, UPDATE ON game.session FROM wyd_admin;
REVOKE SELECT, UPDATE ON game.server_channel FROM wyd_admin;
REVOKE SELECT, INSERT ON game.account_ledger FROM wyd_admin;
REVOKE SELECT, UPDATE ON game.account_wallet FROM wyd_admin;
REVOKE SELECT, INSERT, UPDATE ON game.account_moderation FROM wyd_admin;

REVOKE USAGE ON SCHEMA ops FROM wyd_admin;
REVOKE USAGE ON SCHEMA game FROM wyd_admin;
```

## 4. Privilégio por rota do C4

| Tabela | Rota | Privilégio | Por que |
|---|---|---|---|
| `game.account_moderation` | §4.1 | `SELECT`, `INSERT`, `UPDATE` | upsert do bloqueio, conferindo `expected_blocked` |
| `game.account_wallet` | §4.2 | `SELECT`, `UPDATE` | trava, confere `expected_coin` e grava o saldo |
| `game.account_ledger` | §4.2 | `SELECT`, `INSERT` | lançamento com `balance_after`, devolvido no `200` |
| `game.server_channel` | §4.3 | `SELECT`, `UPDATE` | estado do canal 1–10 |
| `game.session` | §4.4 | `SELECT`, `UPDATE` | `released_at`/`released_reason` só depois do ack |
| `game.city` | §4.6 | `SELECT`, `UPDATE` | alíquota com `expected_tax_rate_bps` |
| `game.item_price` | §4.7 | `SELECT`, `UPDATE` | preços com os dois `expected_*` |
| `game.character_item` | §4.9–§4.13 (emenda de 2026-11-22) | `SELECT`, `INSERT`, `DELETE` | custódia: a instância sai da mochila (`DELETE`) e volta na entrega/devolução (`INSERT`) |
| `game.character` | §4.9–§4.13 (emenda de 2026-11-22) | `SELECT` | conferir o titular do item antes de custodiá-lo |
| `ops.audit_log` | todas | `INSERT` + `SELECT (id)` | auditoria da ação; `audit_id` continua no `200`, então a coluna precisa ser lida |

**`DELETE` só na mochila.** `game.character_item` é a única tabela em que apagar linha **é** a operação:
adquirir um item é tirá-lo do inventário, e a custódia é a transação que o faz (`ddl-confirmado.md`
§5.34). Nas outras tabelas o `DELETE` não existe — nenhuma rota do C4 pede, e apagar estado vivo continua
sendo do `wyd_game` (`0001` §7).

## 5. O que este arquivo não faz

- **Não cria o papel.** `CREATE ROLE` é do provisionamento (`ADR-DB-0001`); se `db/roles.sql` não rodou,
  o `GRANT` falha com `role "wyd_admin" does not exist`.
- **Não alcança `portal` nem `content`.** As quatro tabelas do `0030` continuam do BFF com `wyd_portal`;
  a divergência de prosa sobre quem é a "API interna" foi fechada em `M28` (redação B, 2026-11-21).
- **Não vale para tabela futura.** Não há `ALTER DEFAULT PRIVILEGES` aqui — decisão `M29` (2026-11-22);
  a regra para tabela nova e a medição estão em `docs/arquitetura/seguranca.md`, e a decisão, em
  `lacunas.md` §2.3.
- **Não tira nada de outro papel.** O `INSERT` em `ops.audit_log` de `wyd_game`/`wyd_portal` (`0002`)
  continua como está; o `down` revoga só o que este arquivo concedeu.

## 6. Provas

| Invariante | Como é verificado |
|---|---|
| nove tabelas e um log, com `DELETE` só na mochila | `crates/wyd-db/tests/contrato/catalogo.rs` confere a lista de `GRANT` contra o C4 |
| escrita de staff não alcança `portal`/`content` | tentativa de `INSERT` como `wyd_admin` numa tabela de `portal` devolve `42501 permission denied for schema portal` |
| `USAGE` presente nos dois schemas | sem ele o `GRANT` de tabela falha; o teste aplica `0032` sobre `db/roles.sql` num PG17 efêmero |
| `down` deixa zero privilégio | `REVOKE` de todos os `GRANT` do `up`, na ordem inversa; `has_schema_privilege`/`has_table_privilege` voltam a `false` |
| idempotência do arquivo | a série não reaplica arquivo aplicado (drift do ledger), e o `down`/`up` do `make migrations-apply` termina limpo |
