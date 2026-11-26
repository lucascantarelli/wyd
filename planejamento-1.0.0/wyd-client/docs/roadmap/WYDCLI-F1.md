# WYDCLI-F1 — Patches e configuração do cliente

> Pergunta: "o que a fatia F1 entrega, do que ela depende e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-client`
> Base: `docs/roadmap/roadmap.md` fatia F1; `docs/specs/requisitos.md` (`RF-CLI-003`, `RF-CLI-004`); `docs/arquitetura/componentes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fatia:** F1 — Patches e configuração do cliente
**Fase:** 2


## Contexto

A fatia F1 do `wyd-client` cobre patches de configuração, arquivos de cliente, validação e existe para
cumprir `RF-CLI-003`, `RF-CLI-004` sem antecipar a fatia seguinte.

## Entregável

patches de configuração, arquivos de cliente, validação

## DoD

- [ ] patch aplica sem quebrar cliente.
- [ ] configuração documentada.
- [ ] teste de regressão verde.

## Fora de escopo

distribuição

## Depende de

F0; `LCLI-02`/`LCLI-03`

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do comando do DoD no PR, com a fatia F1 concluída em `docs/roadmap/roadmap.md`.
