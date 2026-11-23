> Pergunta: "por onde começo a ler o planejamento do portal e o que cada página responde?"
> Público: Desenvolvedor novo, produto, QA e operação
> Dono: arquitetura
> Base: `wyd-portal-1.0.0.md`; `lacunas.md`; `docs/arquitetura/`; `docs/specs/`; `docs/roadmap/`
> Revisado em: 2026-11-20

**Estado:** aceito (2026-11-20)

O `wyd-portal` é a aplicação Next.js que serve o portal público e a área administrativa do WYD
`1.0.0`. Este índice é a porta de entrada do planejamento; o plano diretor mede o estado, e as
páginas abaixo respondem perguntas específicas.

## 1. Ordem de leitura

| Se você quer… | Leia |
|---|---|
| saber o escopo e o estado medido | `wyd-portal-1.0.0.md` (na raiz do sistema, fora deste site) |
| entender o limite com `wyd-db` e `wyd-admin` | `arquitetura/fronteira.md` |
| ver telas, rotas e quem pode abrir | `arquitetura/rotas-e-permissoes.md` |
| saber o que cada tela lê e escreve | `arquitetura/consumo-do-banco.md` |
| saber de onde vêm CPU/RAM/latência, logs, painéis e telemetria do launcher | `arquitetura/monitoramento.md` |
| ver requisitos com critério de aceitação | `specs/requisitos.md` |
| entender o que o prompt pediu e o que virou decisão | `specs/analise-do-prompt.md` |
| saber a ordem de construção e o DoD | `roadmap/roadmap.md` |
| ver o que ainda não está decidido | `lacunas.md` (na raiz do sistema, fora deste site) |

## 2. O que este planejamento assume

- O escopo foi decidido em 2026-11-20 (`M21`): portal público **e** área administrativa, com o
  administrador configurando o servidor pelo portal.
- O banco é o do `wyd-db` (`1.0.0`, 34 arquivos e 109 tabelas planejadas); este plano **não** cria
  tabela.
- O monitoramento (seis telas de staff) lê Prometheus, Loki e Grafana do `wyd-ops` por HTTP e a
  presença em `game.v_live_session`; nada de série, log ou evento vira tabela no banco.
- A autenticação do portal é a de `portal.person` (`0003`/`0004`); a sessão de jogo é outra coisa.
- O prompt recebido (`../wyd-portal-prompt.md`) é orientação de produto (`H`), lida em
  `specs/analise-do-prompt.md`.

## 3. O que **não** está aqui

| Fora | Onde está |
|---|---|
| DDL, migrations e permissões de banco | `planejamento-1.0.0/wyd-db/` |
| regra de jogo (dano, drop, progressão) | `wyd-core`/`wyd-sim` (a planejar) |
| API interna de staff | `wyd-admin` (a planejar) |
| patch e manifesto do cliente | `wyd-launcher`/`wyd-ops` |
| site de documentação | `wyd-docs` |

## 4. Convenções

- Toda página tem cabeçalho de cinco campos e linha `**Estado:**` com data.
- Afirmação leva `V` (medido), `H` (hipótese com ID de lacuna) ou `P` (planejado); a marca cola na
  frase.
- Lacuna só existe em `lacunas.md`, com ID e dono.
- Número sem fonte e data não entra.

*Página de planejamento. Nenhum código foi escrito.*
## Decisão registrada

`LAC-POR-09` foi fechada em 2026-11-23: o BFF do portal fecha o leilão vencido e o `wyd-ops` agenda e
alerta. A comparação das alternativas e o comportamento em falha estão em
[Cobertura do `wyd-portal`](proposta-decisoes-portal.md); o estado da lacuna continua em `lacunas.md`.
O portal está em **0 abertas, 0 propostas e 9 fechadas**.
