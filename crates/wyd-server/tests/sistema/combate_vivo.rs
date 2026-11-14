//! Prova viva de **combate**: personagem novo (nível 1) abate um monstro de campo e a EXP do
//! template é creditada.
//!
//! O alvo é o `Porco` herdado do dump (AC 2, HP 70, EXP 295 — medidos do `npc_template`), que
//! fica no raio do ponto de save do Campo. O dano por golpe sai da `BASE_GetDamage` do legado
//! (`dam - ac/2` + escada de pisos), nunca de 1 fixo: com `Damage` 5 do template o golpe é 12.
//!
//! Gated por `WYD_LIVE_CPSOCK` + `WYD_TEST_DSN`/`DATABASE_URL`, como os demais testes vivos.

#[path = "../common/mod.rs"]
mod common;

use std::sync::Arc;

use wyd_admin::auth::hash_secret;
use wyd_bots::{Bot, BotConfig, Race};
use wyd_db::{Accounts, Characters, PgBackend, PoolConfig, Role};

const SENHA: &str = "qa12345678";
const CONTA: &str = "qacombate01";
const NOME: &str = "QACOMBATE01";
/// `point_of_save` do template da classe (mesma origem do `campo_vivo`).
const INICIO: (i32, i32) = (2096, 2096);

#[test]
fn personagem_nivel_1_abate_porco_e_recebe_exp() {
    let Ok(endereco) = std::env::var("WYD_LIVE_CPSOCK") else {
        eprintln!("SKIP: WYD_LIVE_CPSOCK ausente");
        return;
    };
    let Some(dsn) = common::dsn_ou_skip("combate vivo") else {
        return;
    };
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .expect("runtime");
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

    let mut config = BotConfig::new(endereco.clone(), CONTA, SENHA);
    config.pin = "2468".to_string();
    config.slot = 0;
    let mut bot = Bot::connect_novo(&config, Race::TransKnight, NOME, Race::TransKnight.class())
        .expect("criação + entrada no mundo");

    // O `CreateMob` do Porco chega pelo delta de cena ao andar; cada passo lê a cena.
    let mut cena = bot.read_scene(1_500).expect("cena");
    let mut porco = cena.iter().find(|mob| mob.name == "Porco").cloned();
    let (mut x, mut y) = INICIO;
    while porco.is_none() && (x, y) != (2113, 2047) {
        let proximo = (x + (2113 - x).clamp(-3, 3), y + (2047 - y).clamp(-3, 3));
        bot.action_at((x, y), proximo).expect("passo no campo");
        std::thread::sleep(std::time::Duration::from_millis(700));
        cena.extend(bot.read_scene(300).expect("cena do passo"));
        porco = cena.iter().find(|mob| mob.name == "Porco").cloned();
        (x, y) = proximo;
    }
    let porco = porco.expect("Porco na cena do campo");
    eprintln!(
        "ALVO porco scene_id={} em ({},{})",
        porco.id, porco.x, porco.y
    );

    // Um tile de distância: o ataque valida o alcance do personagem.
    let perto = (porco.x, porco.y + 1);
    bot.walk_to((x, y), perto).expect("caminhada até o porco");
    // `kill_at` do bot tem janela de 20 s e o golpe do servidor sai a cada ~4 s no caminho real
    // (ida e volta do `_MSG_Attack` + eco). 70 HP / 12 de dano = 6 golpes: repete a janela.
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
    assert!(
        abatido,
        "Porco não morreu em quatro janelas de `kill_at` (12 de dano por golpe, 70 de HP)"
    );
    eprintln!("PASS(abate): Porco {} saiu da cena", porco.id);

    // EXP do template creditada no banco (EXP 295 do `Porco`, sem multiplicador no canal).
    let personagem = rt
        .block_on(backend.list_by_account(conta.id))
        .expect("roster")
        .into_iter()
        .find(|personagem| personagem.slot == 0)
        .expect("personagem criado pelo protocolo");
    let mut exp = personagem.exp;
    let mut tentativas = 0;
    while exp <= 0 && tentativas < 20 {
        std::thread::sleep(std::time::Duration::from_millis(250));
        exp = rt
            .block_on(backend.list_by_account(conta.id))
            .expect("roster")
            .into_iter()
            .find(|personagem| personagem.slot == 0)
            .expect("personagem")
            .exp;
        tentativas += 1;
    }
    assert!(
        exp > 0,
        "abate sem EXP creditada (exp={exp}); o log do canal deve trazer event=death"
    );
    eprintln!("PASS(exp): personagem={} exp={exp}", personagem.name);
}
