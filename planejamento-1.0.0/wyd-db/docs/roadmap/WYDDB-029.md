# WYDDB-029 — testar contrato catálogo ↔ série

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 6 — testes de lacuna e harness

**Dono do valor:** fase, requisitos, dependência, entregável, DoD e fora de
escopo são desta página; `fases.md` cita o link e não repete.
**Prioridade:** P3

## Contexto

O catálogo e a série podem divergir sem ninguém notar; o alvo de contrato precisa comparar as duas fontes e reprovar tabela ausente, schema errado ou nota faltante.

## Entregável

Criar `contrato_catalogo` para comparar o documento de catálogo com os `CREATE TABLE` da série, incluindo schema e nota, sem abrir banco.

## DoD

- [ ] Teste reprova tabela ausente, schema divergente e tabela sem nota.
- [ ] O alvo lê a série viva e o catálogo como texto.
- [ ] A saída aponta a tabela e a página do dicionário.
- [ ] O teste roda sem PostgreSQL.

## Fora de escopo

Não gerar o catálogo, não criar ferramenta de parser e não medir cobertura.

## Risco

O catálogo pode não existir ainda; a issue deve declarar a dependência e não passar por ausência.

## Depende de

WYDDB-028.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans` |

A skill principal é a que o agente carrega antes de começar. A lista é a fechada de nove skills de
`.github/ISSUE_TEMPLATE/*` (`planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md` §3.1).
Duas existem **em disco** em `.agents/skills/`: `planejar-sistema` e `auditar-planejamento`. As demais
(`documentation-writer`, `finishing-a-development-branch`, `frontend-design`, `github-actions-docs`,
`python-testing-patterns`, `skill-creator`, `writing-plans`) são **cloud skills do projeto**: não têm
`SKILL.md` no repositório e são carregadas pelo ambiente do agente.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do alvo e diff do catálogo quando existir.
