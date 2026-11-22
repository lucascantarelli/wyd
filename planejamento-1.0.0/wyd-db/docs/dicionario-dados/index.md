> Pergunta: "por onde começo a ler o dicionário e qual página cobre cada assunto?"
> Público: Desenvolvedor novo no `wyd-db`
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §1; `plano-migrations.md` §3; `migrations-checklist.md` §1.2
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL fechado, série não aplicada.

O dicionário tem duas camadas: **domínio** (esta pasta) explica o assunto e as regras que atravessam
tabelas; **tabela** (`gerado/<tabela>.md`) resume uma tabela e aponta para a página de `migrations/` dona
da forma. As páginas por tabela são escritas à mão em lotes (`M26`), sem gerador; quantas das 109 já existem, e em
que lote cada uma entrou, diz [`gerado/index.md`](gerado/index.md), dono único da contagem.

## Os 19 domínios

| Domínio | Cobre | Tabelas |
|---|---|---:|
| [acesso](dominio-acesso.md) | pessoa, perfil, consentimento, sessão e token do portal, conta, carteira, reset de PIN e sessão de jogo | 9 |
| [canal](dominio-canal.md) | topologia de servidor: grupo, canal, capacidade e papel | 1 |
| [auditoria](dominio-auditoria.md) | trilha de ação, tentativa de login e registro de idempotência | 3 |
| [configuração](dominio-configuracao.md) | versão do conteúdo, revisão de configuração e ajuste por seção | 5 |
| [personagem](dominio-personagem.md) | identidade do personagem e satélites de estado | 5 |
| [progressão](dominio-progressao.md) | skills, linhagem, afetos e chaos | 5 |
| [inventário](dominio-inventario.md) | mochila, baú e piedade de refino | 3 |
| [social](dominio-social.md) | guilda, membros, relações e torre | 4 |
| [mundo](dominio-mundo.md) | mapas, rotas, NPCs, spawns, regiões e zonas | 13 |
| [catálogo](dominio-catalogo.md) | taxonomias, item e skill | 7 |
| [combate](dominio-combate.md) | stats de inimigo, equipamento e drop | 4 |
| [refino](dominio-refino.md) | refino, SANC, composição, gema, acessório e overrides | 8 |
| [economia](dominio-economia.md) | cidade, preço, loja, imposto, cash, ledger e o mercado entre jogadores | 22 |
| [quests](dominio-quests.md) | definição, passo, recompensa e progresso | 4 |
| [eventos](dominio-eventos.md) | evento, agenda, participação, Kefra e castelo | 5 |
| [arena](dominio-arena.md) | modo, partida, round e placar | 4 |
| [moderação](dominio-moderacao.md) | bloqueio de conta | 1 |
| [ponte web↔jogo](dominio-ponte-web-jogo.md) | fila de concessão do portal para o jogo | 1 |
| [conteúdo do portal](dominio-conteudo-portal.md) | notícia, banner, guia, versão do launcher e segundo fator de staff | 5 |

## Cobertura

As 19 páginas cobrem **109 tabelas distintas**, uma menção por tabela — **nenhuma fica sem domínio**.
`game.account_moderation` pertence a [moderação](dominio-moderacao.md), e [acesso](dominio-acesso.md)
apenas aponta para lá. As cinco tabelas de `0030`/`0031` passaram a ter dono com a criação do 19º
domínio, [conteúdo do portal](dominio-conteudo-portal.md), e as onze do mercado (`0034`) e a de
idempotência (`0033`) entraram em [economia](dominio-economia.md) e [auditoria](dominio-auditoria.md)
em 2026-11-22 — a segunda, por não ser tabela de dinheiro nem de conteúdo.

## Como ler

A anatomia da página, as marcas `V`/`H`/`P` e a regra da divergência estão em
[como ler o dicionário](como-ler.md).

1. comece pelo domínio do assunto que você vai mexer;
2. confira o estado do arquivo em `migrations-checklist.md` §1.2 — o dicionário não mantém escala própria;
3. para coluna, tipo, nulidade, `CHECK` e índice, vá à página de `migrations/` do arquivo dono;
4. `H` significa hipótese com o que falta, nunca fato. A marcação é de
   [`../convencoes.md`](../convencoes.md).
