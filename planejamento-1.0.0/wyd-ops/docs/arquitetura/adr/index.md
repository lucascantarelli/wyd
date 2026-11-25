# Índice de ADRs — projeto WYD 1.0.0

> Pergunta: "quais decisões estruturais estão registradas como ADR e como propor uma nova?"
> Público: Arquitetura, desenvolvimento e revisores
> Dono: arquitetura (com o dono do sistema afetado)
> Base: `AGENTS.md` §3, §5 (invariante 3); `planejamento-1.0.0/wyd-db/docs/arquitetura/adr/index.md`; `planejamento-1.0.0/wyd-ops/auditoria-de-organizacao.md`; `planejamento-1.0.0/wyd-1.0.0.md` §8
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). **15 ADRs aceitos** hoje; o restante das decisões vive em
`lacunas.md`/`docs/` do sistema dono e em `decisoes-registradas.md`.

## 1. Regra de ouro (AGENTS.md §5, item 3)

ADR só existe quando uma decisão foi **tomada e aprovada pelo dono**. Decisão sem "sim" do dono é
`proposta`, não `aceito`, e não vira ADR. ADR não documenta prática do legado nem hipótese.
Numeração segue o sistema dono: `ADR-<SISTEMA>-NNNN` (ex.: `ADR-DB-0001`).

## 2. ADRs aceitos

| ID | Título | Sistema | Status | Arquivo |
|---|---|---|---|---|
| `ADR-DB-0001` | Provisionamento de papel fora da série | `wyd-db` | aceito (2026-11-22) | `planejamento-1.0.0/wyd-db/docs/arquitetura/adr/adr-db-0001-provisionamento-de-papel-fora-da-serie.md` |
| `ADR-DB-0002` | Runner de migrations próprio | `wyd-db` | aceito (2026-11-22) | `planejamento-1.0.0/wyd-db/docs/arquitetura/adr/adr-db-0002-runner-de-migrations-proprio.md` |
| `ADR-NET-0001` | Política de limites (Q-NET-1 = A) | `wyd-net` | aceito (2026-11-25) | `planejamento-1.0.0/wyd-net/docs/arquitetura/adr/adr-net-0001-politica-de-limites.md` |
| `ADR-CORE-0001` | Decisão C4 | `wyd-core` | aceito (2026-11-25) | `planejamento-1.0.0/wyd-core/docs/arquitetura/adr/adr-core-0001-decisao-c4.md` |
| `ADR-SRV-0001` | Captura S7 (peer de A1) | `wyd-server` | aceito (2026-11-25) | `planejamento-1.0.0/wyd-server/docs/arquitetura/adr/adr-srv-0001-captura-s7.md` |
| `ADR-ADM-0001` | Canal de controle HTTP síncrono com ack | `wyd-admin` | aceito (2026-11-25) | `planejamento-1.0.0/wyd-admin/docs/arquitetura/adr/adr-adm-0001-canal-controle.md` |
| `ADR-POR-0001` | Leilão: BFF fecha, wyd-ops agenda/alerta | `wyd-portal` | aceito (2026-11-25) | `planejamento-1.0.0/wyd-portal/docs/arquitetura/adr/adr-por-0001-leilao-bff.md` |
| `ADR-DOC-0001` | Publicação é ato à parte | `wyd-docs` | aceito (2026-11-25) | `planejamento-1.0.0/wyd-docs/docs/arquitetura/adr/adr-doc-0001-publicacao-ato-a-parte.md` |
| `ADR-CLI-0001` | Uso e distribuição (Q-IP-3..Q-IP-6) | `wyd-client` | aceito (2026-11-25) | `planejamento-1.0.0/wyd-client/docs/arquitetura/adr/adr-cli-0001-uso-distribuicao.md` |
| `ADR-OPS-0001` | Workspace Cargo único, toolchain e versões (D-05 = A) | `wyd-ops` | aceito (2026-11-25) | `planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/adr-ops-0001-workspace-cargo-unico.md` |
| `ADR-OPS-0002` | Hospedagem de produção em VPS paga com Compose (Q-OPS-2 = A) | `wyd-ops` | aceito (2026-11-25) | `planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/adr-ops-0002-hospedagem-producao.md` |
| `ADR-LCH-0001` | Manifesto canônico com assinatura Ed25519 destacada (`Q-LCH-1` + `Q-LCH-3` = A) | `wyd-launcher` | aceito (2026-11-23) | `planejamento-1.0.0/wyd-launcher/docs/arquitetura/adr/adr-lch-0001-manifesto-canonico-assinado.md` |
| `ADR-SIM-0001` | Aritmética inteira em bps, arredondamento único (`Q-SIM-4` = A) | `wyd-sim` | aceito (2026-11-23) | `planejamento-1.0.0/wyd-sim/docs/arquitetura/adr/adr-sim-0001-aritmetica-inteira-em-bps.md` |
| `ADR-PRT-0001` | Build-alvo é o cliente 7662 (`Q-PRT-1` = A) | `wyd-protocol` | aceito (2026-11-23) | `planejamento-1.0.0/wyd-protocol/docs/arquitetura/adr/adr-prt-0001-build-alvo-7662.md` |
| `ADR-BOT-0001` | Ambiente de bot em CI efêmero, com servidor e seed descartáveis (`Q-BOT-1` + `Q-BOT-2` = A) | `wyd-bots` | aceito (2026-11-23) | `planejamento-1.0.0/wyd-bots/docs/arquitetura/adr/adr-bot-0001-ambiente-ci-efemero.md` |

## 2.1 ADRs propostos (rascunhos — não aceitos)

Nenhum. Os 7 candidatos foram formalizados como `aceito` em 2026-11-25 (ver §2).

## 3. Como propor um ADR

1. Abrir issue com template `decisao.yml` (decisão, alternativas, impacto).
2. Dono do sistema aprova ou rejeita.
3. Se aprovada: criar `planejamento-1.0.0/<sistema>/docs/arquitetura/adr/adr-<sistema>-NNNN-<slug>.md`
   usando `template.md` (neste diretório).
4. Atualizar este índice e `decisoes-registradas.md`.
5. Fechar a lacuna correspondente em `lacunas.md` com data e evidência.

## 4. Candidatos a ADR (formalizados em 2026-11-25)

Os 7 candidatos abaixo foram aprovados pelo dono em 2026-11-25 e formalizados como ADR (ver §2).
Além deles, `D-05` e `Q-OPS-2` foram formalizados em `ADR-OPS-0001` e `ADR-OPS-0002`. Não há candidato
pendente.

| ID de decisão | Sistema | Resumo | Onde vive |
|---|---|---|---|
| `Q-NET-1` = A | `wyd-net` | política de limites | `wyd-net/docs/proposta-decisoes-net.md` |
| `C4` | `wyd-core` | (decidida 2026-11-23) | `wyd-core-1.0.0.md` |
| `S7` (captura) | `wyd-server` | peer de A1, implementação pendente | `wyd-server-1.0.0.md` |
| `LAC-ADM-03` | `wyd-admin` | canal de controle HTTP síncrono com ack | `wyd-admin/docs/arquitetura/contrato-c4.md` |
| `LAC-POR-09` | `wyd-portal` | BFF fecha leilão; wyd-ops agenda/alerta | `wyd-portal/docs/...` |
| `LPDOC-08` | `wyd-docs` | publicação é ato à parte (arquitetura aprova, docs publica) | `wyd-docs/lacunas.md` |
| `Q-IP-3`–`Q-IP-6` | `wyd-client` | uso/distribuição | `wyd-client/docs/specs/propriedade-intelectual-e-distribuicao.md` |

## 6. Cruzamento com auditoria e especificações

| ADR | Achado da auditoria | Seção de especificação | Evidência |
|---|---|---|---|
| `ADR-DB-0001`, `ADR-DB-0002` | `wyd-db` é referência do checklist e tem fronteira/contratos fechados | `wyd-1.0.0.md` §8.7; `wyd-db/docs/arquitetura/*` | migrations e runner próprios aceitos |
| `ADR-NET-0001` | `wyd-net` tem contratos com 6 decididos; `LNET-01` continua medição | `wyd-1.0.0.md` §8.7; `wyd-net/docs/proposta-decisoes-net.md` | política de limites registrada |
| `ADR-CORE-0001` | `wyd-core` tem fronteira e contratos fechados; C4 decidida | `wyd-1.0.0.md` §8.7; `wyd-core/docs/*` | decisão C4 registrada |
| `ADR-SRV-0001` | `wyd-server` tem fronteira e contratos fechados; peer de captura pendente | `wyd-1.0.0.md` §8.7; `wyd-server/docs/specs/endpoint-de-captura.md` | captura S7 registrada |
| `ADR-ADM-0001` | `wyd-admin` é referência do checklist | `wyd-1.0.0.md` §8.7; `wyd-admin/docs/arquitetura/contrato-c4.md` | canal de controle registrado |
| `ADR-POR-0001` | `wyd-portal` tem fronteira e contratos fechados | `wyd-1.0.0.md` §8.7; `wyd-portal/docs/*` | leilão BFF registrado |
| `ADR-DOC-0001` | `wyd-docs` tem site implementado e gate de publicação | `wyd-1.0.0.md` §8.7; `wyd-docs/lacunas.md` | publicação é ato à parte |
| `ADR-CLI-0001` | `wyd-client` tem fronteira e contratos fechados; direitos parciais | `wyd-1.0.0.md` §8.7; `wyd-client/docs/specs/*` | uso e distribuição registrados |

A auditoria (`auditoria-de-organizacao.md` §2) declara 13 sistemas organizados, com `wyd-ops` mantendo
fronteira/contratos parciais. Nenhum ADR fecha por si só uma lacuna de implementação: a lacuna continua
registrada em `lacunas.md` do sistema até a evidência existir.

## 7. O que esta página não cobre

- **Decisões `propostas`** (sem aprovação): vivem em `lacunas.md`/`docs/` do sistema dono e em
  `decisoes-registradas.md`.
- **Regra de negócio ou schema**: são de `specs/` e do dicionário.