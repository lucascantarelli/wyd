# Estrutura de pastas e mapa de documentação por módulo

> Pergunta: "onde mora cada artefato do projeto, qual sistema é dono dele e qual documento detalha cada módulo?"
> Público: Arquitetura, desenvolvimento e agentes
> Dono: `wyd-ops` (layout físico); cada sistema é dono de sua pasta e de seus módulos
> Base: `planejamento-1.0.0/wyd-1.0.0.md` §8; `planejamento-1.0.0/wyd-ops/auditoria-de-organizacao.md` §2-§5; `AGENTS.md` §5-§6
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Consolida o layout de §8 do panorama e o mapa de módulos já existente; não cria pasta, código ou issue.

## 1. Regra de ouro

`planejamento-1.0.0/` é só documentação (`AGENTS.md` §5, item 6). Artefato executável mora na pasta real
do projeto, na raiz. O dono único do layout físico é `planejamento-1.0.0/wyd-1.0.0.md` §8; esta página
consolida e indexa, não substitui.

- Pasta com `.gitkeep` é **reservada**, não implementada.
- `src/`, `tests/`, `prompts/` e `skills/` só nascem onde o dono abriu uma pergunta própria.
- Nenhum `.rs`, `.sql`, `.ts`, `.py` ou script executável entra em `planejamento-1.0.0/`.
- Mudança de fronteira entre sistemas exige decisão registrada (`AGENTS.md` §5, item 10).

## 2. Estrutura reservada e implementada

### 2.1 Raiz

```text
.
├── crates/<sistema>/              # crate Rust por sistema; hoje com código: wyd-db, wyd-launcher,
│                                  #   wyd-ops-signer (assinador do manifesto)
├── wyd-portal/                    # portal Next.js; hoje só .gitkeep
├── wyd-docs/                      # site MkDocs, tema, preparar.py, verificar*.py
├── wyd-ops/                       # imagem, Compose auxiliar, scripts, smoke tests
├── wyd-client/                    # binário ignorado + ferramenta de captura (LCLI-04)
├── .github/workflows/             # CI (caminho fixo do GitHub Actions)
├── .github/actions/               # actions compostas
├── docker-compose.yml             # raiz por descoberta da ferramenta
├── docker-compose.override.yml     # gate automático
├── docker-compose.dev.yml         # build opt-in da árvore de trabalho
└── .dockerignore                  # raiz por contexto do Docker
```

### 2.2 Planejamento por sistema

```text
planejamento-1.0.0/
├── wyd-1.0.0.md                   # panorama e dono do layout (§8)
├── status-planejamento.md         # relatório consolidado e índice de artefatos
├── <sistema>/
│   ├── <sistema>-1.0.0.md         # dashboard e plano diretor (exceto wyd-ops)
│   ├── lacunas.md                 # dono único de lacuna (exceto wyd-ops)
│   ├── docs/
│   │   ├── arquitetura/           # fronteira, contratos, componentes, ADRs
│   │   ├── specs/                 # requisitos, contratos, fluxos, estados
│   │   ├── planos/                # testes, dados, qualidade, riscos, execução
│   │   ├── roadmap/               # fases, DoD e fatias por issue
│   │   └── modulos.md             # índice de módulos (onde existe)
│   ├── src/.gitkeep               # reservado; nasce com a página aceita
│   └── tests/.gitkeep             # reservado; nasce com o plano de teste
└── wyd-ops/
    ├── docs/                      # propostas, prioridade, imagem/Compose, ADR global
    └── src/.gitkeep, tests/.gitkeep
```

### 2.3 Rust e testes

```text
crates/<sistema>/
├── Cargo.toml                     # membro do workspace (D-05 = A aprovada)
├── src/
│   ├── lib.rs                     # API pública
│   ├── domain/                    # regras puras, sem I/O
│   ├── application/               # casos de uso e portas
│   ├── adapter/                   # PostgreSQL, HTTP, filesystem, relógio
│   └── bin/<sistema>.rs           # composição
├── tests/
│   ├── contract_<alvo>.rs
│   ├── system_<alvo>.rs
│   └── fixtures/
└── migrations/                    # somente wyd-db
```

Sistema puro (`wyd-protocol`, `wyd-sim`) não cria `adapter/` vazio. `wyd-client`, `wyd-docs`,
`wyd-ops` e `wyd-portal` não são crate Rust.

## 3. Mapa de documentação por módulo

| Sistema | Módulo/área | Documento dono | Evidência de organização |
|---|---|---|---|
| Todos | fronteira e componentes | `docs/arquitetura/componentes.md` | auditoria §2: 13 organizados |
| `wyd-db` | provisionamento, repositórios, filas, observabilidade, segurança, baseline | `docs/modulos.md` + `docs/arquitetura/*` | 18 RF + 14 RNF; 12 testes |
| `wyd-ops` | ci, imagem, ambiente, monitoramento, segredos, deploy, gate, dashboard | `docs/modulos.md` | 8 RF + 7 RNF; fronteira/contratos parciais |
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
  tem nove skills: `auditar-planejamento`, `documentation-writer`, `finishing-a-development-branch`,
  `frontend-design`, `github-actions-docs`, `planejar-sistema`, `python-testing-patterns`,
  `skill-creator` e `writing-plans`. Skill complementar é opcional e usa a mesma lista. Dessas nove, **duas existem em disco** em
  `.agents/skills/` (`planejar-sistema` e `auditar-planejamento`) e **sete são cloud skills do projeto**
  (`documentation-writer`, `finishing-a-development-branch`, `frontend-design`, `github-actions-docs`,
  `python-testing-patterns`, `skill-creator`, `writing-plans`): não têm `SKILL.md` no repositório e são
  carregadas pelo ambiente do agente. Toda página de issue declara essa distinção no bloco de skills.
- O gate confere a regra de dono com `wyd-docs/verificar-skills.py`: roadmap que nomeia a skill, citação
  que não resolve e página sem `**Dono do valor:**` reprovam com arquivo e linha.
- **Fase, dependência, DoD e fora de escopo da fatia são da página da issue, não do roadmap.**
  `roadmap.md` e `fases.md` mantêm só a ordem, o risco e as regras de fatiamento; a tabela de fatias
  cita a página (com `§Skills`) e não repete célula `Fase`, `Requisitos`, `Depende de`, `Entrega`,
  `DoD` ou `Fora`. `index.md` é catálogo: não repete fila, fase, prioridade, bloqueio nem estado —
  aponta o campo da página. O mesmo gate reprova a célula repetida, a página de fatia não citada pelo
  roadmap e o roadmap cujo link de `§Skills` não resolve.
- O gate de contagens (`wyd-docs/verificar-contagens.py`) mede páginas de issue, fatias e páginas
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
- O gate também cobra a prova do ledger: `wyd-docs/verificar-lacunas.py` reprova toda lacuna marcada
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
  (`verificar-skills.py`) reprova a linha que republicar o valor, com arquivo e linha.
- `docs/planos/plano-de-testes.md` de cada sistema declara CI, cobertura e monitoramento com comando
  ou artefato verificável; não há seção vazia nem limiar inventado.

## 4. Cruzamento com a auditoria

| Achado da auditoria | Onde está | Efeito no layout |
|---|---|---|
| 13 sistemas organizados (12 plenos + `wyd-ops` parcial) | `auditoria-de-organizacao.md` §2 | §2.2 e §3 refletem esse veredito |
| `wyd-ops` tem fronteira/contratos parciais | `auditoria-de-organizacao.md` §2-§3 | não declarar `wyd-ops` pleno; lacunas de fronteira permanecem |
| 11 sistemas não tinham RF/RNF antes da segunda passada | `auditoria-de-organizacao.md` §3 | §3 lista RF/RNF medidos por sistema |
| `wyd-db` e `wyd-admin` são a referência | `auditoria-de-organizacao.md` §2 | usar como molde, não como cópia |
| 10 sistemas não têm RF numerado (estado histórico) | `auditoria-de-organizacao.md` §3 | resolvido na segunda passada; manter medição |
| `wyd-ops` abriu dashboard, requisitos, teste e roadmap em 2026-11-25 | `wyd-ops-1.0.0.md`; `docs/lacunas.md` | fronteira/contratos O1–O5 e aplicação de `Q-OPS-2` continuam pendentes |

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

```bash
# 1. nenhum artefato executável dentro do planejamento
find planejamento-1.0.0 -type f \( -name '*.rs' -o -name '*.sql' -o -name '*.ts' -o -name '*.py' -o -name '*.sh' \)

# 2. o único Dockerfile do projeto novo é o do wyd-ops
find . -name '*Dockerfile*' -not -path './deprecated/*' -not -path './.git/*'

# 3. pastas reservadas com .gitkeep (inclui prompts/ e skills/ do wyd-db)
find planejamento-1.0.0 -name .gitkeep | sort

# 4. nenhuma página de docs/ fora do alcance de docs/index.md
#    (script de travessia de links; 0 órfãos é o esperado)

# 5. templates declaram skill existente
grep -A12 'id: skill_principal' .github/ISSUE_TEMPLATE/*.yml
ls .agents/skills

# 6. gate de documentação
docker compose --profile gate run --rm gate
```

## 7. Decisões e perguntas abertas

- `wyd-ops` continua com fronteira/contratos parciais; aprovação de arquitetura é necessária antes de
  declará-lo pleno.
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
