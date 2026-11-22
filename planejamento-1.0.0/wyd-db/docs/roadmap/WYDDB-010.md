# WYDDB-010 — provar cadeia de erro sem segredo

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 2 — contratos e erros tipados
**Prioridade:** P1

## Contexto

Erros encadeados são úteis, mas uma mensagem de conexão pode carregar a DSN inteira. O crate precisa de um teste que varra a saída de todas as variantes.

## Entregável

Adicionar teste de redação que percorre `MigrateError` e `ProvisionError`, incluindo `source`, e falha se a saída contiver senha, token, `postgres://` ou texto de SQL.

## DoD

- [ ] Teste usa uma DSN fictícia com senha e confirma ausência do valor em `Display` e `Debug`.
- [ ] Erro de SQL não inclui o `up`/`down` inteiro.
- [ ] Nenhuma mensagem imprime PIN, hash ou token.
- [ ] O teste roda sem banco e sem variável de ambiente.

## Fora de escopo

Não implementar logger, collector ou destino de log; isso é da fase 5.

## Risco

Um `#[derive(Debug)]` novo pode furar a redação; o teste precisa cobrir os tipos públicos.

## Depende de

WYDDB-005, WYDDB-008, WYDDB-009.

## Evidência ao fechar

Teste de redação verde e revisão do checklist de PR.
