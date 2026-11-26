# Pacote de aprovação — contratos O1–O5 do `wyd-ops`

> Pergunta: "o que cada dono precisa responder para liberar O1–O5 e o que continua bloqueado se a resposta não vier?"
> Público: arquitetura, dono do produto, `wyd-ops`, `wyd-bots`, `wyd-server`, `wyd-net` e `wyd-docs`
> Dono: equipe do `wyd-ops`; cada aprovação pertence ao dono listado na seção
> Base: `docs/lacunas.md` §4 (`OPS-03.1`–`OPS-03.5`); `wyd-ops-1.0.0.md` §3; `docs/arquitetura/componentes.md`; `AGENTS.md` §5, item 8
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Instrumento de leitura e aprovação; **não decide, não aprova e não altera** `lacunas.md` §4. Nenhum "sim" foi registrado.

## 1. Como responder

Cada seção tem uma pergunta fechada e um conjunto de donos. **Sim** aprova o contrato na forma descrita e autoriza registrar a resposta em `lacunas.md` §4; **não** deixa `OPS-03.x` bloqueada e devolve a objeção; ausência de resposta mantém bloqueada. O pacote não fecha a decisão por conta própria.

## 2. O1 — arquitetura + `wyd-ops` + `wyd-bots`

**Pergunta (sim/não):** a arquitetura, o `wyd-ops` e o `wyd-bots` aprovam o contrato O1 — Postgres e servidor descartáveis por execução do bot, com ciclo de vida, cleanup e artefato por execução, sem estado global entre cenários?

**O que aprova:** o contrato operacional, não a política de `LBOT-01`/`LBOT-02`, que já está decidida. **ADR:** não; a política já existe e o que falta é a forma do contrato.

**Se ficar aberto:** o `wyd-bots` fica sem ambiente isolado aprovado; a medição de `LBOT-03` continua sem execução em CI; o plano de teste dos bots não tem o que executar.

## 3. O2 — arquitetura + `wyd-ops` + `wyd-server` + `wyd-net`

**Pergunta (sim/não):** a arquitetura, o `wyd-ops`, o `wyd-server` e o `wyd-net` aprovam o contrato O2 — `/metrics` Prometheus e logs de metadados, com nomes e labels, intervalo de coleta, retenção e schema de log definidos, sem payload nem PII?

**O que aprova:** a forma do contrato, não a política de `LNET-04`/`LAC-POR-06`, que já está decidida. **ADR:** não; a política já existe e o que falta é a forma do contrato.

**Se ficar aberto:** o monitoramento fica sem formato acordado; `RF-OPS-004` não pode ser implementado; dashboards, alertas e retenção ficam sem base; o diagnóstico de incidente continua manual.

## 4. O3 — arquitetura + `wyd-ops` + `wyd-docs`

**Pergunta (sim/não):** a arquitetura, o `wyd-ops` e o `wyd-docs` aprovam o contrato O3 — publicação do artefato estático no Pages por workflow, com janela, aprovação e rollback ensaiado?

**O que aprova:** o contrato operacional, não a decisão de `LPDOC-01`/`ADR-DOC-0001` de que publicar é ato à parte. **ADR:** não; a decisão já existe e o que falta é a forma do contrato.

**Se ficar aberto:** a publicação do site fica sem contrato reproduzível; o rollback não pode ser exigido; cada publicação vira uma exceção manual.

## 5. O4 — arquitetura + `wyd-ops`

**Pergunta (sim/não):** a arquitetura e o `wyd-ops` aprovam o contrato O4 — inventário de segredos, emissor, consumidor, armazenamento e rotação, com segredo de controle, DSN, chave de assinatura e token de serviço separados?

**O que aprova:** uma decisão nova; `LAC-ADM-03` §3.3 cobre só o token de controle. **ADR:** sim, depois do "sim" e antes da implementação.

**Se ficar aberto:** sem inventário e rotação, o deploy de servidor e admin fica bloqueado; segredos podem ficar sem dono ou compartilhados; a auditoria de segurança não fecha.

## 6. O5 — produto + arquitetura + `wyd-ops`

**Pergunta (sim/não):** o produto, a arquitetura e o `wyd-ops` aprovam o contrato O5 — staging, gatilho, aprovação, artefato e rollback de servidor e launcher, com rollback por comando e dono definidos?

**O que aprova:** uma decisão nova; O1–O4 não cobrem deploy de servidor nem launcher. **ADR:** sim, depois do "sim" e antes do primeiro deploy real.

**Se ficar aberto:** o primeiro deploy real de servidor e launcher fica bloqueado; não há staging nem rollback ensaiado; a release vira um ato manual sem critério de volta.

## 7. Mensagens prontas para envio (não enviadas)

Textos curtos derivados de §2–§6 e de `docs/lacunas.md` §4, para copiar e enviar ao dono indicado.
Cada item é o desbloqueio citado em `docs/lacunas.md` §4: o registro aponta `§7.1`–`§7.7` e não
repete a pergunta. A visão consolidada de todas as decisões pendentes do projeto é a
pauta de decisões em `pauta-de-decisoes-pendentes.md`; ela aponta de volta para cada item desta seção.
**Nenhuma foi enviada**: não houve issue, e-mail, Slack ou qualquer outro canal. A repetição da pergunta
e do impacto é intencional: a mensagem precisa se sustentar sozinha fora deste repositório, e o registro
continua sendo `docs/lacunas.md` §4. Um "sim" dito na conversa só vale depois de registrado lá.

### 7.1 Arquitetura — O1 a O5

**Para:** arquitetura · **Assunto:** Aprovação dos contratos O1–O5 do `wyd-ops` (sim/não)

*Registro consolidado: [pauta de decisões pendentes](pauta-de-decisoes-pendentes.md).*

> Preciso do seu sim/não em cinco contratos do `wyd-ops`. Cada um pede a forma do contrato; as políticas
> já estão decididas. **O1** — Postgres e servidor descartáveis por execução do bot, com ciclo de vida,
> cleanup e artefato por execução, sem estado global entre cenários (sem ADR). **O2** — `/metrics`
> Prometheus e logs de metadados, com nomes, labels, intervalo de coleta, retenção e schema de log, sem
> payload nem PII (sem ADR). **O3** — publicação do artefato estático no Pages por workflow, com janela,
> aprovação e rollback ensaiado (sem ADR). **O4** — inventário de segredos com emissor, consumidor,
> armazenamento e rotação (com ADR depois do sim). **O5** — staging, gatilho, aprovação, artefato e
> rollback de servidor e launcher (com ADR depois do sim).
>
> Se ficar aberto: O1 deixa o `wyd-bots` sem ambiente isolado e `LBOT-03` sem execução em CI; O2 deixa
> `RF-OPS-004` sem formato e o incidente manual; O3 deixa a publicação sem contrato reproduzível e o
> rollback sem exigência; O4 bloqueia o deploy de servidor e admin; O5 bloqueia o primeiro deploy real de
> servidor e launcher. Pode responder por contrato — um "sim" parcial também serve.

### 7.2 Produto — O5

**Para:** dono do produto · **Assunto:** Aprovação do contrato O5 — deploy de servidor e launcher (sim/não)

*Registro consolidado: [pauta de decisões pendentes](pauta-de-decisoes-pendentes.md).*

> Você aprova o contrato **O5** — staging, gatilho, aprovação, artefato e rollback de servidor e launcher,
> com rollback por comando e dono definidos? É decisão nova, exige ADR depois do sim e antes do primeiro
> deploy real. Se ficar aberto, o primeiro deploy de servidor e launcher continua bloqueado, não há staging
> nem rollback ensaiado, e a release vira um ato manual sem critério de volta.

### 7.3 `wyd-ops` — O1 a O5

**Para:** equipe do `wyd-ops` · **Assunto:** Confirmação dos contratos O1–O5 (sim/não)

*Registro consolidado: [pauta de decisões pendentes](pauta-de-decisoes-pendentes.md).*

> Como dono dos cinco contratos, você confirma **O1 a O5** na forma do pacote, junto com a arquitetura e os
> consumidores? O1–O3 não reabrem as políticas de origem; O4 e O5 abrem ADR depois do sim. Se algum ficar
> aberto: sem O1 o `wyd-bots` não tem ambiente isolado; sem O2 não há formato de métrica e log para
> implementar; sem O3 a publicação do site fica manual; sem O4 o deploy de servidor e admin fica bloqueado;
> sem O5 não há staging nem rollback de servidor e launcher.

### 7.4 `wyd-bots` — O1

**Para:** equipe do `wyd-bots` · **Assunto:** Aprovação do contrato O1 — ambiente efêmero (sim/não)

*Registro consolidado: [pauta de decisões pendentes](pauta-de-decisoes-pendentes.md).*

> O `wyd-bots` aprova o contrato **O1** — Postgres e servidor descartáveis por execução, com ciclo de vida,
> cleanup e artefato por execução, sem estado global entre cenários? A política já está decidida em
> `LBOT-01`/`LBOT-02`; falta só a forma do contrato operacional. Se ficar aberto, os bots ficam sem ambiente
> isolado aprovado, a medição de `LBOT-03` continua sem execução em CI e o plano de teste não tem o que executar.

### 7.5 `wyd-server` — O2

**Para:** equipe do `wyd-server` · **Assunto:** Aprovação do contrato O2 — métricas e logs (sim/não)

*Registro consolidado: [pauta de decisões pendentes](pauta-de-decisoes-pendentes.md).*

> O `wyd-server` aprova o contrato **O2** — `/metrics` Prometheus e logs de metadados, com nomes e labels de
> métrica, intervalo de coleta, retenção e schema de log definidos, sem payload nem PII? A política já está
> decidida em `LNET-04`/`LAC-POR-06`; falta a forma. Se ficar aberto, `RF-OPS-004` não pode ser implementado,
> dashboards e alertas ficam sem base e o diagnóstico de incidente continua manual.

### 7.6 `wyd-net` — O2

**Para:** equipe do `wyd-net` · **Assunto:** Aprovação do contrato O2 — métricas e logs (sim/não)

*Registro consolidado: [pauta de decisões pendentes](pauta-de-decisoes-pendentes.md).*

> O `wyd-net` aprova o contrato **O2** — `/metrics` Prometheus e logs de metadados, com nomes e labels de
> métrica, intervalo de coleta, retenção e schema de log definidos, sem payload nem PII? A política já está
> decidida em `LNET-04`/`LAC-POR-06`; falta a forma. Se ficar aberto, o que o `wyd-net` emite não tem nome
> nem label acordados com a coleta, `RF-OPS-004` fica sem formato e dashboards e alertas ficam sem base.

### 7.7 `wyd-docs` — O3

**Para:** equipe do `wyd-docs` · **Assunto:** Aprovação do contrato O3 — publicação do site (sim/não)

*Registro consolidado: [pauta de decisões pendentes](pauta-de-decisoes-pendentes.md).*

> O `wyd-docs` aprova o contrato **O3** — publicação do artefato estático no Pages por workflow, com janela de
> publicação, aprovação e rollback ensaiado? A decisão de que publicar é ato à parte já existe em
> `LPDOC-01`/`ADR-DOC-0001`; falta a forma do contrato. Se ficar aberto, a publicação fica sem contrato
> reproduzível, o rollback não pode ser exigido e cada publicação vira uma exceção manual.

## 8. Limites

- Não cria ADR, issue, board, commit, push ou deploy.
- `docs/lacunas.md` §4 continua sendo o registro único; este pacote só organiza a leitura por dono.
- `OPS-04`, `OPS-05`, `D-06`, `Q-SERIE-1`, `Q-SEP-1`, `LNET-01`/`LBOT-03` e `LCLI-02`/`LCLI-05` ficam fora deste pacote e têm mensagem no [pacote fora de O1–O5](pacote-aprovacao-decisoes-fora-o1-o5.md) §7.
- O1–O3 não reabrem as políticas de origem; O4 e O5 exigem ADR após a aprovação.
