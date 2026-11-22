> Pergunta: "como a armaria e o mercado global entram na `1.0.0` do portal, com que decisão, banco, contrato e ordem?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: produto (escopo) com arquitetura (fronteira)
> Base: `docs/specs/requisitos.md` §4; `docs/roadmap/roadmap.md` F7–F11; `docs/arquitetura/{fronteira,consumo-do-banco,rotas-e-permissoes}.md`; `lacunas.md`; `wyd-db/armaria-e-mercado.md`; `M16`, `M17`, `M21`, `M25`
> Revisado em: 2026-11-22

**Estado:** decidido (2026-11-22) — D1–D10 fixadas pelo texto da missão de produto; pacote de
planejamento fechado, implementação não iniciada.

As decisões abaixo não foram escolhidas por preferência: cada uma tem a frase da missão que a obriga e
o registro da alternativa recusada. Não resta decisão de produto em aberto. O executor da transferência de
item foi definido em 2026-11-22 (§8, `LAC-POR-07` fechada); o que ainda depende de outro dono é o lote de
schema (`LAC-ADM-07`) e a credencial portal→admin (`LAC-ADM-04`).

## 1. O pedido normalizado

| # | Item da missão | Requisito | Artefato de banco | Fatia |
|---:|---|---|---|---|
| A1 | perfil com armas e equipamentos equipados | `RF-POR-019`, `RF-POR-020` | `game.v_character_equipment` | F7 |
| A2 | perfil público para outros jogadores | `RF-POR-019` | `game.v_character_armory` | F7 |
| A3 | dados reais, extraídos das fichas | `RF-POR-019` | as três views de `game` | F7 |
| A4 | armaria somente leitura, sem edição | `RF-POR-019` | views `SELECT` puro | F7 |
| A5 | dano, alcance, durabilidade, habilidades especiais | `RF-POR-021` | `content.v_item_sheet` + §4 | F7 |
| A6 | filtro e ordenação de equipamento | `RF-POR-022` | PK `character_item (character_id, slot)` | F7 |
| A7 | experiência no estilo do Armory do WoW | direção de produto | — | F7 |
| M1 | listar itens com preço do jogador | `RF-POR-023`, `RF-POR-024` | `portal.market_listing` | F8 |
| M2 | filtro por categoria, preço e popularidade | `RF-POR-024` | índices `ix_market_listing_*` | F8 |
| M3 | leilão com prazo e lances | `RF-POR-025` | `market_listing`, `market_bid` | F9 |
| M4 | transações seguras e registradas | `RF-POR-027` | `market_transaction`, `market_ledger` | F8 |
| M5 | reputação do vendedor | `RF-POR-029` | `market_review`, `market_reputation` | F10 |
| M6 | notificações de oferta, lance e venda | `RF-POR-030` | `market_notification` | F9, F10 |
| M7 | acesso a todos os jogadores | `RF-POR-031` | rotas sem gate | F8 |
| M8 | taxa e comissão | `RF-POR-028` | `market_fee_rule` | F8 |
| M9 | consistência e atualização em tempo real | `RNF-POR-008` | view + janela declarada | F7, F8 |
| M10 | antifraude | `RF-POR-033` | ledger append-only + `audit_log` | F11 |
| M11 | disputas e suporte | `RF-POR-032` | `market_dispute` | F11 |
| M12 | escala com muitas transações | `RNF-POR-009` | índices de §6 e cursor | F8, F9 |
| M13 | itens reais dos personagens | `RF-POR-023`, `RF-POR-027` | `game.market_escrow` | F8 |
| M14 | moeda da conta no portal, não o ouro | `RF-POR-026` | `market_wallet`, `market_ledger` | F8 |

`A7` não vira requisito verificável: "familiar e intuitiva" não tem critério binário. O que a missão
comprova é comportamento — perfil público, slots, ficha do item, filtro — e é isso que `RF-POR-019` a
`RF-POR-022` exigem.

## 2. O que já existe e serve (V, 2026-11-22)

| Peça | Estado | Serve para |
|---|---|---|
| `game.character` (`0006`) | nome, classe 0–5, nível, tier, `content_version`, soft-delete; guilda em `game.guild_member` | cabeçalho do perfil |
| `game.character_item` (`0009`) | PK `(character_id, slot)`, `slot` 0–15 equip / 16–79 mochila, `qty`, `refine` 0–15, `ancient`, `gem_effect`, `gem_value`, `flags` | equipamento e custódia |
| `game.character_learned_skill` (`0008`) | máscara por categoria | habilidades do perfil |
| `content.item_catalog` (`0014`) | nome, categoria, `effect_names` (`EF_*`), `grade`, `base_price`, requisitos | ficha do item |
| `portal.donate_credit` (`0011`) | ledger de crédito com `idempotency_key` única (`M17`), `account_id` sem FK (`P-17`) | origem do saldo de mercado |
| `game.account_wallet` / `account_ledger` (`0005`) | ouro do jogo (`M16`) | **fora**: moeda do mercado é a do portal |
| `ops.delivery_queue` (`0012`) | fila web→jogo, idempotente, dreno `SKIP LOCKED` (`M25`) | entrega de concessão; **não** transfere instância |
| `ops.audit_log` / `ops.config_*` (`M21`) | auditoria e configuração versionada | decisão de staff e trilha |

## 3. O que falta (V) e como fica

| Falta | Medição | Como o pacote resolve |
|---|---|---|
| menção a armaria/mercado no planejamento | `grep -rni "armaria\|mercado\|leilão"` em `planejamento-1.0.0`: 0 antes de 2026-11-22 | requisitos §4, fatias F7–F11 e este documento |
| rota pública de personagem | 43 rotas até 2026-11-22, nenhuma pública de personagem | `RF-POR-019`; rotas 44–50 acrescentadas em `rotas-e-permissoes.md` (total 50) |
| view de personagem, item e inventário | `0029` cria só `game.v_live_session` e `portal.v_person_identity` | 4 views em `wyd-db/armaria-e-mercado.md` §2 |
| estatística de arma | sem coluna de dano, alcance ou durabilidade | `RF-POR-021` + caminho de verificação (§4 da página de banco); `LAC-POR-08` |
| carteira do portal | só `donate_credit`/`donate_debit` | `market_wallet` + `market_ledger` (§3.1–3.2 da página de banco) |
| anúncio, lance, transação, avaliação, aviso, disputa, taxa | nenhuma tabela | 10 tabelas em `portal` + `game.market_escrow` |
| executor da transferência de item | contrato definido em 2026-11-22 | quatro rotas no C4 do `wyd-admin` (`contrato-c4.md` §4.9–§4.13); `LAC-POR-07` fechada |
| fechamento de leilão | nenhum job planejado | `LAC-POR-09`; job idempotente na F9 |
| rota e tela administrativa do mercado | `/admin/*` sem mercado | F11, com MFA e auditoria |

## 4. Fronteira aplicada

`LAC-POR-01` e `P-17` continuam valendo: o portal **não escreve** em `game`. `0029` §10 mediu
`SET ROLE wyd_portal; SELECT` em `game.v_live_session` devolvendo `42501`, então nem a leitura é conexão
direta do BFF: a leitura é do adapter do `wyd-db` sob papel autorizado, e as views dão forma a ela.

Consequência para a armaria: nenhuma. Perfil público é `SELECT` por view, e a fatia não muda permissão
nem cria escrita.

Consequência para o mercado: tudo que é decisão, dinheiro e estado vive em `portal`; o item, que só existe
em `game.character_item`, é movido pelo executor autorizado. O portal manda `custodiar`, `entregar`,
`devolver` ou `cancelar` e guarda a referência; nunca emite `UPDATE` em `game`.

## 5. Decisões fechadas

### D1 — Estatísticas de arma (`RF-POR-021`)

**Decidida pela missão:** "estatísticas detalhadas das armas e equipamentos, incluindo dano, alcance,
durabilidade e habilidades especiais" e "os dados devem ser reais".

**Decisão:** os quatro campos entram na ficha. A origem é o trace da decodificação do item no
`wyd-server` cruzada com o vetor `EF_*` de `content.item_catalog`; a publicação exige o trace feito, e o
resultado define a forma (coluna no catálogo, tabela de efeito 1:N ou view calculada). Habilidades
especiais são exibidas pelo símbolo desde já, porque o símbolo existe (`V`).

**Fallback honesto:** se o trace provar que o número não existe em lugar nenhum, a lacuna `LAC-POR-08`
fecha com a decisão registrada e a ficha declara `stat_source = 'effect_names'`, mostrando só o efeito
provado. Nunca zero, nunca estimativa, nunca remoção do requisito.

**Recusadas:** publicar a armaria sem os campos (a missão os exige); estimar a partir de `grade` ou
`base_price` (número inventado); adiar a armaria inteira até o trace (a missão pede a funcionalidade na
`1.0.0`, e o fallback mantém a fatia de pé).

### D2 — Moeda do mercado (`RF-POR-026`)

**Decidida pela missão:** "a moeda do mercado deve ser a moeda da conta no portal, não deve ser o gold do
jogo".

**Decisão:** carteira própria em `portal` — `market_wallet` (saldo, moeda `portal_credit`) e
`market_ledger` (append-only, idempotente). Recarga nasce de `portal.donate_credit`; débito e crédito
acontecem só por linha do ledger; a taxa é linha própria e não volta a carteira nenhuma.

**Recusadas:** ouro do jogo (`game.account_wallet`), proibido pela missão; reusar `donate_credit` como
saldo, que misturaria compra de crédito com saldo de mercado e perderia a trilha por transação; saldo em
`ops.config_*`, que não é dinheiro.

### D3 — Custódia do item (`RF-POR-023`, `RF-POR-027`)

**Decidida pela missão:** "transações seguras", "itens reais dos personagens", "integridade".

**Decisão:** custódia transacional. `game.market_escrow` recebe a instância e `game.character_item` perde
a linha **na mesma transação**, pelo executor; o anúncio só fica `ativo` com `escrow_ref`. O caminho de
ida e volta é o mesmo contrato, idempotente.

**Recusadas:** `ops.delivery_queue` — é assíncrona e **concede** item a uma conta, não transfere instância
existente; usá-la deixaria anúncio ativo sem garantia de que o item saiu do inventário. Metadados só no
portal com ação posterior: cria janela em que o mesmo item está anunciado e equipado. Restringir a itens
do baú não resolve, porque o baú também é inventário do jogador.

### D4 — Leilão (`RF-POR-025`)

**Decidida pela missão:** "sistema de leilão, onde os jogadores podem dar lances em itens, com um tempo
limite para o término do leilão".

**Decisão:** leilão está na `1.0.0`, na fatia F9. Prazo em `ends_at`; lance reserva o crédito
(`retencao_lance`) e é devolvido quando superado; fechamento por job idempotente com
`uq_market_transaction_listing`; leilão sem lance expira e devolve a custódia.

**Recusadas:** só venda direta na `1.0.0` (contraria a missão); fechar "no acesso", sem job (não escala e
não notifica ninguém); lance sem reserva (arremate pode falhar por saldo e a transação vira promessa).

### D5 — Reputação (`RF-POR-029`)

**Decidida pela missão:** "os compradores avaliem a confiabilidade dos vendedores com base em suas
experiências de compra".

**Decisão:** avaliação por transação concluída — nota 1–5, comentário opcional, uma por transação, só
pelo comprador — com agregado derivado (`sales_completed`, `rating_sum`/`rating_count`, disputas) e
estado `visivel`/`oculto` para moderação sem apagar prova.

**Recusadas:** só contadores automáticos, sem ato de avaliar (a missão pede que o comprador avalie); texto
livre sem moderação, que abre abuso e retaliação.

### D6 — Notificação (`RF-POR-030`)

**Decidida pela missão:** "informar os jogadores sobre novas ofertas, lances e vendas concluídas".

**Decisão:** aviso in-app em `market_notification`, com contador de não lidos e marcação de leitura;
consulta com janela declarada. Eventos cobertos: lance recebido, lance superado, arremate, venda,
expiração, disputa e avaliação.

**Recusadas:** e-mail transacional, que exigiria provedor externo não aprovado; mensagem dentro do jogo,
que depende do executor e não cobre lance no portal.

### D7 — Taxa (`RF-POR-028`)

**Decidida pela missão:** "taxas e comissões ... garantindo que o sistema seja sustentável e
financeiramente viável".

**Decisão:** percentual em bps com vigência (`market_fee_rule`), fotografado no anúncio, cobrado no
fechamento e registrado como linha `taxa` — sumidouro, nunca creditada a alguém.

**Recusadas:** taxa fixa por anúncio, que penaliza item barato e empurra o mercado para fora; ausência de
taxa, que contraria a missão; creditar a taxa a uma conta, que reintroduz o dinheiro e inflaciona.

### D8 — Fraude e disputa (`RF-POR-032`, `RF-POR-033`)

**Decidida pela missão:** "prevenir fraudes e atividades suspeitas" e "suporte ao cliente para resolver
disputas".

**Decisão:** disputa registrada em `market_dispute`, uma aberta por transação, aberta só pelas partes,
decidida por staff com segundo fator e `audit_log`; limites por conta (anúncios ativos, lances por
minuto), recusa de compra da própria conta e de ciclo entre as mesmas contas; ledger append-only como
trilha.

**Recusadas:** suporte só por e-mail (a missão pede canal no produto); banimento automático por heurística,
sem dado que sustente o limiar.

### D9 — Tempo real (`RNF-POR-008`)

**Decidida pela missão:** "informações ... consistentes e atualizadas em tempo real" e "sem problemas de
desempenho".

**Decisão:** leitura com janela declarada e cache curto (≤ 60 s), a mesma escolha do ranking
(`LAC-POR-04`), mais fechamento por job. "Tempo real" é cumprido como consistência declarada, não como
promessa de socket.

**Recusadas:** WebSocket/SSE no portal, infraestrutura nova sem requisito de sub-segundo; realtime do
jogo, que não alcança a página do portal.

### D10 — Privacidade do perfil (`RF-POR-019`)

**Decidida pela missão:** o perfil é público para outros jogadores.

**Decisão:** público com o que já é público hoje (nome, classe, nível, guilda, equipamento, habilidades) e
nada além: sem `account_id`, e-mail, IP, sessão ou ouro; personagem com `deleted_at` devolve `404`.

**Recusadas:** opt-in por personagem (contraria "tornar o perfil público"); expor a ficha inteira da conta
(vaza identidade e permite correlação).

## 6. Fatiamento

O dono da ordem é `docs/roadmap/roadmap.md`; aqui fica só o recorte e o motivo.

| Fatia | Entrega | Prova mínima |
|---|---|---|
| F7 | armaria read-only: `/personagem/[id]`, 16 slots, habilidades, ficha do item, filtro | responde sem sessão; sem PII; estatística com origem declarada |
| F8 | mercado de venda direta: carteira, anúncio com custódia, compra, taxa, extrato | item sai do inventário antes do anúncio ativo; `taxa + líquido = bruto` |
| F9 | leilão: prazo, lances com reserva, fechamento idempotente | fechar duas vezes não cobra duas vezes; perdedor recebe a reserva |
| F10 | reputação e notificação | uma avaliação por transação; um aviso por evento |
| F11 | disputa, antifraude e fechamento | staff decide com MFA e auditoria; limite e ciclo recusados |

Todas dentro da `1.0.0`. O contrato do executor está fechado (`LAC-POR-07`), então o que segura a F8 é o
lote de `game.market_escrow` (`LAC-ADM-07`); F9 não começa sem F8; F10 e F11 não começam sem transação
concluída.

## 7. Requisitos

Numeração e critério de aceitação vivem em `docs/specs/requisitos.md` §4, e não se repetem aqui:
`RF-POR-019` perfil público · `020` equipamento e habilidades · `021` ficha com estatísticas · `022`
filtro e ordenação · `023` anúncio com custódia · `024` busca do mercado · `025` leilão · `026` carteira e
extrato · `027` transação registrada · `028` taxa · `029` reputação · `030` notificação · `031` acesso
irrestrito · `032` disputa · `033` antifraude · `RNF-POR-008` janela declarada · `009` escala · `010`
escrita segura.

## 8. Contrato exigido de fora do portal

Quem executa a transferência de item é o `wyd-admin`, e o contrato existe desde 2026-11-22:
`POST /characters/{character_id}/escrow` (custodiar), `POST /escrows/{id}/deliver` (entregar),
`POST /escrows/{id}/return` (devolver) e `POST /escrows/{id}/cancel` (cancelar), em
`wyd-admin/docs/arquitetura/contrato-c4.md` §4.9–§4.13, com efeito por tabela e transação em
`acoes.md` §8 do mesmo sistema. Cada chamada leva `Idempotency-Key` e `X-Actor-Account-Id` (a conta do
jogador, nunca staff); a resposta é síncrona e o erro é tipado — em particular `409 ADM_INVENTORY_FULL`
deixa o item `retido`, nunca descartado.

O que ainda falta não é executor, e sim schema: `game.market_escrow` e `portal.market_*` precisam de lote
aprovado no `wyd-db` (`LAC-ADM-07`).

Por que síncrono: o anúncio não pode ficar `ativo` antes de o item sair do inventário; uma fila
assíncrona devolveria "aceito" sem garantia de custódia, que é exatamente o furo que a missão proíbe.

## 9. O que cada sistema precisa

| Sistema | Precisa | Estado |
|---|---|---|
| `wyd-db` | lote com quatro views e onze tabelas (`armaria-e-mercado.md`), texto em `ddl-confirmado.md` | forma proposta; número de lote não reservado |
| `wyd-admin` | contrato de custódia e entrega de item | **proposto em 2026-11-22**: C4 §4.9–§4.13, `acoes.md` §8, `ADM-10`; falta o lote de schema |
| `wyd-server` | aplicar custódia/entrega na transação; expor o dado de §5 | a planejar com o trace de `LAC-POR-08` |
| `wyd-ops` | job de fechamento e expiração, métricas do mercado | dono em `LAC-POR-09` |
| `wyd-docs` | publicar as páginas novas e realinhar contagens | quando o lote virar página |

## 10. Lacunas

| ID | Assunto | Fecha quando |
|---|---|---|
| `LAC-POR-07` | executor da transferência de item | **fechada em 2026-11-22**: contrato no C4 do `wyd-admin` (§4.9–§4.13) e fatia `ADM-10` |
| `LAC-POR-08` | origem de dano, alcance e durabilidade | trace do struct de item concluído e forma decidida |
| `LAC-POR-09` | dono do fechamento de leilão | decidido se o job mora no BFF ou no `wyd-ops` |

Registradas em `lacunas.md` em 2026-11-22; `LAC-POR-07` foi fechada no mesmo dia, com o contrato escrito.
As duas restantes não são decisão de produto em aberto: são dependências com dono e critério de
fechamento.

## 11. Riscos

- **Item duplicado** entre inventário e anúncio é o risco que mata o mercado: a custódia é uma transação,
  e o anúncio não fica ativo sem `escrow_ref`.
- **Schema de custódia atrasado** segura a F8: o executor está definido, falta o lote de
  `game.market_escrow`; `delivery_queue` não é plano B.
- **Leilão sem comprador** prende item: expiração devolve a custódia no mesmo job.
- **Inflação**: a taxa precisa ser sumidouro, e o saldo de mercado nunca nasce de lançamento manual.
- **PII no perfil**: nome real, e-mail e sessão nunca entram na rota pública.
- **Escala**: filtro sem índice vira varredura; `p95` é medido, não prometido.
- **Estatística inventada** destrói a confiança no perfil mais rápido do que a falta dela: por isso o
  fallback declara o que não foi medido.

## 12. O que este documento não decide

| Assunto | Dono |
|---|---|
| Texto final de DDL e número do lote | `wyd-db` (`ddl-confirmado.md`) |
| Contrato HTTP do `wyd-admin` | arquitetura + `wyd-admin` |
| Valor da taxa e política econômica | produto |
| Biblioteca de UI e forma visual | fatia, com registro se mudar fronteira |
| Detecção estatística de fraude | fase posterior, com dado medido |
