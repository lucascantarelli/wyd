> Pergunta: "qual plano eu leio, e qual vale agora?"
> Público: Produto, arquitetura e operação do `wyd-db`
> Dono: equipe do `wyd-db`
> Base: `wyd-db-1.0.0.md` §6.9; `proposta-paginas.md` §8; `docs/convencoes.md`
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

Plano é **decisão de como fazer**, não decisão de arquitetura: ele diz a ordem, o critério de saída e
quem executa, e não substitui o dono do schema, da migration ou do requisito. Esta página é o mapa
dos nove planos da pasta e responde qual deles governa a fase atual.

## 1. Os nove planos

| Plano | Pergunta | Estado |
|---|---|---|
| [`plano-de-documentacao.md`](plano-de-documentacao.md) | como esta documentação é produzida? | escrito |
| `plano-de-testes.md` | como provamos o banco? | recorte (§5) |
| [`plano-de-implantacao.md`](plano-de-implantacao.md) | como isto entra no ar? | escrito |
| [`plano-de-dados-e-seeds.md`](plano-de-dados-e-seeds.md) | que dado inicial existe? | escrito |
| `plano-de-manutencao.md` | o que fazer toda semana/mês? | não escrito |
| [`plano-de-qualidade-e-gates.md`](plano-de-qualidade-e-gates.md) | quando uma mudança é aceita? | escrito |
| `metricas.md` | quanto já está pronto? | não escrito |
| [`riscos.md`](riscos.md) | o que pode dar errado? | escrito |
| [`registro-de-execucao.md`](registro-de-execucao.md) | o que já foi entregue? | escrito |

Seis páginas estão escritas e três não: caminho em `code` não é link. O estado de recorte ou de
não escrita é o de `proposta-paginas.md` §5–§6; esta página só o reflete.

## 2. Qual plano é vigente

**Hoje, o plano vigente é o de documentação** — `plano-de-documentacao.md`, escrito em 2026-11-22 —,
porque a fase é **planejamento**, não execução da série (`wyd-1.0.0.md` §4). A fila executável
continua em `proposta-paginas.md` §8, que distribui as páginas em lotes; a página de documentação é a
timeline única (`D-A`, fechada em 2026-11-19).

| Fase | Plano que governa | Por quê |
|---|---|---|
| planejamento (2026-11-22) | `plano-de-documentacao.md` + `proposta-paginas.md` §8 | a entrega é página, não schema aplicado |
| execução da série | [`plano-de-implantacao.md`](plano-de-implantacao.md), [`plano-de-dados-e-seeds.md`](plano-de-dados-e-seeds.md) e [`plano-de-qualidade-e-gates.md`](plano-de-qualidade-e-gates.md) | ordem, dado inicial e aceite |
| operação | `plano-de-manutencao.md` | rotina, vacuum, backup e crescimento |

Nenhum plano é `aceito`: todos são `proposto`/`P`. A fase muda por decisão do dono, não por esta
tabela.

## 3. Como um plano é aprovado e revisado

| Etapa | Regra | Fonte |
|---|---|---|
| escrita | o plano nasce `proposto`, com data e pergunta no cabeçalho | `docs/convencoes.md` §Estados |
| aprovação | o dono diz “sim”; o estado passa a `aceito` com a data | `docs/convencoes.md`; `AGENTS.md` §7 |
| revisão | muda o corpo e a data de `Revisado em`; o histórico fica no git | `docs/convencoes.md` |
| decisão estrutural | se muda schema, fronteira ou operação, vira ADR; plano não é ADR | `AGENTS.md` §7; `wyd-db-1.0.0.md` §6.4 |
| defeito | entra em [`riscos.md`](riscos.md), dono do registro | `docs/convencoes.md` §Quando a página está errada |

## 4. O que esta página não cobre

- **A fila de escrita e o estado de cada página:** é de `proposta-paginas.md` §8 e §4–§6.
- **O aceite de uma mudança:** é de [`plano-de-qualidade-e-gates.md`](plano-de-qualidade-e-gates.md).
- **O risco e o defeito:** são de [`riscos.md`](riscos.md).
- **A contagem de páginas e o percentual:** são de `metricas.md` (planejado) e do censo do site.
- **O quadro de fechamento com ID, dono e desbloqueio:** é de `pendencias-de-fechamento.md`, na raiz de
  `planejamento-1.0.0/wyd-db/`; esta página só aponta os planos.
