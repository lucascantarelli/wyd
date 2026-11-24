-- 0024_content_maps: definição de mapa e rota de teleporte por coordenada.
-- Forma proposta a partir das fichas do legado em deprecated/.agents/prompts/análises/wyd-db/database/tabelas/;
-- toda semântica histórica não explicitada no planejamento novo permanece H, nunca fato por cópia.
-- Não usa COMMENT ON: a série documenta semântica nas páginas de migration.
-- Tabela base seed-only e read-only em runtime: mapa e rota são dado, não estado de personagem.

-- migrate:up
-- Chave natural do cliente: o id do mapa é o que o STRUCT_MOB já carrega. Sem identidade (P-04).
CREATE TABLE content.map_definition (
    map_id     SMALLINT NOT NULL,
    name       TEXT NOT NULL,
    -- Região é retângulo em `content.region` (0019); nula enquanto o mapa não tiver retângulo.
    region_id  BIGINT,
    is_pvp     BOOLEAN NOT NULL DEFAULT false,
    is_safe    BOOLEAN NOT NULL DEFAULT false,
    is_dungeon BOOLEAN NOT NULL DEFAULT false,
    CONSTRAINT pk_map_definition PRIMARY KEY (map_id),
    CONSTRAINT fk_map_definition_region FOREIGN KEY (region_id)
        REFERENCES content.region (id) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_map_definition_id CHECK (map_id >= 0),
    CONSTRAINT ck_map_definition_nome CHECK (char_length(btrim(name)) BETWEEN 1 AND 64)
);

-- P-15: FK nullable indexada. A leitura quente é por retângulo; a busca por mapa é a PK.
CREATE INDEX ix_map_definition_region ON content.map_definition (region_id)
    WHERE region_id IS NOT NULL;

-- Coordenada explícita origem→destino: destino é ponto, nunca "centro da região" (mundo.md §5.1).
CREATE TABLE content.teleport_route (
    id             BIGINT GENERATED ALWAYS AS IDENTITY,
    from_map_id    SMALLINT NOT NULL,
    from_x         SMALLINT NOT NULL,
    from_y         SMALLINT NOT NULL,
    to_map_id      SMALLINT NOT NULL,
    to_x           SMALLINT NOT NULL,
    to_y           SMALLINT NOT NULL,
    -- NPC que oferece a rota; nulo quando a rota é do mapa (zona de teleporte, portal).
    npc_id         INTEGER,
    -- Preço em ouro do PERSONAGEM (`game.character.coin`), nunca a carteira da conta.
    coin           BIGINT NOT NULL DEFAULT 0,
    req_level      SMALLINT NOT NULL DEFAULT 1,
    -- Item exigido (pergaminho, pedido de caça): índice do catálogo, não preço.
    req_item_index INTEGER,
    enabled        BOOLEAN NOT NULL DEFAULT true,
    CONSTRAINT pk_teleport_route PRIMARY KEY (id),
    -- Dois NPCs podem oferecer o mesmo par; o mesmo NPC não repete a rota.
    CONSTRAINT uq_teleport_route_rota UNIQUE NULLS NOT DISTINCT (
        from_map_id, from_x, from_y, to_map_id, to_x, to_y, npc_id),
    CONSTRAINT fk_teleport_route_origem FOREIGN KEY (from_map_id)
        REFERENCES content.map_definition (map_id) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT fk_teleport_route_destino FOREIGN KEY (to_map_id)
        REFERENCES content.map_definition (map_id) ON UPDATE RESTRICT ON DELETE RESTRICT,
    -- Apagar o NPC desliga a rota em vez de derrubar o mapa: o destino continua válido.
    CONSTRAINT fk_teleport_route_npc FOREIGN KEY (npc_id)
        REFERENCES content.npc_definition (id) ON UPDATE RESTRICT ON DELETE SET NULL,
    CONSTRAINT fk_teleport_route_item FOREIGN KEY (req_item_index)
        REFERENCES content.item_catalog (item_index) ON UPDATE RESTRICT ON DELETE RESTRICT,
    CONSTRAINT ck_teleport_route_custo CHECK (coin >= 0),
    CONSTRAINT ck_teleport_route_nivel CHECK (req_level >= 1),
    CONSTRAINT ck_teleport_route_item CHECK (
        req_item_index IS NULL OR req_item_index BETWEEN 0 AND 32767),
    -- SMALLINT não negativo, igual a `npc_definition.pos_x`: o teto 4095 é regra do loader do
    -- cliente, não do banco (`mundo.md` §3 preserva um y2 = 6841 no retângulo da região).
    CONSTRAINT ck_teleport_route_origem_pos CHECK (
        from_x BETWEEN 0 AND 32767 AND from_y BETWEEN 0 AND 32767),
    CONSTRAINT ck_teleport_route_destino_pos CHECK (
        to_x BETWEEN 0 AND 32767 AND to_y BETWEEN 0 AND 32767)
);

-- P-15: as FKs nullable e a de destino. `from_map_id` já é prefixo do único.
CREATE INDEX ix_teleport_route_npc ON content.teleport_route (npc_id)
    WHERE npc_id IS NOT NULL;
CREATE INDEX ix_teleport_route_item ON content.teleport_route (req_item_index)
    WHERE req_item_index IS NOT NULL;
CREATE INDEX ix_teleport_route_destino ON content.teleport_route (to_map_id);
