> Pergunta: "por onde começo a ler o planejamento do `wyd-admin` e o que já está decidido?"
> Público: Desenvolvedor novo, arquitetura e staff técnico
> Dono: equipe do wyd-admin
> Base: `wyd-portal/docs/arquitetura/fronteira.md` §3 (C4); `AGENTS.md` §5/§6
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-21); custódia de item do mercado acrescentada em 2026-11-22. Planejamento
aberto, nenhum código escrito.

O `wyd-admin` é a **API interna de staff e de custódia de item do mercado**: o único componente
autorizado a escrever em `game` em nome do portal (`LAC-POR-01`, decidida em 2026-11-20). Ele não tem
tela, não serve HTML e não decide regra de jogo — recebe uma ação tipada, executa a escrita transacional,
audita e responde.

## Ordem de leitura

1. [Fronteira](arquitetura/fronteira.md) — o que ele faz, o que não faz, quem consome e o que quebra
   se cair.
2. [Contrato C4](arquitetura/contrato-c4.md) — rotas, payload, idempotência, erros, prazos e
   autenticação.
3. [Autenticação e controle](arquitetura/autenticacao-e-controle.md) — as duas propostas que aguardam o
   dono: token de serviço portal → admin e canal de controle com o `wyd-server`, com opções e trade-off.
4. [Ações e efeito no banco](specs/acoes.md) — o que cada ação grava, em qual tabela e sob qual
   constraint.
5. [Requisitos](specs/requisitos.md) — `RF-ADM`/`RN-ADM`/`RNF-ADM` com critério de aceitação.
6. [Componentes](arquitetura/componentes.md) — unidades de código, invariantes e testes.
7. [Plano de testes](planos/plano-de-testes.md) — alvos, isolamento e comando.
8. [Roadmap](roadmap/roadmap.md) — fatias com DoD verificável.

## O que já está decidido e o que não está

| Assunto | Estado | Dono |
|---|---|---|
| `wyd-admin` é o único executor de escrita em `game` | **decidido** (`LAC-POR-01`, 2026-11-20) | arquitetura |
| Transporte HTTP/JSON interno, com timeout, chave de idempotência e erro tipado | **decidido em direção**; forma nesta árvore | arquitetura + `wyd-admin` |
| Rotas, payload, códigos e prazos | **proposto** (2026-11-21); custódia de item em 2026-11-22 (§4.9–§4.13) | `wyd-admin` |
| Canal com o `wyd-server` e autenticação portal → admin | **propostos em 2026-11-22, aguardando o dono** (`LAC-ADM-03` e `LAC-ADM-04`) — [autenticação e controle](arquitetura/autenticacao-e-controle.md); armazenamento de idempotência, retenção e schema de custódia fecharam em 2026-11-22 | arquitetura, `wyd-ops` |

Nada aqui foi implementado: nenhuma rota responde, nenhum papel existe no `provision` e nenhuma
tabela nova foi proposta como aprovada.
