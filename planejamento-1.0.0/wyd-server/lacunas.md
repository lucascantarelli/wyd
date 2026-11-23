# `wyd-server` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-23).** Único lugar onde uma lacuna do `wyd-server` tem dono,
> impacto e estado. Cada linha diz o estado; `LSRV-01`–`LSRV-04` fecharam em 2026-11-23 e
> `LSRV-05` permanece aberta.

## 0. Como ler

Estados: **aberta**, **proposta**, **fechada** (com evidência). `arquitetura` propõe; o dono do
sistema aprova.

## 1. Situação em uma linha

| Grupo | Lacunas | Abertas | Propostas | Fechadas |
|---|---:|---:|---:|---:|
| Topologia | 2 | 0 | 0 | 2 |
| Operação | 1 | 0 | 0 | 1 |
| Ciclo de vida | 1 | 0 | 0 | 1 |
| Captura | 1 | 1 | 0 | 0 |
| **Total** | **5** | **1** | **0** | **4** |

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `LSRV-01` | Como o processo é composto (net + core + acesso a dados) e onde termina o `main`? | sem composição definida, cada crate vira um binário e a ordem de inicialização fica implícita | arquitetura | fechada (2026-11-23: um processo por canal; um binário; `net`, `core` e dados como bibliotecas; `main` só compõe) |
| `LSRV-02` | Quantos canais a `1.0.0` abre e como o jogador escolhe? | a lista de canais aparece no launcher e no login; sem número, o teste de dois canais não existe | produto + `wyd-ops` | fechada (2026-11-23: 2 canais no total; 1 na primeira fatia; escolha por lista no launcher/login) |
| `LSRV-03` | O que `ready` significa: socket aberto, banco alcançável ou tick rodando? | `wyd-ops` usa prontidão para liberar tráfego; prontidão errada derruba jogador em deploy | `wyd-ops` + arquitetura | fechada (2026-11-23: socket aceito + banco alcançável + tick rodando; senão 503) |
| `LSRV-04` | O que acontece no encerramento: drena conexão, salva e sai, ou mata o processo? | o save em lote é de `wyd-db` (`WYDDB-017`, `WYDDB-021`); sem ordem, há perda de estado | `wyd-db` + arquitetura | fechada (2026-11-23: execução, não desenho; gate drena/persiste; `WYDDB-021` dona da sequência; sinal/graça/kill fora do escopo) |
| `LSRV-05` | Quem completa o handshake CPSock em A1: servidor local novo, harness mínimo, legado de `deprecated/` ou modo de captura no `wyd-server`? | sem peer, o `ECAP` grava bytes mas A1 fica inconclusiva e `LPRT-03` não fecha | arquitetura + `wyd-server`; `wyd-protocol` valida o peer | **aberta** (Q-SRV-5; recomendação C, não decisão; spec em `docs/specs/endpoint-de-captura.md` §9) |

**`LSRV-01` — decisão de 2026-11-23.** O `wyd-server` é **um processo por canal**, em **um binário**;
`net`, `core` e o acesso a dados entram como bibliotecas, e o `main` só compõe, lê configuração e
inicia o canal. Nenhum crate vira binário próprio. A `1.0.0` abre **2 canais**, com escolha por lista
no launcher/login; a primeira fatia usa **1 canal** (`LSRV-02`, fechada em 2026-11-23).

**`LSRV-02` — decisão de 2026-11-23.** São **2 canais no total da `1.0.0`**; a primeira fatia usa
**1**, e o segundo entra depois sem mudar a composição de `LSRV-01`. O jogador escolhe por lista no
launcher/login.

**`LSRV-03` — decisão de 2026-11-23.** `ready` só responde 200 quando o socket aceita, o banco está alcançável e o tick está rodando; qualquer condição ausente responde 503. O `wyd-ops` só libera tráfego depois desse sinal; o startup fica condicionado aos três.

**`LSRV-04` — decisão de 2026-11-23.** `LSRV-04` não é pergunta de desenho: o gate já exige que `SIGTERM` drene, persista, feche a fila e encerre; `WYDDB-021` é a dona da sequência. Sinal, tempo de graça e política de kill continuam fora do escopo. O que falta é execução.

**`LSRV-05` — decisão aberta (2026-11-23).** O endpoint de captura `ECAP` está especificado em
`docs/specs/endpoint-de-captura.md`, mas o peer que completa o handshake em A1 não foi escolhido. As
opções A–D, com vantagens, custos e riscos, estão em `docs/proposta-decisoes-server.md` `Q-SRV-5`; a
recomendação C não é resposta do dono. Até a resposta, A1 pode gravar bytes, mas não é conclusiva.

## 3. O que não é lacuna deste sistema

- Forma do comando de controle e autenticação do elo: **decidido** (`LAC-ADM-03`, 2026-11-22).
- Métricas expostas e coleta: **decidido** o caminho (`LAC-POR-06`); o catálogo de métricas é de
  `wyd-ops`.
