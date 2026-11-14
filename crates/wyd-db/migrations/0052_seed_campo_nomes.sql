-- 0052_seed_campo_nomes: nomes de tela do Campo no lugar dos técnicos do dump.
--
-- O dump traz `Treinador1`, `Chefe_de_Treino`, `Unicornio_Puro` como `display_name`, e é esse
-- campo que o cliente desenha sobre o NPC. O guia oficial usa "Primeiro/Segundo/Terceiro
-- Treinador", mas `MobName` é `char[16]` **sem** byte extra: nome com 16+ bytes sai cortado
-- (`Primeiro Treinad`) e o cliente desenha o nome truncado. Por isso os nomes aqui cabem em 15
-- bytes e são ASCII — o cliente legado não é UTF-8 (recebe byte cru).

-- migrate:up
UPDATE npc_definition SET display_name = 'Prim. Treinador'
 WHERE template_name = 'Treinador1' AND map_id = 0;
UPDATE npc_definition SET display_name = 'Seg. Treinador'
 WHERE template_name = 'Treinador2' AND map_id = 0;
UPDATE npc_definition SET display_name = 'Terc. Treinador'
 WHERE template_name = 'Treinador3' AND map_id = 0;
UPDATE npc_definition SET display_name = 'Chefe de Treino'
 WHERE template_name = 'Chefe_de_Treino' AND map_id = 0;
UPDATE npc_definition SET display_name = 'Unicornio Puro'
 WHERE template_name = 'Unicornio_Puro' AND map_id = 0;
UPDATE npc_definition SET display_name = 'Ajudante'
 WHERE template_name = 'Ajudante' AND map_id = 0;

-- migrate:down
UPDATE npc_definition SET display_name = 'Treinador1'
 WHERE template_name = 'Treinador1' AND map_id = 0;
UPDATE npc_definition SET display_name = 'Treinador2'
 WHERE template_name = 'Treinador2' AND map_id = 0;
UPDATE npc_definition SET display_name = 'Treinador3'
 WHERE template_name = 'Treinador3' AND map_id = 0;
UPDATE npc_definition SET display_name = 'Chefe_de_Treino'
 WHERE template_name = 'Chefe_de_Treino' AND map_id = 0;
UPDATE npc_definition SET display_name = 'Unicornio_Puro'
 WHERE template_name = 'Unicornio_Puro' AND map_id = 0;
UPDATE npc_definition SET display_name = 'Ajudante'
 WHERE template_name = 'Ajudante' AND map_id = 0;
