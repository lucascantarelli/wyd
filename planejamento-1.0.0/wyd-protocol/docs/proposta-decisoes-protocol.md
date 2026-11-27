# Decisões do `wyd-protocol` — build-alvo

> Pergunta: "qual build é o alvo?"
> Público: pesquisa, arquitetura, `wyd-protocol`, `wyd-client` e QA
> Dono: equipe do `wyd-protocol`
> Base: `lacunas.md` (`LPRT-03`, `LPRT-06`, `LPRT-09`, `LPRT-10`); `docs/convencoes.md`;
> `docs/goldens/como-capturar.md`; `wyd-client/registro-do-cliente.md`;
> `wyd-1.0.0.md` §7.1, §7.3 (X12) e §7.6; `AGENTS.md` §5, item 8 e §8
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). **Uma pergunta de decisão** (`LPRT-09`), **respondida em 2026-11-23
na opção A**: build-alvo é o cliente **7662**, identificado por versão e SHA-256 completos. `LPRT-03` e
`LPRT-06` seguem medição/execução, cobertas pelo procedimento de `goldens/como-capturar.md`. Nada além do
registro foi aplicado: não há captura, golden nem código.

## 1. Fronteira

**Fecha:** build-alvo (`LPRT-09`), decidido em 2026-11-23. **Não fecha:** `LPRT-03` (origem da
`KeyWord`) e `LPRT-06` (encoding/terminador), que são medição e execução do procedimento, não escolha de
desenho.

**Não decide:** o processo de captura (`LPRT-10`, decidido), a ferramenta (`LCLI-04`, decidida), o
socket (`wyd-net`), a regra (`wyd-core`/`wyd-sim`) nem o manifesto (`wyd-launcher`).

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| lacunas abertas | **2** (`LPRT-03`, `LPRT-06`), ambas de medição; nenhuma pergunta de desenho | `lacunas.md` §1 |
| capturas existentes | **0** | `docs/index.md`; `goldens/como-capturar.md` |
| goldens escritos | **1** — o procedimento, não um frame | `docs/index.md` |
| build-alvo | **decidido em 2026-11-23: cliente 7662**, identificado por versão e SHA-256 completos | §4 `Q-PRT-1`; `lacunas.md` `LPRT-09` |
| hash de `WYD.exe` | SHA-256 completo medido em 2026-11-22 | `wyd-client/registro-do-cliente.md` §4 |
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

**Medido.** O repositório fala em cliente 7662; `WYD.exe` tem 2.347.008 bytes e **SHA-256 completo**
medido em `wyd-client/registro-do-cliente.md` §4; a decisão de 2026-11-23 promove esse build de candidato
a alvo, e `LPRT-09` fecha nela.

**O que está em jogo.** Sem build fixo, offset e opcode ficam ambíguos; "o mais recente" não tem prova
de layout; esperar a captura mantém o fio parado.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Fixar 7662 + identidade completa (versão + hash) como alvo; qualquer outro build é não suportado | offset/golden ganham alvo; exige aceitar um build — o hash já está medido |
| B | Usar o build mais recente encontrado | pode refletir o cliente atual; sem captura, o layout não está provado |
| C | Não fixar até a captura | mantém a pergunta aberta; o fio continua sem alvo |

**Recomendação: A.** É o único build com medição no registro; o hash foi o candidato e esta decisão o
promoveu a identidade do alvo.
**Custo:** um build suportado. **Fontes:** `lacunas.md` `LPRT-09`; `wyd-client/registro-do-cliente.md`. **Recusa B** por falta de prova; **C** por bloquear.

**Por que agora.** `LPRT-09` não é lacuna de captura: é pergunta de decisão, e a campanha a tratou como
paralela a `Q-SRV-5` quando a dependência mostra o contrário. A ficha de toda captura identifica a build
usada (`goldens/como-capturar.md` §5), a opção C daquele peer acrescenta um segundo binário cuja versão
precisa ser pareada com o alvo do cliente, e `LNET-02` declara depender deste ID. A decisão não impede
gravar os bytes de A1; impede declarar A1 conclusiva. Decidir antes também evita capturar contra um alvo
que a ficha ainda não pode nomear.

**Resposta do dono em 2026-11-23: A.** O build-alvo é o **cliente 7662**, identificado pela versão e pelo
**SHA-256 completo** já medido em `wyd-client/registro-do-cliente.md` §4; **qualquer outro build é não
suportado**. Consequências: offset, opcode e golden passam a ter alvo; o `client_version` de
`game.session` ganha referência; `LNET-02` fica decidível; e a ficha de toda captura passa a nomear o
alvo. **O que continua não medido:** o que muda em relação a um build anterior — a comparação é leitura
datada (§5 de `goldens/como-capturar.md`), marcada `H`, e não uma decisão pendente. `LPRT-09` fechada
como decisão de alvo.

## 5. Cobertura das lacunas

| Lacuna | Pergunta | Situação |
|---|---|---|
| `LPRT-09` | `Q-PRT-1` | fechada (A, 2026-11-23): alvo 7662 + SHA-256 completo; outro build não suportado |
| `LPRT-03` | — | aberta no registro; medição/execução. O crate já recebe a `KeyWord` do chamador e não assume origem; falta capturar o handshake, não escolher desenho. Procedimento em `goldens/como-capturar.md` §3. |
| `LPRT-06` | — | aberta no registro; medição/execução. O codec já não transcodifica e a string é bytes de tamanho fixo; falta medir encoding/terminador. Procedimento em `goldens/como-capturar.md` §4. |

## 6. Registro da decisão

**Respondida em 2026-11-23.** `LPRT-09` fechou na opção A; `LPRT-03` e `LPRT-06` continuam medição
pendente, sem pergunta de desenho. Nada além do registro foi aplicado. A decisão antecede a
conclusividade de A1 — é o que permite à ficha nomear a build — e remove o bloqueio de build que
`LNET-02` citava; não decide se a versão é negociada ou imposta — essa política foi decidida depois em
`Q-NET-2` = A (2026-11-23): a sessão impõe o build no handshake —, nem substitui a leitura que confirma a
versão declarada. **A1 foi autorizada em 2026-11-23 (resposta A):** peer da opção A, `ECAP` e CLI
autorizados; a captura não foi executada; `LPRT-03` só fecha após a segunda leitura e `LPRT-06` continua
nas capturas B/C.

| Pergunta | Lacuna | Resposta |
|---|---|---|
| `Q-PRT-1` | `LPRT-09` | A — build-alvo 7662 com versão e SHA-256 completos; outro build não suportado |
