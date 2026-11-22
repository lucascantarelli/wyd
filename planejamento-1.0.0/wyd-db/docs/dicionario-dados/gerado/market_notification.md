> Pergunta: "o que `portal.market_notification` guarda do aviso do mercado e por que a referência não tem FK?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.34; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Aviso do mercado para uma conta: lance recebido, lance superado, arrematação, venda, expiração, disputa,
decisão e avaliação. É caixa de entrada do portal, não trilha de auditoria — o fato mora na tabela de
origem.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | gerado | identidade (`P-03`) |
| `account_id` | `BIGINT` | não | — | conta destinatária; sem FK |
| `kind` | `TEXT` | não | — | tipo do aviso em lista fechada |
| `ref_kind` | `TEXT` | não | — | tipo do documento de origem |
| `ref_id` | `BIGINT` | não | — | id do documento de origem |
| `read_at` | `TIMESTAMPTZ` | sim | — | leitura; nulo = não lida |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | carimbo de criação |

## Chave primária

`pk_market_notification (id)` — identidade sintética; o mesmo evento pode gerar aviso para contas
diferentes, e o par evento/destinatário não é chave.

## FKs de saída e de entrada

- **Saída:** nenhuma. `account_id` é número de `game.account` (`P-17`) e `ref_kind`/`ref_id` são
  **referência polimórfica sem FK**: `anuncio`, `transacao` e `disputa` são tabelas diferentes, e alvo
  múltiplo não tem FK única.
- **Entrada:** nenhuma.

## Unicidade

Só a PK. O DDL **não** impede o mesmo aviso duas vezes: deduplicar é regra de quem escreve, e a chave
natural seria `(account_id, kind, ref_kind, ref_id)` — que este lote não declarou.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_market_notification_tipo` | `kind` fora de `lance_recebido`, `lance_superado`, `arrematado`, `vendido`, `expirado`, `disputa_aberta`, `disputa_decidida` e `avaliacao_recebida` |
| `ck_market_notification_ref` | `ref_kind` fora de `anuncio`, `transacao` e `disputa` |

`ref_id` é **obrigatório** nas três: aviso sem documento de origem não existe neste schema.

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_market_notification_conta_nao_lida (account_id) WHERE read_at IS NULL` | contador e lista de não lidas da conta; parcial para não carregar o histórico lido |

## Armadilha de leitura

`read_at IS NULL` é o único sinal de não lida; `created_at` não diz nada sobre leitura. `kind` e `ref_kind`
**não** são a mesma coisa: `vendido` aponta para `transacao`, `expirado` para `anuncio`, e `disputa_decidida`
para `disputa` — juntar exige `CASE`. Como não há FK, `ref_id` pode apontar para linha já expurgada, e o
aviso continua válido como notificação, não como prova. A retenção desta tabela ainda não foi decidida
(`ddl-confirmado.md` §5.34, "o que este lote não decide"), então não presuma expurgo.

## Papéis

- **Escreve:** `wyd_portal` — `SELECT, INSERT, UPDATE, DELETE` herdados do default privilege de `portal`;
  `read_at` é movido por `UPDATE`.
- **Lê:** `wyd_portal` e `wyd_read` (herdado); `wyd_game` não alcança `portal`.

## Migration

Forma (dona): `migrations/0034-portal-market.md` — arquivo `0034_portal_market.sql`; DDL em
`ddl-confirmado.md` §5.34.
