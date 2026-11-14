# AUD-19 — Fechar as incógnitas da auditoria (`UNK-01`…`UNK-37`)

> **ID canônico:** `AUD-19`
> **ID histórico:** `AUD-19` (aberta em 2026-11-14, origem na auditoria)
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Investigação / conhecimento · **Esforço:** XL (decompor por área) · **Depende de:** `CLIENT-06` para as incógnitas de fio · **Bloqueia:** `AUD-03`, `AUD-09`, `GAME-11`, `AUD-17`

**Origem:** `docs/audit/18-unknown-features.md` (37 incógnitas) e `docs/audit/17-priorities-and-roadmap.md` Fase 0.

## Objetivo

Dar dono, método e critério de fechamento a **cada** incógnita registrada pela auditoria, para que
nenhuma pergunta do tipo “não sabemos” fique sem issue rastreável. Fechar uma incógnita significa
publicar a resposta medida (não a hipótese) em `docs/cliente/` ou `docs/arquitetura/`, com o teste
ou a captura anexada.

## Contexto

- A auditoria listou 37 perguntas abertas e não abriu issue para nenhuma: o roadmap só tinha as
  entregas do §17. Sem esta issue, um leitor do audit não encontra trabalho correspondente.
- As que dependem do cliente 7662 continuam bloqueadas por `CLIENT-06`; as outras (documento,
  binário local, medição no repo) podem começar hoje.

## Itens e método

| Grupo | IDs | Método de fechamento | Fase |
|---|---|---|---|
| Fonte do legado | `UNK-01` | `git fetch` + `git diff --stat origin/main` comparado com o remoto `w2pp-OpenWYD`; registrar hash e veredito | 0 |
| Fio do cliente | `UNK-02`, `UNK-03`, `UNK-04`, `UNK-05`, `UNK-06`, `UNK-09`, `UNK-13` | captura bidirecional (`CLIENT-06`) + golden de frame no `wyd-protocol` | 0 |
| Opcode ignorado | `UNK-12` | já provado por desassemblage (`0x0117` fora do roteador do 7662); falta só captura mostrando a recusa — fecha junto com `BUG-01` | 0 |
| Layouts de item/trade/texto | `UNK-07`, `UNK-08`, `UNK-11` | frame real + golden; cada um destrava uma issue (`GAME-11`, `CLIENT-10`, `BUG-01`) | 0 |
| Catálogo do cliente | `UNK-10`, `UNK-14`, `UNK-15`, `UNK-19`, `UNK-20`, `UNK-22` | leitura binária/RE do loader + tabela medida em `docs/cliente/formatos.md` | 1 |
| Mapas e colisão | `UNK-17`, `UNK-18`, `UNK-21` | leitura de `Field*.dat/.trn`, `AttributeMap.dat` e teste de visão com NPC a N células | 1 |
| Familiar e cidadania | `UNK-16`, `UNK-31` | UI + struct do cliente; diálogo de NPC | 2 |
| Integridade e rede | `UNK-23`, `UNK-24` | teste de dois saves concorrentes e auditoria dirigida de `.ok()`/`let _ =` em caminho de rede | 2 |
| Fórmulas e economia | `UNK-25`, `UNK-26`, `UNK-27`, `UNK-28`, `UNK-29`, `UNK-30`, `UNK-32` | golden comparado com o legado + decisão registrada (ver `AUD-20`) | 2 |
| Ambiente gráfico | `UNK-33`, `UNK-34`, `UNK-35`, `UNK-36`, `UNK-37` | ProcExp/ProcMon, troca de GPU/modo de vídeo, wayback; `UNK-37` já decidido como “não pesquisar” | 4 |

## Critérios de aceite

- [ ] Cada `UNK-NN` tem veredito publicado (medido, com arquivo/comando) ou “não investigar” com motivo.
- [ ] Nenhuma incógnita é fechada por leitura de documento do próprio audit: exige medição nova.
- [ ] `UNK-18` (dimensão de `AttributeMap.dat`) destrava `AUD-04` e é entregue primeiro.
- [ ] O resultado atualiza `docs/audit/18-unknown-features.md` na mesma entrega.

## Testes exigidos

- Um teste ou script por incógnita fechada; para captura, o frame + hash anexado em
  `docs/rust/evidencias/`.
- Checker `node tools/content/check-audit-coverage.mjs` continua em 0 depois de cada fechamento.

## Definição de pronto

- Issue, índice e painel com o mesmo status; evidência referenciada por caminho real.
- `make check` verde e `check-issue-status` sem divergência.
