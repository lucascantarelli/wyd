# WYDCLI-F2 — Fechamento

> Pergunta: "o que a fatia F2 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-client`
> Base: `docs/roadmap/roadmap.md` fatia F2; `docs/specs/requisitos.md` (`RF-CLI-005`, `RF-CLI-006`, `RF-CLI-007`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F2 — Fechamento
**Fase:** 3


## Contexto

A fatia F2 do `wyd-client` cobre lacunas fechadas, documentação final, suíte e existe para
cumprir `RF-CLI-005`, `RF-CLI-006`, `RF-CLI-007` sem antecipar a fatia seguinte.

## Entregável

lacunas fechadas, documentação final, suíte

## DoD

- [ ] `LCLI-02`/`LCLI-03`/`LCLI-05` resolvidas ou com dono+prazo.
- [ ] cobertura ≥ 85% onde aplicável.

## Fora de escopo

distribuição pública

## Depende de

F0–F1; CI real

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F2 concluída em `docs/roadmap/roadmap.md`.
