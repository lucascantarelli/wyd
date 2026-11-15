# Painel de senha errada: mesma forma do caso que o jogador vê (2026-11-14)

> **Superado em parte (mesma data, rodada seguinte):** a comparação campo a campo do painel
> continua válida para o fluxo de exclusão, mas a sequência `[0x0119, 0x0101]` não é desenhada pela
> tela de login do 7662. A recusa passou a ser **um** `_MSG_AlreadyPlaying` (`0x011C`, 12 B); ver
> `login-tela-handler-2026-11-14.md`.

Pergunta: *por que a senha errada aparece na exclusão de personagem e não no login?* O servidor
já emitia `0x0117` + `0x0119` + `0x0101` no canal real; o que faltava era comparar esse painel com
o único fluxo que o jogador **confirma visível** e corrigir a diferença.

Medido com cliente TCP cru no canal 1 vivo (`127.0.0.1:8281`), sem GUI. O render do 7662 continua
não observável daqui: está provado o que o servidor emite, byte a byte.

## 1. Comparação campo a campo

Referência = exclusão de personagem com senha errada (`_MSG_DeleteCharacter`, fluxo cujo texto o
jogador vê). Alvo = login com senha errada / conta inexistente / conta bloqueada.

| Campo | Referência (delete) | Login **antes** | Login **depois** |
|---|---|---|---|
| frames antes do painel | 1: `0x011b` | 2: `0x0117`, `0x0119` | 1: `0x0119` |
| tamanho do `*Fail` | **12 B** (header-only) | 13 B (header + trailer) | **12 B** |
| `Size@0` do painel | 140 | 140 | 140 |
| `Type@4` | `0x0101` | `0x0101` | `0x0101` |
| `id@8` | 30000 (`ID_SCENE`) | 30000 | 30000 |
| texto | offset **12**, campo 128 B NUL-padded | igual | igual |
| depois do painel | conexão **aberta** (laço do canal) | FIN imediato | aberta (drena até 2 s) |

Três divergências, todas no servidor:

1. **frame morto**: `0x0117 NewAccountFail` não é comparado pelo roteador do 7662 (`DIV-01`) e era
   um frame a mais em relação ao caso visível;
2. **`*Fail` com trailer**: o caso visível manda 12 B; o login mandava 13 B;
3. **lifecycle**: o login fechava o soquete no instante do painel; o caso visível não fecha.

## 2. Bytes crus

### Referência visível (delete com senha errada)

```
c2s  size=44  hex=2c000b3f2fe7f5fd0aedccf933f118fd0a5f2c322d183030f2e40cfb1cf1f8fdd15602655b1ea96f6a52805e
s2c  size=12  hex=0c000bac1b01010000000000
     Size@0=12  Type@4=0x011b (DeleteCharacterFail)  id@8=1  tick@10=0
s2c  size=140 hex=8c000b5a010130750000000053656e686120696e636f72726574612e2054656e7465206e6f76616d656e74652e00…
     Size@0=140 Type@4=0x0101 id@8=30000 texto@12="Senha incorreta. Tente novamente." (128 B NUL-padded)
```

### Login falho — **antes** (imagem anterior, 21:40Z)

```
sequencia = 0x0117(13B), 0x0119(13B), 0x0101(140B)   -> FIN imediato
0x0117: 0d000bde1701010000000000 01   (trailer motivo 1)
0x0119: 0d000bde1901010000000000 01
0x0101: 8c000b5a0101307500000000 + "Senha ou conta incorreta" (140 B, texto@12)
```

### Login falho — **depois** (imagem 22:0xZ, três caminhos, bytes idênticos)

```
PASS(senha_errada):     CharacterLoginFail size=12 [0c000bac1901010000000000] | MessagePanel size=140
PASS(conta_bloqueada):  CharacterLoginFail size=12 [0c000bac1901010000000000] | MessagePanel size=140
PASS(conta_inexistente):CharacterLoginFail size=12 [0c000bac1901010000000000] | MessagePanel size=140

fail   size=12  hex=0c000bac1901010000000000            Size@0=12, Type@4=0x0119, id@8=1
painel size=140 hex=8c000b5a010130750000000053656e6861206f7520636f6e746120696e636f7272657461…
               Size@0=140, Type@4=0x0101, id@8=30000, texto@12="Senha ou conta incorreta"
```

Comparação executada pelo teste `sistema_painel_login_canal` (dump em
`tmp/painel-login/comparacao.txt`), que assere `Size`, `Type`, `id`, offset do texto, campo de 128 B
NUL-padded e **conexão aberta depois do painel** — nos dois fluxos. Resultado da execução depois da
correção: `PASS: painel igual em Size/Type/id/offset/pad; sequência login = [0x0119, 0x0101] (2
frames); conexão aberta depois do painel`.

O que **não** é observável daqui: se o 7662 desenha o painel na tela de login. A forma agora é a
mesma (bytes e lifecycle) do painel que ele desenha na seleção/exclusão; a diferença que resta é o
estado do cliente (tela de login × tela de seleção), fora do servidor.

## 3. Efeito colateral medido: alerta `WydPacotesRecusados`

A suíte de integração gera recusas legítimas (conta inexistente, senha errada). Medido com 4
execuções de `sistema_login_canal` seguidas, consultando o Prometheus:

| amostra | `rate(rejected[5m])` | `rate(packets[5m])` | razão |
|---|---|---|---|
| após 1 execução | 0,023/s | 0,054/s | 0,44 |
| após 4 execuções | **0,045/s** | 0,117/s | **0,38** |

Razão de 38% (acima dos 10% da regra) com volume de 0,045 recusa/s: a regra antiga alertava sem
incidente. `rate`/`ip`/`total` estavam em zero — nenhum cliente foi estrangulado; as recusas eram
`auth` (teste) e `timeout` (sessão de teste que abre, lê e sai).

Correção na regra (`docker/monitoring/prometheus/alerts.yml`): piso de volume antes da razão —

```
sum(rate(wyd_packets_rejected_total[5m])) > 0.5
and
sum(rate(wyd_packets_rejected_total[5m])) / clamp_min(sum(rate(wyd_packets_total[5m])), 0.001) > 0.1
```

Verificado: `promtool check rules` = SUCCESS (18 regras), `promtool test rules` = SUCCESS com dois
casos (volume alto dispara; 12 recusas/min sobre 60 pacotes/min **não** dispara), reload do
Prometheus vivo (`POST /-/reload` = 200) e a regra em execução passou a ter o piso
(`/api/v1/rules`), com `ALERTS{alertname="WydPacotesRecusados"}` **vazio** depois das execuções.

## 4. Testes e gates

| Teste | O que assere | Resultado |
|---|---|---|
| `sistema_painel_login_canal::painel_do_login_tem_a_mesma_forma_do_painel_da_exclusao` | painel do login idêntico ao da exclusão em `Size`/`Type`/`id`/offset/campo de 128 B; sequência `[0x0119 12 B, 0x0101 140 B]`; conexão aberta depois | 1/1 no canal vivo |
| `sistema_login_canal::recusas_de_login_no_canal_real` | senha errada, conta bloqueada e conta inexistente com os mesmos bytes; PIN inalterado | 1/1 no canal vivo |
| `sistema_login::senha_errada_e_recusada_sem_sessao`, `conta_inexistente_…`, `conta_bloqueada_…` | forma nova em processo | 9/9 serial |

Gates: `cargo fmt --all --check` = 0, `cargo clippy --workspace --all-targets -- -D warnings` = 0,
`cargo test --workspace` = 863 passed / 0 failed, `check-docs-consistency` = 0, `promtool check
rules`/`test rules` = 0.

## 5. Imagem

`docker compose build server-canal-1 server-canal-2` + `up -d` → `BUILD=0`, `UP=0`;
`docker inspect wyd-server-canal-1-1` saudável e `/health` `{"ok":true,"channels":2,"schema":52}`.
Os bytes da seção 2 ("depois") e das três recusas vieram dessa imagem.
