# WYDDB-028 — testar contrato da série

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Fase:** 6 — testes de lacuna e harness
**Prioridade:** P3

## Contexto

O contrato da série precisa reprovar buraco, `down` incompleto, FK para o futuro, seed fora de ordem, constraint anônima e tipo proibido, sem abrir conexão.

## Entregável

Criar o alvo `contrato_migrations` com fixture boa e ruim, lendo a série como texto e apontando arquivo/linha do defeito.

## DoD

- [ ] Fixture boa passa; cada uma das sete invariantes tem uma fixture ruim que falha.
- [ ] O teste não abre conexão nem usa `sqlx`.
- [ ] A mensagem de falha nomeia o arquivo e a regra.
- [ ] O alvo é declarado no `Cargo.toml` e roda com `cargo test -p wyd-db --test contrato_migrations`.

## Fora de escopo

Não implementar o parser de catálogo, não rodar contra PG e não substituir o teste de sistema.

## Risco

O leitor atual não entende schema qualificado; a fixture deve cobrir o caso que o parser novo precisa suportar.

## Depende de

WYDDB-027.

## Evidência ao fechar

Saída do alvo e fixtures versionadas.
