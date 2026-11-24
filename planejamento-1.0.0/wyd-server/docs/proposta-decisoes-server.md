# Decisões do `wyd-server` — composição, canais, prontidão, encerramento e peer de captura

> Pergunta: "como o processo é composto, quantos canais a 1.0.0 abre, o que `ready` significa e quem completa o handshake de A1?"
> Público: arquitetura, produto, `wyd-server`, `wyd-client`, `wyd-protocol`, `wyd-ops` e `wyd-db`
> Dono: equipe do `wyd-server`
> Base: `lacunas.md` (`LSRV-01`–`LSRV-05`); `docs/index.md`; `wyd-server-1.0.0.md` §1–§6;
> `docs/planos/esqueleto-de-ponta-a-ponta.md`; `docs/specs/endpoint-de-captura.md`;
> `wyd-1.0.0.md` §7.1, §7.3 (X7) e §7.6;
> `AGENTS.md` §5, item 8 e §8
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). **Cinco perguntas respondidas em 2026-11-23**: `Q-SRV-1`–`Q-SRV-4`
na opção A e `Q-SRV-5` na opção C, com quarentena do servidor legado. O encerramento já é exigido
pelo gate (`SIGTERM` drena e persiste) e o sinal/tempo pertence ao ambiente, não a uma pergunta de
desenho. Nada foi aceito editorialmente. Não há código, endpoint nem teste; qualquer número de canais
é decisão, não medição.

## 1. Fronteira

**Fecha:** composição `net` + `core` + dados, número de canais, significado de `ready`, encerramento e o
peer que completa o handshake em A1 — `LSRV-01` a `LSRV-05`. **Não fecha:** bytes, offsets de payload,
encoding (`LPRT-03`, `LPRT-06`), que dependem da captura — o build-alvo foi decidido em `LPRT-09`
(2026-11-23); sinal, tempo de graça e kill continuam fora do escopo operacional (`WYDDB-021`).

**Não decide:** fio (`wyd-protocol`), socket/limite (`wyd-net`), regra (`wyd-core`/`wyd-sim`),
SQL (`wyd-db`), deploy (`wyd-ops`).

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| lacunas do sistema | **0 abertas, 0 propostas, 5 fechadas** | `lacunas.md` §1 |
| arquivos de código | **0** (só `.gitkeep`) | `wyd-server-1.0.0.md` §4 |
| endpoints implementados | **0** | idem |
| endpoint de captura | **especificado, não implementado** | `docs/specs/endpoint-de-captura.md` |
| peer de A1 | **decidido em C, condição reprovada**: A1 depende de A (`LSRV-05`, 2026-11-23) | §4 `Q-SRV-5`; spec §9.1 |
| canais abertos | **0** | idem |
| controle administrativo | **decidido** (`LAC-ADM-03`) | `wyd-server-1.0.0.md` §3 S1/S2 |
| métricas para o NOC | **decidido** (`LAC-POR-06`) | §3 S4 |
| teste de dois canais | alvo, nenhum teste | `wyd-1.0.0.md` §7.6 |

## 3. Já decidido — não reabrir

| Item | Decisão vigente | Fonte | Efeito |
|---|---|---|---|
| controle administrativo | HTTP síncrono com `200 {delivered_to}`; sem ack não há escrita | `LAC-ADM-03`; X8 | não é decisão desta rodada |
| autenticação do elo | segredo próprio `WYD_SERVER_CONTROL_TOKEN` | `LAC-ADM-03` §3.3 | idem |
| métricas | `/metrics` Prometheus, coleta `wyd-ops` | `LAC-POR-06`; X10 | idem |
| mapa canal → URL | `WYD_SERVER_CONTROL_URLS` | `LAC-ADM-03` §3.3 | idem |

**Especificado, não implementado:** o endpoint de captura `ECAP` em loopback, separado de `EPROD`, está
em [`specs/endpoint-de-captura.md`](specs/endpoint-de-captura.md); o peer de A1 foi decidido em
`Q-SRV-5`, mas o `ECAP` e a CLI continuam sem código.

## 4. Decisões

### Q-SRV-1 — Como o processo é composto?

**Medido.** Não há composição `net` + `core` + dados; `X7` estava em lacuna até a resposta e o plano
exige "uma porta por canal, sem estado global entre canais". O esqueleto de ponta a ponta é proposta,
não decisão.

**O que está em jogo.** Um binário com um processo por canal é simples de operar; binários separados
isolam, mas duplicam infra. A opção multi-canal já foi descartada: contraria X7, que fixa uma porta
por canal.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Um binário `wyd-server`, com um processo por canal, compondo `wyd-net` + `wyd-core` + adapter do `wyd-db` | isolamento por processo e topologia explícita; um processo por canal |
| B | Binários separados para net, core e dados | isolamento máximo; três deploys e contratos de rede internos |

**Recomendação: A.** `wyd-1.0.0.md` §7.1 já diz "um processo por canal" e §7.3 X7 pede uma porta por
canal; A mantém esse contrato com um binário e um processo por canal. **Custo:** um processo por canal.
**Fontes:** `lacunas.md` `LSRV-01`; `wyd-1.0.0.md` §7.1 e X7; `wyd-server-1.0.0.md` §1 e §3 S5.
**Recusa B** por complexidade; a opção multi-canal foi retirada por contrariar X7.

**Resposta do dono em 2026-11-23: A.** `LSRV-01` fechada.

### Q-SRV-2 — Quantos canais a 1.0.0 abre?

**Medido.** Nenhum canal foi aberto; o plano não afirma número e o teste de dois canais é alvo. O
número era **decisão proposta**, não medição; a resposta fixa 2.

**O que está em jogo.** Um canal não prova isolamento; dois provam o teste mínimo; "N depois da
carga" adia a topologia e mantém o teste inexistente.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Dois canais na 1.0.0, com portas e URLs distintas, escolha por lista no launcher/login; a primeira fatia usa um | teste de dois canais possível; custo de dois processos e configuração |
| B | Um canal | menor custo; não prova isolamento nem o teste de dois canais |
| C | N canais definidos depois da carga | não fecha a topologia; o teste de dois canais continua sem alvo |

**Recomendação: A.** O número 2 é a menor topologia que prova isolamento; não é medição nem meta de
capacidade. A primeira fatia usa um canal. **Custo:** dois processos. **Fontes:** `lacunas.md`
`LSRV-02`; `wyd-server-1.0.0.md` §6; `wyd-1.0.0.md` §7.6. **Recusa B** por não provar; **C** por adiar.

**Resposta do dono em 2026-11-23: A.** `LSRV-02` fechada.

### Q-SRV-3 — O que `ready` significa?

**Medido.** Não há `/ready` implementado; o contrato S3 estava proposto e `LSRV-03` estava aberta. `wyd-ops`
usa prontidão para liberar tráfego; a resposta fecha o contrato.

**O que está em jogo.** Socket aberto sem banco/tick libera jogador para um servidor que não atende;
banco sem tick idem; exigir os três é mais lento, mas honesto.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | `ready` só quando socket, banco alcançável e tick rodando | não libera tráfego para processo incompleto; startup mais lento |
| B | Socket aberto | rápido; libera jogador antes do banco/tick |
| C | Banco alcançável | garante persistência; não garante atendimento |

**Recomendação: A.** É o único sinal que corresponde a "pode atender"; `wyd-ops` depende dele para
liberar tráfego. **Custo:** startup condicionado aos três. **Fontes:** `lacunas.md` `LSRV-03`;
`wyd-server-1.0.0.md` §3 S3; `wyd-1.0.0.md` §7.6. **Recusa B/C** por liberar cedo.

**Resposta do dono em 2026-11-23: A.** `LSRV-03` fechada.

### Q-SRV-4 — Encerramento é decisão de desenho ou execução?

**Medido.** O gate já exige que `SIGTERM` drene e persista; `WYDDB-021` é a issue que fecha a sequência
(save, fila e pool). Sinal, tempo de graça e política de kill estão fora do escopo.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Fechar `LSRV-04` como execução: drenar, persistir, fechar fila e sair; `WYDDB-021` detalha a ordem | uma lacuna a menos, sem inventar número operacional |
| B | Definir sinal e tempo de graça agora | mais clareza de deploy; bloqueia o fechamento e duplica `WYDDB-021` |
| C | Transferir a política para `wyd-ops` | mantém a lacuna aberta e o gate de deploy sem dono explícito |

**Recomendação: A.** O comportamento já foi decidido pelo gate; o que falta é implementar. **Custo:**
nenhum novo; a issue `WYDDB-021` continua dona da sequência. **Fontes:** `lacunas.md` `LSRV-04`;
`wyd-server-1.0.0.md` §6; `wyd-db/docs/roadmap/WYDDB-021.md`. **Recusa B** por criar política que a
fonte deixou fora do escopo; **C** por não fechar o registro do servidor.

**Resposta do dono em 2026-11-23: A.** `LSRV-04` fechada como execução.

### Q-SRV-5 — Quem completa o handshake em A1? (respondida em 2026-11-23)

**Contexto.** O `ECAP` está especificado, mas a topologia deixava o peer em aberto. O `wyd-server` não tem
processo; o servidor legado está em `deprecated/`, mas é referência ignorada, não infraestrutura
aprovada. Sem peer, o proxy grava bytes e A1 fica inconclusiva; por consequência, `LPRT-03` e `LPRT-06`
continuam sem a captura que as fecha, e o passo 1 do esqueleto não tem prova.

**O que está em jogo.** A1 precisa de dois sentidos do CPSock. O peer tem de completar o handshake sem
exigir que o `wyd-server` já implemente o handshake que a própria A1 vai medir. Também precisa ficar em
loopback, sem produção, e poder ser descartado sem virar fonte de verdade.

**Critério de escolha:** completar o CPSock nos dois sentidos; permitir o primeiro byte do cliente; não
criar dependência circular com o handshake; ficar fora de produção; ser descartável.

| Opção | O que é | Destrava | Bloqueia / custo | Risco |
|---|---|---|---|---|
| **A** | servidor novo do `wyd-server` no passo 0, executado localmente como peer | caminho de produção e peer real | A1 espera a implementação do passo 0, banco/canal/configuração; não é captura imediata | circular: o handshake é o que A1 mede; atrasa o fio |
| **B** | harness mínimo que responde ao CPSock | um peer isolado e controlável | não destrava A1 de forma confiável; precisa adivinhar o handshake | inventa comportamento; pode gerar captura inconclusiva e um protocolo paralelo |
| **C** | servidor legado de `deprecated/` como harness de captura | A1 agora, **se** o legado executar de forma reproduzível; dois sentidos sem inventar CPSock novo | não destrava o servidor de produção; exige build/versão/hash e quarentena; executabilidade não verificada | contaminar conclusões com comportamento legado; ausência em clone limpo; marcar `H` e descartar |
| **D** | modo de captura com feature flag dentro do `wyd-server` | reutiliza o caminho do servidor e um binário | A1 espera servidor e handshake; amplia a superfície de configuração | modo de captura vazar para deploy; mais código antes de A1 |

**Recomendação: C, com quarentena.** É a única opção que, se o legado puder ser executado de forma
reproduzível, permite A1 agora sem exigir que o `wyd-server` já saiba o handshake que A1 vai medir e
sem inventar um CPSock de teste. O peer legado entra apenas como harness local: versão/hash na ficha,
loopback, sem cópia de código, sem promover os bytes do peer a fato e sem concluir comportamento do
servidor a partir dele. Se o legado não puder ser executado de forma reproduzível, a recomendação não
autoriza B; A1 espera A ou D.

**Resposta do dono em 2026-11-23: C, com quarentena.** O peer de A1 seria o **servidor legado de
`deprecated/`**, executado localmente e somente como harness de captura. A decisão destrava a captura de
A1 sem acoplar o `wyd-server` ao handshake que ainda está sendo medido, e não fecha `LPRT-03` ou `LPRT-06`
sozinha: ela libera a captura que as fecha. O build-alvo (`Q-PRT-1`, decidido em 2026-11-23) não impede
executar A1 e já não é pendência para declará-la conclusiva; a ficha ainda identifica a build e o par
cliente–peer. `LSRV-05` fechada nesta opção.

**Trilha alternativa de implementação.** Se a prioridade for implementar em vez de decidir o harness,
a trilha é **A**: implementar o peer no `wyd-server` (passo 0) e o `ECAP`, com a CLI do `wyd-client`;
A1 espera essa implementação, mas o caminho fica mais próximo da produção. **B não é trilha**: ela
inventa o handshake e pode produzir captura que não prova o cliente.

**Condições da quarentena (vinculantes para a captura).** O peer legado roda apenas em loopback, fora de
produção e fora do compose de jogo; a ficha registra **versão e hash** do binário executado; nenhum
código é copiado para a árvore nova; os bytes do peer entram como `H`, nunca como comportamento do
servidor novo; o harness é descartado ao fim de A1. O peer também tem build: a ficha registra a
versão/hash dele e o par com o alvo fixado em `Q-PRT-1`.

**Resultado da condição (2026-11-23): reprovada.** O binário legado `0.1.0` (identidade, versão e hash
em [`specs/endpoint-de-captura.md`](specs/endpoint-de-captura.md) §9.1) **sobe em loopback** e aplica 53
migrations em banco vazio, com `/health` 200 — mas a opção não serve para A1:
`deprecated/*` está no `.gitignore` e `git ls-files deprecated` devolve 0 arquivos (clone limpo não tem o
peer); o boot exige `MIGRATIONS_DIR` apontado manualmente, porque o default `/migrations` é caminho de
container; e o binário compila `PROTOCOL_VERSION = 12000` com gate estrito, enquanto o cliente medido
declara 7640/7662. **Pela contingência declarada acima, A1 passa a depender da opção A**; B segue
recusada. A medição completa está em [`specs/endpoint-de-captura.md`](specs/endpoint-de-captura.md) §9.1.

**Fora de escopo.** Bytes, offsets, origem da `KeyWord` (`LPRT-03`), encoding/terminador (`LPRT-06`),
build-alvo (`LPRT-09`/`Q-PRT-1`), implementação do `ECAP`/CLI, schema e cópia de código legado. A
execução do legado era **condição** da opção C, não decisão desta pergunta, e foi medida em 2026-11-23
com resultado negativo (§9.1 da spec).

**Próximo passo depois da resposta.** Verificar se o legado executa de forma reproduzível e, em caso
positivo, implementar o `ECAP` e a CLI, executar A1 e registrar versão/hash na ficha. A spec técnica está
em [`specs/endpoint-de-captura.md`](specs/endpoint-de-captura.md) §9.

## 5. Cobertura das lacunas

| Lacuna | Pergunta | Situação |
|---|---|---|
| `LSRV-01` | `Q-SRV-1` | fechada (A, 2026-11-23) |
| `LSRV-02` | `Q-SRV-2` | fechada (A, 2026-11-23) |
| `LSRV-03` | `Q-SRV-3` | fechada (A, 2026-11-23) |
| `LSRV-04` | `Q-SRV-4` | fechada (A, 2026-11-23) como execução; `WYDDB-021` é a dona da sequência |
| `LSRV-05` | `Q-SRV-5` | fechada (C, com quarentena, 2026-11-23); condição reprovada na medição da mesma data — A1 depende de A |

## 6. Registro da decisão

**Respondidas em 2026-11-23; nenhuma pendente.** `Q-SRV-1`–`Q-SRV-4` foram aprovadas na opção A e
`Q-SRV-5` na opção C, com quarentena — cuja condição foi medida e reprovada, levando A1 à opção A.
`LSRV-04` foi fechada como execução, não como nova política. Nenhuma página foi marcada `aceito`.

| Pergunta | Lacuna | Resposta |
|---|---|---|
| `Q-SRV-1` | `LSRV-01` | A — um binário, um processo por canal; bibliotecas |
| `Q-SRV-2` | `LSRV-02` | A — 2 canais no total; 1 na primeira fatia; lista no launcher/login |
| `Q-SRV-3` | `LSRV-03` | A — socket + banco + tick; senão 503 |
| `Q-SRV-4` | `LSRV-04` | A — execução; gate drena/persiste; `WYDDB-021` dona da sequência |
| `Q-SRV-5` | `LSRV-05` | C — servidor legado em quarentena; condição reprovada em 2026-11-23 (clone limpo, `MIGRATIONS_DIR`, gate 12000); A1 passa a A |

## 7. Autorização de A1 — resposta A (2026-11-23)

**A1 = A.** O dono autorizou a implementação do peer da opção A com handshake mínimo local, do `ECAP` e
da CLI, e a execução da captura A1. Não é nova lacuna e não altera as contagens do sistema
(**5 = 0 abertas/0 propostas/5 fechadas**).

**O que a autorização fecha:** nada imediatamente. **O que ela permite fechar:** `LPRT-03`, no
`wyd-protocol`, após captura completa, segunda leitura e golden; a ficha registra a build de `LPRT-09`.
**O que não fecha:** `LPRT-06`, que depende das capturas B/C com conta sintética e `seeds/dev.sql`; o nome
do QA (`LCLI-03`); `LCLI-02`/`LCLI-05`; F9 do portal e os seis seeds do `wyd-db`.

**Custo do recorte autorizado:** **11 arquivos autorais** — 1 `Cargo.toml` de workspace na raiz; 6 em
`crates/wyd-server` (`Cargo.toml`, `src/main.rs`, `src/ecap.rs`, `src/capture.rs`, `src/peer.rs`,
`tests/ecap.rs`); 4 em `wyd-client/tools` (`Cargo.toml`, `src/main.rs`, `src/ficha.rs`,
`src/client_config.rs`). **0 migrations, 0 tabelas, 0 colunas, 0 workflows e 0 mudanças de gate.** O
`Cargo.lock` é gerado pelas ferramentas; a captura bruta fica fora do repositório.
