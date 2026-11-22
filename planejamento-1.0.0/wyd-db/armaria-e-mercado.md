> Pergunta: "que views e tabelas o banco precisa criar para a armaria e o mercado global do portal, com que chave, invariante e índice?"
> Público: arquitetura, `wyd-db` e desenvolvedor novo
> Dono: `wyd-db` (forma) com arquitetura (fronteira)
> Base: `padrao-banco.md` (`P-03`, `P-06`, `P-13`, `P-17`, `M16`, `M17`, `M25`); `0009`; `0029` §10; `0014`; `ddl-confirmado.md`; `lacunas.md` `M31`; `planejamento-1.0.0/wyd-portal/armaria-e-mercado.md`
> Revisado em: 2026-11-22

**Estado:** forma aprovada em 2026-11-22 (`M31`, fechando `LAC-ADM-07`). O lote é o
`0034_portal_market.sql`, o texto do DDL é `ddl-confirmado.md` §5.34 e a série passa a **34 arquivos e
109 tabelas**. Este documento continua sendo a **razão** da forma — fronteira, relação e consulta —, não
o texto executável.

## 1. Fronteira aplicada

| Regra | Efeito neste desenho |
|---|---|
| `wyd_portal` não tem `USAGE` em `game` (`P-17`; `0029` §10 mediu `42501` até em `game.v_live_session`) | as views de `game` existem para dar **forma** à leitura; a execução é pelo adapter autorizado do `wyd-db`, não por conexão SQL do BFF |
| Ação no processo do jogo só pelo `wyd-admin` (`LAC-POR-01`) | nenhuma escrita do portal em `game`; a custódia é executada pelo contrato de §5 |
| Moeda do mercado é o crédito da conta no portal | carteira e ledger em `portal`, separados de `game.account_wallet` (`M16`) |
| Item real, nunca cópia | o item sai de `game.character_item` na mesma transação que cria a custódia |
| `BIGINT GENERATED ALWAYS AS IDENTITY`, constraint nomeada, `TIMESTAMPTZ`, taxa em bps (`P-03`, `P-06`, `P-13`) | aplicado em todas as tabelas abaixo |

## 2. Views de leitura da armaria

Quatro views, nenhuma tabela nova. Todas são `SELECT` puro: sem `INSERT`, sem `UPDATE`, sem coluna
calculada por job, sem cópia materializada.

### 2.1 `game.v_character_armory` — cabeçalho do perfil

| Coluna | Origem | Regra |
|---|---|---|
| `character_id` | `game.character.id` | chave da view |
| `name`, `class`, `level`, `tier`, `content_version` | `game.character` | como está no banco |
| `guild_id`, `guild_name`, `guild_role` | `game.guild_member` (`role` em `leader`/`sub_leader`/`member`) → `game.guild` (`0010`) | `NULL` sem guilda |

Filtro: `deleted_at IS NULL`. **Não expõe `account_id`, e-mail, IP, sessão nem `coin`** — o perfil é
público e nenhum identificador de conta pode ser correlacionado por ele.

### 2.2 `game.v_character_equipment` — equipamento

| Coluna | Origem | Regra |
|---|---|---|
| `character_id`, `slot` | `game.character_item` | `slot BETWEEN 0 AND 15` (equipado; 16–79 é mochila e não entra) |
| `item_index`, `qty`, `refine`, `ancient`, `gem_effect`, `gem_value`, `flags` | `game.character_item` | como está; `flags` é `H` de semântica |

Leitura por `character_id` usa a PK `pk_character_item (character_id, slot)` (`0009`): prefixo de PK, sem
índice novo. Ordenação por slot é feita na consulta.

### 2.3 `game.v_character_skill` — habilidades aprendidas

| Coluna | Origem | Regra |
|---|---|---|
| `character_id` | `game.character_learned_skill` | — |
| `category` | `0..3` | `ck_character_learned_skill_categoria` limita a faixa |
| `skill_index` | bit ligado em `mask` (`SMALLINT` 0–255) | uma linha por bit ligado, na ordem do array de skill do servidor (`0008`); o desmembramento fica na view para o portal não reimplementar a máscara |

`character_short_skill` (atalhos) não entra: é preferência de interface do jogador, não retrato do
personagem.

### 2.4 `content.v_item_sheet` — ficha do item

| Coluna | Origem | Regra |
|---|---|---|
| `item_index`, `name`, `kind_slug`, `grade`, `effect_names`, `req_level`, `req_str`, `req_int`, `req_dex`, `req_con` | `content.item_catalog` (`0014`) | como está; `kind_slug` é a categoria e `name_fold` fica fora (só serve à busca interna) |
| `stat_source` | literal | `'effect_names'` enquanto as estatísticas numéricas não existirem (§4) |

`base_price` fica fora: é preço de NPC, não valor de mercado, e expô-lo convidaria a comparar com o
preço do jogador. O portal já lê `content.*` (`RF-POR-008`), então a view não é exigência de permissão —
é o contrato único da ficha: quando as estatísticas de §4 existirem, só ela muda.

## 3. Tabelas novas do mercado

Dez tabelas em `portal` e uma em `game`. Todas nascem com `created_at TIMESTAMPTZ NOT NULL DEFAULT now()`;
onde há mudança, `updated_at` com o trigger `trg_<tabela>_set_updated_at` do padrão.

### 3.1 `portal.market_wallet` — saldo

| Coluna | Tipo | Regra |
|---|---|---|
| `account_id` | `BIGINT` | PK `pk_market_wallet`; **sem FK** para `game.account`, como `portal.donate_credit` (`P-17`) |
| `currency` | `TEXT` | `NOT NULL DEFAULT 'portal_credit'`; `ck_market_wallet_moeda` em `('portal_credit')` |
| `balance` | `BIGINT` | `NOT NULL DEFAULT 0`; `ck_market_wallet_saldo` `>= 0` |

Invariante: `balance = SUM(market_ledger.amount)` da conta. O saldo é cache do ledger, nunca fonte.

### 3.2 `portal.market_ledger` — extrato append-only

| Coluna | Tipo | Regra |
|---|---|---|
| `id` | `BIGINT` identity | PK `pk_market_ledger` |
| `account_id` | `BIGINT` | `NOT NULL`, sem FK |
| `kind` | `TEXT` | `ck_market_ledger_tipo` em `('recarga','compra','venda','taxa','retencao_lance','devolucao_lance','estorno')` |
| `amount` | `BIGINT` | `NOT NULL`, `ck_market_ledger_valor` `<> 0`; sinal indica débito/crédito |
| `balance_after` | `BIGINT` | `NOT NULL`, `>= 0` |
| `ref_kind`, `ref_id` | `TEXT`, `BIGINT` | `ref_kind` em `('anuncio','transacao','disputa','recarga')`; `ck_market_ledger_ref` exige os dois juntos |
| `idempotency_key` | `TEXT` | `NOT NULL`, `uq_market_ledger_idempotency` (`M17`) |

Invariantes: nenhuma linha é `UPDATE` ou `DELETE` (privilégio revogado, como em `M16`); `balance_after`
encadeia com a linha anterior da mesma conta; `taxa` é linha própria e **não** volta a carteira nenhuma
— é sumidouro, e é isso que sustenta a economia.

### 3.3 `portal.market_fee_rule` — taxa vigente

| Coluna | Tipo | Regra |
|---|---|---|
| `id` | `BIGINT` identity | PK `pk_market_fee_rule` |
| `fee_bps` | `INTEGER` | `ck_market_fee_rule_bps` `BETWEEN 0 AND 10000` |
| `effective_from` | `TIMESTAMPTZ` | `NOT NULL`, `uq_market_fee_rule_vigencia` |
| `created_by_person_id` | `BIGINT` | autor de staff, para auditoria |
| `note` | `TEXT` | justificativa da mudança |

Regra: taxa não é configuração genérica (`ops.config_*`, `M21`) — precisa de histórico com vigência, e o
anúncio fotografa o `fee_bps` no momento em que nasce. Duas fontes de verdade para o mesmo número seria
defeito; por isso `ops.config_*` fica fora deste assunto.

### 3.4 `portal.market_listing` — anúncio

| Coluna | Tipo | Regra |
|---|---|---|
| `id` | `BIGINT` identity | PK `pk_market_listing` |
| `seller_account_id` | `BIGINT` | `NOT NULL`, sem FK |
| `character_id` | `BIGINT` | personagem de onde o item saiu |
| `item_index` | `INTEGER` | `NOT NULL` |
| `item_kind_slug` | `TEXT` | cópia de `content.item_catalog.kind_slug` no anúncio, para filtrar por categoria sem `JOIN` |
| `qty`, `refine`, `ancient`, `gem_effect`, `gem_value`, `flags` | — | retrato da instância; `ck_market_listing_qtd` `qty >= 1`; `ck_market_listing_refino` 0–15 |
| `kind` | `TEXT` | `ck_market_listing_tipo` em `('venda','leilao')` |
| `price` | `BIGINT` | `NOT NULL`, `ck_market_listing_preco` `> 0`; no leilão é o lance mínimo |
| `buyout_price` | `BIGINT` | nulo fora do leilão; `ck_market_listing_compra_ja` `NULL OR >= price` |
| `fee_bps` | `INTEGER` | retrato da regra vigente no anúncio |
| `ends_at` | `TIMESTAMPTZ` | `ck_market_listing_prazo`: `(kind = 'leilao') = (ends_at IS NOT NULL)` |
| `status` | `TEXT` | `ck_market_listing_estado` em `('rascunho','ativo','vendido','expirado','cancelado','disputa')` |
| `escrow_ref` | `BIGINT` | id em `game.market_escrow`; sem FK entre schemas |
| `current_bid_id`, `bid_count`, `view_count` | `BIGINT`, `INTEGER`, `INTEGER` | popularidade derivada; `>= 0` |

Invariantes: `status = 'ativo'` exige `escrow_ref IS NOT NULL` (`ck_market_listing_custodia`); `price`,
`item_index` e `character_id` são imutáveis depois de `ativo`; o estado só avança
(`rascunho → ativo → {vendido, expirado, cancelado, disputa}`), nunca retrocede.

### 3.5 `portal.market_bid` — lance

| Coluna | Tipo | Regra |
|---|---|---|
| `id` | `BIGINT` identity | PK `pk_market_bid` |
| `listing_id` | `BIGINT` | `NOT NULL`, `fk_market_bid_listing` → `market_listing` |
| `bidder_account_id` | `BIGINT` | `NOT NULL`, sem FK |
| `amount` | `BIGINT` | `ck_market_bid_valor` `> 0` |
| `status` | `TEXT` | `ck_market_bid_estado` em `('ativo','superado','vencedor','perdido','cancelado')` |
| `hold_ledger_id` | `BIGINT` | linha `retencao_lance` que reservou o crédito |

Invariantes: `amount` acima do maior lance ativo da lista (garantido pela transação que bloqueia a lista,
não por `CHECK`); um lance ativo por comprador por lista (`uq_market_bid_comprador_ativo`, único parcial
`WHERE status = 'ativo'`); só existe em `kind = 'leilao'`; não nasce depois de `ends_at`.

### 3.6 `portal.market_transaction` — transação

| Coluna | Tipo | Regra |
|---|---|---|
| `id` | `BIGINT` identity | PK `pk_market_transaction` |
| `listing_id` | `BIGINT` | `NOT NULL`, `fk_market_transaction_listing`; `uq_market_transaction_listing` (um desfecho por anúncio) |
| `buyer_account_id`, `seller_account_id` | `BIGINT` | `ck_market_transaction_partes` exige diferentes |
| `price_gross`, `fee_amount`, `net_amount` | `BIGINT` | `ck_market_transaction_soma`: `fee_amount + net_amount = price_gross` |
| `fee_bps` | `INTEGER` | retrato do anúncio |
| `escrow_ref` | `BIGINT` | custódia que originou a entrega |
| `settled_at` | `TIMESTAMPTZ` | `NOT NULL` |
| `idempotency_key` | `TEXT` | `uq_market_transaction_idempotency` |

Invariante: dinheiro conservado — débito do comprador = `price_gross`, crédito do vendedor = `net_amount`,
`fee_amount` vira linha `taxa` e não é creditado a ninguém.

### 3.7 `portal.market_review` — avaliação

| Coluna | Tipo | Regra |
|---|---|---|
| `id` | `BIGINT` identity | PK `pk_market_review` |
| `transaction_id` | `BIGINT` | `fk_market_review_transaction`; `uq_market_review_transaction` (uma por transação) |
| `reviewer_account_id`, `seller_account_id` | `BIGINT` | avaliador é o comprador da transação |
| `rating` | `SMALLINT` | `ck_market_review_nota` `BETWEEN 1 AND 5` |
| `comment` | `TEXT` | opcional; `ck_market_review_comentario` até 500 caracteres |
| `state` | `TEXT` | `ck_market_review_estado` em `('visivel','oculto')` — moderação sem apagar prova |

### 3.8 `portal.market_reputation` — agregado do vendedor

| Coluna | Tipo | Regra |
|---|---|---|
| `account_id` | `BIGINT` | PK `pk_market_reputation` |
| `sales_completed`, `sales_disputed`, `disputes_lost` | `INTEGER` | `>= 0` |
| `rating_sum`, `rating_count` | `BIGINT`, `INTEGER` | média calculada na leitura; sem `numeric` fracionário solto |
| `updated_at` | `TIMESTAMPTZ` | recalculado na mesma transação do desfecho |

Invariante: todo número é derivado de `market_transaction`/`market_review`/`market_dispute`; nada é
digitado por staff. Avaliação `oculta` não soma.

### 3.9 `portal.market_notification` — aviso

| Coluna | Tipo | Regra |
|---|---|---|
| `id` | `BIGINT` identity | PK `pk_market_notification` |
| `account_id` | `BIGINT` | destinatário |
| `kind` | `TEXT` | `ck_market_notification_tipo` em `('lance_recebido','lance_superado','arrematado','vendido','expirado','disputa_aberta','disputa_decidida','avaliacao_recebida')` |
| `ref_kind`, `ref_id` | `TEXT`, `BIGINT` | `('anuncio','transacao','disputa')` |
| `read_at` | `TIMESTAMPTZ` | nulo = não lida |

### 3.10 `portal.market_dispute` — disputa

| Coluna | Tipo | Regra |
|---|---|---|
| `id` | `BIGINT` identity | PK `pk_market_dispute` |
| `transaction_id` | `BIGINT` | `fk_market_dispute_transaction` |
| `opened_by_account_id` | `BIGINT` | comprador ou vendedor da transação |
| `reason_code` | `TEXT` | `ck_market_dispute_motivo` em `('nao_recebido','divergente','cobranca_indevida','outro')` |
| `description` | `TEXT` | `NOT NULL`, até 2.000 caracteres |
| `state` | `TEXT` | `ck_market_dispute_estado` em `('aberta','em_analise','resolvida_comprador','resolvida_vendedor','cancelada')` |
| `resolution` | `TEXT` | obrigatória quando resolvida (`ck_market_dispute_desfecho`) |
| `decided_by_person_id` | `BIGINT` | staff que decidiu |

Invariante: uma disputa aberta por transação (`uq_market_dispute_aberta`, único parcial
`WHERE state IN ('aberta','em_analise')`); decidir exige `decided_by_person_id` e gera linha em
`audit_log` (`M21`).

### 3.11 `game.market_escrow` — custódia (escrita pelo executor)

| Coluna | Tipo | Regra |
|---|---|---|
| `id` | `BIGINT` identity | PK `pk_market_escrow` |
| `character_id`, `item_index`, `qty`, `refine`, `ancient`, `gem_effect`, `gem_value`, `flags` | — | retrato do item retirado |
| `state` | `TEXT` | `ck_market_escrow_estado` em `('retido','entregue','devolvido')` |
| `listing_ref`, `transaction_ref` | `BIGINT` | referência ao `portal`; sem FK entre schemas |
| `idempotency_key` | `TEXT` | `uq_market_escrow_idempotency` |

Invariante: a criação da linha e a remoção de `game.character_item (character_id, slot)` acontecem na
**mesma transação**; enquanto `state = 'retido'`, o item não existe em inventário nem em baú. É a única
tabela de `game` que este assunto cria, e quem escreve nela é o executor de §5 — nunca o portal.

## 4. Estatísticas de arma: o que falta e como provar

A missão exige dano, alcance, durabilidade e habilidades especiais. O que existe hoje:

| Dado | Onde está | Situação |
|---|---|---|
| nome, categoria, grau, efeitos `EF_*`, requisitos | `content.item_catalog` (`0014`) | `V` — verificável na página do lote |
| habilidades especiais como símbolo | `effect_names` | `V` — o símbolo existe; o **valor** que ele aplica não está em coluna |
| dano, alcance | — | **não existe coluna** em `content.item_catalog` nem em `game.character_item` |
| durabilidade | — | não existe coluna; se existir no cliente, é estado de instância e morre com a sessão |

Caminho de verificação, obrigatório antes de fechar a fatia da armaria:

1. Trace no `wyd-server` da decodificação do item: qual campo do struct vira dano, qual vira alcance, se
   há campo de durabilidade e onde ele é persistido (ou se só vive em memória do cliente).
2. Confrontar com o vetor `EF_*` do catálogo: quais símbolos carregam valor numérico e quais são só
   marcador.
3. Só então decidir a forma: coluna em `content.item_catalog`, tabela `content.item_effect_value`
   1:N por item, ou view que calcula a partir do efeito.
4. Registrar o resultado na página `migrations/00NN` do lote e fechar `LAC-POR-08`.

Enquanto o trace não existir, `content.v_item_sheet` entrega `stat_source = 'effect_names'` e a ficha
exibe **o efeito pelo nome**, declarando que o número não foi medido. Nunca zero, nunca estimativa,
nunca remoção silenciosa do requisito.

## 5. Contrato exigido de fora do portal

O mercado não funciona só com DDL em `portal`: mover item exige alguém com escrita em `game`.

A forma das quatro operações (`custodiar`, `entregar`, `devolver`, `cancelar`) é do `wyd-admin`:
`docs/arquitetura/contrato-c4.md` §4.9–§4.13, com payload, erros e efeito por transação em
`docs/specs/acoes.md` §8. Aqui fica só o que a **fronteira** exige dele, e que a forma não pode perder:

1. **Resposta síncrona** — o anúncio só fica `ativo` com custódia confirmada.
2. **Idempotência por chave** (`M17`) — repetir não move o item duas vezes.
3. **Auditoria dos dois lados** (`ops.audit_log`), com a conta do jogador como autora.
4. **Atomicidade** — o item sai de `character_item` e nasce em `market_escrow` na mesma transação.
5. **Nada se perde** — mochila cheia recusa a entrega com o item ainda `retido`.

`ops.delivery_queue` (`0012`) **não serve**: ela concede item novo a uma conta, de forma assíncrona, e
não transfere instância existente de um personagem para outro. Usá-la deixaria o anúncio ativo sem
garantia de que o item saiu do inventário — o caminho foi rejeitado, não adiado.

O executor está definido desde 2026-11-22 (`LAC-POR-07` fechada) e o schema também: `game.market_escrow` e as
dez `portal.market_*` entraram no lote `0034` (`M31`, `ddl-confirmado.md` §5.34). O `0034` **emenda o
`0032`**: a custódia exige `SELECT, INSERT, DELETE` em `game.character_item` e `SELECT` em
`game.character` para `wyd_admin`, que a primeira versão dos `GRANT` não concedia.

## 6. Índices que sustentam busca, filtro e escala

| Índice | Colunas | Consulta que atende |
|---|---|---|
| `ix_market_listing_item_status_preco` | `(item_index, status, price)` | busca por item e faixa de preço |
| `ix_market_listing_kind_status_preco` | `(item_kind_slug, status, price)` | filtro por categoria com faixa de preço |
| `ix_market_listing_status_popularidade` | `(status, bid_count DESC, view_count DESC)` | ordenação por popularidade |
| `ix_market_listing_status_prazo` | `(status, ends_at)` parcial `WHERE status = 'ativo'` | leilões a fechar e vitrine |
| `ix_market_bid_listing_criado` | `(listing_id, created_at DESC)` | histórico de lances |
| `ix_market_transaction_comprador` / `_vendedor` | `(buyer_account_id, settled_at DESC)` / `(seller_account_id, settled_at DESC)` | “minhas compras” e “minhas vendas” |
| `ix_market_ledger_conta_criado` | `(account_id, created_at DESC)` | extrato da carteira |
| `ix_market_notification_conta_nao_lida` | `(account_id)` parcial `WHERE read_at IS NULL` | badge de avisos |
| `ix_market_dispute_estado_aberta` | `(state, created_at)` parcial `WHERE state IN ('aberta','em_analise')` | fila de staff |
| `ix_market_escrow_estado` (`game`) | `(state, created_at)` | conferência de custódia pendente |
| `ux_market_bid_comprador_ativo` | `(listing_id, bidder_account_id)` único parcial `WHERE status = 'ativo'` | um lance ativo por comprador e anúncio |
| `ux_market_dispute_aberta` | `(transaction_id)` único parcial `WHERE state IN ('aberta','em_analise')` | uma disputa viva por transação |
| `ix_market_bid_ledger`, `ix_market_dispute_transaction`, `ix_market_escrow_character` | coluna da FK | integridade das três chaves estrangeiras (`P-15`) |

Sem partição e sem tabela agregada de vitrine na `1.0.0`: paginação por cursor (`price, id` ou
`ends_at, id`), não por `OFFSET`. Se a medição mostrar que não basta, a partição entra como decisão
própria, com número na mão.

## 7. O que este documento não faz

- Não é o dono do número: o lote `0034` foi reservado em `plano-migrations.md` §3 (`M31`) e a série está
  contada lá e em `migrations-checklist.md` §2.
- Não escreve `DDL` executável: colunas, chaves e invariantes aqui são forma, e `ddl-confirmado.md` §5.34 é
  o dono do texto final.
- Não decide execução de permissão: `USAGE` cruzado continua proibido (`P-17`) e o adapter é o caminho.
- Não fecha `LAC-POR-08` (estatísticas), que precisa do trace. O executor saiu daqui em 2026-11-22:
  `LAC-POR-07` foi fechada com o contrato do `wyd-admin` e o schema saiu em `M31` (`LAC-ADM-07` fechada).
