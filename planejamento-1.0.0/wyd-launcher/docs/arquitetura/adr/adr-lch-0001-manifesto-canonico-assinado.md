# ADR-LCH-0001 — Manifesto canônico com assinatura Ed25519 destacada

> Pergunta: "qual é o formato do manifesto e como o launcher prova que ele veio de quem diz vir?"
> Público: Arquitetura, desenvolvimento do launcher e operação de release
> Dono: equipe do `wyd-launcher` (decisão do dono em 2026-11-23)
> Base: `docs/proposta-decisoes-launcher.md` §4 (`Q-LCH-1`, `Q-LCH-3`); `lacunas.md` `LLCH-01`, `LLCH-03`; `wyd-launcher-1.0.0.md` §3 e §6; contrato `X11` em `wyd-1.0.0.md` §7.3
> Revisado em: 2026-11-25

**Estado:** aceito (2026-11-23) — aprovado pelo dono em `Q-LCH-1` = A e `Q-LCH-3` = A.

## Contexto

`X11` exige "manifesto com hash por arquivo" e estava aberto em duas lacunas que a decisão fechou:
`LLCH-01` (formato) e `LLCH-03` (assinatura). **Medido em 2026-11-23:** não havia manifesto, schema,
versão, assinatura, chave nem verificador no repositório; o registro de publicação, porém, já existia —
`portal.launcher_release` guarda `channel` (`stable`/`beta`), `version` semver, `manifest_url`,
`manifest_sha256`, `min_version` e status (`wyd-db/migrations/0030_portal_content.md:172–203`).

O risco declarado é execução de código arbitrário por patch adulterado: `sha256` detecta corrupção, mas
não autentica a origem. Sem formato canônico, uma assinatura sobre JSON reordenado não verifica.

## Decisão

O manifesto é **JSON versionado e canônico** (UTF-8, canonização **RFC 8785**) com os campos `schema`,
`channel`, `version`, `client_build`, `min_version` e `files[{path, size, sha256, url}]`, e leva
**assinatura Ed25519 destacada** — arquivo irmão `manifest.json.sig` — verificada **antes de aplicar**
qualquer arquivo. O launcher embute a **chave pública** e o `key_id`; a **chave privada** fica só no
secret do `wyd-ops`.

## Consequências

- **Positivas:** manifesto auditável arquivo a arquivo; a assinatura cobre bytes estáveis; a origem do
  patch é autenticada antes de o cliente rodar.
- **Negativas/custos:** exige schema, canonização, assinador, verificador, fixture e cerimônia de chave;
  a rotação de chave exige novo launcher com a nova pública (ou lista de chaves), sem migration.
- **Contratos afetados:** o `wyd-ops` assina e publica (`release.yml` futuro, `/admin/launcher`);
  o `wyd-client` produz o patch e o hash; `portal.launcher_release` continua sendo o registro de
  publicação, sem coluna nova (`manifest_sha256` já existe).

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Binário compacto | exige codec e ferramenta de inspeção; não há ganho de tamanho que pague a auditabilidade |
| Reutilizar o manifesto do legado | não tem canal, `min_version`, URL nem assinatura; `deprecated/` não se copia (`AGENTS.md` §5, item 2) |
| Só `sha256`, sem assinatura | detecta corrupção, não autentica a origem; acrescentar assinatura depois muda formato, launcher e release |

## Como verificar

A decisão está em vigor quando existirem, e o gate os cobrar, **1 página** `docs/specs/manifesto-e-patch.md`,
**1 JSON Schema**, **1 módulo** `crates/wyd-launcher/src/manifest.rs` (o crate precisa de `Cargo.toml`),
**1 fixture** manifesto+assinatura e **1 passo** no futuro `release.yml`. **Medido em 2026-11-25:**
existem o descritor (`docs/specs/manifesto-e-patch.md`), o manifesto medido
(`docs/specs/patch-1.0.0.json`), o crate com `src/manifest.rs` e `src/verify.rs` e as fixtures de
`crates/wyd-launcher/tests/fixtures/`; existem também a canonização (`src/canon.rs`, vetor do RFC em
`tests/rfc8785.rs`) e a **verificação** da assinatura Ed25519 (`src/signature.rs`, fixture em
`tests/fixtures/assinatura/`). O JSON Schema está publicado e validado por teste
(`planejamento-1.0.0/wyd-launcher/docs/specs/manifest.schema.json`, `tests/schema.rs`), o assinador
do `wyd-ops` existe (`crates/wyd-ops-signer`, bloco `manifesto-assinado` em
`.github/workflows/release.yml`, secret `WYD_LAUNCHER_SIGNING_KEY`) e o ciclo assinador → verificador
está coberto por `tests/ciclo-assinatura.rs`. **Falta apenas** o teste de ordem "assinatura reprovada
antes de qualquer escrita", que depende do passo 6 (aplicar patch), ainda sem código. `LLCH-01` e
`LLCH-03` estão fechadas como decisão.

## Referências

- Decisões: `Q-LCH-1` = A e `Q-LCH-3` = A (2026-11-23), em `docs/proposta-decisoes-launcher.md` §4.
- Lacunas fechadas: `LLCH-01`, `LLCH-03`.
- Contrato: `X11` em `wyd-1.0.0.md` §7.3; itens 1 e 2 do gate em `wyd-launcher-1.0.0.md` §6.
- Registro de publicação: `wyd-db/migrations/0030_portal_content.md:172–203`.
