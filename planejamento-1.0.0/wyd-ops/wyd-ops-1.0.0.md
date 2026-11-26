# wyd-ops 1.0.0 — abertura do planejamento

> Pergunta: "o que o `wyd-ops` faz, quais contratos ele tem com os outros sistemas e o que falta para começar o código?"
> Público: Produto, arquitetura e equipe do `wyd-ops`
> Dono: equipe do `wyd-ops`
> Base: `wyd-ops/docs/index.md`; `AGENTS.md` §6; `wyd-portal/docs/specs/requisitos.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Abertura inicial: fronteira, contratos e lacunas nomeadas.
**Nenhum código próprio**; requisitos, plano de teste e roadmap foram abertos em 2026-11-25 e o registro de lacunas vive em `docs/lacunas.md`.

**Organização (D-04 = A): fechada em 2026-11-25.** A auditoria (`wyd-ops/auditoria-de-organizacao.md`) listava para este sistema: sem dashboard próprio, sem `RF-OPS`, sem plano de teste e roadmap com DoD, fronteira/contratos parciais. Fechado em 2026-11-25 com `src/.gitkeep` + `tests/.gitkeep`, `docs/arquitetura/componentes.md`, `docs/planos/plano-de-testes.md`, `docs/specs/requisitos.md` (7 `RF-OPS` + 5 `RNF-OPS`) e `docs/roadmap/` (index.md + roadmap.md com DoD por fatia). A fronteira e os contratos O1–O5 estão aprovados (O4/O5 formalizados em 2026-11-26); a implementação de CI/imagem/ambiente/deploy segue pendente.

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
| CDN e domínio próprio | fora do escopo da `1.0.0`; a VPS paga é a hospedagem de produção por `ADR-OPS-0002` |

## 3. Contratos

| # | Contrato | Direção | Forma | Garantia | Dono | Estado |
|---|---|---|---|---|---|---|
| O1 | ambiente efêmero | ops → `wyd-bots` | Postgres + servidor descartáveis por execução | sem estado global entre cenários | `wyd-ops` | **aprovado em 2026-11-25**; contrato nesta seção; execução em F0/F1 e `LBOT-03` |
| O2 | métricas e logs | ops ← `wyd-server` | `/metrics` Prometheus + logs de metadados | sem payload/PII | `wyd-ops` | **aprovado em 2026-11-25**; `RF-OPS-004` implementável na F3 |
| O3 | publicação do site | ops → Pages | workflow + artefato estático | rollback validado | `wyd-ops` | **aprovado em 2026-11-25**; rollback ensaiado exigido antes de publicar |
| O4 | segredos | ops emite | secret store; nunca em código | rotação documentada | `wyd-ops` | **aprovado em 2026-11-25**; inventário e rotação em `ADR-OPS-0011`; aplicação depende de `OPS-04` |
| O5 | deploy de servidor e launcher | ops → host/release | workflow + imagem + artefato | rollback validado e aprovação | `wyd-ops` | **aprovado em 2026-11-25**; contrato no `ADR-OPS-0007` revisado; implementação em F4/`OPS-04` |

## 4. Estado medido (`V`, 2026-11-26)

| Medida | Valor | Comando |
|---|---:|---|
| Páginas de planejamento | 35 (2026-11-25) | `find planejamento-1.0.0/wyd-ops -name '*.md'` |
| Arquivos de código | 0 | `find crates/wyd-ops -type f` (só `.gitkeep`) |
| Requisitos com ID | 7 `RF-OPS` + 5 `RNF-OPS` | esta rodada |
| Issues abertas no GitHub | 121 no repositório; 7 do `wyd-ops` | leitura de 2026-11-26; board de 121 itens |

## 5. O que falta

Implementação de CI, imagem, ambiente, monitoramento, segredos e deploy; aplicação de `Q-OPS-2`
(provedor, VPS, contrato, backup) e do inventário de segredos no ambiente de produção. Os contratos
O1–O5 estão aprovados e formalizados (`ADR-OPS-0011` e `ADR-OPS-0007` revisado); o que falta é
execução, não aprovação. O planejamento de organização está fechado: dashboard, `RF-OPS`, plano de
teste e roadmap existem. O registro vive em `docs/lacunas.md` §4.

## 6. Gate para sair do papel

1. `RF-OPS` numerados com critério verificável (feito).
2. Árvore de `src/` e `tests/` planejada (feito).
3. Plano de teste com alvos, dado, isolamento e comando (feito).
4. Gate verde: `docker compose --profile gate run --rm gate`.
5. Fronteira e contratos O1–O5 aprovados (feito em 2026-11-25; O4/O5 formalizados em 2026-11-26);
   falta a implementação de F3/F4, dependente de `OPS-04`.

## 7. Próximas páginas

`OPS-03` fechou em 2026-11-26: O1–O5 estão aprovados, e O4/O5 foram formalizados em `ADR-OPS-0011` e
no `ADR-OPS-0007` revisado. As lacunas abertas são `OPS-04` (aplicação de `Q-OPS-2`, adiada pelo dono)
e `OPS-05` (fechada em 2026-11-26: 121 issues e board de 121 itens). O registro é `docs/lacunas.md` §4. Não há
página de organização pendente.