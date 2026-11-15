//! Nível no fio medido **nos bytes crus**, nos offsets calculados do legado.
//!
//! O codec do porte não serve de prova: este teste lê `frame[12..]` (corpo) com aritmética de
//! offset derivada de `Source/Code/Basedef.h` e grava os frames em `tmp/nivel-wire/` para o
//! parser independente (`tools/client/nivel-do-legado.mjs`) reconferir.
//!
//! Layout do legado (alinhamento natural do MSVC x86, sem `#pragma pack`):
//!
//! - `STRUCT_SCORE` (`Basedef.h:524`): `int Level` em +0, 48 B no total.
//! - `STRUCT_MOB` (`Basedef.h:556`): `BaseScore` em +44, `CurrentScore` em +92, 816 B.
//! - `STRUCT_SELCHAR` (`Basedef.h:1002`): `Score[4]` em +80, 48 B por slot.
//! - `MSG_DBCNFAccountLogin` (`Basedef.h:1413`, o corpo do `0x010A`): `HashKeyTable[16]` +
//!   `int` → `sel` em **+20** do corpo.
//! - `MSG_CNFNewCharacter` (`Basedef.h:1639`): `sel` logo após o `_MSG`; o cliente 7662 copia
//!   840 B de `frame+0x10` (`docs/cliente/pacotes.md`), ou seja `sel` em **+4** do corpo.
//! - `MSG_CNFCharacterLogin` (`Basedef.h:1683`): `short PosX, PosY` → `mob` em **+4** do corpo.
//! - `MSG_CreateMob` (`Basedef.h:1918`): `Score` em **+136** do frame → **+124** do corpo.
//! - `_MSG_UpdateScore` (`docs/arquitetura/protocolo.md` §5.2): `STRUCT_SCORE` em **+0**.
//!
//! Gated por `WYD_LIVE_CPSOCK` + `WYD_TEST_DSN`/`DATABASE_URL`.

#[path = "../common/mod.rs"]
mod common;

use std::sync::Arc;
use std::time::Duration;

use wyd_admin::auth::hash_secret;
use wyd_bots::{Bot, BotConfig, Race};
use wyd_db::{Accounts, Character, Characters, PgBackend, PoolConfig, Role};
use wyd_protocol::wire::header::HEADER_SIZE;
use wyd_protocol::wire::opcode::Opcode;

const SENHA: &str = "qa12345678";
const CONTA: &str = "qanivel01";
const CONTA_NIVEL: &str = "qanivel02";
/// `Porco` do campo (EXP 295 do template), alvo do level-up.
const PORCO: (i32, i32) = (2113, 2047);
/// Ponto de save do template do TK — mesma origem do `combate_vivo`.
const INICIO: (i32, i32) = (2096, 2096);
/// Dump no workspace (o `cargo test` roda com CWD no pacote).
const RELATORIO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tmp/nivel-wire");

/// `sizeof(STRUCT_SCORE)` e offset do `Level` dentro dele (`Basedef.h:524-552`).
const SCORE_LEN_LEGADO: usize = 48;
const SCORE_LEVEL_LEGADO: usize = 0;
/// `STRUCT_MOB`: `BaseScore`/`CurrentScore` (`Basedef.h:556-596`).
const MOB_BASE_SCORE_LEGADO: usize = 44;
const MOB_CURRENT_SCORE_LEGADO: usize = 92;
/// `STRUCT_SELCHAR.Score[4]` (`Basedef.h:1002-1017`).
const SELCHAR_SCORE_LEGADO: usize = 80;
/// `MSG_DBCNFAccountLogin.sel` (`Basedef.h:1415-1421`).
const SEL_AT_ACCOUNT_LEGADO: usize = 20;
/// `MSG_CNFNewCharacter`: o cliente lê em `frame+0x10` (`docs/cliente/pacotes.md`).
const SEL_AT_NEW_LEGADO: usize = 4;
/// `MSG_CNFCharacterLogin.mob` após `short PosX, PosY`.
const MOB_AT_LOGIN_LEGADO: usize = 4;
/// `MSG_CreateMob.Score` (`Basedef.h:1934`): 136 no frame − 12 do header.
const CREATE_MOB_SCORE_LEGADO: usize = 124;

fn runtime() -> Arc<tokio::runtime::Runtime> {
    Arc::new(
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .expect("runtime"),
    )
}

#[test]
fn nivel_do_banco_chega_ao_cliente_sem_salto() {
    let Ok(endereco) = std::env::var("WYD_LIVE_CPSOCK") else {
        eprintln!("SKIP: WYD_LIVE_CPSOCK ausente");
        return;
    };
    let Some(dsn) = common::dsn_ou_skip("nivel no fio") else {
        return;
    };
    let rt = runtime();
    let backend = Arc::new(
        rt.block_on(PgBackend::connect(&dsn, PoolConfig::local()))
            .expect("banco"),
    );

    let hash = hash_secret(SENHA).expect("argon2id");
    match rt.block_on(backend.create_account(CONTA, Role::Player)) {
        Ok(_) => {}
        Err(wyd_db::DbError::ConstraintViolation { .. }) => {}
        Err(error) => panic!("conta: {error}"),
    }
    rt.block_on(backend.set_credentials(CONTA, &hash, Role::Player))
        .expect("senha da conta");
    let conta = rt.block_on(backend.find_by_login(CONTA)).expect("conta");
    for personagem in rt
        .block_on(backend.list_by_account(conta.id))
        .expect("roster")
    {
        rt.block_on(backend.soft_delete(personagem.id))
            .expect("limpa personagem antigo");
    }
    std::fs::create_dir_all(RELATORIO).expect("tmp/nivel-wire");

    let mut config = BotConfig::new(endereco.clone(), CONTA, SENHA);
    config.pin = "2468".to_string();

    // Níveis gravados direto no banco para os casos intermediário e teto.
    for (slot, nivel) in [(0_i32, 200_i64), (1, 399)] {
        let personagem = Character {
            id: 0,
            account_id: conta.id,
            slot: i64::from(slot),
            name: format!("QANIVEL{slot}"),
            class: 1,
            level: nivel,
            hp: 900,
            mp: 300,
            pos_x: i64::from(INICIO.0),
            pos_y: i64::from(INICIO.1),
            gold: 1_000,
            hp_max: 900,
            mp_max: 300,
            ..Character::default()
        };
        rt.block_on(backend.create_character(&personagem))
            .expect("personagem de borda");
    }

    // Criação por protocolo, duas vezes: `_MSG_CreateCharacter` → `CNFNewCharacter` +
    // `CNFCharacterLogin`. O banco nasce nível 1.
    for (slot, nome) in [(2_i32, "QANIVEL2"), (3, "QANIVEL3")] {
        config.slot = slot;
        let mut bot =
            Bot::connect_novo(&config, Race::TransKnight, nome, Race::TransKnight.class())
                .expect("criação + entrada no mundo");
        // O `CreateMob` do próprio personagem chega no tick seguinte ao login.
        let cena = bot.read_scene(1_500).expect("cena da criação");
        assert!(
            cena.iter().any(|mob| mob.name == nome),
            "{nome} sem CreateMob na cena: {:?}",
            cena.iter().map(|mob| mob.name.clone()).collect::<Vec<_>>()
        );
        let banco = ler_personagem(&rt, &backend, conta.id, slot);
        assert_eq!(banco.level, 1, "{nome}: banco nasce 1");
        despejar(&format!("criacao-{nome}"), &bot);
        let medido = medir(&bot, slot, "criacao", nome);
        conferir(medido, banco.level, "criacao", nome);
    }

    // Login de personagem existente: intermediário (200) e teto (399).
    for slot in [0_i32, 1] {
        config.slot = slot;
        let mut bot = Bot::connect(&config, Race::TransKnight).expect("login do char existente");
        bot.read_scene(1_500).expect("cena do login");
        let banco = ler_personagem(&rt, &backend, conta.id, slot);
        assert_eq!(
            banco.level,
            [200_i64, 399][usize::try_from(slot).expect("slot")]
        );
        despejar(&format!("login-slot{slot}"), &bot);
        let medido = medir(&bot, slot, "login", &banco.name);
        conferir(medido, banco.level, "login", &banco.name);
        drop(bot);
        std::thread::sleep(Duration::from_millis(300));
    }

    // Level-up ao vivo: personagem de nível 1 com 499 de EXP (o nível 2 custa 500 —
    // `LEVEL_CURVE[1]`), abatendo o `Porco` do campo (EXP 295 do template). O alvo é
    // estacionário e o caminho é o mesmo do `combate_vivo`.
    let conta_nivel = setup_conta(&rt, &backend, CONTA_NIVEL).expect("conta do level-up");
    let personagem = Character {
        id: 0,
        account_id: conta_nivel.id,
        slot: 0,
        name: "QANIVELUP".to_string(),
        class: 1,
        level: 1,
        exp: 499,
        hp: 83,
        mp: 46,
        pos_x: i64::from(INICIO.0),
        pos_y: i64::from(INICIO.1),
        gold: 1_000,
        hp_max: 83,
        mp_max: 46,
        ..Character::default()
    };
    rt.block_on(backend.create_character(&personagem))
        .expect("personagem do level-up");
    config.account = CONTA_NIVEL.to_string();
    config.slot = 0;
    let mut bot = Bot::connect(&config, Race::TransKnight).expect("login do char do level-up");
    let mut cena = bot.read_scene(1_500).expect("cena");
    let mut porco = cena.iter().find(|mob| mob.name == "Porco").cloned();
    let (mut x, mut y) = INICIO;
    while porco.is_none() && (x, y) != PORCO {
        let proximo = (
            x + (PORCO.0 - x).clamp(-3, 3),
            y + (PORCO.1 - y).clamp(-3, 3),
        );
        bot.action_at((x, y), proximo).expect("passo no campo");
        std::thread::sleep(Duration::from_millis(700));
        cena.extend(bot.read_scene(300).expect("cena do passo"));
        porco = cena.iter().find(|mob| mob.name == "Porco").cloned();
        (x, y) = proximo;
    }
    let porco = porco.unwrap_or_else(|| {
        panic!(
            "Porco ausente da cena (canal sem respawn? reinicie o canal-1): {:?}",
            cena.iter()
                .map(|m| (m.name.clone(), m.x, m.y))
                .collect::<Vec<_>>()
        )
    });
    eprintln!("ALVO Porco id={} ({},{})", porco.id, porco.x, porco.y);
    bot.walk_to((x, y), (porco.x, porco.y + 1))
        .expect("caminhada até o porco");
    let mut abatido = false;
    for _ in 0..4 {
        match bot.kill_at(porco.id, porco.x, porco.y) {
            Ok(()) => {
                abatido = true;
                break;
            }
            Err(erro) if erro.kind() == std::io::ErrorKind::TimedOut => continue,
            Err(erro) => panic!("abate do Porco: {erro}"),
        }
    }
    assert!(abatido, "Porco não morreu: sem EXP não há level-up");
    let mut banco = ler_personagem(&rt, &backend, conta_nivel.id, 0);
    for _ in 0..20 {
        if banco.level > 1 {
            break;
        }
        std::thread::sleep(Duration::from_millis(300));
        banco = ler_personagem(&rt, &backend, conta_nivel.id, 0);
    }
    assert!(
        banco.level > 1,
        "abate sem level-up no banco (level={} exp={})",
        banco.level,
        banco.exp
    );
    despejar("levelup", &bot);
    let update = nivel_do_update_score(&bot).expect("_MSG_UpdateScore do level-up");
    assert_eq!(
        i64::from(update + 1),
        banco.level,
        "UpdateScore do level-up: fio {update} + 1 = banco {}",
        banco.level
    );
    eprintln!(
        "PASS(levelup): banco={} exp={} UpdateScore[0]={update} desenhado={}",
        banco.level, banco.exp, banco.level
    );

    // O save de saída grava o estado vivo: nível e EXP têm de continuar os do abate. Antes do
    // `ActorRefresh::from_mob`, o logout regravava a EXP do login e o abate "sumia".
    let (nivel, exp) = (banco.level, banco.exp);
    drop(bot);
    std::thread::sleep(Duration::from_millis(2_000));
    let salvo = ler_personagem(&rt, &backend, conta_nivel.id, 0);
    assert_eq!(
        (salvo.level, salvo.exp),
        (nivel, exp),
        "save de saída regravou nível/EXP do login"
    );
    eprintln!(
        "PASS(save): banco apos logout nivel={} exp={}",
        salvo.level, salvo.exp
    );
}

/// Conta com credenciais e roster vazio; cria os personagens depois.
fn setup_conta(
    rt: &Arc<tokio::runtime::Runtime>,
    backend: &Arc<PgBackend>,
    conta_login: &str,
) -> Option<wyd_db::Account> {
    let hash = hash_secret(SENHA).expect("argon2id");
    match rt.block_on(backend.create_account(conta_login, Role::Player)) {
        Ok(_) => {}
        Err(wyd_db::DbError::ConstraintViolation { .. }) => {}
        Err(error) => panic!("conta: {error}"),
    }
    rt.block_on(backend.set_credentials(conta_login, &hash, Role::Player))
        .ok()?;
    let conta = rt.block_on(backend.find_by_login(conta_login)).ok()?;
    for personagem in rt.block_on(backend.list_by_account(conta.id)).ok()? {
        rt.block_on(backend.soft_delete(personagem.id)).ok()?;
    }
    Some(conta)
}

/// Valor cru lido nos quatro pacotes, cada um no offset do legado.
struct Medido {
    selchar: i32,
    selchar_offset: usize,
    login: i32,
    login_offset: usize,
    createmob: i32,
    createmob_offset: usize,
}

/// Lê o nível dos frames crus do bot nos offsets do legado (sem usar o codec do porte).
fn medir(bot: &Bot, slot: i32, caminho: &str, nome: &str) -> Medido {
    let slot = usize::try_from(slot).expect("slot");
    let sel_off = match caminho {
        "criacao" => SEL_AT_NEW_LEGADO,
        _ => SEL_AT_ACCOUNT_LEGADO,
    } + SELCHAR_SCORE_LEGADO
        + slot * SCORE_LEN_LEGADO
        + SCORE_LEVEL_LEGADO;
    let login_off = MOB_AT_LOGIN_LEGADO + MOB_CURRENT_SCORE_LEGADO + SCORE_LEVEL_LEGADO;

    // Criação: o roster que interessa é o `CNFNewCharacter` (o `CNFAccountLogin` anterior à
    // criação ainda não tem o personagem). Login: o `CNFAccountLogin`.
    let roster = match caminho {
        "criacao" => corpo_de(bot, Opcode::CNFNewCharacter)
            .or_else(|| corpo_de(bot, Opcode::CNFAccountLogin)),
        _ => corpo_de(bot, Opcode::CNFAccountLogin)
            .or_else(|| corpo_de(bot, Opcode::CNFNewCharacter)),
    }
    .expect("roster cru (CNFAccountLogin/CNFNewCharacter)");
    let login = corpo_de(bot, Opcode::CNFCharacterLogin).expect("CNFCharacterLogin cru");
    let proprio = corpo_create_mob(bot, nome).expect("CreateMob do próprio personagem");

    let login_base = MOB_AT_LOGIN_LEGADO + MOB_BASE_SCORE_LEGADO + SCORE_LEVEL_LEGADO;
    assert_eq!(
        i32_em(&login, login_base),
        i32_em(&login, login_off),
        "BaseScore[{login_base}] ≠ CurrentScore[{login_off}]"
    );

    Medido {
        selchar: i32_em(&roster, sel_off),
        selchar_offset: sel_off,
        login: i32_em(&login, login_off),
        login_offset: login_off,
        createmob: i32_em(&proprio, CREATE_MOB_SCORE_LEGADO + SCORE_LEVEL_LEGADO),
        createmob_offset: CREATE_MOB_SCORE_LEGADO + SCORE_LEVEL_LEGADO,
    }
}

fn conferir(medido: Medido, banco: i64, caminho: &str, nome: &str) {
    let banco = i32::try_from(banco).expect("nível");
    assert_eq!(
        medido.selchar + 1,
        banco,
        "{caminho} {nome}: Selchar[{}] cru + 1 ≠ banco",
        medido.selchar_offset
    );
    assert_eq!(
        medido.login + 1,
        banco,
        "{caminho} {nome}: CurrentScore[{}] cru + 1 ≠ banco",
        medido.login_offset
    );
    assert_eq!(
        medido.createmob + 1,
        banco,
        "{caminho} {nome}: CreateMob.Score[{}] cru + 1 ≠ banco",
        medido.createmob_offset
    );
    eprintln!(
        "PASS({caminho}): {nome} banco={banco} \
         Selchar[{}]={} login[{}]={} CreateMob[{}]={} desenhado={}",
        medido.selchar_offset,
        medido.selchar,
        medido.login_offset,
        medido.login,
        medido.createmob_offset,
        medido.createmob,
        medido.createmob + 1
    );
}

/// Nível do `_MSG_UpdateScore` (o refresh do level-up): `STRUCT_SCORE` em +0 do corpo.
fn nivel_do_update_score(bot: &Bot) -> Option<i32> {
    bot.raw_frames()
        .iter()
        .rfind(|frame| {
            Opcode::from_wire(u16::from_le_bytes([frame[4], frame[5]])) == Some(Opcode::UpdateScore)
        })
        .map(|frame| i32_em(&frame[HEADER_SIZE..], SCORE_LEVEL_LEGADO))
}

/// Corpo (sem header) do primeiro frame do opcode.
fn corpo_de(bot: &Bot, opcode: Opcode) -> Option<Vec<u8>> {
    bot.raw_frames()
        .iter()
        .find(|frame| Opcode::from_wire(u16::from_le_bytes([frame[4], frame[5]])) == Some(opcode))
        .map(|frame| frame[HEADER_SIZE..].to_vec())
}

/// Corpo do `CreateMob` do nome dado (`MobName` em +6 do corpo).
fn corpo_create_mob(bot: &Bot, nome: &str) -> Option<Vec<u8>> {
    bot.raw_frames().iter().find_map(|frame| {
        if Opcode::from_wire(u16::from_le_bytes([frame[4], frame[5]])) != Some(Opcode::CreateMob) {
            return None;
        }
        let corpo = &frame[HEADER_SIZE..];
        let fim = corpo[6..22]
            .iter()
            .position(|byte| *byte == 0)
            .unwrap_or(16);
        let visto = String::from_utf8_lossy(&corpo[6..6 + fim]);
        (visto == nome).then(|| corpo.to_vec())
    })
}

/// `i32` little-endian no offset dado do corpo.
fn i32_em(corpo: &[u8], offset: usize) -> i32 {
    i32::from_le_bytes(
        corpo[offset..offset + 4]
            .try_into()
            .expect("4 bytes do int"),
    )
}

/// Grava o dump hexadecimal dos frames de nível para o parser independente.
fn despejar(nome: &str, bot: &Bot) {
    let mut saida = String::new();
    for frame in bot.raw_frames() {
        let tipo = u16::from_le_bytes([frame[4], frame[5]]);
        let hex: String = frame.iter().map(|byte| format!("{byte:02x}")).collect();
        saida.push_str(&format!(
            "type=0x{tipo:04x} size={} hex={hex}\n",
            frame.len()
        ));
    }
    std::fs::write(format!("{RELATORIO}/{nome}.txt"), saida).expect("dump");
}

fn ler_personagem(
    rt: &Arc<tokio::runtime::Runtime>,
    backend: &Arc<PgBackend>,
    conta_id: i64,
    slot: i32,
) -> Character {
    rt.block_on(backend.list_by_account(conta_id))
        .expect("roster")
        .into_iter()
        .find(|personagem| personagem.slot == i64::from(slot))
        .expect("personagem do slot")
}
