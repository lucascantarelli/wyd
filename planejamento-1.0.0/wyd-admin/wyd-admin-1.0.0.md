> Pergunta: "qual é o estado medido do planejamento do `wyd-admin` e o que falta para começar o código?"
> Público: Produto, arquitetura e equipe do wyd-admin
> Dono: equipe do wyd-admin
> Base: árvore em `planejamento-1.0.0/wyd-admin/`; `wyd-portal/docs/arquitetura/fronteira.md` §3; `AGENTS.md` §6
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-21); custódia de item do mercado acrescentada em 2026-11-22. Planejamento
aberto, **nenhum código escrito**, nenhuma issue criada.

## 1. Escopo da `1.0.0`

API HTTP/JSON interna, server-to-server, que o BFF do portal chama para **escrever em `game`**: moderação
de conta, carteira, canal, imposto, preço e **custódia de item do mercado global** (`custodiar`,
`entregar`, `devolver`, `cancelar`). Kick e broadcast entram quando existir controle com o `wyd-server`
(`LAC-ADM-03`). O admin não tem tela, não atende navegador e não escreve em `portal`.

A direção está decidida (`LAC-POR-01`, 2026-11-20); a **forma** do contrato é proposta desta árvore:
7 rotas de ação de staff, 4 de custódia e 3 utilitárias, com `Idempotency-Key`, erro tipado e prazos.

## 2. Métricas do planejamento

| Medida | Valor | Fonte (`V`, 2026-11-21) |
|---|---:|---|
| Páginas em `docs/` | 8 | `find docs -name '*.md'` |
| Páginas com cabeçalho de 5 campos | 8 | `grep -rl '^> Pergunta:' docs` |
| Requisitos funcionais | 15 | `docs/specs/requisitos.md` (11 de 2026-11-21 + 4 de custódia) |
| Regras de negócio | 8 | idem |
| Requisitos não funcionais | 5 | idem |
| Rotas no contrato | 14 | 7 de ação de staff + 4 de custódia + `health`, `ready`, `actions/{key}` |
| Fatias de roadmap | 10 | `docs/roadmap/roadmap.md` |
| Lacunas abertas | 2 | `lacunas.md` §Abertas (`03`, `04`); `02`, `06` e `07` fecharam em 2026-11-22 |
| Arquivos de código | 0 | `find src tests -type f` (só `.gitkeep`) |
| Issues criadas | 0 | nenhuma ação no GitHub |

## 3. O que está pronto para revisão

| Página | Responde |
|---|---|
| [Fronteira](docs/arquitetura/fronteira.md) | o que faz, o que não faz, quem consome, o que quebra |
| [Contrato C4](docs/arquitetura/contrato-c4.md) | rotas, payload, idempotência, erros, prazos, auth |
| [Ações](docs/specs/acoes.md) | tabela, transação, constraint e auditoria de cada ação |
| [Requisitos](docs/specs/requisitos.md) | `RF`/`RN`/`RNF` com critério de aceitação |
| [Componentes](docs/arquitetura/componentes.md) | unidades, invariantes, árvore de `src/` e `tests/` |
| [Plano de testes](docs/planos/plano-de-testes.md) | alvos, isolamento, comandos, 85% |
| [Roadmap](docs/roadmap/roadmap.md) | 9 fatias com DoD executável |

## 4. Gates

```bash
grep -rL '^> Pergunta:' planejamento-1.0.0/wyd-admin/docs --include='*.md'   # vazio
find planejamento-1.0.0/wyd-admin -type f \( -name '*.rs' -o -name '*.sql' -o -name '*.ts' \)  # vazio
cd wyd-docs && python preparar.py && mkdocs build --dir .build/docs --output site && python verificar.py site
```

O gate do site inclui o `wyd-admin` no `SISTEMAS`; link relativo quebrado reprova.

## 5. Bloqueios antes da primeira linha

1. `LAC-ADM-04` — autenticação portal → admin.
2. `LAC-ADM-03` — controle com o `wyd-server` (só para `ADM-08`).

Os três bloqueios de schema e de papel fecharam: `LAC-ADM-01` em 2026-11-21 (papel `wyd_admin` e `GRANT`
no `0032`), `LAC-ADM-02` e `LAC-ADM-06` em 2026-11-22 (`ops.idempotency_record`, `0033`, retenção de 7
dias) e `LAC-ADM-07` em 2026-11-22 (`game.market_escrow` e as dez `portal.market_*`, `0034`). `LAC-ADM-05`
não bloqueia desde 2026-11-21 — a escrita de `game.account_moderation` é deste sistema (C4 §4.1) e o
documento do portal foi corrigido. `ADM-01` (esqueleto e saúde) pode começar sem 1–2; `ADM-02` depende de
1 (`LAC-ADM-04`) e `ADM-08` de 2 (`LAC-ADM-03`).

## 6. Próximo passo

Revisão do contrato por quem consome (BFF do portal) e pelos donos das lacunas. Aprovado o contrato,
`ADM-01` e `ADM-02` são as primeiras fatias; nenhuma issue é aberta sem pedido explícito.
