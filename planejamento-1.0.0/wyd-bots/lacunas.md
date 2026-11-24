# `wyd-bots` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-23).** Único lugar onde uma lacuna do `wyd-bots` tem dono,
> impacto e estado. `LBOT-04` fechou em 2026-11-23 por **regra vigente** (`Q-BOT-4` = A derivada),
> sem escolha do usuário, e `LBOT-01`/`LBOT-02` fecharam na mesma data pelas respostas do dono
> (`Q-BOT-1` = A: CI efêmero; `Q-BOT-2` = A: seed declarativo em `crates/wyd-db/seeds/dev.sql`).
> `Q-BOT-3` = A fixou a **política** do limiar, mas `LBOT-03` **segue aberta como medição**: não há
> número, conexão nem latência medidos. O briefing das respostas é `../briefing-de-decisoes-1.0.0.md`
> §2 e §3.

## 0. Como ler

Estados: **aberta**, **proposta**, **fechada** (com evidência). `arquitetura` propõe; o dono do
sistema aprova.

## 1. Situação em uma linha

| Grupo | Lacunas | Abertas | Propostas | Fechadas |
|---|---:|---:|---:|---:|
| Ambiente | 1 | 0 | 0 | 1 |
| Dado | 1 | 0 | 0 | 1 |
| Medição | 1 | 1 | 0 | 0 |
| Relatório | 1 | 0 | 0 | 1 |
| **Total** | **4** | **1** | **0** | **3** |

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `LBOT-01` | Em que ambiente o bot roda: efêmero do CI, homologação ou máquina local? | bot em produção simula jogador real e distorce métrica; bot em CI efêmero precisa de servidor completo, que ainda não existe | `wyd-ops` + arquitetura | fechada (2026-11-23: decisão do dono, `Q-BOT-1` opção **A** — o ambiente alvo é o **job de CI em runner hospedado**, com Postgres e servidor descartáveis por execução e sem estado entre cenários; o contrato B2 passa a `CI efêmero` e o replay pode ser gate de PR; homologação fica só como paliativo declarado, com data e dono, até a fatia do `wyd-server` caber no job; não implementa o job, o servidor, a fixture nem o bot; registro em `docs/proposta-decisoes-bots.md` §4 e §6) |
| `LBOT-02` | Como nasce a conta e o personagem sintético de cada cenário? | conta real ou banco compartilhado produz teste não reprodutível e risco de dado pessoal | `wyd-db` + QA | fechada (2026-11-23: decisão do dono, `Q-BOT-2` opção **A** — **seed declarativo** em `crates/wyd-db/seeds/dev.sql`, aplicado depois das migrations no banco recriado por execução; o seed grava 5 linhas em 5 tabelas (`portal.person`, `game.account`, `content.content_version`, `game.character`, `game.server_channel`), com hash Argon2id de `dev-only`; não é migration, a série de 34 arquivos e o `SCHEMA_VERSION` não mudam, os seis seeds de conteúdo continuam bloqueados e o arquivo é dev-only (`M19`); registro em `docs/proposta-decisoes-bots.md` §4 e §6) |
| `LBOT-03` | Qual é o limiar de carga (conexões, duração, latência aceitável) e o que reprova? | limiar sem medição vira promessa; `AGENTS.md` §8 exige gate medido | QA + `wyd-ops` | aberta (2026-11-23: decisão do dono, `Q-BOT-3` opção **A** — **medir e congelar**: a primeira execução no CI efêmero mede P50/P99, sucesso e recusas em degraus; saturação é o primeiro degrau em que as recusas deixam de ser zero ou o P99 cresce sem voltar ao patamar anterior; o valor medido vira o gate com data. Até lá o cenário é exploratório e não reprova; ao atingir o limiar congelado, o bot para de subir, drena e anexa o relatório de `LBOT-04`. **A lacuna não fecha hoje**: depende da fatia do `wyd-server`, do `Q-NET-1` e da primeira medição; 0 migration, 0 tabela, 0 coluna) |
| `LBOT-04` | O que uma execução falha entrega: log, captura, trace ou nada? | sem artefato a falha não é reproduzível e o custo do teste não se paga | QA | fechada (2026-11-23: **derivada das regras vigentes, sem escolha do usuário** — o gate do sistema já exige “log + captura” por execução falha e proíbe segredo de jogador (`wyd-bots-1.0.0.md` §6, itens 4–5), e `AGENTS.md` §5, item 7 e `P-22` restringem o artefato a metadados/frame anonimizado; registro em `docs/proposta-decisoes-bots.md` §4 e §6) |

## 3. O que não é lacuna deste sistema

- Codec em modo cliente: **decidido** (`wyd-protocol` §1.4).
- Provisionamento de ambiente: dono é `wyd-ops`.
