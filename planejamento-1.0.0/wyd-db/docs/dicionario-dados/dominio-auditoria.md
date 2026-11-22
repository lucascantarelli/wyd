> Pergunta: "o que o banco registra sobre o que aconteceu — ação de staff e tentativa de login — e o que ele não registra?"
> Público: Arquiteto de schema e desenvolvedor do `wyd-db`
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.3, §5.4, §5.8, §5.33; `migrations-checklist.md` §2; `0002-ops-audit.md`
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL fechado, série não aplicada.

## Propósito do domínio

Auditoria é **a trilha do que aconteceu**, separada do estado atual. Três perguntas diferentes, três
tabelas:

- `ops.audit_log` — "quem fez o quê, quando", para ação de pessoa, conta ou processo.
- `ops.login_attempt` — "quantas tentativas falharam para este identificador", para abuso e suporte.
- `ops.idempotency_record` — "esta ação já rodou com esta chave, e o que ela devolveu", para o contrato
do `wyd-admin` (`M30`, 2026-11-22).

Nenhuma das três é estado de jogo: elas **sobrevivem** ao ator. Por isso não há FK para `portal.person`
nem para `game.account` — apagar ou fechar o ator não pode apagar a prova.

**Não cobre:** estado operacional de sessão (`released_reason` em `game.session`, ver
[acesso](dominio-acesso.md)), configuração (`ops.config_revision`, ver
[configuração](dominio-configuracao.md)) nem telemetria do launcher (não tem tabela).

## Tabela de tabelas

| Tabela | Papel em uma linha | Escreve | Lê | Colunas |
|---|---|---|---|---|
| `ops.audit_log` (0002) | evento auditável com ator tripartido, ação, alvo e detalhe | jogo e portal (`INSERT` só) | `wyd_read`; jogo/portal leem por `SELECT` herdado | `gerado/audit_log.md` |
| `ops.login_attempt` (0002) | tentativa de autenticação com identificador **hasheado**, origem, IP e resultado | jogo e portal (`INSERT` só) | `wyd_read` | `gerado/login_attempt.md` |
| `ops.idempotency_record` (0033) | ação executada por chave, com status e corpo devolvidos | `wyd_admin` (`SELECT`, `INSERT`, `UPDATE`) | `wyd_read` | `gerado/idempotency_record.md` |

## Garantias do schema

**Append-only por permissão, não por trigger.** `0002` concede `INSERT` a `wyd_game` e `wyd_portal` e
**nunca** `UPDATE`/`DELETE`; o `SELECT` vem do default privilege de `0001`. Reescrever história exigiria
trocar privilégio, não só rodar SQL.

- **ator coerente:** `actor_kind` é `person`, `account` ou `system`; `system` não tem `actor_id`, e ator
  humano sempre tem `actor_ref` nulo. As duas `CHECK`s tornam o estado misto impossível.
- **ação não vazia:** `char_length(action) > 0`.
- **identificador nunca em claro:** `login_attempt.identifier_hash` é sha256 hex
  (`^[0-9a-f]{64}$`) do e-mail dobrado (portal) ou do login dobrado (jogo).
- **resultado finito:** `ok`, `bad_secret`, `bad_second_factor`, `blocked`, `locked`,
  `unknown_identifier`.
- **superfície finita:** `portal` ou `game`.

## Armadilhas de leitura

- `audit_log.target` e `detail` são `TEXT` herdado do legado, **não** `JSONB`. Virar JSONB é decisão que
  ninguém tomou; consulta não pode assumir estrutura dentro deles.
- `unknown_identifier` é registrado **de propósito**: é o sinal de enumeração de contas, e o hash de um
  endereço inexistente não é dado pessoal de ninguém.
- o **segredo nunca entra**, nem em claro nem hasheado: senha, PIN e código de cerimônia não têm coluna.
- `login_attempt.ip` é `INET` e anulável; índice parcial em `(ip, occurred_at DESC)` existe para abuso,
  não para identificar pessoa (`P-10`).
- `ops` é lido por `wyd_read`: por isso o identificador é hash. E-mail em claro em `ops` seria PII fora de
  `portal` (`P-22`).

## Retenção

Decisão `M22` (§5.8), não schema: `ops.audit_log` **730 dias**, append-only; o expurgo de
`login_attempt` e de sessão segue o mesmo registro. Nenhum `DELETE` de runtime está previsto.

`ops.idempotency_record` tem decisão própria (`M30`): **7 dias após `created_at`**, qualquer estado, por
expurgo diário de operação. O piso é a janela de replay de 24 h do chamador (`M17`) e o teto é o custo de
guardar corpo de resposta — ação de staff, volume baixo. `DELETE` também não é de runtime aqui.

## O que o registro de idempotência **não** é

- **Não é log de auditoria.** `audit_log` guarda a ação para sempre e sem corpo de resposta; o registro
guarda o corpo por 7 dias para responder replay. Quem precisa da trilha consulta `audit_log` pelo
`audit_id` que o registro carrega.
- **Não é fila nem trava.** A linha nasce e fecha na mesma transação da ação: `in_progress` não é
  commitado, então não há estado a expirar nem lease a renovar.
- **Não guarda segredo.** O corpo é a resposta da API para a mesma conta; nenhuma senha, PIN ou código de
  cerimônia entra ali, como em nenhuma tabela de `ops`.

## `H` — semântica não verificada

| Ponto | Falta |
|---|---|
| vocabulário de `action` | catálogo fechado de ações ainda não existe; hoje só `session_released` está citado |
| conteúdo de `target`/`detail` por ação | formato por ação é do código que escreve, não do banco |
| janela de retenção de `login_attempt` | `M22` fixa as outras três; esta não tem número próprio |
