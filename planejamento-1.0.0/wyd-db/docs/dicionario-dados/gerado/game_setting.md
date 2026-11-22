> Pergunta: "o que `content.game_setting` guarda e por que um parâmetro cabe em duas colunas de valor?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.21; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — DDL confirmado, série não aplicada.

## Papel

Parâmetro do `gameconfig.txt` por seção e chave — o valor base que o servidor lê no boot.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `section` | `TEXT` | não | — | seção do arquivo, 1–64 caracteres após aparar; PK com `key` |
| `key` | `TEXT` | não | — | chave dentro da seção, 1–64 caracteres após aparar |
| `value_int` | `BIGINT` | sim | — | inteiro, só quando o parâmetro é um inteiro único |
| `value_text` | `TEXT` | não | `''` | forma textual, inclusive para valor de vários números (`evstart`) |
| `source_line` | `INTEGER` | não | — | linha do arquivo de origem, `> 0` |
| `created_at` | `TIMESTAMPTZ` | não | `now()` | criação da linha |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | mantido pelo trigger |

## Chave primária

`pk_game_setting (section, key)` — o par é a chave; não há id próprio nem tabela de seção à parte.

## FKs de saída e de entrada

- **Saída:** nenhuma.
- **Entrada por texto, sem FK:** `ops.config_change` (`0002`) descreve o alvo da edição como
  `target_schema` + `target_table` + `target_key`; não existe FK de `ops` para `content`, e a conferência
  de que o valor aplicado é o registrado é de código (`migrations/0002-ops-audit.md` §7).

## Unicidade

Só a PK: o par não se repete, e dois parâmetros diferentes podem vir da mesma `source_line`.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_game_setting_secao` | `section` vazia, só com espaço, ou acima de 64 caracteres |
| `ck_game_setting_chave` | `key` vazia, só com espaço, ou acima de 64 caracteres |
| `ck_game_setting_origem` | `source_line` menor ou igual a zero |

Nenhum `CHECK` exige que um dos dois valores esteja preenchido: `value_int` nulo com `value_text` vazio é
linha aceita, e a escolha de qual coluna ler é do runtime.

## Índices

Nenhum além da PK. Trigger `trg_game_setting_set_updated_at`.

## Papéis

- **Escreve:** `wyd_migrator` — carga e seed (`0020`).
- **Lê:** `wyd_game`, `wyd_portal` e `wyd_read` (`SELECT` default de `content` em `0001`).
- **Não escreve em runtime:** o default privilege de `0001` concede só `SELECT` em `content`, e a série
  não tem `GRANT` de `UPDATE` aqui. O comentário de §5.21 diz que o portal edita esta tabela; como o papel
  não recebe `UPDATE` em `content`, essa frase não descreve permissão vigente.

## Migration

Forma (dona): `migrations/0019-content-world.md` — arquivo `0019_content_world.sql`.
