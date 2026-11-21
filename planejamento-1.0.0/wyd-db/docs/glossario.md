> Pergunta: "o que significa este termo?"
> Público: Desenvolvedor novo
> Dono: equipe do wyd-db
> Base: —
> Revisado em: 2026-11-18

Um verbete por termo, com definição de uma a duas linhas e o dono da página que aprofunda. Termo
que aparece em duas páginas com sentidos diferentes é defeito: o dono é único e os outros apontam.

## Domínio

| Termo | Definição | Dono |
|---|---|---|
| **Anct** (Ancient) | Composição Ancient do Compositor de Azran: arma **+9** (nUnique 41–49) + 1 jóia + até 6 sacrifícios em +7/+8/+9; no sucesso a arma **volta a +7** e recebe o efeito da jóia. Taxa = base + soma dos bônus dos sacrifícios (1% + 2/4/10 no legado). A flag `ancient` fica na instância do item — forma em `dominio-inventario.md`. | `dicionario-dados/dominio-refino.md` |
| **canal** | Instância do mundo dentro de um grupo (`game.server_channel`, PK `(group_id, channel)`): o **grupo** é a instalação de servidor, cada grupo tem `channel` 1–10 nos hosts do `8281` e o `channel = 0` é a linha de status do `serv00.htm`. Capacidade, manutenção e papel vivem na **própria linha do canal** — `max_players` (padrão 1000 = `MAX_USER` do legado), `maintenance` e `role` (`normal`/`verde`) —, e o `channel = 0` não tem nenhum dos três, garantido pelo `ck_server_channel_jogavel`. Não há tabela satélite: era 1:1 e existia só porque o legado proibia `ALTER TABLE` no `up` (`M23`). | `dicionario-dados/dominio-canal.md` |
| **catálogo** | Conteúdo estático do jogo (`content.item_catalog`, `content.skill_catalog`, `content.npc_template`, `content.npc_shop_item`, `content.mob_template_stat`, `content.mob_template_equip`): define o que **existe**, não o que o jogador tem. | `dicionario-dados/dominio-catalogo.md` |
| **conteúdo estático** | Dado gravado por seed na migration; a versão vigente fica em `content_version` (`0001_bootstrap.sql`) e a procedência de cada arquivo de origem em `content.content_source` (`0019_content_world.sql`; colunas em `ddl-confirmado.md` §5.21, aprovadas em 2026-11-20). Muda por deploy, nunca por ação de jogador — ao contrário de `game_setting`/`config_revision`, que são configuração editável e têm dono próprio (`dominio-configuracao.md`). O **procedimento** de semear e reverter é `planos/plano-de-dados-e-seeds.md`. | `dicionario-dados/dominio-catalogo.md` |
| **conta de jogo** | Linha em `game.account`: `login` de 1–16 caracteres único global (`login_fold`, `ux_account_login`), senha e PIN próprios, papel de jogo e uma vaga 1–5 dentro da pessoa (`ux_account_slot`); é a dona dos personagens e do cofre. | `dicionario-dados/dominio-acesso.md` |
| **delivery queue** | Fila `ops.delivery_queue`: pedido vindo da web que o servidor de jogo drena. É a ponte única para conceder item, ouro ou nível a uma conta de jogo ou personagem, com o staff autor registrado na própria linha (coluna do legado: `created_by_portal_user_id`); crédito de donate não passa por ela, tem razão própria em `game.donate_credit` (`dominio-economia.md`). Grant pendente impede apagar conta de jogo ou personagem (`ON DELETE RESTRICT`). A chave de idempotência é `idempotency_key` `UUID` única (`M17`, decidida em 2026-11-19); o desenho da tabela é `ddl-confirmado.md` §5.16, o dreno reivindica com `SKIP LOCKED` e a linha fechada é expurgada 30 dias após `closed_at` (`M25`, decidida em 2026-11-20). | `dicionario-dados/dominio-ponte-web-jogo.md` |
| **gema** | Efeito aplicado sobre o item (jogar/arrastar, não em NPC), parametrizado em `content.refine_gem_effect`; vale para item +10 ou superior e para item com a flag `ancient` — arma Anct fica em +7, e a flag dispensa o +10, não é equivalência de nível. As 4 gemas de troca são os itens 3386–3389 do `content.item_catalog` (`dominio-catalogo.md`). | `dicionario-dados/dominio-refino.md` |
| **grant** | Concessão pedida pela web — item, ouro, nível (`kind` em `item`/`gold`/`level`) — endereçada a uma conta de jogo ou personagem, que só passa a valer quando o jogo drena a linha correspondente da `ops.delivery_queue`. | `dicionario-dados/dominio-ponte-web-jogo.md` |
| **item** (instância) | Linha em `game.character_item` (do personagem, `slot` 0–79) ou `game.vault_item` (do baú da conta de jogo, PK `(account_id, slot)`, `slot` 0–127), que guarda o estado do exemplar: `refine` 0–15, flag `ancient`, o par `gem_effect`/`gem_value` (um sem o outro é linha inválida) e `qty`. **DDL confirmado em 2026-11-20** (`ddl-confirmado.md` §5.12). `item_id` é o `item_index` do `content.item_catalog`, mas **sem FK** no schema; gema exige `refine >= 10` ou `ancient`. | `dicionario-dados/dominio-inventario.md` |
| **personagem** | Linha em `game.character`: pertence a uma conta de jogo (`account_id`), ocupa um `slot` de 0 a 3, único por conta entre os não apagados, e tem nome único na mesma condição; o soft-delete (`deleted_at`) libera nome e slot. **DDL confirmado em 2026-11-20** (`ddl-confirmado.md` §5.10). | `dicionario-dados/dominio-personagem.md` |
| **pessoa** | Linha em `portal.person`: e-mail único normalizado por `email_fold` (`ux_person_email`) e senha do portal; possui até 5 contas de jogo e é quem autentica no site e no launcher, nunca no cliente. | `dicionario-dados/dominio-acesso.md` |
| **refino** | Nível de melhoria do item, de +0 a +15. Taxa por passo e insumo em `content.refine_rate` (`kind`: `dust` = pó, `compose` = +10/Anct/+12..+15); taxa por pó e índice atual é do SANC, não daqui. Proteção em `game.refine_pity`, efeito por gema em `content.refine_gem_effect`, regra de acessório em `content.accessory_refine_rule`. | `dicionario-dados/dominio-refino.md` |
| **SANC** | Taxa de sucesso do refino com pó, em %: `content.sanc_rate`, PK `powder` × `refine_index` (`PO`/`PL`/`AMAGO`, chave do legado). O `SancRate.txt` do legado traz 28 células e é aplicado sobre o default compilado (`g_pSancRate`), dando **36 efetivas**; a leitura é em `nível + 1`, não no índice do nome. Edição do staff em `content.sanc_rate_override`, lida por `COALESCE`. | `dicionario-dados/dominio-refino.md` |
| **sessão única** | `game.session`: no máximo uma sessão viva por `account_id` (índice parcial `ux_session_viva_por_conta`) e uma por `character_id` quando preenchido (`ux_session_viva_por_personagem`); a pessoa pode manter as **5** contas online ao mesmo tempo, em qualquer canal, porque a cerca é por conta e nunca por pessoa ou IP. `renewed_at` é o sinal de vida que evita travar conta ociosa. | `dicionario-dados/dominio-acesso.md` |
| **slot** | Posição numérica dentro do dono. É parte da chave primária em `game.character_item.slot` (0–79) e `game.vault_item.slot` (0–127), dono `dominio-inventario.md`; em `game.character.slot` (0–3) não está na PK, que é `id`, e sim num índice único parcial por conta (`dominio-personagem.md`). O tipo e os `CHECK`s dessas três tabelas estão no DDL confirmado (`ddl-confirmado.md` §5.10 e §5.12, 2026-11-20). A vaga da conta (`game.account.slot`, 1–5) é termo próprio: ver `vaga` (`dominio-acesso.md`). | `dicionario-dados/dominio-inventario.md` |
| **vaga** | Posição 1–5 de uma conta de jogo dentro da pessoa (`game.account.slot`). O índice único `ux_account_slot (person_id, slot) WHERE status <> 'closed'` é o que garante no banco o limite de 5 contas; conta fechada devolve a vaga, e a criação ocupa a menor livre. | `dicionario-dados/dominio-acesso.md` |

## Técnico

| Termo | Definição | Dono |
|---|---|---|
| **checksum** | SHA-256 do **arquivo inteiro** da migration — cabeçalho, `up` e `down` — com `CRLF` normalizado para `LF` antes de hashear; comparado com o gravado no ledger. Arquivo já aplicado que mudou é erro nomeado (`drift`), nunca skip. Cobre o `down` de propósito: alterá-lo invalida o rollback (`src/migrate/spec.md` §4). | `arquitetura/migracoes.md` |
| **ledger** | Tabela `public.schema_migrations (version, slug, checksum, applied_at)`, criada pelo runner antes do primeiro arquivo e validada por forma; `version` é a PK, `slug` é o nome do arquivo aplicado — renomear arquivo já aplicado é erro, não atualização de linha. Não guarda `down` nem duração (`src/migrate/spec.md` §3). | `arquitetura/migracoes.md` |
| **migration** | Arquivo `.sql` numerado, com `migrate:up` e `migrate:down` no mesmo arquivo. A série proíbe `ALTER TABLE` no `up`: coluna nova nasce na criação da tabela. A série é reescrita do zero para a `1.0.0` (plano diretor §1.2): cada arquivo cria as tabelas completas do seu domínio e nenhum altera tabela criada por outro; abre em `0001_bootstrap.sql` (4 schemas + `content_version`), a identidade entra em `0003_portal_person.sql`, a sessão de jogo em `0007_game_session.sql`, e o total planejado — 31 arquivos, 97 tabelas — vive em `plano-migrations.md` §3. | `guia-desenvolvedor/como-adicionar-migration.md` |
| **seed** | Dados iniciais de catálogo ou conteúdo gravados dentro da migration, não em passo manual; o `down` restaura o estado anterior da fonte em vez de apagar linha de outra migration. | `planos/plano-de-dados-e-seeds.md` |
| **tick** | Laço do servidor de jogo (passo de ~50 ms). Não faz I/O nem `await`: o estado do personagem sai por `SaveQueue::enqueue` (`try_send`, `crates/wyd-db/src/transfer/save.rs:3`). A `ops.delivery_queue` é o caminho **oposto**, Cold → Hot. | `arquitetura/fronteira-hot-cold.md` |
| **Hot × Cold** | Caminho quente (tick, memória, sem espera) contra caminho frio (banco, web, assíncrono). A fronteira entre os dois é a regra central do desenho: quente→frio é a `SaveQueue`, frio→quente é a `ops.delivery_queue`. | `arquitetura/fronteira-hot-cold.md` |

## Decisões de dono

| Data | Questão | Decisão |
|---|---|---|
| 2026-11-18 | `content_version` e `content.content_source` pertencem a `dominio-catalogo.md` ou a `dominio-configuracao.md`? | **`dominio-catalogo.md`**: versionam o **conteúdo** que o gerador carrega (o quê, de qual arquivo, com que checksum), não configuração do dono. `dominio-configuracao.md` fica com `game_setting`, `config_revision` e `config_change`. O procedimento de semear/reverter é de `planos/plano-de-dados-e-seeds.md`, e não vira dono do verbo. |

## Como um termo entra aqui

1. O termo é de domínio, não um detalhe de implementação. Nome de coluna, tipo, CHECK ou índice
   citado num verbete é **atalho de leitura**: a forma canônica é `dicionario-dados/gerado/<tabela>.md`
   (§6.8 do plano diretor), e a página de domínio explica o significado. O verbete que precisa do nome
   do objeto para fazer sentido mantém o nome e o aponta aqui, sem virar dono dele.
2. A definição cabe em duas linhas e não usa outro termo que ainda não esteja nesta página.
3. Existe **um** dono: a página que aprofunda o assunto. Se dois candidatos disputam, o dono é o
   mais próximo do dado, e o outro aponta.
4. Sinônimo aceito entra na mesma linha; sinônimo que muda o sentido vira termo próprio.
