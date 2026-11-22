> Pergunta: "como aplico o `0009_game_inventory` e como inventário, baú e piedade de refino passam a existir?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3; `ddl-confirmado.md` §5.12 (confirmado); `padrao-banco.md` §1/§5/§9; `lacunas.md` §2.3
> Revisado em: 2026-11-20

Nono arquivo da série. Cria o inventário do personagem, o baú da conta e o contador de piedade de
refino. O catálogo de item não nasce aqui: `item_index` é número do cliente, e nome, preço e efeito
moram em `content.item_catalog` (`0014`) e nas tabelas de economia (`0011`).

!!! note "Aprovado em 2026-11-20"
    O dono aprovou o lote na rodada de perguntas de 2026-11-20, na opção "Sim, aprovar como está",
    depois de escolher as cinco formas de §10. O DDL abaixo é o confirmado em `ddl-confirmado.md`
    §5.12, as três tabelas entram nos 30 de 92 e este arquivo está `concluída` no checklist.

## 1. Identificação

| Campo | Valor |
|---|---|
| Número | `0009` |
| Arquivo | `crates/wyd-db/migrations/0009_game_inventory.sql` |
| Schema | `game` |
| Posição | 9 de 34 (inventário e baú, primeiro arquivo depois das skills) |
| Depende de | `0001` (schemas, `ops.fn_set_updated_at`, default privileges), `0005` (`game.account`, dono do baú) e `0006` (`game.character`, dono do inventário) |
| É dependência de | `0010`/`0011` não referenciam estas tabelas; o catálogo de `0014` é referenciado por valor, não por FK |
| Tabelas criadas | 3 (`character_item`, `vault_item`, `refine_pity`) |
| Objetos criados | 3 tabelas, 2 índices, 3 triggers |
| `SCHEMA_VERSION` | **não muda**: continua `53` até a troca de série (`plano-migrations.md` §6) |

## 2. Objetivo

Guardar, por personagem, o que está em cada slot (item, quantidade, refino, Ancient e gema); por
conta, o conteúdo do baú; e, por slot de personagem, quantas tentativas de refino sem sucesso já
aconteceram — sem duplicar conteúdo de item e sem aceitar piedade órfã.

## 3. Pré-requisitos

1. `0001_bootstrap` aplicado: `game`, `ops.fn_set_updated_at()` e default privileges existem.
2. `0005_game_account` aplicado: `game.account` existe (FK do baú).
3. `0006_game_character` aplicado: `game.character` existe (FK do inventário e da piedade).
4. Uma conta e um personagem de prova já criados. Sem eles, as provas 2 e 7 falham com `23503` no
   lugar errado.
5. Conexão como `wyd_migrator`.

## 4. DDL do `up`

Texto de `ddl-confirmado.md` §5.12, copiado sem alteração.

```sql
CREATE TABLE game.character_item (
    character_id BIGINT NOT NULL,
    slot         SMALLINT NOT NULL,
    item_index   INTEGER NOT NULL,
    qty          INTEGER NOT NULL DEFAULT 1,
    flags        INTEGER NOT NULL DEFAULT 0,
    refine       SMALLINT NOT NULL DEFAULT 0,
    ancient      BOOLEAN NOT NULL DEFAULT false,
    gem_effect   SMALLINT NOT NULL DEFAULT 0,
    gem_value    SMALLINT NOT NULL DEFAULT 0,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_character_item PRIMARY KEY (character_id, slot),
    CONSTRAINT fk_character_item_character FOREIGN KEY (character_id)
        REFERENCES game.character (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_character_item_slot CHECK (slot BETWEEN 0 AND 79),
    CONSTRAINT ck_character_item_qtd CHECK (qty >= 1),
    CONSTRAINT ck_character_item_refino CHECK (refine BETWEEN 0 AND 15),
    CONSTRAINT ck_character_item_gema CHECK (gem_effect BETWEEN 0 AND 4 AND gem_value >= 0),
    -- Efeito e valor andam juntos: um sem o outro é linha inconsistente.
    CONSTRAINT ck_character_item_gema_par CHECK ((gem_effect = 0) = (gem_value = 0)),
    -- Gema em item comum exige +10; Ancient é a exceção.
    CONSTRAINT ck_character_item_gema_refino CHECK (gem_effect = 0 OR ancient OR refine >= 10)
);

-- Sem FK para `content.item_catalog` (0014): catálogo é de arquivo posterior.
CREATE INDEX ix_character_item_indice ON game.character_item (item_index);
CREATE TRIGGER trg_character_item_set_updated_at
    BEFORE UPDATE ON game.character_item
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.vault_item (
    account_id BIGINT NOT NULL,
    slot       SMALLINT NOT NULL,
    item_index INTEGER NOT NULL,
    qty        INTEGER NOT NULL DEFAULT 1,
    flags      INTEGER NOT NULL DEFAULT 0,
    refine     SMALLINT NOT NULL DEFAULT 0,
    ancient    BOOLEAN NOT NULL DEFAULT false,
    gem_effect SMALLINT NOT NULL DEFAULT 0,
    gem_value  SMALLINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_vault_item PRIMARY KEY (account_id, slot),
    CONSTRAINT fk_vault_item_account FOREIGN KEY (account_id)
        REFERENCES game.account (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_vault_item_slot CHECK (slot BETWEEN 0 AND 127),
    CONSTRAINT ck_vault_item_qtd CHECK (qty >= 1),
    CONSTRAINT ck_vault_item_refino CHECK (refine BETWEEN 0 AND 15),
    CONSTRAINT ck_vault_item_gema CHECK (gem_effect BETWEEN 0 AND 4 AND gem_value >= 0),
    CONSTRAINT ck_vault_item_gema_par CHECK ((gem_effect = 0) = (gem_value = 0)),
    CONSTRAINT ck_vault_item_gema_refino CHECK (gem_effect = 0 OR ancient OR refine >= 10)
);

CREATE INDEX ix_vault_item_indice ON game.vault_item (item_index);
CREATE TRIGGER trg_vault_item_set_updated_at
    BEFORE UPDATE ON game.vault_item
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();

CREATE TABLE game.refine_pity (
    character_id BIGINT NOT NULL,
    slot         SMALLINT NOT NULL,
    pity         SMALLINT NOT NULL DEFAULT 0,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT pk_refine_pity PRIMARY KEY (character_id, slot),
    -- Piedade sem item no slot é estado impossível; sai junto com o item.
    CONSTRAINT fk_refine_pity_item FOREIGN KEY (character_id, slot)
        REFERENCES game.character_item (character_id, slot)
        ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT ck_refine_pity_valor CHECK (pity BETWEEN 0 AND 20)
);
CREATE TRIGGER trg_refine_pity_set_updated_at
    BEFORE UPDATE ON game.refine_pity
    FOR EACH ROW EXECUTE FUNCTION ops.fn_set_updated_at();
```

## 5. DDL do `down`

Ordem inversa: `refine_pity` depende de `character_item`, então cai primeiro.

```sql
-- migrate:down
DROP TRIGGER trg_refine_pity_set_updated_at ON game.refine_pity;
DROP TABLE game.refine_pity;
DROP TRIGGER trg_vault_item_set_updated_at ON game.vault_item;
DROP INDEX game.ix_vault_item_indice;
DROP TABLE game.vault_item;
DROP TRIGGER trg_character_item_set_updated_at ON game.character_item;
DROP INDEX game.ix_character_item_indice;
DROP TABLE game.character_item;
```

## 6. Tabelas de colunas

### 6.1 `game.character_item`

| Coluna | Tipo | Nulo | Default | O que é / origem |
|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | Dono; FK `CASCADE` para `game.character` (`0006`). |
| `slot` | `SMALLINT` | não | — | Slot 0–79: 0–15 equip, 16–79 carry (`H`, `Equip`/`Carry` do `STRUCT_MOB`). |
| `item_index` | `INTEGER` | não | — | Índice do item no cliente (`STRUCT_ITEM.sIndex`, `H`); sem FK, catálogo em `0014`. |
| `qty` | `INTEGER` | não | `1` | Quantidade empilhada, no mínimo 1. |
| `flags` | `INTEGER` | não | `0` | Flags de instância do legado (`character_items.flags`, `H`); semântica a reverificar. |
| `refine` | `SMALLINT` | não | `0` | Nível de refino 0–15 (`H`, legado `refine`). |
| `ancient` | `BOOLEAN` | não | `false` | Item Ancient, exceção da regra de gema (`H`). |
| `gem_effect` | `SMALLINT` | não | `0` | Efeito da gema, 0–4 (`H`, `EF_SANC`). |
| `gem_value` | `SMALLINT` | não | `0` | Valor da gema, não negativo; anda junto com o efeito. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Marcas de tempo; trigger em `updated_at` (`A9`). |

### 6.2 `game.vault_item`

| Coluna | Tipo | Nulo | Default | O que é / origem |
|---|---|---|---|---|
| `account_id` | `BIGINT` | não | — | Dono; FK `CASCADE` para `game.account` (`0005`). |
| `slot` | `SMALLINT` | não | — | Slot do baú 0–127 (`H`, legado `vault_items`). |
| `item_index` | `INTEGER` | não | — | Índice do item no cliente; sem FK. |
| `qty` / `flags` / `refine` / `ancient` / `gem_effect` / `gem_value` | mesmos tipos de `character_item` | não | 1 / 0 / 0 / false / 0 / 0 | Mesmas regras de instância do item. |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Marcas de tempo com trigger. |

### 6.3 `game.refine_pity`

| Coluna | Tipo | Nulo | Default | O que é / origem |
|---|---|---|---|---|
| `character_id` | `BIGINT` | não | — | Dono do slot; parte da FK composta. |
| `slot` | `SMALLINT` | não | — | Slot do inventário do personagem; FK composta fecha com `character_item`. |
| `pity` | `SMALLINT` | não | `0` | Tentativas sem sucesso, 0–20 (`H`, legado `0043`). |
| `created_at` / `updated_at` | `TIMESTAMPTZ` | não | `now()` | Marcas de tempo com trigger. |

## 7. Contratos e invariantes

| # | Objeto | Estado que impede | SQLSTATE |
|---:|---|---|---|
| 1 | PKs das três tabelas | duas linhas no mesmo slot (personagem ou conta) | `23505` |
| 2 | `fk_character_item_character` / `fk_vault_item_account` | item de personagem ou baú de conta inexistente | `23503` |
| 3 | `fk_refine_pity_item` | piedade em slot sem item — inclusive depois de o item sair do slot | `23503` |
| 4 | `ck_character_item_slot` | slot fora de 0–79 | `23514` |
| 5 | `ck_vault_item_slot` | slot de baú fora de 0–127 | `23514` |
| 6 | `ck_*_qtd` | quantidade zero ou negativa | `23514` |
| 7 | `ck_*_refino` | refino fora de 0–15 | `23514` |
| 8 | `ck_*_gema` | efeito fora de 0–4 ou valor negativo | `23514` |
| 9 | `ck_*_gema_par` | efeito sem valor (ou valor sem efeito) | `23514` |
| 10 | `ck_*_gema_refino` | gema em item comum abaixo de +10 sem ser Ancient | `23514` |
| 11 | `ck_refine_pity_valor` | piedade fora de 0–20 | `23514` |
| 12 | `ON DELETE CASCADE` | personagem/conta apagados deixando item órfão | — (cascata) |
| 13 | permissão | `wyd_portal` lendo inventário cru | `42501` |

**O que o banco não expressa.** Que o `item_index` exista no catálogo (`0014`) é verificado na carga
e no código, não por FK. Que slot 0–15 seja equip e 16–79 carry é regra de jogo, não coluna. O teto de
refino por item e o efeito de cada gema vêm de conteúdo. E `refine_pity` não guarda histórico: o
contador é estado do slot, não trilha de tentativas.

## 8. Permissões

Nenhum `GRANT` explícito. Default privileges do `0001`:

| Papel | Efeito | Origem |
|---|---|---|
| `wyd_game` | `SELECT, INSERT, UPDATE, DELETE` nas três tabelas | default privilege em `game` |
| `wyd_read` | `SELECT` nas três tabelas | default privilege em `game` |
| `wyd_portal` | **nada**: sem `USAGE` em `game` | fronteira `P-17`; admin vê pela view de `0029` |
| `PUBLIC` | nada | `0001`/provisionamento |

## 9. Prova de execução

PG17 efêmero, com `0001`, `0005` e `0006` aplicados e uma conta/personagem de prova (`game.account`
id 1, `game.character` id 1). A prova 13 usa `DELETE` de propósito, para medir a cascata.

```bash
psql "$DSN" -v ON_ERROR_STOP=1 -f crates/wyd-db/migrations/0009_game_inventory.sql
```

| # | Prova | Comando | Esperado |
|---:|---|---|---|
| 1 | três tabelas | `SELECT count(*) FROM information_schema.tables WHERE table_schema='game' AND table_name IN ('character_item','vault_item','refine_pity');` | `3` |
| 2 | item de personagem inexistente | `INSERT INTO game.character_item (character_id, slot, item_index) VALUES (999, 0, 10);` | `23503` em `fk_character_item_character` |
| 3 | slot 80 no personagem | `… VALUES (1, 80, 10);` | `23514` em `ck_character_item_slot` |
| 4 | quantidade zero | `… VALUES (1, 0, 10, 0);` | `23514` em `ck_character_item_qtd` |
| 5 | gema sem valor | `… VALUES (1, 0, 10, 1, 0, 12, true, 3, 0);` | `23514` em `ck_character_item_gema_par` |
| 6 | gema em comum abaixo de +10 | `… VALUES (1, 0, 10, 1, 0, 5, false, 1, 7);` | `23514` em `ck_character_item_gema_refino` |
| 7 | item válido | `… VALUES (1, 0, 10);` | `INSERT 0 1` |
| 8 | baú em slot 128 | `INSERT INTO game.vault_item (account_id, slot, item_index) VALUES (1, 128, 10);` | `23514` em `ck_vault_item_slot` |
| 9 | baú de conta inexistente | `… VALUES (999, 0, 10);` | `23503` em `fk_vault_item_account` |
| 10 | piedade sem item no slot | `INSERT INTO game.refine_pity (character_id, slot) VALUES (1, 1);` | `23503` em `fk_refine_pity_item` |
| 11 | piedade válida e fora da faixa | `… VALUES (1, 0);` e depois `UPDATE game.refine_pity SET pity = 21 WHERE character_id = 1;` | `INSERT 0 1` e `23514` em `ck_refine_pity_valor` |
| 12 | segunda linha no mesmo slot | dois `INSERT` em `(1, 0)` | `23505` em `pk_character_item` |
| 13 | cascata | `DELETE FROM game.character WHERE id = 1;` e contar `character_item`/`refine_pity` | `0` nas duas |
| 14 | portal não lê | `SET ROLE wyd_portal; SELECT count(*) FROM game.character_item;` | `42501` |
| 15 | `down` derruba só o lote | aplicar o bloco `-- migrate:down` e repetir 1 | `0` |

## 10. O que este arquivo não decide

As cinco escolhas de forma foram decididas com o lote em 2026-11-20:

| # | Escolha | O que o lote fixa |
|---|---|---|
| 1 | Baú da conta ou do personagem | da **conta** (`vault_item.account_id`), como no legado |
| 2 | Personagem e baú na mesma tabela | **não**: duas tabelas com as mesmas colunas de item |
| 3 | Quantidade e durabilidade | `qty` por linha (empilhamento); durabilidade não vira coluna |
| 4 | Limite de slots | personagem 0–79; baú 0–127 |
| 5 | O que `refine_pity` guarda | contador 0–20 por `(personagem, slot)`, apagado com o item |

| Assunto | Onde é decidido |
|---|---|
| Catálogo, nome, preço e efeito de item | `content.item_catalog` (`0014`) e `0011` (loja/economia) |
| Regra de refino, teto por item e efeito de gema | `wyd-sim`/conteúdo, ainda a planejar |
| Slot de equip × carry e limite de 80 | `STRUCT_MOB` (`H`) e regra de jogo no core |
| Piedade como histórico | fora deste lote; se o dono quiser trilha, é tabela de evento em lote próprio |

*Página de planejamento. Nenhum SQL foi executado e nenhum arquivo de `crates/` foi criado.*
