# WYDDB-020 — registrar crédito e débito idempotentes

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 4 — filas, save e shutdown

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; `fases.md` cita o link e não repete.
**Prioridade:** P2

## Contexto

Crédito e débito de moeda são operações de dinheiro: repetir a mensagem não pode creditar duas vezes, e falha no meio não pode deixar saldo sem ledger.

## Entregável

Implementar crédito/débito com registro em `ops.idempotency_record`, soma atômica e uma linha de histórico; o saldo não é atualizado fora da transação.

## DoD

- [ ] Teste prova que a mesma chave credita/debita uma vez.
- [ ] Saldo e histórico são consistentes após falha simulada.
- [ ] Valor negativo/zero é recusado onde o contrato exigir.
- [ ] Nenhum valor monetário usa `float`.

## Fora de escopo

Não definir preço, taxa, limite ou regra de negócio; não implementar loja ou cash.

## Risco

A ordem `saldo → histórico` e a chave idempotente precisam ser a mesma transação; testar só o caminho feliz esconde inconsistência.

## Depende de

WYDDB-018, WYDDB-019.

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

Teste de sistema de crédito/débito e página `src/repo/economy.rs`.
