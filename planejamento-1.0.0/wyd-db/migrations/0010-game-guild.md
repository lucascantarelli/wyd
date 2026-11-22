> Pergunta: "como aplico o `0010_game_guild` e como guilda, membros, relações e GTorre passam a existir?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3; `ddl-confirmado.md` §5.13 (confirmado); `padrao-banco.md` §1/§5/§9; `lacunas.md` §2.3
> Revisado em: 2026-11-22

Décimo arquivo da série. Cria a guilda, o vínculo de personagem com papel, as relações entre guildas
e a linha única da GTorre. Zona de guilda, preço e spawn de guilda são conteúdo (`0019`), e não
entram aqui.

!!! note "Aprovado em 2026-11-20"
    O dono aprovou o lote na rodada de perguntas de 2026-11-20, na opção "Sim, aprovar como está",
    depois de escolher as quatro formas de §10. O DDL abaixo é o confirmado em `ddl-confirmado.md`
    §5.13. As quatro tabelas contam entre as que têm DDL fechado; a contagem vigente é
    `ddl-confirmado.md` §1, e este arquivo está `concluída` no checklist.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0010` |
| Arquivo | `crates/wyd-db/migrations/0010_game_guild.sql` |
| Schema | `game` |
| Posição | 10 de 34 (primeiro arquivo de organização social) |
| Depende de | `0001` (schemas, `ops.fn_set_updated_at`, default privileges) e `0006` (`game.character`, dono do vínculo) |
| É dependência de | nenhum arquivo da série; `0019` cria `guild_zone` como conteúdo, sem FK para cá |
| Tabelas criadas | 4 (`guild`, `guild_member`, `guild_relation`, `guild_tower`) |
| Objetos criados | 4 tabelas, 5 índices (2 deles parciais), 4 triggers |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca de série (`plano-migrations.md` §6) |

## 2. Objetivo

Persistir guilda e nome único, o vínculo de cada personagem com papel (um líder por guilda), a
relação declarada de aliança ou hostilidade entre guildas e a linha única que diz qual guilda domina
a GTorre.

## 3. Pré-requisitos

1. `0001_bootstrap` aplicado: `game`, `ops.fn_set_updated_at()` e default privileges existem.
2. `0006_game_character` aplicado: `game.character` existe (FK do vínculo).
3. Personagens de prova já criados; sem eles, as provas de FK falham com `23503` no lugar errado.
4. Conexão como `wyd_migrator`.

## 4. DDL do `up`

Texto de `ddl-confirmado.md` §5.13, copiado sem alteração.

```sql
-- migrate:up
CREATE TABLE game.guild (
    id         BIGINT GENERATED ALWAYS AS IDENTITY,
    name       TEXT NOT NULL,
    name_fold  TEXT GENERATED ALWAYS AS (lower(btrim(name))) STORED,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_guild PRIMARY KEY (id),
    CONSTRAINT ck_guild_nome CHECK (char_length(btrim(name)) BETWEEN 3 AND 16)
);

-- Nome único entre guildas existentes: `name_fold` compara sem caixa e sem espaço de borda.
CREATE UNIQUE INDEX ux_guild_nome ON game.guild (name_fold);
CREATE TRIGGER trg_guild_set_updated_at
    BEFORE UPDATE ON game.guild
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.guild_member (
    guild_id     BIGINT NOT NULL,
    character_id BIGINT NOT NULL,
    role         TEXT NOT NULL DEFAULT 'member',
    joined_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_guild_member PRIMARY KEY (guild_id, character_id),
    CONSTRAINT fk_guild_member_guild FOREIGN KEY (guild_id)
        REFERENCES game.guild (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT fk_guild_member_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_guild_member_papel CHECK (role IN ('leader', 'sub_leader', 'member'))
);

-- Um personagem pertence a uma guilda só (regra do legado, garantida no banco).
CREATE UNIQUE INDEX ux_guild_member_personagem ON game.guild_member (character_id);
-- Um líder por guilda, inclusive sob concorrência.
CREATE UNIQUE INDEX ux_guild_member_lider ON game.guild_member (guild_id) WHERE role = 'leader';
CREATE TRIGGER trg_guild_member_set_updated_at
    BEFORE UPDATE ON game.guild_member
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.guild_relation (
    guild_id        BIGINT NOT NULL,
    target_guild_id BIGINT NOT NULL,
    kind            TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_guild_relation PRIMARY KEY (guild_id, target_guild_id),
    CONSTRAINT fk_guild_relation_guild FOREIGN KEY (guild_id)
        REFERENCES game.guild (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT fk_guild_relation_target FOREIGN KEY (target_guild_id)
        REFERENCES game.guild (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_guild_relation_kind CHECK (kind IN ('ally', 'hostile')),
    CONSTRAINT ck_guild_relation_distinta CHECK (guild_id <> target_guild_id)
);

CREATE INDEX ix_guild_relation_target ON game.guild_relation (target_guild_id);
CREATE TRIGGER trg_guild_relation_set_updated_at
    BEFORE UPDATE ON game.guild_relation
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.guild_tower (
    id             SMALLINT NOT NULL DEFAULT 1,
    owner_guild_id BIGINT,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_guild_tower PRIMARY KEY (id),
    CONSTRAINT ck_guild_tower_singleton CHECK (id = 1),
    -- GTorre sem dono é `NULL`, nunca um id sentinela: guilda dissolvida zera o dono.
    CONSTRAINT fk_guild_tower_owner FOREIGN KEY (owner_guild_id)
        REFERENCES game.guild (id) ON DELETE SET NULL ON UPDATE NO ACTION
);
CREATE TRIGGER trg_guild_tower_set_updated_at
    BEFORE UPDATE ON game.guild_tower
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();
```

## 5. DDL do `down`

Ordem inversa: GTorre e relações antes da guilda, vínculo antes de ambos.

```sql
-- migrate:down
DROP TRIGGER trg_guild_tower_set_updated_at ON game.guild_tower;
DROP TABLE game.guild_tower;
DROP TRIGGER trg_guild_relation_set_updated_at ON game.guild_relation;
DROP INDEX game.ix_guild_relation_target;
DROP TABLE game.guild_relation;
DROP TRIGGER trg_guild_member_set_updated_at ON game.guild_member;
DROP INDEX game.ux_guild_member_lider;
DROP INDEX game.ux_guild_member_personagem;
DROP TABLE game.guild_member;
DROP TRIGGER trg_guild_set_updated_at ON game.guild;
DROP INDEX game.ux_guild_nome;
DROP TABLE game.guild;
```

## 6. Tabelas de colunas

### 6.1 `game.guild`

| Coluna | Tipo | Nulo | Default | O que é / origem |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | identity | Identificador da guilda. |
| `name` | `TEXT` | não | — | Nome exibido, 3–16 caracteres (`H`, `guilds.name` e nome de clã do cliente). |
| `name_fold` | `TEXT` gerada STORED | não | `lower(btrim(name))` | Chave de comparação sem caixa e sem borda. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Criação e atualização com trigger (`A9`). |

### 6.2 `game.guild_member`

| Coluna | Tipo | Nulo | Default | O que é / origem |
|---|---|---|---|---|
| `guild_id` | `BIGINT` | não | — | Guilda do vínculo; FK `CASCADE`. |
| `character_id` | `BIGINT` | não | — | Personagem membro; FK `CASCADE` e único na tabela. |
| `role` | `TEXT` | não | `'member'` | `leader`, `sub_leader` ou `member` (`H`, `STRUCT_MOB.GuildLevel`). |
| `joined_at` | `TIMESTAMPTZ` | não | `now()` | Entrada na guilda (`H`, `guild_members.joined_at`). |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Marcas de tempo com trigger. |

### 6.3 `game.guild_relation`

| Coluna | Tipo | Nulo | Default | O que é / origem |
|---|---|---|---|---|
| `guild_id` | `BIGINT` | não | — | Guilda que declara a relação; FK `CASCADE`. |
| `target_guild_id` | `BIGINT` | não | — | Guilda alvo; FK `CASCADE`, indexada. |
| `kind` | `TEXT` | não | — | `ally` ou `hostile` (`H`, legado `kind` 1/2). |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Marcas de tempo com trigger. |

### 6.4 `game.guild_tower`

| Coluna | Tipo | Nulo | Default | O que é / origem |
|---|---|---|---|---|
| `id` | `SMALLINT` | não | `1` | Linha única; `CHECK (id = 1)`. |
| `owner_guild_id` | `BIGINT` | sim | — | Guilda dona da GTorre; `NULL` = sem dono; FK `SET NULL`. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Marcas de tempo com trigger. |

## 7. Contratos e invariantes

| # | Objeto | Estado que impede | SQLSTATE |
|---:|---|---|---|
| 1 | `pk_guild` | duas guildas com o mesmo `id` | `23505` |
| 2 | `ux_guild_nome` | nome repetido sem caixa (`Kersef` × `kersef`) | `23505` |
| 3 | `ck_guild_nome` | nome vazio, com menos de 3 ou mais de 16 caracteres | `23514` |
| 4 | `pk_guild_member` | dois vínculos do mesmo personagem na mesma guilda | `23505` |
| 5 | `ux_guild_member_personagem` | o mesmo personagem em duas guildas | `23505` |
| 6 | `ux_guild_member_lider` | segundo `leader` na mesma guilda | `23505` |
| 7 | `ck_guild_member_papel` | papel fora dos três valores | `23514` |
| 8 | `fk_guild_member_guild` / `_character` | vínculo com guilda ou personagem inexistente | `23503` |
| 9 | `pk_guild_relation` | duas relações para o mesmo par | `23505` |
| 10 | `ck_guild_relation_distinta` | guilda aliada/hostil de si mesma | `23514` |
| 11 | `ck_guild_relation_kind` | tipo fora de `ally`/`hostile` | `23514` |
| 12 | `ck_guild_tower_singleton` | segunda linha na GTorre | `23514` |
| 13 | `fk_guild_tower_owner` | dono inexistente / guilda apagada deixando dono pendurado | `23503` / `SET NULL` |
| 14 | `ON DELETE CASCADE` | guilda apagada deixando membro ou relação órfãos | — (cascata) |
| 15 | permissão | `wyd_portal` lendo guilda crua | `42501` |

**O que o banco não expressa.** Que o `leader` de uma guilda exista enquanto a guilda existir é regra
de aplicação (o banco garante no máximo um, não no mínimo um). Que `sub_leader` seja limitado a três
(`STRUCT_GUILDINFO.SubLiderName[3]`, `H`) é regra de código, não índice. E qual guilda domina a
GTorre é estado da linha única, não histórico: troca de dono não deixa trilha nesta tabela.

## 8. Permissões

Nenhum `GRANT` explícito. Default privileges do `0001`:

| Papel | Efeito | Origem |
|---|---|---|
| `wyd_game` | `SELECT, INSERT, UPDATE, DELETE` nas quatro tabelas | default privilege em `game` |
| `wyd_read` | `SELECT` nas quatro tabelas | default privilege em `game` |
| `wyd_portal` | **nada**: sem `USAGE` em `game` | fronteira `P-17`; admin vê pela view de `0029` |
| `PUBLIC` | nada | `0001`/provisionamento |

## 9. Prova de execução

PG17 efêmero, com `0001` e `0006` aplicados e personagens de prova (`game.character` ids 1 e 2). A
prova 13 usa `DELETE` de propósito, para medir a cascata.

```bash
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0010_game_guild.sql
```

| # | Prova | Comando | Esperado |
|---:|---|---|---|
| 1 | quatro tabelas | `SELECT count(*) FROM information_schema.tables WHERE table_schema='game' AND table_name IN ('guild','guild_member','guild_relation','guild_tower');` | `4` |
| 2 | nome curto | `INSERT INTO game.guild (name) VALUES ('ab');` | `23514` em `ck_guild_nome` |
| 3 | nome repetido sem caixa | inserir `Kersef` e `kersef` | segundo `23505` em `ux_guild_nome` |
| 4 | vínculo em guilda inexistente | `INSERT INTO game.guild_member (guild_id, character_id) VALUES (99, 1);` | `23503` em `fk_guild_member_guild` |
| 5 | personagem em duas guildas | dois vínculos do personagem 1 em guildas diferentes | segundo `23505` em `ux_guild_member_personagem` |
| 6 | dois líderes | dois vínculos `role='leader'` na mesma guilda | segundo `23505` em `ux_guild_member_lider` |
| 7 | papel inventado | `role='boss'` | `23514` em `ck_guild_member_papel` |
| 8 | guilda aliada de si mesma | `INSERT INTO game.guild_relation (guild_id, target_guild_id, kind) VALUES (1, 1, 'ally');` | `23514` em `ck_guild_relation_distinta` |
| 9 | relação válida | `… VALUES (1, 2, 'hostile');` | `INSERT 0 1` |
| 10 | segunda linha na GTorre | dois `INSERT` em `guild_tower` | segundo `23505` em `pk_guild_tower` (id 1 repetido) |
| 11 | GTorre sem dono | `INSERT INTO game.guild_tower (owner_guild_id) VALUES (NULL);` | `INSERT 0 1` |
| 12 | guilda apagada zera o dono | `DELETE FROM game.guild WHERE id = 1;` e reler `guild_tower.owner_guild_id` | `NULL` |
| 13 | cascata de membros | apagar a guilda 2 e contar `guild_member` dela | `0` |
| 14 | portal não lê | `SET ROLE wyd_portal; SELECT count(*) FROM game.guild;` | `42501` |
| 15 | `down` derruba só o lote | aplicar o bloco `-- migrate:down` e repetir 1 | `0` |

## 10. Decisões aprovadas e limites do lote

As quatro escolhas de forma foram decididas com o lote em 2026-11-20:

| # | Escolha | O que o lote fixa |
|---|---|---|
| 1 | Liderança e cargos | `role` textual em `guild_member`, um líder por guilda por índice parcial, sem `master_character_id` |
| 2 | Unicidade do nome | `name_fold` gerada com índice total; guilda dissolvida é apagada |
| 3 | Saída de membro | ao sair ou ser expulso, apaga-se o vínculo, sem histórico; apagar a guilda também remove seus vínculos por FK `ON DELETE CASCADE` |
| 4 | Dono da GTorre | `owner_guild_id` anulável com `ON DELETE SET NULL`; `NULL` = sem dono |

| Assunto | Onde é decidido |
|---|---|
| Zona de guilda, `ChargeGuild`/`ChallangeGuild` e spawn | `content.guild_zone` (`0019`) |
| Fama e nível de guilda (`STRUCT_GUILDINFO`) | decisão própria; sem semântica medida, não viram coluna aqui |
| Limite de três sub-líderes | regra de código (`H`, `SubLiderName[3]`) |
| Histórico de domínio da GTorre | fora deste lote; exige tabela de evento própria |

*Página de planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado.*