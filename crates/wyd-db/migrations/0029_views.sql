-- 0029_views: presença para o portal e identidade da pessoa para o jogo.
-- As duas views existem para que nenhum domínio leia a tabela interna do outro (P-17, padrao-banco §7).
-- Não usa COMMENT ON: a série documenta semântica nas páginas de migration.
-- Nenhum GRANT neste arquivo: permissão de leitura é decisão própria (ver página §7 e §10).

-- migrate:up
-- Presença viva por sessão: sem IP, sem e-mail e sem tabela interna do jogo exposta.
-- LEFT JOIN porque a sessão nasce no handshake, antes da seleção de personagem.
CREATE VIEW game.v_live_session AS
SELECT s.id             AS session_id,
       s.account_id     AS account_id,
       s.character_id   AS character_id,
       c.name           AS character_name,
       s.group_id       AS group_id,
       s.channel        AS channel,
       s.client_version AS client_version,
       s.opened_at      AS opened_at,
       s.renewed_at     AS renewed_at
FROM game.session s
LEFT JOIN game.character c ON c.id = s.character_id
WHERE s.released_at IS NULL;

-- Identidade mínima da pessoa para o jogo: id, papel e estado; nunca e-mail, hash ou PIN.
CREATE VIEW portal.v_person_identity AS
SELECT p.id     AS person_id,
       p.role   AS role,
       p.status AS status
FROM portal.person p;
-- migrate:down
-- Reversão de 0029: as views caem antes das tabelas que leem (`0007`, `0003`).
DROP VIEW game.v_live_session;
DROP VIEW portal.v_person_identity;
