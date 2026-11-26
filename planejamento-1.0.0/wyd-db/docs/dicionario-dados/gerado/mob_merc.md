> Pergunta: "o que `content.mob_merc` guarda do mercenário e por que os itens ficam em JSON?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.21; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Gerador de mercenário por índice: template do NPC, intervalo de renovação do estoque, tempo de
renascimento e a lista de itens — ainda não normalizada, porque a estrutura interna do insumo não tem
consumidor provado.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `generator_index` | `INTEGER` | não | — | índice do gerador do mercador |
| `template_name` | `TEXT` | não | — | corpo do mercador (FK) |
| `renew_seconds` | `INTEGER` | não | — | intervalo de renovação do estoque |
| `reborn_seconds` | `INTEGER` | não | — | tempo de renascimento |
| `items` | `JSONB` | não | `'[]'` | lista de itens ainda não normalizada |

## Chave primária

`pk_mob_merc (generator_index)` — um mercador por posição de gerador.

## FKs de saída e de entrada

- **Saída, `RESTRICT`:** `fk_mob_merc_template`:
  `template_name → content.npc_template (template_name)`, `ON UPDATE RESTRICT`, `ON DELETE RESTRICT` —
  template de mercador não é apagado em silêncio.
- **Entrada:** nenhuma.
- **Sem FK cruzada com `npc_definition`:** o `generator_index` daqui e o de lá usam o mesmo vocabulário
  de índice, mas nenhuma constraint liga os dois (`V`, 2026-11-22: o único de `npc_definition` é
  `(map_id, generator_index)`, local àquela tabela).

## Unicidade

Só a PK — nenhum único adicional.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_mob_merc_indice` | `generator_index` negativo |
| `ck_mob_merc_renovacao` | `renew_seconds` negativo |
| `ck_mob_merc_renascimento` | `reborn_seconds` negativo |
| `ck_mob_merc_itens` | `items` que não é array JSON |

`renew_seconds` e `reborn_seconds` aceitam zero: o schema não distingue "sem renovação" de "renovação
imediata", e essa leitura é do runtime, não do banco.

## Índices

Nenhum além da PK. Sem carimbos e sem trigger: import puro.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed (`0020`); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` (default privilege de `content`, `0001_bootstrap.md` §7).
- **Não lê:** `wyd_admin` (`0032` não concede `content`). Dono da permissão:
  `migrations/0019_content_world.md` §8.

## Migration

Forma (dona): `migrations/0019_content_world.md` — arquivo `0019_content_world.sql`.
