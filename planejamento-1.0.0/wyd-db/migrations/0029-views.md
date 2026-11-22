> Pergunta: "como aplico o `0029_views` e como o portal lê presença sem acessar as tabelas do jogo?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3 (linha 29); `ddl-confirmado.md` §5.29 (confirmado em 2026-11-21); `padrao-banco.md` §7 (`P-17`); `0007` (`game.session`), `0003` (`portal.person`)
> Revisado em: 2026-11-21

Vigésimo nono arquivo da série, e o único sem tabela. Cria as duas views de
leitura cruzada prometidas por `P-17`: presença para o portal e identidade mínima para o jogo.

!!! note "Aprovado em 2026-11-21"
    O dono aprovou o conjunto em 2026-11-21 e respondeu a escolha de permissão de §8: **sem permissão por
    enquanto**. As duas views existem e ficam ilegíveis pelo outro domínio até decisão explícita de
    fronteira. O lote está `concluída` em `migrations-checklist.md` §2.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0029` (checklist, linha 113) |
| Arquivo | `crates/wyd-db/migrations/0029_views.sql` |
| Schema | `game` e `portal` (2 views, 0 tabelas) |
| Posição | 29 de 34 |
| Depende de | `0001`; `0003` (`portal.person`); `0006` (`game.character`); `0007` (`game.session`) |
| É dependência de | nenhuma da série (último objeto de leitura) |
| Objetos criados | 2 views, 0 tabelas, 0 índices, 0 triggers |
| `SCHEMA_VERSION` | Continua `53` até a troca da série (`plano-migrations.md` §6) |

## 2. Objetivo

Dar ao portal a presença viva do jogo e ao jogo a identidade mínima da pessoa, sem que nenhum dos dois
ganhe acesso às tabelas internas do outro (`P-17`, `padrao-banco.md` §7).

## 3. Pré-requisitos

1. `0001_bootstrap`: schemas e `USAGE`; `game.session` e `portal.person` existem depois de `0007`/`0003`.
2. `0006_game_character`: `game.character(id, name)` para o nome exibido na presença.
3. Conexão como `wyd_migrator`; a view pertence a ele e roda com privilégio do dono.

## 4. DDL do `up`

Texto confirmado em `ddl-confirmado.md` §5.29, copiado sem alteração.

```sql
-- 0029_views: presença para o portal e identidade da pessoa para o jogo.
-- As duas views existem para que nenhum domínio leia a tabela interna do outro (P-17, padrao-banco §7).
-- Não usa COMMENT ON: a série documenta semântica nas páginas de migration.
-- Nenhum GRANT neste arquivo: permissão de leitura é decisão própria (ver página §7 e §10).

-- migrate:up
-- Presença viva por sessão: sem IP, sem e-mail e sem tabela interna do jogo exposta.
-- LEFT JOIN porque a sessão nasce no handshake, antes da seleção de personagem.
CREATE VIEW game.v_live_session AS
SELECT s.id             AS session_id,
       s.account_id     AS account_id,
       s.character_id   AS character_id,
       c.name           AS character_name,
       s.group_id       AS group_id,
       s.channel        AS channel,
       s.client_version AS client_version,
       s.opened_at      AS opened_at,
       s.renewed_at     AS renewed_at
FROM game.session s
LEFT JOIN game.character c ON c.id = s.character_id
WHERE s.released_at IS NULL;

-- Identidade mínima da pessoa para o jogo: id, papel e estado; nunca e-mail, hash ou PIN.
CREATE VIEW portal.v_person_identity AS
SELECT p.id     AS person_id,
       p.role   AS role,
       p.status AS status
FROM portal.person p;
```

## 5. DDL do `down`

```sql
-- migrate:down
DROP VIEW portal.v_person_identity;
DROP VIEW game.v_live_session;
```

## 6. Colunas

Não há ficha de tabela; as colunas abaixo vêm do propósito fixado em `padrao-banco.md` §7 e das
tabelas de origem (`0003`, `0006`, `0007`).

### `game.v_live_session`

| Coluna | Origem | Por quê |
|---|---|---|
| `session_id` | `P` | `game.session.id`; identifica a sessão viva. |
| `account_id` | `P` | Conta dona; o painel conta jogadores por conta. |
| `character_id` | `P` | Personagem selecionado; nulo antes da seleção. |
| `character_name` | `P` | `game.character.name`, para exibição; `LEFT JOIN` por causa da sessão sem personagem. |
| `group_id` | `P` | Grupo/instalação e canal, como em `0007`. |
| `channel` | `P` | Canal jogável (1–10). |
| `client_version` | `P` | Versão do cliente no handshake. |
| `opened_at` | `P` | Abertura da sessão. |
| `renewed_at` | `P` | Último sinal de vida; é o que separa sessão viva de zumbi. |

Fora da view: `ip` (`P-10`), `released_at`/`released_reason` (a view só mostra viva) e qualquer coluna
de `game.account`.

### `portal.v_person_identity`

| Coluna | Origem | Por quê |
|---|---|---|
| `person_id` | `P` | `portal.person.id`. |
| `role` | `P` | Papel no portal (`member`, `moderator`, `admin`). |
| `status` | `P` | Estado (`pending`, `active`, `suspended`, `closed`). |

Fora da view: `email`, `email_fold`, `password_hash`, `pin_hash`, contadores de falha, carimbos de
suspensão e `closed_at` (`A12`; `padrao-banco.md` §7).

## 7. Constraints e índices

- Views não têm PK, FK, CHECK nem índice próprio. O filtro `released_at IS NULL` é a única condição de
  linha, e o `LEFT JOIN` preserva a sessão sem personagem.
- Nenhum `GRANT` neste arquivo. `0001` não dá `USAGE` em `game` ao `wyd_portal` nem `USAGE` em `portal`
  ao `wyd_game`, então a view existe e ainda não é legível pelo outro lado; a permissão é a escolha
  levada ao dono em §8.
- Sem `security_invoker`: a view roda com privilégio do dono e não exige `SELECT` nas tabelas de
  origem. Se o dono escolher expor a leitura de outra forma, o `security_invoker` volta à mesa.

### Diferenças em relação ao plano

| Plano | Aqui | Por quê |
|---|---|---|
| `game.v_live_session` “vive no schema de quem lê” (`P-17`) | vive em `game` | o nome é fixado em `padrao-banco.md` §7 e citado pelo `wyd-portal`; mover para `portal` trocaria esse contrato |
| leitura pelo portal | sem `GRANT` | permissão é decisão de fronteira; ver §8 e §10 |

## 8. Decisões de forma

1. Presença projeta só sessão viva, sem IP e sem tabela de conta.
2. Identidade projeta só `person_id`, `role` e `status`, nunca e-mail ou credencial.
3. Leitura pelo portal: nenhuma permissão por enquanto, respondido pelo dono em 2026-11-21. Sem
   `GRANT USAGE` no schema cruzado e sem `SELECT` na view; mover a view para o schema do leitor fica
   fora até nova decisão.

## 9. Prova prevista (não executada)

1. `SELECT * FROM game.v_live_session` como `wyd_migrator` → só sessões com `released_at IS NULL`.
2. Sessão aberta sem personagem → aparece com `character_name` nulo.
3. `SELECT * FROM portal.v_person_identity` → três colunas, sem e-mail.
4. `SET ROLE wyd_portal; SELECT * FROM game.v_live_session` → `42501` (sem `USAGE`), e continua assim
   com a decisão de 2026-11-21; o portal não lê a view até nova decisão de fronteira.
5. `down` derruba as duas views na ordem inversa, sem erro de dependência.

## 10. O que não decide

| Assunto | Dono/fechamento |
|---|---|
| `GRANT`/`USAGE` que tornaria a view legível | decidido em 2026-11-21: sem permissão por enquanto; `0001` é fechado e não muda por conta própria |
| Política de retenção da sessão | `M22` (365 dias após `released_at` para `game.session`) |
| Painel que consome a presença | `wyd-portal`, `LAC-POR-06` |
| View materializada | proibida por `P-17` até haver medição |

*Página de planejamento. Nenhum SQL foi executado nem arquivo `.sql` criado.*
