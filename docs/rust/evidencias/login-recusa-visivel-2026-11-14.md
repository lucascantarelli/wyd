# Login recusado: forma medida no canal real (2026-11-14)

> **Superado em parte (mesma data, rodada seguinte):** os bytes abaixo descrevem a recusa como
> `0x0119` + painel. O RE do cliente mostrou que a tela de login não compara `0x0119` nem desenha
> o texto do `0x0101`; a recusa passou a ser **um** `_MSG_AlreadyPlaying` (`0x011C`, 12 B), o
> único braço que chega à rotina de mensagem `0x404109` naquela etapa. Ver
> `login-tela-handler-2026-11-14.md`.

Item do relato: *"usuário/senha errada não informa no cliente, não aparece nada"*. Esta rodada
fecha o lado do servidor com bytes medidos, estado de banco e o lifecycle da conexão — e declara
o que continua fora de alcance (o desenho na tela do 7662).

## 1. O que o caminho visível faz — e o que o login fazia

Referência de "visível" confirmada pelo jogador: **senha errada na exclusão de personagem**. Ela
manda **um** `*Fail` header-only (12 B, `0x011b`) + `_MSG_MessagePanel` (140 B, `0x0101`,
`id=30000`, texto@12) e **não fecha a conexão**: o laço do canal segue atendendo.

O login recusado tinha três diferenças, todas no servidor, e as três foram eliminadas:

| # | antes | depois (medido) |
|---|---|---|
| 1 | frame morto `0x0117` (não comparado pelo 7662 — `DIV-01`) | fora do fio |
| 2 | `*Fail` com trailer (13 B) | header-only, **12 B** (`0x0119`) |
| 3 | **FIN imediato** depois do painel (2 s) | conexão mantida aberta até 30 s; sem FIN no instante do painel |

O `FIN` é o candidato mais forte para "não aparece nada": é exatamente o que mata a janela de erro
do cliente antes de ela ser desenhada, e é o que o caso visível nunca faz.

## 2. Bytes medidos (canal 1 vivo, imagem de 2026-11-14T23:01:10Z, healthy)

Senha errada (conta existente), conta bloqueada e conta inexistente devolvem **os mesmos bytes**:

```
c2s  size=116 hex=74000bff2be7f5fd07edccf9…            _MSG_AccountLogin
s2c  size=12  hex=0c000bac1901010000000000
     Size@0=12  key@2=0x0b  checksum@3=0xac  Type@4=0x0119  id@8=1  tick@10=0
s2c  size=140 hex=8c000b5a0101307500000000 + "Senha ou conta incorreta"
     Size@0=140 Type@4=0x0101 id@8=30000 texto@12 (128 B NUL-padded)
depois do painel: conexão aberta, sem bytes novos, sem EOF (medido 1,2 s)
```

Saída dos testes no binário acima:

```
PASS(senha_errada):     CharacterLoginFail size=12 [0c000bac1901010000000000] | MessagePanel size=140 texto="Senha ou conta incorreta"
PASS(conta_bloqueada):  CharacterLoginFail size=12 [0c000bac1901010000000000] | MessagePanel size=140 texto="Senha ou conta incorreta"
PASS(conta_inexistente):CharacterLoginFail size=12 [0c000bac1901010000000000] | MessagePanel size=140 texto="Senha ou conta incorreta"
```

## 3. Nada é autenticado nem gravado

| conta (prefixo do teste) | caminho | `character_session` | `characters` |
|---|---|---|---|
| `qasenha32302000`, `qasenha96100800`, `qasenha07431300` | login com **senha errada** | **0 linhas** | **0** |
| `qanada…` | login com **usuário inexistente** | conta nem existe | — |
| `qapin62760200`, `qacr595700`, `qacr373900` | login **correto** + PIN | **2** sessões | 1 personagem |

Consulta usada:
`select a.login, (select count(*) from character_session s where s.account_login=a.login),
(select count(*) from characters c where c.account_id=a.id and c.deleted_at is null) from accounts a
where a.login like 'qasenha%' or …`.

Além disso os testes de canal asserem que o **primeiro** frame da recusa é `0x0119` — nunca
`CNFAccountLogin` (0x010A) —, ou seja o roster não é enviado, e que o `0x0119`+painel saem com a
conexão ainda aberta.

## 4. Controle: login correto continua entrando com o roster do banco

`PASS(pin_correto): AccountSecure size=12 [0c000bacde0f010000000000] | CNFNewCharacter size=856 | banco=1`
e, no teste de painel: `CNFAccountLogin` de **2008 B** com o `Selchar` do roster
(`sel@20`, `Score[0].Level@100`), personagem selecionado devolvendo `CNFCharacterLogin` de 1832 B.

## 5. Regressão de PIN na criação (não regrediu)

```
PASS(pin_divergente): AccountSecureFail 13 [0d000bdedf0f01000000000002] | MessagePanel 140 | roster=0
PASS(pin_igual): CNFNewCharacter 856 B Level@96=0 | banco level=1
PASS(nivel_bordas): roster fio=[399, 398, 1] banco=[400, 399, 2] | BaseScore@60=399 CurrentScore@108=399
```

## 6. Testes e gates

| Teste | Resultado |
|---|---|
| `sistema_login_canal::recusas_de_login_no_canal_real` (senha errada, bloqueada, inexistente, PIN errado, PIN correto, seleção sem PIN) | 1/1 |
| `sistema_painel_login_canal::painel_do_login_tem_a_mesma_forma_do_painel_da_exclusao` | 1/1 |
| `sistema_criacao_canal::criacao_por_protocolo_confere_pin_e_nivel` | 1/1 |
| `sistema_login` (in-process, 9 testes) | 9/9 serial |

`cargo fmt --all --check` = 0, `cargo clippy --workspace --all-targets -- -D warnings` = 0,
`cargo test --workspace` = 865 passed / 0 failed, `check-docs-consistency` = 0,
`ALERTS{alertname="WydPacotesRecusados"}` vazio.

## 7. Imagem

`docker compose build server-canal-1 server-canal-2` + `up -d` → `BUILD=0`, `UP=0`;
`docker inspect` → containers criados em **2026-11-14T23:01:10Z**, healthy; `/health`
`{"ok":true,"channels":2,"schema":52}`. Todos os bytes acima vieram dessa imagem.

## 8. O que continua fora de alcance — e o que foi eliminado do lado do servidor

Não é observável daqui o desenho na tela do 7662. O que se pode afirmar com a medição:

- o painel sai **byte a byte igual** ao do caso que o jogador vê, precedido do mesmo tipo de frame
  (`*Fail` header-only) e **com a conexão aberta**, sem FIN;
- não há autenticação nem escrita no banco;
- o RE do binário (`docs/cliente/pacotes.md`) registra que `0x0119/0x011a/0x011b` "cada um exibe
  mensagem própria do cliente" — o `0x0119` que sai aqui é justamente um deles.

Se, com esta forma, a tela do 7662 continuar muda, o que resta é o estado do cliente na tela de
login (fora do servidor): os bytes, o lifecycle e a ausência de escrita já estão provados.
