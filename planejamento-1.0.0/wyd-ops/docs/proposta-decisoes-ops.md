# Decisões do `wyd-ops` — custo de produção

> Pergunta: "onde a produção roda, dado que o free tier não serve?"
> Público: frente `wyd-ops`, arquitetura e dono do produto
> Dono: frente `wyd-ops`; custo de produção é decisão do dono do produto
> Base: `proposta-gestao-github-e-ambientes.md` §3.2 e §7.3; `docs/index.md`;
> `wyd-ops/plano-ci-real.md`; `wyd-ops/plano-imagem-e-compose-do-site.md`; `AGENTS.md` §4 e §9
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). Uma pergunta para a proposta de hospedagem ainda aberta. §8 registra a ação 2 como
executada e §10 lista os campos do project, mas não repete os issue fields; §3.2 ainda rotula os
tipos/campos como "Proposta em aberto". Essa divergência documental é registrada abaixo e não vira
nova pergunta de desenho. Nada foi aplicado. **Nenhum número desta página é medição**: não há custo, capacidade nem produção medidos; a
escolha de hospedagem é decisão de custo, não estimativa.

## 1. Fronteira

**Fecha:** hospedagem de produção (`§7.3`). **Não reabre:** tipos/campos de issue: §8 registra a
ação 2 como executada em 2026-11-19; §10 lista os campos do project, não repete os issue fields; §3.2
ainda diz "Proposta em aberto".

**Não decide:** CI (já decidido em `wyd-ops/plano-ci-real.md`), imagem do site (decidida em
`wyd-ops/plano-imagem-e-compose-do-site.md`), regra de jogo, fio ou escrita administrativa.

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| lacunas formais | **0** — `wyd-ops` não tem `lacunas.md`; das duas propostas do texto, uma foi aprovada/executada e uma segue aberta | `docs/index.md` |
| `RF-OPS`, roadmap e plano de teste | **não existem** | `docs/index.md` "O que falta" |
| CI real | **stub**; workflow de docs é o único gate ativo | `AGENTS.md` §4/§9; `wyd-ops/plano-ci-real.md` |
| produção | **nada publicado**; free tier não serve | `wyd-ops/proposta-gestao-github-e-ambientes.md` §7.3 |
| custo de hospedagem | **não medido** | idem |

## 3. Já decidido — não reabrir

| Item | Decisão vigente | Fonte | Efeito |
|---|---|---|---|
| CI | só `run:` e action própria; nenhuma action de terceiro sem SHA | `wyd-ops/plano-ci-real.md`; `docs/index.md` | não é decisão desta rodada |
| tipos/campos de issue | `Bug`/`Task`/`Feature`, `Priority` P0–P3 e `Effort` P/M/G | `wyd-ops/proposta-gestao-github-e-ambientes.md` §3.2, §8 e §10 | §8 registra a ação 2 como executada em 2026-11-19; §10 não repete os issue fields; §3.2 conserva o rótulo "Proposta em aberto". Divergência da fonte; não é pergunta de desenho |
| deploy | `docker compose` em host com systemd; não Kubernetes | `wyd-ops/plano-ci-real.md` §3 | a decisão de host não muda o runtime |
| imagem do site | CI constrói e Compose consome | `wyd-ops/plano-imagem-e-compose-do-site.md` | não é decisão desta rodada |
| free tier | não serve para produção de jogo; serve para vitrine/homologação | `AGENTS.md` §9; `wyd-ops/proposta-gestao-github-e-ambientes.md` §7.3 | a pergunta é qual host pago/controlado |

## 4. Decisões

### Q-OPS-2 — Onde a produção roda?

**Medido.** Nada está publicado; o documento diz que free tier não sustenta processo persistente,
porta TCP e estado em memória. Não há custo, VPS nem máquina **registrados**; a escolha está aberta.

**O que está em jogo.** Máquina própria dá controle e custo fixo; VPS barata reduz operação; crédito
de provedor adia o custo, mas cria dependência. O número é decisão de custo, não medição.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | VPS paga com processo persistente e deploy Compose, escolhida por custo/controle | produção fora do free tier; exige contrato, backup e operação |
| B | Máquina própria | controle total; exige rede, energia, backup e operação local |
| C | Crédito de provedor | adia o custo; expira e não é plano de produção estável |

**Recomendação: A.** É a opção que mantém o runtime Compose já decidido e não depende de crédito
temporário; o valor do contrato é decisão do dono, não estimativa desta página. **Custo:** contrato e
operação. **Fontes:** `wyd-ops/proposta-gestao-github-e-ambientes.md` §7.3; `AGENTS.md` §9; `wyd-ops/plano-ci-real.md` §3.
**Recusa B** por operação local; **C** por prazo.

## 5. Cobertura das propostas

| Item | Pergunta | Situação |
|---|---|---|
| tipos/campos de issue (`§3.2`) | — | §8 registra a ação 2 como executada em 2026-11-19; §10 lista os campos do project, não os issue fields; §3.2 mantém "Proposta em aberto". Divergência da fonte, sem pergunta de desenho |
| hospedagem de produção (`§7.3`) | `Q-OPS-2` | aberta; a escolha de host pago/controlado continua sem resposta |

## 6. Registro da decisão

**Aguardando resposta do dono.** A hospedagem continua em aberto; §8 registra a ação 2 dos
tipos/campos como executada, mas §3.2 conserva o rótulo antigo. Nada foi aplicado nesta rodada.

| Pergunta | Item | Resposta |
|---|---|---|
| `Q-OPS-2` | hospedagem de produção | — |
