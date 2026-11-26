# WYDPOR-F5 — Admin: jogadores, contas e economia

> Pergunta: "o que a fatia F5 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-portal`
> Base: `docs/roadmap/roadmap.md` fatia F5; `docs/specs/requisitos.md` (`RF-POR-011`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F5 — Admin: jogadores, contas e economia
**Fase:** 4


## Contexto

A fatia F5 do `wyd-portal` cobre `/admin/jogadores*`, `/admin/contas*`, `/admin/economia*`, `/admin/seguranca` e existe para
cumprir `RF-POR-011` sem antecipar a fatia seguinte.

## Entregável

`/admin/jogadores*`, `/admin/contas*`, `/admin/economia*`, `/admin/seguranca`

## DoD

- [ ] ficha não devolve hash.
- [ ] bloqueio sem motivo recusado.
- [ ] ajuste de moeda vira linha no ledger.
- [ ] toda ação com autor em `audit_log`.

## Fora de escopo

alteração de inventário item a item (precisa de decisão de operação)

## Depende de

F4; `0005`, `0006`, `0008`–`0011`; `wyd-admin` para ação no processo

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `frontend-design` |
| `skills_complementares` | `documentation-writer` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F5 concluída em `docs/roadmap/roadmap.md`.
