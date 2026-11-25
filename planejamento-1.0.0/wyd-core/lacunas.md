# `wyd-core` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-23).** Único lugar onde uma lacuna do `wyd-core` tem dono,
> impacto e estado. `LCORE-01`–`LCORE-04` fecharam em 2026-11-23; não há lacuna bloqueada neste sistema.

## 0. Como ler

Estados: **fechada** (decidida ou produzida, com evidência citada) e **bloqueada** (falta decisão,
execução ou evidência; a linha nomeia o desbloqueio — dono e documento). Dono propõe `arquitetura`; o dono do sistema aprova.

## 1. Situação em uma linha

| Grupo | Lacunas | Bloqueadas | Fechadas |
|---|---:|---:|---:|
| Escopo e catálogo | 1 | 0 | 1 |
| Modelo de estado | 1 | 0 | 1 |
| Fronteira com `wyd-sim` | 1 | 0 | 1 |
| Persistência | 1 | 0 | 1 |
| **Total** | **4** | **0** | **4** |

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `LCORE-01` | Que funcionalidades o `wyd-core` entrega na `1.0.0`, já que o catálogo funcional não existe? | sem entrada de catálogo não há meta de cobertura nem tamanho de sistema; hoje a `1.0.0` é "jogo completo" sem lista (`wyd-1.0.0.md` §1.5) | produto + arquitetura | fechada (2026-11-23: a `1.0.0` continua jogo completo; o escopo é o catálogo completo medido no 7662, e o primeiro domínio executável é entrar na cena com posição/estado inicial, sem inventário, combate ou chat) |
| `LCORE-02` | O que é estado em memória e o que é durável, e com que frequência o durável é escrito? | define save em lote, perda aceitável e o contrato com `wyd-db` (`WYDDB-017`) | arquitetura + `wyd-db` | fechada (2026-11-23: durável crítica = identidade/sessão/personagem/inventário/economia/progressão/auditoria, escrita antes do ack; durável eventual = posição/cena, checkpoint no release, sem periódico na primeira fatia; memória = derivado/rebuildável; dado novo entra na tabela §3.2 antes do código) Refletida em `planejamento-1.0.0/wyd-core/docs/proposta-decisoes-core.md`. |
| `LCORE-03` | Onde termina a regra do core e começa a fórmula do `wyd-sim`? | a divisão foi decidida em tese (2026-11-20); sem critério de fronteira, a mesma regra nasce duas vezes | arquitetura | fechada (2026-11-23: `sim` = função total, pura e determinística de entrada tipada imutável para saída tipada, sem estado/I/O/relógio/RNG; `core` = lê e altera estado, decide alvo/momento, consome sorteio, aplica efeito e persiste) Refletida em `planejamento-1.0.0/wyd-core/docs/proposta-decisoes-core.md`. |
| `LCORE-04` | Como o core lê e grava sem SQL próprio, e quais operações são transacionais entre domínios? | `P-17` e a separação de papéis proíbem SQL espalhado; a porta precisa de contrato | `wyd-db` + arquitetura | fechada (2026-11-23: C4 expõe `load_session`, `load_character` e `commit(UnitOfWork)`; `wyd-db` é dono da transação; core é dono da ordem/conteúdo da UoW; ausência é `None`, erro é tipado; nenhum tipo de persistência cruza para o `sim`) Refletida em `planejamento-1.0.0/wyd-core/docs/proposta-decisoes-core.md`. |

**Decisão de escopo (2026-11-23).** A `1.0.0` continua **jogo completo**: o escopo funcional é o
catálogo completo, medido no cliente 7662. O primeiro domínio executável é **entrar na cena com
posição/estado inicial**, sem inventário, combate ou chat; o recorte do `esqueleto` é a primeira prova,
não a `1.0.0` inteira. A lista do catálogo continua sendo execução, não uma lista fixa inventada agora;
`LCORE-02` e `LCORE-04` fecharam em 2026-11-23 como uma única fronteira de persistência: classificação de estado em `wyd-core-1.0.0.md` §3.2 e porta C4 no mesmo documento.

**Decisão de fronteira (2026-11-23).** `wyd-sim` só contém função total, pura e determinística
`entrada tipada imutável -> saída tipada`: não recebe `World`, sessão, banco, relógio ou RNG, não muta
estado e não faz I/O; o mesmo vetor produz o mesmo vetor. `wyd-core` é dono de ler/validar estado,
escolher alvo e momento, consumir o sorteio, aplicar o resultado, emitir evento e persistir. A regra
completa e a prova estão em `wyd-core-1.0.0.md` §3.1.

## 3. O que não é lacuna deste sistema

- Payload tipado na fronteira: **decidido** (`LPRT-18`).
- Ordem por conexão: **decidida** (`LPRT-12`), dono `wyd-net`.
