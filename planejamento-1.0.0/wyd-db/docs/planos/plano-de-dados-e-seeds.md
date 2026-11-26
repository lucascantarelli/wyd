> Pergunta: "que dado inicial existe, de onde ele vem e como ele muda?"
> Público: Curadoria de conteúdo, arquitetura e operação
> Dono: equipe do `wyd-db` com a curadoria de conteúdo
> Base: `plano-migrations.md` §3–§5; `migrations-checklist.md` §2–§3; `wyd-db-1.0.0.md` §1.2; `lacunas.md` `M19`
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

A série da `1.0.0` tem **seis arquivos de seed**, um por dataset, e **nenhum seed de patch**. Seed não
é dado de jogador: é o conteúdo que o jogo lê no boot — catálogo, mundo, regras, quests e eventos —
gravado na mesma série que cria as tabelas, imediatamente depois do DDL do domínio. Esta página é o
plano desses seis arquivos; ela **não** contém as linhas de nenhum deles, porque o dado de origem
ainda não foi reconferido.

## 1. O que é seed e o que não é

| É seed | Não é seed |
|---|---|
| conteúdo versionado, gravado em `content.*` | estado de jogador, conta, personagem, item possuído |
| arquivo próprio, depois do DDL do domínio | `ALTER TABLE`, patch de dado ou correção incremental |
| dado conferido na fonte, com contagem e `sha256` | linha inventada para “fechar” uma tabela |
| executa uma vez, com `content_version` incrementado | rotina que roda a cada boot |

A série herdada tinha **26 arquivos de seed**, dos quais **7 eram patch** (`seed_campo_*`,
`seed_correcao_catalogo_game12_15`, `seed_sem_duplicatas`, `seed_campo_nomes`,
`seed_limpeza_fixtures_campo`). Patch de dado é dívida do dado de origem, não da série: a `1.0.0`
não os reproduz. A decisão de **não ter gerador** é de 2026-11-21 — os seis arquivos são escritos à
mão, como qualquer outra migration, quando a fonte estiver conferida (`plano-migrations.md` §4).

## 2. Os seis datasets

| # | Arquivo | Dataset | Contagem a preservar | Estado |
|---:|---|---|---|---|
| 16 | `0016-seed-taxonomy.sql` | taxonomia | 20 sistemas, 6 classes, 13 linhagens, 16 categorias | `bloqueada` |
| 17 | `0017-seed-item-catalog.sql` | catálogo de item | 3.220 itens + 55 `EF_*` | `bloqueada` |
| 18 | `0018-seed-skill-catalog.sql` | catálogo de skill | 151 skills + 3 Kersef + linhagem curada | `bloqueada` |
| 20 | `0020-seed-world.sql` | mundo | 1.991 templates, 6.100 NPCs, 6.706 lojas, 161 `level_item`, 52 regiões, 49 itens de chão | `bloqueada` |
| 23 | `0023-seed-rules.sql` | regras | 14 passos de refino, 4 gemas, SANC, combine, acessório | `bloqueada` |
| 27 | `0027-seed-quests-events.sql` | quests e eventos | quests de campo, 3 `world_event` + agenda, 2 arenas | `bloqueada` |

As contagens foram medidas em **2026-11-08/15** e são as que o seed tem de reproduzir
(`plano-migrations.md` §4). A origem de cada uma é o **dataset legado** — o arquivo de conteúdo que
já existe no repositório —, não uma estimativa desta página. O que falta, por arquivo, é a
reconferência: `0016` taxonomia; `0017` itens e efeitos; `0018` classe e linhagem das skills; `0020`
templates, NPCs, lojas, níveis, regiões e itens de chão; `0023` refino, gemas, SANC e composição;
`0027` cadeia de quests do 7662 e calendário de eventos. **Enquanto a fonte não for conferida, o
arquivo não é escrito** (`plano-migrations.md` §4; `migrations-checklist.md` §3).

## 3. Proveniência e versão

| Mecanismo | Regra | Fonte |
|---|---|---|
| `content_source` | cada dataset grava `path`, `sha256`, `size_bytes` e `row_count`; a proveniência acompanha o seed | `plano-migrations.md` §4 |
| `content_version` | incrementa **uma vez por seed aplicado**; fecha em **7** na `1.0.0` | `plano-migrations.md` §4 e §7 |
| idempotência | `ON CONFLICT DO NOTHING` **só** onde o dataset é reaplicável; o seed roda uma vez | `plano-migrations.md` §4 |
| ordem | cada seed vem imediatamente depois do DDL do domínio, nunca no fim da série | `plano-migrations.md` §2 |
| contagem | 3.220 itens, 55 efeitos, 154 skills (63 com linhagem), 1.991 templates, 6.100 NPCs, 6.706 lojas, 161 `level_item`, 52 regiões, 49 itens de chão, 14 passos de refino, 4 gemas, 4 cidades, 2 arenas, 3 eventos | `plano-migrations.md` §4 |

`sha256` e `row_count` são a prova de que o arquivo veio da fonte certa e não foi editado depois. Um
seed sem esses dois valores é linha sem procedência, e o gate de conteúdo reprova.

## 4. Dado de jogo × configuração do dono

| Tipo | Exemplo | Onde mora | Quem escreve |
|---|---|---|---|
| conteúdo | item, skill, NPC, mapa, quest, evento | `content.*`, por seed da série | curadoria, via migration |
| configuração do ambiente | canal (`grupo`, `host`, `maintenance`), DSN, topologia | `game.server_channel` e ambiente | operação, fora do seed |
| dado de desenvolvimento | conta de teste, senha `dev-only` (`M19`) | `seeds/dev.sql` | desenvolvedor, só em desenvolvimento |
| dado de jogador | conta real, personagem, item possuído, saldo | `game`/`portal`, em runtime | jogo e portal, nunca seed |

`game.server_channel` **não** é semeado: as linhas do corte vertical vêm da fixture de
`ddl-confirmado.md` §5.5, porque grupo, host e URL de status são dado de instalação
(`plano-migrations.md` §3.2). O seed de desenvolvimento usa a senha fixa `dev-only`, decidida em
`M19`, e ela **nunca** vale fora de desenvolvimento.

## 5. Como atualizar

**Antes de aplicar:** corrigir o dado **na origem**, recontar, regravar o `sha256` e só então
reescrever o arquivo — o seed não corrige conteúdo por cima. Um erro de desenho antes do `apply` é
reescrita do arquivo; depois do `apply`, o arquivo é imutável (`plano-migrations.md` §2) e a mudança
vira arquivo novo.

**Depois de aplicar:** não existe seed de patch nem edição de arquivo aplicado. A atualização é um
lote novo, com o delta nomeado; **o formato desse lote não está decidido** nesta página. O que a
série garante é a imutabilidade e o drift acusado pelo ledger (`src/migrate/spec.md` §4).

## 6. Como reverter

| Situação | Caminho | Limite |
|---|---|---|
| desenvolvimento, antes de o dado valer | `down` do próprio arquivo, na mesma transação, derrubando o que o `up` criou | não devolve conteúdo sobrescrito por outro seed |
| produção | restore do backup verificado, com a janela da página de implantação | perde o que foi escrito depois do backup |
| patch de conteúdo | não existe | a série nova não tem `seed_correcao_*` |

O `down` de seed é de **desenvolvimento**; em produção o plano A é restore, não `down`
([`plano-de-implantacao.md`](plano-de-implantacao.md) §Rollback).

## 7. O que nunca entra em produção
**Sem arquivo no repositório:** `resources/accounts.json` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

- **Dado de jogador real**, conta, personagem ou saldo — o seed é conteúdo, não carga de base.
- **Senha ou segredo real**: a única senha de seed é `dev-only`, e só em desenvolvimento.
- **Fixture, conta de teste ou `resources/accounts.json`** — são de teste, não de catálogo.
- **Seed de patch** ou linha escrita para “fazer a contagem fechar”.
- **Arquivo sem `content_source`**: sem `path`, `sha256` e `row_count`, não há como provar a origem.

## 8. O que não está decidido

- **Os seis seeds:** bloqueados por dado de origem, não por desenho; nenhum é escrito por dedução. O
  pedido de curadoria — o que reconferir em cada um e o formato da medição — está em
  `pedido-de-curadoria-dos-seeds.md`, na raiz do sistema.
- **Atualização pós-`apply`:** o formato do lote de atualização de conteúdo não está decidido.
- **Contagens medidas:** as da §2 são as da fonte em 2026-11-08/15; o valor final só vale depois de
  recontado na fonte e gravado no seed.
