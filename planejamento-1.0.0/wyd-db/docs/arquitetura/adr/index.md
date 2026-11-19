> Pergunta: "qual decisão estrutural já foi tomada, e em que estado ela está?"
> Público: Arquiteto de schema e desenvolvedor que for mexer em `crates/wyd-db/`
> Dono: equipe do `wyd-db`
> Base: —
> Revisado em: 2026-11-18

# wyd-db — decisões de arquitetura (ADR-DB)

Uma ADR por decisão estrutural que muda schema, fronteira de escrita entre schemas ou a forma de
operar o banco. O estado segue [`docs/convencoes.md`](../../convencoes.md): `proposto` é decisão
escrita aguardando aceite do dono; `aceito` é decisão vigente, e o código deve segui-la.

| # | Decisão | Estado |
|---|---|---|
| [ADR-DB-0001](adr-db-0001-provisionamento-de-papel-fora-da-serie.md) | Provisionamento de papel é fase própria, fora da série versionada | aceito |
| [ADR-DB-0002](adr-db-0002-runner-de-migrations-proprio.md) | Runner de migrations próprio, com ledger por versão | aceito |
