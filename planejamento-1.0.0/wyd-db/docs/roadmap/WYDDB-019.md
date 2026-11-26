# WYDDB-019 — aplicar grants de portal sem duplicar

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

Pedidos de item, moeda e grant chegam do portal; reaplicar o mesmo pedido não pode duplicar o efeito nem criar linha órfã no quente.

## Entregável

Implementar a aplicação de grants com chave idempotente, validação de conta/personagem e uma transação que grava o efeito e o registro de idempotência juntos.

## DoD

- [ ] Teste aplica o mesmo grant duas vezes e mantém uma única linha de efeito.
- [ ] Conta inexistente ou bloqueada falha sem escrita parcial.
- [ ] O grant não copia identidade do portal para `game`.
- [ ] O código de erro distingue duplicado de inválido.

## Fora de escopo

Não definir regra de negócio de item, preço ou limite; isso é do produto/servidor.

## Risco

Sem uma chave de idempotência estável, o retry do portal duplica; a chave é do pedido, não do processo.

## Depende de

WYDDB-018.

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

Teste de sistema de grant idempotente e página `src/repo/grants.rs`.
