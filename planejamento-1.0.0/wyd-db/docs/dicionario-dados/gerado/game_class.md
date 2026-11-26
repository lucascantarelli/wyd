> Pergunta: "o que `content.game_class` guarda além do nome e por que o teto de maestria mora nela?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.17; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Classe jogável, com uma coluna de regra: `mastery_cap`, o teto de maestria **por classe**. É a única
tabela de `0013` que guarda número de regra; `0008_game_skills.md` §10 manda o limite de maestria ser
lido daqui, e o `CHECK` global de `game.character_skill` (320) é só o teto do cliente. O conteúdo
(6 classes) está previsto no seed `0016`, que segue `bloqueada` até as contagens serem reconferidas na
fonte (`plano-migrations.md` §4).

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `slug` | `TEXT` | não | — | identificador da classe, `snake_case` de 2–32; PK |
| `name` | `TEXT` | não | — | nome exibido, 1–64 caracteres após aparar |
| `mastery_cap` | `SMALLINT` | não | `320` | teto de maestria da classe, 0–320; lido por `0008` |
| `enabled` | `BOOLEAN` | não | `true` | `false` aposenta a classe sem `DELETE` |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da linha |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

O default `320` é o teto global de `ck_character_skill_maestria`, não uma restrição: a classe só pode
**reduzir** o limite. Não há coluna de evolução (`evolution_of`), de requisito nem de stat inicial.

## Chave primária

`pk_game_class (slug)` — código estável como identidade (`P-04`). Nome não é chave.

## FKs de saída e de entrada

- **Saída:** nenhuma; o schema `content` não aponta para `game`/`portal`.
- **Entrada, `RESTRICT` (2 FKs):** `fk_skill_catalog_class` em `content.skill_catalog.class_slug`
  (`0015`) e `fk_quest_classe` em `content.quest.class_code` (`0026`), ambas `ON UPDATE RESTRICT` e
  `ON DELETE RESTRICT`. Classe referenciada não é apagada em silêncio.
- **Sem FK de `game`:** `game.character.class` e `content.level_item.class_id` são números do cliente,
  não o `slug` desta tabela — a correspondência numérica é do arquivo, não do banco
  (`docs/dicionario-dados/dominio-personagem.md` §"Armadilhas de leitura").

## Unicidade

Só a PK — nenhum único adicional.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_game_class_slug` | `slug` fora de `^[a-z][a-z0-9_]{1,31}$` |
| `ck_game_class_nome` | `name` vazio, só com espaço, ou acima de 64 caracteres |
| `ck_game_class_maestria` | `mastery_cap` fora de 0–320 |

`mastery_cap = 0` é aceito pelo `CHECK`; o que a classe com teto zero significa não está decidido no
planejamento — não inferir "classe sem skill" sem decisão registrada.

## Índices

Nenhum além do índice da PK. Trigger `trg_game_class_set_updated_at` (`BEFORE UPDATE`, por linha).

## Armadilha de leitura

`mastery_cap` default 320 **não** restringe classe nenhuma: quem quiser teto menor precisa de `UPDATE`
no seed (`0016`). E o teto não é o número do cliente: `ck_character_skill_maestria` continua sendo o
limite global de 320, então uma classe com `mastery_cap > 320` (recusada pela constraint) mudaria a
regra de maestria sem mudar a do cliente.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed (`0016`), que está `bloqueada`.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001_bootstrap.md` §7). Dono da permissão: `migrations/0013_content_taxonomy.md` §8.
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032_admin_grants.md` §5).

## Migration

Forma (dona): `migrations/0013_content_taxonomy.md` — arquivo `0013_content_taxonomy.sql`.
