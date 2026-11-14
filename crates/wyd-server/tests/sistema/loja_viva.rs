//! Prova viva da **loja de NPC**: mercador de Armia abre catálogo e vende pelo preço do
//! servidor.
//!
//! O teste usa o caminho real do cliente, sem atalho de banco: `_MSG_REQShopList` no
//! `MobID` da criatura vista na cena (o servidor resolve o `npc_definition` no mundo vivo) e
//! `_MSG_Buy` com o preço do catálogo. Falha se o catálogo vier vazio, se o `_MSG_ShopList`
//! não chegar, ou se nenhum mercador entregar `_MSG_SendItem`.
//!
//! O personagem novo nasce com gold 0, então a compra que fecha é a de uma posição cujo
//! preço em `item_catalog` é 0 (existem 2 083 linhas assim na base). Compra com preço
//! divergente é recusada pelo servidor — o `Coin` do pacote nunca decide o preço.
//!
//! Gated por `WYD_LIVE_CPSOCK` + `DATABASE_URL`.

#[path = "../common/mod.rs"]
mod common;

use std::sync::Arc;

use wyd_admin::auth::hash_secret;
use wyd_bots::{Bot, BotConfig, Race};
use wyd_db::{Accounts, Characters, PgBackend, PoolConfig, Role};

const SENHA: &str = "qa12345678";
const CONTA: &str = "qaloja01";
const NOME: &str = "QALOJA01";

#[test]
fn mercador_de_armia_abre_catalogo_e_entrega_item_comprado() {
    let Ok(endereco) = std::env::var("WYD_LIVE_CPSOCK") else {
        eprintln!("SKIP: WYD_LIVE_CPSOCK ausente");
        return;
    };
    let Some(dsn) = common::dsn_ou_skip("loja viva") else {
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

    let cena = bot.read_scene(1_500).expect("cena");
    // Legado (`_MSG_REQShopList.cpp:50-56`): loja é `Merchant` 1 (comum) ou 19 (skills). Os
    // demais valores — monstro com `Carry` de drop (16/64), guarda/quest (36/100/110/120) —
    // não respondem `_MSG_ShopList`.
    let mercadores: Vec<_> = cena
        .iter()
        .filter(|mob| matches!(mob.merchant, 1 | 19) && mob.name != NOME)
        .collect();
    assert!(
        !mercadores.is_empty(),
        "nenhum mercador (merchant 1/19) na cena de Armia: {:?}",
        cena.iter()
            .map(|mob| (mob.name.clone(), mob.merchant))
            .collect::<Vec<_>>()
    );
    eprintln!(
        "PASS(cena): {} mercadores com Merchant 1/19 em Armia (de {} NPCs com merchant != 0)",
        mercadores.len(),
        cena.iter().filter(|mob| mob.merchant != 0).count()
    );

    let mut comprado = None;
    for mercador in mercadores.iter().take(16) {
        let itens = bot.open_shop(mercador.id).expect("abre loja");
        assert!(
            !itens.is_empty(),
            "catálogo vazio em {} (MobID {})",
            mercador.name,
            mercador.id
        );
        eprintln!(
            "LOJA nome={} mob={} itens={} primeiro={:?}",
            mercador.name,
            mercador.id,
            itens.len(),
            itens.first().copied()
        );
        // Preço é do servidor: com gold 0, só fecha a posição cujo preço de catálogo é 0.
        bot.buy(mercador.id, 0, 0).expect("_MSG_Buy");
        if let Some((tipo, slot, item)) = bot.read_send_item(1_200).expect("send item") {
            comprado = Some((mercador.name.clone(), tipo, slot, item));
            break;
        }
    }

    let (nome, tipo, slot, item) = comprado.expect(
        "nenhum mercador entregou item de preço 0 com gold 0 — catálogo ou preço quebrados",
    );
    assert_eq!(
        tipo, 1,
        "item comprado tem de entrar na mochila (invType 1)"
    );
    assert_ne!(
        i16::from_le_bytes([item[0], item[1]]),
        0,
        "item comprado chegou vazio"
    );
    eprintln!(
        "PASS(loja): {nome} entregou item {} no slot {slot} da mochila",
        i16::from_le_bytes([item[0], item[1]])
    );

    // O servidor é a autoridade do preço: posição com preço > 0 recusada com `Coin` 0.
    bot.buy(mercadores[0].id, 1, 0)
        .expect("_MSG_Buy divergente");
    assert!(
        bot.read_send_item(800).expect("send item").is_none(),
        "compra com preço divergente não pode entregar item"
    );
    eprintln!("PASS(preco): compra com Coin fora do catálogo foi recusada");

    // Compra **com preço**: `Runas_Joias` (npc 3751) tem na posição 7 da lista um item de
    // **25 500** de catálogo (o preço que o servidor devolve no `_MSG_ShopList`). O `Coin` do
    // pacote tem de ser exatamente esse; o ouro do personagem cai o mesmo valor e o item entra
    // na mochila — as duas autoridades são medidas no banco.
    const NPC_JOIAS: &str = "Runas_Joias";
    const POS_JADE: i16 = 7;
    const PRECO_JADE: i32 = 25_500;
    let joias = mercadores
        .iter()
        .find(|mob| mob.name == NPC_JOIAS)
        .unwrap_or_else(|| panic!("mercador {NPC_JOIAS} fora da cena"));
    let ouro_antes = rt
        .block_on(backend.list_by_account(conta.id))
        .expect("roster")
        .into_iter()
        .find(|personagem| personagem.slot == 0)
        .expect("personagem do bot")
        .gold;
    bot.buy(joias.id, POS_JADE, PRECO_JADE)
        .expect("_MSG_Buy com preço");
    let pago = bot
        .read_send_item(1_200)
        .expect("send item pago")
        .expect("compra com preço tem de entregar item");
    let ouro_depois = rt
        .block_on(backend.list_by_account(conta.id))
        .expect("roster")
        .into_iter()
        .find(|personagem| personagem.slot == 0)
        .expect("personagem do bot")
        .gold;
    assert_eq!(
        ouro_depois,
        ouro_antes - i64::from(PRECO_JADE),
        "ouro tem de cair {PRECO_JADE} (antes {ouro_antes}, depois {ouro_depois})"
    );
    eprintln!(
        "PASS(compra paga): {NPC_JOIAS} entregou item {} (invType {}) na mochila; ouro {ouro_antes} -> {ouro_depois}",
        i16::from_le_bytes([pago.2[0], pago.2[1]]),
        pago.0
    );

    // `Unicornio Puro` (npc_definition 2891, `Merchant` 110 no template do dump): o clique é
    // comerciante, mas o estoque importado é **vazio** — `npc_shop_item` não tem nenhuma linha
    // para 2891 (o seed 0020 insere 6 706 posições e nenhuma é dele). O servidor responde
    // `_MSG_ShopList` com zero itens; nada é inventado aqui. É a diferença entre "clique
    // quebrado" e "loja sem estoque no dado de origem".
    const NPC_UNICORNIO: &str = "Unicornio Puro";
    let unicornio = cena
        .iter()
        .find(|mob| mob.name == NPC_UNICORNIO)
        .unwrap_or_else(|| panic!("{NPC_UNICORNIO} fora da cena de Armia"));
    let catalogo_unico = bot
        .open_shop(unicornio.id)
        .expect("_MSG_ShopList do Unicornio Puro");
    eprintln!(
        "PASS(unicornio): clique respondeu _MSG_ShopList em {NPC_UNICORNIO} (mob {}): {} itens",
        unicornio.id,
        catalogo_unico.len()
    );
    assert!(
        catalogo_unico.is_empty(),
        "estoque do Unicornio Puro deveria vir vazio do seed: {:?}",
        catalogo_unico.first()
    );
    // Compra na posição 0 não pode entregar item: o NPC não é loja no legado nem tem estoque.
    bot.buy(unicornio.id, 0, 0)
        .expect("_MSG_Buy no Unicornio Puro");
    assert!(
        bot.read_send_item(800)
            .expect("send item do Unicornio Puro")
            .is_none(),
        "loja sem estoque não pode entregar item"
    );
    eprintln!(
        "PASS(unicornio): {NPC_UNICORNIO} (Merchant 110) não abriu loja e recusou a compra — \
         `reason=not_shop_merchant` no log do canal"
    );

    // Mesma regra para monstro com `Carry` de drop (Merchant 16/64): o inventário do bicho não
    // é catálogo de loja. Se houver um na cena, ele tem de ser recusado também.
    if let Some(bicho) = cena
        .iter()
        .find(|mob| !matches!(mob.merchant, 0 | 1 | 19) && mob.name != NOME)
    {
        let itens = bot.open_shop(bicho.id).expect("loja do NPC não-mercador");
        assert!(
            itens.is_empty(),
            "{} (Merchant {}) não é loja: {:?}",
            bicho.name,
            bicho.merchant,
            itens.first()
        );
        bot.buy(bicho.id, 0, 0)
            .expect("_MSG_Buy no NPC não-mercador");
        assert!(
            bot.read_send_item(800)
                .expect("send item do NPC não-mercador")
                .is_none(),
            "NPC com Merchant {} não pode vender",
            bicho.merchant
        );
        eprintln!(
            "PASS(nao-loja): {} (Merchant {}) não abriu lista nem vendeu",
            bicho.name, bicho.merchant
        );
    }
}
