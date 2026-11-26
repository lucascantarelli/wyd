> Pergunta: "o que `content.region` guarda do Regions.txt e por que o nome não é único?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.21; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada.

## Papel

Região de PvP/siege como retângulo nomeado do mapa. É a **dona única do retângulo**: `map_definition`
(`0024`) aponta para cá em vez de repetir coordenada.

## Colunas
**Sem arquivo no repositório:** `Regions.txt` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | — | identidade da região; o arquivo não a numera de forma estável |
| `name` | `TEXT` | não | — | nome exibido; repetido é legítimo |
| `name_fold` | `TEXT` gerada `STORED` | não | — | expressão `content.fn_fold(name)` |
| `x1` / `y1` / `x2` / `y2` | `INTEGER` | não | — | cantos do retângulo |
| `source_line` | `INTEGER` | não | — | linha do `Regions.txt` |
| `enabled` | `BOOLEAN` | não | `true` | aposentadoria sem `DELETE` |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | carimbos; `updated_at` pelo trigger |

`name_fold` nunca é nula porque `name` é `NOT NULL` e `content.fn_fold` devolve `NULL` só para entrada
nula — mas a coluna **não declara** `NOT NULL`, então o catálogo a reporta como anulável.

## Chave primária
**Sem arquivo no repositório:** `Regions.txt` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

`pk_region (id)` — identidade própria porque o `Regions.txt` **não numera a região de forma estável**;
o nome sozinho não serve, já que Erion e Armia aparecem em mais de um retângulo.

## FKs de saída e de entrada

- **Saída:** nenhuma.
- **Entrada, `RESTRICT`:** `map_definition.region_id` (`fk_map_definition_region`, `0024`) — mapa
  aponta para a região que o recorta; apagar região referenciada é decisão de seed.

## Unicidade

`uq_region_retangulo (name, x1, y1, x2, y2)` — o único é o **retângulo inteiro**, não o nome: duas
regiões podem compartilhar nome e não podem compartilhar nome e as quatro coordenadas.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_region_nome` | `name` vazio, só com espaço, ou acima de 64 caracteres |
| `ck_region_retangulo` | `x2 < x1` ou `y2 < y1` |
| `ck_region_origem` | `source_line` menor ou igual a zero |

O `CHECK` do retângulo **não tem teto**: `y2 = 6841` é aceito porque o limite de 4095 é regra do loader
do cliente, não do banco (`migrations/0024_content_maps.md` §4 registra o caso real).

## Índices

| Índice | Consulta que justifica |
|---|---|
| `ix_region_name_fold (name_fold text_pattern_ops)` | busca por prefixo do nome dobrado (`P-11`) |

Trigger `trg_region_set_updated_at`. `name_fold` sai de `content.fn_fold(name)` (`0013`) e não tem
`CHECK` próprio: a normalização é da função, não da tabela.

## Papéis

- **Escreve:** `wyd_migrator`, na carga e no seed (`0020`); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` (default privilege de `content`, `0001_bootstrap.md` §7).
- **Não lê:** `wyd_admin` (`0032` não concede `content`). Dono da permissão:
  `migrations/0019_content_world.md` §8.

## Migration

Forma (dona): `migrations/0019_content_world.md` — arquivo `0019_content_world.sql`.
