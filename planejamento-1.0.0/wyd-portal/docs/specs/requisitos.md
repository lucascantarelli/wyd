> Pergunta: "quais requisitos o portal precisa cumprir na 1.0.0 e como saber que cada um está atendido?"
> Público: Desenvolvedor novo, produto e QA
> Dono: produto
> Base: `M21`; `wyd-portal-prompt.md` §5/§6; `docs/arquitetura/rotas-e-permissoes.md`; `docs/arquitetura/consumo-do-banco.md`; `docs/arquitetura/monitoramento.md`
> Revisado em: 2026-11-22

**Estado:** aceito (2026-11-20); §4 (armaria e mercado) acrescentada em 2026-11-22 pela missão de produto;
RF-POR-021 e RF-POR-025 alinhados às decisões de 2026-11-22

Requisito sem critério de aceitação é intenção. Cada bloco tem entrada, saída, critério verificável e
o contrato que usa. Numeração `RF-POR-nnn` (funcional) e `RNF-POR-nnn` (não funcional); não se
renumera o que já existe.

## 1. Identidade e acesso

### RF-POR-001 — Cadastro de pessoa com ativação pendente
- **Entrada:** e-mail, senha, aceite de termos e privacidade.
- **Saída:** linha em `portal.person` com `status = 'pending'` e consentimentos em `person_consent`.
- **Aceitação:** cadastro com e-mail repetido devolve `23505` de `ux_person_email` tratado como
  “e-mail já cadastrado”; senha nunca aparece em log; `email_verified_at` só é preenchido pelo fluxo
  de token.
- **Contrato:** C2 (`0003`, `0004`).

### RF-POR-002 — Login, sessão e logout
- **Entrada:** e-mail e senha; cookie de sessão.
- **Saída:** `person_session` viva; `last_seen_at` renovado em uso.
- **Aceitação:** segunda sessão não invalida a primeira; logout grava `revoked_at`; sessão expirada
  não é renovada (`ck_person_session_sinal`).
- **Contrato:** C2.

### RF-POR-003 — Recuperação de senha e verificação de e-mail
- **Entrada:** pedido por e-mail → token de cerimônia; link com token.
- **Saída:** `person_token` consumido uma vez; `person.password_changed_at` atualizado; sessões
  antigas revogadas.
- **Aceitação:** reusar token consumido falha; dois cliques simultâneos trocam a senha uma vez;
  token de outro `purpose` é recusado.
- **Contrato:** C2 (`0004`).

### RF-POR-004 — Conta de jogo ligada à pessoa
- **Entrada:** pessoa autenticada cria conta com login e senha de jogo.
- **Saída:** `game.account` com `slot` 1–5 e vínculo `person_id`.
- **Aceitação:** sexta conta falha em `ux_account_slot` (`23505`); login repetido falha em
  `ux_account_login`; conta fechada devolve a vaga.
- **Contrato:** C2 + leitura de `0005`.

## 2. Portal público

### RF-POR-005 — Home com estado do servidor
- **Entrada:** sessão viva e topologia.
- **Saída:** contagem de online, capacidade por canal e destaques.
- **Aceitação:** com `game.session` vazia a home mostra `0`, não erro; número tem horário de medição.
- **Contrato:** C1 (view de `0029`, `0007`).

### RF-POR-006 — Notícias e guias editoriais
- **Entrada:** conteúdo publicado pelo CMS.
- **Saída:** lista e página de notícia/guia com `slug` estável.
- **Aceitação:** rascunho não aparece no público; `slug` duplicado é recusado.
- **Contrato:** C5 — `portal.news`/`banner`/`guide`, DDL no lote `0030`, aprovado em 2026-11-20 (`ddl-confirmado.md` §5.15).

### RF-POR-007 — Rankings
- **Entrada:** personagens vivos, guildas e PvP.
- **Saída:** três listas com posição, nome, nível/evolução e valor.
- **Aceitação:** personagem com `deleted_at` não aparece; guildas sem membro não aparecem; consulta
  às tabelas de origem `game` tem `LIMIT`; o BFF mantém o resultado em cache por 60 segundos e informa
  o instante de leitura/atualização e o limite aplicado.
- **Contrato:** fonte `game.character`/`game.guild`, sem tabela nem view materializada nova; decisão
  de cache em `LAC-POR-04`. Respeitar `P-17`: o BFF não recebe `USAGE` SQL em `game`; a forma
  autorizada de executar a consulta é contrato de fronteira, não permissão implícita.

### RF-POR-008 — WYDpedia: itens, skills e mobs
- **Entrada:** `item_index`, código de skill ou id de NPC.
- **Saída:** ficha com atributos, requisitos, drops e links cruzados.
- **Aceitação:** item inexistente devolve 404, não página vazia; busca por nome ignora caixa
  (`name_fold`); toda ficha linka para pelo menos uma entidade relacionada.
- **Contrato:** C5 (`0013`–`0019`).

### RF-POR-009 — Atlas de mapas
- **Entrada:** mapa e filtros de camada (NPC, portal, spot, boss).
- **Saída:** mapa com pins e coordenada do cursor.
- **Aceitação:** mapa sem pins mostra estado vazio, não erro; coordenada exibida bate com a do
  `content_source`.
- **Contrato:** C5 (`0024`).

## 3. Área administrativa

### RF-POR-010 — NOC e canais
- **Entrada:** sessões vivas, topologia e capacidade; CPU, RAM e latência por canal vindas da API do
  Prometheus do `wyd-ops`.
- **Saída:** KPIs (online, pico, contas novas, ouro em circulação) e estado por canal, com ocupação do
  banco e telemetria da fonte externa.
- **Aceitação:** todo card agregado leva a uma tela de detalhe com filtro aplicado; alterar
  `max_players`/`maintenance` grava revisão de canal e linha em `audit_log`; cada card de telemetria
  mostra o instante da amostra e vira **“não medido”** quando a consulta falha ou a série não existe;
  nenhum card de CPU/RAM/latência é servido por tabela de `portal`/`ops`.
- **Contrato:** C1 + C3 + C4 (`0005`–`0007`) para estado e configuração; telemetria por API HTTP do
  `wyd-ops`, sem tabela no `wyd-db` (`LAC-POR-06`, decidida e reconfirmada em 2026-11-20). O detalhe de
  cada fonte, com paridade às telas do legado, está em `monitoramento.md` e nas rotas 38–43.

### RF-POR-011 — Jogadores, contas e moderação
- **Entrada:** busca por nome, login ou e-mail.
- **Saída:** ficha com inventário, baú, skills, guilda e histórico; bloqueio/desbloqueio de conta.
- **Aceitação:** `password_hash`/`pin_hash`/`token_hash` nunca saem na resposta; bloqueio sem motivo
  é recusado (`ck_account_moderation_motivo`); toda ação aparece em `audit_log` com autor.
- **Contrato:** C1 + C3 + C4 (`0005`, `0006`, `0008`–`0010`); bloqueio de conta é escrita em `game.account_moderation` pelo `wyd-admin` (C4 §4.1), não pelo portal; kick também é `wyd-admin`.

### RF-POR-012 — Configuração do servidor com revisão
- **Entrada:** chave, valor novo e justificativa.
- **Saída:** `ops.config_revision` + `ops.config_change` na mesma transação.
- **Aceitação:** revisão sem mudança é recusada; valor anterior e novo ficam registrados; autor
  (`actor_kind`, `actor_id`) confere com a sessão; duas revisões concorrentes não perdem mudança.
- **Contrato:** C3 (`0002` §5.9).

### RF-POR-013 — Segundo fator de staff (TOTP)
- **Entrada:** pessoa com `role IN ('admin','moderator')` e senha de portal válida; código TOTP de 6
  dígitos do autenticador confirmado.
- **Saída:** sessão de portal só emitida no segundo passo; uso do passo registrado em
  `portal.person_authenticator.last_used_step`.
- **Aceitação:** `admin`/`moderator` sem autenticador confirmado não recebe sessão; código de passo
  já usado é recusado; `member` não é afetado (MFA fora da `1.0.0` para ele); recuperação do fator é
  ato de staff, nunca autoatendimento.
- **Contrato:** C2 estendido — `portal.person_authenticator`, com DDL no lote `0031`
  (`ddl-confirmado.md` §5.20, aprovado em 2026-11-20). `LAC-POR-05` fechada quanto ao mecanismo e ao dono.

### RF-POR-014 — Monitoramento: resumo e motor por canal
- **Entrada:** consulta ao Prometheus (`wyd_*`) e presença viva no banco.
- **Saída:** estado das quatro fontes, ticks/s, P99 do tick e orçamento, entidades, jogadores e mobs
  por canal, alertas em disparo e volume de log em 15 min.
- **Aceitação:** fonte fora deixa **só** os cartões dela em “não medido”, sem zerar os demais; cada
  cartão agregado leva à subpágina com o filtro aplicado; nenhum número de CPU/RAM/latência vem de
  tabela de `portal`/`ops`.
- **Contrato:** `monitoramento.md` §1/§2/§8; rotas 38 e 39; `LAC-POR-06`.

### RF-POR-015 — Alvos, coletor e alertas
- **Entrada:** `up` e a lista de alertas do Prometheus.
- **Saída:** alvos `up`/`down` por job, saúde do coletor e alertas com severidade, resumo e instante.
- **Aceitação:** coletor fora mostra “coletor fora” com o motivo, nunca “nenhum alerta”; o portal não
  cria, edita nem silencia regra; alerta exibe o instante de ativação.
- **Contrato:** `monitoramento.md` §2/§5; rota 40.

### RF-POR-016 — Painéis do Grafana
- **Entrada:** catálogo de dashboards provisionados.
- **Saída:** saúde/versão do Grafana, lista de dashboards e o painel escolhido embutido.
- **Aceitação:** Grafana fora mostra o motivo e mantém o link direto; catálogo vazio tem estado vazio,
  não erro; o `iframe` usa só a URL pública, nunca o endereço interno.
- **Contrato:** `monitoramento.md` §4/§8; rota 41.

### RF-POR-017 — Logs por serviço e exportação
- **Entrada:** serviço, nível e texto opcional de até 64 caracteres, todos em allowlist.
- **Saída:** linhas, erros e avisos por serviço em 15 min; download NDJSON da última hora.
- **Aceitação:** `limit` fora de 1–1.000 devolve `422`; metacaractere de LogQL devolve `422`; `member`
  recebe `403`; a resposta de exportação não tem cache.
- **Contrato:** `monitoramento.md` §5; rota 42 e `/api/logs/export`.

### RF-POR-018 — Telemetria anônima do launcher
- **Entrada:** evento do launcher (`check`, `config`, `patch`, `launch`) em contrato fechado.
- **Saída:** agregado por versão e operação — eventos, falhas, taxa de falha, duração média e máxima,
  último evento, chaves e descartes.
- **Aceitação:** campo extra, login ou IP devolve `400`; corpo acima do limite devolve `413`; sucesso
  devolve `204` sem corpo; a tela declara a janela “desde o boot” e o log estruturado como série
  histórica; nada identifica jogador.
- **Contrato:** `monitoramento.md` §3; rota 43 e `POST /api/launcher/telemetria`.

## 4. Armaria e mercado global

Requisitos desta seção nascem da missão de produto de 2026-11-22 e são verificáveis na implementação. As
decisões que os sustentam e as alternativas recusadas estão em `armaria-e-mercado.md`; a forma de
banco, em `wyd-db/armaria-e-mercado.md`.

### RF-POR-019 — Perfil público de personagem (armaria)
- **Entrada:** identificador do personagem numa rota pública, sem sessão.
- **Saída:** cabeçalho (nome, classe, nível, tier, guilda), equipamento, habilidades e ficha de cada item.
- **Aceitação:** responde sem sessão; personagem com `deleted_at` devolve `404`; a resposta não contém
  `account_id`, e-mail, IP, sessão nem ouro; a tela declara o instante da leitura e a janela de cache.
- **Contrato:** `game.v_character_armory`, `game.v_character_equipment`, `game.v_character_skill` e
  `content.v_item_sheet` (`wyd-db/armaria-e-mercado.md` §2), lidas pelo adapter do `wyd-db` (`P-17`).

### RF-POR-020 — Equipamento e habilidades no perfil
- **Entrada:** personagem da armaria.
- **Saída:** os 16 slots de equipamento (0–15), com slot vazio declarado, e as habilidades por categoria.
- **Aceitação:** slot vazio mostra “vazio”, não erro; `item_index` fora do catálogo mostra “item não
  catalogado” sem quebrar a página; a ordem é estável por slot; atalhos (`character_short_skill`) não
  aparecem, porque são interface do jogador.
- **Contrato:** `game.v_character_equipment` e `game.v_character_skill` (`§2.2`, `§2.3`).

### RF-POR-021 — Ficha do item com estatísticas
- **Entrada:** item da armaria ou da WYDpedia.
- **Saída:** nome, categoria, grau, efeitos pelo símbolo, refino, ancient, gema e, quando houver
  origem provada, dano e alcance; durabilidade não é exibida como número.
- **Aceitação:** nenhum número de dano, alcance ou durabilidade é inventado; a ficha exibe
  `stat_source` e declara a estatística como não medida enquanto `P-POR-08` não provar a origem;
  durabilidade não vira coluna nem número (`0009` §10, item 3); quando a captura fechar, o teste cobre
  dano e alcance. `LAC-POR-08` fechada em 2026-11-22.
- **Contrato:** `content.v_item_sheet` (`§2.4`) e o caminho de verificação de `§4` do mesmo documento.

### RF-POR-022 — Filtro e ordenação do equipamento
- **Entrada:** slot ou categoria, grau e texto opcional.
- **Saída:** lista filtrada e ordenada, paginada por cursor.
- **Aceitação:** filtro combina com ordenação sem perder item; o desempate é por identificador e portanto
  estável; resultado vazio tem estado vazio, não `404`.
- **Contrato:** consulta sobre `§2.2`/`§2.4` no adapter; nenhuma tabela nova.

### RF-POR-023 — Anúncio de venda com item real em custódia
- **Entrada:** personagem, slot do item, tipo (`venda`/`leilao`), preço e, no leilão, `ends_at`.
- **Saída:** anúncio `ativo` **somente** depois de custódia confirmada; o item deixa o inventário no mesmo
  instante.
- **Aceitação:** item inexistente, já retido ou de personagem de outra conta é recusado; nenhum anúncio
  fica `ativo` sem `escrow_ref`; repetir a requisição com a mesma chave não cria segundo anúncio nem
  segunda custódia.
- **Contrato:** `portal.market_listing` (`§3.4`), `game.market_escrow` (`§3.11`) e `POST
  /characters/{character_id}/escrow` do `wyd-admin` (C4 §4.9), proposto em 2026-11-22 (`LAC-POR-07`
  fechada).

### RF-POR-024 — Busca e filtro do mercado
- **Entrada:** categoria, faixa de preço, ordenação (`recentes`, `preco`, `popularidade`) e texto.
- **Saída:** vitrine paginada com preço, tipo e popularidade de cada anúncio.
- **Aceitação:** cada filtro tem índice correspondente (`§6` do documento de banco); filtro por categoria
  não faz varredura de tabela; popularidade sai de `bid_count`/`view_count`; paginação por cursor, sem
  `OFFSET`.
- **Contrato:** `ix_market_listing_item_status_preco`, `_kind_status_preco`, `_status_popularidade`.

### RF-POR-025 — Leilão com prazo e lances
- **Entrada:** lance em anúncio `leilao` ativo, antes de `ends_at`.
- **Saída:** lance ativo com crédito reservado; no fechamento, um vencedor e uma cobrança.
- **Aceitação:** lance menor ou igual ao maior ativo é recusado; lance sem saldo é recusado; lance depois
  de `ends_at` é recusado; fechar duas vezes não cobra duas vezes (`uq_market_transaction_listing` mais
  idempotência); perdedores têm a reserva devolvida e recebem aviso.
- **Contrato:** `portal.market_bid` (`§3.5`), `portal.market_transaction` (`§3.6`); fechamento
  idempotente por job, com dono em `LAC-POR-09` (`proposta`: `P-POR-09` recomenda BFF fecha e
  `wyd-ops` agenda/alerta).

### RF-POR-026 — Carteira e extrato em crédito do portal
- **Entrada:** conta do portal autenticada.
- **Saída:** saldo e extrato com tipo, valor, saldo após, referência e instante.
- **Aceitação:** saldo nunca negativo (`ck_market_wallet_saldo`); saldo sempre igual à soma do ledger da
  conta; nenhuma leitura nem escrita em `game.account_wallet`; recarga vem de `portal.donate_credit`,
  nunca de lançamento manual de staff.
- **Contrato:** `portal.market_wallet` (`§3.1`) e `portal.market_ledger` (`§3.2`).

### RF-POR-027 — Transação registrada e conservação do dinheiro
- **Entrada:** compra imediata ou fechamento de leilão.
- **Saída:** linha em `market_transaction` com bruto, taxa e líquido; item entregue ao comprador.
- **Aceitação:** `taxa + líquido = bruto`; débito do comprador igual ao bruto; crédito do vendedor igual ao
  líquido; falha na entrega reverte dinheiro e custódia na mesma transação; comprador e vendedor
  consultam a transação no próprio extrato.
- **Contrato:** `market_transaction` (`§3.6`) e a operação `entregar` do contrato de `§5`.

### RF-POR-028 — Taxa e comissão
- **Entrada:** regra de taxa vigente no instante do anúncio.
- **Saída:** `fee_bps` no anúncio, `fee_amount` na transação e linha `taxa` no ledger.
- **Aceitação:** mudar a regra não altera anúncio já ativo; a taxa não é creditada a conta nenhuma; a
  linha de taxa aponta para a transação; taxa `0` é permitida e registrada como tal.
- **Contrato:** `portal.market_fee_rule` (`§3.3`) e `kind = 'taxa'` em `market_ledger`.

### RF-POR-029 — Reputação do vendedor
- **Entrada:** avaliação do comprador depois de transação concluída.
- **Saída:** nota de 1 a 5 com comentário opcional; agregado público do vendedor.
- **Aceitação:** uma avaliação por transação (`uq_market_review_transaction`); só o comprador da
  transação avalia; o agregado conta apenas transações concluídas e avaliações visíveis; a média aparece
  com a contagem ao lado; avaliação oculta não soma.
- **Contrato:** `portal.market_review` (`§3.7`) e `portal.market_reputation` (`§3.8`).

### RF-POR-030 — Notificação de oferta, lance e venda
- **Entrada:** evento do mercado (lance, lance superado, arremate, venda, expiração, disputa, avaliação).
- **Saída:** aviso in-app com contador de não lidos e marcação de leitura.
- **Aceitação:** cada evento gera exatamente um aviso por destinatário; marcar como lido não apaga;
  o contador zera ao ler; nenhum aviso carrega dado de outra conta.
- **Contrato:** `portal.market_notification` (`§3.9`) e `ix_market_notification_conta_nao_lida`.

### RF-POR-031 — Acesso irrestrito ao mercado
- **Entrada:** conta do portal com sessão válida.
- **Saída:** vitrine, compra e venda liberadas.
- **Aceitação:** nenhum gate de nível, tier, classe ou tempo de jogo; conta de jogo recém-criada compra e
  vende; a única exigência é sessão de portal; `member` não recebe restrição adicional.
- **Contrato:** rotas 45–50 de `rotas-e-permissoes.md` (acrescentadas em 2026-11-22); nenhum gate de
  nível ou classe no servidor.

### RF-POR-032 — Disputa e suporte
- **Entrada:** comprador ou vendedor de transação concluída, com motivo e descrição.
- **Saída:** disputa registrada e fila de staff com decisão e desfecho.
- **Aceitação:** uma disputa aberta por transação (`uq_market_dispute_aberta`); só as partes abrem;
  decidir exige staff com segundo fator (`RF-POR-013`) e grava `audit_log`; o desfecho aplica estorno ou
  liberação e notifica as duas partes.
- **Contrato:** `portal.market_dispute` (`§3.10`); `audit_log` (`M21`).

### RF-POR-033 — Antifraude e limites
- **Entrada:** toda escrita do mercado.
- **Saída:** trilha auditável e limites aplicados no servidor.
- **Aceitação:** toda escrita tem chave de idempotência (`M17`); limite de anúncios ativos por conta e de
  lances por minuto é aplicado no servidor, não no cliente; compra da própria conta e ciclo de
  transferência entre as mesmas contas são recusados; toda decisão de staff fica em `audit_log` com autor.
- **Contrato:** `M17`, `audit_log`, `market_ledger` append-only (`§3.2`).

## 5. Requisitos não funcionais

| ID | Requisito | Critério verificável |
|---|---|---|
| RNF-POR-001 | Toda rota administrativa valida sessão e papel no servidor, não no cliente | teste E2E: `member` recebe 403 em `/admin/*` |
| RNF-POR-002 | Nenhum segredo em log, resposta ou página | varredura de logs e de resposta HTTP sem `password_hash`, `pin_hash`, `token_hash` |
| RNF-POR-003 | Acessibilidade mínima AA nos dois temas | contraste ≥ 4.5:1 no texto e foco visível em todos os controles |
| RNF-POR-004 | Página pública responde em menos de 1 s no P95 local | medição no E2E com orçamento de 1000 ms |
| RNF-POR-005 | Mutação administrativa é atômica com a auditoria | falha injetada no `audit_log` desfaz a mutação |
| RNF-POR-006 | Cobertura mínima de 85% no portal | gate de cobertura do workspace |
| RNF-POR-007 | Credencial e URL interna de observabilidade nunca chegam ao navegador | varredura de resposta HTML/JSON sem `PROMETHEUS_URL`, `LOKI_URL`, `GRAFANA_URL` e sem segredo |
| RNF-POR-008 | Armaria e mercado refletem o dado com janela declarada | cada tela declara o instante da leitura e a janela de cache (≤ 60 s, como `RF-POR-007`); escrita aparece no carregamento seguinte |
| RNF-POR-009 | Mercado escala com muitas transações | vitrine e busca com índice e cursor; `p95` ≤ 300 ms na listagem e ≤ 2 s no fechamento, medidos com 10 mil anúncios; fechamento concorrente não liquida duas vezes; atraso do fechamento tem métrica e alerta |
| RNF-POR-010 | Escrita do mercado é segura e rastreável | nenhuma escrita sem idempotência; `UPDATE`/`DELETE` revogados em `market_ledger`; 100% das decisões de staff em `audit_log` |

## 6. Rastreio rápido

| Requisito | Rota | Tabela/contrato | Lacuna |
|---|---|---|---|
| RF-POR-001..004 | `/cadastro`, `/login`, `/conta/*` | `portal.*`, `game.account` | — |
| RF-POR-013 | `/login` (staff), `/admin/seguranca` | `portal.person_authenticator` (lote `0031`) | `LAC-POR-05` fechada; schema desenhado |
| RF-POR-005, 007 | `/`, `/rankings` | Home usa views de `0029`; rankings consultam `game.character`/`game.guild` com `LIMIT` e cache BFF de 60 s | `LAC-POR-04` fechada |
| RF-POR-006 | `/noticias*`, `/wydpedia/guias/*` | `portal.news`/`banner`/`guide` | — (lote `0030` aprovado em 2026-11-20) |
| RF-POR-008, 009 | `/wydpedia/*`, `/atlas*` | `content.*` | — |
| RF-POR-010 | `/admin`, `/admin/canais` | `0007`, `audit_log`; métricas pela API do `wyd-ops` | `LAC-POR-06` fechada; forma do contrato de métricas no `wyd-ops` |
| RF-POR-014, 015 | `/admin/monitoramento`, `/admin/monitoramento/servidor`, `/admin/monitoramento/prometheus` | Prometheus (`wyd_*`, `up`, alertas) + `game.v_live_session` | `LAC-POR-06` reconfirmada em 2026-11-20 |
| RF-POR-016, 017 | `/admin/monitoramento/grafana`, `/admin/monitoramento/logs` | Grafana (`/api/health`, `/api/search`) e Loki (`query`, `query_range`) | `LAC-POR-06`; retenção no `wyd-ops` |
| RF-POR-018 | `/admin/monitoramento/launcher` | `POST /api/launcher/telemetria` + log estruturado no Loki | `LAC-POR-06`; sem tabela |
| RF-POR-011 | `/admin/jogadores*`, `/admin/contas*` | `0005`–`0010` + C4 | contrato do `wyd-admin` |
| RF-POR-012 | `/admin/config` | `0002` §5.9 | — |
| RF-POR-019 a 022 | `/personagem/[id]` | Views `game.v_character_*` e `content.v_item_sheet` (`wyd-db/armaria-e-mercado.md` §2) | `LAC-POR-08` fechada (2026-11-22); captura de dano/alcance em `P-POR-08` |
| RF-POR-023 a 029 | `/mercado`, `/mercado/[id]`, `/mercado/minhas`, `/mercado/carteira` | `portal.market_*` (`§3`); `game.market_escrow` (`§3.11`); contrato `wyd-admin` C4 §4.9–§4.13 | `LAC-POR-07` fechada (2026-11-22); `LAC-POR-09` em proposta (`P-POR-09`) |
| RF-POR-030 a 033 | `/mercado/*`, `/admin/mercado` | `market_notification`, `market_dispute`, `audit_log` | — |
| RNF-POR-008 a 010 | `/mercado*`, `/personagem/[id]` | índices de `§6`; `market_ledger` append-only | — |
