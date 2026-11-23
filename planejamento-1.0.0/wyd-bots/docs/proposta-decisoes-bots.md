# Decisões do `wyd-bots` — as quatro lacunas abertas

> Pergunta: "em que ambiente o bot roda, como nasce o dado sintético, qual limiar reprova a carga e o
> que uma falha entrega?"
> Público: QA, arquitetura e `wyd-ops`
> Dono: equipe do `wyd-bots`
> Base: `lacunas.md` (`LBOT-01`–`LBOT-04`); `wyd-bots-1.0.0.md` §3–§6; `docs/index.md`;
> `wyd-1.0.0.md` §7.1, §7.3 (X13) e §7.6; `AGENTS.md` §8
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). Quatro perguntas, uma por lacuna aberta. Nenhuma está respondida e
nada foi aplicado. **Não há medição de carga, conexão simultânea nem latência**; o único número desta
página é a meta de 85% de `AGENTS.md` §8, que é regra existente, não medição. Qualquer limiar numérico
é decisão proposta.

## 1. Fronteira

**Fecha:** ambiente do bot, origem da conta/personagem sintéticos, critério de carga e artefato de
falha — `LBOT-01` a `LBOT-04`.

**Não decide:** codec e fio (`wyd-protocol`), socket e rate-limit (`wyd-net`), regra de jogo
(`wyd-core`/`wyd-sim`), provisionamento e métrica de produção (`wyd-ops`).

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| lacunas do sistema | **4 abertas, 0 propostas, 0 fechadas** | `lacunas.md` §1 |
| arquivos de código | **0** (só `.gitkeep`) | `wyd-bots-1.0.0.md` §4 |
| cenários escritos | **0** | idem |
| testes de carga medidos | **0** | idem |
| contrato de codec em modo cliente | **decidido** | `wyd-bots-1.0.0.md` §3 (B1) |
| teste de dois canais / E2E | o `wyd-bots` é o próprio E2E; nenhum existe | `wyd-1.0.0.md` §7.6 |

## 3. Já decidido — não reabrir

| Item | Decisão vigente | Fonte | Efeito |
|---|---|---|---|
| codec em modo cliente | o bot usa o mesmo `encode`/`decode`, sem atalho de teste | `wyd-bots-1.0.0.md` §3 B1 | a lacuna não é de codec |
| cobertura e anti-skip | 85% por sistema, sem `SKIP` silencioso | `AGENTS.md` §8 | o relatório de falha não pode esconder cenário |

## 4. Decisões

### Q-BOT-1 — Onde o bot roda?

**Medido.** Não há ambiente de bot decidido nem servidor completo; a lacuna inclui CI efêmero,
homologação e máquina local, e o plano exige que nenhum bot rode em produção sem autorização.

**O que está em jogo.** CI efêmero é reproduzível, mas precisa de um servidor completo; homologação é
mais próxima do real, mas compartilhada; máquina local não é reproduzível.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | CI efêmero, com servidor completo descartável por execução | reproduzível e sem risco de produção; depende de o `wyd-server` existir no job, o que ainda não acontece |
| B | Homologação dedicada | disponível antes do servidor em CI; exige isolamento e limpeza, e o resultado pode variar com o ambiente |
| C | Máquina local do QA | começa sem infraestrutura; não é reproduzível nem serve de gate |

**Recomendação: A.** O bot é E2E e precisa de ambiente descartável; até o servidor caber no CI, B
pode ser paliativo declarado, não a decisão final. **Custo:** job de CI mais caro e dependência do
`wyd-server`. **Fontes:** `lacunas.md` `LBOT-01`; `wyd-bots-1.0.0.md` §3 B2 e §6; `AGENTS.md` §9
(runner hospedado, sem self-hosted em PR). **Recusa C** por não ser reproduzível; **B** por não ser o
alvo.

### Q-BOT-2 — Como nasce a conta e o personagem sintéticos?

**Medido.** Não há fixture, seed de bot nem conta de teste; o plano exige "fixture, não conta real".
`M19` (senha de dev) está **fechada** em 2026-11-19, mas `seeds/dev.sql` ainda não existe; os seis
seeds de conteúdo seguem bloqueados.

**O que está em jogo.** Conta real ou banco compartilhado produz teste não reprodutível e risco de
dado pessoal; fixture em banco efêmero isola, mas precisa de reset e de um caminho de criação.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Fixture/seed sintético no banco efêmero do cenário, com reset por execução | nenhum dado de jogador; reproduzível; exige um seed de bot e um reset barato |
| B | Criar conta e personagem pela API do portal | exercita o fluxo real; depende do portal e deixa estado persistido se o reset falhar |

**Recomendação: A.** O contrato B3 já diz "fixture, não conta real"; a decisão é onde a fixture mora
e como reseta. **Custo:** um seed e um reset a manter. **Fontes:** `lacunas.md` `LBOT-02`;
`wyd-bots-1.0.0.md` §3 B3; `AGENTS.md` §5.7; `M19` em `lacunas.md` do `wyd-db`. **Conta real não é
opção**: contraria B3. **Recusa B** por acoplar o bot ao portal.

### Q-BOT-3 — Qual limiar de carga reprova?

**Medido.** Não há conexões simultâneas, duração, latência nem taxa de sucesso medidas; o plano diz
que "limiar é medido, não estimado" e `AGENTS.md` §8 exige gate medido.

**O que está em jogo.** Fixar um número agora é promessa sem fonte; não ter critério nenhum deixa a
carga sem gate. A decisão é **como** o número entra, não qual é.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Medir primeiro em ambiente controlado; nenhum limiar numérico entra no gate até a medição | nenhum número inventado; a carga fica sem reprovação até a primeira medição, que vira o gate |
| B | Adotar um teto provisório de recurso do ambiente (conexões que o processo suporta) | dá um gate imediato; o teto é de infraestrutura, não de comportamento do jogo |
| C | Fixar agora conexões/latência alvo | permite escrever o teste; o número não tem fonte e vira promessa |

**Recomendação: A.** O número é decisão, não medição; a primeira execução mede e o resultado vira o
limiar, com data. **Custo:** a carga não reprova antes da medição. **Fontes:** `lacunas.md` `LBOT-03`;
`wyd-bots-1.0.0.md` §3 B4 e §6; `AGENTS.md` §8. **Recusa C** por inventar número; **B** por medir
infraestrutura, não o cenário.

### Q-BOT-4 — O que uma execução falha entrega?

**Medido.** Não há relatório, log, captura nem trace de bot; o gate do plano pede "relatório por
execução com falha reproduzível (log + captura)" e `AGENTS.md` §8 proíbe `SKIP` silencioso.

**O que está em jogo.** Sem artefato, a falha não é reproduzível e o custo do teste não se paga; log
com payload ou dado de jogador pode carregar segredo/PII e conflita com a higiene de `AGENTS.md` §5.7
e com a fronteira de PII do `wyd-db` (`P-22`).

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Log estruturado + trace da conexão + frame/captura anonimizada no ponto da falha | falha reproduzível; exige scrubbing e armazenamento fora do git |
| B | Só log textual | simples; não prova o estado do fio nem reproduz a falha |
| C | Nada além do exit code | mínimo; não há como diagnosticar |

**Recomendação: A.** O gate já pede log + captura; a decisão é o formato e a higiene. **Custo:**
armazenamento e revisão de PII. **Fontes:** `lacunas.md` `LBOT-04`; `wyd-bots-1.0.0.md` §6;
`AGENTS.md` §5.7 e §8; `wyd-db/padrao-banco.md` `P-22`. **Recusa B/C** por não reproduzirem a falha.

## 5. Cobertura das lacunas

| Lacuna | Pergunta | Situação |
|---|---|---|
| `LBOT-01` | `Q-BOT-1` | aberta |
| `LBOT-02` | `Q-BOT-2` | aberta; conta real não é opção (B3) |
| `LBOT-03` | `Q-BOT-3` | aberta |
| `LBOT-04` | `Q-BOT-4` | aberta |

## 6. Registro da decisão

**Aguardando resposta do dono.** Nenhuma das quatro foi respondida; a linha de cada lacuna continua
`aberta` em `lacunas.md`. O registro entra aqui com data, opção e arquivo afetado.

| Pergunta | Lacuna | Resposta |
|---|---|---|
| `Q-BOT-1` | `LBOT-01` | — |
| `Q-BOT-2` | `LBOT-02` | — |
| `Q-BOT-3` | `LBOT-03` | — |
| `Q-BOT-4` | `LBOT-04` | — |
