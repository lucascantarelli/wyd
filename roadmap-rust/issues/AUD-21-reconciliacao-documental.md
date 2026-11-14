# AUD-21 — Reconciliação documental e achados sem issue (`DIV-03/07/08/09/12/14`, `N-*`)

> **ID canônico:** `AUD-21`
> **ID histórico:** `AUD-21` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** 🚧 · ver `roadmap-rust/index.md`.

**Tipo:** Documentação / higiene · **Esforço:** M · **Depende de:** — · **Bloqueia:** leitura correta do roadmap e do checklist

**Origem:** `docs/audit/15-bugs-and-divergences.md` §1 e §3 (itens sem issue) e `docs/audit/17-priorities-and-roadmap.md` §6.

## Objetivo

Fazer a documentação normativa dizer a verdade sobre o estado atual, fechando os achados que a
auditoria listou sem issue. Cada item tem uma ação objetiva; nada aqui é código de gameplay.

## Itens

| ID | Achado | Ação | Estado |
|---|---|---|---|
| `DIV-03` | `CharacterLogin` tem 36 B no 7662 × 20 B no `Basedef.h` | documentar o tamanho real no catálogo de pacotes | ✅ feito em 2026-11-14 (`docs/cliente/pacotes.md` já registra 36 B e o teste de fio cobre) |
| `DIV-07` | `item_catalog` (3.220) × `ItemList.bin` (6.500 posições) | regra “cruzar por índice” escrita no doc de itens | ⬜ |
| `DIV-08` | `analise-portabilidade-legado.md` diz 7 crates / `wyd-net` 458 linhas | atualizar para 9 crates e a contagem medida | ✅ feito em 2026-11-14 |
| `DIV-09` | checklist 7.2 = ⬜, mas `tests/sistema/loja_viva.rs` existe | marcar o checklist com o estado real e citar o teste | ✅ feito em 2026-11-14 |
| `DIV-12` | `ItemList` 140 B / `SkillData` 96 B neste build | registrar no dossiê (não copiar layout de 7.69+) | ⬜ |
| `DIV-14` | “Familiares” não é sistema oficial do 7662 | marcar `docs/wyd/sistemas/familiares.md` como sistema nosso | ⬜ |
| `N-1` | erro de *render target* degrada efeito | runbook de GPU/Windows + `CLIENT-12` | ⬜ |
| `N-2` | `client/` é repack de comunidade | declarar proveniência no dossiê | ⬜ |
| `N-3` | `extraitem.bin` 106 B é stub | nota no doc de formatos antes de usar | ⬜ |
| `N-6` | checklist 1.11 ✅ com resíduo visual | marcar 🟡 e apontar `GAME-17` | ✅ feito em 2026-11-14 |
| `N-8` | `client/` sem `Graphics.ini` | setup gráfico no dossiê | ⬜ |
| `N-9` | 124 `unwrap()` em `src` | triagem por caminho quente/rede (`docs/audit/14` §7) | ⬜ |

> Já cobertos por issue própria: `DIV-01`→`BUG-01`, `DIV-02`→`AUD-02` (fechada), `DIV-04`→`CLIENT-10`,
> `DIV-10`→`AUD-04`, `DIV-11`→`GAME-20`, `N-4`/`N-5`→`AUD-05`, `N-7`→`GAME-05`, `N-10`→`UNK-23` (`AUD-19`),
> `PAR-MOV-02`→`AUD-04`.

## Critérios de aceite

- [ ] Nenhum documento normativo cita número, crate, coluna ou teste inexistente.
- [ ] Todo item acima fechado com o arquivo alterado citado na coluna.
- [ ] `check-docs-consistency` em 0 e links válidos.

## Testes exigidos

- Sem teste de código: a prova é o link/arquivo alterado e o checker de docs.
- Onde a correção tocar um número medido, anexar o comando que o produziu.

## Definição de pronto

- `docs/audit/15` e `18` atualizados na mesma entrega; índice e painel com o mesmo status.
