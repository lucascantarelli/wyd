# wyd-ops 1.0.0 — abertura do planejamento

> Pergunta: "o que o `wyd-ops` faz, quais contratos ele tem com os outros sistemas e o que falta para começar o código?"
> Público: Produto, arquitetura e equipe do `wyd-ops`
> Dono: equipe do `wyd-ops`
> Base: `wyd-ops/docs/index.md`; `AGENTS.md` §6; `wyd-portal/docs/specs/requisitos.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Abertura inicial: fronteira, contratos e lacunas nomeadas.
**Nenhum código próprio**, nenhum requisito numerado (`RF-OPS`) antes desta rodada, nenhum plano de teste próprio.

**Organização (D-04 = A): fechada em 2026-11-25.** A auditoria (`wyd-ops/auditoria-de-organizacao.md`) listava para este sistema: sem dashboard próprio, sem `RF-OPS`, sem plano de teste e roadmap com DoD, fronteira/contratos parciais. Fechado em 2026-11-25 com `src/.gitkeep` + `tests/.gitkeep`, `docs/arquitetura/componentes.md`, `docs/planos/plano-de-testes.md`, `docs/specs/requisitos.md` (7 `RF-OPS` + 5 `RNF-OPS`) e `docs/roadmap/` (index.md + roadmap.md com DoD por fatia). Fronteira e contratos permanecem parciais: a implementação de CI/imagem/ambiente/deploy segue pendente.

## 1. O que ele faz

É o dono de CI, imagem, ambiente, monitoramento, segredos, deploy e gate do projeto. Compõe o ambiente
efêmero do CI (Postgres + servidor descartáveis), publica o site e o servidor, coleta métricas e executa
o gate que falha de verdade.

## 2. O que ele **não** faz

| Fora | Dono |
|---|---|
| codec e layout do fio | `wyd-protocol` |
| regra de jogo | `wyd-core` / `wyd-sim` |
| conteúdo de sistema | cada sistema |
| hospedagem paga/CDN/domínio próprio | fora do escopo da `1.0.0` |

## 3. Contratos

| # | Contrato | Direção | Forma | Garantia | Dono | Estado |
|---|---|---|---|---|---|---|
| O1 | ambiente efêmero | ops → `wyd-bots` | Postgres + servidor descartáveis por execução | sem estado global entre cenários | `wyd-ops` | **decidido** (`LBOT-01`, `LBOT-02`) |
| O2 | métricas e logs | ops ← `wyd-server` | `/metrics` Prometheus + logs de metadados | sem payload/PII | `wyd-ops` | **decidido** (`LNET-04`, `LAC-POR-06`) |
| O3 | publicação do site | ops → Pages | workflow + artefato estático | rollback validado | `wyd-ops` | **decidido** (`LPDOC-01`) |
| O4 | segredos | ops emite | secret store; nunca em código | rotação documentada | `wyd-ops` | **decidido** (`LAC-ADM-03` §3.3) |

## 4. Estado medido (`V`, 2026-11-25)

| Medida | Valor | Comando |
|---|---:|---|
| Páginas de planejamento | 3 (index, prioridade, proposta-decisoes) | `find planejamento-1.0.0/wyd-ops -name '*.md'` |
| Arquivos de código | 0 | `find crates/wyd-ops -type f` (só `.gitkeep`) |
| Requisitos com ID | 7 `RF-OPS` + 5 `RNF-OPS` | esta rodada |
| Issues criadas | 0 | nenhuma ação no GitHub |

## 5. O que falta

Implementação de CI, imagem, ambiente, monitoramento, segredos e deploy; `RF-OPS` numerados (feito nesta
rodada); roadmap com DoD (pendente); `wyd-ops` não tinha dashboard próprio até esta rodada.

## 6. Gate para sair do papel

1. `RF-OPS` numerados com critério verificável (feito).
2. Árvore de `src/` e `tests/` planejada (feito).
3. Plano de teste com alvos, dado, isolamento e comando (feito).
4. Gate verde: `docker compose --profile gate run --rm gate`.
5. Fronteira e contratos aprovados por arquitetura.

## 7. Próximas páginas

`docs/arquitetura/fronteira.md`, `docs/specs/` com `RF-OPS`, `docs/planos/plano-de-testes.md` (feito)
e `docs/roadmap/` com DoD.