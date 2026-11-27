# Decisões dos seeds — exceção pontual ao "sem gerador" em `0017` e `0020`

> Pergunta: "os dois seeds de volume (`0017` e `0020`) podem ser produzidos por um script de passagem
> única a partir do dataset já medido, ou a regra 'seed escrito à mão, sem gerador' vale sem exceção?"
> Público: dono do `wyd-db`, arquitetura, curadoria de conteúdo e quem for escrever os seis seeds
> Dono: `wyd-db` — a decisão é do dono; esta página formula as perguntas e registra as respostas
> Base: `plano-migrations.md` §4; `docs/planos/plano-de-dados-e-seeds.md` §2 e §3;
> `pedido-de-curadoria-dos-seeds.md` §3 e §4; `migrations-checklist.md` §2 e §3; `lacunas.md` `M26` e
> `M33`; os seeds legados em `deprecated/crates/wyd-db/migrations/`; o dataset medido em
> `deprecated/.agents/prompts/análises/wyd-db/database/plan/dados/`; `AGENTS.md` §5, itens 2, 4, 5 e 8
> Revisado em: 2026-11-27

**Estado:** decidido (2026-11-27). As três perguntas foram respondidas pelo dono na opção A e as
entregas de §7 foram aplicadas nesta rodada: a exceção está datada em `plano-migrations.md` §4, as 4
cidades têm dono `0020`, e `M34` fecha em `lacunas.md` §2.3 com o placar recontado. As contagens são
medições de arquivo de 2026-11-27 (`V`); custo é medido em linhas e arquivos, não em prazo, porque o
projeto ordena por dependência e não por calendário.

## 1. Fronteira desta rodada

**Fecha:** como `0017` e `0020` são produzidos depois que a medição do 7662 voltar — à mão ou por um
script de passagem única —, sem estender a exceção aos outros quatro seeds.

**Não decide:** o conteúdo de qualquer seed; a medição do cliente 7662, que é da curadoria; o aceite
das seis páginas; as pendências de dado de §8 (acessório, `kind_slug`, divergências de contagem); e o
conteúdo dos quatro seeds curados (`0016`, `0018`, `0023`, `0027`).

**Não reabre:** a fonte dos seis ser o cliente 7662 medido (2026-11-23); os seis ficarem fora da onda
A (`Q-M33-6`); "ausência é dado" e "contagem que não fecha vem com a explicação e a diferença"
(`pedido-de-curadoria-dos-seeds.md` §4); a proibição de valor estimado; e a regra de corrigir dado na
origem, nunca em migration de correção (`plano-migrations.md` §4).

## 2. O que a medição permite afirmar (`V`, 2026-11-27)

| Fato | Medição | Fonte |
|---|---|---|
| seeds ausentes | **6** (`0016`–`0018`, `0020`, `0023`, `0027`) | `migrations-checklist.md` §2 |
| `CREATE TABLE` necessário | **0** — as 31 tabelas alvo já existem em `0013`/`0014`/`0015`/`0019`/`0021`/`0022`/`0026` | `crates/wyd-db/migrations/*.sql` |
| `INSERT` a escrever | **31**, um por tabela alvo | medição dos seeds legados |
| linhas de conteúdo | **18.800** | idem |
| por arquivo | `0016` 55 · `0017` 3.275 · `0018` 154 · `0020` 15.142 · `0023` 158 · `0027` 16 | idem |
| volume nos dois grandes | **18.417 linhas — 98% do total** | aritmética da linha acima |
| proveniência já medida | **12 fontes** com `path`, `sha256`, `size_bytes` e `row_count` | `.../dados/content-source.json` |
| cliente medido | **95 arquivos** com `sha256` | `.../dados/manifest.json` |
| `content_source` | criada no `0019`; proveniência de `0016`–`0018` tem de ser gravada a partir do `0020` | `0019_content_world.sql`; `0020_seed_mundo.sql` |
| colunas do item | seed legado grava 25; o schema novo aceita 11 | `0018_seed_item_catalog.sql`; `0014_content_item.sql` |
| skills no cliente | **124** registros não vazios para 151 escritas + 3 Kersef | `resumo.json`; `0024_seed_skill_catalog_kersef.sql` |
| itens no cliente | **3.216** slots ocupados para 3.220 no catálogo | `resumo.json` |
| cidades | a tabela existe — `game.city` em `0011` — e **nenhum dos seis seeds a escreve**; o legado gravava 4 linhas | `crates/wyd-db/migrations/0011_game_economy.sql`; `0023_seed_mundo_fixo.sql` |
| acessório | regra é sistema posterior ao 7662 (`H`) | `0038_seed_acessorio_refino.sql` |

**Não há volume a estimar.** 98% do trabalho dos seis seeds é projeção de linha que já está medida e
com `sha256`; o que não está medido é o dado que a curadoria ainda não entregou. O risco da rodada,
portanto, não é "inventar conteúdo" — é transcrever.

## 3. Já decidido — não reabrir

| Item | Decisão vigente | Fonte |
|---|---|---|
| seed é conteúdo versionado, não estado de jogador | dataset + `content_source` por arquivo | `plano-migrations.md` §4 |
| seed escrito à mão, sem gerador | arquivo por arquivo, sem saída de script (2026-11-21) | `plano-migrations.md` §4 |
| fonte é o 7662 medido | decisão de 2026-11-23; valor de 2026-11-08/15 não vira verdade sem reconferência | `plano-migrations.md` §4 |
| seis fora da onda A | a série de 28 anda sem eles | `lacunas.md` `M33`, `Q-M33-6` |
| ausência é dado | vai para `ausentes.tsv` com motivo; preencher com `0`, média ou "parecido" reprova | `pedido-de-curadoria-dos-seeds.md` §4 |
| contagem que não fecha | entrega com a explicação e a diferença, nunca com o número ajustado | `pedido-de-curadoria-dos-seeds.md` §4 |
| correção de dado | na origem, antes do seed; não vira migration | `plano-migrations.md` §4 |
| produção manual já foi preferida a gerador | `M26`: o dicionário deixou de ser gerador presumido e passou a lotes escritos à mão | `lacunas.md` `M26` |

## 4. As duas perguntas

### Q-M34-1 — Como `0017` e `0020` são produzidos?

**Medido.** Os dois somam **18.417 das 18.800 linhas**; as fontes dos dois já têm `sha256`, tamanho e
contagem em `content-source.json`, e nenhuma das 31 tabelas alvo precisa de DDL. Os outros quatro
seeds somam 383 linhas e dependem de curadoria em documento (`docs/wyd/`), não de projeção.

**O que está em jogo.** A regra "sem gerador" existe para impedir linha inventada, não para impedir
projeção de linha medida. Estendê-la a todos os seis abriria gerador para os conjuntos curados
(taxonomia, regras, quests), onde o risco de invenção é real; mantê-la sem exceção obriga a transcrever
18.417 linhas à mão, sem ganho de verificação.

| Opção | O que é | Consequência |
|---|---|---|
| **A** | Sem exceção: os seis à mão | 18.800 linhas digitadas; a regra fica intacta; a chance de erro de transcrição cresce com o volume e nenhuma etapa nova confere o resultado |
| **B (recomendada)** | Exceção pontual: `0017` e `0020` por um script de passagem única sobre o dataset medido; os outros quatro à mão | 383 linhas à mão + 18.417 projetadas e conferidas por diff contra o seed legado; a regra continua valendo para os quatro e para o futuro |
| **C** | Renumerar a série viva de 28 para 28 contíguos e adiar os seis seeds para a 1.1 | `CAP-01` fecha sem dado; a 1.0.0 sai sem conteúdo; reabre os 34 arquivos de `plano-migrations.md` §3, `SCHEMA_VERSION` e a definição de `CAP-01` |

**Recomendação: B.** É a única opção que mantém o conteúdo da 1.0.0, não inventa linha (projeta o que
já tem hash e contagem) e limita a exceção ao ponto onde ela não tem risco de invenção. **Custo:** B =
383 linhas à mão, 2 TSV, 2 seeds e 1 relatório de diff; A = 18.800 linhas à mão; C = renumerar 28
arquivos e atualizar plano, `SCHEMA_VERSION`, `CAP-01` e checklist, sem escrever linha de conteúdo.
**Recusa A** porque 98% do esforço vira transcrição sem verificação; **recusa C** porque troca conteúdo
por contiguidade e redefine a capacidade, o que não é decisão de seed.

### Q-M34-2 — O script da exceção fica no repositório?

**Medido.** `planejamento-1.0.0/` não aceita executável (`AGENTS.md` §5, item 6;
`estrutura-de-pastas-e-modulos.md` §2.4); a raiz não tem `scripts/`; script de banco e CI mora em
`wyd-ops/scripts/`. O precedente `M26` tirou a produção do dicionário de um gerador presumido.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Descartável: roda fora do repositório; ficam versionados o TSV, o seed e a linha de `content_source` | exceção de uma rodada, sem ferramenta a manter; a reprodutibilidade fica no `sha256` + `row_count` |
| **B** | Versionado em `wyd-ops/scripts/`, com teste | vira ferramenta de conteúdo com dono e manutenção — um gerador de fato, que é o que a regra evita |
| **C** | Versionado junto ao seed, em `crates/wyd-db/` | mistura ferramenta de produção de conteúdo com o crate que é dono do schema e dos testes |

**Recomendação: A.** A exceção é pontual por definição: o artefato que precisa sobreviver é o dado e a
prova dele, não o código que o projetou. **Custo:** A = 0 arquivo de ferramenta; B = 1 script + 1 teste
+ dono em `wyd-ops`; C = 1 script num crate que não é dono dele. **Recusa B** porque transforma exceção
em infraestrutura; **recusa C** porque cria dono errado para o artefato.

### Q-M34-3 — Onde entram as 4 cidades?

**Medido.** A tabela existe: `game.city` é criada em `0011_game_economy.sql:2`, e duas tabelas de
`game` apontam para ela com `city_code NOT NULL` — `game.personal_shop`
(`fk_personal_shop_city`, linha 54) e `game.city_tax_ledger` (`fk_city_tax_ledger_city`, linha 80).
Nenhum dos seis seeds escreve nela: o legado gravava as 4 linhas em `0023_seed_mundo_fixo.sql:9`
(Armia, Azran, Erion e Noatun, capital), e o mesmo arquivo trazia as 2 arenas e os 3 eventos com agenda,
que são de `0027`. O produto já pede o fluxo: `RF-ADM-006` altera imposto de cidade
(`wyd-admin/docs/specs/requisitos.md:53`), e o legado lê o livro por cidade em
`deprecated/crates/wyd-admin/src/api/economy.rs:4`.

**O que está em jogo.** Com `game.city` vazia, loja pessoal e livro de imposto não têm linha para
referenciar (`city_code NOT NULL`), e o `ADM-07` fica sem objeto. Deixar as 4 fora não quebra o schema;
quebra o fluxo que o planejamento do `wyd-admin` já declara.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | As 4 linhas entram no `0020-seed-world.sql`, na tabela `game.city` | 4 linhas, nenhum DDL; o `0020` passa a escrever uma tabela de `game` e declara isso na página; o `0027` fica só com quests, eventos e arenas |
| B | Ficam fora da 1.0.0 e a contagem "4 cidades" sai de `plano-migrations.md` §4 | tabela vazia; `ADM-07` e as duas FKs ficam sem linha; o dado volta na 1.1 |
| C | Vão para o `0027`, junto das arenas e eventos, porque o legado as trazia no mesmo arquivo | `0027` vira seed de economia; `world_event.requires_city` é `BOOLEAN` (`0026_content_quests_events.sql:101`) e não pede a tabela, então a vizinhança no legado não é vínculo |

**Recomendação: A.** É o único destino que fecha a contagem sem DDL e sem misturar dataset: cidade é
dado fixo de mundo e economia, e o `0020` é o primeiro seed depois do `0011` que carrega dado fixo de
mundo. **Custo:** A = 4 linhas e uma nota de schema na página do `0020`; B = 0 linha, mas o
`ADM-07` fica sem dado e a contagem precisa ser retirada de dois documentos; C = 4 linhas, com o
`0027` herdando economia e a divergência de dataset. **Recusa B** porque o fluxo de imposto é 1.0.0;
**recusa C** porque o arquivo legado era "mundo fixo", não "quests e eventos". A ordem de
`plano-migrations.md` §2 pede o seed logo após o DDL do domínio; como a série não tem seed de economia,
o `0020` é o primeiro posterior e a página registra esse desvio de ordem.

**O que a linha não carrega:** `map_id` fica nulo — o cliente 7662 não publica o mapa (`H`,
`0011_game_economy.sql:4`); `region_id` não existe no schema novo, embora o legado ligasse a cidade ao
retângulo de `Regions.txt` (`deprecated/crates/wyd-db/migrations/0010_economia.sql:15`); e
`tax_rate_bps` fica em 1000 nas quatro, como no legado (`0023_seed_mundo_fixo.sql:10`). A tabela
`game.city` não tem coluna de proveniência, então as 4 linhas não ganham `content_source` de dataset: a
origem fica declarada na página do `0020` como `docs/wyd/economia.md` e `decisoes-servidor.md` §3, sem
`sha256` de dataset medido.

## 5. As condições da exceção (se B vencer)

1. Vale **só** para `0017` e `0020`; os outros quatro seeds continuam escritos à mão, e a regra "sem
   gerador" continua sendo a regra.
2. Só entra linha presente no dataset medido. Ausência vai para `ausentes.tsv` com motivo; nada é
   preenchido por default, média ou "parecido" (`pedido-de-curadoria-dos-seeds.md` §4).
3. Cada tabela projetada é conferida por **diff contra o seed legado**, com relatório de divergência
   (nome, preço, requisito, efeito), e o seed grava `content_source` com `path`, `sha256`,
   `size_bytes` e `row_count` da fonte.
4. Coluna sem origem fica nula onde o DDL admite; onde não admite, a linha não é escrita e o caso vai
   para `ausentes.tsv`.
5. Seed e TSV passam por revisão do dono antes do aceite; o script não entra no gate, não vira
   dependência de CI e não mora em `planejamento-1.0.0/`.
6. A exceção **expira** quando os dois arquivos existirem; não se estende a `0016`, `0018`, `0023` nem
   `0027`.
7. A proveniência de `0016`–`0018` é gravada no `0020`, porque `content_source` nasce no `0019`.
8. As 4 cidades entram em `game.city` pelo `0020`, com `map_id` nulo; nada de inventar vínculo com
   região (`region_id` não existe no schema novo) nem mapa que o cliente não publica.

## 6. O que esta rodada não decide

- **Não implementa nada:** não escreve seed, script, TSV nem `content_source`.
- **Não mede o 7662:** a entrega é da curadoria (`pedido-de-curadoria-dos-seeds.md` §4).
- **Não decide as pendências de dado** de §8 — elas sobrevivem a qualquer opção.
- **Não aceita página** de migration: o aceite é ato do dono, página a página (`Q-M33-6`).
- **A exceção entrou em `plano-migrations.md` §4 na rodada da decisão (2026-11-27);** a proposta não
  alterou regra nenhuma antes do "sim".
- **Não cria ADR:** a decisão não muda schema, fronteira nem operação; fica no documento dono, com o
  ID `M34`.

## 7. O que mudou com a resposta (aplicado em 2026-11-27)

| # | Entrega | Arquivo | Dono |
|---|---|---|---|
| 1 | exceção datada, limitada a `0017`/`0020` e com as condições de §5 | `plano-migrations.md` §4 | `wyd-db` |
| 2 | nota de que o método mudou para dois datasets e os seis seguem bloqueados por dado | `docs/planos/plano-de-dados-e-seeds.md` §2 | `wyd-db` |
| 3 | linha `M34` com a resposta e o placar de lacunas recontado | `lacunas.md` §2.3 e §1; `status-planejamento.md` | `wyd-db` |
| 4 | frase do bloqueio de seed aponta a exceção e mantém o dono | `migrations-checklist.md` §3 | `wyd-db` |
| 5 | contagem "4 cidades" com dono `0020` e §3.7 do pedido resolvido | `plano-migrations.md` §4; `pedido-de-curadoria-dos-seeds.md` §3.7 | `wyd-db` |
| 6 | nota de que o seed escreve uma tabela de `game`, com o motivo | página planejada `migrations/0020-seed-world.md`, quando escrita | `wyd-db` |
| 7 | registro das três respostas nesta página | `proposta-decisoes-seeds.md` §9 | `wyd-db` |

Nenhuma entrega escreve seed nem script: elas registram a decisão. A escrita dos arquivos continua
condicionada à medição da curadoria.

## 8. O que continua bloqueado depois desta rodada

- **Os seis seeds**, por dado de origem, com dono em `migrations-checklist.md` §3; a exceção decide o
  método, não entrega a medição.
- **`CAP-01`**, vermelho até os seis arquivos existirem na série.
- **O vínculo cidade ↔ região** do legado (`city.region_id` → `region.id`): o schema novo não o modela,
  e recriá-lo é decisão de DDL, não de seed.
- **A receita de acessório**, `H` porque é sistema posterior ao 7662.
- **O mapa de `kind_slug`** dos 3.220 itens para as 16 categorias de `0016`.
- **As divergências de contagem:** 63 × 69 linhagens de skill, 14 × 15 passos de refino, 4 × 5 gemas,
  124 × 151 skills e 3.216 × 3.220 itens.
- **O conteúdo dos quatro seeds curados**, que esta proposta mantém à mão.

## 9. Registro da decisão

**Respondidas em 2026-11-27.** O dono aprovou a opção A nas três perguntas; `M34` fecha em `lacunas.md`
§2.3 (`39` lacunas, `36` fechadas) e as entregas de §7 foram aplicadas na mesma rodada. Nenhum seed foi
escrito: a decisão muda o método e o destino das cidades, não entrega a medição.

| Pergunta | Resposta | Data |
|---|---|---|
| `Q-M34-1` | A — exceção pontual para `0017`/`0020`, por script de passagem única; os outros quatro à mão | 2026-11-27 |
| `Q-M34-2` | A — script descartável; ficam versionados o TSV, o seed e o `content_source` | 2026-11-27 |
| `Q-M34-3` | A — 4 cidades em `game.city` pelo `0020`, `map_id` nulo e sem `region_id` | 2026-11-27 |
