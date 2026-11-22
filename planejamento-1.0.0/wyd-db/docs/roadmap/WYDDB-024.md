# WYDDB-024 — correlacionar erro por versão e arquivo

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 5 — observabilidade e diagnóstico
**Prioridade:** P2

## Contexto

Um erro de SQL sem a versão e o arquivo obriga o operador a procurar no diretório inteiro; a taxonomia já carrega contexto, falta garantir a correlação.

## Entregável

Garantir que todo erro de aplicação carregue `Version`/`slug` e que log, `Display` e `Report` usem o mesmo identificador, sem o texto do SQL.

## DoD

- [ ] Teste de erro em arquivo 3 confirma versão e slug na mensagem e no log.
- [ ] O mesmo identificador aparece no resumo e na evidência da issue.
- [ ] Erro de lock não é confundido com erro de arquivo.
- [ ] Nenhuma mensagem inclui o conteúdo do `up`/`down`.

## Fora de escopo

Não criar correlation-id distribuído, OpenTelemetry ou coletor; a correlação é local ao runner.

## Risco

Um erro de conexão antes do arquivo não tem versão; a mensagem deve dizer que a falha é anterior à série.

## Depende de

WYDDB-008, WYDDB-022.

## Evidência ao fechar

Teste de mensagem/log e página `src/migrate/error.md`.
