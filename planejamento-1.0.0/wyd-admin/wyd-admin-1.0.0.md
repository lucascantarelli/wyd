> Pergunta: "qual é o estado medido do planejamento do `wyd-admin` e o que falta para começar o código?"
> Público: Produto, arquitetura e equipe do wyd-admin
> Dono: equipe do wyd-admin
> Base: árvore em `planejamento-1.0.0/wyd-admin/`; `wyd-portal/docs/arquitetura/fronteira.md` §3; `AGENTS.md` §6
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-21); custódia de item do mercado acrescentada em 2026-11-22. Planejamento
aberto, **nenhum código escrito**; as 10 fatias foram publicadas como issues #208–#217 e estão no board `WYD 1.0.0`.

## 1. Escopo da `1.0.0`

API HTTP/JSON interna, server-to-server, que o BFF do portal chama para **escrever em `game`**: moderação
de conta, carteira, canal, imposto, preço e **custódia de item do mercado global** (`custodiar`,
`entregar`, `devolver`, `cancelar`). Kick e broadcast entram na fatia `ADM-08`, com o elo de `LAC-ADM-03`
**decidido em 2026-11-22** (HTTP interno síncrono com ack). O admin não tem tela, não atende navegador e
não escreve em `portal`.

A direção está decidida (`LAC-POR-01`, 2026-11-20); a **forma** do contrato é proposta desta árvore:
7 rotas de ação de staff, 4 de custódia e 3 utilitárias, com `Idempotency-Key`, erro tipado e prazos.

## 2. Métricas do planejamento

| Medida | Valor | Fonte (`V`, 2026-11-21) |
|---|---:|---|
| Páginas em `docs/` | 20 | `find docs -name '*.md'` (`V`, 2026-11-26) |
| Páginas com cabeçalho de 5 campos | 20 | `grep -rl '^> Pergunta:' docs` (`V`, 2026-11-26) |
| Requisitos funcionais | 15 | `docs/specs/requisitos.md` (11 de 2026-11-21 + 4 de custódia) |
| Regras de negócio | 8 | idem |
| Requisitos não funcionais | 5 | idem |
| Rotas no contrato | 14 | 7 de ação de staff + 4 de custódia + `health`, `ready`, `actions/{key}` |
| Fatias de roadmap | 10 | `docs/roadmap/roadmap.md` e as 10 páginas `WYDADM-nn` |
| Lacunas abertas | 0 | `lacunas.md` §Fechadas: as sete fecharam (`01` e `05` em 2026-11-21; `02`, `03`, `04`, `06` e `07` em 2026-11-22) |
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
| [Autenticação e controle](docs/arquitetura/autenticacao-e-controle.md) | as duas decisões de transporte de 2026-11-22: token de serviço portal → admin e canal de controle com o `wyd-server` |
| [Plano de testes](docs/planos/plano-de-testes.md) | alvos, isolamento, comandos, 85% |
| [Roadmap](docs/roadmap/roadmap.md) e [índice](docs/roadmap/index.md) | 10 fatias com DoD executável, uma página por issue |

## 4. Gates

```bash
grep -rL '^> Pergunta:' planejamento-1.0.0/wyd-admin/docs --include='*.md'   # vazio
find planejamento-1.0.0/wyd-admin -type f \( -name '*.rs' -o -name '*.sql' -o -name '*.ts' \)  # vazio
cd wyd-docs && python preparar.py && mkdocs build --dir .build/docs --output site && python -m gate check links
```

O gate do site inclui o `wyd-admin` no `SISTEMAS`; link relativo quebrado reprova.

## 5. Bloqueios antes da primeira linha

**Nenhum.** As duas últimas lacunas fecharam em 2026-11-22, com o "sim" do dono às opções recomendadas
de [autenticação e controle](docs/arquitetura/autenticacao-e-controle.md): token de serviço por ambiente
com dois valores na rotação (`LAC-ADM-04`, §2) e HTTP interno síncrono com ack (`LAC-ADM-03`, §3).

O histórico dos sete bloqueios, para não ser reaberto: `LAC-ADM-01` fechou em 2026-11-21 (papel
`wyd_admin` e `GRANT` no `0032`), `LAC-ADM-05` em 2026-11-21 (a escrita de `game.account_moderation` é
deste sistema), `LAC-ADM-02` e `LAC-ADM-06` em 2026-11-22 (`ops.idempotency_record`, `0033`, retenção de
7 dias), `LAC-ADM-07` em 2026-11-22 (`game.market_escrow` e as dez `portal.market_*`, `0034`) e
`LAC-ADM-03`/`LAC-ADM-04` em 2026-11-22. Nenhuma tabela nasceu das decisões de transporte: a série segue
em **34 arquivos e 109 tabelas**.

## 6. Próximo passo

Revisão do contrato por quem consome (BFF do portal) e, aprovado, o começo do código por `ADM-01`
(esqueleto e saúde) e `ADM-02` (autenticação, papel mínimo e `trace_id`), que não dependem de mais
ninguém. Nenhuma issue é aberta sem pedido explícito.
