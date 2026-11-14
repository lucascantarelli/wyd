//! Primeiro gameplay no servidor **vivo**: personagem novo → mundo → quest do Campo aceita.
//!
//! Gated por `WYD_LIVE_CPSOCK` e `DATABASE_URL`/`WYD_TEST_DATABASE_URL`. O personagem é criado
//! pelo protocolo (`_MSG_CreateCharacter` do `wyd-bots`), não por SQL: é o mesmo caminho da tela
//! de criação do 9662/7662. A prova é o estado real — `character_quest` com `step=1`,
//! `progress=0` — e o segundo clique, sem a chave 451, recusando o objetivo pendente.

#[path = "../common/mod.rs"]
mod common;

use std::sync::Arc;

use wyd_admin::auth::hash_secret;
use wyd_bots::{Bot, BotConfig, Race};
use wyd_db::{Accounts, Characters, PgBackend, PoolConfig, Role};

const SENHA: &str = "qa12345678";
const CONTA: &str = "qacampo01";
const NOME: &str = "QACAMPO01";
const QUEST: &str = "campo_de_treinamento";
/// `MobID` não: a âncora é a coordenada do treinador **herdado** do dump, que agora carrega o
/// `merchant` 36 da etapa 1 (a cópia `campo-treinador-1` do seed 0046 saiu na 0051).
const TREINADOR: (i32, i32) = (2080, 2018);
/// Chave da primeira porta (`ItemDropList.txt` do `GremlinDecoy`) e kit pago na etapa 1.
const CHAVE_451: i64 = 451;
const QUEST_HEAL_KIT: i64 = 682;
/// Ponto de save do personagem criado (`point_of_save` do template da classe).
const INICIO: (i32, i32) = (2096, 2096);
/// Coordenadas das fixtures removidas pela `0047`.
const FIXTURES: [(i32, i32); 8] = [
    (2108, 2046),
    (2112, 2046),
    (2116, 2046),
    (2120, 2046),
    (2115, 2042),
    (2120, 2048),
    (2125, 2052),
    (2130, 2055),
];

#[test]
fn personagem_novo_entra_e_aceita_a_quest_do_campo() {
    let Ok(endereco) = std::env::var("WYD_LIVE_CPSOCK") else {
        eprintln!("SKIP: WYD_LIVE_CPSOCK ausente");
        return;
    };
    let Some(dsn) = common::dsn_ou_skip("quest do campo") else {
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

    // Conta fixa sem personagem: o roster é limpo a cada execução para o char ser "novo".
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

    let mut cena = bot.read_scene(1500).expect("cena");
    cena.extend(bot.read_scene(1500).expect("cena 2"));
    for (x, y) in FIXTURES {
        let sobra = cena.iter().filter(|mob| mob.x == x && mob.y == y).count();
        assert_eq!(sobra, 0, "fixture removida ainda na cena em ({x},{y})");
    }
    // Sem duplicatas do Campo (0051): o dump já traz o NPC certo e o seed 0046 não repete.
    for nome in ["Unicorn", "Cabuncle Wind"] {
        assert_eq!(
            cena.iter().filter(|mob| mob.name == nome).count(),
            0,
            "cópia removida continua na cena: {nome}"
        );
    }
    // O treinador (2080,2018) está a 78 tiles do ponto de save, fora do raio de 56 da cena por
    // proximidade. Anda-se em passos curtos **lendo a cena a cada passo**: o `CreateMob` do NPC
    // entra no meio da caminhada e um `read_scene` só no fim não veria o que já foi consumido.
    let (mut x, mut y) = INICIO;
    while (x, y) != (TREINADOR.0, TREINADOR.1 + 1) {
        let proximo = (
            x + (TREINADOR.0 - x).clamp(-3, 3),
            y + (TREINADOR.1 + 1 - y).clamp(-3, 3),
        );
        bot.action_at((x, y), proximo)
            .expect("passo até o treinador");
        std::thread::sleep(std::time::Duration::from_millis(700));
        cena.extend(bot.read_scene(300).expect("cena do passo"));
        (x, y) = proximo;
    }
    cena.extend(bot.read_scene(1500).expect("cena do treinador"));
    // Acumula por `MobID`: a cena por proximidade só envia o que entra/sai, e um NPC que já
    // estava na vista antes da caminhada não é reenviado.
    let vistos: std::collections::BTreeMap<u16, wyd_bots::SceneMob> =
        cena.iter().cloned().map(|mob| (mob.id, mob)).collect();
    // Nomes ≤ 15 bytes: `MobName` é `char[16]` sem espaço para o `NUL` extra.
    // O Segundo Treinador fica em (2142,1990), 62 tiles a nordeste, fora do raio de 56.
    for nome in [
        "Chefe de Treino",
        "Unicornio Puro",
        "Ajudante",
        "Prim. Treinador",
        "Terc. Treinador",
    ] {
        assert_eq!(
            vistos.values().filter(|mob| mob.name == nome).count(),
            1,
            "NPC herdado deve aparecer uma vez só: {nome}"
        );
    }
    let treinador = vistos
        .values()
        .filter(|mob| mob.x == TREINADOR.0 && mob.y == TREINADOR.1)
        .collect::<Vec<_>>();
    assert_eq!(
        treinador.len(),
        1,
        "Primeiro Treinador deve ter uma única instância na cena"
    );
    let treinador = treinador[0].clone();

    // Aceite: primeiro clique no Primeiro Treinador.
    let painel = bot.quest(i32::from(treinador.id)).expect("aceite da quest");
    assert!(
        painel.contains("aceita"),
        "painel do aceite inesperado: {painel}"
    );

    let personagem = rt
        .block_on(backend.list_by_account(conta.id))
        .expect("roster")
        .into_iter()
        .find(|personagem| personagem.slot == 0)
        .expect("personagem criado pelo protocolo");
    let estado = rt
        .block_on(backend.quest_state(personagem.id, QUEST))
        .expect("estado da quest")
        .expect("linha em character_quest depois do aceite");
    assert_eq!(estado.stage, 0, "Newbie 0 = etapa 1 pendente");
    assert_eq!(estado.reward_mask, 0, "nenhuma recompensa paga no aceite");
    assert!(!estado.completed);

    // Objetivo é real: sem a chave 451 o mesmo treinador recusa e nada muda.
    let painel = bot
        .quest(i32::from(treinador.id))
        .expect("recusa sem prova");
    assert!(
        painel.contains("451"),
        "recusa deveria citar a prova 451: {painel}"
    );
    let depois = rt
        .block_on(backend.quest_state(personagem.id, QUEST))
        .expect("estado da quest")
        .expect("estado preservado");
    assert_eq!(depois.stage, 0, "sem prova o objetivo não avança");
    assert_eq!(depois.reward_mask, 0, "sem prova nada é pago");

    // Passo 1 real: abater o GremlinDecoy (`ItemDropList.txt`: dropa a chave 451).
    // `_MSG_Attack` leva o tile do alvo; o servidor resolve o NPC, rola a tabela do abate e
    // entrega o loot no inventário — nada é escrito à mão no banco.
    // O ataque valida a distância **do personagem** (2 tiles): o primeiro objetivo exige
    // andar até o guarda. Cada passo é um `_MSG_Action` validado no tick.
    let guarda = vistos
        .values()
        .find(|mob| mob.name == "Gremlin Decoy")
        .expect("GremlinDecoy na cena");
    let perto_do_guarda = (guarda.x, guarda.y + 1);
    bot.walk_to((TREINADOR.0, TREINADOR.1 + 1), perto_do_guarda)
        .expect("caminhada até o guarda");
    // O GremlinDecoy do dump tem 600 de HP e AC 40; com a `BASE_GetDamage` do legado
    // (`dam - ac/2 = -15` → escada de pisos) cada golpe de nível 1 tira **5**. São 120 golpes:
    // o `kill_at` do bot gasta ~5 s por tentativa (retorno do `_MSG_AttackOne`), então o ataque é
    // ritmado aqui no cooldown do servidor (800 ms) em vez da janela de 20 s.
    const GOLPES: usize = 140;
    for golpe in 0..GOLPES {
        bot.attack_at(guarda.x, guarda.y).expect("golpe no guarda");
        if golpe + 1 == GOLPES {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(900));
    }
    eprintln!(
        "PASS(passo 1): GremlinDecoy abatido em ({},{})",
        guarda.x, guarda.y
    );
    let apos_abate = rt
        .block_on(backend.items(personagem.id))
        .expect("inventário depois do abate");
    assert!(
        apos_abate.iter().any(|linha| linha.item_id == CHAVE_451),
        "a chave 451 precisa estar no inventário: {} linhas",
        apos_abate.len()
    );

    // Anda até o Primeiro Treinador (alcance de conversa) e entrega pelo `_MSG_Quest`.
    bot.walk_to(perto_do_guarda, (TREINADOR.0, TREINADOR.1 + 1))
        .expect("caminhada até o treinador");
    // Os `MobID` da cena são reciclados quando uma criatura sai (o guarda abatido libera o
    // índice e o respawn reaproveita): o id lido antes da caminhada pode já ser do decoy que
    // voltou. O alvo da entrega é resolvido de novo **pela coordenada** na cena acumulada.
    cena.extend(bot.read_scene(1_500).expect("cena da entrega"));
    let entrega = cena
        .iter()
        .rev()
        .find(|mob| mob.x == TREINADOR.0 && mob.y == TREINADOR.1 && mob.name.starts_with("Prim."))
        .cloned()
        .expect("Primeiro Treinador na cena da entrega");
    let painel = bot
        .quest(i32::from(entrega.id))
        .expect("entrega da chave 451");
    assert!(
        painel.contains("etapa 1/4"),
        "painel da entrega inesperado: {painel}"
    );
    let avancado = rt
        .block_on(backend.quest_state(personagem.id, QUEST))
        .expect("estado da quest")
        .expect("linha em character_quest");
    assert_eq!(avancado.stage, 1, "etapa 1 entregue (Newbie 1)");
    assert_eq!(avancado.reward_mask, 1, "bit da etapa 1 pago");
    let final_ = rt
        .block_on(backend.items(personagem.id))
        .expect("inventário final");
    assert!(
        !final_.iter().any(|linha| linha.item_id == CHAVE_451),
        "a chave 451 é consumida na entrega"
    );
    assert!(
        final_.iter().any(|linha| linha.item_id == QUEST_HEAL_KIT),
        "o kit 682 da etapa 1 entra no inventário"
    );
    eprintln!(
        "PASS(campo vivo): conta={CONTA} personagem={} id={} quest={QUEST} step=2 progress=1 npc={}",
        personagem.name, personagem.id, treinador.id
    );
}
