# `wyd-server` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-25).** Único lugar onde uma lacuna do `wyd-server` tem dono,
> impacto e estado. `LSRV-01`–`LSRV-05` fecharam em 2026-11-23 e `LSRV-05` foi **formalizado em
> `ADR-SRV-0001` em 2026-11-25** (aprovado pelo dono). A medição do peer reprovou a condição da
> opção C: A1 passa a depender de A (ver o parágrafo final da §2).

## 0. Como ler

Estados: **fechada** (com evidência citada) e **bloqueada** (falta decisão, execução ou evidência; a
linha nomeia o desbloqueio — dono e documento). `arquitetura` propõe; o dono do
sistema aprova.

## 1. Situação em uma linha

| Grupo | Lacunas | Bloqueadas | Fechadas |
|---|---:|---:|---:|
| Topologia | 2 | 0 | 2 |
| Operação | 1 | 0 | 1 |
| Ciclo de vida | 1 | 0 | 1 |
| Captura | 1 | 0 | 1 |
| **Total** | **5** | **0** | **5** |

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `LSRV-01` | Como o processo é composto (net + core + acesso a dados) e onde termina o `main`? | sem composição definida, cada crate vira um binário e a ordem de inicialização fica implícita | arquitetura | fechada (2026-11-23: um processo por canal; um binário; `net`, `core` e dados como bibliotecas; `main` só compõe) Refletida em `planejamento-1.0.0/wyd-server/docs/proposta-decisoes-server.md`. |
| `LSRV-02` | Quantos canais a `1.0.0` abre e como o jogador escolhe? | a lista de canais aparece no launcher e no login; sem número, o teste de dois canais não existe | produto + `wyd-ops` | fechada (2026-11-23: 2 canais no total; 1 na primeira fatia; escolha por lista no launcher/login) Refletida em `planejamento-1.0.0/wyd-server/docs/planos/esqueleto-de-ponta-a-ponta.md`. |
| `LSRV-03` | O que `ready` significa: socket aberto, banco alcançável ou tick rodando? | `wyd-ops` usa prontidão para liberar tráfego; prontidão errada derruba jogador em deploy | `wyd-ops` + arquitetura | fechada (2026-11-23: socket aceito + banco alcançável + tick rodando; senão 503) Refletida em `planejamento-1.0.0/wyd-server/docs/proposta-decisoes-server.md`. |
| `LSRV-04` | O que acontece no encerramento: drena conexão, salva e sai, ou mata o processo? | o save em lote é de `wyd-db` (`WYDDB-017`, `WYDDB-021`); sem ordem, há perda de estado | `wyd-db` + arquitetura | fechada (2026-11-23: execução, não desenho; gate drena/persiste; `WYDDB-021` dona da sequência; sinal/graça/kill fora do escopo) Refletida em `planejamento-1.0.0/wyd-server/docs/proposta-decisoes-server.md`. |
| `LSRV-05` | Quem completa o handshake CPSock em A1: servidor local novo, harness mínimo, legado de `deprecated/` ou modo de captura no `wyd-server`? | sem peer, o `ECAP` grava bytes mas A1 fica inconclusiva e `LPRT-03` e `LPRT-06` não fecham | arquitetura + `wyd-server`; `wyd-protocol` valida o peer | fechada (2026-11-23: **C, com quarentena**, condição reprovada na medição; **A1 passa a depender de A**. Formalizada em `ADR-SRV-0001` em 2026-11-25, aprovada pelo dono) Refletida em `planejamento-1.0.0/wyd-server/docs/proposta-decisoes-server.md`. |

**`LSRV-01` — decisão de 2026-11-23.** O `wyd-server` é **um processo por canal**, em **um binário**;
`net`, `core` e o acesso a dados entram como bibliotecas, e o `main` só compõe, lê configuração e
inicia o canal. Nenhum crate vira binário próprio. A `1.0.0` abre **2 canais**, com escolha por lista
no launcher/login; a primeira fatia usa **1 canal** (`LSRV-02`, fechada em 2026-11-23).

**`LSRV-02` — decisão de 2026-11-23.** São **2 canais no total da `1.0.0`**; a primeira fatia usa
**1**, e o segundo entra depois sem mudar a composição de `LSRV-01`. O jogador escolhe por lista no
launcher/login.

**`LSRV-03` — decisão de 2026-11-23.** `ready` só responde 200 quando o socket aceita, o banco está alcançável e o tick está rodando; qualquer condição ausente responde 503. O `wyd-ops` só libera tráfego depois desse sinal; o startup fica condicionado aos três.

**`LSRV-04` — decisão de 2026-11-23.** `LSRV-04` não é pergunta de desenho: o gate já exige que `SIGTERM` drene, persista, feche a fila e encerre; `WYDDB-021` é a dona da sequência. Sinal, tempo de graça e política de kill continuam fora do escopo. O que falta é execução.

**`LSRV-05` — decisão de 2026-11-23.** O peer de A1 foi decidido como **servidor legado de
`deprecated/`**, executado localmente e somente como harness de captura (**opção C, com quarentena**).
Entra em loopback, fora de produção, com **versão e hash** na ficha, sem cópia de código e com os bytes
do peer marcados como `H`; não se conclui comportamento do servidor novo a partir dele.

**A1 autorizada em 2026-11-23 (resposta A).** Não é nova lacuna: o dono autorizou implementar o peer da
opção A com handshake mínimo local, o `ECAP` e a CLI, e executar a captura A1. `LPRT-03` continua bloqueada
até a captura completa e a segunda leitura; `LPRT-06` continua dependente das capturas B/C e do seed
sintético. A contagem do sistema permanece **5 = 0/0/5**.

**A condição de C foi medida e reprovada em 2026-11-23** (`docs/specs/endpoint-de-captura.md` §9.1):
`deprecated/*` está no `.gitignore` e `git ls-files deprecated` devolve 0 arquivos, então clone limpo não
tem o peer; o boot só acontece com `MIGRATIONS_DIR` apontado manualmente para fora do default
`/migrations`; e o binário compila `PROTOCOL_VERSION = 12000` com gate estrito, enquanto o cliente medido
declara 7640/7662 — o handshake seria recusado. **Por decisão do dono na mesma data, A1 passa a depender
da opção A** (peer do `wyd-server` no passo 0, com `ECAP` e CLI), conforme a contingência já declarada em
`Q-SRV-5`; B segue recusada. `LSRV-05` permanece fechada como decisão — a lacuna era qual peer, e a
execução de A é implementação, não nova pergunta.

## 3. O que não é lacuna deste sistema

- Forma do comando de controle e autenticação do elo: **decidido** (`LAC-ADM-03`, 2026-11-22).
- Métricas expostas e coleta: **decidido** o caminho (`LAC-POR-06`); o catálogo de métricas é de
  `wyd-ops`.
