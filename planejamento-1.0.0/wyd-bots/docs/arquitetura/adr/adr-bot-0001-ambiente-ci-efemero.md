# ADR-BOT-0001 — Ambiente de bot é CI efêmero, com servidor e seed descartáveis

> Pergunta: "onde o bot roda e de onde vem a conta e o personagem sintéticos?"
> Público: Arquitetura, QA e operação
> Dono: equipe do `wyd-bots` (decisão do dono em 2026-11-23)
> Base: `docs/proposta-decisoes-bots.md` §4 (`Q-BOT-1`, `Q-BOT-2`); `lacunas.md` `LBOT-01`, `LBOT-02`; `wyd-bots-1.0.0.md` §3 (B2) e §6; `AGENTS.md` §9
> Revisado em: 2026-11-23

**Estado:** aceito (2026-11-23) — aprovado pelo dono em `Q-BOT-1` = A e `Q-BOT-2` = A.

## Contexto

**Medido em 2026-11-23:** não havia ambiente de bot decidido nem servidor completo; o plano exigia que
nenhum bot rodasse em produção sem autorização. Homologação dedicada varia com carga e estado; máquina
local não é reproduzível nem serve de gate. Sem dado sintético declarado, o cenário não teria ponto de
partida estável.

## Decisão

O ambiente-alvo do bot é o **job de CI em runner hospedado**, com **Postgres e servidor descartáveis por
execução** e **sem estado entre cenários**; o contrato B2 passa de lacuna a decidido e o replay pode ser
gate de PR. A conta e o personagem sintéticos vêm de **seed declarativo** em
`crates/wyd-db/seeds/dev.sql`, aplicado no banco recriado por execução. Homologação fica reservada a
**paliativo declarado**, com data e dono, até a fatia do `wyd-server` caber no job — nunca vira decisão
final, e produção continua fora.

## Consequências

- **Positivas:** execução reproduzível, sem risco de produção e sem número medido em ambiente
  compartilhado; o replay dos 7 goldens ganha ambiente em CI.
- **Negativas/custos:** job mais caro e dependente do `wyd-server`; para rodar faltam **1 workflow** em
  `.github/workflows/` com `services: postgres`, **1 `Cargo.toml`** de workspace na raiz, código em
  **5–6 crates hoje vazios** (`wyd-protocol`, `wyd-net`, `wyd-db`, `wyd-core`, `wyd-sim`, `wyd-server`),
  **1 `seeds/dev.sql`**, **1 runner de cenário** em `crates/wyd-bots` e a captura A1
  (`LPRT-03`/`LPRT-06`).
- **Contratos afetados:** `wyd-ops` (runner e imagem), `wyd-db` (seed), `wyd-server` (servidor no job) e
  `wyd-protocol` (captura que o replay compara).

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Homologação dedicada | ambiente compartilhado varia com carga e estado; o número medido lá não vale como gate de PR |
| Máquina local do QA | não é reproduzível e não pode reprovar nada; cada troca de ambiente refaz cenário e limiar |

## Como verificar

A decisão está em vigor quando o workflow de CI subir Postgres e servidor descartáveis, aplicar
`crates/wyd-db/seeds/dev.sql` e rodar um cenário de bot sem estado entre execuções. **Hoje nenhum desses
arquivos existe** — `LBOT-01` e `LBOT-02` estão fechadas como decisão; `LBOT-03` continua bloqueada como
medição e não é fechada por esta ADR.

## Referências

- Decisões: `Q-BOT-1` = A e `Q-BOT-2` = A (2026-11-23), em `docs/proposta-decisoes-bots.md` §4.
- Lacunas fechadas: `LBOT-01`, `LBOT-02`; bloqueada: `LBOT-03`.
- Especificação: `wyd-bots-1.0.0.md` §3 (B2) e §6; `AGENTS.md` §9 (runner hospedado, sem self-hosted em PR).
