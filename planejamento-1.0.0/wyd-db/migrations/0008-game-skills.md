> Pergunta: "o que o `0008_game_skills` cria no banco para skills, linhagem, afetos e chaos?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3; `ddl-confirmado.md` §5.11; `padrao-banco.md` §1/§5/§9; `lacunas.md` §2.3
> Revisado em: 2026-11-20

**Estado:** aceito (2026-11-24)

Terceiro arquivo do bloco de estado do jogador. Ele cria o que a ficha carrega **depois** de
identidade, atributos e inventário: skills aprendidas com maestria, a máscara de aprendizado por
categoria, maestria por linhagem, afetos (buffs/debuffs) e os pontos de chaos.

!!! note "Aprovado em 2026-11-20"
    O dono aprovou o lote na rodada de perguntas de 2026-11-20, na opção "Sim, aprovar como está",
    depois de escolher as quatro formas de §10. O DDL abaixo é o confirmado em `ddl-confirmado.md`
    §5.11, as cinco tabelas entram nos 27 de 92 e este arquivo está `concluída` no checklist.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0008` |
| Arquivo | `crates/wyd-db/migrations/0008-game-skills.sql` |
| Schema | `game` |
| Posição | 8 de 34 (segundo arquivo do bloco de estado do jogador) |
| Depende de | `0001` (schemas, `ops.fn_set_updated_at`, default privileges) e `0006` (`game.character` — todas as cinco tabelas têm FK para ela) |
| É dependência de | nenhum arquivo da série: `skill_index` e `lineage_code` **não** têm FK para o catálogo, que só nasce em `0013`/`0015` |
| Tabelas criadas | 5 (`character_skill`, `character_learned_skill`, `character_lineage`, `character_affect`, `character_chaos`) |
| Objetos criados | 5 tabelas, 3 índices, 5 triggers |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca de série (`plano-migrations.md` §6) |

## 2. Objetivo

Guardar, por personagem, **quais skills foram aprendidas e com que maestria**, **quais categorias
estão marcadas na máscara de aprendizado**, **quanto de maestria cada linhagem acumulou**, **quais
afetos estão ativos e por quanto tempo** e **o valor de chaos** — sem duplicar nada que pertença a
`character` (`0006`), inventário (`0009`), guilda (`0010`) ou catálogo (`0013`+).

## 3. Pré-requisitos

1. `0001_bootstrap` aplicado: `game`, `ops.fn_set_updated_at()` e os default privileges.
2. `0005_game_account` e `0006_game_character` aplicados: `game.character` existe; sem ela a FK falha
   com `42P01`.
3. `content.content_version` com a linha `id = 1` — `game.character.content_version` tem FK para ela,
   então provar este lote exige o seed de versão antes.
4. Conexão como `wyd_migrator`.

## 4. DDL do `up`

Texto idêntico ao de `ddl-confirmado.md` §5.11.

```sql
-- migrate:up
CREATE TABLE game.character_skill (
    character_id BIGINT NOT NULL,
    skill_index  SMALLINT NOT NULL,
    mastery      SMALLINT NOT NULL DEFAULT 0,
    learned_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_character_skill PRIMARY KEY (character_id, skill_index),
    CONSTRAINT fk_character_skill_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_character_skill_indice CHECK (skill_index BETWEEN 0 AND 247),
    CONSTRAINT ck_character_skill_maestria CHECK (mastery BETWEEN 0 AND 320)
);

-- Sem FK para `content.skill_catalog` (0015): o catálogo é de arquivo posterior e cobre só
-- parte dos 248 índices do cliente.
CREATE INDEX ix_character_skill_indice ON game.character_skill (skill_index);
CREATE TRIGGER trg_character_skill_set_updated_at
    BEFORE UPDATE ON game.character_skill
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.character_learned_skill (
    character_id BIGINT NOT NULL,
    category     SMALLINT NOT NULL,
    mask         SMALLINT NOT NULL DEFAULT 0,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_character_learned_skill PRIMARY KEY (character_id, category),
    CONSTRAINT fk_character_learned_skill_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_character_learned_skill_categoria CHECK (category BETWEEN 0 AND 3),
    CONSTRAINT ck_character_learned_skill_mascara CHECK (mask BETWEEN 0 AND 255)
);
CREATE TRIGGER trg_character_learned_skill_set_updated_at
    BEFORE UPDATE ON game.character_learned_skill
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.character_lineage (
    character_id BIGINT NOT NULL,
    lineage_code TEXT NOT NULL,
    mastery      SMALLINT NOT NULL DEFAULT 0,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_character_lineage PRIMARY KEY (character_id, lineage_code),
    CONSTRAINT fk_character_lineage_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_character_lineage_codigo CHECK (lineage_code ~ '^[a-z][a-z0-9_]{1,31}$'),
    CONSTRAINT ck_character_lineage_maestria CHECK (mastery BETWEEN 0 AND 320)
);

-- Sem FK para `content.skill_lineage` (0013): catálogo de conteúdo é de arquivo posterior.
CREATE INDEX ix_character_lineage_codigo ON game.character_lineage (lineage_code);
CREATE TRIGGER trg_character_lineage_set_updated_at
    BEFORE UPDATE ON game.character_lineage
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.character_affect (
    character_id    BIGINT NOT NULL,
    affect_type     SMALLINT NOT NULL,
    value           SMALLINT NOT NULL,
    level           INTEGER NOT NULL DEFAULT 0,
    remaining_ticks BIGINT NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_character_affect PRIMARY KEY (character_id, affect_type),
    CONSTRAINT fk_character_affect_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_character_affect_tipo CHECK (affect_type >= 0),
    CONSTRAINT ck_character_affect_nivel CHECK (level >= 0),
    CONSTRAINT ck_character_affect_restante CHECK (remaining_ticks >= 0)
);
CREATE INDEX ix_character_affect_tipo ON game.character_affect (affect_type);
CREATE TRIGGER trg_character_affect_set_updated_at
    BEFORE UPDATE ON game.character_affect
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.character_chaos (
    character_id BIGINT NOT NULL,
    cp           SMALLINT NOT NULL DEFAULT 0,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_character_chaos PRIMARY KEY (character_id),
    CONSTRAINT fk_character_chaos_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_character_chaos_cp CHECK (cp BETWEEN -75 AND 75)
);
CREATE TRIGGER trg_character_chaos_set_updated_at
    BEFORE UPDATE ON game.character_chaos
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();
```

## 5. DDL do `down`

Ordem inversa, índices qualificados pelo mesmo motivo de `0004` (o runner fecha o `search_path`).

```sql
-- migrate:down
DROP TABLE game.character_chaos;
DROP TRIGGER trg_character_affect_set_updated_at ON game.character_affect;
DROP INDEX game.ix_character_affect_tipo;
DROP TABLE game.character_affect;
DROP TRIGGER trg_character_lineage_set_updated_at ON game.character_lineage;
DROP INDEX game.ix_character_lineage_codigo;
DROP TABLE game.character_lineage;
DROP TRIGGER trg_character_learned_skill_set_updated_at ON game.character_learned_skill;
DROP TABLE game.character_learned_skill;
DROP TRIGGER trg_character_skill_set_updated_at ON game.character_skill;
DROP INDEX game.ix_character_skill_indice;
DROP TABLE game.character_skill;
```

## 6. Tabelas de colunas

### 6.1 `game.character_skill`

| Coluna | Tipo | Nulo | Default | O que é | Origem |
|---|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | Dono da skill; FK `CASCADE`. | `0006` |
| `skill_index` | `SMALLINT` | não | — | Índice da skill no cliente, 0–247. Sem FK: catálogo em `0015`. | `H` (legado `character_skills`) |
| `mastery` | `SMALLINT` | não | `0` | Pontos de maestria, 0–320; o teto por classe é regra de jogo. | `H` |
| `learned_at` | `TIMESTAMPTZ` | não | `now()` | Quando a skill foi comprada. | lote `0008` |
| `updated_at` | `TIMESTAMPTZ` | não | `now()` | Última alteração, mantida pelo trigger. | `A9` |

### 6.2 `game.character_learned_skill`

| Coluna | Tipo | Nulo | Default | O que é | Origem |
|---|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | Dono da máscara. | `0006` |
| `category` | `SMALLINT` | não | — | Categoria da máscara, 0–3 (os quatro bytes de `LearnedSkill`). | `H` (`STRUCT_MOB`) |
| `mask` | `SMALLINT` | não | `0` | Bits aprendidos na categoria, 0–255. | `H` |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Criação e última alteração; trigger em `updated_at`. | `A9` |

### 6.3 `game.character_lineage`

| Coluna | Tipo | Nulo | Default | O que é | Origem |
|---|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | Dono da maestria. | `0006` |
| `lineage_code` | `TEXT` | não | — | Código `snake_case` da linhagem; sem FK (catálogo em `0013`). | `H` |
| `mastery` | `SMALLINT` | não | `0` | Maestria na linhagem, 0–320. | `H` |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Marcas de tempo com trigger. | `A9` |

### 6.4 `game.character_affect`

| Coluna | Tipo | Nulo | Default | O que é | Origem |
|---|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | Dono do afeto. | `0006` |
| `affect_type` | `SMALLINT` | não | — | Tipo numérico do afeto; sem FK (semântica `H`). | `H` |
| `value` | `SMALLINT` | não | — | Intensidade do efeito, no sinal do legado. | `H` |
| `level` | `INTEGER` | não | `0` | Nível/grau do efeito. | `H` |
| `remaining_ticks` | `BIGINT` | não | `0` | Ticks restantes; o tick de 8 s é regra de código, não coluna. | `H` |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Marcas de tempo com trigger. | `A9` |

### 6.5 `game.character_chaos`

| Coluna | Tipo | Nulo | Default | O que é | Origem |
|---|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | Dono do CP. | `0006` |
| `cp` | `SMALLINT` | não | `0` | Chaos points reais, −75 a 75; o cliente lê `cp + 75`. | `H` |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Marcas de tempo com trigger. | `A9` |

## 7. Contratos e invariantes

| # | Objeto | O que proíbe | SQLSTATE |
|---:|---|---|---|
| 1 | PKs das cinco tabelas | duas linhas com a mesma chave (personagem + skill/categoria/linhagem/tipo) | `23505` |
| 2 | `fk_*_character` | skill, máscara, linhagem, afeto ou CP de personagem inexistente | `23503` |
| 3 | `ck_character_skill_indice` | índice fora de 0–247 | `23514` |
| 4 | `ck_character_skill_maestria` / `ck_character_lineage_maestria` | maestria fora de 0–320 | `23514` |
| 5 | `ck_character_learned_skill_categoria` / `_mascara` | categoria fora de 0–3 ou máscara fora de 0–255 | `23514` |
| 6 | `ck_character_lineage_codigo` | código vazio, com maiúscula ou fora de `snake_case` | `23514` |
| 7 | `ck_character_affect_tipo` / `_nivel` / `_restante` | tipo negativo, nível negativo ou ticks negativos | `23514` |
| 8 | `ck_character_chaos_cp` | CP fora de −75..75 | `23514` |
| 9 | `ON DELETE CASCADE` | personagem apagado deixando estado órfão | — (cascata) |
| 10 | permissão | `wyd_portal` lendo estado de jogo | `42501` |

**O que o banco não expressa.** O limite de 32 afetos ativos, o teto de maestria por classe
(200/255/320) e a coerência entre `character.learned_skill_mask` (`0006`) e as quatro linhas de
`character_learned_skill` são regra de código; o banco garante faixa, unicidade e dono. Nenhuma
tabela usa `deleted_at`: estado de skill/afeto é linha viva, e o soft-delete é só do personagem
(`A14`).

## 8. Permissões

Nenhum `GRANT`/`REVOKE` neste arquivo. Valem os default privileges do `0001`:

| Papel | Efeito |
|---|---|
| `wyd_game` | `SELECT, INSERT, UPDATE, DELETE` nas cinco tabelas |
| `wyd_read` | `SELECT` |
| `wyd_portal` | **nada**: sem `USAGE` em `game` (`P-17`), leitura só pela view de `0029` |

## 9. Prova de execução

PG17 efêmero, com `0001`, `0005`, `0006` aplicados e `content.content_version` com a linha `1`.
Supõe uma conta (`game.account` id 1) e um personagem (`game.character` id 1) de prova já criados
com o DDL de `0005`/`0006`; a prova 12 usa `DELETE` de propósito, para medir a cascata, não como
caminho de operação.

```bash
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0008-game-skills.sql
```

| # | Prova | Comando | Esperado |
|---:|---|---|---|
| 1 | cinco tabelas | `SELECT count(*) FROM information_schema.tables WHERE table_schema='game' AND table_name IN ('character_skill','character_learned_skill','character_lineage','character_affect','character_chaos');` | `5` |
| 2 | skill órfã | `INSERT INTO game.character_skill (character_id, skill_index) VALUES (999, 10);` | `23503` em `fk_character_skill_character` |
| 3 | índice fora da faixa | `… VALUES (1, 248);` | `23514` em `ck_character_skill_indice` |
| 4 | maestria acima do teto | `… VALUES (1, 10, 321);` | `23514` em `ck_character_skill_maestria` |
| 5 | skill válida | `… VALUES (1, 10, 0);` | `INSERT 0 1` |
| 6 | categoria de máscara inválida | `INSERT INTO game.character_learned_skill (character_id, category) VALUES (1, 4);` | `23514` em `ck_character_learned_skill_categoria` |
| 7 | código de linhagem com maiúscula | `INSERT INTO game.character_lineage (character_id, lineage_code) VALUES (1, 'Kersef');` | `23514` em `ck_character_lineage_codigo` |
| 8 | CP fora da faixa | `INSERT INTO game.character_chaos (character_id, cp) VALUES (1, 76);` | `23514` em `ck_character_chaos_cp` |
| 9 | segundo CP do mesmo personagem | `… VALUES (1, 10);` | `23505` em `pk_character_chaos` |
| 10 | afeto com ticks negativos | `INSERT INTO game.character_affect (character_id, affect_type, value, remaining_ticks) VALUES (1, 7, 1, -1);` | `23514` em `ck_character_affect_restante` |
| 11 | portal não lê estado | `SET ROLE wyd_portal; SELECT count(*) FROM game.character_skill;` | `42501` |
| 12 | cascata | `DELETE FROM game.character WHERE id = 1;` e contar as cinco | `0` em todas |
| 13 | `down` derruba só o que criou | aplicar o bloco `-- migrate:down` e repetir 1 | `0` |
| 14 | ciclo fecha | refazer o `up` | prova 1 de novo (`5`) |

## 10. O que este arquivo não decide

As quatro escolhas de forma foram decididas com o lote em 2026-11-20:

| # | Escolha | O que está proposto |
|---|---|---|
| 1 | Onde mora a máscara de skills aprendidas | `character_learned_skill` (4 linhas por categoria) como fonte; `character.learned_skill_mask` do `0006` fica como cache do fio, escrito na mesma transação |
| 2 | Níveis/graus de skill | só `mastery`; o grau exibido é derivado de faixas e não vira coluna |
| 3 | Linhagem/herança de skill | tabela 1:N por linhagem (`lineage_code TEXT`), sem FK ao catálogo de `0013` |
| 4 | Afetos: forma e tempo | tipo numérico, `value`, `level` e `remaining_ticks`; o tick de 8 s e o limite de 32 são código |

| Assunto | Onde é decidido |
|---|---|
| Catálogo de skills e de linhagens | `0013`/`0015` (arquivo posterior; aqui só `skill_index`/`lineage_code`) |
| Teto de maestria por classe | `content.game_class` (`0013`) e código |
| Limite de 32 afetos | regra de código, sem coluna |
| Máscara de `0006` | `ddl-confirmado.md` §5.10, sujeita à escolha 1 |

*Página de planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado.*
