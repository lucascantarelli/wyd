# Índice de ADRs — projeto WYD 1.0.0

> Pergunta: "quais decisões estruturais estão registradas como ADR e como propor uma nova?"
> Público: Arquitetura, desenvolvimento e revisores
> Dono: arquitetura (com o dono do sistema afetado)
> Base: `AGENTS.md` §3, §5 (invariante 3); `planejamento-1.0.0/wyd-db/docs/arquitetura/adr/index.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Dois ADRs aceitos hoje; o restante das decisões vive em
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
Não há candidato pendente.

| ID de decisão | Sistema | Resumo | Onde vive |
|---|---|---|---|
| `Q-NET-1` = A | `wyd-net` | política de limites | `wyd-net/docs/proposta-decisoes-net.md` |
| `C4` | `wyd-core` | (decidida 2026-11-23) | `wyd-core-1.0.0.md` |
| `S7` (captura) | `wyd-server` | peer de A1, implementação pendente | `wyd-server-1.0.0.md` |
| `LAC-ADM-03` | `wyd-admin` | canal de controle HTTP síncrono com ack | `wyd-admin/docs/arquitetura/contrato-c4.md` |
| `LAC-POR-09` | `wyd-portal` | BFF fecha leilão; wyd-ops agenda/alerta | `wyd-portal/docs/...` |
| `LPDOC-08` | `wyd-docs` | publicação é ato à parte (arquitetura aprova, docs publica) | `wyd-docs/lacunas.md` |
| `Q-IP-3`–`Q-IP-6` | `wyd-client` | uso/distribuição | `wyd-client/docs/specs/propriedade-intelectual-e-distribuicao.md` |

## 5. O que esta página não cobre

- **Decisões `propostas`** (sem aprovação): vivem em `lacunas.md`/`docs/` do sistema dono e em
  `decisoes-registradas.md`.
- **Regra de negócio ou schema**: são de `specs/` e do dicionário.