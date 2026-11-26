# Decisões do `wyd-ops` — custo de produção

> Pergunta: "onde a produção roda, dado que o free tier não serve?"
> Público: frente `wyd-ops`, arquitetura e dono do produto
> Dono: frente `wyd-ops`; custo de produção é decisão do dono do produto
> Base: `proposta-gestao-github-e-ambientes.md` §3.2 e §7.3; `docs/index.md`;
> `wyd-ops/docs/devops/index.md`; `wyd-ops/plano-imagem-e-compose-do-site.md`; `AGENTS.md` §4 e §9
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-23); a hospedagem foi respondida em **A** pelo dono em 2026-11-24. §8 registra a ação 2 como
executada e §10 lista os campos do project, mas não repete os issue fields; §3.2 ainda rotula os
tipos/campos como "Proposta em aberto". Essa divergência documental é registrada abaixo e não vira
nova pergunta de desenho. **Nada foi aplicado**: não há VPS, contrato, ambiente publicado, custo ou
capacidade medidos; a escolha foi decisão de custo, não estimativa. **Nenhum número desta página é
medição** de hospedagem.

## 1. Fronteira

**Fecha:** hospedagem de produção (`§7.3`). **Não reabre:** tipos/campos de issue: §8 registra a
ação 2 como executada em 2026-11-19; §10 lista os campos do project, não repete os issue fields; §3.2
ainda diz "Proposta em aberto".

**Não decide:** CI (já decidido em `wyd-ops/docs/devops/index.md`), imagem do site (decidida em
`wyd-ops/plano-imagem-e-compose-do-site.md`), regra de jogo, fio ou escrita administrativa.

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| lacunas formais | **5** — 4 fechadas, 1 bloqueada (aplicação de `Q-OPS-2`); o board do GitHub fechou em 2026-11-26 | `docs/lacunas.md` §1 |
| `RF-OPS`, roadmap e plano de teste | **existem** desde 2026-11-25 | `docs/specs/requisitos.md`; `docs/roadmap/`; `docs/planos/plano-de-testes.md` |
| CI real | **implementado em F0+F1 (2026-11-26)**; seis workflows e quatro gates ativos | `AGENTS.md` §4/§9; `wyd-ops/docs/devops/index.md` |
| produção | **nada publicado**; free tier não serve | `wyd-ops/proposta-gestao-github-e-ambientes.md` §7.3 |
| custo de hospedagem | **não medido** | idem |

## 3. Já decidido — não reabrir

| Item | Decisão vigente | Fonte | Efeito |
|---|---|---|---|
| CI | só `run:` e action própria; nenhuma action de terceiro sem SHA | `wyd-ops/docs/devops/index.md`; `docs/index.md` | não é decisão desta rodada |
| tipos/campos de issue | `Bug`/`Task`/`Feature`, `Priority` P0–P3 e `Effort` P/M/G | `wyd-ops/proposta-gestao-github-e-ambientes.md` §3.2, §8 e §10 | §8 registra a ação 2 como executada em 2026-11-19; §10 não repete os issue fields; §3.2 conserva o rótulo "Proposta em aberto". Divergência da fonte; não é pergunta de desenho |
| deploy | `docker compose` em host com systemd; não Kubernetes | `wyd-ops/docs/arquitetura/adr/adr-ops-0002-hospedagem-producao.md` | a decisão de host não muda o runtime |
| imagem do site | CI constrói e Compose consome | `wyd-ops/plano-imagem-e-compose-do-site.md` | não é decisão desta rodada |
| free tier | não serve para produção de jogo; serve para vitrine/homologação | `AGENTS.md` §9; `wyd-ops/proposta-gestao-github-e-ambientes.md` §7.3 | a pergunta é qual host pago/controlado |

## 4. Decisões

### Q-OPS-2 — Onde a produção roda?

**Medido.** Nada está publicado; o documento diz que free tier não sustenta processo persistente,
porta TCP e estado em memória. Não há custo, VPS nem máquina **registrados**. A escolha foi respondida
em **A** pelo dono em 2026-11-24; a infraestrutura, o contrato e o backup continuam não executados.

**O que está em jogo.** Máquina própria dá controle e custo fixo; VPS barata reduz operação; crédito
de provedor adia o custo, mas cria dependência. O número é decisão de custo, não medição.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | VPS paga com processo persistente e deploy Compose, escolhida por custo/controle | produção fora do free tier; exige contrato, backup e operação |
| B | Máquina própria | controle total; exige rede, energia, backup e operação local |
| C | Crédito de provedor | adia o custo; expira e não é plano de produção estável |

**Recomendação: A.** É a opção que mantém o runtime Compose já decidido e não depende de crédito
temporário; o valor do contrato é decisão do dono, não estimativa desta página. **Custo:** contrato e
operação. **Fontes:** `wyd-ops/proposta-gestao-github-e-ambientes.md` §7.3; `AGENTS.md` §9; `wyd-ops/docs/arquitetura/adr/adr-ops-0002-hospedagem-producao.md`.
**Recusa B** por operação local; **C** por prazo.

**Resposta do dono (2026-11-24): A — VPS paga com processo persistente e deploy Compose.** A decisão
escolhe o ambiente, mas **não cria** VPS, contrato, rotina de backup, RPO/RTO nem página de operação.
O que resta é aplicação: contratar/escolher o provedor, provisionar o host, configurar Compose e
conectar a rotina de backup de `Q-RET-7`; nada disso foi executado.

## 5. Cobertura das propostas

| Item | Pergunta | Situação |
|---|---|---|
| tipos/campos de issue (`§3.2`) | — | §8 registra a ação 2 como executada em 2026-11-19; §10 lista os campos do project, não os issue fields; §3.2 mantém "Proposta em aberto". Divergência da fonte, sem pergunta de desenho |
| hospedagem de produção (`§7.3`) | `Q-OPS-2` | **respondida em A (2026-11-24)**; VPS paga com processo persistente e Compose; contrato, provisionamento e backup pendentes |

## 6. Registro da decisão

**Respondida pelo dono em 2026-11-24.** A hospedagem de produção será **VPS paga com processo
persistente e deploy Compose** (A); §8 registra a ação 2 dos tipos/campos como executada, mas §3.2
conserva o rótulo antigo. **Nada foi aplicado**: contrato, VPS, backup e operação continuam pendentes.

| Pergunta | Item | Resposta |
|---|---|---|
| `Q-OPS-2` | hospedagem de produção | **A — VPS paga com processo persistente e Compose** (2026-11-24); aplicação pendente |
