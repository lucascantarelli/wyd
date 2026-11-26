# Módulos do `wyd-db` — detalhamento

> Pergunta: "o que cada módulo do `wyd-db` entrega, suas interfaces, dependências, testes, segurança, monitoramento e DoD?"
> Público: Desenvolvimento
> Dono: `wyd-db`
> Base: `docs/arquitetura/componentes-c4.md`; `docs/arquitetura/contexto-c4.md`; `docs/arquitetura/modelo-logico.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25).

O `wyd-db` é o piloto: o detalhamento por módulo vive em `docs/arquitetura/` (componentes-c4, contexto-c4,
fluxo-de-dados, integridade-e-constraints, migracoes, padroes-postgres, retencao-e-capacidade, seguranca,
visao-geral). Esta página é o índice.

## Módulos

| Módulo | Detalhe | DoD |
|---|---|---|
| provisionamento | `docs/arquitetura/migracoes.md`, `padrao-banco.md` | migrations aplicam/desfazem |
| repositórios | `docs/arquitetura/modelo-logico.md`, `fluxo-de-dados.md` | testes de contrato |
| filas/save/shutdown | `docs/arquitetura/fluxo-de-dados.md` | efeito único |
| observabilidade | `docs/arquitetura/seguranca.md` | incidente diagnosticado |
| segurança | `docs/arquitetura/seguranca.md` | sem segredo em log |
| baseline/lint | `WYDDB-001` | cargo doc sem warning |