# Plano da série de migrations 1.0.0 — do zero

> **Estado: proposta.** Nenhum arquivo de migration novo foi escrito. A série atual
> (`crates/wyd-db/migrations/`, 53 arquivos, 87 tabelas) continua intacta e é a que roda hoje.
>
> **Assina:** `padrao-banco.md` (schemas, nomes, tipos, constraints, roles, migrations).
> **Substitui como fonte da série:** `09-banco-novo-serie-0001.md` (31 arquivos, 66 tabelas novas) e a
> lista de 50 arquivos que vivia em `rascunho-accounts.md` §11 — seção removida em 2026-11-17, quando
> o rascunho foi reduzido a histórico de requisitos.
> **Medido no dia:** 53 migrations, 87 tabelas, 625 colunas, `SCHEMA_VERSION = 53`.

Números deste plano são **planejados**: **31 arquivos e 97 tabelas** — 92 originais mais as 4 do
adendo `0030` e a do adendo `0031`, aprovados em 2026-11-20. O número real sai do catálogo
depois que a série existir — e **não é meta** (é consequência das decisões).

---

## 1. Pré-requisitos: as ferramentas hoje recusam este desenho

O padrão usa schema qualificado, `INET` e views. As ferramentas atuais **não conhecem** nenhum dos
três, e falham em silêncio ou com erro obscuro.

> **Decisão de 2026-11-17 (dono): o PR de ferramentas está fora desta rodada.** Nenhum arquivo de
> `tools/` é alterado e as ferramentas não são adotadas por enquanto. A série é escrita à mão contra
> `padrao-banco.md`, e os gates de §7 viram **conferência manual** até o PR ser autorizado. §1.3 não
> some: vira o critério do dia em que ele for liberado.
>
> **Decisão de 2026-11-20 (dono): o PR de ferramentas está liberado** — as mudanças de §1.1 são o
> escopo dele. **Ferramenta não é pré-requisito do DDL** (corrigido em 2026-11-21): a série é escrita e
> conferida à mão, como já era desde 2026-11-17, e a ordem de aplicação é `provision render` →
> `provision apply` → `0001`…`0029` → catálogo → gates. O PR de ferramentas, quando autorizado, entra
> antes do **catálogo**; nenhum arquivo `.sql` espera por ele.
>
> **Decisão de 2026-11-21 (dono): não há gerador de seed.** Os seis arquivos de seed são escritos à
> mão, como qualquer outra migration, quando o dado de origem estiver conferido; gerar arquivo não é
> pré-requisito do planejamento nem da série. Ferramenta ou gerador novo, inclusive dentro de
> `tools/content/`, só entra com aprovação explícita e pedido próprio — nenhum está previsto nesta
> rodada (`migrations-checklist.md` §3). A ordem de 2026-11-20 continua valendo para o resto.
>
> **Decisão de 2026-11-21 (dono): o lote `0030` permanece aprovado e não há lote novo.** As quatro
> tabelas de conteúdo do portal (`portal.news`, `banner`, `guide` e `launcher_release`) ficam como
> estão: o DDL é o de `ddl-confirmado.md` §5.15, a página é `migrations/0030-portal-content.md` e o
> lote conta como `concluída` no checklist. Reabrir o desenho exigiria proposta de revisão própria com
> o delta nomeado, que não foi pedida. A série continua em **31 arquivos e 97 tabelas**; nenhum arquivo
> `0032` nasce desta conclusão.

| Arquivo | O que muda | Por quê |
|---|---|---|
| `tools/content/banco-parse.mjs` | regex de `CREATE TABLE`, `REFERENCES`, `CREATE INDEX … ON` aceitando `schema.tabela`; `TIPOS` com `INET` e `NUMERIC(p,s)`; captura de `CREATE VIEW`/`CREATE TRIGGER` | hoje `CREATE TABLE game.account (` **não casa** e a tabela desaparece do catálogo sem erro |
| `tools/content/check-migrations.mjs` | mesmas regexes; `DROP TABLE` qualificado; contagens documentadas lidas de `banco-fontes.mjs` (hoje `{ migrations: 53, tables: 87 }` fixo); gates novos: constraint sem nome, tipo proibido (`serial`, `timestamp`, `money`, `varchar`) | sem isso a série nova reprova por contagem e o gate não pega o que o padrão proíbe |
| `tools/content/pg.mjs` + `migrate-cycle.mjs` | `resetSchema` derrubando **todos** os schemas de app; contagens por `table_schema <> 'pg_catalog'`; `afterDown = 0` somando schemas; `search_path` explícito | hoje reset e contagem são `public`-only: `down` deixaria tabela em `game` e o teste passaria |
| `crates/wyd-db/tests/contrato/catalogo.rs` | nomes qualificados; `SCHEMA_VERSION`; regra "seed só em tabela já criada" com schema | é o gate doc ↔ SQL |
| `tools/content/banco-fontes.mjs` + `gen-catalogo-banco.mjs` | campo `schema` por tabela; seções de view/função/trigger/role | o catálogo gerado é contrato de leitura humana |
| `tools/content/verify-schema.mjs` | `information_schema` por schema; coluna `schema` no relatório | prova banco real ↔ documento |
| `docker-compose.yml` / bootstrap | `db/roles.sql` gerado por `provision render` e aplicado por `provision apply` antes das migrations; conexões com role própria | permissão é o que faz a fronteira valer, e o arquivo não é escrito à mão |

**Ordem:** `provision render` (gera `db/roles.sql`) → `provision apply` → `0001` → … → `0029` →
catálogo → gates. As ferramentas de §1.1 não entram nessa fila: são conferência automatizada do que já
existe e só rodam se o PR for autorizado — até lá, os gates de §7 são conferência manual.

### 1.1 O que muda em cada arquivo, concretamente

**`tools/content/banco-parse.mjs`** — hoje o casamento é ancorado em nome simples:

```js
/^CREATE TABLE ([a-z_][a-z0-9_]*) \($/gm
/REFERENCES\s+([a-z_]+)/
/CREATE (UNIQUE )?INDEX ([a-z_]…)\s+ON\s+([a-z_]…)/
```

`CREATE TABLE game.account (` **não casa**: a tabela some do catálogo (e o índice dela depois falha
com "tabela inexistente", erro no lugar errado). Mudanças:

- **Nome qualificado** em `CREATE TABLE`, `REFERENCES` e `ON` de índice: `([a-z_][a-z0-9_]*\.)?[a-z_][a-z0-9_]*`.
  Cada tabela passa a guardar `schema` e `nome`, e a chave do modelo vira **`schema.tabela`** — o
  mesmo nome em dois schemas é legal agora e precisa deixar de ser colisão.
- **Tipos**: `INET: 'inet'` em `TIPOS` (o padrão exige `INET`, P-10) e normalização de `NUMERIC(p,s)`
  para `numeric` — hoje qualquer precisão explícita estoura `tipo SQL não mapeado`.
- **Views e triggers**: nenhum dos dois é lido hoje. `CREATE VIEW schema.v_x` e
  `CREATE TRIGGER trg_x … ON schema.tabela` entram no modelo, porque §7 do padrão os declara e o
  catálogo tem de publicá-los.
- **Função**: `CREATE FUNCTION (schema\.)?fn_x`.
- **Gate de constraint anônima**: elemento do corpo que casa `^(PRIMARY KEY|UNIQUE|CHECK|FOREIGN KEY)`
  mas não começa com `CONSTRAINT nome` reprova (P-13). Hoje passa em silêncio e vira nome truncado
  pelo Postgres.

**`tools/content/check-migrations.mjs`** — mesmas regexes qualificadas em `CREATE TABLE`, `DROP TABLE`
e `REFERENCES`; `DOCUMENTED` deixa de ser `{ migrations: 53, tables: 87 }` fixo e é lido de
`banco-fontes.mjs` (uma fonte só, por série). Gates novos: constraint anônima (acima), tipo proibido
em posição de coluna (`serial`, `bigserial`, `timestamp` sem fuso, `time`, `money`, `varchar`, `enum`)
e todo `INSERT` em arquivo sem `_seed_` no nome.

**`tools/content/pg.mjs` + `migrate-cycle.mjs`** — hoje o reset é `DROP SCHEMA public CASCADE` e a
contagem é `table_schema = 'public'`. Com 4 schemas isso dá **falso verde**: o `down` deixa tabela em
`game` e o ciclo passa. Mudanças:

- `resetSchema` derruba **todo** schema de app (`content`, `game`, `portal`, `ops`) e recria `public`
  vazio — via `pg_namespace`, nunca lista escrita à mão.
- `afterDown` soma `information_schema.tables` de **todos** os schemas de app (base table **e** view).
- antes de cada `up`/`down`, o ciclo conecta **como** `wyd_migrator` (DSN própria; `SET ROLE` não
  serve, porque o preflight do runner recusa outro papel — `src/migrate/spec.md` §7): o efeito de
  `ALTER DEFAULT PRIVILEGES` depende de quem cria o objeto (`ddl-confirmado.md` §3). Sem isso os
  `GRANT` de `0001` não pegam nada e o teste de fronteira falha por motivo errado.
- `db/roles.sql` é gerado por `provision render` e chega ao cluster por `provision apply`, fora da
  série e fora do ciclo `up`/`down` (papel é do cluster, não vai no dump): não é executado à mão nem
  montado em `/docker-entrypoint-initdb.d/`.
- a consulta de contagens sai do código: cada série traz seu `contagens.sql` (a herdada usa as 20
  linhas de hoje; a nova usa `content.item_catalog`, `content.npc_template`… com nome qualificado).

**`crates/wyd-db/src/migrate/`** — o runner é desenho novo, não herdado: o formato do arquivo, a leitura
do diretório e a validação da série estão em `src/migrate/spec.md` §2, onde quem lê o disco é a porta
`Source` (adapter de filesystem lista `*.sql`; a ordenação e a contiguidade são do `set` puro). A
instalação aponta a série por `WYD_MIGRATIONS_DIR` (`src/migrate/spec.md` §9) e a conexão vem de
`WYD_DB_URL_MIGRATOR`. Duas consequências para as ferramentas, ambas com dono no desenho: (a) **toda
instrução da série é qualificada**, e o runner executa cada arquivo sob a regra de `search_path` de
`src/migrate/spec.md` §6; (b) o ledger é criado pelo runner **antes** de `0001` — nome, forma e
leitura são de `src/migrate/spec.md` §3, a localização em `public` é a exceção declarada em
`ddl-confirmado.md` §3, e o privilégio de `wyd_migrator` ali é de ambiente, do provisionamento
(`src/provision/spec.md` §4). Ledger é do runner, não do domínio.

**`crates/wyd-db/tests/contrato/{catalogo,migrations}.rs`** — leem `crates/wyd-db/migrations` fixo e
casam por nome simples; passam a ler `WYD_MIGRATIONS_DIR`, que **definem** para `crates/wyd-db/migrations`
— sem default próprio: o default de `/migrations` é do binário (`src/provision/spec.md` §3) — e casar por
`schema.tabela`. O gate doc ↔ SQL continua exigindo que toda tabela do SQL esteja no catálogo e vice-versa.

**`tools/content/banco-fontes.mjs` + `gen-catalogo-banco.mjs` + `verify-schema.mjs`** — campo `schema`
por tabela, seções de view/função/trigger/papel, e `banco-catalogo.md` com a coluna `schema`; o
`verify-schema` passa a comparar por `(schema, tabela)` em vez de nome solto.

**`docker-compose.yml`** — `db/roles.sql` gerado por `provision render` e aplicado por
`provision apply` antes das migrations (o entrypoint do Postgres só roda com `PGDATA` vazio, então não
provisiona) e **uma conexão por serviço**: cada serviço recebe a DSN do seu papel, e nenhum lê
`DATABASE_URL` — variáveis, leitor e a ausência de default estão em `src/provision/spec.md` §3
(decisão `M24`).

**Fixture do PR**: `crates/wyd-db/tests/contrato/fixtures/serie-schema/` com 2 arquivos mínimos
(`0001` cria os 4 schemas e `game.account`; `0002` cria `content.item_kind` + uma view) e o par
negativo (`0003` com constraint anônima, `timestamp` e `varchar`). O teste prova três coisas: o parser
**vê** `game.account`, o gate **reprova** os três vícios, e a série real (53 arquivos) continua com o
mesmo resultado de antes do PR.

### 1.2 Duas séries convivem (a troca ainda não aconteceu)

| Série | Onde | Estado | Quem usa |
|---|---|---|---|
| herdada | `crates/wyd-db/migrations/` | **congelada**, intacta | o produto hoje: `make migrations-apply`, `cargo test -p wyd-db`, `SCHEMA_VERSION = 53` |
| 1.0.0 | `crates/wyd-db/migrations/`, na branch `serie-1.0.0` | nasce agora | PG17 efêmero do `migrate-cycle` e o corte vertical de §3.2 |

A 1.0.0 **não** ganha diretório próprio: a pasta é `crates/wyd-db/migrations/` antes e depois da
troca — o que muda é o conteúdo. Enquanto a troca não acontecer, os 31 arquivos vivem na branch
`serie-1.0.0` e `develop` segue com os 53 herdados. O que isso custa: uma branch, e o gate que crava
contagem (`tools/content/check-migrations.mjs`, `{ migrations: 53, tables: 87 }`) reprovando na branch
com 31 arquivos — consequência aceita em §1.3. Nada renomeado, nada apagado, `SCHEMA_VERSION`
continua 53 em `develop`.

### 1.3 Critério de pronto do PR de ferramentas

1. `node tools/content/check-migrations.mjs` na série herdada: `53 | 87`, os mesmos problemas de hoje
   (nenhum), zero regressão.
2. `node tools/content/check-migrations.mjs --dir crates/wyd-db/tests/contrato/fixtures/serie-schema`
   passa na fixture boa e **reprova** cada vício da fixture ruim com mensagem própria.
3. `cargo test -p wyd-db` verde (os dois testes de contrato leem a série viva).
4. `make migrations-apply` (up/down/up) verde com a série herdada **depois** das mudanças de `pg.mjs`
   — é o teste de que o reset multi-schema não quebrou o ciclo antigo.
5. `make docs-check` verde.

Só depois disso se escreve SQL. Enquanto 1–5 não fecham, qualquer migration nova é escrita contra um
leitor que a ignora.

**Estado (2026-11-21): em espera por decisão do dono.** O PR não foi executado; 1–5 continuam sendo
o aceite dele e **não** são pré-requisito de nenhum arquivo da série. Consequência aceita:
`make migrations-check` reprova na série nova por contagem (`{ migrations: 53, tables: 87 }` cravado
em `tools/content/check-migrations.mjs`) e `banco-parse` não enxerga `CREATE TABLE game.account (`
— nenhum dos dois vira gate da série nova enquanto isso, e o corte vertical de §3.2 é provado por
`psql` direto no PG17 efêmero.

**Enunciação (2026-11-18):** o **estado** do gate — que ele não existe hoje (`V`) — e o que fazer
enquanto isso (conferência manual dos oito itens) são lidos em
`docs/guia-desenvolvedor/como-adicionar-tabela.md` §6; o que a implantação assume do binário e da série
sem ele está em `docs/planos/plano-de-implantacao.md` §Pré-requisitos. Aqui ficam os **defeitos de cada
ferramenta** e o **critério de pronto** de §1.3, que é o aceite do PR quando ele for autorizado.

---

## 2. Formato e numeração

- O **formato mecânico** do arquivo — um `-- migrate:up` e um `-- migrate:down`, obrigatórios e não
  vazios, sem transação própria nem comando não transacional — é do runner: `src/migrate/spec.md` §2.
  Aqui ficam só as regras da **série**.
- `crates/wyd-db/migrations/NNNN_dominio.sql`, 4 dígitos, **contíguo**, sem lacunas; a numeração
  e a ordem são deste plano (§3). O diretório é o mesmo da série herdada; o runner o recebe por
  `WYD_MIGRATIONS_DIR` (`src/migrate/spec.md` §9) e não depende do nome da pasta.
- `-- migrate:up` e `-- migrate:down` **sozinhos na linha**; `CREATE TABLE schema.tabela (` na linha e
  `);` sozinho no fim — o parser do catálogo depende disso e quebra alto fora desse estilo.
- Sem `ALTER TABLE` no `up`; sem tabela partida entre arquivos; sem FK para arquivo posterior.
- Seed em arquivo próprio, imediatamente depois do DDL do domínio (não no fim da série).
- `SCHEMA_VERSION` acompanha o total de arquivos da série e muda **só na troca** (§6): **31**.
  Enquanto a série herdada é a que roda, a constante continua 53. `down` derruba **no mesmo arquivo** o
  que o `up` criou.
- Arquivo aplicado é imutável: o runner compara arquivo e ledger e acusa drift (`src/migrate/spec.md`
  §4). Erro de desenho antes de aplicar = reescrever; depois = arquivo novo.

**Enunciação (2026-11-18):** o **procedimento** que aplica estas regras está em
`docs/guia-desenvolvedor/como-adicionar-migration.md` §Regra da série e §Passo a passo; o
**porquê** de cada uma, em `docs/arquitetura/migracoes.md` §2. Aqui ficam as regras como **contrato**
— inclusive o que o parser exige literalmente (marcadores sozinhos na linha) — e a numeração continua
sendo de §3, que nenhuma das duas enuncia. Regra nova entra aqui e aparece nas duas páginas na mesma
entrega.

---

## 3. A série (31 arquivos; 97 tabelas)

Antes do arquivo 1 existe a **etapa 0**, que não é migration: `db/roles.sql`, os papéis do cluster —
artefato gerado por `wyd-db provision render`, especificado em `src/provision/spec.md`
(**planejamento**) e decidido em ADR-DB-0001. Ela não entra nesta tabela nem na contagem, porque papel
não é objeto do banco e não tem `up`/`down`.

O andamento de cada uma destas 31 páginas é controlado em `migrations-checklist.md`, na raiz desta
pasta de planejamento — e não em `migrations/`, que guarda só página de migration.

| # | Arquivo | Schema | Cria | Tab. |
|---|---|---|---|---:|
| 1 | `0001_bootstrap.sql` | — | 4 schemas, `content_version`, `ops.fn_set_updated_at()` | 1 |
| 2 | `0002_ops_audit.sql` | `ops` | `audit_log`, `login_attempt`, `config_revision`, `config_change` | 4 |
| 3 | `0003_portal_person.sql` | `portal` | `person`, `person_profile`, `person_consent` | 3 |
| 4 | `0004_portal_access.sql` | `portal` | `person_session`, `person_token` | 2 |
| 5 | `0005_game_account.sql` | `game` | `account`, `account_wallet`, `account_ledger` (`M16`), `account_moderation`, `pin_reset` | 5 |
| 6 | `0006_game_character.sql` | `game` | `character`, `character_status`, `character_scores`, `character_special`, `character_short_skill` | 5 |
| 7 | `0007_game_session.sql` | `game` | `session`, `server_channel` (com `max_players`/`maintenance`/`role`, `M23`) | 2 |
| 8 | `0008_game_skills.sql` | `game` | `character_skill`, `character_learned_skill`, `character_lineage`, `character_affect`, `character_chaos` | 5 |
| 9 | `0009_game_inventory.sql` | `game` | `character_item`, `vault_item`, `refine_pity` | 3 |
| 10 | `0010_game_guild.sql` | `game` | `guild`, `guild_member`, `guild_relation`, `guild_tower` | 4 |
| 11 | `0011_game_economy.sql` | `game` + `portal` | `city`, `city_tax_ledger`, `item_price`, `personal_shop` (em `game`); `donate_shop_item`, `daily_reward_item`, `daily_reward_claim`, `kingdom_cape_price`, `donate_credit`, `donate_debit` (6 em `portal`, `A19`) | 10 |
| 12 | `0012_ops_delivery.sql` | `ops` | `delivery_queue` | 1 |
| 13 | `0013_content_taxonomy.sql` | `content` | `game_system`, `game_class`, `skill_lineage`, `item_kind`, `item_effect`, `fn_fold` | 5 |
| 14 | `0014_content_item.sql` | `content` | `item_catalog` (PK `item_index`; `name_fold` gerada; GIN em `effect_names`) | 1 |
| 15 | `0015_content_skill.sql` | `content` | `skill_catalog` | 1 |
| 16 | `0016_seed_taxonomy.sql` | `content` | 20 sistemas, 6 classes, 13 linhagens, 16 categorias | — |
| 17 | `0017_seed_item_catalog.sql` | `content` | 3.220 itens + 55 `EF_*` (gerado) | — |
| 18 | `0018_seed_skill_catalog.sql` | `content` | 151 skills + 3 Kersef + linhagem curada (gerado) | — |
| 19 | `0019_content_world.sql` | `content` | `content_source`, `npc_template`, `npc_definition`, `npc_shop_item`, `level_item`, `guard_spawn`, `guard_spawn_reward`, `mob_merc`, `game_setting`, `game_matrix`, `region`, `world_item_seed`, `guild_zone` | 13 |
| 20 | `0020_seed_world.sql` | `content` | 1.991 templates, 6.100 NPCs, 6.706 lojas, 161 `level_item`, 52 regiões, 49 itens de chão (gerado) | — |
| 21 | `0021_content_rules.sql` | `content` | `refine_rate`, `refine_gem_effect`, `sanc_rate`, `combine_value`, `accessory_refine_rule` | 5 |
| 22 | `0022_content_overrides.sql` | `content` | `refine_rate_override`, `sanc_rate_override`, `combine_value_override` | 3 |
| 23 | `0023_seed_rules.sql` | `content` | 14 passos de refino, 4 gemas, `sanc_rate`, combine, acessório | — |
| 24 | `0024_content_maps.sql` | `content` | `map_definition`, `teleport_route` | 2 |
| 25 | `0025_content_combat.sql` | `content` | `mob_template_stat`, `mob_template_equip`, `drop_rate_override`, `content_rate_override` | 4 |
| 26 | `0026_content_quests_events.sql` | `content` | `quest`, `quest_step`, `quest_reward`, `world_event`, `event_schedule`, `arena` | 6 |
| 27 | `0027_seed_quests_events.sql` | `content` | quests de campo, 3 `world_event` + agenda, 2 `arena` (gerado) | — |
| 28 | `0028_game_quests_events.sql` | `game` | `character_quest`, `event_participation`, `kefra_state`, `castle_state`, `arena_match`, `arena_round`, `arena_score` | 7 |
| 29 | `0029_views.sql` | — | `game.v_live_session`, `portal.v_person_identity` | — |
| 30 | `0030_portal_content.sql` | `portal` | `news`, `banner`, `guide`, `launcher_release` — CMS e launcher do `wyd-portal` (aprovado em 2026-11-20) | 4 |
| 31 | `0031_portal_mfa.sql` | `portal` | `person_authenticator` — segundo fator de staff do `wyd-portal` (`A20`, aprovado em 2026-11-20) | 1 |

**Total: 31 arquivos, 97 tabelas** e 6 arquivos de seed.

O `0030` nasceu da decisão de 2026-11-20 no `wyd-portal` (`LAC-POR-02`/`LAC-POR-03`): quatro tabelas
em `portal` para notícia, banner, guia e versão de launcher. Ele vem **depois** de `0029` porque não
tem FK de saída e nenhuma view o lê; renumerar as views não mudaria dependência e quebraria a citação
de `0029` em vinte páginas. A ordem de §3.1 continua valendo para os 29 originais.

O `0031` é o segundo adendo (2026-11-20) e fecha a última peça que o `wyd-portal` pedia:
`portal.person_authenticator`, a credencial de segundo fator que `A20` reservou. Como o `0030`, não tem
FK de saída e nenhuma view o lê, então entra no fim. O número **coincide** com o `0031` da série
herdada, absorvido por `0001` (`wyd-db-1.0.0.md`): são séries distintas, e a nova é contígua por `B2`.

**DDL já decidido:** **25 dos 31 arquivos** fecham inteiros hoje — **97 de 97 tabelas** —, com o texto
completo em `ddl-confirmado.md`. Nome de tabela nesta lista **não** é DDL: dos 97 nomes, nenhum fica
sem coluna desenhada e nenhum aguarda aprovação. Os seis restantes são seeds, bloqueados por dado de
origem não verificado (§4), não por desenho.

Os lotes de `0002` (`ops.config_revision`/`ops.config_change`), de `0006` (`game.character` e os
quatro satélites) e de `0008` (`character_skill` e os quatro satélites) foram aprovados em 2026-11-20
e contam na série, assim como o de `0009` (`character_item`, `vault_item` e `refine_pity`), aprovado em
2026-11-20 (§5.12), o de `0010` (`guild`, `guild_member`, `guild_relation` e `guild_tower`), aprovado
na mesma data (§5.13), e o de `0011` (economia, 10 tabelas), aprovado em §5.14.

O `0012` (`ops.delivery_queue`) fechou em 2026-11-20 (`ddl-confirmado.md` §5.16), junto com `M25`:
dreno com `SKIP LOCKED` e expurgo de 30 dias após `closed_at`. O `0013` (`content.taxonomy`: cinco
tabelas e `fn_fold`) fechou em 2026-11-20 (`ddl-confirmado.md` §5.17), com `slug` como PK e
`item_effect` como look-up. O `0014` (`content.item_catalog`) fechou na mesma data
(`ddl-confirmado.md` §5.18), e o `0015` (`content.skill_catalog`) também (§5.19). Os seeds `0016`–`0018`
seguem `bloqueada` pelo dado de origem (§4); o `0019` (`content.world`, 13 tabelas) fechou em
2026-11-20 (`ddl-confirmado.md` §5.21) e o `0021` (`content.rules`, 5 tabelas) também (§5.22).
O `0024` (`content.maps`, 2 tabelas) fechou em 2026-11-21 (§5.24) e, na mesma data, o dono aprovou os
cinco lotes que faltavam: `0022` (`content.overrides`, 3 tabelas, §5.23), `0025` (combate, 4, §5.25),
`0026` (quests, eventos e arena, 6, §5.26), `0028` (estado de jogo, 7, §5.28) e `0029` (views, §5.29).
Nenhum arquivo de DDL espera aprovação; restam os seis seeds.

### 3.1 Por que esta ordem

1. **Identidade primeiro** (`0001`–`0007`): é a raiz do grafo e o que dá o corte vertical de §3.2.
   Nada de conteúdo é necessário para autenticar.
2. **`ops.audit_log` antes de tudo que audita** (`0002`), sem FK — auditoria nunca bloqueia escrita.
3. **`portal` antes de `game`**: `game.account.person_id → portal.person`. Jogo referencia a pessoa,
   nunca copia e-mail.
4. **Estado de jogador de leitura simples** (`0008`–`0011`) e **entrega** (`0012`, FK para
   `game.account`/`game.character` com `RESTRICT`).
5. **Conteúdo depois** (`0013`–`0027`): nenhuma tabela de conteúdo tem FK de saída para `portal`/`game`,
   então pode ser escrito em paralelo ao estado de jogo, sem risco de ordem.
6. **`0028`** é o único bloco de jogo com FK para `content` (`quest`, `arena`) — por isso fecha depois
   do conteúdo. **`0029`** (views) precisa de tudo.

### 3.2 Corte vertical (o que provar antes de continuar)

Com `0001`–`0007` aplicados em PG17 efêmero (7 arquivos, 21 tabelas):

- uma pessoa com e-mail e senha de portal autentica no BFF e recebe sessão viva — token opaco, só o `sha256` no banco (`A16`);
- cria a 1ª conta na vaga 1; a 6ª falha por `ux_account_slot`;
- o cliente 7662 faz login (login 16 B + senha 12 B + PIN 6 B), abre `game.session` e escolhe
  personagem; a segunda conexão da mesma conta é recusada por `ux_session_viva_por_conta`;
- 5 contas da mesma pessoa abertas ao mesmo tempo em canais diferentes **passam**;
- `portal` recebe `permission denied` ao escrever em `game`; `game` não lê `portal.person` cru.

As linhas de `game.server_channel` do teste vêm da **fixture** de `ddl-confirmado.md` §5.5, não de
seed: a série não semeia canal (grupo/`host`/`status_url` são dado de instalação).

Passar aqui é o que autoriza escrever os 24 arquivos restantes (22 dos 29 originais + os adendos
`0030` e `0031`). Sem isso, o plano é papel.

---

## 4. Seeds: 26 arquivos viram 6

A série herdada tem **26 arquivos de seed**, dos quais **7 são patch** (`seed_campo_*`,
`seed_correcao_catalogo_game12_15`, `seed_sem_duplicatas`, `seed_campo_nomes`,
`seed_limpeza_fixtures_campo`). Patch de dado é dívida do dado de origem, não da série.

| Regra | Efeito |
|---|---|
| Correção de dado é feita no dado de origem, antes de o seed ser escrito, e não vira migration | a série nova não tem `seed_correcao_*` |
| Um arquivo por dataset gerado (taxonomia, item, skill, mundo, regras, quests/eventos) | 6 arquivos em vez de 26 |
| `content_version` incrementa uma vez por seed aplicado | continua sendo a versão do conteúdo |
| `content_source` (`path`, `sha256`, `size_bytes`, `row_count`) acompanha cada dataset | proveniência preservada |
| Idempotência por `ON CONFLICT DO NOTHING` só onde o dataset é reaplicável | seed roda uma vez; reaplicar não duplica |

Contagens a preservar (medidas em 2026-11-08/15): 3.220 itens, 55 efeitos, 154 skills (63 com
linhagem), 1.991 templates, 6.100 NPCs, 6.706 slots de loja, 161 `level_item`, 52 regiões, 49 itens
de chão, 14 passos de refino, 4 gemas, 4 cidades, 2 arenas, 3 eventos, `content_version = 7`.

**O que falta em cada seed (2026-11-21)** — dado de origem, nunca ferramenta: `0016` reconferir as
contagens de taxonomia; `0017` recontar itens e efeitos do catálogo; `0018` curadoria de classe e
linhagem das skills; `0020` recontar templates, NPCs, lojas, níveis, regiões e itens de chão; `0023`
conferir valores de refino, gemas, SANC e composição; `0027` fechar a cadeia de quests do 7662 e o
calendário de eventos. Enquanto o dado não estiver conferido, o arquivo não é escrito.

**Como os 6 seeds nascem (2026-11-21):** à mão, arquivo por arquivo, a partir do DDL fechado e do dado
conferido na fonte. Cada linha é escrita e revisada como qualquer outra da série — não há gerador, não
há saída de script e não há dependência de ferramenta para liberar `0016`–`0027`. O bloqueio é de
**dado**: enquanto a contagem e o valor não estiverem verificados, a página declara `H` e o arquivo não
é escrito, porque inventar linha de conteúdo seria pior que a lacuna.

---

## 5. O que sai e o que entra

**Não volta:** `character_session` (vira `game.session` com `account_id`), `pin_reset_token` (vira
`game.pin_reset`), `account_play_code` e `donate_topup_order` (Cold Path comercial, fora do schema),
`accounts` (vira `game.account` + `portal.person`), os 7 seeds de patch, os 4 arquivos absorvidos
(`0030`, `0031`, `0034`, `0053` da série antiga).

**Entra:** `ops.login_attempt` (trilha de falha de autenticação — hoje só existe contador),
`portal.person_profile` e `portal.person_consent` (PII e base legal isoladas),
`portal.person_token` (recuperação de senha, verificação de e-mail e troca de endereço da pessoa, com `purpose` explícito — `M5`, 2026-11-17),
`game.session.channel` com FK composta para `game.server_channel` (topologia com um dono),
`content.arena` separado de `game.arena_*` (definição × partida), `ops.config_revision`/`config_change`
(overlay com revisão e autoria, em vez de sobrescrever a taxa base).

**Decisões que mexeram nesta lista e já fecharam** (corte vertical não travado por nenhuma): `A17`
— conta não é transferível nem vendável, então `0005` não tem objeto de titularidade; `A19` — loja e
cash existem, com 6 tabelas em `portal` e 4 em `game`, e `0011` mantém as 10; e `A18` — a conta nasce
só no portal e `game.account.person_id NOT NULL` é a fronteira. As três fecharam em 2026-11-19
(`lacunas.md` §2.2).

---

## 6. Troca da série: decidida, execução pendente

A troca está **decidida**: um commit tira os 53 herdados e põe os 29 na mesma pasta `migrations/`, e o
rollback é `git revert` desse commit ou a tag `pre-serie-1.0.0`. O que ficou **recusado em 2026-11-17**
foi o corte imediato e destrutivo — `git tag` de corte, `git rm` das 53 migrations, `docker compose
down -v`. A consequência está em §1.2: a série nova é escrita numa branch, na mesma pasta
`crates/wyd-db/migrations/`, e a herdada continua sendo a que o produto roda em `develop`.

**Por que a pasta não leva sufixo de versão:** o ledger tem a versão como chave primária única
(`src/migrate/spec.md` §3), então duas pastas no mesmo banco colidiriam já no `0001` — e a linha não
diria de qual pasta veio. Coexistir com
a herdada é problema de árvore, não de nome de pasta: resolve-se por **branch e tag**, como §1.2
descreve, e o rollback é `git revert`, não variável de ambiente.

**O que a execução pendente não impede:** escrever as 29 migrations na branch, aplicá-las num
PG17 efêmero, provar o corte vertical de §3.2 e medir o catálogo. Nada disso toca a série viva nem o
banco de desenvolvimento de quem usa a stack.

**O que fica bloqueado até a troca:**

| Item | Hoje (série herdada) | Na troca |
|---|---|---|
| diretório vivo | `migrations/` (53 arquivos, em `develop`) | `migrations/` (31 arquivos, no commit da troca) |
| `SCHEMA_VERSION` | `53` | `31` |
| `make migrations-apply` | série herdada | série nova |
| catálogo/doc | `public`, 87 tabelas | 4 schemas, 97 tabelas + 2 views |
| imagem/runner | `WYD_MIGRATIONS_DIR=/migrations` | mesmo caminho, conteúdo novo |

Procedimento da troca, **quando `0001`–`0007` estiverem verdes** (não executar antes): (1) `git tag pre-serie-1.0.0` no
commit atual; (2) commit só da troca: os 53 saem e os 31 entram na mesma pasta `migrations/`, vindos
da branch; (3) `SCHEMA_VERSION = 31`; (4) `provision render` + `provision apply` antes das migrations e `make stack-config && make
stack-up` com banco recriado (`down -v` só com pedido explícito); (5) `make migrations-apply`
(up/down/up), `make banco-catalogo && make banco-verify`; (6) docs na mesma entrega (`modelo-banco.md`,
`banco-catalogo.md`, `03-progresso.md`, issue no `roadmap-rust/`, `changelog.md`).

**Rollback:** `git revert` do commit da troca (os 53 voltam) ou checkout da tag `pre-serie-1.0.0`. O
rollback **não** é trocar `WYD_MIGRATIONS_DIR`: as duas séries usam o mesmo caminho. É por isso que a
troca não pode vir antes de `0001`–`0007` verdes: a série antiga intacta no histórico **é** o rollback.

---

## 7. Gates de aceite da série

| Gate | Critério |
|---|---|
| estrutura | 31 arquivos contíguos; 0 tabela ao fim do `down` (somando os 4 schemas) |
| catálogo | 97 tabelas + 2 views no documento gerado, todas com schema, nota e fonte |
| tipos | nenhum `serial`, `timestamp`, `money`, `varchar`, `enum` na série |
| constraints | 100% nomeadas; toda constraint tem teste que a viola e recebe o erro esperado |
| fronteira | `permission denied` para portal escrevendo em `game`; jogo lendo `portal.person` |
| conteúdo | contagens de §4 idênticas às medidas, `content_version = 7` |
| vertical | o corte de §3.2 passa com o cliente 7662 (login + PIN + seleção + 5 contas) |
| ambiente | o contrato de ambiente vale em todo binário: DSN ausente falha **nomeando-a** e nenhum lê `DATABASE_URL` (`src/provision/spec.md` §3) |

---

## 8. Riscos

| Risco | Efeito | Mitigação |
|---|---|---|
| Ferramentas não atualizadas antes do SQL | tabela "desaparece" do catálogo sem erro; `down` verde falso | adiado por decisão do dono (2026-11-17); mitigação enquanto isso = conferência manual de §7 e contagem por `psql` no PG17 efêmero; o PR (§1.3) volta quando autorizado |
| `search_path` ausente na conexão do servidor | query de `game.account` cai na tabela homônima de outro schema (ou falha) | schema explícito em toda query ou role com `search_path` fixo; teste de fronteira |
| Big bang sem build verde até `0007` | semanas sem `cargo test` útil | corte vertical de §3.2 como primeiro gate |
| Conteúdo escrito antes do corte vertical | retrabalho se o modelo de conta mudar | conteúdo é independente: só começa depois de `0001`–`0007` verdes |
| Seed escrito divergir do medido | perda silenciosa de itens/NPCs | conferir contagem e `sha256` por dataset em `content_source` antes de escrever o arquivo |
| Renomear 87 tabelas antigas | links e docs quebrados | catálogo gerado + `check-docs-consistency` + `modelo-banco.md` na mesma entrega |
| Série antiga apagada cedo demais | perde referência de comparação | não é apagada (§6): fica congelada em `migrations/` como referência e rollback |
| Duas séries divergirem em silêncio | ferramenta valida uma e a outra apodrece | todo alvo de verificação aceita `SERIE` e o PR de ferramentas roda nas duas (§1.3) |

---

## 9. Etapas de código (a execução da série)

1. **[adiado em 2026-11-17]** PR de ferramentas (`banco-parse`, `check-migrations`, `pg`, `catalogo.rs`,
   `src/migrate/`)
   + fixture, com o critério de pronto de §1.3 nas **duas** séries. Sem autorização, este passo não
   existe: o próximo é o 2.
2. `provision render` (gera `db/roles.sql`) + `provision apply` no banco (papéis e o estado de
   `public`, fora da série; `src/provision/spec.md` §4) + compose com uma conexão por role.
3. `0001`–`0007` em `migrations/`, na branch `serie-1.0.0`, com **corte vertical** provado no 7662
   (PG17 efêmero).
4. Em paralelo: `0008`–`0012` (estado de jogo restante + entrega) e `0013`–`0027` (conteúdo).
5. `0028`–`0029` (estado dependente de conteúdo + views).
6. Catálogo, `banco-verify`, docs, `make check` — ainda contra a branch
   (`--dir crates/wyd-db/migrations`).
7. **Trocar a série** (§6), com `0001`–`0007` verdes. Enquanto o passo 3 não fecha, o passo 7 não
existe.

Passo 3 é o que dá direito a continuar; o 7 é o único que muda o que roda. Todos os outros são
execução e não tocam a série viva.

**Enunciação (2026-11-18):** a **ordem de implantação** (provisionar papéis, aplicar a série,
semear, subir serviços, verificar) está em `docs/planos/plano-de-implantacao.md` §Ordem, com o
"como falha" de cada etapa. Esta seção é a ordem de **construção** da série — ferramentas, corte
vertical, troca — e não serve de playbook: quem instala segue a página. Passo novo de construção
entra aqui; de implantação, lá.

---

## 10. O que este plano não decide

Regra de produto (§5), retenção final por tabela (`M22`), se a loja/cash existe (`A19`), e o formato
de `audit_log.actor` (`M18`). **`M18` está decidido (2026-11-17):** a proposta `M18-A` foi aprovada —
`actor_kind` + `actor_id` + `actor_ref`, 3 CHECKs de coerência, índice `ix_audit_actor` e **sem FK por
desenho**, detalhada em `lacunas.md` §2.3. `0002_ops_audit.sql` nasce assim; não é mais pergunta
aberta. As três de produto travam `game.account`/`portal`, não o corte vertical.
