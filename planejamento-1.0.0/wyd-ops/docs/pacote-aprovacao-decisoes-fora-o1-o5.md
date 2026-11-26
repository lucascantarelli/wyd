# Pacote de aprovação — decisões fora dos contratos O1–O5

> Pergunta: "o que falta aprovar além de O1–O5, quem responde cada item e o que continua bloqueado sem resposta?"
> Público: dono do produto, arquitetura, `wyd-db`, QA, `wyd-ops` e revisão jurídica
> Dono: equipe do `wyd-ops`; cada aprovação pertence ao dono listado na seção
> Base: `docs/lacunas.md` §4 (`OPS-04`, `OPS-05`, `D-06`, `Q-SERIE-1`, `Q-SEP-1`, `LNET-01`/`LBOT-03`, `LCLI-02`/`LCLI-05`); `docs/planos/medicoes-lnet-01-lbot-03.md`; `docs/proposta-decisoes-ops.md`; `ADR-OPS-0002`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Instrumento de leitura e aprovação das decisões que o pacote O1–O5
declara fora de escopo; **não decide, não aprova e não altera** `docs/lacunas.md` §4. Em 2026-11-25 o dono
respondeu às decisões de §2–§5: `Q-SERIE-1` = B foi implementada e provada como ciclo SQL em PG17,
`D-06` = A, `OPS-04` foi adiada, `OPS-05` foi executada em 2026-11-26 (labels e 71 issues, PR #128) e
`Q-SEP-1` = A′ renomeou os 28 nomes para `_` (todos parseiam). Nenhuma mensagem foi enviada.

## 1. Como responder

Cada seção tem uma pergunta fechada e um conjunto de donos. **Sim** aprova a decisão na forma descrita e autoriza registrar a resposta em `docs/lacunas.md` §4; **não** devolve a objeção e mantém o item bloqueado; ausência de resposta mantém bloqueado. A pergunta canônica continua sendo a de `docs/lacunas.md` §4 — este pacote só organiza a leitura e dá o texto do pedido.

## 2. Produto — `OPS-04`, `OPS-05` e `D-06`

| Item | Pergunta (sim/não) | O que aprova | ADR |
|---|---|---|---|
| `OPS-04` | O produto e o `wyd-ops` aprovam o provedor de VPS, o contrato, a região, a rotina de backup e o RPO/RTO da hospedagem `Q-OPS-2` = A? | aplicação de uma decisão já tomada (`ADR-OPS-0002`), com custo e credencial | não |
| `OPS-05` | O produto autoriza criar o board e as novas issues a partir de `docs/github-mapping.md`? | ato de gestão no GitHub, não arquitetura | não |
| `D-06` | O produto confirma a próxima frente depois de `D-05`? | prioridade entre sistemas | não |

**Se ficar aberto:** sem `OPS-04` a produção continua inexistente e o backup e o RPO/RTO seguem indefinidos; `OPS-05` **já foi executada em 2026-11-26** — 61 fatias e 10 lacunas criadas com `area/*`, restando apenas o board. **`D-06` = A foi decidida em 2026-11-25** (frente `wyd-db`) e não está mais aberta.

## 3. Arquitetura e `wyd-db` — `Q-SERIE-1` e `Q-SEP-1`

**Pergunta única em aberto — `Q-SEP-1` revisitada:** a arquitetura e o `wyd-db` escolhem **A′** (renomear
os 28 para `_`), **B′** (`Slug` aceitar `-`) ou **C′** (normalizar `-`→`_` no parser)? As opções, a
recomendação **A′** e o custo de cada uma estão no documento dono,
`planejamento-1.0.0/wyd-db/migrations/serie-e-regras.md` §9.1; esta página não repete as letras porque
uma cópia local já descreveu opção diferente da canônica.

**`Q-SERIE-1` = B já está decidida e implementada:** os 28 `down` existem e o ciclo `up → down → up`
passou em PG17 por `psql`; `ADR-DB-0002` fica como está. **`Q-SEP-1` = A′ foi decidida e aplicada:** os
28 arquivos foram renomeados para `NNNN_slug_com_underscores.sql` e todos parseiam. **`CAP-37` também já
fechou**, por `runner_pg` (5/5 em PG17, incluindo o advisory lock em duas sessões).

**O que resta na série:** as versões `0016`, `0017`, `0018`, `0020`, `0023` e `0027` dependem de curadoria
de seed; sem elas o runner não aplica a série real, embora a porta já esteja provada. `Q-SEP-1` não gerou
ADR.

## 4. QA e `wyd-ops` — `LNET-01` e `LBOT-03`

**Pergunta (sim/não):** QA e `wyd-ops` autorizam a implementação mínima de servidor e bot e a execução de `docs/planos/medicoes-lnet-01-lbot-03.md` no CI efêmero, para congelar os valores de `LNET-01` e `LBOT-03`?

**O que aprova:** a execução da medição, não a política — `Q-NET-1` = A e `Q-BOT-3` = A já foram decididas. **ADR:** não; falta medição, não decisão estrutural.

**Resposta do dono (2026-11-25): sim** — a implementação mínima e a execução da medição estão autorizadas; falta executar no CI efêmero de `OPS-03.1`. **Enquanto não executada:** os dois números continuam sem valor medido; limites de rede e carga de bots ficam sem base para o plano de teste, e nenhum P50/P99 pode ser citado.

## 5. Jurídico — `LCLI-02` e `LCLI-05`

**Pergunta (sim/não):** o produto e a revisão jurídica confirmam que há liberação do titular para distribuir o cliente e para a classificação jurídica de `LCLI-02` e `LCLI-05`?

**O que aprova:** a liberação de distribuição e a classificação jurídica; a distribuição continua congelada até a resposta. **ADR:** não — direito e contrato, não arquitetura. **Resposta do dono (2026-11-25):** opção A de `LCLI-02` (dados + launcher próprio) e de `LCLI-05` (só dados e peças próprias, distribuição congelada), com o pedido `Q-IP-1`/`Q-IP-2` aprovado para envio; a resposta do titular continua pendente e é ela que fecha as duas.

**Se ficar aberto:** o cliente segue congelado, sem patch publicado nem arquivo redistribuído; o pedido ao titular está redigido e não enviado, e nenhuma decisão técnica substitui essa resposta.

## 6. Resumo por entrada

| Entrada em `docs/lacunas.md` §4 | Dono | Mensagem |
|---|---|---|
| `OPS-04` | produto + `wyd-ops` | §7.1 |
| `OPS-05` | produto | §7.1 |
| `D-06` | produto | §7.1 |
| `Q-SERIE-1` | arquitetura + `wyd-db` | §7.2 — **decidida em B e provada como SQL** |
| `Q-SEP-1` | arquitetura + `wyd-db` | §7.2 — **decidida em A′ e aplicada** |
| `LNET-01`/`LBOT-03` | QA + `wyd-ops` | §7.3 |
| `LCLI-02`/`LCLI-05` | produto + revisão jurídica | §7.4 |

## 7. Mensagens prontas para envio (não enviadas)

Textos curtos derivados de §2–§5 e de `docs/lacunas.md` §4, para copiar e enviar ao dono indicado.
Cada item é o desbloqueio citado em `docs/lacunas.md` §4, que aponta `§7.1`–`§7.4` e não repete a
pergunta. A visão consolidada de todas as decisões pendentes do projeto é a
pauta de decisões em `pauta-de-decisoes-pendentes.md`; ela aponta de volta para cada item desta seção. **Nenhuma foi enviada**: não houve issue, e-mail, Slack ou qualquer outro canal. A repetição
da pergunta e do impacto é intencional, porque a mensagem precisa se sustentar fora do repositório; o
registro continua sendo `docs/lacunas.md` §4 e um "sim" dito na conversa só vale depois de registrado lá.

### 7.1 Produto — OPS-04, OPS-05 e D-06

**Para:** dono do produto · **Assunto:** Duas aprovações em aberto fora dos contratos O1–O5 (sim/não)

*Registro consolidado: [pauta de decisões pendentes](pauta-de-decisoes-pendentes.md).*

> Preciso do seu sim/não em dois itens que ficaram fora do pacote O1–O5. **OPS-04** — provedor de VPS,
> contrato, região, rotina de backup e RPO/RTO da hospedagem `Q-OPS-2` = A: a decisão já existe em
> `ADR-OPS-0002`, falta aplicação, custo e credencial; ela foi **adiada** para depois do primeiro deploy
> interno. **OPS-05** — autorização para criar o board e as novas issues a partir do `github-mapping.md`;
> o repositório já tem 40 issues e milestones, a autorização existe e nada foi criado; falta decidir o
> namespace de label (`area/*` existente × `sistema/*` do mapping). **D-06** está **decidida em A** desde
> 2026-11-25 (frente `wyd-db`) e não pede nova resposta.
>
> **OPS-05 executada em 2026-11-26:** labels e 71 issues no repositório, com PR #128; o board foi
> montado depois, com 121 itens em 2026-11-26. **OPS-05 está fechada.**
>
> Se ficar aberto: sem `OPS-04` a produção continua inexistente, sem backup nem RPO/RTO. O texto
> abaixo é histórico da autorização; o estado vigente é o da pauta canônica.

### 7.2 Arquitetura e wyd-db — Q-SERIE-1 e Q-SEP-1

**Para:** arquitetura + equipe do `wyd-db` · **Assunto:** Compatibilidade dos nomes das migrations (A′/B′/C′)

*Registro consolidado: [pauta de decisões pendentes](pauta-de-decisoes-pendentes.md).*

> A arquitetura e o `wyd-db` escolhem como resolver **`Q-SEP-1` revisitada**, agora que `Q-SERIE-1` = B
> está implementada e provada como ciclo SQL em PG17? **A′** renomear os 28 arquivos para `_`
> (**recomendada**), **B′** fazer `Slug` aceitar `-`, **C′** normalizar `-`→`_` no parser. O custo e o
> risco de cada uma estão em `planejamento-1.0.0/wyd-db/migrations/serie-e-regras.md` §9.1; nada foi
> renomeado nem alterado no parser.
>
> **Resolvido sem nova pergunta:** `Q-SEP-1` = A′ foi decidida e aplicada em 2026-11-25 — os 28 nomes
> usam `_` — e `CAP-37` fechou por `runner_pg`. Resta apenas a contiguidade das seis versões ausentes.

### 7.3 QA e wyd-ops — LNET-01 e LBOT-03

**Para:** QA + equipe do `wyd-ops` · **Assunto:** Autorização de execução das medições `LNET-01` e `LBOT-03` (sim/não)

*Registro consolidado: [pauta de decisões pendentes](pauta-de-decisoes-pendentes.md).*

> QA e `wyd-ops` autorizam a implementação mínima de servidor e bot e a execução de
> `docs/planos/medicoes-lnet-01-lbot-03.md` no CI efêmero, para congelar os valores de `LNET-01` e
> `LBOT-03`? A política já está decidida (`Q-NET-1` = A, `Q-BOT-3` = A); falta a medição, não ADR.
>
> Se ficar aberto: os dois números continuam sem valor medido, limites de rede e carga de bots ficam sem
> base para o plano de teste, e nenhum P50/P99 pode ser citado.

### 7.4 Jurídico — LCLI-02 e LCLI-05

**Para:** produto + revisão jurídica · **Assunto:** Liberação de distribuição e classificação jurídica do cliente (sim/não)

*Registro consolidado: [pauta de decisões pendentes](pauta-de-decisoes-pendentes.md).*

> O produto e a revisão jurídica confirmam que há liberação do titular para distribuir o cliente e para
> a classificação jurídica de `LCLI-02` e `LCLI-05`? É direito e contrato, não arquitetura, então não
> gera ADR. O pedido ao titular está redigido e não enviado.
>
> Se ficar aberto: o cliente continua congelado, sem patch publicado nem arquivo redistribuído, e
> nenhuma decisão técnica substitui essa resposta.

## 8. Limites

- Não cria ADR, issue, board, commit, push ou deploy, e não envia mensagem.
- `docs/lacunas.md` §4 continua sendo o registro único; este pacote só organiza a leitura por dono.
- Os contratos O1–O5 continuam no [pacote O1–O5](pacote-aprovacao-o1-o5.md); este documento não os reabre.
