# WYDPOR-F11 — Mercado: disputa, antifraude e fechamento

> Pergunta: "o que a fatia F11 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F11; `docs/specs/requisitos.md` (`RF-POR-032`, `RF-POR-033`, `RNF-POR-010`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F11 — Mercado: disputa, antifraude e fechamento
**Fase:** 6


## Contexto

A fatia F11 do `wyd-portal` cobre `/admin/mercado`, `/admin/mercado/disputas`; disputa com desfecho, limites por conta e trilha de auditoria e existe para
cumprir `RF-POR-032`, `RF-POR-033`, `RNF-POR-010` sem antecipar a fatia seguinte.

## Entregável

`/admin/mercado`, `/admin/mercado/disputas`; disputa com desfecho, limites por conta e trilha de auditoria

## DoD

- [ ] uma disputa aberta por transação.
- [ ] só as partes abrem.
- [ ] staff decide com segundo fator e a decisão fica em `audit_log`.
- [ ] desfecho aplica estorno ou liberação e notifica as partes.
- [ ] `member` recebe `403` em `/admin/mercado`.
- [ ] limite de anúncios ativos e de lances por minuto é recusado no servidor.
- [ ] ciclo de transferência entre as mesmas contas é recusado.

## Fora de escopo

detecção estatística de fraude, repasse financeiro externo e saque de crédito

## Depende de

F8; F10 para o aviso de disputa

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F11 concluída em `docs/roadmap/roadmap.md`.
