//! Combate pelo **pacote de ataque do cliente** (`_MSG_Attack`, `0x0367`, 164 B com
//! `Dam[13]`), não por chamada interna: o bot envia o frame cheio e o teste lê os bytes
//! enviados e as respostas nos offsets do legado.
//!
//! Layout do legado (`Basedef.h:2399-2432`, alinhamento natural do MSVC x86 — 12 B de header
//! `_MSG` + `Unk_1[4]` + `int CurrentHp` + `Unk_2[4]` + `long long CurrentExp` +
//! `short unk0` + `PosX/PosY` + `TargetX/TargetY` + `AttackerID` + `Progress` + 4 bytes de
//! flags + `short Rsv` + `int CurrentMp` + `SkillIndex` + `ReqMp` + `Dam[13]`, com
//! `STRUCT_DAM { int TargetID; int Damage; }`):
//!
//! | campo | offset no frame |
//! |---|---:|
//! | `CurrentHp` | 16 |
//! | `CurrentExp` | 24 |
//! | `PosX`/`PosY` | 34/36 |
//! | `TargetX`/`TargetY` | 38/40 |
//! | `AttackerID`/`Progress` | 42/44 |
//! | `Motion`/`SkillParm`/`DoubleCritical`/`FlagLocal` | 46/47/48/49 |
//! | `Rsv` | 50 |
//! | `CurrentMp` | 52 |
//! | `SkillIndex`/`ReqMp` | 56/58 |
//! | `Dam[0].TargetID`/`Dam[0].Damage` | 60/64 |
//!
//! A resposta do canal é `_MSG_AttackOne` (`0x039D`, 68 B) com `Dam[0] = { cena do alvo,
//! dano aplicado }` nos mesmos offsets — é o número que o cliente desenha.

#[path = "../common/mod.rs"]
mod common;

use std::sync::Arc;
use std::time::Duration;

use wyd_admin::auth::hash_secret;
use wyd_bots::{Bot, BotConfig, Race, SceneMob};
use wyd_db::{Accounts, Characters, PgBackend, PoolConfig, Role};
use wyd_protocol::wire::header::HEADER_SIZE;
use wyd_protocol::wire::opcode::Opcode;

const SENHA: &str = "qa12345678";
const CONTA: &str = "qaatk01";
const NOME: &str = "QAATK01";
const INICIO: (i32, i32) = (2096, 2096);
/// Canto do campo onde o `Porco` aparece (`combate_vivo`).
const PORCO: (i32, i32) = (2113, 2047);

// Offsets do `MSG_Attack`/`MSG_AttackOne` no legado (frame-relative).
const ATK_CURRENT_HP: usize = 16;
const ATK_POS: usize = 34;
const ATK_TARGET: usize = 38;
const ATK_MOTION: usize = 46;
const ATK_MP: usize = 52;
const ATK_DAM: usize = 60;
/// `Dam[0].Damage` — 4 bytes depois de `Dam[0].TargetID` (`STRUCT_DAM`).
const ATK_DANO: usize = ATK_DAM + 4;
const ATK_LEN_13: usize = 164;
const ATK_LEN_1: usize = 68;
/// O frame que o cliente 7662 manda: `0x039D` com 168 B (164 do `MSG_Attack` + 4 de trailer).
const CLIENT_ATTACK_LEN: usize = ATK_LEN_13 + 4;

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
fn ataque_do_cliente_derruba_dois_alvos_e_persiste_exp() {
    let Ok(endereco) = std::env::var("WYD_LIVE_CPSOCK") else {
        eprintln!("SKIP: WYD_LIVE_CPSOCK ausente");
        return;
    };
    let Some(dsn) = common::dsn_ou_skip("combate pelo pacote") else {
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
    std::fs::create_dir_all(RELATORIO).expect("tmp/combate-pacote");

    let mut config = BotConfig::new(endereco.clone(), CONTA, SENHA);
    config.pin = "2468".to_string();
    config.slot = 0;
    let mut bot = Bot::connect_novo(&config, Race::TransKnight, NOME, Race::TransKnight.class())
        .expect("criação + entrada no mundo");
    let antes = ler_personagem(&rt, &backend, conta.id);
    let itens_antes = rt
        .block_on(backend.items(antes.id))
        .expect("inventário inicial");
    eprintln!(
        "ANTES: banco nivel={} exp={} itens={:?}",
        antes.level,
        antes.exp,
        itens_antes
            .iter()
            .map(|linha| (linha.slot, linha.item_id, linha.qty))
            .collect::<Vec<_>>()
    );

    // --- alvo 1: Porco (AC 2, HP 70, EXP 295) -------------------------------------------
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
    let porco = porco.expect("Porco na cena");
    bot.walk_to((x, y), (porco.x, porco.y + 1))
        .expect("caminhada até o porco");

    let (sent_marca, raw_marca) = (bot.sent_frames().len(), bot.raw_frames().len());
    let abatido = atacar_ate_morrer(&mut bot, &porco);
    if !abatido {
        for frame in bot.raw_frames().iter().skip(raw_marca) {
            eprintln!("recv type=0x{:04x} id={}", tipo(frame), u16_em(frame, 6));
        }
    }
    assert!(abatido, "Porco não morreu pelo _MSG_Attack");
    conferir_ataque(&bot, sent_marca, raw_marca, &porco, "Porco", ATK_LEN_13);
    let depois_porco = ler_personagem(&rt, &backend, conta.id);
    assert_eq!(depois_porco.exp, 295, "EXP do Porco no banco");
    eprintln!(
        "PASS(porco): banco exp={} nivel={} (antes exp={} nivel={})",
        depois_porco.exp, depois_porco.level, antes.exp, antes.level
    );

    // --- alvo 2: criatura **com tabela de drop real** (EXP + item) ----------------------
    // `drop_rate_override` tem só quatro templates, todos com peso 1.000.000 (100%):
    // `GremlinDecoy` (451), `OrcSniper` (453), `GiantCrill` (452) e `OrcWarrior` (524).
    // O `Porco` não tem tabela, então o item só pode sair de um destes — e o escolhido tem de
    // estar na cena desta sessão, senão o teste falha mostrando o que veio no `CreateMob`.
    // Ordem por tempo de abate: o `Gremlin Decoy` fica parado ao lado do treinador (600 de HP);
    // o `Orc Sniper` anda e tem 2 500 de HP, então fica por último.
    const COM_DROP: [(&str, i64); 4] = [
        ("Gremlin Decoy", 451),
        ("Giant Crill", 452),
        ("Guerreiro Orc", 524),
        ("Orc Sniper", 453),
    ];
    // A criatura com tabela (o `Gremlin Decoy`, 600 de HP) fica no Campo de Treinamento, longe
    // do `Porco`. Anda-se até lá e **reloga**: o `CreateMob` de um NPC que já estava na vista
    // não é reenviado, então a cena cheia do login é o jeito de garantir a lista completa.
    let (mut x, mut y) = (porco.x, porco.y + 1);
    while (x, y) != (2080, 2019) {
        let proximo = (x + ((2080 - x).clamp(-3, 3)), y + ((2019 - y).clamp(-3, 3)));
        bot.action_at((x, y), proximo)
            .expect("passo até o campo de treinamento");
        std::thread::sleep(Duration::from_millis(700));
        cena.extend(bot.read_scene(300).expect("cena do passo"));
        (x, y) = proximo;
    }
    std::thread::sleep(Duration::from_millis(1_500));
    drop(bot);
    std::thread::sleep(Duration::from_millis(2_000));
    let mut bot = Bot::connect(&config, Race::TransKnight).expect("relogin no campo");
    cena.extend(bot.read_scene(2_000).expect("cena do relogin"));
    let (alvo2, item_esperado) = COM_DROP
        .iter()
        .find_map(|(nome, item)| {
            cena.iter()
                .find(|mob| mob.name == *nome)
                .map(|mob| (mob.clone(), *item))
        })
        .unwrap_or_else(|| {
            let mut nomes: Vec<String> = cena
                .iter()
                .map(|mob| format!("{}@{},{} id={}", mob.name, mob.x, mob.y, mob.id))
                .collect();
            nomes.sort();
            nomes.dedup();
            panic!("criatura com tabela de drop na cena da sessão (CreateMob vistos: {nomes:?})")
        });
    eprintln!(
        "alvo 2: {} id={} em ({},{}) hp={} item da tabela={item_esperado}",
        alvo2.name, alvo2.id, alvo2.x, alvo2.y, alvo2.max_hp
    );
    bot.walk_to((2080, 2019), (alvo2.x, alvo2.y + 1))
        .expect("caminhada até o alvo com drop");
    std::thread::sleep(Duration::from_millis(3_000));

    let (sent_marca, raw_marca) = (bot.sent_frames().len(), bot.raw_frames().len());
    let mut abatido = false;
    // 24 janelas de 25 s: o alvo de 600 de HP cai em ~100 golpes de 6.
    for _ in 0..24 {
        if atacar_ate_morrer(&mut bot, &alvo2) {
            abatido = true;
            break;
        }
    }
    assert!(
        abatido,
        "{} não morreu pelo frame de ataque do cliente",
        alvo2.name
    );
    conferir_ataque(&bot, sent_marca, raw_marca, &alvo2, &alvo2.name, ATK_LEN_13);
    // O `_MSG_SendItem` do drop sai do canal logo depois do `RemoveMob`; a leitura o recolhe
    // antes de conferir o frame e o inventário.
    let _ = bot.read_scene(1_500);
    // Despeja já aqui: o dump é a prova crua mesmo se uma asserção abaixo falhar.
    despejar("combate", &bot);
    std::thread::sleep(Duration::from_millis(800));
    let depois_alvo2 = ler_personagem(&rt, &backend, conta.id);
    assert!(
        depois_alvo2.exp > depois_porco.exp,
        "EXP do abate com drop não entrou no banco ({} -> {})",
        depois_porco.exp,
        depois_alvo2.exp
    );
    let itens_depois = rt
        .block_on(backend.items(depois_alvo2.id))
        .expect("inventário depois do abate");
    let presentes: Vec<i64> = itens_depois
        .iter()
        .filter(|linha| linha.item_id == item_esperado)
        .map(|linha| linha.slot)
        .collect();
    assert!(
        !presentes.is_empty(),
        "drop {item_esperado} ausente no inventário: {:?}",
        itens_depois
            .iter()
            .map(|linha| (linha.slot, linha.item_id, linha.qty))
            .collect::<Vec<_>>()
    );
    let novos: Vec<String> = itens_depois
        .iter()
        .filter(|linha| {
            !itens_antes
                .iter()
                .any(|velho| velho.slot == linha.slot && velho.item_id == linha.item_id)
        })
        .map(|linha| {
            format!(
                "slot={} item={} qty={}",
                linha.slot, linha.item_id, linha.qty
            )
        })
        .collect();
    assert!(
        novos
            .iter()
            .any(|novo| novo.contains(&format!("item={item_esperado}"))),
        "o item novo do abate não está no delta do inventário: {novos:?}"
    );
    // O cliente só sabe do item se o canal mandar `_MSG_SendItem`; o eixo do item no frame é
    // o `STRUCT_ITEM` de 8 B em +16 (índice em +16, `little-endian`).
    // `raw_frames()` já guarda o frame **de-ofuscado** (o `recv` decodifica a `KeyWord`
    // antes de empilhar), então tipo/slot/item saem direto dos offsets.
    let enviados_item: Vec<(u16, u16)> = bot
        .raw_frames()
        .iter()
        .skip(raw_marca)
        .filter(|frame| tipo(frame) == Opcode::SendItem.wire())
        .map(|frame| (u16_em(frame, 14), u16_em(frame, 16)))
        .collect();
    let tipos: Vec<u16> = bot
        .raw_frames()
        .iter()
        .skip(raw_marca)
        .map(|frame| tipo(frame))
        .collect();
    assert!(
        enviados_item
            .iter()
            .any(|(_, indice)| i64::from(*indice) == item_esperado),
        "nenhum _MSG_SendItem com item {item_esperado}: {enviados_item:?} (opcodes lidos: {tipos:?})"
    );
    eprintln!(
        "PASS({}): banco exp={} nivel={} item={item_esperado} slots={presentes:?} novos={novos:?} SendItem={enviados_item:?}",
        alvo2.name, depois_alvo2.exp, depois_alvo2.level
    );

    // --- persistência: o save de saída não devolve EXP/nível do login -------------------
    let (nivel, exp) = (depois_alvo2.level, depois_alvo2.exp);
    despejar("combate", &bot);
    drop(bot);
    std::thread::sleep(Duration::from_millis(2_000));
    let salvo = ler_personagem(&rt, &backend, conta.id);
    assert_eq!(
        (salvo.level, salvo.exp),
        (nivel, exp),
        "save de saída regravou nível/EXP do login"
    );
    let itens_salvos = rt
        .block_on(backend.items(salvo.id))
        .expect("inventário salvo");
    assert!(
        itens_salvos
            .iter()
            .any(|linha| linha.item_id == item_esperado),
        "drop {item_esperado} sumiu no save de saída: {:?}",
        itens_salvos
            .iter()
            .map(|linha| (linha.slot, linha.item_id, linha.qty))
            .collect::<Vec<_>>()
    );
    eprintln!(
        "PASS(save): apos logout nivel={} exp={} item={item_esperado} presente",
        salvo.level, salvo.exp
    );

    // --- relogin: o personagem volta do banco com o item -------------------------------
    let mut relogin = Bot::connect(&config, Race::TransKnight).expect("relogin no canal");
    relogin.read_scene(1_200).expect("cena do relogin");
    let itens_relogin = rt
        .block_on(backend.items(salvo.id))
        .expect("inventário após relogin");
    assert!(
        itens_relogin
            .iter()
            .any(|linha| linha.item_id == item_esperado),
        "drop {item_esperado} ausente após relogin"
    );
    eprintln!(
        "PASS(relogin): conta={CONTA} personagem={NOME} item={item_esperado} presente exp={}",
        ler_personagem(&rt, &backend, conta.id).exp
    );
    despejar("relogin", &relogin);
    drop(relogin);
}

const RELATORIO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tmp/combate-pacote");

/// Ataca por até uma janela do bot com o **frame do cliente**; `true` quando o
/// `_MSG_RemoveMob` do alvo chega.
fn atacar_ate_morrer(bot: &mut Bot, alvo: &SceneMob) -> bool {
    bot.kill_client_at(alvo.id, alvo.x, alvo.y).is_ok()
}

/// Confere o frame **enviado** e as respostas nos offsets do legado.
fn conferir_ataque(
    bot: &Bot,
    sent_marca: usize,
    raw_marca: usize,
    alvo: &SceneMob,
    nome: &str,
    tamanho: usize,
) {
    // `sent_frames` guarda o frame **como saiu no fio** (cabeçalho e corpo ofuscados pela
    // `KeyWord`); de-ofuscar aqui é o que prova que o byte enviado é o `MSG_Attack` do legado.
    let enviados: Vec<Vec<u8>> = bot
        .sent_frames()
        .iter()
        .skip(sent_marca)
        .map(|frame| {
            let mut copia = frame.clone();
            wyd_protocol::codec::keyword::decode_in_place(&mut copia).expect("decode da KeyWord");
            copia
        })
        .filter(|frame| tipo(frame) == Opcode::AttackOne.wire())
        .collect();
    assert!(
        !enviados.is_empty(),
        "{nome}: nenhum frame de ataque enviado"
    );
    // O primeiro golpe pode ir só com o tile (`Dam[0].TargetID = 0`); o que importa é que o
    // pacote cheio com o id do alvo tenha ido — é ele que prova a rota por `Dam[i].TargetID`.
    let com_id = enviados
        .iter()
        .position(|frame| i32_em(frame, ATK_DAM) == i32::from(alvo.id))
        .unwrap_or_else(|| {
            panic!(
                "{nome}: nenhum frame levou Dam[0].TargetID={} (ids vistos: {:?})",
                alvo.id,
                enviados
                    .iter()
                    .map(|f| i32_em(f, ATK_DAM))
                    .collect::<Vec<_>>()
            )
        });
    let primeiro = &enviados[com_id];
    assert_eq!(
        primeiro.len(),
        CLIENT_ATTACK_LEN,
        "{nome}: Size do frame do cliente (0x039D, {} B)",
        tamanho
    );
    let alvo_x = u16_em(primeiro, ATK_TARGET);
    let alvo_y = u16_em(primeiro, ATK_TARGET + 2);
    assert_eq!(
        (i32::from(alvo_x), i32::from(alvo_y)),
        (alvo.x, alvo.y),
        "{nome}: TargetX@38/TargetY@40 do frame enviado"
    );
    assert_eq!(
        i32_em(primeiro, ATK_DAM),
        i32::from(alvo.id),
        "{nome}: Dam[0].TargetID@60"
    );
    assert_eq!(
        i32_em(primeiro, ATK_DANO),
        0,
        "{nome}: Dam[0].Damage@64 é do cliente (nunca creditado)"
    );
    assert_eq!(
        u16_em(primeiro, ATK_POS),
        alvo_x,
        "{nome}: PosX@34 (o bot repete o alvo, como o `AttackOne` do cliente)"
    );
    assert_eq!(primeiro[ATK_MOTION], 1, "{nome}: Motion@46");
    let _ = (ATK_CURRENT_HP, ATK_MP);

    // Respostas: `_MSG_AttackOne` com o dano aplicado em `Dam[0]`.
    let ecos: Vec<&Vec<u8>> = bot
        .raw_frames()
        .iter()
        .skip(raw_marca)
        .filter(|frame| tipo(frame) == Opcode::AttackOne.wire())
        .collect();
    assert!(!ecos.is_empty(), "{nome}: nenhum eco _MSG_AttackOne");
    eprintln!(
        "{nome}: ecos={} [{}]",
        ecos.len(),
        ecos.iter()
            .map(|eco| format!("{}/{}", i32_em(eco, ATK_DAM), i32_em(eco, ATK_DANO)))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let mut total = 0_i64;
    for eco in &ecos {
        assert_eq!(eco.len(), ATK_LEN_1, "{nome}: Size do eco");
        assert_eq!(
            i32_em(eco, ATK_DAM),
            i32::from(alvo.id),
            "{nome}: eco Dam[0].TargetID@60 (hex={})",
            eco.iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        );
        let dano = i32_em(eco, ATK_DANO);
        assert!(dano > 0, "{nome}: eco com dano {dano}");
        total += i64::from(dano);
    }
    let hp = i64::from(alvo.max_hp);
    assert!(
        total >= hp,
        "{nome}: dano somado {total} < HP {hp} (não derrubaria)"
    );
    // O último golpe não passa do HP restante.
    let ultimo = i32_em(ecos[ecos.len() - 1], ATK_DANO);
    assert!(
        i64::from(ultimo) <= hp,
        "{nome}: golpe final {ultimo} maior que o HP {hp}"
    );
    assert!(
        bot.raw_frames()
            .iter()
            .skip(raw_marca)
            .any(|frame| tipo(frame) == Opcode::RemoveMob.wire()),
        "{nome}: sem _MSG_RemoveMob depois do abate"
    );
    eprintln!(
        "PASS({nome}): env Attack Size={} TargetX@38={} TargetY@40={} Dam[0].TargetID@60={} | \
         ecos={} dano_total={} HP={} ultimo={} RemoveMob=sim",
        primeiro.len(),
        alvo_x,
        alvo_y,
        i32_em(primeiro, ATK_DAM),
        ecos.len(),
        total,
        hp,
        ultimo
    );
}

/// `Type` do frame (já de-ofuscado) e checagem de tamanho mínimo.
fn tipo(frame: &[u8]) -> u16 {
    u16::from_le_bytes([frame[4], frame[5]])
}

fn u16_em(frame: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([frame[offset], frame[offset + 1]])
}

fn i32_em(frame: &[u8], offset: usize) -> i32 {
    i32::from_le_bytes(
        frame[offset..offset + 4]
            .try_into()
            .expect("4 bytes do int"),
    )
}

/// Dump hexadecimal dos frames enviados e recebidos (o byte que passou no fio).
fn despejar(nome: &str, bot: &Bot) {
    let mut saida = String::new();
    for frame in bot.sent_frames() {
        let bruto: String = frame.iter().map(|byte| format!("{byte:02x}")).collect();
        let mut copia = frame.clone();
        wyd_protocol::codec::keyword::decode_in_place(&mut copia).expect("decode da KeyWord");
        let hex: String = copia.iter().map(|byte| format!("{byte:02x}")).collect();
        saida.push_str(&format!("wire size={} hex={bruto}\n", frame.len()));
        saida.push_str(&format!("c2s size={} hex={hex}\n", copia.len()));
    }
    for frame in bot.raw_frames() {
        let hex: String = frame.iter().map(|byte| format!("{byte:02x}")).collect();
        saida.push_str(&format!("s2c size={} hex={hex}\n", frame.len()));
    }
    std::fs::write(format!("{RELATORIO}/{nome}.txt"), saida).expect("dump");
}

fn ler_personagem(
    rt: &Arc<tokio::runtime::Runtime>,
    backend: &Arc<PgBackend>,
    conta_id: i64,
) -> wyd_db::Character {
    rt.block_on(backend.list_by_account(conta_id))
        .expect("roster")
        .into_iter()
        .find(|personagem| personagem.slot == 0)
        .expect("personagem do slot 0")
}

// `HEADER_SIZE` entra só para documentar que os offsets acima são frame-relative.
const _: () = assert!(HEADER_SIZE == 12);
