# Decisões do `wyd-protocol` — build-alvo

> Pergunta: "qual build é o alvo?"
> Público: pesquisa, arquitetura, `wyd-protocol`, `wyd-client` e QA
> Dono: equipe do `wyd-protocol`
> Base: `lacunas.md` (`LPRT-03`, `LPRT-06`, `LPRT-09`, `LPRT-10`); `docs/convencoes.md`;
> `docs/goldens/como-capturar.md`; `wyd-client/registro-do-cliente.md`;
> `wyd-1.0.0.md` §7.1, §7.3 (X12) e §7.6; `AGENTS.md` §5.8 e §8
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). **Uma pergunta de decisão** (`LPRT-09`); `LPRT-03` e `LPRT-06` são
medição/execução e não escolha de desenho, cobertas pelo procedimento de `goldens/como-capturar.md`.
Nada foi aplicado. **Nenhum número desta página é medição**: não há captura nem build decidido; o hash
citado é o candidato medido no cliente.

## 1. Fronteira

**Fecha:** build-alvo (`LPRT-09`). **Não fecha:** `LPRT-03` (origem da `KeyWord`) e `LPRT-06`
(encoding/terminador), que são medição e execução do procedimento, não escolha de desenho.

**Não decide:** o processo de captura (`LPRT-10`, decidido), a ferramenta (`LCLI-04`, decidida), o
socket (`wyd-net`), a regra (`wyd-core`/`wyd-sim`) nem o manifesto (`wyd-launcher`).

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| lacunas abertas | **3** (`LPRT-03`, `LPRT-06`, `LPRT-09`); só `LPRT-09` é pergunta de desenho | `lacunas.md` §1 |
| capturas existentes | **0** | `docs/index.md`; `goldens/como-capturar.md` |
| goldens escritos | **1** — o procedimento, não um frame | `docs/index.md` |
| build-alvo | **não decidido**; `WYD.exe` é candidato | `wyd-client/registro-do-cliente.md` |
| hash de `WYD.exe` | prefixo medido `04c9d2a1…` | idem |
| processo de captura | **decidido** (`LPRT-10`): bytes + ficha + anonimização + 2ª leitura | `lacunas.md`; `goldens/como-capturar.md` |

## 3. Já decidido — não reabrir

| Item | Decisão vigente | Fonte | Efeito |
|---|---|---|---|
| processo de captura | `LPRT-10`: bytes, ficha, anonimização e segunda leitura | `lacunas.md`; `goldens/como-capturar.md` | `LPRT-03`/`06`/`09` usam esse processo, não o rediscutem |
| codec | `encode`/`decode` com erro tipado, sem I/O | `LPRT-18`; X1 | não é decisão desta rodada |
| erro de codec | reação é da conexão | `LPRT-13` | idem |
| `Type` desconhecido | `UnknownType` tipado | `LPRT-08` | idem |
| ordem por conexão | entrega serializada | `LPRT-12`; X2 | idem |

## 4. Decisão

### Q-PRT-1 — Qual build é o alvo?

**Medido.** O repositório fala em cliente 7662; `WYD.exe` tem 2.347.008 bytes e hash com prefixo
`04c9d2a1…`; `LPRT-09` segue aberta e o registro chama o hash de candidato.

**O que está em jogo.** Sem build fixo, offset e opcode ficam ambíguos; "o mais recente" não tem prova
de layout; esperar a captura mantém o fio parado.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Fixar 7662 + identidade completa (versão + hash) como alvo; qualquer outro build é não suportado | offset/golden ganham alvo; exige completar o hash e aceitar um build |
| B | Usar o build mais recente encontrado | pode refletir o cliente atual; sem captura, o layout não está provado |
| C | Não fixar até a captura | mantém a pergunta aberta; o fio continua sem alvo |

**Recomendação: A.** É o build com medição no registro; o hash é o candidato e a decisão o promove.
**Custo:** um build suportado. **Fontes:** `lacunas.md` `LPRT-09`; `wyd-client/registro-do-cliente.md`. **Recusa B** por falta de prova; **C** por bloquear.

## 5. Cobertura das lacunas

| Lacuna | Pergunta | Situação |
|---|---|---|
| `LPRT-09` | `Q-PRT-1` | decisão de build-alvo; aberta |
| `LPRT-03` | — | aberta no registro; medição/execução. O crate já recebe a `KeyWord` do chamador e não assume origem; falta capturar o handshake, não escolher desenho. Procedimento em `goldens/como-capturar.md` §3. |
| `LPRT-06` | — | aberta no registro; medição/execução. O codec já não transcodifica e a string é bytes de tamanho fixo; falta medir encoding/terminador. Procedimento em `goldens/como-capturar.md` §4. |

## 6. Registro da decisão

**Aguardando resposta do dono.** `LPRT-09` continua `aberta`; `LPRT-03` e `LPRT-06` continuam medição
pendente, sem pergunta de desenho. Nada foi aplicado.

| Pergunta | Lacuna | Resposta |
|---|---|---|
| `Q-PRT-1` | `LPRT-09` | — |

