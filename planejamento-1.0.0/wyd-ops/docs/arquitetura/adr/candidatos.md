# Candidatos a ADR — backlog de decisões

> Pergunta: "quais decisões futuras podem virar ADR, quando viram e quem precisa aprovar?"
> Público: Arquitetura, desenvolvimento e revisores
> Dono: arquitetura (com o dono do sistema afetado)
> Base: `AGENTS.md` §3 e §5 (invariante 3); `wyd-ops/docs/arquitetura/adr/index.md`; `wyd-ops/docs/arquitetura/adr/template.md`; `wyd-ops/docs/arquitetura/decisoes-registradas.md`; `wyd-ops/docs/prioridade-do-projeto.md`; `wyd-ops/auditoria-de-organizacao.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Backlog de planejamento; nenhum ADR foi criado ou alterado por este documento.

## 1. Regra

ADR só existe quando uma decisão foi **tomada e aprovada pelo dono** (AGENTS.md §5, item 3). Este documento não cria ADR, não numera decisão futura e não transforma prática do legado ou hipótese em decisão. Ele lista gatilhos e evidências necessárias para que uma decisão futura possa virar ADR.

## 2. Candidatos futuros

| Gatilho | Sistema | Decisão candidata | Quando vira ADR | Evidência mínima |
|---|---|---|---|---|
| `D-06` respondido | projeto | frente de execução depois de `D-05` | após escolha do dono | resposta registrada em `prioridade-do-projeto.md` |
| `Q-SERIE-1` respondido | `wyd-db` | política de `down` dos 28 arquivos | após escolha do dono | política registrada e testada |
| `Q-SEP-1` respondido | `wyd-db` | separador de nomes de migration | após escolha do dono | regra de nomenclatura registrada |
| primeira execução real de carga | `wyd-bots` | limites medidos (`LBOT-03`) | após medição real | `p95` e taxa de erro registrados |
| primeira captura do cliente 7662 | `wyd-client`/`wyd-protocol` | protocolo e árvore do cliente | após captura validada | golden e inventário conferidos |
| `OPS-03.4` aprovado | `wyd-ops` | inventário, armazenamento e rotação de segredos | após o dono aprovar O4 | inventário fechado; rotação e consumo provados |
| `OPS-03.5` aprovado | `wyd-ops` + produto | contrato de deploy/rollback de servidor e launcher | após o dono aprovar O5 | staging, gatilho, aprovação e rollback ensaiado |
| primeiro deploy real | `wyd-ops` | política de deploy/rollback | após staging verde | rollback validado |
| primeiro sistema com gate fechado | sistema dono | cobertura bloqueante por sistema | após declaração do gate | evidência de cobertura e capacidade |
| mudança de fronteira entre sistemas | sistema afetado + arquitetura | contrato ou fronteira | após aprovação do dono | ADR com contexto, decisão, consequências, alternativas e como verificar |

## 3. O que não é candidato

- Prática do legado sem reverificação.
- Hipótese sem ID de lacuna.
- Decisão de nomenclatura ou estilo sem impacto estrutural.
- Regra de negócio que pertence a `specs/` e não muda schema, fronteira ou operação.
- Métrica ou contagem que ainda não foi medida.

## 4. Processo para virar ADR

1. A decisão aparece como `proposta` em `lacunas.md` ou documento dono.
2. O dono do sistema e a arquitetura aprovam ou rejeitam.
3. Se aprovada, criar `planejamento-1.0.0/<sistema>/docs/arquitetura/adr/adr-<sistema>-NNNN-<slug>.md` usando `template.md`.
4. Atualizar `adr/index.md` e `decisoes-registradas.md`.
5. Fechar a lacuna correspondente em `lacunas.md` com data e evidência.
6. Registrar no gate que o ADR existe e tem decisão aprovada.

## 5. Cruzamento com auditoria e especificação

| Referência | Relação |
|---|---|
| `wyd-ops/auditoria-de-organizacao.md` §2–§3 | 13 sistemas organizados; `wyd-ops` mantém fronteira/contratos parciais; este backlog não fecha organização por si só |
| `wyd-ops/docs/arquitetura/adr/index.md` | índice e regra de ouro dos ADRs |
| `wyd-ops/docs/arquitetura/adr/template.md` | molde para futuros ADRs |
| `wyd-ops/docs/arquitetura/decisoes-registradas.md` | registro consolidado de decisões |
| `wyd-ops/docs/prioridade-do-projeto.md` | decisões de foco e execução (`D-06`, `Q-SERIE-1`, `Q-SEP-1`) |
| `wyd-ops/docs/specs/requisitos.md` | requisitos que podem gerar decisões futuras |

## 6. O que esta página não cobre

- Decisões já aprovadas e registradas em `decisoes-registradas.md`.
- ADR formal para decisão não aprovada.
- Regra de negócio, schema ou catálogo: vivem em `specs/` e no dicionário.