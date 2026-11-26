# Decisões do `wyd-net` — limites, versão e observabilidade

> Pergunta: "qual é o limite/timeout/rate-limit, a versão é negociada ou imposta e o que o transporte
> expõe em métrica e log?"
> Público: arquitetura, `wyd-net`, produto, `wyd-client` e `wyd-ops`
> Dono: equipe do `wyd-net`
> Base: `lacunas.md` (`LNET-01`–`LNET-04`); `docs/index.md`; `wyd-net-1.0.0.md` §1–§6;
> `wyd-1.0.0.md` §7.1, §7.3 (X1–X3) e §7.6; `AGENTS.md` §5, item 7 e §8
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). **Nenhuma pergunta aguarda o dono**: `Q-NET-1` e `Q-NET-2` foram
respondidas (= A), `LNET-03` fechou pela regra `R6`/`D9`/`ux_session_viva_por_conta` e `LNET-04` por
**regra vigente** (`Q-NET-4` = A, derivada em §4), sem escolha do usuário. `LNET-01` continua **aberta
como medição**: a política foi decidida, o valor não. Nada foi implementado. **Nenhum número desta página
é medição**: não há conexões, latência nem carga medidas; todo limite numérico é decisão proposta.

## 1. Fronteira

**Decide:** limites por conexão/timeout/rate-limit — `LNET-01`. **Já decididas:** `LNET-02` (política
de versão), fechada em 2026-11-23 por `Q-NET-2` = A; `LNET-03` (modelo de sessão), fechada pela regra
`R6`/`D9`/`ux_session_viva_por_conta`; e `LNET-04` (observabilidade), fechada por decisão derivada das
regras vigentes (§4). Em todas, o transporte implementa; não escolhe. A política de `LNET-01` foi
decidida em `Q-NET-1` = A, mas a lacuna **não fecha** até a primeira medição no CI efêmero.

**Não decide:** byte/opcode/ofuscação (`wyd-protocol`), regra de jogo (`wyd-core`/`wyd-sim`), processo
por canal (`wyd-server`), sessão persistida (`wyd-db`).

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| lacunas do sistema | **1 aberta, 0 propostas, 3 fechadas** | `lacunas.md` §1 |
| arquivos de código | **0** (só `.gitkeep`) | `wyd-net-1.0.0.md` §4 |
| conexões, carga e latência | **nada medido** | `docs/index.md`; `wyd-1.0.0.md` §7.6 |
| contratos herdados | `LPRT-13`, `LPRT-08`, `LPRT-12`, `LPRT-18` decididos | `docs/index.md` |
| teste alvo | duas conexões sem vazamento; `bots` conectando | `wyd-1.0.0.md` §7.6 |

## 3. Já decidido — não reabrir

| Item | Decisão vigente | Fonte | Efeito |
|---|---|---|---|
| erro de codec | a reação é da conexão, não do codec | `LPRT-13` | não é decisão desta rodada |
| `Type` desconhecido | `UnknownType` tipado; reação é daqui | `LPRT-08` | idem |
| ordem por conexão | entrega serializada | `LPRT-12`; X2 | idem |
| payload tipado | mensagem cruza validada, nunca por offset | `LPRT-18`; X1/X3 | idem |

## 4. Decisões

### Q-NET-1 — Qual limite, timeout e rate-limit?

**Medido.** Não há conexões, latência, abuso nem timeout medidos; `LNET-01` está aberta e o plano diz
que "sem número, o limite vira opinião".

**O que está em jogo.** Fixar números agora é promessa sem fonte; não ter limite deixa abuso e conexão
zumbi sem defesa. A decisão é **como** o número entra.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Medir primeiro em ambiente controlado; limite/timeout só entram no gate depois da medição, com data | nenhum número inventado; a defesa fica sem valor até a medição |
| B | Adotar um teto provisório por conexão/IP, explicitamente provisório | defesa imediata; o número não é medido e pode bloquear jogador legítimo |
| C | Fixar agora conexões, ociosidade e rate-limit alvo | permite escrever o teste; o número não tem fonte |

**Recomendação: A.** `AGENTS.md` §8 exige gate medido; o limite é decisão, não estimativa. **Custo:**
nenhum limite numérico antes da medição. **Fontes:** `lacunas.md` `LNET-01`; `docs/index.md`;
`AGENTS.md` §8. **Recusa C** por inventar número; **B** por tratar provisório como regra.

**Resposta do dono em 2026-11-23: A.** A **forma** do contrato N6 fica fechada agora: balde por conexão,
porte por IP, teto total por shard e timeout de handshake/ociosidade; ao estourar, o servidor devolve
**recusa tipada** (`RateLimited` para pacote, `TooManyPerIp`/`TooManyTotal` para handshake, timeout para
handshake/ociosidade), **conta o motivo** e **fecha a conexão** quando a recusa é de conexão ou timeout.
Os **valores** não entram antes da primeira execução no CI efêmero de `Q-BOT-1`, com o seed de `Q-BOT-2`
e o método de `Q-BOT-3`; o valor medido é congelado com ambiente/data. O cenário de carga declara o
**perfil de teste** ou usa origens distintas, para que o limite por IP não seja confundido com saturação
do shard. Nenhum número é inventado e a lacuna não fecha nesta decisão.

| Alternativa | Por que foi descartada |
|---|---|
| B — teto provisório | defende antes da medição, mas o número é de infraestrutura e pode bloquear jogador legítimo |
| C — número fixado agora | não tem fonte; vira promessa sem `V`, contra `AGENTS.md` §5, item 4 e §8 |

**Custo do que A ainda exige, não implementado.** **0 migration, 0 tabela, 0 coluna.** Na implementação:
**1 página** `docs/specs/limites-e-timeouts.md`, **1 `Cargo.toml`** em `crates/wyd-net`, **1 `src/lib.rs`**,
**1 `src/limits/rate.rs`**, **1 `src/metrics.rs`**, **1 teste** `tests/integracao/flood.rs` e **1
`Cargo.toml`** de workspace na raiz — **7 arquivos**; o timeout pode acrescentar `src/session/mod.rs` e
levar a **8**. Nenhum workflow novo; o job de carga é o de `Q-BOT-1`. O gate do `wyd-net` continua devendo
`RF-NET`, árvore de `src/`/`tests/` e plano de teste.

**Efeito registrado (`V`, 2026-11-23).**

| Efeito | Fonte |
|---|---|
| política do contrato N6 decidida | `wyd-net-1.0.0.md` §3 |
| `LNET-01` continua aberta como medição | `lacunas.md` §1 |
| resposta `Q-NET-1` encerrada; a seguinte foi `LAC-POR-09` | §6 |

**Não fecha:** `LNET-01`, os valores, o crate, o teste, o `/metrics` novo, o `wyd-server`, o codec e o gate
do sistema. `LBOT-03` segue aberta como medição, mas passa a ter um caminho interpretável quando o perfil
de teste for declarado.

### Q-NET-2 — A versão do protocolo é negociada ou imposta pelo build?

**Medido.** `LPRT-09` foi decidido em 2026-11-23 (cliente 7662 + SHA-256), então o build está fixado; não
há versão de protocolo negociada. A pergunta de negociação foi respondida abaixo, em 2026-11-23.

**O que está em jogo.** Negociação permite vários builds; impor o build simplifica o handshake, mas
quebra compatibilidade. Com o alvo fixado, a escolha deixa de depender de build e passa a ser de
política de compatibilidade.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Impor o build-alvo no handshake, com erro tipado para build não suportado | handshake simples e testável; um build por vez |
| B | Negociar versão na sessão | aceita mais de um build; exige tabela de compatibilidade e testes por versão |
| C | Aceitar qualquer build e descobrir o layout em runtime | não bloqueia; offset ambíguo e falha silenciosa |

**Recomendação: A.** Com o build fixado em 2026-11-23, impor é o menor contrato verificável. **Custo:**
um build suportado. **Fontes:** `lacunas.md` `LNET-02`; `LPRT-09`; `wyd-net-1.0.0.md` §3. **Recusa B**
por compatibilidade sem prova; **C** por ambiguidade.

**Resposta do dono em 2026-11-23: A.** O `wyd-net` impõe o build-alvo **7662** no handshake e recusa
build não suportado com erro tipado. Negociar versão (B) exigiria tabela de compatibilidade e testes por
versão, e não é o que a `1.0.0` faz. O formato do erro e o lugar da versão no fio não são desta página:
o layout é de `wyd-protocol`. A lacuna fechou por decisão; o teste do handshake nasce com a
implementação, no gate do sistema.

### Q-NET-4 — O que o transporte expõe em métrica e log?

**Medido.** Não há métrica nem log de transporte; `wyd-ops` opera por Prometheus/Loki e `AGENTS.md`
§5.7 proíbe payload/credencial em log. A lacuna está aberta.

**O que está em jogo.** Métrica de conexão/erro/latência ajuda a operar; log de payload viola
segredo/PII; sem métrica, o NOC fica cego.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Métricas de conexões, erros por tipo, bytes e latência; log só de metadados, sem payload/PII | operável e seguro; exige catálogo e scrub |
| C | Sem métrica/log | mínimo; o NOC não vê o transporte |

**Recomendação: A.** É o único conjunto que serve ao NOC sem violar §5.7. **Custo:** catálogo de
métricas e revisão de log. **Fontes:** `lacunas.md` `LNET-04`; `AGENTS.md` §5, item 7;
`wyd-1.0.0.md` §7.3 (X10). **Recusa C** por cegueira operacional; log de payload não é alternativa
válida pela regra de segredo/PII.

**Resposta derivada das regras vigentes, sem escolha do usuário (2026-11-23): A.** `X10` já decidiu que
a métrica do NOC é `/metrics` Prometheus coletada pelo `wyd-ops` (`wyd-1.0.0.md` §7.3), `LAC-POR-06`
já decidiu que a telemetria vem de fonte externa e o portal mostra “não medido” quando ela falha
(`wyd-portal/lacunas.md` §2), e `AGENTS.md` §5, item 7 proíbe payload e PII em log. A opção C contradiz as
duas primeiras; log de payload contradiz a terceira. O transporte expõe **conexões, erros por tipo,
bytes e latência** e registra **só metadados** (evento, sessão, tamanhos, caminho); nome, rótulo e
unidade das métricas são implementação, não decisão pendente. A lacuna `LNET-04` fechou por essa regra;
`LNET-01` não fecha, porque o valor do limite é medição (§4 `Q-NET-1`). A pauta em `../../wyd-ops/docs/pauta-de-decisoes-pendentes.md` e `../../lacunas.md`
registram as respostas.

## 5. Cobertura das lacunas

| Lacuna | Pergunta | Situação |
|---|---|---|
| `LNET-01` | `Q-NET-1` | **aberta como medição** (2026-11-23): política A decidida — balde/porte/total/timeout com recusa tipada; valores só após a primeira execução no CI efêmero. |
| `LNET-02` | `Q-NET-2` | fechada em 2026-11-23 (A): impor o build-alvo 7662 no handshake, com erro tipado para build não suportado; o transporte implementa, não escolhe. |
| `LNET-03` | — | fechada em 2026-11-23: a regra `R6`/`D9` e `ux_session_viva_por_conta` fixam uma sessão por conta e recusam a segunda; `wyd-db/rascunho-accounts.md` §2 (`R6`) e §6 (`D9`); §3 proíbe limite por IP como identidade. O transporte implementa, não escolhe. |
| `LNET-04` | `Q-NET-4` | fechada em 2026-11-23 (A): **derivada das regras vigentes, sem escolha do usuário** — métricas de conexão/erro/bytes/latência em `/metrics` (`X10`), fonte externa (`LAC-POR-06`) e log só de metadados, sem payload/PII (`AGENTS.md` §5, item 7). |

## 6. Registro da decisão

**`Q-NET-1` e `Q-NET-2` respondidas e `Q-NET-4` derivada em 2026-11-23.** `LNET-02` fechou na opção A;
`LNET-04` fechou por **regra vigente** (A derivada, §4), sem escolha do usuário; `Q-NET-1` = A fixou a
política de `LNET-01`, que continua `aberta` como medição. `LNET-03` está `fechada` pela regra de sessão
já existente. Nada foi implementado: o transporte só executa o que aqui foi decidido.

| Pergunta | Lacuna | Resposta |
|---|---|---|
| `Q-NET-1` | `LNET-01` | A — **medir e congelar** (2026-11-23): forma do N6 fechada (balde/porte/total/timeout, recusa tipada, contador e fechamento); valores só após a primeira execução no CI efêmero; `LNET-01` continua aberta como medição |
| `Q-NET-2` | `LNET-02` | A — impor o build-alvo 7662 no handshake, com erro tipado para build não suportado (2026-11-23) |
| `Q-NET-4` | `LNET-04` | A — **derivada das regras vigentes, sem escolha do usuário** (2026-11-23): métricas de conexão/erro/bytes/latência em `/metrics` e log só de metadados, sem payload/PII |
