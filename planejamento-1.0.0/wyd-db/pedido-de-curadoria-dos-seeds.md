> Pergunta: "o que a curadoria precisa reconferir, exatamente, para os seis seeds saírem de `bloqueada`?"
> Público: Curadoria de conteúdo, dono do cliente 7662 e equipe do `wyd-db`
> Dono: equipe do `wyd-db` com a curadoria de conteúdo
> Base: `plano-migrations.md` §4; `docs/planos/plano-de-dados-e-seeds.md` §2–§3; `migrations-checklist.md` §3; `ddl-confirmado.md` §5.17–§5.26; seções `H` de `docs/dicionario-dados/`
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-24)

Este documento é o **pedido de curadoria** dos seis seeds que seguem `bloqueada`: `0016`, `0017`,
`0018`, `0020`, `0023` e `0027`. Ele diz, arquivo por arquivo, **o que reconferir** no cliente 7662,
**em que unidade** e **em que formato a medição volta** — e não contém nenhuma linha de seed, nenhuma
contagem nova e nenhum valor estimado. O que está escrito aqui é pedido, não resultado.

## 1. O que está bloqueado e por quê

A fonte dos seis é o **cliente 7662 medido** (decisão do dono, 2026-11-23), e nenhuma contagem de
2026-11-08/15 vira verdade sem reconferência (`plano-migrations.md` §4). Enquanto a medição não voltar,
o arquivo não é escrito: os seis estão fora da onda A por `Q-M33-6`, a série de 28 migrations anda sem
eles, e o risco é `R-6` (`docs/planos/riscos.md`).

| # | Arquivo | Dataset | Contagem declarada a reconferir |
|---:|---|---|---|
| 16 | `0016_seed_taxonomy.sql` | taxonomia | 20 sistemas, 6 classes, 13 linhagens, 16 categorias |
| 17 | `0017_seed_item_catalog.sql` | catálogo de item | 3.220 itens + 55 `EF_*` |
| 18 | `0018_seed_skill_catalog.sql` | catálogo de skill | 151 skills + 3 Kersef (154), 63 com linhagem |
| 20 | `0020_seed_world.sql` | mundo | 1.991 templates, 6.100 NPCs, 6.706 slots de loja, 161 `level_item`, 52 regiões, 49 itens de chão |
| 23 | `0023_seed_rules.sql` | regras | 14 passos de refino, 4 gemas, SANC, combine, acessório |
| 27 | `0027_seed_quests_events.sql` | quests e eventos | cadeia de quests do 7662, 3 `world_event` + agenda, 2 arenas |

**O que já está decidido e não se reabre aqui:** seed é conteúdo versionado, não estado de jogador; é
escrito à mão, sem gerador; correção de dado acontece **na origem**, antes do seed, e não vira
migration de correção; cada dataset carrega `content_source` (`path`, `sha256`, `size_bytes`,
`row_count`); `content_version` incrementa uma vez por seed aplicado e fecha em **7**
(`plano-migrations.md` §4; `plano-de-dados-e-seeds.md` §3).

## 2. O que o `wyd-db` já tem, e o que ele não tem

O **DDL está fechado** nas seis frentes — o que falta é dado, não desenho. É por isso que o pedido pode
nomear coluna:

| Seed | Bloco de DDL fechado | Tabelas que o seed preenche |
|---|---|---|
| `0016` | §5.17 | `content.game_system`, `content.game_class`, `content.skill_lineage`, `content.item_kind` |
| `0017` | §5.17 e §5.18 | `content.item_effect`, `content.item_catalog` |
| `0018` | §5.19 | `content.skill_catalog` |
| `0020` | §5.21 | `content.npc_template`, `npc_definition`, `npc_shop_item`, `level_item`, `region`, `world_item_seed` e os conjuntos sem contagem declarada |
| `0023` | §5.22 e §5.23 | `content.refine_rate`, `refine_gem_effect`, `sanc_rate`, `combine_value`, `accessory_refine_rule` |
| `0027` | §5.26 | `content.quest`, `quest_step`, `quest_reward`, `world_event`, `event_schedule`, `arena` |

**O que não existe:** nenhuma linha de seed escrita, nenhuma medição do 7662 registrada com `sha256` e
nenhum gerador. O pedido não pede ferramenta; pede medição.

## 3. O pedido, seed por seed

Cada bloco tem a mesma forma: o que reconferir, a unidade, onde o valor entra e **o que faz a medição ser
aceita**. `H` marca semântica que o dicionário já declara não verificada — é o que a curadoria fecha.

### 3.1 `0016` — taxonomia

| O que reconferir | Unidade | Onde entra | Aceite da medição |
|---|---|---|---|
| os **20** sistemas de jogo | 1 linha por sistema | `game_system.slug`, `name` | contagem fecha em 20; `slug` único |
| as **6** classes | 1 linha por classe | `game_class.slug`, `name`, `mastery_cap` | contagem fecha em 6; dizer se o teto por classe vem do cliente ou fica no default 320 |
| as **13** linhagens de skill | 1 linha por linhagem | `skill_lineage.slug`, `name` | contagem fecha em 13; `slug` único |
| as **16** categorias de item | 1 linha por categoria | `item_kind.slug`, `name` | contagem fecha em 16; `slug` único |
| grafia dos nomes | texto | `name` | nome como está no cliente; `slug` em minúsculas, sem espaço nem acento |

### 3.2 `0017` — catálogo de item

| O que reconferir | Unidade | Onde entra | Aceite da medição |
|---|---|---|---|
| os **55** efeitos `EF_*` | 1 linha por efeito | `item_effect.slug`, `name` | contagem fecha em 55; o conjunto é **exatamente** o dos efeitos habilitados no cliente (`H`) |
| os **3.220** itens | 1 linha por item | `item_catalog` | contagem fecha em 3.220; `item_index` único e dentro de 0–32767 |
| índice, nome, categoria e efeitos | campo por item | `item_index`, `name`, `kind_slug`, `effect_names` | todo `kind_slug` existe nas 16 categorias de `0016`; todo efeito citado existe nos 55 |
| preço, grau e requisitos | campo por item | `base_price`, `grade`, `req_level`, `req_str`, `req_int`, `req_dex`, `req_con` | inteiro ≥ 0; para `grade` e `requirements`, entregar a **lista de valores observados** — a semântica é `H` e não se deduz |
| itens de loja fora do catálogo | chave | `npc_shop_item.item_index` | os **22** casos medidos no insumo (`H` do domínio `mundo`) entram como lista de `item_index` sem correspondência, com o NPC de origem |

### 3.3 `0018` — catálogo de skill

| O que reconferir | Unidade | Onde entra | Aceite da medição |
|---|---|---|---|
| as **151** skills + as **3** da Kersef | 1 linha por skill | `skill_catalog.skill_index`, `name` | contagem fecha em 154; `skill_index` único e ≤ 247 |
| classe e linhagem de cada skill | campo por skill | `class_slug`, `lineage_slug` | **63** com linhagem, 91 sem; todo `class_slug` existe nas 6 classes de `0016` |
| os valores de combate e custo | campo por skill | `skill_point`, `mana_spent`, `delay_ticks`, `affect_time`, `range_tiles`, `max_target`, `party`, `passive`, `aggressive` | inteiro ≥ 0; tipo (`target_type`, `instance_type`, `tick_type`, `affect_type`) com a lista de valores observados |
| por que **3** skills vêm da Kersef | nota | — | a curadoria diz se é origem de conteúdo, override de cliente ou dado herdado (`H`); a linha não é escrita sem isso |

### 3.4 `0020` — mundo

| O que reconferir | Unidade | Onde entra | Aceite da medição |
|---|---|---|---|
| **1.991** templates de NPC | 1 linha por template | `npc_template.template_name`, `file_name`, `source_size`, `encoding_origin` | contagem fecha em 1.991; `template_name` único |
| **6.100** NPCs | 1 linha por NPC | `npc_definition` (`map_id`, `pos_x`, `pos_y`, `route_type`, `merchant`, `generator_index`, `min_group`, `max_group`, `formation`) | contagem fecha em 6.100; todo `template_name` existe nos 1.991 |
| **6.706** slots de loja | 1 linha por slot | `npc_shop_item.npc_id`, `slot`, `item_index`, `quantity` | contagem fecha em 6.706; `slot` único por NPC; `quantity` inteiro ≥ 1 |
| **161** `level_item` | 1 linha por nível | `level_item` (`ordinal`, `level`, `class_id`, `type_id`, `item_index`, `extras`) | contagem fecha em 161; `ordinal` único |
| **52** regiões | 1 linha por região | `region.name`, `source_line` | contagem fecha em 52; nome único |
| **49** itens de chão | 1 linha por item | `world_item_seed` (`map_id`, `item_index`, `pos_x`, `pos_y`, `rotate`) | contagem fecha em 49; `map_id` e `item_index` resolvem contra mapa e catálogo |
| **decodificação dos nomes** | regra | `name_recovered` | dizer qual conversão foi aplicada (`cp949-as-latin1` é a registrada, `H`); nome sem `?` nem caractere de substituição |
| conjuntos **sem contagem declarada** | presença/ausência | `game_setting`, `game_matrix`, `mob_merc`, `guard_spawn`, `guard_spawn_reward`, `guild_zone` | para cada um: ou a contagem medida no cliente, ou "ausente no 7662" com o motivo — nunca 0 por suposição |
| **mapas** (`map_definition`, `teleport_route`) | fora deste pedido | — | pertencem ao bloco `0024`, que não é seed; se o dado existir no cliente, registrar como insumo de outra frente |

### 3.5 `0023` — regras

| O que reconferir | Unidade | Onde entra | Aceite da medição |
|---|---|---|---|
| **14** passos de refino | 1 linha por passo e família | `refine_rate.kind`, `step`, `rate_bps` | contagem de passos fecha em 14; `step` contíguo por `kind`; `rate_bps` entre 0 e 10000 |
| as **4** gemas | 1 linha por gema e efeito | `refine_gem_effect` (`item_index`, `effect_code`, `flat_value`, `per_level_value`, `min_refine`, `max_refine`, `allows_ancient`) | contagem fecha em 4 itens de gema; `item_index` resolve no catálogo de `0017` |
| SANC | 1 linha por pó e índice | `sanc_rate.powder`, `refine_index`, `rate_bps` | lista **explícita** das células presentes **e** das ausentes (`H`: a regra de célula ausente é do runtime) |
| combine | 1 linha por família e chave | `combine_value.family`, `key`, `rate_bps`, `coin` | famílias e chaves listadas; `coin` inteiro ≥ 0; nenhuma chave sem família |
| regra de acessório | 1 linha por receita e passo | `accessory_refine_rule` (`kind`, `recipe`, `step_from`, `step_to`, `rate_bps`, `fail_mode`, `fail_bonus_item`, `fail_bonus_bps`, `gold_cost`, `material_item`, `material_name`, `material_qty`) | `fail_mode` com a lista de valores observados; se a receita **não existir no 7662**, entregar como ausente com o motivo (`H`: origem posterior ao 7662) |
| semântica de `step` e das famílias | nota | — | a curadoria liga cada `kind`/`family` ao assunto que ele governa; sem isso a linha é `H` |

### 3.6 `0027` — quests e eventos

| O que reconferir | Unidade | Onde entra | Aceite da medição |
|---|---|---|---|
| a **cadeia de quests** do 7662 | 1 linha por quest | `quest` (`code`, `name`, `kind`, `min_level`, `class_code`, `repeatable`, `cooldown_seconds`, `requires`, `description`) | contagem e **ordem** da cadeia declaradas; `code` único |
| os passos de cada quest | 1 linha por passo | `quest_step` (`quest_code`, `step`, `objective`, `target_id`, `count`, `map_id`, `pos_x`, `pos_y`) | `step` contíguo a partir de 1; todo `map_id` resolve |
| as recompensas | 1 linha por recompensa | `quest_reward` (`quest_code`, `kind`, `item_index`, `amount`) | `amount` inteiro ≥ 1; `item_index` resolve no catálogo quando `kind` é item |
| **3** eventos de mundo | 1 linha por evento | `world_event` (`code`, `name`, `kind`, `enabled`, `requires_city`, `description`) | contagem fecha em 3; `code` único; `kind` com a lista de valores observados (`H`) |
| a agenda dos eventos | 1 linha por janela | `event_schedule` (`event_code`, `weekday`, `start_minute`, `duration_minutes`, `timezone`, `every_n_weeks`, `enabled`) | `timezone` **explícito** em cada linha; `start_minute` 0–1439; `every_n_weeks` inteiro ≥ 1 |
| **2** arenas | 1 linha por arena | `arena` (`code`, `name`, `kind`, `team_size`, `rounds`, `round_ms`, `respawns`, `min_level`, `no_drop`) | contagem fecha em 2; inteiros ≥ 0 |
| semântica de `requires` e do reset diário | nota | — | formato do pré-requisito e regra de reset do dia, como o cliente aplica (`H` do domínio `quests`) |

### 3.7 Uma contagem sem arquivo dono

As **4 cidades** aparecem na lista de contagens a preservar (`plano-migrations.md` §4) **sem arquivo
atribuído**, e não existe seed de economia entre os seis. A curadoria deve dizer se o dado vem do 7662 e
em qual dataset ele entra; até lá, a contagem fica declarada e sem dono — e não é escrita em seed
nenhum.

## 4. Formato de entrega da medição

A entrega é **uma pasta por dataset** (`0016/`, `0017/`, `0018/`, `0020/`, `0023/`, `0027/`), cada uma
com três arquivos:

| Arquivo | Formato | Conteúdo obrigatório |
|---|---|---|
| `<dataset>.tsv` | TSV, UTF-8 sem BOM, primeira linha = cabeçalho | uma linha por entidade, **na ordem de colunas do §3**; campo vazio só onde o DDL admite nulo |
| `source.txt` | texto, uma chave por linha | `path=`, `sha256=`, `size_bytes=`, `row_count=`, `medido_em=`, `medido_por=`, `cliente=7662`, e `regra_de_decodificacao=` quando houver |
| `ausentes.tsv` | TSV, cabeçalho `chave` + `motivo` | o que existe no DDL e **não** existe no cliente: célula de SANC, receita de acessório, conjunto sem contagem, item de loja sem catálogo |

**Regras da entrega:**

1. **Nenhum valor estimado.** Contagem que não fecha com a declarada em §1 vem com a explicação e a
   diferença, não com o número ajustado.
2. **Ausência é dado.** O que não existe no cliente vai em `ausentes.tsv`; preencher com `0`, com a
   média dos pares ou com linha "parecida" reprova a entrega inteira.
3. **Procedência é parte do dado.** Dataset sem `sha256` e `row_count` não é aceito: é o que prova que o
   arquivo veio do cliente medido e não foi editado depois (`plano-de-dados-e-seeds.md` §3).
4. **Texto como está no cliente.** Nome não é traduzido nem normalizado; a conversão de codificação, se
   houver, é declarada em `source.txt` e o nome final é o decodificado.
5. **Nada de segredo, binário ou dado de jogador.** A entrega é conteúdo: não inclui executável do
   cliente, print como substituto de tabela, conta, personagem, saldo nem credencial.
6. **Uma medição por dataset.** Se dois datasets saírem do mesmo arquivo de origem, o `sha256` se repete
   e o `path` diz qual é.

## 5. Como o `wyd-db` confere e o que muda quando a entrega chega

| Passo | O que acontece | Falha se |
|---|---|---|
| 1 | conta as linhas do `.tsv` e compara com `row_count` e com a contagem de §1 | divergir sem explicação |
| 2 | recalcula o `sha256` do arquivo de origem | não bater com `source.txt` |
| 3 | resolve as chaves contra os outros datasets (categoria, efeito, template, mapa, catálogo) | chave órfã que não está em `ausentes.tsv` |
| 4 | confere unicidade das chaves naturais | `item_index`, `skill_index`, `template_name`, `code` ou `slot` repetido |
| 5 | só então escreve a página do seed e, aprovada, o arquivo `.sql` à mão | qualquer passo anterior falhar |

Aprovada a entrega, o efeito é: a linha do arquivo em `migrations-checklist.md` §3 sai de `bloqueada`;
a página da migration é escrita; o seed entra na série imediatamente depois do DDL do seu domínio
(`plano-migrations.md` §2); `content_version` incrementa uma vez. Nada disso acontece neste documento.

## 6. O que este pedido não faz

- **Não escreve as páginas** dos seis seeds nem o `.sql`: a escrita é do lote seguinte, depois da
  entrega conferida.
- **Não inventa dado nem contagem:** repete as declaradas, com a data de origem, e trata todas como `H`
  até a medição voltar.
- **Não altera o DDL** nem reabre `Q-M33-6`: os seis continuam fora da onda A.
- **Não fixa prazo** nem data de entrega: o pedido é de escopo e formato, não de calendário.
- **Não decide a atualização pós-`apply`** do conteúdo, que continua sem formato definido
  (`plano-de-dados-e-seeds.md` §8).
- **Não abre lacuna nova:** o bloqueio é o dos seis arquivos em `migrations-checklist.md` §3 e o risco
  `R-6`; este documento é o instrumento do pedido, não um registro paralelo.
