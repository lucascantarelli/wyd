# ADM-06 — configuração de canal

> Pergunta: "o que a issue ADM-06 entrega, do que depende e como sei que terminou?"
> Público: desenvolvimento, QA e revisor
> Dono: equipe do `wyd-admin`
> Base: `docs/roadmap/roadmap.md`; `docs/arquitetura/contrato-c4.md`; `docs/specs/requisitos.md`; `docs/planos/plano-de-testes.md`
> Revisado em: 2026-11-26

**Estado:** proposto (2026-11-26)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** P1
**Prioridade:** média

## Contexto

A fatia `ADM-06` do `wyd-admin` entrega configuração de canal e serve ao contrato C4 e aos requisitos de
`docs/specs/requisitos.md`. O `wyd-admin` é o único escritor em `game` em nome do portal; esta issue não
abre uma segunda escrita nem uma rota fora do contrato.

## Entregável

`PUT /channels/{g}/{c}`.

## DoD

- [ ] Canal 0 = `422`; atualização grava e audita; nenhum `INSERT`.
- [ ] `make gate-fast` verde no ciclo curto e `make gate` verde no PR.

## Fora de escopo

O fora de escopo da `1.0.0` está em `docs/roadmap/roadmap.md` §Fora de escopo; esta issue não
acrescenta item. Tela, fila durável, escrita em `portal` e regra de jogo continuam fora.

## Depende de

`ADM-03`, `LAC-ADM-01`.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `writing-plans` |
| `skills_complementares` | `planejar-sistema`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída dos comandos do DoD, diff dos arquivos alterados e `make gate` verde no PR.
