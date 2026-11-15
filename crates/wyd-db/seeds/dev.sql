-- seeds/dev.sql — dados de DESENVOLVIMENTO. Valores obviamente falsos (DEV-ONLY);
-- nunca commitar segredo real aqui. Produção usa runbook próprio (FASE-05), não este arquivo.
--
-- Contas PADRÃO de teste (2026-11-15), para o banco de testes começar do zero:
--   login `teste01`..`teste04` · senha `teste1234` · PIN `1234`
--   um personagem por classe em Armia (mapa 0), ao lado do Campo de Treinamento;
--   classe 0=TransKnight, 1=Foema, 2=BeastMaster, 3=Huntress (docs/wyd/classes.md).
-- A senha cabe em `password[12]` do `_MSG_AccountLogin` e o nome em `MobName[16]`.
--
-- Nenhuma conta de staff: o banco de teste começa SEM admin (pedido de 2026-11-15).
-- Staff nasce de `wyd-gateway --seed-staff` ou do portal quando o teste exigir;
-- seed versionado com papel `admin` não pode existir.
--
-- Aplicar UMA vez, com o schema recém-criado (`psql -f seeds/dev.sql`). Sem
-- `ON CONFLICT`: reaplicar sobre conta existente tem de falhar, não sobrescrever senha.

INSERT INTO accounts (login, password_hash, pin_hash, role) VALUES
    ('teste01', '$argon2id$v=19$m=19456,t=2,p=1$YSwKRVby/7pInPQ8KwEMeA$M2VKtf+3/CgwLTxOvnuUw2/L9JreZW/iNcIHVOkxWUw', '$argon2id$v=19$m=19456,t=2,p=1$OjHZeuz8wMxVDt0WjEvc/Q$M7mWxM2THo2yW3s0/Q2LAA9yDBPZTE+Y8QIjewnKR1M', 'player'),
    ('teste02', '$argon2id$v=19$m=19456,t=2,p=1$iE0nw7VCaIkFEyxbS5E0TA$Zpp/InOuiAHWeqXgaMm1P+ewVM/QO211+ZrTaL0XjeE', '$argon2id$v=19$m=19456,t=2,p=1$6c1/usYivZ7VTG4lfyMgyw$AVeVr5UfcYuaWjOVPZO6HHKykRFSiq/l0CS3KOx+pQQ', 'player'),
    ('teste03', '$argon2id$v=19$m=19456,t=2,p=1$+JQ9VVzpkUr0lP3GbRcF/A$a/hM8rrEIZ8nXckp4plHn5QgO8FtoTkONI7v7yocxks', '$argon2id$v=19$m=19456,t=2,p=1$dt6LjmU3Np2e0k3C5L4yaA$XHgmDfWU0yo5vHDtmdHCN9lmdvamuH5EnJpZGvotDvo', 'player'),
    ('teste04', '$argon2id$v=19$m=19456,t=2,p=1$GUvmjxd1velqrq/xLDeVzA$bsqGN/0A1sw3g+mGcNZ7PiYPb1OCtDWQjfmAfU99BAY', '$argon2id$v=19$m=19456,t=2,p=1$mfi/jYG3ofH5lUbs2Uz7BQ$hWeuUOOgKKmAvaZf5kZX8Jvj1/0k1Qvs+yhs0LyiD3U', 'player');

INSERT INTO characters (account_id, slot, name, class, level, exp, hp, mp, hp_max, mp_max, pos_x, pos_y)
VALUES
    ((SELECT id FROM accounts WHERE login = 'teste01'), 0, 'TesteTK', 0, 1, 0, 100, 50, 100, 50, 2115, 2050),
    ((SELECT id FROM accounts WHERE login = 'teste02'), 0, 'TesteFM', 1, 1, 0, 100, 50, 100, 50, 2116, 2050),
    ((SELECT id FROM accounts WHERE login = 'teste03'), 0, 'TesteBM', 2, 1, 0, 100, 50, 100, 50, 2117, 2050),
    ((SELECT id FROM accounts WHERE login = 'teste04'), 0, 'TesteHT', 3, 1, 0, 100, 50, 100, 50, 2118, 2050);

-- Inventário mínimo no primeiro slot de `Carry` (16 = Carry[0]): 10 Poções de Cura
-- (item 401) no TransKnight, para o teste de uso de consumível ter o que usar.
INSERT INTO character_items (character_id, slot, item_id, qty)
VALUES
    ((SELECT id FROM characters WHERE name = 'TesteTK' AND deleted_at IS NULL), 16, 401, 10);
