# Estrutura de pastas e mapa de documentação por módulo

> Pergunta: "onde mora cada artefato do projeto, qual sistema é dono dele e qual documento detalha cada módulo?"
> Público: Arquitetura, desenvolvimento e agentes
> Dono: `wyd-ops` (layout físico); cada sistema é dono de sua pasta e de seus módulos
> Base: `planejamento-1.0.0/wyd-1.0.0.md` §8 (origem); `planejamento-1.0.0/wyd-ops/auditoria-de-organizacao.md` §2-§5; `AGENTS.md` §5-§6
> Revisado em: 2026-11-26

**Estado:** proposto (2026-11-26); consolidado e verificável. Esta é a **página autoritativa operacional** do
layout: o §8 do panorama registra a origem e o histórico, mas onde os dois divergirem vale esta página,
porque é a que o `make layout` executa. Ela não cria pasta, código, issue ou requisito novo.

## 1. Regra de ouro

`planejamento-1.0.0/` é só documentação (`AGENTS.md` §5, item 6). Artefato executável mora na pasta real
do projeto, na raiz. A árvore de §2 é a **árvore medida** em 2026-11-26; a regra por tipo de artefato
está em §2.4 e o verificador em §6.

- Pasta com `.gitkeep` é **reservada**, não implementada; `.gitkeep` só é aceito nas pastas de §2.4.
- `src/`, `tests/` e `prompts/` só nascem onde o dono abriu uma pergunta própria.
- `skills/` existe em **todo** sistema desde 2026-11-27 (decisão do dono): cada pasta tem
  `skills/README.md` como roteador e as skills daquele sistema, no molde de `wyd-db`
  (`§5.14`/`§6.19` do plano diretor). Skill de sistema não é cloud skill por definição: ela é lida pelo
  caminho e só entra na lista fechada de `skill_principal` quando também for registrada como cloud
  skill.
- Nenhum `.rs`, `.sql`, `.ts`, `.py`, `.sh` ou script executável entra em `planejamento-1.0.0/`.
- Script de CI, imagem e política de dependência moram em `wyd-ops/`, não na raiz: `wyd-ops/scripts/`
  (incluindo `ci/`), `wyd-ops/*.Dockerfile` e `wyd-ops/deny.toml`.
- Mudança de fronteira entre sistemas exige decisão registrada (`AGENTS.md` §5, item 10).

## 2. Estrutura reservada e implementada

### 2.1 Raiz

```text
.
├── .agents/                       # README, prompts/ e skills/; memory/ é ignorado
├── .github/                       # workflows/, actions/ e ISSUE_TEMPLATE/ (caminho fixo)
├── .vscode/                       # settings, launch, tasks e extensions (descoberta da IDE)
├── crates/<sistema>/              # crate Rust por sistema; 10 crates em 2026-11-26
├── planejamento-1.0.0/            # só documentação (panorama, status e 13 sistemas)
├── wyd-client/                    # tools/ (CLI de captura); files/ existe e é ignorado
├── wyd-docs/                      # site MkDocs, tema, gate, preparar.py, verificar*.py
├── wyd-ops/                       # dono de imagem, scripts, política e smoke tests
│   ├── *.Dockerfile               # wyd-docs, wyd-server, wyd-admin e wyd-portal
│   ├── deny.toml                  # política do cargo-deny
│   └── scripts/                   # ci/ (db-prepare, apply-series, db-schema-local) e sync-issue-status.mjs
├── wyd-portal/                    # Next.js: src/app + módulos do plano; tests/ e public/
├── docker-compose.yml             # raiz por descoberta da ferramenta; sem build:
├── docker-compose.override.yml    # gate automático
├── docker-compose.dev.yml         # build opt-in: docs `:dev` e perfil `stack`
├── .dockerignore                  # raiz por contexto do Docker
├── Makefile                       # fachada de comandos (make gate, gate-fast, layout)
├── rust-toolchain.toml            # descoberta do cargo/rustc
└── AGENTS.md, README.md, LICENSE, .env.example, .gitignore, .gitattributes
```

A raiz **não** tem `scripts/`, `deny.toml` nem Dockerfile. `wyd-ops/tests/smoke/` é reservado e só
nasce quando o primeiro teste de fumaça for escrito; não é `.gitkeep` de projeto vazio.

### 2.2 Planejamento por sistema

```text
planejamento-1.0.0/
├── wyd-1.0.0.md                   # panorama; §8 registra a origem do layout
├── status-planejamento.md         # relatório consolidado e índice de artefatos
├── <sistema>/                     # 13 pastas de sistema
│   ├── <sistema>-1.0.0.md         # dashboard e plano diretor
│   ├── lacunas.md                 # dono único de lacuna (exceto wyd-ops)
│   ├── inventario-*.tsv           # evidência de captura/inventário (hoje: wyd-client)
│   ├── docs/
│   │   ├── arquitetura/           # fronteira, contratos, componentes, ADRs
│   │   ├── specs/                 # requisitos, contratos, fluxos, estados
│   │   ├── planos/                # testes, dados, qualidade, riscos, execução e fatias
│   │   ├── roadmap/               # fases, DoD e fatias por issue
│   │   └── modulos.md             # índice de módulos (onde existe)
│   ├── src/.gitkeep               # reservado; nasce com a página aceita
│   ├── tests/.gitkeep             # reservado; nasce com o plano de teste
│   ├── prompts/.gitkeep           # só wyd-db, onde a pergunta própria já foi aberta
│   └── skills/README.md           # roteador de skills do sistema (todo sistema, 2026-11-27)
└── wyd-ops/
    ├── docs/                      # propostas, prioridade, imagem/Compose, ADR global e planos
    └── src/.gitkeep, tests/.gitkeep
```

Não há `.gitkeep` na raiz de um sistema: o dashboard já mantém a pasta. `planejamento-1.0.0/` não
recebe `.sql`, `.rs`, `.ts`, `.py`, `.sh`, manifesto, Dockerfile nem Compose; um plano novo nasce em
`<sistema>/docs/planos/` e uma fatia de produção em `<sistema>/docs/planos/fatia-*.md`.

### 2.3 Rust e testes

```text
crates/<sistema>/
├── Cargo.toml                     # manifesto próprio; não há workspace na raiz (D-05 aprovada)
├── Cargo.lock                     # lock próprio, versionado
├── src/
│   ├── lib.rs                     # API pública
│   ├── main.rs                    # composição, quando o crate tem binário
│   └── <módulo>.rs                # um módulo por responsabilidade do plano
├── tests/                         # contrato, sistema e fixtures (quando existirem)
└── migrations/                    # somente wyd-db
```

**Medido em 2026-11-26:** os 10 crates têm `Cargo.toml`, `Cargo.lock` e `src/lib.rs`; `wyd-server`,
`wyd-admin` e `wyd-ops-signer` têm `src/main.rs`; `wyd-db` tem `migrations/` e `tests/`. O alvo
`domain/`/`application/`/`adapter/` de §8.7 continua valendo para código com I/O, mas **não** se
move um binário para `src/bin/` só por simetria: `src/main.rs` é o caminho padrão do Cargo para um
crate com um binário. Sistema puro (`wyd-protocol`, `wyd-sim`) não cria `adapter/` vazio.
`wyd-client`, `wyd-docs`, `wyd-ops` e `wyd-portal` não são crate Rust.

### 2.4 Regra por tipo de artefato

| Artefato | Lugar previsto | Regra e dono |
|---|---|---|
| plano, dashboard, lacuna, ADR, fatia | `planejamento-1.0.0/<sistema>/` | Markdown/TSV de planejamento; a raiz do planejamento só tem o panorama e o status. Dono: o sistema. |
| evidência e captura | `planejamento-1.0.0/<sistema>/` (`inventario-*.tsv`, `docs/`); binário em `wyd-client/files/` | Evidência versionada é texto/tabela; binário capturado é ignorado e descrito por hash. Não vira script. Dono: o sistema que mede; captura do cliente em `wyd-client/`, análise de build/dependência em `wyd-ops/`. |
| script de CI, banco ou issues | `wyd-ops/scripts/` (CI em `ci/`) | Só `wyd-ops`; a raiz não tem `scripts/`. Dono: `wyd-ops`. |
| política de dependência | `wyd-ops/deny.toml` | `cargo deny --config wyd-ops/deny.toml`; não fica na raiz. Dono: `wyd-ops`. |
| config de IDE | `.vscode/` na raiz | `settings.json`, `launch.json`, `tasks.json` e `extensions.json`; descoberta da IDE. Dono: `wyd-ops` (experiência de desenvolvimento). |
| Dockerfile de serviço | `wyd-ops/<serviço>.Dockerfile` | Quatro: docs, server, admin e portal; nenhum Dockerfile dentro de `wyd-*` ou crate. Dono: `wyd-ops`. |
| Compose | raiz (`docker-compose*.yml`) | Base, override automático e dev opt-in; `dockerfile:` sempre aponta para `wyd-ops/`. Dono: `wyd-ops`. |
| código de crate | `crates/<sistema>/` | `Cargo.toml`, `Cargo.lock`, `src/` e `tests/`; `migrations/` só no `wyd-db`. Dono: o sistema. |
| código do portal | `wyd-portal/src/app` e `src/<módulo>` | Módulos do plano (`auth`, `banco`, `bff`, `admin`, `cache`, `monitoramento`, `observabilidade`); testes em `tests/`. Dono: `wyd-portal`. |
| dados, migration e seed | `crates/wyd-db/migrations/*.sql` | SQL é artefato do `wyd-db`, nunca de `planejamento-1.0.0/`; seed segue a mesma série. Dono: `wyd-db`. |
| site e gate | `wyd-docs/` | `preparar.py`, `verificar*.py`, `gate/`, `tema/`, `docs/` e `tests/`. Dono: `wyd-docs`. |
| cliente e ferramenta de captura | `wyd-client/tools/` | CLI/captura; `files/` é ignorado. Dono: `wyd-client`. |

**Exceções de raiz** são só as que a ferramenta descobre: `.github/`, `.vscode/`, `.agents/`,
`docker-compose*.yml`, `.dockerignore`, `Makefile`, `rust-toolchain.toml` e os arquivos de
repositório (`README`, `AGENTS`, `LICENSE`, `.gitignore`, `.gitattributes`, `.env.example`).

### 2.5 Portal: por que `src/app`

O plano genérico de D-04 em `wyd-1.0.0.md` §8.7 desenhou `app/` na raiz do portal; a árvore real do
Next.js 16 e o plano de componentes do `wyd-portal` usam `src/app` e `src/<módulo>`. Esta página
resolve o conflito em favor da árvore real: **`wyd-portal/src/app`**, com os módulos de §2.4, e
`tests/` na raiz do portal. O `docker-compose.dev.yml` e `wyd-ops/wyd-portal.Dockerfile` já apontam
para `wyd-portal/`; o Dockerfile continua em `wyd-ops/`, não no portal.

## 3. Mapa de documentação por módulo

| Sistema | Módulo/área | Documento dono | Evidência de organização |
|---|---|---|---|
| Todos | fronteira e componentes | `docs/arquitetura/componentes.md` | auditoria §2: 13 organizados |
| `wyd-db` | provisionamento, repositórios, filas, observabilidade, segurança, baseline | `docs/modulos.md` + `docs/arquitetura/*` | 18 RF + 14 RNF; 12 testes |
| `wyd-ops` | ci, imagem, ambiente, monitoramento, segredos, deploy, gate, dashboard | `docs/modulos.md` | 8 RF + 7 RNF; contratos O1–O5 aprovados |
| `wyd-core` | tick, ECS, combate, cena, IA | `docs/modulos.md` | 8 RF + 6 RNF |
| `wyd-client` | captura, direitos, inventário, patch | `docs/modulos.md` | 7 RF + 5 RNF |
| `wyd-bots` | bots de QA, carga e convivência | `docs/modulos.md` | 6 RF + 5 RNF |
| `wyd-docs` | site, tema, gate, publicação | `docs/modulos.md` | 7 RF + 5 RNF |
| `wyd-net` | transporte, sessão, rate-limit, hardening | `docs/arquitetura/componentes.md`, `docs/specs/requisitos.md` | 8 RF + 6 RNF |
| `wyd-server` | composição, ECAP, peer de captura | `docs/arquitetura/componentes.md`, `docs/specs/requisitos.md` | 7 RF + 6 RNF |
| `wyd-sim` | fórmulas, curvas, simulações | `docs/arquitetura/componentes.md`, `docs/specs/requisitos.md` | 7 RF + 6 RNF |
| `wyd-protocol` | pacotes, opcodes, offsets, goldens | `docs/arquitetura/componentes.md`, `docs/specs/requisitos.md` | 6 RF + 6 RNF |
| `wyd-launcher` | patch, manifesto, updater | `docs/arquitetura/componentes.md`, `docs/specs/requisitos.md` | 8 RF + 5 RNF |
| `wyd-admin` | administração, controle HTTP, custódia | `docs/arquitetura/componentes.md`, `docs/specs/requisitos.md` | 15 RF + 5 RNF |
| `wyd-portal` | BFF, login, cadastro, armaria, mercado | `docs/arquitetura/componentes.md`, `docs/specs/requisitos.md` | 35 RF + 10 RNF |

Seis sistemas já têm `docs/modulos.md` detalhado (`wyd-db`, `wyd-ops`, `wyd-core`, `wyd-client`,
`wyd-bots`, `wyd-docs`). Os sete restantes usam `componentes.md` como documento de módulo — ali estão
responsabilidade, API, invariantes, dependências, testes e risco — e `requisitos.md` para o critério de
aceite. Não há `modulos.md` duplicado nesses sete: a página adicional só nasce quando houver pergunta
própria que `componentes.md` não responda.

### 3.1 Indexação e skills

- Todo `docs/index.md` tem um **Mapa de páginas** que torna alcançável cada página de `docs/`; página
  nova entra no mapa da pasta e no índice, ou o gate de órfãos falha.
- Os quatro templates de `.github/ISSUE_TEMPLATE/` exigem `skill_principal` em lista fechada. A lista
  tem doze skills: `auditar-planejamento`, `documentation-writer`, `escrever-migration`,
  `finishing-a-development-branch`, `frontend-design`, `github-actions-docs`, `implementar-issue`,
  `planejar-sistema`, `python-testing-patterns`, `revisar-documentacao`, `skill-creator` e
  `writing-plans`. Skill complementar é opcional e usa a mesma lista. Das doze, **duas existem em disco**
  em `.agents/skills/` (`planejar-sistema` e `auditar-planejamento`); **sete são cloud skills do
  projeto** (`documentation-writer`, `finishing-a-development-branch`, `frontend-design`,
  `github-actions-docs`, `python-testing-patterns`, `skill-creator`, `writing-plans`), sem `SKILL.md` no
  repositório; e **três são skills de sistema do `wyd-db`** (`escrever-migration`, `implementar-issue`,
  `revisar-documentacao`), com `SKILL.md` em `planejamento-1.0.0/wyd-db/skills/` e cópia registrada como
  cloud skill em 2026-11-27. Toda página de issue declara essa distinção no bloco de skills.
- As outras cinco skills de `planejamento-1.0.0/wyd-db/skills/` (`levantar-requisito`, `planejar-onda`,
  `revisar-schema`, `provar-schema`, `provisionar-dados-de-teste`) são lidas pelo caminho, não entram na
  lista fechada: elas só valem dentro do `wyd-db`, e o roteador de `skills/README.md` diz quando usar
  cada uma.
- O gate confere a regra de dono com `wyd-docs/gate/checks/skills.py`: roadmap que nomeia a skill, citação
  que não resolve e página sem `**Dono do valor:**` reprovam com arquivo e linha.
- **Fase, dependência, DoD e fora de escopo da fatia são da página da issue, não do roadmap.**
  `roadmap.md` e `fases.md` mantêm só a ordem, o risco e as regras de fatiamento; a tabela de fatias
  cita a página (com `§Skills`) e não repete célula `Fase`, `Requisitos`, `Depende de`, `Entrega`,
  `DoD` ou `Fora`. `index.md` é catálogo: não repete fila, fase, prioridade, bloqueio nem estado —
  aponta o campo da página. O mesmo gate reprova a célula repetida, a página de fatia não citada pelo
  roadmap e o roadmap cujo link de `§Skills` não resolve.
- O gate de contagens (`wyd-docs/gate/checks/contagens.py`) mede páginas de issue, fatias e páginas
escritas em **todos** os dashboards e em `status-planejamento.md`: citação de página de issue é
conferida contra o `docs/roadmap/` do próprio sistema (ou o total, no status), `N fatias` contra as
páginas de fatia (`wyd-admin` conta IDs `ADM-nn` do roadmap, exceção declarada) e `N páginas
escritas` e `N páginas no wyd-db` contra as páginas `.md` do sistema nomeado. Linha que se declara
medição de passada anterior (ou data entre parênteses) é impressa como histórica e não reprova. O
mesmo gate reprova, nos 13 dashboards, `N tabelas`/`N arquivos` fora da série e `site passa com N
páginas`.
- A régua alcança também a estrutura e o censo de páginas: `N páginas em `docs/`` e cada item `**N na
raiz**` / `**N em `<pasta>/`**` (soma conferida contra a contagem recursiva da pasta), `(N sistemas)`
contra os `*/src/.gitkeep`, `docs/modulos.md em N sistemas` contra os arquivos existentes, e `N páginas
de módulo` só passa com o marcador `planejadas` — o `docs/src/` não existe e a contagem é meta, não
medida. Citação de estado do GitHub (board, issues abertas) não é número corrente: traz a fonte, a data
da leitura e a declaração de que o gate não a mede.
- Duas frases do status têm régua própria: `os N restantes` (na linha de `docs/modulos.md`) confere
com (sistemas com dashboard) − (arquivos `docs/modulos.md`), e `todos os N planos declaram` confere com
os `plano-de-testes.md` que citam CI, cobertura e monitoramento. `0 órfãos` e percentual de
alcançabilidade não são medidos por gate nenhum, então afirmá-los reprova.
- Também têm régua: `plano-de-testes.md` (nos|de) N sistemas (arquivos existentes), `N páginas
`WYDDB-nnn`` (páginas de issue do `wyd-db`), `N por fatia` (total menos as do `wyd-db`) e `nos N
sistemas` na linha de fatia (sistemas com página de fatia), `N páginas de domínio`
(`dicionario-dados/dominio-*.md`) e `N páginas `NNNN-`` só com o marcador `planejadas`.
- Inventário de docs: `alvos/*.md (N páginas)` confere com `tests/alvos/` e `specs/*.md (N páginas)`
com `tests/specs/` do sistema; `N páginas, uma por arquivo|alvo` e `N arquivos em `src/migrate/``
exigem `planejadas`/`previsto`, porque essas pastas não existem. Linha de árvore (com `▲`) e a tabela
de ondas (coluna **Arquivos**, declarada composição planejada) ficam fora.
- O gate também cobra a prova do ledger: `wyd-docs/gate/checks/lacunas.py` reprova toda lacuna marcada
  como fechada que não cite um artefato existente **que trate do item** (documento, teste, ADR,
  migration ou pergunta registrada) e imprime o placar medido (`118 = 108 fechadas + 10
  bloqueadas`), reprovando qualquer documento que cite soma diferente. A mesma régua vale para as
  somas publicadas em tabela (linha por sistema e linha `**Total**`) e em fórmula `N = a + p + f +
  e`: `Fechadas` tem de ser a medição de fechadas e `Abertas + Propostas + Externas` a de
  bloqueadas; trecho histórico (`após`, `estado anterior`, `antes do briefing`) fica fora.
- **Arquivo de IA cita o fato normativo, não o publica.** `AGENTS.md` e as skills de
  `.agents/skills/` não repetem o comando do gate, a lista/contagem de sistemas, contagem de ADR, de
  migration/tabela/seed, o limiar de cobertura nem o formato do cabeçalho: cada um tem dono aqui (§2,
  §6), em `wyd-1.0.0.md` §1, no índice de ADRs, em `plano-migrations.md` §3, em
  `estrategia-de-testes.md` §6 e em `wyd-db/docs/convencoes.md` §Cabeçalho de página. O mesmo gate
  (`gate/checks/skills.py`) reprova a linha que republicar o valor, com arquivo e linha.
- `docs/planos/plano-de-testes.md` de cada sistema declara CI, cobertura e monitoramento com comando
  ou artefato verificável; não há seção vazia nem limiar inventado.

## 4. Cruzamento com a auditoria

| Achado da auditoria | Onde está | Efeito no layout |
|---|---|---|
| 13 sistemas organizados (12 plenos + `wyd-ops` com gate parcial) | `auditoria-de-organizacao.md` §2 | §2.2 e §3 refletem esse veredito |
| `wyd-ops` tem contratos aprovados e gate parcial | `auditoria-de-organizacao.md` §2-§3 | não declarar `wyd-ops` pleno; a implementação permanece |
| 11 sistemas não tinham RF/RNF antes da segunda passada | `auditoria-de-organizacao.md` §3 | §3 lista RF/RNF medidos por sistema |
| `wyd-db` e `wyd-admin` são a referência | `auditoria-de-organizacao.md` §2 | usar como molde, não como cópia |
| 10 sistemas não têm RF numerado (estado histórico) | `auditoria-de-organizacao.md` §3 | resolvido na segunda passada; manter medição |
| `wyd-ops` abriu dashboard, requisitos, teste e roadmap em 2026-11-25 | `wyd-ops-1.0.0.md`; `docs/lacunas.md` | contratos O1–O5 aprovados; aplicação de `Q-OPS-2` continua pendente |

## 5. Cruzamento com a especificação

| Seção de especificação | O que define | Onde refletir |
|---|---|---|
| `wyd-1.0.0.md` §8.1 | `planejamento-1.0.0/` é só documentação | §1 |
| `wyd-1.0.0.md` §8.2 | artefato, lugar e dono | §2.1-§2.3 |
| `wyd-1.0.0.md` §8.3 | arquivos de raiz exigidos pela ferramenta | §2.1 |
| `wyd-1.0.0.md` §8.4 | pastas reservadas | §2.2-§2.3 |
| `wyd-1.0.0.md` §8.5 | o que não criar por analogia | §1 e §2.3 |
| `wyd-1.0.0.md` §8.7 | organização de código, testes e operação | §2.3 e §3 |
| `AGENTS.md` §5 | invariantes, ADR, segredo, estrutura antes de código | §1 e §6 |
| `AGENTS.md` §6 | sistemas e estado de planejamento | §3 |
| `AGENTS.md` §8-§9 | testes, CI e operação | §2.3 e §6 |

## 6. Como verificar

O verificador executável é o domínio `layout` do gate. Ele varre a árvore de trabalho (incluindo
não rastreados), ignora só `deprecated/`, `target/`, `node_modules/`, `site/`, `.build/`, `.venv/`,
`.next/`, `.tmp/`, `.freebuff/` e `wyd-client/files/`, e reprova com `caminho: motivo`:

```bash
make layout          # só o layout, com a lista completa de desvios
make gate-fast       # layout + skills + lacunas + contagens + censos (sem Docker/MkDocs)
make gate            # caminho completo do CI/container (build, links e todos os domínios)
```

O que o check exige, além da zona de cada arquivo:

- os 10 crates com `Cargo.toml`, `Cargo.lock` e `src/lib.rs`; `main.rs` em server/admin/signer;
  `migrations/` no `wyd-db`;
- portal com `package.json`, lock, `src/app/page.jsx` e `layout.jsx`; site com `preparar.py`,
  `gate.sh`, `gate/checks` e `tests`;
- os quatro `wyd-ops/*.Dockerfile`, os scripts `wyd-ops/scripts/ci/`, `deny.toml` e
  `wyd-client/tools/.gitkeep`;
- todo `dockerfile:` do Compose aponta para arquivo existente e o Compose base não tem `build:`;
- nenhuma referência a `scripts/ci/` da raiz em `Makefile`, `.vscode/` ou workflows.

Conferências manuais complementares (não substituem o `make layout`):

```bash
# 1. nenhum artefato executável dentro do planejamento (saída vazia)
find planejamento-1.0.0 -type f \( -name '*.rs' -o -name '*.sql' -o -name '*.ts' \
  -o -name '*.py' -o -name '*.sh' \)

# 2. Dockerfiles do projeto novo, todos em wyd-ops/
find . -name '*Dockerfile*' -not -path './deprecated/*' -not -path './.git/*'

# 3. referências antigas que não podem voltar
grep -rnE '(^|/)scripts/ci/|\.github/scripts/' Makefile .vscode .github/workflows

# 4. templates declaram skill existente
grep -A12 'id: skill_principal' .github/ISSUE_TEMPLATE/*.yml
ls .agents/skills
```

## 7. Conflitos resolvidos nesta consolidação

| Conflito | Resolução | Evidência |
|---|---|---|
| `scripts/ci/` na raiz versus `wyd-ops/scripts/` | Movido para `wyd-ops/scripts/ci/`; `ROOT`/`RAIZ` recalculados para `../../..`. | `git mv`; `make layout`; `make gate-fast`. |
| `.github/scripts/sync-issue-status.mjs` | Movido para `wyd-ops/scripts/sync-issue-status.mjs`; workflow e ADR-OPS-0009 citam o novo caminho. | `issue-status.yml`; `grep`. |
| `deny.toml` na raiz | Movido para `wyd-ops/deny.toml`; workflows e planos usam `--config wyd-ops/deny.toml`. | `security.yml`; `rg`. |
| Dockerfile do portal em `wyd-portal/` (texto antigo de §8.6) versus `wyd-ops/` | Vale `wyd-ops/wyd-portal.Dockerfile`; nenhum Dockerfile dentro do portal. | §8.2 do panorama já lista `wyd-ops/`; `find`. |
| Portal `app/` (árvore genérica de §8.7) versus `src/app` | Vale `src/app`, com os módulos do plano do portal; §2.5 registra. | `wyd-portal/src/app`; `componentes.md`. |
| `slice-wyd-db-001-wyd-net-001.md` solto na raiz do planejamento | Movido para `wyd-ops/docs/planos/fatia-001-wyd-db-wyd-net.md`; status atualizado. | `git mv`; `make layout`. |
| `.vscode/` na raiz | Exceção de descoberta da IDE, como `.github/`; JSON de configuração, não código de domínio. | `make layout`; `tasks.json`. |
| `.gitkeep` em projeto já implementado | Removido de `crates/*`, `wyd-ops/`, `wyd-docs/`, `wyd-portal/` e `.github/actions/`; mantido só em pasta reservada de §2.4. | `find`; `make layout`. |
| Evidência de captura do cliente | `inventario-arvore-cliente.tsv` fica no sistema `wyd-client`; binário em `files/` ignorado. | `make layout`; `registro-do-cliente.md`. |
| Evidência de depreciação | `wyd-ops/inventario-depreciacao.tsv` e `wyd-ops/docs/devops/evidencias-de-depreciacao.md`; mede toolchain, portal e scripts. | `make layout`; `make gate-fast`. |
| `.agents/memory/` | Ignorado pelo git e pelo layout; não é doc publicável. | `.gitignore`; `make layout`. |

## 7. Decisões e perguntas abertas

- `wyd-ops` tem os contratos O1–O5 aprovados; a implementação e o gate continuam pendentes, então ele
  não é declarado pleno.
- Criar issue, milestone, board, commit, push ou deploy continua sendo ato explícito do usuário.
- Nenhum número de censo ou contagem foi alterado por esta página.
- A implementação da árvore de `src/`/`tests/` continua subordinada à página `aceita` e ao gate do
  sistema (`AGENTS.md` §5, item 1).

## 8. Definition of Done

Esta página fecha quando:

1. o layout de §8 do panorama estiver refletido sem duplicação conflitante;
2. o mapa de módulos apontar para o documento dono de cada sistema;
3. os links internos e âncoras passarem no gate;
4. a auditoria e as seções de especificação estiverem citadas por nome e seção;
5. nenhuma pasta ou código tiver sido criado por esta página.
