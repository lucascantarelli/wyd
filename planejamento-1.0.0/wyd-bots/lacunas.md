# `wyd-bots` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-23).** Único lugar onde uma lacuna do `wyd-bots` tem dono,
> impacto e estado. `LBOT-04` fechou em 2026-11-23 por **regra vigente** (`Q-BOT-4` = A derivada),
> sem escolha do usuário, e `LBOT-01`/`LBOT-02` fecharam na mesma data pelas respostas do dono
> (`Q-BOT-1` = A: CI efêmero; `Q-BOT-2` = A: seed declarativo em `crates/wyd-db/seeds/dev.sql`).
> `Q-BOT-3` = A fixou a **política** do limiar em 2026-11-23; `LBOT-03` continua **bloqueada como medição**
> do valor final: não há número, conexão nem latência medidos, e a **autorização de execução** da medição
> — com o ambiente efêmero de `OPS-03.1` — foi **autorizada em 2026-11-25**, mas o ambiente ainda depende da aprovação de `OPS-03.1` na
> [`pauta de decisões`](../wyd-ops/docs/pauta-de-decisoes-pendentes.md) (dono: QA + `wyd-ops`). Nada foi
> aprovado em 2026-11-25 além do aceite da opção A e da autorização de execução: o que existe é a política de 2026-11-23, não a medição. A pauta em `../wyd-ops/docs/pauta-de-decisoes-pendentes.md` §2 e os documentos donos registram as
> respostas.

## 0. Como ler

Estados: **fechada** (com evidência citada) e **bloqueada** (falta decisão, execução ou evidência; a
linha nomeia o desbloqueio — dono e documento). `arquitetura` propõe; o dono do
sistema aprova.

## 1. Situação em uma linha

| Grupo | Lacunas | Bloqueadas | Fechadas |
|---|---:|---:|---:|
| Ambiente | 1 | 0 | 1 |
| Dado | 1 | 0 | 1 |
| Medição | 1 | 1 | 0 |
| Relatório | 1 | 0 | 1 |
| **Total** | **4** | **1** | **3** |

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `LBOT-01` | Em que ambiente o bot roda: efêmero do CI, homologação ou máquina local? | bot em produção simula jogador real e distorce métrica; bot em CI efêmero precisa de servidor completo, que ainda não existe | `wyd-ops` + arquitetura | fechada (2026-11-23: decisão do dono, `Q-BOT-1` opção **A** — o ambiente alvo é o **job de CI em runner hospedado**, com Postgres e servidor descartáveis por execução e sem estado entre cenários; o contrato B2 passa a `CI efêmero` e o replay pode ser gate de PR; homologação fica só como paliativo declarado, com data e dono, até a fatia do `wyd-server` caber no job; não implementa o job, o servidor, a fixture nem o bot; registro em `docs/proposta-decisoes-bots.md` §4 e §6) |
| `LBOT-02` | Como nasce a conta e o personagem sintético de cada cenário? | conta real ou banco compartilhado produz teste não reprodutível e risco de dado pessoal | `wyd-db` + QA | fechada (2026-11-23: decisão do dono, `Q-BOT-2` opção **A** — **seed declarativo** em `crates/wyd-db/seeds/dev.sql`, aplicado depois das migrations no banco recriado por execução; o seed grava 5 linhas em 5 tabelas (`portal.person`, `game.account`, `content.content_version`, `game.character`, `game.server_channel`), com hash Argon2id de `dev-only`; não é migration, a série de 34 arquivos e o `SCHEMA_VERSION` não mudam, os seis seeds de conteúdo continuam bloqueados e o arquivo é dev-only (`M19`); registro em `docs/proposta-decisoes-bots.md` §4 e §6) |
| `LBOT-03` | Qual é o limiar de carga (conexões, duração, latência aceitável) e o que reprova? | limiar sem medição vira promessa; `AGENTS.md` §8 exige gate medido | QA + `wyd-ops` | **bloqueada como medição** (2026-11-25: política `Q-BOT-3` = A e autorização de execução dada; a execução de prontidão encontrou **0** `Cargo.toml` em `crates/wyd-bots`, **0** cenário e **0** listener, então não há P50/P99 nem taxa de sucesso para congelar; tarefa em `../wyd-ops/docs/planos/medicoes-lnet-01-lbot-03.md` §4.6; pergunta consolidada em `../wyd-ops/docs/lacunas.md` §4, item 7) |
| `LBOT-04` | O que uma execução falha entrega: log, captura, trace ou nada? | sem artefato a falha não é reproduzível e o custo do teste não se paga | QA | fechada (2026-11-23: **derivada das regras vigentes, sem escolha do usuário** — o gate do sistema já exige “log + captura” por execução falha e proíbe segredo de jogador (`wyd-bots-1.0.0.md` §6, itens 4–5), e `AGENTS.md` §5, item 7 e `P-22` restringem o artefato a metadados/frame anonimizado; registro em `docs/proposta-decisoes-bots.md` §4 e §6) |

## 3. O que não é lacuna deste sistema

- Codec em modo cliente: **decidido** (`wyd-protocol` §1.4).
- Provisionamento de ambiente: dono é `wyd-ops`.
---

## 4. Bloco de decisão da lacuna bloqueada

A linha `LBOT-03` em §2 não muda de estado até o artefato de fechamento existir; o que segue é a
proposta da arquitetura, **aceita pelo dono em 2026-11-25 na opção A** (`AGENTS.md` §5, item 8).

**LBOT-03 — limiar de carga.**
- **Dono da decisão:** QA + `wyd-ops`, com aprovação do dono do sistema; o método está em
  `planejamento-1.0.0/wyd-ops/docs/planos/medicoes-lnet-01-lbot-03.md` §4.
- **Pergunta fechada:** qual conjunto de números a execução de carga congela — conexões simultâneas,
  duração, P50/P99 de ida e volta, taxa de sucesso e recusas por motivo — e qual valor reprova o job?
- **Opções:** (A) **limiar absoluto por canal** — conexões e duração fixas, P99 máximo e taxa de
  sucesso mínima, com o job reprovando qualquer degrau abaixo do congelado; custo: o número precisa ser
  re-medido quando o hardware do runner mudar; (B) **limiar relativo ao baseline da própria execução** —
  reprova quando P99 passa de um múltiplo do baseline medido na mesma corrida; custo: não detecta
  regressão global e não serve de promessa ao jogador.
- **Recomendação:** A, porque promessa de capacidade precisa de número absoluto e datado; o relativo
  serve de alerta, não de critério.
- **Aceite do dono (2026-11-25):** opção **A** aprovada — limiar absoluto por canal, com o job reprovando qualquer degrau abaixo do congelado; a execução está autorizada e depende do ambiente de `OPS-03.1`.
- **Artefato que falta (o que fecha):** execução datada preenchendo `medicoes-lnet-01-lbot-03.md` §4.6 com os valores
  congelados e um job de CI que reprova abaixo deles; a linha `LBOT-03` passa a citar a execução.
