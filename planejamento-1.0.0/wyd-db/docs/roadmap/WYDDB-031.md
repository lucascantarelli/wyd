# WYDDB-031 — testar fronteira de papéis e views

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 6 — testes de lacuna e harness
**Prioridade:** P3

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

## Evidência ao fechar

Saída do alvo com as DSNs de papel e banco descartável.
