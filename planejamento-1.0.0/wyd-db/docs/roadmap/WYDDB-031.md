# WYDDB-031 — testar fronteira de papéis e views

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 6 — testes de lacuna e harness


## Contexto

A fronteira de escrita é uma garantia do banco; se o teste só olhar o código, um `GRANT` errado passa e o portal escreve em `game`.

## Entregável

Criar `sistema_sessoes` para provar `permission denied` por papel, leitura cruzada só por view e isolamento de sessão.

## DoD

- [ ] Portal não escreve em `game`; jogo não lê `portal.person` cru.
- [ ] As duas views devolvem o recorte esperado para cada papel.
- [ ] Sessão única é arbitrada pelo índice, não por `SELECT` prévio.
- [ ] O teste falha se um `GRANT` for ampliado.

## Fora de escopo

Não definir RLS, política de senha ou `pg_hba.conf`; são do provisionamento/ambiente.

## Risco

Um banco de teste com superusuário mascara a fronteira; o alvo precisa conectar com cada papel.

## Depende de

WYDDB-015, WYDDB-030.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `planejar-sistema` |
| `skills_complementares` | `writing-plans`, `auditar-planejamento` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Saída do alvo com as DSNs de papel e banco descartável.
