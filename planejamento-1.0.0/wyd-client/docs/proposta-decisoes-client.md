# Decisões do `wyd-client` — patch e aprovação da captura

> Pergunta: "o que o patch entrega e quem aprova a segunda leitura da captura?"
> Público: frente `wyd-client`, `wyd-protocol`, QA e `wyd-launcher`
> Dono: equipe do `wyd-client`
> Base: `lacunas.md` (`LCLI-01`–`LCLI-05`); `registro-do-cliente.md`; `docs/index.md`;
> `wyd-1.0.0.md` §7.1, §7.3 (X11, X12) e §7.6; `AGENTS.md` §5, itens 6 e 7, e §8
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). `Q-CLI-3` foi respondida (= A) e o nome do QA foi definido em
2026-11-23; `Q-CLI-2` continua aberta. `LCLI-01` é a mesma decisão de `LPRT-09` e não é duplicada;
`LCLI-04` já foi decidida. As políticas de `Q-IP-3`–`Q-IP-6` e o pedido externo de `Q-IP-1`/`Q-IP-2`
estão registrados na página própria; o inventário item a item e o pacote do pedido existem, mas o pedido
não foi enviado; `LCLI-05` continua `proposta`. Nada foi aplicado. O hash e o tamanho citados são
**medições** do registro e do inventário; nenhum outro número é medição.

## 1. Fronteira

**Fecha:** `LCLI-02` (conteúdo e versionamento do patch) e `LCLI-03` (aprovador da segunda leitura;
nome definido em 2026-11-23 e local do registro já fixado por `LPRT-10`, falta a primeira ficha
assinada).
**Não duplica:** `LCLI-01` (build-alvo e identidade) é `LPRT-09` vista do lado do cliente; a pergunta
`Q-PRT-1` é o dono único.

**Não decide:** a ferramenta de captura (`LCLI-04`, decidida), a titularidade e a distribuição
(`LCLI-05`, página própria), o layout do fio (`wyd-protocol`), o manifesto (`wyd-launcher`) e a
hospedagem (`wyd-ops`).

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| lacunas do sistema | **5**: 2 abertas, 1 proposta (`LCLI-05`), 2 fechadas (`LCLI-04`, `LCLI-01`) | `lacunas.md` §1 |
| build-alvo | **decidido em 2026-11-23** (`Q-PRT-1`/`LPRT-09`): cliente 7662, versão + SHA-256 | `registro-do-cliente.md` §4 |
| tamanho de `WYD.exe` | **2.347.008 bytes** | `registro-do-cliente.md` |
| hash de `WYD.exe` | SHA-256 completo medido em 2026-11-22; promovido a identidade do alvo em 2026-11-23 | idem |
| arquivos do cliente versionados no git | **0** — só `wyd-client/.gitkeep` | `registro-do-cliente.md` §3 |
| capturas existentes | **0** | `docs/index.md` |
| código do cliente | **0** | `lacunas.md`; `AGENTS.md` §5, item 6 |

## 3. Já decidido — não reabrir

| Item | Decisão vigente | Fonte | Efeito |
|---|---|---|---|
| ferramenta de captura | **opção A**: ferramenta específica em `wyd-client/tools/`, ainda não criada | `lacunas.md` `LCLI-04` | não se pergunta de novo |
| processo de captura | bytes + ficha + anonimização + segunda leitura, com registro em `tests/golden/` | `LPRT-10`; `goldens/como-capturar.md` | o local está fixado; `LCLI-03` tem nome definido e espera a primeira ficha |
| dado de jogador | captura só sintética, sem produção/terceiros | `LCLI-04`; `LPRT-10` | nenhuma opção pode usar conta real |

## 4. Decisões

### Q-CLI-2 — O que o patch entrega e como versiona?

**Medido.** Não há lista de arquivos do patch nem versão de patch; o `wyd-launcher` depende do
manifesto (X11). `LCLI-05` segue `proposta` e mantém a autorização de conteúdo em aberto. O registro
lista `WYD.exe`, mas não decide o que é distribuível.

**O que está em jogo.** "Cópia da árvore inteira" aumenta o patch e o risco jurídico; um conjunto
mínimo é verificável, mas precisa de lista e de versão.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Pacote mínimo declarado: `strdef.bin`, configurações e o executável **se liberados**, com versão de patch e lista por arquivo | patch pequeno e verificável; depende de `LCLI-05` para cada item |
| B | Árvore inteira do cliente | simples de montar; grande, difícil de versionar e inclui material não liberado |
| C | Só o executável | mínimo; não cobre `strdef.bin`/config e pode não rodar |

**Recomendação: A.** O manifesto precisa de lista e hash por arquivo (X11); a lista mínima é a única
que pode ser provada item a item contra `LCLI-05`. **Custo:** manter a lista e a versão. **Fontes:**
`lacunas.md` `LCLI-02`; `wyd-1.0.0.md` §7.3 X11; `LCLI-05`; `registro-do-cliente.md`. **Recusa B** por
risco e tamanho; **C** por não cobrir o patch.

### Q-CLI-3 — Quem aprova a segunda leitura da captura?

**Medido.** `LPRT-10` já fixa o registro em `tests/golden/`, a ficha e a segunda leitura; o **aprovador**
da segunda leitura foi nomeado em 2026-11-23 (`LCLI-03`). Não há captura nem ficha preenchida.

**O que está em jogo.** Aprovador anônimo ou acumulado na mesma frente que captura reduz a
independência da segunda leitura; nomear uma pessoa ou uma dupla fecha a cadeia.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Pessoa de QA **ou do `wyd-protocol`**, independente de quem capturou, assina a ficha e a entrada do catálogo | segunda leitura independente e rastreável; exige disponibilidade e registro do nome |
| B | O dono do `wyd-client` aprova cada captura | autoridade centralizada; a mesma frente captura e aprova, então a independência é menor |
| C | Dupla revisão: QA + `wyd-protocol` assinam | mais forte para o fio; custa duas pessoas por captura |

**Recomendação: A.** `LPRT-10` exige uma segunda leitura por outra pessoa; nomear alguém de QA,
independente da captura, preserva essa separação sem transformar o dono do artefato em revisor único.
**Custo:** nomear a pessoa e manter a ficha. **Fontes:** `lacunas.md` `LCLI-03`; `LPRT-10`;
`wyd-protocol/docs/goldens/como-capturar.md` §7; `LCLI-04` D6. **Recusa B** por reduzir a independência;
**C** por custo duplo sem exigência da fonte.

**Resposta do dono em 2026-11-23: A — QA designado assina; dono do `wyd-protocol` é suplente.** O QA
designado é **Lucas Cantarelli** (dono do produto), que assumiu a função na mesma data e **não é o
operador**; `LCLI-03` continua aberta até a primeira ficha assinada, com o nome do operador. O local já está fixado: frame e ficha em
`tests/golden/`, entrada pública no catálogo do `wyd-protocol`
(`../../wyd-protocol/docs/goldens/catalogo.md`), bruto fora do repositório (D5).

### `LCLI-05` — página própria; decisões refletidas aqui

A titularidade e a distribuição têm seis perguntas em
`docs/specs/propriedade-intelectual-e-distribuicao.md` (`Q-IP-1`–`Q-IP-6`). Esta página não as repete:
em 2026-11-23 o dono do produto assumiu a responsabilidade nominal por direitos (**Lucas Cantarelli**) e
congelou a distribuição. As políticas de `Q-IP-3`–`Q-IP-6` foram decididas e o pedido de `Q-IP-1`/`Q-IP-2`
foi iniciado; **`LCLI-05` continua `proposta`** até a evidência do titular. O registro completo está em
`docs/specs/propriedade-intelectual-e-distribuicao.md` §8.

### Q-IP-1–Q-IP-6 — decisões de 2026-11-23

**Decisor:** Lucas Cantarelli (dono do produto).

| ID | Estado em 2026-11-23 |
|---|---|
| `Q-IP-1` | pedido de proveniência iniciado; inventário técnico em paralelo; sem resposta do titular |
| `Q-IP-2` | pedido de autorização escrita iniciado; **escopo B = árvore inteira** (7.147 arquivos/567 MB, incluindo assets e screenshots) decidido em 2026-11-23; inventário e pacote prontos, canal de suporte decidido, envio pelo dono ainda não realizado; sem resposta do titular |
| `Q-IP-3` | **A** — só hash/metadado/referência no repositório público |
| `Q-IP-4` | **A** — GPL v2 no código original; ativos/configurações do cliente excluídos |
| `Q-IP-5` | **A** — nenhum logo/arte/screenshot do cliente até autorização escrita |
| `Q-IP-6` | **A** — release bloqueada sem registro por item e aprovação do dono |

**Q-IP-2 — escopo B (árvore inteira), 2026-11-23.** O pedido ao titular passa a cobrir a árvore inteira
do cliente, incluindo assets e screenshots; custo estimado de 1–2 sessões e revisão jurídica ampla
(semanas). O inventário está em [`specs/inventario-da-arvore.md`](specs/inventario-da-arvore.md) e o
pacote em [`specs/pedido-ao-titular.md`](specs/pedido-ao-titular.md); o pedido não foi enviado. Nada é
liberado até a resposta e o registro por item; `Q-IP-3`, `Q-IP-5` e `LCLI-05` continuam valendo.

Nenhuma dessas decisões fecha lacuna: `LCLI-05` segue `proposta`; `LCLI-02` foi executada e fechada em
2026-11-25 (`registro-do-cliente.md` §5.1). O detalhe do que
cada uma autoriza e não autoriza está na página própria, §8.

## 5. Cobertura das lacunas

| Lacuna | Pergunta | Situação |
|---|---|---|
| `LCLI-01` | — | fechada em 2026-11-23 pela decisão de `LPRT-09`/`Q-PRT-1` (cliente 7662 + SHA-256); não duplicada aqui |
| `LCLI-02` | `Q-CLI-2` | fechada (2026-11-25, executada) |
| `LCLI-03` | `Q-CLI-3` | aberta; papel A decidido e QA definido (**Lucas Cantarelli**, dono do produto, que não é o operador); falta a primeira ficha assinada com o nome do operador; local fixado por `LPRT-10` |
| `LCLI-04` | — | fechada em 2026-11-23 (opção A); D3–D6 decididos; implementação pendente |
| `LCLI-05` | `Q-IP-1`–`Q-IP-6` | **proposta**; responsável nominal e distribuição congelada; `Q-IP-3`–`Q-IP-6` decididas, `Q-IP-1`/`Q-IP-2` com pedido externo iniciado e sem evidência; `LCLI-02` fechada em 2026-11-25 |

## 6. Registro da decisão

**Parcialmente respondida em 2026-11-23.** `Q-CLI-3` escolheu A (QA designado; dono do `wyd-protocol`
suplente) e o nome foi definido: **Lucas Cantarelli** (dono do produto, que não é o operador; o operador
entra na ficha). `LCLI-03` continua `aberta` até a primeira ficha assinada; a captura pode ser executada,
não aceita. `Q-IP-3`–`Q-IP-6` foram decididas e o pedido de `Q-IP-1`/`Q-IP-2` foi iniciado, mas `LCLI-05`
segue **proposta** até a evidência do titular; `LCLI-02` fechou em 2026-11-25 pela opção A executada; `LCLI-01` fechou em 2026-11-23 com
`LPRT-09` (`Q-PRT-1`, opção A).

| Pergunta | Lacuna | Resposta |
|---|---|---|
| `Q-CLI-2` | `LCLI-02` | A — dados + launcher próprio, executado em 2026-11-25 |
| `Q-CLI-3` | `LCLI-03` | A — QA designado; suplente `wyd-protocol`; **Lucas Cantarelli** (dono do produto) assina |
| `Q-IP-1`–`Q-IP-6` | `LCLI-05` | `Q-IP-3`–`Q-IP-6` = A (política); `Q-IP-1`/`Q-IP-2` = pedido externo iniciado, sem resposta; `LCLI-05` segue proposta; `LCLI-02` fechada em 2026-11-25 |
| A1 | — (não é lacuna) | A — peer da opção A + `ECAP` + CLI + captura, autorizada em 2026-11-23; implementação e captura pendentes |

**A1 autorizada em 2026-11-23.** A resposta A não reabre `LCLI-02`/`LCLI-03`/`LCLI-05` e não fecha
`LPRT-03` ou `LPRT-06`: ela autoriza o peer da opção A, o `ECAP` e a CLI, e a execução da captura A1.
`LPRT-03` só fecha após a segunda leitura; `LPRT-06` continua nas capturas B/C com seed sintético.
