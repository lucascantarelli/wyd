# Autenticação no canal real: senha errada e PIN (2026-11-14)

> **Atualização (mesma data, rodada seguinte):** a forma de recusa de senha descrita aqui foi
> ajustada para ficar **idêntica ao painel que o jogador vê na exclusão**: o `0x0117` saiu do fio,
> o `*Fail` passou a header-only (12 B) e a conexão é drenada em vez de fechar no instante do
> painel. Os bytes atuais, com antes/depois, estão em
> [`painel-login-2026-11-14.md`](painel-login-2026-11-14.md); as seções abaixo descrevem a medição
> daquele momento e continuam válidas para PIN e para o estado de banco. **Segunda atualização:**
> a recusa passou a ser **um** `_MSG_AlreadyPlaying` (`0x011C`, 12 B) — o único braço de mensagem
> da etapa de login do 7662 ([`login-tela-handler-2026-11-14.md`](login-tela-handler-2026-11-14.md)).

Pergunta do relato: *"usuário/senha errada não informa nada no cliente"* e *"segunda senha aceita
qualquer coisa"*. Esta rodada mede as duas pernas **no canal 1 vivo** (`127.0.0.1:8281`) com
cliente TCP cru, e confronta com o banco. O que o 7662 desenha na tela não é observável por este
agente (sem GUI): o que está provado aqui são os bytes que o servidor emite e o estado do banco.

## 1. Recusa de senha — bytes medidos

Comando:

```
WYD_LIVE_CPSOCK=127.0.0.1:8281 WYD_TEST_REQUIRED=1 \
WYD_TEST_DSN=postgres://admin:admin123@127.0.0.1:5432/wyd \
cargo test -p wyd-server --test sistema_login_canal -- --nocapture
```

Saída medida (uma linha por caso, bytes crus do canal):

```
PASS(senha_errada): NewAccountFail size=13 [0d000bde1701010000000000] |
  CharacterLoginFail size=13 [0d000bde1901010000000000] |
  MessagePanel size=140 texto="Senha ou conta incorreta" | EOF
PASS(conta_inexistente): NewAccountFail size=13 [0d000bde1701010000000000] |
  CharacterLoginFail size=13 [0d000bde1901010000000000] |
  MessagePanel size=140 texto="Senha ou conta incorreta" | EOF
```

Frames completos (`tmp/login-canal/recusas.txt`, ofuscação `key=0x0B` já removida pelo leitor):

| # | Caso | Bytes | Offsets |
|---|---|---|---|
| c2s | senha errada | `74000bff2be7f5fd07edccf9…` (116 B) | `AccountLogin`: nome@12 (16 B), senha@28 (12 B), versão@92 (`0x2ee0` = 12000) |
| s2c 1 | `_MSG_NewAccountFail` | `0d000bde 1701 0100 00000000 01` | `Size@0`=13, `Type@4`=`0x0117`, `motivo@12`=`0x01` (`Senha`) |
| s2c 2 | `_MSG_CharacterLoginFail` | `0d000bde 1901 0100 00000000 01` | `Size@0`=13, `Type@4`=`0x0119`, `motivo@12`=`0x01` (`Senha`) |
| s2c 3 | `_MSG_MessagePanel` | `8c000b5a 0101 3075 00000000` + `Senha ou conta incorreta` | `Size@0`=140, `Type@4`=`0x0101`, `id@8`=30000, **texto@12** (128 B, NUL-padded) |

Por que estes três: o RE estático do build 7662 (`docs/audit/04-client-analysis.md` §5,
`docs/audit/06-protocol-catalog.md` PKT-0x0101/PKT-0x0119) mostra que `0x0117` **não** é comparado
pelo binário, `0x0119` é, e `0x0101` é a única via de texto livre. O porte emite os três: o
`0x0117` mantém paridade com o servidor legado, o `0x0119` é o `*Fail` que o cliente trata e o
`0x0101` carrega o motivo legível. Conta inexistente produz **a mesma tríade** (não revela se o
login existe).

Antes/depois do estado: em ambos os casos nenhuma linha de `sessions`/`characters` é criada para a
conta e a conexão fecha logo após o painel (`EOF`, 0 B extras) — medido pelo `read_to_end` do
teste, que falha se sobrar byte.

## 2. Segunda senha (PIN) — buraco fechado e medido

**Defeito medido antes**: em `cpsock/login.rs`, o braço `CreateCharacter | DeleteCharacter |
CharacterLogin` abria a sessão **sem** olhar o PIN quando `registrado == true` (log
`result=deferred … reason=selection_first`). Cliente que pulasse `_MSG_AccountSecure` entrava sem
PIN; com o PIN errado o servidor só recusava quem *mandasse* o PIN.

**Correção**: nesse braço, conta com PIN cadastrado que manda a seleção direto recebe a mesma
recusa do PIN errado — `_MSG_AccountSecureFail` (`0x0FDF`, motivo `PinIncorreto`) +
`_MSG_MessagePanel` — e a sessão não abre. Conta sem PIN continua entrando direto
(`primeira_entrada_sem_pin_vai_direto_para_a_selecao` segue verde: PIN é opcional no cadastro).

Bytes medidos no canal real (`tmp/login-canal/pin.txt`):

| Caso | Bytes do canal | Leitura |
|---|---|---|
| PIN errado (`0000`) | `0d000bde df0f 0100 00000000 02` | `Size@0`=13, `Type@4`=`0x0FDF`, `motivo@12`=`0x02` (`PinIncorreto`) |
| painel do PIN | `8c000b5a 0101 3075 00000000` + `PIN incorreto. Tente novamente.` | `Size@0`=140, `Type@4`=`0x0101`, texto@12 |
| PIN correto (`2468`) | `0c000bac de0f 0100 00000000` | `Size@0`=12, `Type@4`=`0x0FDE` (`AccountSecure`), sem corpo |
| seleção sem PIN | c2s `24000b112de7f5fd09edccf934f118fd0a6164666816312df2e40cfb1cf1f8fd5ef194fd` (36 B) | `_MSG_CreateCharacter` slot 2 |
| recusa da seleção | `0d000bde df0f 0100 00000000 02` | `0x0FDF` + motivo `PinIncorreto` |

Estado do banco medido no mesmo teste:

| Passo | `accounts.pin_hash` | `characters` da conta |
|---|---|---|
| PIN cadastrado | hash Argon2id gravado | 0 linhas |
| PIN **errado** | **inalterado** (comparado byte a byte pelo teste) | **0 linhas** |
| PIN **correto** + criação | inalterado | 1 linha (`Pcpin291`) |
| seleção **sem** PIN | inalterado | continua 1 linha; `Pppin291` **não** existe |

Contador de falhas: o PIN errado grava `accounts.pin_failures`/`pin_locked_until` (lockout de 10
min, já coberto por `cinco_pins_errados_bloqueiam_a_conta` e `pin_persistencia`); o hash e o
personagem não mudam — é o que a tabela acima afirma.

## 3. Testes

| Teste | O que assere | Resultado |
|---|---|---|
| `sistema_login_canal::recusas_de_login_no_canal_real` (novo, canal vivo) | tríade de senha errada, idem conta inexistente, PIN errado (0x0FDF + painel + `pin_hash` e roster intactos), PIN correto (0x0FDE + `CNFNewCharacter` + linha no banco), seleção sem PIN recusada sem gravar | 1/1 |
| `sistema_login::senha_errada_e_recusada_sem_sessao` | consome `0x0117` → `0x0119` (motivo `Senha`) → `0x0101` 140 B e EOF | ok |
| `sistema_login::conta_inexistente_usa_o_mesmo_painel_de_senha_errada` | mesma tríade | ok |
| `sistema_login::selecao_sem_pin_em_conta_com_pin_e_recusada_e_nao_grava` (novo) | `0x0FDF` + painel "Senha numérica exigida." e roster igual antes/depois | ok |

Os dois testes antigos de senha errada estavam **vermelhos** (`left: 13, right: 140`): o servidor
já emitia três frames e o helper esperava o painel como segundo. Agora eles asserem a ordem medida,
e não a expectativa antiga.

`cargo test -p wyd-server --test sistema_login` com DSN completo (9 testes) é instável em paralelo
por disputa de porta/linha de canal no mesmo Postgres: em duas execuções completas uma falha
diferente apareceu (setup), e cada teste passa isolado. Rodado serial (`--test-threads=1`): **9
passed, 0 failed** em 79,76 s. Sem DSN esses testes fazem skip — por isso `test --workspace` (sem
DSN) é sempre verde. Isso é anterior a esta mudança e está registrado aqui para não ser confundido
com regressão.

Gates desta rodada: `cargo fmt --all --check` = 0, `cargo clippy --workspace --all-targets -- -D
warnings` = 0, `cargo test --workspace` = **863 passed, 0 failed**, `check-docs-consistency` = 0
(61 issues, 9 crates, 52 migrations, 270 docs, 408 links).

## 4. Imagem do canal

```
docker compose build server-canal-1 server-canal-2 && docker compose up -d server-canal-1 server-canal-2
docker inspect -> wyd-server-canal-1-1 criado=2026-11-14T21:40:06Z img=sha256:fce9efb3b177…, healthy
curl http://127.0.0.1:80/health -> {"ok":true,"channels":2,"schema":52}
```

O binário que respondeu todos os bytes acima é o desta imagem (build posterior à correção do PIN).

## 5. O que continua aberto

- Render do cliente 7662: não observável daqui. Os bytes de `0x0119` e `0x0101` são emitidos; se o
  build do jogador não desenhar, o próximo passo é capturar a tela dele com estes mesmos bytes.
- `0x0117` é emitido por paridade com o legado e ignorado pelo 7662 (RE acima): não é a via visível.
