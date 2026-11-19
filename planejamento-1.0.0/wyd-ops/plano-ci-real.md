# Plano do CI real — Fase 4 (`wyd-ops`)

> **Estado: proposta (`P`, 2026-11-19).** Especifica, arquivo por arquivo, o CI da Fase 4:
> workflows reutilizáveis, actions compostas, jobs e o status check que passa a ser obrigatório em
> `develop`. Não altera workflow, ruleset nem ambiente — nada aqui está aplicado.
>
> **Relação com o dono:** `planejamento-1.0.0/wyd-ops/proposta-gestao-github-e-ambientes.md` §4.1
> decide a **arquitetura** (workflow reutilizável, action composta, SHA fixado, permissão mínima,
> estágios). Este documento não repete a regra: aplica-a a nomes de arquivo, jobs e gatilhos, e
> resolve o que §4.1 deixa em aberto — o check obrigatório. Conflito entre os dois é defeito a
> corrigir aqui.

---

## 1. Estado medido (`V`, 2026-11-19)

| Fato | Medição |
|---|---|
| workflows | 2: `Full Workflow` (`.github/workflows/full-workflow.yml`) e `docs` (`.github/workflows/docs.yml`) |
| actions compostas | 1: `.github/actions/docs-check/action.yml` (mais `.gitkeep`) |
| `docs` | jobs `verificar` (gate do site) e `publicar` (Pages, só fora de PR); checkout e setup pinados por SHA |
| `Full Workflow` | 1 job `Get Info`, `success`, saída `version`/`commit` vazias por `$(...)` interpretado pelo shell (§2.1 do dono) |
| `Full Workflow` — pinagem | `actions/checkout@v7`, **tag móvel**, contra a regra de SHA fixado (§4.1) |
| ruleset `develop` | `required_approving_review_count: 0`; **nenhum** `required_status_checks` — nada bloqueia por check hoje |
| código Rust | `crates/*` só com `.gitkeep`; sem `Cargo.toml` na raiz |
| portal | sem `package.json`; nada a compilar |

Consequência: dos cinco estágios de §4.2, **só o de documentação tem o que rodar hoje**. Criar
`_rust.yml` ou `_portal.yml` agora produziria job que sempre passa — o que §4.2 proíbe.

---

## 2. Inventário proposto

### 2.1 Actions compostas (`.github/actions/`)

| Action | Inputs | O que faz | Entra quando |
|---|---|---|---|
| `docs-check` **(existe)** | `python` | instala `wyd-docs/requirements.txt`, roda `preparar.py`, `mkdocs build`, `verificar.py` | já em uso |
| `setup-rust` | `toolchain` (`file` ou versão) | instala o toolchain de `rust-toolchain.toml`, cache de `~/.cargo` e `target/` com chave por `Cargo.lock` | quando houver `Cargo.toml` |
| `setup-node` | `node-version` | instala Node, cache de npm com chave por `package-lock.json` | quando houver `package.json` |
| `coverage-gate` | `path`, `minimum` (padrão 85) | lê o relatório e falha abaixo do mínimo | junto de `_rust.yml` |
| `setup-postgres` | `version` (padrão 17) | sobe serviço Postgres, espera `pg_isready`, exporta DSN de teste | junto de `_db.yml` |

Regras herdadas de §4.1 e válidas para todas: `using: composite`, `shell: bash` em cada `run`,
nenhum segredo lido de variável global, nenhuma action de terceiro por tag.

### 2.2 Workflows reutilizáveis (`.github/workflows/`)

Todos com `on: workflow_call`, `permissions: contents: read` no topo e inputs explícitos.

| Arquivo | Jobs | O que cada job faz | Falha quando |
|---|---|---|---|
| `_docs.yml` | `gate` | monta, constrói e verifica o site (`docs-check`) | link, âncora, navegação ou título reprovam |
| `_rust.yml` | `fmt`, `clippy`, `test`, `coverage` | `cargo fmt --check`; `clippy -D warnings`; testes por alvo; cobertura com `--fail-under` | qualquer aviso, teste vermelho ou cobertura abaixo do mínimo |
| `_db.yml` | `migrations`, `isolation` | aplica a série em Postgres 17 real e roda o runner; prova isolamento `portal` × `game` por `permission denied` | migration falha, checksum diverge, acesso indevido passa |
| `_portal.yml` | `typecheck`, `test`, `e2e` | `tsc --noEmit`; Vitest; Playwright com artefato de trace | tipo, teste ou E2E falha |
| `_security.yml` | `deps` | `dependency-review` em PR; `cargo-deny` quando houver workspace | advisory ou licença proibida |

`CodeQL` continua workflow próprio (já ativo, `Analyze` ×3 verde) e **não** vira reutilizável: o
GitHub gerencia o ciclo dele.

### 2.3 Orquestrador e publicação

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

| Passo | Entrega | Depende de | Custo |
|---:|---|---|---|
| 1 | `_docs.yml` + `docs.yml` chamando o reutilizável | — | ~40 min |
| 2 | `ci.yml` com `gate` agregando só `docs`; `docs.yml` mantém `publicar` | 1 | ~40 min |
| 3 | remover `full-workflow.yml` (o metadado de versão, se preciso, vai para `release.yml`) | 2 | ~15 min |
| 4 | **habilitar `gate` como required em `develop` e `main`** | 2 verde em PR real | ~5 min + admin |
| 5 | `setup-rust` + `_rust.yml` + `coverage-gate`, e `rust` entra no `gate` | workspace Rust existir | 1 sessão |
| 6 | `setup-postgres` + `_db.yml` | migrations do `wyd-db` existirem | 1 sessão |
| 7 | `setup-node` + `_portal.yml` | `package.json` do portal existir | 1 sessão |
| 8 | `_security.yml` (`dependency-review`, `cargo-deny`) | 5 | ~1 h |

O passo 4 é o único que muda governança; 1–3 são invisíveis para quem não abre Actions e podem
entrar antes de qualquer código.

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

- **Habilitar `gate` como required** (passo 4): admin do repositório; o plano não altera ruleset.
- **Aprovar a remoção de `full-workflow.yml`** (passo 3): apaga workflow existente; reversível por
  revert, mas é mudança visível na aba Actions.
- **Decidir se cobertura bloqueia desde o primeiro sistema** ou começa advisory e vira bloqueante
  quando o primeiro crate chegar a 85% — `AGENTS.md` §8 fixa a meta, não o momento de bloquear.
- **Decidir se `main` espelha os checks de `develop`** — proposto sim, mas `main` hoje só recebe
  release.

---

## 8. Divergências encontradas nesta leitura (`V`, 2026-11-19)

| Onde | O que diz | O que é medido |
|---|---|---|
| `wyd-ops` §4.1 | `docs-check/action.yml # mkdocs build --strict + links` | a 2.0 **não tem** `--strict` (§2.2 do plano do site); a action real roda `preparar.py`, `mkdocs build` e `verificar.py` |
| `full-workflow.yml:37` | `actions/checkout@v7` | tag móvel, contra a regra de SHA fixado de §4.1; `docs.yml` já usa SHA |
| `docs.yml` — filtros | `push` inclui `planejamento-1.0.0/wyd-1.0.0.md`; `pull_request` **não** | PR que altera só o panorama não roda o gate de docs, mas o merge publica — assimetria a resolver no passo 1 |

Nenhuma das três foi corrigida: são achados deste plano, e duas exigem mexer em workflow.

## 9. O que este documento não decide

- Não altera workflow, ruleset, environment ou permissão.
- Não define versões de action: `docs.yml` já fixa as suas; as demais entram travadas no PR do passo.
- Não decide cobertura bloqueante, release nem `main`.
- Não substitui §4.1/§4.2 do dono: só os aplica em nomes de arquivo e jobs.
