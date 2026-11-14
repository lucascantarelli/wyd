-- 0051_seed_campo_sem_duplicatas: tira as cópias do Campo de Treinamento.
--
-- O dump herdado já posiciona o Campo inteiro (treinadores, Chefe de Treino, Unicórnio Puro e
-- Ajudante). O seed 0046 acrescentou linhas `campo-*` a 1–2 tiles dessas, então a cena
-- materializava **dois** "Chefe de Treino", dois unicórnios e dois NPCs de buff — e as cópias
-- usavam templates/props diferentes do legado (`Unicorn`/`Cabuncle Wind` em vez de
-- `Unicornio_Puro`/`Ajudante`), o que trocava o NPC correto pelo errado.
--
-- Correção: remove as cópias e leva o `merchant` dos treinadores herdados para a sequência que
-- o servidor usa para as etapas da quest (`gameplay::MERCHANT_TRAINER_*` = 36/40/41/100).
-- As quatro criaturas de prova (`campo-gremlin-decoy`…`campo-orc-warrior`) ficam: são elas que
-- carregam os drops 451/452/453/524 do Campo.

-- migrate:up
DELETE FROM npc_definition
 WHERE id IN (6301, 6302, 6303, 6304, 6305, 6306)
   AND slug LIKE 'campo-%';

UPDATE npc_definition SET merchant = 36 WHERE template_name = 'Treinador1' AND map_id = 0;
UPDATE npc_definition SET merchant = 40 WHERE template_name = 'Treinador2' AND map_id = 0;
UPDATE npc_definition SET merchant = 41 WHERE template_name = 'Treinador3' AND map_id = 0;
-- Chefe de Treino (`Chefe_de_Treino`) já é merchant 100, Unicórnio Puro 110 e Ajudante 120
-- no dump; não se toca neles.

-- migrate:down
UPDATE npc_definition SET merchant = 100 WHERE template_name = 'Treinador1' AND map_id = 0;
UPDATE npc_definition SET merchant = 104 WHERE template_name = 'Treinador2' AND map_id = 0;
UPDATE npc_definition SET merchant = 105 WHERE template_name = 'Treinador3' AND map_id = 0;

INSERT INTO npc_definition
    (id, slug, template_name, display_name, enabled, map_id, pos_x, pos_y, merchant, origin)
VALUES
    (6301, 'campo-treinador-1', 'FirstTrainer',  'Primeiro Treinador', true, 0, 2077, 2017,  36, 'custom'),
    (6302, 'campo-treinador-2', 'SecondTrainer', 'Segundo Treinador',  true, 0, 2144, 1989,  40, 'custom'),
    (6303, 'campo-treinador-3', 'ThirdTrainer',  'Terceiro Treinador', true, 0, 2082, 1966,  41, 'custom'),
    (6304, 'campo-chefe-treino', 'TrainerChief', 'Chefe de Treino',    true, 0, 2122, 2041, 100, 'custom'),
    (6305, 'campo-unicorn',      'Unicorn',      'Unicorn',            true, 0, 2130, 2040,   0, 'custom'),
    (6306, 'campo-cabuncle',     'Cabuncle',     'Cabuncle Wind',      true, 0, 2131, 2033,   0, 'custom')
ON CONFLICT (id) DO NOTHING;
