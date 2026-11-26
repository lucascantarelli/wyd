> Pergunta: "o que `content.npc_template` guarda do arquivo do NPC e por que o blob de 816 B não entra?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.21; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Identidade do corpo do NPC por `template_name`, com a prova do arquivo de origem: nome do arquivo,
tamanho declarado, hash e codificação do nome. Não guarda o `STRUCT_MOB` binário de 816 B — nenhum
consumidor da 1.0.0 lê o blob, e `sha256` mais `source_size` preservam a prova do insumo.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `template_name` | `TEXT` | não | — | nome do template (chave do `STRUCT_MOB`) |
| `file_name` | `TEXT` | não | — | arquivo de origem do template |
| `source_size` | `SMALLINT` | não | — | tamanho declarado na origem, não o medido no arquivo |
| `sha256` | `TEXT` | não | — | hash do blob de origem (o blob não entra) |
| `encoding_origin` | `TEXT` | não | `'utf8'` | codificação do nome na origem |
| `name_recovered` | `TEXT` | sim | — | nome recuperado por decodificação alternativa |

`name_recovered` só é aceito com `encoding_origin = 'cp949-as-latin1'`: nome "recuperado" sobre origem
UTF-8 não faz sentido e é recusado.

## Chave primária

`pk_npc_template (template_name)` — o nome do template é a chave natural; não há id próprio nem tabela
de arquivo separada.

## FKs de saída e de entrada

- **Saída:** nenhuma.
- **Entrada, `RESTRICT`:** `npc_definition` (`fk_npc_definition_template`) e `mob_merc`
  (`fk_mob_merc_template`), em `0019` — apagar template referenciado é decisão de seed, não de runtime.
- **Entrada, `CASCADE`:** `mob_template_stat` (`fk_mob_template_stat_template`), `mob_template_equip`
  (`fk_mob_template_equip_template`) e `drop_rate_override` (`fk_drop_rate_override_template`), em
  `0025` — a linha filha cai com o pai.

## Unicidade

Só a PK: dois arquivos com o mesmo `file_name` são aceitos, porque o nome do template é o que identifica
o corpo.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_npc_template_nome` | `template_name` vazio, só com espaço, ou acima de 96 caracteres |
| `ck_npc_template_arquivo` | `file_name` vazio, só com espaço, ou acima de 128 caracteres |
| `ck_npc_template_tamanho` | `source_size` fora de 756, 816 e 920 |
| `ck_npc_template_sha256` | hash que não casa `^[0-9a-f]{64}$` |
| `ck_npc_template_codificacao` | `encoding_origin` fora de `utf8`/`cp949-as-latin1` |
| `ck_npc_template_recuperado` | `name_recovered` preenchido com origem `utf8`, ou fora de 1–96 caracteres |

## Índices

Nenhum além da PK. Sem carimbos e sem trigger: import puro.

## Armadilha de leitura

`source_size` aceita 756 e 920 além de 816 porque **222 linhas do legado declaram tamanho errado**
(`H`, `migrations/0019_content_world.md` §6.2). O `CHECK` registra o defeito em vez de escondê-lo: o
tamanho aqui é o declarado na origem, não o medido no arquivo.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed (`0020`); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` (default privilege de `content`, `0001_bootstrap.md` §7).
- **Não lê:** `wyd_admin` (`0032` não concede `content`). Dono da permissão:
  `migrations/0019_content_world.md` §8.

## Migration

Forma (dona): `migrations/0019_content_world.md` — arquivo `0019_content_world.sql`.
