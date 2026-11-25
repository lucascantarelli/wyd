> Pergunta: "o que `content.mob_template_stat` guarda de cada template de mob e por que a chave é o nome do template?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.25; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Stats de combate do template de NPC: nível, PV, ataque e defesa física e mágica, absorção, EXP e faixa
de ouro. É linha **possuída** do template, não uma entidade com identidade própria: o `template_name` é a
PK e a FK ao mesmo tempo. Tabela de conteúdo, seed-only e read-only em runtime — a fórmula de combate é
do `wyd-server`, não do banco (`padrao-banco.md` §0).

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `template_name` | `TEXT` | não | — | nome do template de `content.npc_template`; PK |
| `level` | `SMALLINT` | não | `1` | nível do mob, 1–400 |
| `max_hp` | `BIGINT` | não | `1` | PV máximo |
| `attack` | `INTEGER` | não | `0` | ataque físico |
| `defense` | `INTEGER` | não | `0` | defesa física |
| `absorb` | `INTEGER` | não | `0` | absorção de dano |
| `magic_attack` | `INTEGER` | não | `0` | ataque mágico |
| `magic_defense` | `INTEGER` | não | `0` | defesa mágica |
| `exp_reward` | `BIGINT` | não | `0` | EXP concedida ao morrer |
| `gold_min` | `BIGINT` | não | `0` | ouro mínimo dropado |
| `gold_max` | `BIGINT` | não | `0` | ouro máximo dropado; `>= gold_min` |

Todo atributo tem `DEFAULT`, então uma linha criada só com `template_name` passa: o schema não distingue
"mob sem ataque" de "stat não preenchido" além do default. Sem `created_at`/`updated_at`, trigger ou
coluna de origem — a proveniência do dataset é `content.content_source` (`0019`).

## Chave primária

`pk_mob_template_stat (template_name)` — um registro por template (`P-04`): o stat é o próprio template,
não uma tabela filha com `id`.

## FKs de saída e de entrada

- **Saída, `CASCADE` (1 FK):** `fk_mob_template_stat_template`:
  `template_name → content.npc_template (template_name)`, `ON UPDATE RESTRICT`, `ON DELETE CASCADE`.
  Apagar o template leva o stat junto, porque o stat não existe sem ele (`P-15`).
- **Entrada:** nenhuma; nenhuma tabela da série aponta para `mob_template_stat`.

## Unicidade

Só a PK: a identidade é o template, e não há segundo único.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_mob_template_stat_nivel` | `level` fora de 1–400 |
| `ck_mob_template_stat_hp` | `max_hp` menor ou igual a zero |
| `ck_mob_template_stat_ataque` | `attack` ou `magic_attack` negativo |
| `ck_mob_template_stat_defesa` | `defense`, `magic_defense` ou `absorb` negativo |
| `ck_mob_template_stat_exp` | `exp_reward` negativo |
| `ck_mob_template_stat_ouro` | `gold_min` negativo, ou `gold_max` menor que `gold_min` |

`gold_max = gold_min` é válido: ouro fixo não é dado inválido.

## Índices

Nenhum além do índice da PK. A FK usa a própria PK como prefixo e não há coluna de busca secundária;
`content.npc_template` é o lado pai, e o índice de lá é o que sustenta o `CASCADE`.

## Armadilha de leitura

O teto `level <= 400` é o mesmo do personagem, não uma faixa medida do mob: a ficha do legado só dava a
coluna, e a faixa é `H`. `exp_reward` e as duas colunas de ouro não têm unidade provada no planejamento
novo (`H`) — são BIGINT de conteúdo, e quem converte é o runtime. Zero em `gold_max` com `gold_min` zero
significa "sem ouro", não "não configurado"; `mob_template_stat` não tem flag de completude.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed de origem (`0020`/`0023`, ambos `bloqueada`,
  `migrations-checklist.md` §2); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001-bootstrap.md` §7).
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032-admin-grants.md` §5).

## Migration

Forma (dona): `migrations/0025-content-combat.md` — arquivo `0025-content-combat.sql`.
