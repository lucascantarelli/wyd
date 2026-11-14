# Cobertura do audit e status provado — 2026-11-14

> **Issues:** `AUD-19` (incógnitas), `AUD-20` (decisões de modelo), `AUD-21` (reconciliação
> documental), `AUD-22` (decisão 4.5)
> **Escopo:** fechar o eixo de alinhamento das issues ao roadmap proposto pela auditoria, com
> status conferido por execução — não por leitura do audit.

## 1. O que faltava

`docs/audit/17-priorities-and-roadmap.md` §2–§6 tem 33 entregas, mas os outros documentos do audit
abrem itens que não tinham issue nenhuma:

| Origem | Itens | Antes |
|---|---|---|
| `18-unknown-features.md` | 37 (`UNK-01`…`UNK-37`) | nenhuma issue |
| `15-bugs-and-divergences.md` §1 | `DIV-03`, `DIV-05`…`DIV-09`, `DIV-12`…`DIV-14` | nenhuma issue (só prosa) |
| `15-bugs-and-divergences.md` §3 | `N-1`…`N-10` | 4 com issue, 6 sem |
| `17` §6 | entrega 4.5 (cliente independente) | “decisão pós-paridade”, sem dono |

## 2. O que foi criado

| Issue | Conteúdo | Fase no roadmap da auditoria | Status |
|---|---|---|---|
| `AUD-19` | tabela por grupo cobrindo cada `UNK-NN`, com método de fechamento e fase | 0 (conhecimento) | ⬜ |
| `AUD-20` | decisão de `DIV-05` (drop), `DIV-06` (piedade) e `DIV-13` (PK), com ADR e simulação | 2 (itens/economia) | ⬜ |
| `AUD-21` | `DIV-03/07/08/09/12/14` + `N-1…N-10` restantes, item por item | 1 (loop fiel, documental) | 🚧 |
| `AUD-22` | item 4.5: decisão go/no-go de cliente independente | 4 (expansão) | ⛔ |

O índice ganhou a tabela **“Cobertura item a item do §17”** (33 linhas, `entrega → issue`) e a
seção “Cobertura do audit além do §17”. Nada ficou órfão: o índice foi de 57 para **61 issues
ativas** e o painel acompanha as quatro novas linhas.

## 3. Gate novo (prova contínua)

`tools/content/check-audit-coverage.mjs`:

1. lê os IDs de `docs/audit/18` (37) e `docs/audit/15` (14 `DIV` + 10 `N-*`);
2. exige que cada ID apareça em alguma issue de `roadmap-rust/issues/`;
3. exige que cada uma das 33 entregas do §17 tenha linha na tabela de cobertura do índice e que
   a issue dona exista (arquivo ativo **ou** registro no changelog, para issues fechadas).

Saída observada:

```text
$ node tools/content/check-audit-coverage.mjs
audit-coverage: 37 UNK, 14 DIV, 10 N-*, 33 entregas do §17
ok: todo item da auditoria tem issue rastreável
```

Está no `make check` (alvo `audit-coverage-check`), então a deriva futura quebra o gate.

## 4. Conferência de status por execução

- As 61 issues ativas não têm nenhuma marcada ✅ além de `AUD-02` e `AUD-18`, ambas fechadas com
  mutação RED→GREEN documentada (ver `AUD-02-createitem-2026-11-14.md` e
  `auditoria-status-honestidade-2026-11-14.md`).
- As 20 issues rebaixadas para 🟢 continuam 🟢 no painel e no índice — o checker de status exige que
  os dois digam a mesma coisa.
- Varredura mecânica das evidências citadas em `docs/rust/03-progresso.md`,
  `roadmap-rust/changelog.md`, `roadmap-rust/index.md` e `docs/rust/08-checklist-funcionalidades.md`:
  113 identificadores entre crases com ≥2 `_`; 29 não aparecem em `crates/**/*.rs`. Dos 29, 28 são
  legítimos fora do código Rust (nomes de alvo de teste em `Cargo.toml` como
  `sistema_personagens_crud`, tabelas/colunas SQL como `world_item_seed` e `per_level_value`,
  o syscall `io_uring_setup`, o banco `wyd_test_login` e o rótulo sintético `prova_online_c2`).
  **Um era defeito real:** `primeira_entrada_cadastra_o_pin` era citado como teste de
  `tests/sistema/login.rs`, onde o nome correto é
  `primeira_entrada_sem_pin_vai_direto_para_a_selecao` — corrigido no checklist (item 1.6).

## 5. Defeitos pequenos corrigidos na mesma passada

| Defeito | Onde | Correção |
|---|---|---|
| `CharacterLogin` “20 B” sem nota do build | `docs/cliente/pacotes.md` | já registrava `20 (cliente 7662: 36)`; conferido contra `WIRE_LEN = 36` em `crates/wyd-protocol/src/payload/character.rs` (`DIV-03` fechado) |
| Portabilidade com 7 crates / 18.242 linhas | `docs/manual-técnico/analise-portabilidade-legado.md` | medido: **9 crates, 203 arquivos, 57.841 linhas, 65 arquivos de teste**; tabela por crate refeita (`DIV-08`) |
| Checklist 7.2 “lojas de NPC” = ⬜ | `docs/rust/08-checklist-funcionalidades.md` | 🟡 com `tests/sistema/loja_viva.rs` e ponteiro para `AUD-07` (`DIV-09`) |
| Checklist 1.11 ✅ com resíduo visual | idem | 🟡 com o resíduo explícito (`GAME-17`/`CLIENT-12`) (`N-6`) |
| Cabeçalho de tabela duplicado no registro de execução | `docs/rust/03-progresso.md` | removido (a tabela voltou a ser uma só) |
| `AUD-04` apontava `tests/integracao/autoridade.rs` sem crate | `roadmap-rust/issues/AUD-04-...` | caminho real `crates/wyd-core/tests/integracao/autoridade.rs` |

## 6. Medições do turno

```text
cargo test --workspace --all-targets      -> 851 passed, 0 failed, 1 ignored, exit 0
cargo fmt --all --check                   -> exit 0
cargo clippy --workspace --all-targets -- -D warnings -> exit 0 (1 aviso pré-existente no manifesto do wyd-core)
node tools/content/check-issue-status.mjs -> 61/61 issues, 0 divergências
node tools/content/check-audit-coverage.mjs -> 37 UNK, 14 DIV, 10 N-*, 33 entregas, 0 órfãos
node tools/dev/check-docs-consistency.mjs -> 61 issues, 9 crates, 50 migrations, 262 docs, 402 links
```

## 7. O que continua aberto

- `AUD-19` depende de `CLIENT-06` para 7 incógnitas de fio; as outras podem começar hoje.
- `AUD-20` espera o laboratório de simulação (`AUD-10`) para comparar curvas com número.
- `AUD-22` é decisão de produto: só o dono fecha.
- Nenhum passo de GUI do `WYD.exe` foi executado; `QA-12`/`QA-13` seguem manuais.
