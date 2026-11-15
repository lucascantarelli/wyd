# Criação de personagem: confirmação da segunda senha e nível de fronteira (2026-11-14)

Duas perguntas do relato: *"segunda senha sem validação, aceita qualquer coisa"* e *"personagem
ainda é criado nível 2 (cliente 0-based, banco 1-based)"*. Medido no canal 1 vivo
(`127.0.0.1:8281`), imagem reconstruída, banco lido na mesma execução.

## 1. Segunda senha na criação — a raiz era o braço "late" do `0x0FDE`

**Causa raiz medida.** `cpsock/action.rs`, braço `Opcode::AccountSecure` da etapa de seleção (o
`_MSG_AccountSecure` que o cliente manda para **confirmar** a senha numérica antes de criar):

```rust
// antes
Opcode::AccountSecure => {
    log("info", "pin", &format!("result=ok account={account} mode=late"));
    reply(stream, key_word, Opcode::AccountSecure)   // responde OK sem comparar o token
}
```

Nenhuma comparação com `accounts.pin_hash`: qualquer token — inclusive uma confirmação divergente —
recebia `0x0FDE` e a criação seguia. Era o "aceita qualquer coisa" no caminho de criação.

**Correção.** Esse braço passa pelo mesmo `Authenticator::secure_pin` do login (a conta recebe o
primeiro PIN quando ainda não tem). Divergência: `_MSG_AccountSecureFail` (`0x0FDF`, motivo
`PinIncorreto`) + `_MSG_MessagePanel` (140 B, texto ASCII) e a conexão **encerra** — o
`_MSG_CreateCharacter` seguinte não chega a ser processado, então nada é gravado. Para isso o
autenticador passou a ser entregue ao laço de seleção (`Sessao { account, auth }`).

Bytes medidos (conta nova, PIN `2468` cadastrado, confirmação enviada como `0000`):

```
c2s  size=32  hex=20000b31fcf4f5fd08edccf96221482dbaf1f4fdfae4f8fcf2e40cfb1cf1f8fd
s2c  size=13  hex=0d000bdedf0f01000000000002      Size@0=13 Type@4=0x0FDF motivo@12=0x02
s2c  size=140 hex=8c000b5a0101307500000000 + "As senhas numericas nao conferem."
     Size@0=140 Type@4=0x0101 id@8=30000 texto@12 (128 B NUL-padded)
roster da conta depois = 0 linhas
```

E com a confirmação correta (mesmo PIN), a criação conclui e grava:

```
s2c  size=856 hex=58030bbb100131750000000000000000000000003008...
     Type@4=0x0110 (CNFNewCharacter), sel@4, Score[0].Level@96 = 0
banco: characters.level = 1
```

Texto ASCII de propósito: o campo do `0x0101` tem 128 B e o painel que o cliente comprovadamente
desenha ("Senha incorreta. Tente novamente.", na exclusão) não tem acento — UTF-8 multibyte num
cliente Windows vira mojibake. (O painel de PIN do login, "Senha numérica exigida.", é anterior a
esta rodada e não foi tocado.)

## 2. Nível: a proposição do usuário, medida nas duas pontas

Proposição: *o cliente é 0-based e o banco 1-based*. Medido — o banco é **1-based** e o fio é
**0-based** (`wire_level = banco − 1`); o cliente desenha `fio + 1`, então a tela mostra exatamente
o valor do banco. Não há +1/−1 espalhado: a conversão tem um ponto único,
`character/snapshot.rs::wire_level`, usado por `Selchar` (roster/login/criação) e por `Mob`
(`CNFCharacterLogin`, `CreateMob` — inclusive NPC, via `world/scene.rs`).

| banco (`characters.level`) | fio medido | onde |
|---|---|---|
| **1** (criação real pelo protocolo) | **0** | `CNFNewCharacter` (0x0110), `Level@96` |
| **2** | **1** | `CNFAccountLogin` (0x010A), roster `sel@20`, `Score[2]@80` |
| **399** | **398** | idem, `Score[1]`; `CNFCharacterLogin` `BaseScore@60`/`CurrentScore@108` |
| **400** | **399** | roster `Score[0]` e `CNFCharacterLogin` do slot 0 |

Saída medida do canal (um login, três personagens de borda `banco=[400,399,2]`):

```
PASS(nivel_bordas): roster fio=[399, 398, 1] banco=[400, 399, 2] |
  CNFCharacterLogin BaseScore@60=399 CurrentScore@108=399
```

Ou seja: banco 1 → **tela 1**, banco 399 → **tela 399**, banco 400 → **tela 400**. O único +1 é o
do cliente, aplicado uma vez sobre o fio. Nenhum ajuste novo foi necessário no nível nesta rodada:
o valor do banco e o valor desenhado coincidem nas quatro fronteiras medidas (1, 2, 399, 400).

## 3. Testes e gates

| Teste | O que assere | Resultado |
|---|---|---|
| `sistema_criacao_canal::criacao_por_protocolo_confere_pin_e_nivel` (novo, canal vivo) | confirmação divergente → `0x0FDF` (motivo 2) + painel 140 B e roster 0; confirmação igual → `CNFNewCharacter` 856 B com `Level@96 = 0` e `characters.level = 1`; bordas 400/399/2 → fio 399/398/1 e `BaseScore`/`CurrentScore` 399 | 1/1 |
| `sistema_login_canal`, `sistema_painel_login_canal`, `sistema_nivel_wire` | pernas anteriores seguem verdes no mesmo binário | 1/1, 1/1, 1/1 |

`cargo fmt --all --check` = 0, `cargo clippy --workspace --all-targets -- -D warnings` = 0,
`cargo test --workspace` = 865 passed / 0 failed, `check-docs-consistency` = 0.
`ALERTS{alertname="WydPacotesRecusados"}` vazio durante a verificação (piso de 0,5 recusa/s em
vigor).

## 4. Imagem

`docker compose build server-canal-1 server-canal-2` + `up -d` → `BUILD=0`, `UP=0`;
`docker inspect` → containers criados em **2026-11-14T22:40:39Z**, healthy; `/health`
`{"ok":true,"channels":2,"schema":52}`. Todos os bytes desta página vieram dessa imagem.

## 5. O que continua fora de alcance

O desenho na tela do 7662 não é observável daqui: está provado o que o servidor emite (bytes e
banco). O `+1` que o cliente aplica ao nível é o que a proposição do usuário descreve e o que a
medição do fio permite afirmar; a tela em si depende do cliente.
