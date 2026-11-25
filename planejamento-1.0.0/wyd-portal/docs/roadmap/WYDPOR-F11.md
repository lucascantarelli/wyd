# WYDPOR-F11 — Mercado: disputa, antifraude e fechamento

> Pergunta: "o que a fatia F11 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F11; `docs/specs/requisitos.md` (`RF-POR-032`, `RF-POR-033`, `RNF-POR-010`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Fatia:** F11 — Mercado: disputa, antifraude e fechamento
**Fase:** 6

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; o roadmap cita o link e não repete.

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

A skill principal é a que o agente carrega antes de começar. A lista é a fechada de nove skills de
`.github/ISSUE_TEMPLATE/*` (`planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md` §3.1).
Duas existem **em disco** em `.agents/skills/`: `planejar-sistema` e `auditar-planejamento`. As demais
(`documentation-writer`, `finishing-a-development-branch`, `frontend-design`, `github-actions-docs`,
`python-testing-patterns`, `skill-creator`, `writing-plans`) são **cloud skills do projeto**: não têm
`SKILL.md` no repositório e são carregadas pelo ambiente do agente.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F11 concluída em `docs/roadmap/roadmap.md`.
