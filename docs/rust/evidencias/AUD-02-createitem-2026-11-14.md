# AUD-02 — direção do `_MSG_CreateItem` (0x026E) corrigida no catálogo

**Data:** 2026-11-14 · **Achado:** `docs/audit/15-bugs-and-divergences.md` `DIV-02` (P1, **Confirmado**) ·
**Issue:** `AUD-02` · **Bloqueava:** `GAME-11` (handler de item no chão)

## O defeito (confirmado no código antes da correção)

`crates/wyd-protocol/src/wire/generated/opcode_table.rs` declarava
`Opcode::CreateItem => &[Flow::Client2Game]`. A evidência de binário registrada em
[`docs/cliente/pacotes.md`](../../cliente/pacotes.md) §“Direção — o `Basedef.h` erra no
`CreateItem`” mostra o oposto: o dispatcher de recebimento do 7662 compara `0x026e`
(`0x495dc5 cmp edx,0x26e` → `call 0x486c25`), ou seja, a mensagem **chega** ao cliente (S→C).
O `Basedef.h` marca `FLAG_CLIENT2GAME` (bit `0x0200`), e era essa flag que a tabela congelada
repetia.

O risco concreto: `GAME-11` implementaria o handler de item no chão na direção errada, e o
dump de captura classificaria `0x026E` como pacote enviado pelo cliente.

## A correção (ponto único)

| Arquivo | Mudança |
|---|---|
| `crates/wyd-protocol/src/wire/generated/opcode_table.rs` | `Opcode::CreateItem => &[Flow::Game2Client]`, com o valor de fio **intocado** (`0x026e`) e comentário citando o VA |
| `tools/protocol/gen-protocol.mjs` | `DIRECTION_OVERRIDES` aplica a mesma correção ao gerar tabela e catálogo (uma regeneração futura não reintroduz a direção do `Basedef.h`) |
| `docs/arquitetura/protocolo.md` §4 | catálogo canônico: `_MSG_CreateItem 0x026E` agora `J→C` |
| `docs/cliente/pacotes.md` | a consequência “não foi feito nesta entrega” virou o registro da correção |

O valor de fio **não muda**: o cliente casa o `Type` inteiro (`0x026E`), não o bit de direção.
Quem consome `flows()` (ferramentas de captura e o futuro handler) passa a ver S→C.

## Teste que prende o comportamento

`crates/wyd-protocol/src/lib.rs::tests::create_item_e_recebido_pelo_cliente_com_o_mesmo_valor_de_fio`

```rust
assert_eq!(Opcode::CreateItem.wire(), 0x026e);
assert_eq!(Opcode::CreateItem.flows(), &[Flow::Game2Client]);
assert_eq!(Opcode::CreateItem.const_name(), "_MSG_CreateItem");
```

## Prova por mutação

Reintroduzido o defeito (`&[Flow::Client2Game]`) no mesmo ponto:

```
$ cargo test -p wyd-protocol --lib create_item_e_recebido_pelo_cliente_com_o_mesmo_valor_de_fio
test tests::create_item_e_recebido_pelo_cliente_com_o_mesmo_valor_de_fio ... FAILED
thread 'tests::create_item_e_recebido_pelo_cliente_com_o_mesmo_valor_de_fio' panicked at crates\wyd-protocol\src\lib.rs:127:9:
assertion `left == right` failed
  left: [Client2Game]
 right: [Game2Client]
test result: FAILED. 0 passed; 1 failed; ...
```

Restaurado o `Flow::Game2Client`:

```
$ cargo test -p wyd-protocol
test result: ok. 45 passed; 0 failed; 0 ignored; ...   (lib)
test result: ok. 10 passed; 0 failed; 0 ignored; ...   (golden)
test result: ok. 4 passed; 0 failed; 0 ignored; ...    (integração)
```

`grep -rn MUTANTE crates` → **0**.

## Verificação completa desta entrega

- `cargo test --workspace --all-targets` → **851 passed, 0 failed, 1 ignored** em 76 alvos com
  resultado, exit 0 (`tmp/test-aud02.log`). O `#[ignore]` é o teste de rede do launcher.
- `cargo fmt --all --check` → 0; `cargo clippy --workspace --all-targets -- -D warnings` → 0
  (só o aviso pré-existente de dependência `tracing` não usada no manifesto do `wyd-core`).
- Suíte de sistema com Postgres real (`WYD_TEST_REQUIRED=1`, `wyd_auditoria`) → **300 passed,
  0 failed, 0 ignored** em 40 alvos, 7 SKIPs só de `WYD_LIVE_CPSOCK` (canal vivo), exit 0
  (`tmp/sistema-aud02.log`).
- `node tools/content/check-issue-status.mjs` → **57/57** issues ativas, 0 divergências, exit 0.
- `node tools/dev/check-docs-consistency.mjs` → **57 issues, 9 crates, 10 membros, 50 migrations,
  255 docs, 393 links — consistente**, exit 0.
- `node --test apps/tools/test/*.test.mjs` (ferramentas de dev, entra no `make check`) → exit 0.

## O que fica aberto

- O **layout** de 30 B do `CreateItem` continua sem round-trip no fio: emissão e handler ficam
  com `GAME-11`; a captura ao vivo (`CLIENT-06`) segue como prova que falta.
- Nenhuma direção mudou além de `0x026E`; `DecayItem`, `CNFGetItem` e `CNFDropItem` já estavam
  S→C e `GetItem`/`DropItem` já estavam C→S na tabela (conferido contra `docs/cliente/pacotes.md`).
