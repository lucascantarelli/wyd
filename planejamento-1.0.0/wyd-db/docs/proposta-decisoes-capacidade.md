# Retenção e capacidade — a decisão que falta

> Pergunta: "a página `arquitetura/retencao-e-capacidade.md` pode ser escrita antes de a série ser
> medida, e o que ela pode afirmar sem volume?"
> Público: arquitetura, operação e `wyd-db`
> Dono: arquitetura + operação (o mesmo dono que `proposta-paginas.md` §6 dá à página)
> Base: `lacunas.md` §2.3 (`M22`, `M25`, `M30`, `M32`) e §2.4; `wyd-db-1.0.0.md` §6.3 e §11;
> `proposta-paginas.md` §6; `docs/planos/plano-de-documentacao.md` §2; `padrao-banco.md` `P-21`;
> `ddl-confirmado.md` §5.1, §5.4, §5.5, §5.16, §5.33 e §5.34; `docs/roadmap/WYDDB-025.md`;
> `docs/arquitetura/fronteira-hot-cold.md`; `proposta-retencao-pendente.md` (os prazos; esta página
> não os decide)
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). Três perguntas, uma por decisão que a página
`retencao-e-capacidade.md` exige antes de existir. Nenhuma está respondida e nada foi aplicado.
**Nenhum número desta página é medição de volume**: o repositório não tem volume medido (§2), e o
único limiar citado (`~10 M` linhas) é decisão de `P-21`, não medição.

## 1. Fronteira desta rodada

**Fecha:** como a página `arquitetura/retencao-e-capacidade.md` nasce sem volume medido; o que ela
pode afirmar por ciclo de vida/estrutura; e quem responde pelo sinal de alerta.

**Não decide:** os prazos das categorias que a `M32` deixou `pendente` — esses estão em
`proposta-retencao-pendente.md` e continuam lá; a rotina de expurgo, o backup e o reaper (operação);
o número medido de `planos/metricas.md` (`M20`); e qualquer DDL ou índice novo.

**Não reabre:** `M22` (sessão e auditoria), `M25` (fila), `M30` (idempotência), `M32` (política de
dado pessoal), `M7` (consentimento versionado), `A12` (fechamento anonimiza e-mail), `P-21`
(retenção declarada e partição) nem `P-22` (PII só em `portal`).

**Descoberta que delimita a rodada.** `lacunas.md` **não tem `M34`**: a última linha numerada é
`M33`, e §2.4 mostra `P3` com apenas `M33`. O item de `P3` que continua o assunto desta rodada é a
página `retencao-e-capacidade.md`, listada como bloqueada por medição em `proposta-paginas.md` §6 e
na 5ª onda de `wyd-db-1.0.0.md` §11. Não é a única página de `P3`; é a que a retenção deixa em
aberto. É esse item — não uma milestone numerada inventada — que esta página documenta.

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| `retencao-e-capacidade.md` | **não escrita**; "o que cresce e o que fazer com isso?" | `docs/arquitetura/index.md` |
| `planos/metricas.md` | **não escrito**; o número medido só existe depois da série (`M20`) | `docs/planos/index.md`; `lacunas.md` `M20` |
| volume, latência e capacidade | **nenhum limiar foi medido**; a página de RNF está bloqueada | `docs/specs/requisitos-funcionais.md` |
| o que a página deve cobrir | por tabela de crescimento: volume esperado, retenção, partição, custo de vacuum; quente × histórica; alerta com limiar | `wyd-db-1.0.0.md` §6.3 |
| quando a página pode fechar | a 5ª onda só fecha depois da série medida; `retencao-e-capacidade` está nela | `docs/planos/plano-de-documentacao.md` §2 |
| bloqueio declarado | "faltam ... a medição de volume/capacidade, que só existe depois da série (`M20`)" | `proposta-paginas.md` §6 |
| limiar de partição | `~10 M` linhas, por tempo (`RANGE (occurred_at)`); **decisão de `P-21`, não medição** | `padrao-banco.md` `P-21` |
| candidatos a partição | auditoria e sessão, "candidatos futuros" | `padrao-banco.md` `P-21` |
| índices de expurgo já no DDL | `ix_person_session_expira`, `ix_audit_log_occurred`, `ix_delivery_queue_fechadas`; `ix_person_token_expira` é parcial (`WHERE used_at IS NULL`) | `ddl-confirmado.md` §5.1/§5.4/§5.16 |
| índices que faltam | `pin_reset` por `expires_at`; `market_notification` por `read_at` | `ddl-confirmado.md` §5.5/§5.34; `proposta-retencao-pendente.md` §2 |
| alerta | nenhum limiar ou alerta automático no `wyd-db`; "depende do `wyd-ops` e de medição" | `docs/roadmap/WYDDB-025.md` |
| dono da operação | retenção, backup, reaper e limite de recurso | `lacunas.md` §0.3 |

**Não há volume, crescimento por dia, bytes por linha, custo de vacuum nem latência medidos.** Todo
número desta rodada é estrutural ou já decidido em outra milestone; onde a página precisaria de um
número, ela declara a lacuna em vez de inventá-lo.

## 3. Já decidido — não reabrir

| Item | Decisão vigente | Fonte | Efeito nesta rodada |
|---|---|---|---|
| `M22` | `person_session` 30 dias após `expires_at`; `game.session` 365 dias após `released_at`; `ops.audit_log` 730 dias | `lacunas.md` §2.3 | a página cita; não rediscute |
| `M25` | fila `applied`/`failed` expurgada 30 dias após `closed_at`; `pending` nunca | `lacunas.md` §2.3 | a página cita; não rediscute |
| `M30` | `ops.idempotency_record` com 7 dias | `lacunas.md` §2.3 | a página cita; não rediscute |
| `M32` | política de dado pessoal; categorias sem prazo ficaram `pendente` com dono | `lacunas.md` §2.3 | os prazos são da `proposta-retencao-pendente.md`; não são desta página |
| `P-21` | toda tabela crescente tem retenção declarada; partição só acima de `~10 M` linhas, por tempo | `padrao-banco.md` `P-21` | é a regra estrutural que a página pode afirmar sem volume |
| `M20` | número medido só depois da série; `metricas.md` não antes | `lacunas.md` `M20` | a página não pode fechar antes da medição |
| `A12` | fechar pessoa anonimiza o e-mail; a linha fica | `lacunas.md` §2.2 | a página trata a linha fechada como histórico, não como lixo |
| `P-22` | PII só em `portal`; `game` referencia `person_id`, nunca copia e-mail | `padrao-banco.md` `P-22` | a capacidade de `game` não é capacidade de PII |
| `M7` | consentimento versionado | `lacunas.md` §2.3 | fora do escopo de capacidade |
| 5ª onda | `retencao-e-capacidade` só fecha depois da série medida | `docs/planos/plano-de-documentacao.md` §2 | a página nasce `proposto`, não `aceito` |

## 4. As três decisões

### Q-CAP-1 — A página é escrita agora, espera a série ou é dividida?

**Medido.** A página não existe; `proposta-paginas.md` §6 a lista como bloqueada por "medição de
volume/capacidade, que só existe depois da série (`M20`)"; `planos/metricas.md` não foi escrito; e a
5ª onda só fecha depois da série medida. Não há volume nem limiar medido para preencher a coluna
"volume esperado".

**O que está em jogo.** Esperar mantém a 5ª onda aberta e não deixa registro do que já está decidido;
escrever agora sem volume arrisca uma página que parece aceita sem responder "quanto cresce?". A
decisão é sobre **o que pode ser escrito sem número**, não sobre inventar o número.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Escrever agora como `proposto`: matriz por tabela com direção de crescimento, retenção já decidida, regra de partição de `P-21`, quente × histórica, custo de vacuum qualitativo e sinal de alerta; toda coluna de volume marcada `pendente de medição` | a página existe e registra o que é estrutural; o bloqueio de `proposta-paginas.md` §6 vira `proposto` com condição, e a página **não** pode ser `aceita` nem fechar a 5ª onda até a medição; a lacuna de volume fica visível, não escondida |
| B | Esperar a série e a medição do catálogo (`M20`) | nenhuma página agora; a 5ª onda continua aberta; o que já está decidido segue só em `lacunas.md` e nas páginas de migration |
| C | Dividir: escrever só a metade de **retenção** agora e deixar **capacidade** (volume, partição, vacuum, alerta) para depois | a metade decidível sai; a página fica com dois donos ou vira duas páginas, contra "um fato, um dono" |

**Recomendação: A.** A página pode afirmar o que é regra — retenção declarada, `P-21`, quente ×
histórica — sem inventar volume; o que falta é número, e número tem dono e data (`M20`). Declarar
`pendente de medição` é diferente de placeholder: é a lacuna com nome. A opção **altera o bloqueio
atual** de `proposta-paginas.md` §6: a página deixa de ser "bloqueada" e passa a `proposto` com
condição de retomada; se o dono preferir fidelidade ao bloqueio, **B** é a resposta segura.
**Custo:** uma página `proposto` que não fecha a onda e uma condição de retomada explícita (série
medida). **Fontes:**
`proposta-paginas.md` §6; `docs/planos/plano-de-documentacao.md` §2; `lacunas.md` `M20`;
`docs/specs/requisitos-funcionais.md` (nenhum limiar medido). **Recusa B** porque não registra o que
já está decidido; **C** porque duplica dono.

### Q-CAP-2 — O que a página pode afirmar sem volume?

**Medido.** A especificação da página pede "volume esperado, política de retenção, partição, custo de
vacuum; tabela quente × histórica; alerta proposto com limiar" (`wyd-db-1.0.0.md` §6.3). Só a
retenção e a regra de partição têm decisão; volume, custo de vacuum e limiar não têm medição.

**O que está em jogo.** Uma matriz por tabela é auditável e mostra o que falta; uma fórmula global
sem valores parece rigorosa e não responde nada; uma lista de tabelas não é página de capacidade.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Matriz por tabela de crescimento: direção (evento/tempo), retenção vigente ou `pendente`, partição por `P-21`, quente × histórica, custo de vacuum qualitativo, sinal de alerta; volume como `pendente de medição` | cada tabela tem uma linha e uma lacuna explícita; a página é verificável contra o DDL e as decisões |
| B | Modelo global com variáveis (linhas/dia, bytes/linha, retenção) e nenhum valor | a página parece completa, mas não diz nada sobre tabela nenhuma; a medição futura teria de refazer a estrutura |
| C | Só a lista de tabelas que crescem e seus donos | não responde "o que fazer com isso?"; não serve ao critério de §11 |

**Recomendação: A.** A pergunta da página é por tabela ("o que cresce e o que fazer com isso?"), e a
matriz é o único formato que mantém a lacuna visível sem inventar número. **Custo:** uma linha por
tabela crescente; as colunas sem medição ficam declaradas. **Fontes:** `wyd-db-1.0.0.md` §6.3;
`padrao-banco.md` `P-21`; `proposta-paginas.md` §6. **Recusa B** porque não responde por tabela;
**C** porque não decide nada.

### Q-CAP-3 — O alerta tem limiar agora ou só sinal e dono?

**Medido.** Não há sistema de alerta no `wyd-db`; `WYDDB-025` diz para não configurar "alerta, painel
ou SLO" e que isso depende do `wyd-ops` e de medição. A operação é dona de retenção, backup, reaper e
limite de recurso (`lacunas.md` §0.3). O único número disponível é o `~10 M` de `P-21`, que é
**limiar de partição**, não de alerta.

**O que está em jogo.** Inventar um limiar de alerta sem volume é o mesmo defeito que inventar
volume; não ter sinal nenhum deixa a página sem a última coluna que §6.3 pede.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | A página propõe **sinal + dono + condição de medição** (ex.: "quando a tabela passar de `~10 M` linhas, reavaliar partição"), sem limiar numérico de alerta | o `wyd-ops` recebe um sinal verificável e a decisão de limiar fica com quem mede; nenhum número inventado |
| B | Usar `~10 M` de `P-21` como limiar de alerta para auditoria e sessão | confunde partição com alerta; o número foi decidido para partição e não para monitoramento |
| C | Remeter todo o alerta ao `wyd-ops` e não escrever a coluna | a página perde a coluna que §6.3 exige; a arquitetura não dá entrada ao alerta |

**Recomendação: A.** Separa o que é regra (sinal e dono) do que é medição (limiar); `P-21` continua
sendo a regra de partição, não um alerta. **Custo:** uma coluna qualitativa e uma condição de
reavaliação; o limiar numérico fica para o `wyd-ops`, com a medição. **Fontes:**
`docs/roadmap/WYDDB-025.md`; `lacunas.md` §0.3; `padrao-banco.md` `P-21`; `wyd-db-1.0.0.md` §6.3.
**Recusa B** porque reusa um limiar de partição como alerta; **C** porque suprime a coluna.

## 5. O quadro que as três decisões produzem

| Decisão | Efeito se A vencer | Já decidido que não muda |
|---|---|---|
| `Q-CAP-1` | página `proposto`, com lacuna de volume declarada | `M20`: número só depois da série; 5ª onda não fecha antes |
| `Q-CAP-2` | matriz por tabela, uma linha por tabela crescente | `P-21`: retenção declarada e partição acima de `~10 M` |
| `Q-CAP-3` | sinal + dono, sem limiar numérico | `WYDDB-025`: sem alerta automático; `wyd-ops` é dono da operação |

## 6. O que esta rodada não decide

- **Não decide os prazos de retenção** das categorias pendentes: eles estão em
  `proposta-retencao-pendente.md` e continuam aguardando resposta.
- **Não mede volume** nem escreve `planos/metricas.md`: o número medido é de `M20`, depois da série.
- **Não cria a rotina de expurgo, o backup, o reaper nem o alerta:** a operação é de `wyd-ops`.
- **Não altera DDL nem índice:** a página de capacidade lê o DDL; não o muda.
- **Não reabre `M22`, `M25`, `M30`, `M32`, `M7`, `A12`, `P-21` nem `P-22`.**
- **Não fecha a 5ª onda:** a página nasce `proposto` e só pode ser `aceita` depois da série medida.

## 7. O que muda quando as três forem respondidas

| # | Entrega | Arquivo | Dono |
|---|---|---|---|
| 1 | página de retenção e capacidade, com matriz e lacuna de volume | `docs/arquitetura/retencao-e-capacidade.md` (nova) | arquitetura + operação |
| 2 | condição de retomada registrada (série medida → preencher volume) | própria página, §volume | arquitetura |
| 3 | sinal de alerta com dono, sem limiar | própria página, §alerta | operação |
| 4 | estado da rodada | `lacunas.md` §2.4 e este registro | `wyd-db` |

A re-medição é a de `M20`: catálogo depois da série. Só então a página pode trocar `pendente de
medição` por número com comando e data.

## 8. O que continua bloqueado depois desta rodada

- **A página `retencao-e-capacidade.md`** continua `proposto` até a série ser medida; a 5ª onda não
  fecha antes.
- **Os prazos de retenção** das categorias pendentes continuam em `proposta-retencao-pendente.md`.
- **O alerta automático** continua fora do `wyd-db`: depende do `wyd-ops` e de medição.
- **O volume por tabela** continua sem medição; não há número a publicar.

## 9. Registro da decisão

**Aguardando resposta do dono.** Nenhuma das três foi respondida; nada foi aplicado nas páginas
donas. Quando houver resposta, o registro entra aqui com a data, a opção escolhida por pergunta, as
alternativas recusadas e o arquivo de cada entrega.

| Pergunta | Resposta | Data |
|---|---|---|
| `Q-CAP-1` | — | — |
| `Q-CAP-2` | — | — |
| `Q-CAP-3` | — | — |
