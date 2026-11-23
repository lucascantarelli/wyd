# Decisões do `wyd-net` — limites, versão e observabilidade

> Pergunta: "qual é o limite/timeout/rate-limit, a versão é negociada ou imposta e o que o transporte
> expõe em métrica e log?"
> Público: arquitetura, `wyd-net`, produto, `wyd-client` e `wyd-ops`
> Dono: equipe do `wyd-net`
> Base: `lacunas.md` (`LNET-01`–`LNET-04`); `docs/index.md`; `wyd-net-1.0.0.md` §1–§6;
> `wyd-1.0.0.md` §7.1, §7.3 (X1–X3) e §7.6; `AGENTS.md` §5.7 e §8
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). **Três perguntas**; `LNET-03` continua `aberta` no registro, mas a
regra `R6`/`D9` e `ux_session_viva_por_conta` já fixam uma sessão por conta e a recusa da segunda; o
transporte só implementa. Nada foi aplicado. **Nenhum número desta página é medição**: não há conexões,
latência nem carga medidas; todo limite numérico é decisão proposta.

## 1. Fronteira

**Fecha:** limites por conexão/timeout/rate-limit, versão do protocolo e observabilidade do transporte —
`LNET-01`, `LNET-02` e `LNET-04`. **Não fecha:** `LNET-03` (modelo de sessão), ainda `aberta` no registro, embora a regra já esteja
decidida por `R6`/`D9`/`ux_session_viva_por_conta`; o transporte só a implementa.

**Não decide:** byte/opcode/ofuscação (`wyd-protocol`), regra de jogo (`wyd-core`/`wyd-sim`), processo
por canal (`wyd-server`), sessão persistida (`wyd-db`).

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| lacunas do sistema | **4 abertas, 0 propostas, 0 fechadas** | `lacunas.md` §1 |
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

### Q-NET-2 — A versão do protocolo é negociada ou imposta pelo build?

**Medido.** `LNET-02` depende de `LPRT-09` (build-alvo), que segue aberta; não há versão de protocolo
negociada nem build fixado.

**O que está em jogo.** Negociação permite vários builds; impor o build simplifica o handshake, mas
quebra compatibilidade. Sem build-alvo, as duas opções ficam no ar.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Impor o build-alvo no handshake, com erro tipado para build não suportado | handshake simples e testável; um build por vez |
| B | Negociar versão na sessão | aceita mais de um build; exige tabela de compatibilidade e testes por versão |
| C | Aceitar qualquer build e descobrir o layout em runtime | não bloqueia; offset ambíguo e falha silenciosa |

**Recomendação: A.** O `LPRT-09` ainda não fixou o build; quando fixar, impor é o menor contrato
verificável. **Custo:** um build suportado. **Fontes:** `lacunas.md` `LNET-02`; `LPRT-09`;
`wyd-net-1.0.0.md` §3. **Recusa B** por compatibilidade sem prova; **C** por ambiguidade.

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
métricas e revisão de log. **Fontes:** `lacunas.md` `LNET-04`; `AGENTS.md` §5.7;
`wyd-1.0.0.md` §7.3 (X10). **Recusa C** por cegueira operacional; log de payload não é alternativa
válida pela regra de segredo/PII.

## 5. Cobertura das lacunas

| Lacuna | Pergunta | Situação |
|---|---|---|
| `LNET-01` | `Q-NET-1` | aberta |
| `LNET-02` | `Q-NET-2` | aberta |
| `LNET-03` | — | aberta no registro, mas a regra `R6`/`D9` e `ux_session_viva_por_conta` fixam uma sessão por conta e recusam a segunda; `wyd-db/rascunho-accounts.md` §2 (`R6`) e §6 (`D9`); §3 proíbe limite por IP como identidade. O transporte implementa, não escolhe. |
| `LNET-04` | `Q-NET-4` | aberta |

## 6. Registro da decisão

**Aguardando resposta do dono.** `LNET-01`, `LNET-02` e `LNET-04` continuam `aberta`; `LNET-03`
segue `aberta` no registro, mas a regra de sessão já existe e não é pergunta de desenho. Nada foi
aplicado.

| Pergunta | Lacuna | Resposta |
|---|---|---|
| `Q-NET-1` | `LNET-01` | — |
| `Q-NET-2` | `LNET-02` | — |
| `Q-NET-4` | `LNET-04` | — |
