# WYDDB-032 — testar stack de capacidades

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 6 — testes de lacuna e harness


## Contexto

A matriz `CAP-nn` liga requisito a alvo; sem um alvo que a execute, a cobertura é declaração sem prova.

## Entregável

Criar `sistema_stack_caps` para executar os cenários da matriz `CAP-nn`, um por capacidade, e falhar quando uma capacidade não tiver cenário.

## DoD

- [ ] Cada linha `CAP-nn` tem cenário e alvo declarados.
- [ ] Uma capacidade sem cenário reprova o alvo, não passa vazio.
- [ ] O teste usa banco real e dado isolado.
- [ ] A saída lista capacidade, cenário e resultado.

## Fora de escopo

Não inventar capacidades, não definir cobertura percentual e não substituir os alvos de contrato/sistema.

## Risco

A matriz existe desde 2026-11-24 (`tests/specs/capacidades.md`, **23** capacidades e **0** cobertas); o risco
agora é o inverso: cenário escrito sem capacidade, ou capacidade declarada coberta sem cenário executado.

## Depende de

WYDDB-030 e WYDDB-031; a matriz `CAP-nn` está escrita em `tests/specs/capacidades.md` desde 2026-11-24.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Matriz versionada e saída do alvo.
