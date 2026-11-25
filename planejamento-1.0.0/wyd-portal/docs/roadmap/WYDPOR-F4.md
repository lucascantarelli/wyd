# WYDPOR-F4 — Admin: NOC, canais, logs e configuração

> Pergunta: "o que a fatia F4 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F4; `docs/specs/requisitos.md` (`RF-POR-010`, `RF-POR-012`, `RF-POR-014` a `RF-POR-018`, `RNF-POR-001`, `RNF-POR-005`, `RNF-POR-007`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Fatia:** F4 — Admin: NOC, canais, logs e configuração
**Fase:** 4

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; o roadmap cita o link e não repete.

## Contexto

A fatia F4 do `wyd-portal` cobre `/admin`, `/admin/canais`, `/admin/logs`, `/admin/config`, as seis telas de monitoramento 38–43, middleware de papel e existe para
cumprir `RF-POR-010`, `RF-POR-012`, `RF-POR-014` a `RF-POR-018`, `RNF-POR-001`, `RNF-POR-005`, `RNF-POR-007` sem antecipar a fatia seguinte.

## Entregável

`/admin`, `/admin/canais`, `/admin/logs`, `/admin/config`, as seis telas de monitoramento 38–43, middleware de papel

## DoD

- [ ] `member` recebe 403 em `/admin`.
- [ ] alteração de canal grava revisão e auditoria.
- [ ] revisão sem mudança é recusada.
- [ ] cada tela de monitoramento tem paridade com o legado (`monitoramento.md` §7).
- [ ] CPU/RAM/latência, logs e painéis vêm das APIs do `wyd-ops` com instante da amostra e viram “não medido” quando a fonte falha.
- [ ] nenhuma URL interna ou credencial chega ao navegador.

## Fora de escopo

kick/broadcast como SQL do portal (passam pelo `wyd-admin`); substituir o Grafana ou guardar série no banco

## Depende de

F1; `0007`, `0002`; contrato do `wyd-admin` (`LAC-POR-01`); Prometheus, Loki e Grafana do `wyd-ops` (`LAC-POR-06`, `monitoramento.md` §4)

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `frontend-design` |
| `skills_complementares` | `documentation-writer` |

A skill principal é a que o agente carrega antes de começar. A lista é a fechada de nove skills de
`.github/ISSUE_TEMPLATE/*` (`planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md` §3.1).
Duas existem **em disco** em `.agents/skills/`: `planejar-sistema` e `auditar-planejamento`. As demais
(`documentation-writer`, `finishing-a-development-branch`, `frontend-design`, `github-actions-docs`,
`python-testing-patterns`, `skill-creator`, `writing-plans`) são **cloud skills do projeto**: não têm
`SKILL.md` no repositório e são carregadas pelo ambiente do agente.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F4 concluída em `docs/roadmap/roadmap.md`.
