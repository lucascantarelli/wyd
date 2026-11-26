# Plano do CI real — Fase 4 (`wyd-ops`)

> **Estado: histórico (`H`, 2026-11-19; superado em 2026-11-26).** Especificou, arquivo por arquivo,
> o CI da Fase 4. A arquitetura vigente é a híbrida por domínio (`ADR-OPS-0003`), implementada em
> `code.yml`, `ci-db.yml`, `docs.yml`, `docs-image.yml`, `security.yml` e `release.yml`; o estado real
> e o que falta estão em `docs/devops/index.md`. Esta página é registro da proposta original, não
> contrato vigente.
>
> **Decisão vigente (`V`, 2026-11-24):** D-02 = **B** — cobertura de 85% bloqueante no gate do sistema,
> informativa no `code.yml`; `cargo audit` e `cargo deny` bloqueantes. **Passo 8a executado em
> 2026-11-24:** o job `security` do `code.yml` roda `cargo audit` e `cargo deny` com `deny.toml`, e o
> job `cargo` publica cobertura informativa (sem `--fail-under`).
>
> **Relação com o dono:** `planejamento-1.0.0/wyd-ops/proposta-gestao-github-e-ambientes.md` §4.1
> decide a **arquitetura** (workflow reutilizável, action composta, SHA fixado, permissão mínima,
> estágios). Este documento não repete a regra: aplica-a a nomes de arquivo, jobs e gatilhos, e
> resolve o que §4.1 deixa em aberto — o check obrigatório. Conflito entre os dois é defeito a
> corrigir aqui.

---

## 1. Estado medido (`V`, 2026-11-19; revisto em 2026-11-24)
**Sem arquivo no repositório:** `_portal.yml` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Fato | Medição |
|---|---|
| workflows | 4: `docs` (`.github/workflows/docs.yml`), `docs-image` (`.github/workflows/docs-image.yml`), `code` (`.github/workflows/code.yml`) e `Full Workflow` (`.github/workflows/full-workflow.yml`, stub) |
| actions compostas | 1: `.github/actions/docs-check/action.yml` (mais `.gitkeep`) |
| `docs` | jobs `verificar` (gate do site) e `publicar` (Pages, só fora de PR); checkout e setup pinados por SHA |
| `Full Workflow` | 1 job `Get Info`, `success`, saída `version`/`commit` vazias por `$(...)` interpretado pelo shell (§2.1 do dono) |
| `Full Workflow` — pinagem | `actions/checkout` fixado por SHA (`v7.0.1`), como `docs` e `code` (2026-11-24) |
| ruleset `develop` | `required_approving_review_count: 0`; **nenhum** `required_status_checks` — nada bloqueia por check hoje |
| código Rust | `crates/wyd-db` tem `Cargo.toml`, `src/lib.rs`, `src/dsn.rs` e `tests/sistema_pg.rs`; 6 testes unitários + 13 de schema (2026-11-24); os demais crates só têm `.gitkeep`; sem workspace na raiz |
| `code` | fmt, clippy, 6 testes unitários e 13 de schema em PG17; cobertura informativa em artefato (sem `--fail-under`) e `cargo audit`/`cargo deny` bloqueantes com `deny.toml` (2026-11-24, passo 8a) |
| portal | sem `package.json`; nada a compilar |

Consequência: dos cinco estágios de §4.2, **documentação e código já têm o que rodar**; o portal não
tem `package.json`; a cobertura é informativa no `code.yml` (o 85% bloqueante continua no gate do
sistema, passo 5) e a segurança já bloqueia. Criar `_portal.yml` agora produziria job que sempre passa —
o que §4.2 proíbe.

---

## 2. Inventário proposto

### 2.1 Actions compostas (`.github/actions/`)
**Sem arquivo no repositório:** `_rust.yml`, `_db.yml` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Action | Inputs | O que faz | Entra quando |
|---|---|---|---|
| `docs-check` **(existe)** | `python` | instala `wyd-docs/requirements.txt`, roda `preparar.py`, `mkdocs build`, `gate/checks/links.py`, `gate/checks/contagens.py` e `gate/checks/censos.py` | já em uso |
| `setup-rust` | `toolchain` (`file` ou versão) | instala o toolchain de `rust-toolchain.toml`, cache de `~/.cargo` e `target/` com chave por `Cargo.lock` | quando houver `Cargo.toml` |
| `setup-node` | `node-version` | instala Node, cache de npm com chave por `package-lock.json` | quando houver `package.json` |
| `coverage-gate` | `path`, `minimum` (padrão 85) | lê o relatório e falha abaixo do mínimo | junto de `_rust.yml` |
| `setup-postgres` | `version` (padrão 17) | sobe serviço Postgres, espera `pg_isready`, exporta DSN de teste | junto de `_db.yml` |

**Medido em 2026-11-24 (passo 8a executado):** o workflow `code.yml` roda `cargo fmt --check`, `cargo clippy --all-targets -D warnings`, os 6 testes unitários e os 13 testes de `sistema_pg` contra um `postgres:17` migrado; a cobertura é publicada como artefato sem `--fail-under`; `cargo audit` e `cargo deny` rodam no job `security` com `deny.toml`. As actions `setup-rust`/`coverage-gate` e o gate de 85% continuam no passo 5.

Regras herdadas de §4.1 e válidas para todas: `using: composite`, `shell: bash` em cada `run`,
nenhum segredo lido de variável global, nenhuma action de terceiro por tag.

### 2.2 Workflows reutilizáveis (`.github/workflows/`)
**Sem arquivo no repositório:** `_docs.yml`, `_rust.yml`, `_db.yml`, `_portal.yml`, `_security.yml` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

Todos com `on: workflow_call`, `permissions: contents: read` no topo e inputs explícitos.

| Arquivo | Jobs | O que cada job faz | Falha quando |
|---|---|---|---|
| `_docs.yml` | `gate` | monta, constrói e verifica o site (`docs-check`) | link, âncora, navegação ou título reprovam |
| `_rust.yml` | `fmt`, `clippy`, `test`, `coverage` | `cargo fmt --check`; `clippy -D warnings`; testes por alvo; cobertura com `--fail-under` **só no gate do sistema** — no incremental é relatório informativo (D-02 = B) | qualquer aviso ou teste vermelho; cobertura abaixo do mínimo só quando o sistema declarar o gate fechado |
| `_db.yml` | `migrations`, `isolation` | aplica a série em Postgres 17 real e roda o runner; prova isolamento `portal` × `game` por `permission denied` | migration falha, checksum diverge, acesso indevido passa |
| `_portal.yml` | `typecheck`, `test`, `e2e` | `tsc --noEmit`; Vitest; Playwright com artefato de trace | tipo, teste ou E2E falha |
| `_security.yml` | `deps` | `dependency-review` em PR; `cargo audit`/`cargo deny` já bloqueiam no `code.yml` (passo 8a, 2026-11-24); reusa quando houver workspace | advisory ou licença proibida |

`CodeQL` continua workflow próprio (já ativo, `Analyze` ×3 verde) e **não** vira reutilizável: o
GitHub gerencia o ciclo dele.

### 2.3 Orquestrador e publicação
**Sem arquivo no repositório:** `_docs.yml` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Arquivo | Papel |
|---|---|
| `ci.yml` | chama os reutilizáveis por caminho alterado e expõe o job **`gate`** |
| `docs.yml` **(existe)** | passa a chamar `_docs.yml` e mantém o job `publicar` (Pages) fora do reutilizável |
| `release.yml` | só com tag `v*`: empacota e publica release — decisão separada, fora deste plano |
| `full-workflow.yml` **(existe)** | **sai**: o job `Get Info` não mede nada e o `$(...)` grava saída vazia; metadado de versão, se necessário, nasce em `release.yml` |

---

## 3. O job `gate` — especificação

Objetivo: **um** check que represente o CI inteiro, para que adicionar job não exija editar ruleset.

```yaml
# .github/workflows/ci.yml (esboço de plano, não aplicado)
name: ci
on:
  pull_request:
    branches: [develop, main]
  push:
    branches: [develop, main]
permissions:
  contents: read
concurrency:
  group: ci-${{ github.ref }}
  cancel-in-progress: ${{ github.event_name == 'pull_request' }}
jobs:
  # `./.github/workflows/_docs.yml` e `./.github/workflows/_rust.yml` ainda **não existem**: são o esboço deste plano.
  docs:
    if: ${{ ... caminhos de documentação alterados ... }}
    uses: ./.github/workflows/_docs.yml
  rust:
    if: ${{ ... caminhos Rust alterados ... }}
    uses: ./.github/workflows/_rust.yml
  gate:
    name: gate
    if: always()
    needs: [docs, rust]
    runs-on: ubuntu-latest
    steps:
      - name: Reprovar se algum job necessário falhou
        run: |
          echo "docs=${{ needs.docs.result }} rust=${{ needs.rust.result }}"
          for r in "${{ needs.docs.result }}" "${{ needs.rust.result }}"; do
            case "$r" in
              failure|cancelled) exit 1 ;;
            esac
          done
```

Três decisões embutidas, todas justificadas:

1. **`if: always()`** — sem isso o `gate` é pulado quando um job falha, e "pulado" não bloqueia.
2. **`skipped` é sucesso** — caminho não alterado não deve reprovar PR; o filtro de caminho é o
   contrato, e o job que não roda não pode falhar por ausência de código.
3. **`cancelled` reprova** — execução cancelada não é prova de nada; reaproveitar o resultado de um
   run cancelado é o caminho mais curto para merge sem CI.

Custo: um job de ~5 s por execução. O que ele agrega é a estabilidade do nome do check, não trabalho.

---

## 4. Status check obrigatório em `develop`
**Sem arquivo no repositório:** `_db.yml` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Campo | Valor proposto |
|---|---|
| Check | **`gate`** (workflow `ci`) |
| Ambiente | ruleset `develop — PR obrigatório` (id `27603854`), regra `required_status_checks` |
| Também em `main`? | sim, mesmo check — `main` só recebe release promovida |
| Aprovações | seguem em **0** (`wyd-ops` §3.7: exigir aprovação sem segundo revisor travaria o dono) |
| `strict` (branch atualizada) | **não** — squash + PR pequeno tornam o requisito caro e o histórico já é linear |

**Por que um só check, e não `docs` + `rust` + `portal`:** com o agregador, criar `_db.yml` amanhã não
exige editar ruleset (que é ação de admin e some do diff do repositório); e a lista de checks exigidos
não fica dependente de o job existir. O preço é depender da lógica do `gate` — por isso ela é
explícita acima e testada no primeiro PR que a introduz.

**O que continua visível mas não obrigatório:** `build e verificação` (job do `docs`), `CodeQL`,
`Analyze` ×3 e `GitGuardian`. Eles reportam; só o `gate` bloqueia.

**Bypass conhecido:** workflow de `pull_request` vindo da mesma branch pode editar `ci.yml`. Se o
`gate` desaparecer, o check exigido fica **ausente**, e ausência bloqueia merge — o que protege o
caso, mas transforma um PR malicioso em PR travado, não em PR verde. Para fork, `pull_request` não
recebe segredo e o `gate` roda igual.

---

## 5. Ordem de implementação (cada passo é um PR)
**Sem arquivo no repositório:** `_docs.yml`, `_rust.yml`, `_db.yml`, `_portal.yml`, `_security.yml` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Passo | Entrega | Depende de | Custo |
|---:|---|---|---|
| 1 | `_docs.yml` + `docs.yml` chamando o reutilizável | — | ~40 min |
| 2 | `ci.yml` com `gate` agregando só `docs`; `docs.yml` mantém `publicar` | 1 | ~40 min |
| 3 | remover `full-workflow.yml` (o metadado de versão, se preciso, vai para `release.yml`) | 2 | ~15 min |
| 4 | **habilitar `gate` como required em `develop` e `main`** | 2 verde em PR real | ~5 min + admin |
| 5 | `setup-rust` + `_rust.yml` + `coverage-gate`, e `rust` entra no `gate` | workspace Rust existir | 1 sessão |
| 6 | `setup-postgres` + `_db.yml` | migrations do `wyd-db` existirem | 1 sessão |
| 7 | `setup-node` + `_portal.yml` | `package.json` do portal existir | 1 sessão |
| 8a | **D-02 = B — executado em 2026-11-24:** `cargo audit` + `cargo deny` bloqueantes e cobertura informativa no `code.yml` | — | feito |
| 8 | `_security.yml` (`dependency-review`, `cargo-deny`) | 5 e 8a | ~1 h |

O passo 4 é o único que muda governança; 1–3 são invisíveis para quem não abre Actions e podem
entrar antes de qualquer código. **O passo 8a não espera 5–7:** D-02 = B o torna a primeira alteração de CI
depois deste plano, e ele não depende de workspace, portal ou runner; os passos 5–7 continuam esperando
seus artefatos.

**O que o passo 8a alterou (2026-11-24):**

- `.github/workflows/code.yml`: novo job `security`, que detecta `Cargo.toml`, exige `Cargo.lock` e roda
  `cargo audit --file <lock>` e `cargo deny --config deny.toml --manifest-path <manifest> check advisories
  bans licenses sources`; o job `cargo` ganhou cobertura com `cargo llvm-cov --lib --summary-only` e
  `--html`, publicada como artefato, sem `--fail-under`.
- `deny.toml` na raiz: lista de licenças permitidas, nenhum advisory ignorado, `wildcards = "deny"` e
  fonte desconhecida bloqueada.
- **Como a falha aparece:** advisory ativo → `cargo audit` sai **1** e o job `security` fica vermelho;
  licença fora da lista → `cargo deny` sai **4** e o job fica vermelho; `Cargo.toml` sem `Cargo.lock` →
  passo falha com `::error::` e exit 1; nenhum `Cargo.toml` → job verde com resumo explícito de que não
  havia código Rust.
- **Provado localmente em 2026-11-24:** fixture com `time 0.1.44` → `RUSTSEC-2020-0071`, exit 1; fixture
  com dependência **privada** `GPL-3.0-only` → `licenses FAILED`, exit 4; cobertura do `wyd-db` → 92,96% de
  regiões e 90,57% de linhas, exit 0 sem limite. O `Cargo.toml` declara GPL-2.0-only e a exceção do
  `deny.toml` vale só para `wyd-db`; `ignore = false` impede que `publish = false` vire bypass.

---

## 6. Riscos

| Risco | Efeito | Mitigação no plano |
|---|---|---|
| `gate` com lógica errada | tudo verde por omissão | job mínimo, com `failure`/`cancelled` explícitos e teste no PR que o cria |
| filtro de caminho errado | PR de código sem `rust`, PR de doc sem `docs` | filtros declarados no `ci.yml`, um lugar só |
| cache envenenado | build usa artefato de PR | cache de `pull_request` em escopo de branch, sem `restore-keys` amplo |
| action por tag móvel | execução muda sem commit | SHA fixado (§4.1) e Dependabot para atualizar |
| jobs longos no PR | fila e cancelamento | `cancel-in-progress` em PR; `concurrency` por ref |
| `full-workflow` removido sem substituto | perde o único metadado de versão | passo 3 só depois do 2, e `release.yml` cobre tag |

---

## 7. O que depende do dono
**Sem arquivo no repositório:** `_security.yml` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

- **Habilitar `gate` como required** (passo 4): admin do repositório; o plano não altera ruleset.
- **Aprovar a remoção de `full-workflow.yml`** (passo 3): apaga workflow existente; reversível por
  revert, mas é mudança visível na aba Actions.
- **D-02 = B aprovada e passo 8a executado em 2026-11-24:** `cargo audit`/`cargo deny` bloqueiam no
  `code.yml` e a cobertura é informativa; resta o passo 8 (`_security.yml`) e o `pip-audit`/hash do site
  (`mkdocs==2.0.dev6`).
- **Decidir se `main` espelha os checks de `develop`** — proposto sim, mas `main` hoje só recebe
  release.

---

## 8. Divergências encontradas nesta leitura (`V`, 2026-11-19)

| Onde | O que diz | O que é medido |
|---|---|---|
| `wyd-ops` §4.1 | `docs-check/action.yml # mkdocs build --strict + links` | a 2.0 **não tem** `--strict` (§2.2 do plano do site); a action real roda `preparar.py`, `mkdocs build` e `gate/checks/links.py` |
| `full-workflow.yml:37` | `actions/checkout@v7` | **corrigido em 2026-11-24:** `actions/checkout` passou a SHA fixo, como `docs.yml` e `code.yml` |
| `docs.yml` — filtros | `push` inclui `planejamento-1.0.0/wyd-1.0.0.md`; `pull_request` **não** | PR que altera só o panorama não roda o gate de docs, mas o merge publica — assimetria a resolver no passo 1 |

A pinagem foi corrigida em 2026-11-24; as outras duas seguem como achados deste plano e exigem mexer
em workflow.

## 9. O que este documento não decide

- Não altera ruleset, environment ou permissão; o passo 8a já alterou `code.yml` e criou `deny.toml`.
- Não define versões de action: `docs.yml` já fixa as suas; as demais entram travadas no PR do passo.
- Não decide cobertura bloqueante, release nem `main`.
- Não substitui §4.1/§4.2 do dono: só os aplica em nomes de arquivo e jobs.
