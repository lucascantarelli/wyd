# WYD 1.0.0 — panorama e plano de execução

> **Estado: plano de trabalho, não decisão aprovada.** Este documento é o painel único do projeto
> inteiro: o que existe, o que está medido, em que ordem o planejamento avança e qual é o gate de
> cada etapa. Ele **não** substitui o plano diretor de nenhum sistema e **não** decide arquitetura.
>
> Medido em **2026-11-19**. Toda contagem traz a fonte; número sem fonte é `H`.

---

## 0. Como ler

| Se você quer… | Vá para |
|---|---|
| saber o que está pronto e o que não está | §1 |
| saber o que já foi decidido e o que está aberto | §2 |
| ver contradições que precisam de decisão do time | §3 |
| saber a ordem de execução e o gate de cada fase | §4 |
| saber o que este plano não decide | §5 |
| saber o que fazer em seguida, por ganho ÷ esforço | §6 |

---

## 1. Estado medido

### 1.1 Por sistema

| Sistema | Documentos | Estado | Evidência |
|---|---:|---|---|
| `wyd-db` | 76 | piloto parcial: 15 páginas com cabeçalho, 2 de 29 migrations, 37 páginas de `src/`, 0 em `tests/`, `prompts/` e `skills/` | `find`/`grep` em 2026-11-19 |
| `wyd-protocol` | 4 | 2º piloto aberto: plano diretor com teto de 131 páginas, 2 escritas, 20 lacunas, nenhum ADR | `wyd-protocol-1.0.0.md` e `lacunas.md` |
| `wyd-docs` | 4 | plano diretor e lacunas (14) + site MkDocs 2.0 implementado | `wyd-docs-1.0.0.md`; `wyd-docs/` |
| `wyd-portal` | 1 | só o prompt de produto | `wyd-portal/wyd-portal-prompt.md` |
| `wyd-admin`, `wyd-bots`, `wyd-client`, `wyd-core`, `wyd-launcher`, `wyd-net`, `wyd-ops`, `wyd-server`, `wyd-sim` | 0 | não planejado; só `.gitkeep` | `find planejamento-1.0.0` |

### 1.2 `wyd-db`, página por pasta

| Pasta | Escritas | Planejadas | Observação |
|---|---:|---:|---|
| `docs/` raiz (`index`, `convencoes`, `glossario`) | 3 | 3 | fecha |
| `docs/arquitetura/` | 7 | 17 | faltam C4, modelo conceitual/lógico, retenção, fronteira |
| `docs/specs/` | 0 | 20 | nenhum requisito com ID existe ainda |
| `docs/dicionario-dados/` | 1 | 20 (+91 geradas) | só `dominio-acesso` |
| `docs/planos/` | 1 | 10 | só `plano-de-implantacao` |
| `docs/roadmap/` | 0 | 42 | 40 issues `WYDDB-nnn` planejadas, nenhuma criada |
| `docs/guia-desenvolvedor/` | 2 | 11 | |
| `docs/guia-administrador/` | 0 | 8 | |
| `docs/guia-teste/` | 1 | 7 | |
| `docs/guia-documentacao/` | 0 | 8 | |
| `migrations/` | 2 | 31 | 2 páginas de seção + 29 arquivos |
| `src/` | 37 | 59 | `migrate/` e `provision/` têm página por arquivo |
| `tests/` | 0 | 12 | |
| `prompts/` | 0 | 9 | |
| `skills/` | 0 | 9 | |

**Totais planejados:** 146 páginas autorais em `docs/` + 91 geradas + 31 de migration + 59 de `src/`
+ 12 de teste + 9 prompts + 9 skills (`wyd-db-1.0.0.md` §4). **Escritas:** 15 em `docs/`, 2 em
`migrations/`, 37 em `src/`.

### 1.3 Série de migrations

| Indicador | Valor | Fonte |
|---|---:|---|
| Arquivos da série | 29 | `plano-migrations.md` §3 |
| Tabelas | 91 | idem |
| Arquivos de seed | 6 | idem |
| Tabelas com DDL fechado | 14 de 91 | `ddl-confirmado.md` §1 |
| Arquivos com DDL inteiro fechado | 4 (`0001`, `0003`, `0004`, `0007`) | `plano-migrations.md` §3 |
| `SCHEMA_VERSION` alvo | 29 | `plano-migrations.md` §3 |

### 1.4 Repositório, GitHub e operação

| Fato | Valor | Fonte |
|---|---|---|
| Visibilidade / branch padrão | público / `develop` | API do GitHub, 2026-11-19 |
| Issues / milestones / Projects | 0 abertas / 6 milestones (M0–M5) / habilitado, project `WYD 1.0.0` nº 10 | API do GitHub, 2026-11-19 |
| Merge permitido | squash, branch apagada no merge | API do GitHub |
| Secret scanning / Dependabot | ativos | API do GitHub |
| `crates/*` | 9 pastas, só `.gitkeep` | `git ls-files` |
| Código Rust real | `deprecated/` com 375 arquivos, **0 versionados** | `git ls-files deprecated` |
| Raiz | sem `Cargo.toml`, `rust-toolchain.toml`, `Makefile` | `ls` |
| CI | `full-workflow`: 1 job (`Get Info`), **verde**, sem build/teste e com `version`/`commit` vazios (`V`: run #14 `44505419869`, job `135495629487`; detalhe em `wyd-ops` §2.1); `docs`: gate + publicação | `.github/workflows/` |
| Workflows de referência no legado | 3 (`ci`, `bench`, `release`) | `deprecated/.github/workflows/` |
| Site de documentação | MkDocs 2.0 (`2.0.dev6`) + Python 3.15; 30 páginas montadas de 3 sistemas; build e `verificar.py` verdes | `wyd-docs/`, 2026-11-19 |
| Cliente | 7.147 arquivos em `wyd-client/files/`, ignorado | `find` |

---

## 2. O que está decidido e o que está aberto

### 2.1 Decidido (`aceito`)

| ID | Decisão | Data |
|---|---|---|
| `P-01` | 4 schemas por domínio: `content`, `game`, `portal`, `ops`; nada em `public` | 2026-11-17 |
| `P-02` | tabela no singular, `snake_case` | 2026-11-17 |
| `P-03`–`P-22` | PK identity, PK composta em filha, sem UUID, `TIMESTAMPTZ`, dinheiro inteiro, `TEXT`+`CHECK`, `INET`, coluna gerada, `NOT NULL` por padrão, constraint nomeada, índice parcial, FK indexada, trigger só mecânica, view de leitura, papéis separados, migration imutável, sem soft-delete genérico, retenção declarada, PII só em `portal` | 2026-11-17 |
| `ADR-DB-0001` | provisionamento de papel é fase própria, fora da série | 2026-11-17 |
| `ADR-DB-0002` | runner de migrations próprio, com ledger por versão | 2026-11-18 |
| `M1`–`M25`, `A9`–`A16`, `D1`–`D14` | decisões de desenho fechadas, cada uma com evidência em `lacunas.md` | 2026-11-17/18 |

São **dois** ADRs. Não existe catálogo prévio; ADR nasce de decisão tomada, não de planejamento.

### 2.2 Aberto

| Grupo | Abertas | Exemplos |
|---|---:|---|
| Lacunas `B`/`A`/`M` | 14 | `B1` (entrega do link de recuperação), `B3` (política de PIN), `A17` (transferência de conta), `A21`, `M16`, `M17` (idempotência de crédito), `M19`, `M22` |
| Externas | 3 | `A11`, `A20`, `M21` |
| Divergências do plano diretor | 3 | `P1`, `P3`, `P5` |
| DDL não desenhado | 77 tabelas | `0012`, `0013`, `0029` totalmente bloqueadas |
| ADR de fronteira pessoa × conta | sem número | citada em `lacunas.md` §0.2 e `rascunho-accounts.md` |

---

## 3. Divergências que precisam de decisão do time

Estas não são tarefas: são contradições entre documentos ou entre documento e realidade. Ficam
registradas aqui até alguém decidir; nenhuma foi corrigida por conta própria.

| # | Divergência | Evidência | Efeito |
|---|---|---|---|
| D1 | `README.md` da raiz afirma desenvolvimento ativo, 10 crates, 34 issues ativas, 58 concluídas, 53 migrations e serviços no ar — **resolvido em 2026-11-19**: reescrito para o estado medido, sem contagem replicada | `README.md` × `git ls-files`, API do GitHub | agente e visitante leem estado falso |
| D2 | `crates/*` estão vazios, mas o README e o CI pressupõem workspace Rust | `find crates` | qualquer `cargo build` na raiz falha |
| D3 | CI tem um job só, e o `run:` monta `$(0.0.0-<sha>)` e `$(<sha>)`, que o shell executa como comando — **medido em 2026-11-19**: o job **não** falha e publica metadado vazio | `V`: run #14 (`44505419869`) e job `Get Info` (`135495629487`) com `success`; log traz `command not found` nas linhas 40 e 44; 0 runs `failure`; detalhe em `wyd-ops` §2.1 | gate verde que não publica `version`/`commit`; não há build, teste, cobertura nem gate de código |
| D4 | O planejamento exige CI composto por actions reutilizáveis; `.github/actions/` tem só `docs-check` | `V`: `git ls-files .github`, 2026-11-19 | meta da `1.0.0` sem base: faltam as actions de Rust, cobertura e segurança de `wyd-ops` §4 |
| D5 | Gestão de trabalho: o roadmap de 40 issues do `wyd-db` não tinha onde viver | `V`, 2026-11-19: `has_projects: true`, 6 milestones (`M0`–`M5`) e project `WYD 1.0.0` nº 10; execução registrada em `wyd-ops` §10 | **resolvido em 2026-11-19**: a estrutura existe (labels, formulários, board); falta criar as issues, que dependem de página com DoD |
| D6 | O plano diretor declara 146 páginas autorais, mas 84 estão bloqueadas por falta de decisão | `proposta-paginas.md` §1 | escrever mais rápido exige fechar lacuna, não escrever autour |
| D7 | Não existe `mkdocs.yml` em lugar nenhum, mas `wyd-docs` é dono do site e o guia do `wyd-db` já prescreve recursos do MkDocs — **resolvido em 2026-11-19**: site em MkDocs 2.0 no `wyd-docs/`, sem `mkdocs.yml`; recursos do MkDocs 1.x traduzidos por `compat.py` (§2.2 e `LPDOC-14`) | `find . -iname 'mkdocs*.yml'` sem resultado | site não publicável; exigência sem base executável |
| D8 | `docs/` na raiz não existe; no legado, `docs/` é referência — **resolvido em 2026-11-19**: a documentação publicada sai de `planejamento-1.0.0/*/docs/` por `wyd-docs/`, e o legado não é publicado | `ls` | risco de alguém publicar o legado como doc oficial |

---

## 4. Plano de fases

Cada fase termina com um gate verificável. Nenhuma fase começa antes do gate anterior, salvo decisão
explícita do time.

### Fase 0 — Fundação e memória do projeto ✅ (2026-11-19)

- [x] Medir repositório, planejamento, GitHub e CI sem alterar nada.
- [x] Reescrever `AGENTS.md` com hierarquia de fontes e invariantes.
- [x] Registrar panorama neste arquivo.
- [x] Criar memória de trabalho em `.agents/memory/estado-do-projeto.md`.
- [x] Criar skills `auditar-planejamento` e `planejar-sistema`, e o prompt de auditoria.

**Gate:** um agente novo consegue dizer o que está planejado, o que é referência e o que é proibido
sem abrir o legado.

### Fase 1 — Gestão do trabalho no GitHub ✅ (2026-11-19)

Proposta em `wyd-ops/proposta-gestao-github-e-ambientes.md` aprovada e executada (§10 dela).

- [x] Habilitar Projects do repositório e criar o project `WYD 1.0.0` com campos `Sistema`,
      `Fase`, `Prioridade` e `Marco` (além do `Status` padrão).
- [x] Criar as labels `area/*` (13), `estado/*` (3), `origem/legado` e `risco/*` (2), sem duplicar
      tipo em label.
- [x] Criar os milestones `M0 — Fundação` a `M5 — Operação`.
- [x] Criar `.github/ISSUE_TEMPLATE/` com `bug`, `planejamento`, `decisao`, `tarefa` e `config.yml`,
      e `.github/pull_request_template.md` com gate de documentação, prova e escopo.
- [x] Configurar ruleset de `develop` e `main`: PR obrigatório, sem deleção, sem force-push.
      **Sem *required status check***: não há check que reporte (entra na Fase 4).
- [ ] Migrar as 40 issues planejadas de `docs/roadmap/` para issues reais **só depois** de as páginas
      existirem; issue sem DoD verificável não é criada.

**Gate:** uma issue criada pelo formulário chega ao board com sistema, fase e prioridade; o board
mostra o mesmo número de itens que o roadmap do sistema. **Pendente:** views do project e o primeiro
item real — nenhuma issue foi criada.

### Fase 2 — Fechar o `wyd-db`

Ordem derivada de `plano-migrations.md` §9 e `proposta-paginas.md`; não é ordem nova.

- [ ] Fechar as lacunas pendentes com o dono nomeado em `lacunas.md` (bloqueantes primeiro).
- [ ] Escrever as 27 páginas de migration restantes sobre DDL fechado, sem inventar coluna.
- [ ] Fechar o DDL das 77 tabelas restantes em `ddl-confirmado.md` antes da página.
- [ ] Escrever `specs/` (requisitos, regras, NFR, casos de uso, fluxos, estados, rastreabilidade).
- [ ] Escrever as 17 páginas de domínio do dicionário e o gerador das 91 páginas.
- [ ] Escrever `planos/` (testes, dados, qualidade, métricas, riscos, execução).
- [ ] Escrever `guia-administrador/`, `guia-documentacao/` e o restante de `guia-teste/`.
- [ ] Criar `tests/` e `prompts/`/`skills/` do sistema.
- [ ] Validar o site: `mkdocs build --strict` sem link morto nem diagrama inválido.

**Gate:** 29 páginas de migration + 18 de domínio + 91 geradas; `planos/metricas.md` mede o
fechamento; nenhuma afirmação de comportamento sem `V`; site estrito passa.

### Fase 3 — Planejar os 12 sistemas restantes

- [ ] Aprovar o molde de planejamento por sistema (o de `wyd-db` é a referência, não o texto).
- [x] Escolher o segundo piloto por dependência: **`wyd-protocol`**, decidido em 2026-11-19;
      `wyd-docs` também foi aberto para publicar a documentação oficial.
- [ ] Para cada sistema: dashboard `<sistema>-1.0.0.md`, `docs/` com público e pergunta por página,
      `src/` por unidade de responsabilidade, `tests/` por alvo, `prompts/`, `skills/`.
- [ ] Definir a comunicação entre sistemas (protocolo, mensagens, dono do contrato) antes de planejar
      o interior de cada um.
- [ ] Mapear o que cada sistema consome do `wyd-db` por porta, não por tabela.

**Gate:** todo sistema tem fronteira, entradas/saídas, requisitos com ID, plano de teste e roadmap
com DoD; nenhum sistema depende de tabela de outro sem contrato.

### Fase 4 — Implementação

- [ ] Criar o workspace Rust na raiz com `Cargo.toml`, `rust-toolchain.toml` e `Makefile`.
- [ ] Implementar na ordem de dependência, uma issue por PR, TDD, cobertura ≥ 85%.
- [ ] Substituir o CI stub por workflows reutilizáveis + actions compostas.
- [ ] Rodar o corte vertical do `wyd-db` (`0001`–`0007`, 21 tabelas) antes dos 22 arquivos restantes.
- [ ] Provar isolamento entre `portal` e `game` por `permission denied`, não por revisão.

**Gate:** `fmt`, `clippy -D warnings`, testes, cobertura, migrations e build verdes em PR; E2E do
sistema executando contra Postgres real.

### Fase 5 — Operação

- [ ] Ambiente local em Docker Compose; CI efêmero em runner hospedado.
- [ ] Homologação em free tier só para portal e banco, com dado sintético.
- [ ] Produção com servidor de jogo em máquina própria/VPS; free tier não serve (hiberna).
- [ ] Observabilidade e alertas definidos antes do primeiro deploy.

**Gate:** nenhum segredo em repositório; deploy reproduzível; rollback ensaiado; alerta testado.

---

## 5. O que este documento não decide

- Não decide arquitetura de nenhum sistema; o dono é o plano diretor de cada um.
- Não decide prioridade de produto nem data de release.
- Não fecha lacuna: lacuna é fechada em `lacunas.md`, pelo dono nomeado.
- Não cria ADR: ADR nasce de decisão tomada, com “como verificar”.
- Não define o molde dos 12 sistemas restantes; propõe que o de `wyd-db` seja a base.
- Não escolhe hospedagem: a proposta está em `wyd-ops/proposta-gestao-github-e-ambientes.md` e
  depende de aprovação.
- Não mede qualidade de código: não há código de produção para medir.

---

## 6. Próximos passos priorizados por ganho ÷ esforço (2026-11-19)

Ordem por **desbloqueio ÷ esforço**, não pela sequência das fases de §4. `automação` significa
executável agora, sem decisão pendente; `usuário` significa que só o dono decide ou autoriza. Todo
item termina num resultado observável — nenhum é "revisar" ou "amadurecer". O único passo que publicou foi o 2,
dentro da janela declarada pelo dono.

| # | Passo | Desbloqueia | Por que é rápido (o que já existe) | Pré-requisito | Executa | Termina quando |
|---:|---|---|---|---|---|---|
| 1 | Capturar o marcador público atual e registrar o custo de publicação | dimensiona a janela de congelamento e fixa o baseline `D0` do conteúdo público antes de congelar | reutilizou runs e deployments já identificados, os horários de `created_at`/status final e o marcador das duas árvores | nenhum | automação | **feito** (`V`, 2026-11-19): `assets/site.js` respondeu 200 com a URL literal do Highlight.js e as durações #4/#6/#8 foram 49/43/29 s; o registro está em `wyd-docs-1.0.0.md` §7.3 |
| 2 | Declarar a janela sem publicação e executar o checklist de rollback | fecha `LPDOC-01`, a lacuna que segura a publicação contínua do site | os dois runs estão elegíveis, o build dos dois commits passou em Linux (`V`, §7.3) e o checklist de 10 passos já está escrito | declaração de congelamento do dono | usuário declara; automação executa | **feito** (`V`, 2026-11-19): #2 publicou `2378b62b` (`npm/highlight.js`), #4 restaurou `0d765858` (`cdn-release`); deployments `7911886524` e `7911904845` `success`; `LPDOC-01` fechada |
| 3 | Decidir o conteúdo que fica público depois da restauração | evita manter o site atrás de `develop` sem decisão registrada | a consequência já estava no critério de parada de §7.3; a decisão coube em uma frase | nenhum | usuário | **feito** (`V`, 2026-11-19): decisão do dono — o site fica em `0d765858…` até o merge do PR, que publica o tip `aedd1f73…` de conteúdo idêntico; registrado em `wyd-docs-1.0.0.md` §7.3 (linha 3) e §9 |
| 4 | Fechar `LPDOC-12` fixando o build suportado em Linux/container | remove ambiguidade do build local; era a lacuna `P1` que faltava fechar | o erro `PosixPath` está reproduzido e o mesmo comando passa em container nos dois commits | nenhum | usuário (dono: operação) | **feito** (`V`, 2026-11-19): decisão do dono fixou Linux/container como build suportado, com Windows fora de escopo; `lacunas.md` fechou `LPDOC-12` e §7.1 registra que o build roda em Linux e que no Windows falha por `PosixPath` |
| 5 | Medir o comportamento real do CI `full-workflow` como baseline do plano do `wyd-ops` | dá base verificável para a Fase 4 (CI real) e tira a causa de `H` | runs, jobs e log já estavam acessíveis por leitura; faltava medir o efeito real do defeito de shell que `D3` apontava | nenhum | automação | **feito** (`V`, 2026-11-19): run #14 (`44505419869`) e job `Get Info` (`135495629487`) `success`, saída `version`/`commit` vazias; registrado em `wyd-ops` §2 e §2.1 |

**Próxima ação imediata (uma só): publicar os arquivos pendentes por PR.** O passo 2 foi executado em
2026-11-19 e fechou `LPDOC-01`; o site está em `0d765858…` e `develop` no tip. O que sobra é o que só
o dono libera: commit, push e PR sob o ruleset de `develop`, cujo merge dispara `docs` e cria um
deployment novo — sem risco de conteúdo, porque as duas versões publicam os mesmos bytes. Os passos 3
e 4 e a `LPDOC-12` já foram decididos e não publicam nada.
