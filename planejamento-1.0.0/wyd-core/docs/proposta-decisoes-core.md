# Decisões do `wyd-core` — escopo, estado, fronteira e persistência

> Pergunta: "o que o core entrega na 1.0.0, o que é memória × durável, onde termina a regra e começa
> a fórmula, e como ele persiste sem SQL?"
> Público: produto, arquitetura, `wyd-core`, `wyd-sim` e `wyd-db`
> Dono: equipe do `wyd-core`
> Base: `lacunas.md` (`LCORE-01`–`LCORE-04`); `docs/index.md`; `wyd-core-1.0.0.md` §1–§6;
> `wyd-1.0.0.md` §1.5, §7.1, §7.3 (X5, X6) e §7.6; `AGENTS.md` §5.8 e §8
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). **Quatro perguntas respondidas** em 2026-11-23; nenhuma pergunta
aberta. A fronteira `core` × `sim` e a fronteira de persistência têm regra verificável e prova definida.
Não há catálogo funcional, código nem teste; nenhum número desta página é medição.

## 1. Fronteira

**Fecha:** escopo funcional sem catálogo, estado em memória × durável, fronteira `core` × `sim` e
contrato de persistência — `LCORE-01`, `LCORE-02`, `LCORE-03` e `LCORE-04`. As quatro fecharam em 2026-11-23; a classificação de
estado e a porta C4 estão em `wyd-core-1.0.0.md` §3.2. **Não fecha:** a fórmula concreta e a aritmética
(`LSIM-01`, `LSIM-04`), que são execução do `wyd-sim`.

**Não decide:** fórmula e balanceamento (`wyd-sim`), SQL/DDL (`wyd-db`), fio (`wyd-protocol`),
socket (`wyd-net`), processo/canal (`wyd-server`).

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| lacunas do sistema | **0 abertas, 0 propostas, 4 fechadas** (`LCORE-01`–`LCORE-04`) | `lacunas.md` §1 |
| catálogo funcional | **não existe**; cobertura "não medido" | `wyd-1.0.0.md` §1.5 |
| arquivos de código | **0** (só `.gitkeep`) | `wyd-core-1.0.0.md` §4 |
| testes | **0** | `wyd-1.0.0.md` §7.6 |
| contratos herdados | payload tipado (`LPRT-18`), ordem (`LPRT-12`), controle (`LAC-ADM-03`) | `docs/index.md` |

## 3. Já decidido — não reabrir

| Item | Decisão vigente | Fonte | Efeito |
|---|---|---|---|
| payload tipado | o core recebe tipo validado, nunca offset | `LPRT-18`; `wyd-1.0.0.md` X3 | não é decisão do core |
| ordem por conexão | o `wyd-net` entrega serializado | `LPRT-12`; X2 | não é decisão do core |
| controle administrativo | HTTP síncrono com ack do admin | `LAC-ADM-03`; X8 | não é decisão do core |

## 4. Decisões

### Q-CORE-1 — Qual é o escopo funcional da 1.0.0 sem inventar catálogo?

**Medido.** O catálogo funcional não existe; a 1.0.0 é "jogo completo" sem lista (`wyd-1.0.0.md` §1.5).
Não há requisito numerado nem código.

**O que está em jogo.** Declarar o catálogo como escopo preserva a meta de 100%, mas não fixa lista;
cortar a 1.0.0 para o esqueleto libera o core agora e muda a meta declarada; esperar o catálogo bloqueia
o passo 5; uma lista fixa inventada não tem prova.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | A `1.0.0` continua jogo completo; o escopo é o catálogo medido no 7662, e o primeiro domínio é entrar na cena com posição/estado inicial, sem inventário, combate ou chat | preserva a meta e libera o passo 5; a lista completa continua execução, não bloqueio de A1 |
| B | Cortar a `1.0.0` para o recorte do esqueleto; catálogo completo vira 1.1 | core desbloqueia agora; muda `wyd-1.0.0.md` §1.5/§4.2 e o gate de 100% |
| C | Lista funcional fixa fornecida pelo dono agora | lista imediata; sem medição no 7662 e com risco de conflito |
| D | Esperar o catálogo completo antes do passo 5 | nenhum retrabalho; o esqueleto e o caminho do fio ficam parados |

**Recomendação: A.** É a única opção que preserva a meta de 100% sem transformar o catálogo inexistente
em bloqueio do primeiro fio; "entrar no mundo" ganha uma fronteira verificável. **Custo:** a cobertura
continua não medida e o catálogo completo segue trabalho de descoberta. **Fontes:** `lacunas.md`
`LCORE-01`; `wyd-1.0.0.md` §1.5, §4.2 e §7.7; `wyd-core-1.0.0.md` §6. **Recusa B** por mudar a meta;
**C** por lista sem prova; **D** por parar o esqueleto.

**Resposta do dono em 2026-11-23: A.** `LCORE-01` fechada.

### Q-CORE-2 — O que é memória, o que é durável e com que frequência se grava?

**Medido (antes da resposta).** Não havia contrato de save nem frequência; `X6` estava em lacuna, e o
`wyd-db` tem `WYDDB-017`/`WYDDB-021` como destinos. Nada foi medido.

**O que está em jogo.** Gravar a cada tick custa I/O; gravar só no encerramento perde estado em queda;
save no release limita a perda ao intervalo desde o último release e evita um job periódico na primeira
fatia; um snapshot periódico reduziria mais a janela, mas custa I/O e job.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Estado vivo em memória; durável crítico antes do ack; posição/cena como durável eventual no release, sem snapshot periódico na primeira fatia; perda limitada ao intervalo desde o release | equilíbrio entre I/O e perda; a frequência é decisão proposta, não medição |
| B | Persistir a cada tick | perda quase zero; custo de I/O e de transação em todo tick |
| C | Só no encerramento | mínimo de I/O; queda do processo perde toda a sessão |

**Recomendação: A.** É o único desenho que declara a perda e cabe no contrato de save em lote do
`wyd-db`. **Custo:** uma janela de perda; o job de snapshot fica fora da primeira fatia. **Fontes:**
`lacunas.md` `LCORE-02`; `wyd-1.0.0.md` X6; `wyd-db/docs/roadmap/WYDDB-017.md` e `WYDDB-021.md`;
`wyd-core-1.0.0.md` §6. **Recusa B** por I/O; **C** por perda silenciosa.

**Resposta do dono em 2026-11-23: A, com recorte de escopo.** O durável crítico é escrito antes do ack;
posição/cena é durável eventual com checkpoint no release e **sem snapshot periódico na primeira
fatia**; memória é derivada/rebuildável. O recorte não muda a classificação, apenas retira o job
periódico do primeiro marco.

### Q-CORE-3 — Onde termina a regra do `core` e começa a fórmula do `sim`?

**Medido.** `X4`/`C3` já exigem função pura e determinística, mas não havia critério que separasse
decisão de cálculo; `LCORE-03` estava aberta e o passo 6 não sabia onde colocar a regra.

**Decisão de 2026-11-23.** A responsabilidade é do `wyd-sim` quando pode ser escrita como função
**total, pura e determinística** `f(entrada tipada e imutável) -> saída tipada`, sem estado de mundo,
sessão ou banco, sem relógio, RNG ou I/O; o mesmo vetor produz sempre o mesmo vetor. É do `wyd-core`
quando precisa ler ou alterar estado, escolher alvo/ordem/momento, validar pré-condição, consumir
sorteio, aplicar efeito, emitir evento ou persistir. O `sim` devolve probabilidade/limiar ou recebe o
resultado do sorteio; o RNG e o estado ficam no core.

**Como verificar.** (1) API do `sim` só recebe entrada tipada e devolve saída tipada, sem `&mut World`,
sessão, repositório, `Clock`, `Rng` ou I/O; (2) cada fórmula tem vetor de entrada/saída (`LSIM-03`); (3)
o core é testado com mundo/repositório falso e não repete a expressão da fórmula; (4) no passo 6 o core
valida sessão/personagem, resolve mapa/colisão e aplica a posição, enquanto o `sim` calcula apenas o
deslocamento puro. A fórmula concreta e a aritmética continuam em `LSIM-01` e `LSIM-04`.

**Efeito.** `LCORE-03` fecha; a fórmula de movimento continua sendo execução, não uma nova decisão de
fronteira.

### Q-CORE-4 — Como o core lê e grava sem SQL, e o que é transacional?

**Medido (antes da resposta).** `X5` estava proposto; `LCORE-04` aberta; não havia porta, adapter nem
teste de contrato. `C4`/`X5` e a separação de papéis (`P-18`) mantêm SQL fora do core; `P-17` é a view
de leitura entre domínios, não a regra de SQL do core.

**O que está em jogo.** A granularidade da porta muda o contrato e a fronteira transacional: uma porta
C4 única reduz interfaces, mas exige uma `UnitOfWork` explícita para não esconder a transação; portas
por agregado deixam a unidade de trabalho explícita, mas multiplicam tipos antes de existir operação
que as separe; separar leitura de comandos aumenta a superfície sem exigência da primeira fatia.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Uma porta C4 (`CoreRepository`) para sessão/personagem, com `UnitOfWork` por comando; adapter no `wyd-db`, que é dono da transação | contrato pequeno e testável; a ordem e o conteúdo da UoW ficam no core; a transação fica explícita no `commit` |
| B | Portas por agregado, cada uma com sua unidade de trabalho | separação maior; mais interfaces e mais composição sem operação que as exija agora |
| C | Portas de leitura separadas de comandos, cada comando com sua unidade de trabalho | leitura e escrita explícitas; mais tipos e mais código de composição |

**Recomendação: A.** É a forma que o contrato X5 já desenha, permite testar o core com repositório
falso e mantém o `wyd-db` como dono da transação. **Custo:** uma porta e uma `UnitOfWork`; a primeira
fatia usa uma operação de escrita por comando. **Fontes:** `lacunas.md` `LCORE-04`; `wyd-1.0.0.md` X5;
`wyd-core-1.0.0.md` §3.2; `wyd-db/padrao-banco.md` `P-18`; `AGENTS.md` §5.8. **Recusa B** por criar
interfaces sem operação que as exija; **C** por aumentar a composição sem exigência da fonte.

**Resposta do dono em 2026-11-23: A, com o recorte C4.** A primeira fatia tem **uma** porta
`CoreRepository` com `load_session`, `load_character` e `commit(UnitOfWork)`; o `wyd-db` abre, executa e
fecha a transação; o core define apenas o conteúdo e a ordem da UoW. Leitura ausente é `None`; os erros
são tipados; `CommitUncertain` exige nova leitura e nunca retry cego. O contrato completo, as operações
e a prova estão em `wyd-core-1.0.0.md` §3.2; a obrigação do adapter está em
`wyd-db/docs/guia-desenvolvedor/portas-e-contratos.md` §5.

## 5. Cobertura das lacunas

| Lacuna | Pergunta | Situação |
|---|---|---|
| `LCORE-01` | `Q-CORE-1` | fechada (A, 2026-11-23) |
| `LCORE-02` | `Q-CORE-2` | fechada (A, 2026-11-23; checkpoint no release, sem snapshot periódico na primeira fatia) |
| `LCORE-03` | `Q-CORE-3` | fechada (regra de 2026-11-23) |
| `LCORE-04` | `Q-CORE-4` | fechada (A, recorte C4, 2026-11-23) |

## 6. Registro da decisão

**Respondidas em 2026-11-23.** As quatro perguntas têm resposta do dono e as quatro lacunas estão
fechadas. Nenhuma página foi marcada `aceito`; o que falta é implementação e prova, não decisão.

| Pergunta | Lacuna | Resposta |
|---|---|---|
| `Q-CORE-1` | `LCORE-01` | A — 1.0.0 = catálogo completo; primeiro domínio = entrada na cena |
| `Q-CORE-2` | `LCORE-02` | A — crítico antes do ack; posição no release, sem periódico na primeira fatia; memória derivada |
| `Q-CORE-3` | `LCORE-03` | Regra — sim puro; core dono do estado/decisão/RNG/aplicação |
| `Q-CORE-4` | `LCORE-04` | A — C4 única com `load_session`, `load_character` e `commit(UoW)`; `wyd-db` dono da transação |
