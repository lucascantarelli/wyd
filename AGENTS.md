# AGENTS.md — WYD 1.0.0 (Rust)

Instruções para agentes de IA e para quem entra no repositório sem contexto. Este arquivo vale para
todo o repositório. Ele **não** repete regra de sistema: aponta o dono.

**Estado:** planejamento da `1.0.0`, sem código de produção. Medido em **2026-11-19**.

---

## 1. O que é este projeto

WYD é a reimplementação do MMORPG **WYD2** (distribuído no Brasil pela Ongame) em Rust, PostgreSQL e
Next.js, com launcher Tauri, documentação MkDocs e operação em Docker. O alvo é a versão `1.0.0`,
planejada sistema por sistema antes de qualquer implementação.

O repositório é **público** e todo o ferramental precisa caber no gratuito (GitHub Free, runners
hospedados, serviços sem cartão). A branch padrão é `develop`; `main` é release.

---

## 2. Ordem de leitura obrigatória

1. `planejamento-1.0.0/wyd-1.0.0.md` — panorama medido e plano de fases. **Comece aqui.**
2. `planejamento-1.0.0/<sistema>/` — plano do sistema em que você vai trabalhar.
3. Para `wyd-db` (piloto): `wyd-db-1.0.0.md` (plano diretor), `lacunas.md` (registro único de
   lacuna, dono e prazo) e `docs/convencoes.md` (contrato de leitura).
4. Referência, nunca fonte: `deprecated/`, `deprecated/docs/wyd/` e o projeto Go
   [w2pp-OpenWYD](https://github.com/Jean1dev/w2pp-OpenWYD).

Se o pedido não indicar o sistema, identifique-o antes de escrever qualquer arquivo.

---

## 3. Hierarquia de fontes de verdade

| Nível | Fonte | Como usar | Proibido |
|---|---|---|---|
| 1 | `planejamento-1.0.0/` | decide o que a `1.0.0` é | tratar como rascunho descartável |
| 2 | `lacunas.md` do sistema | dono único de lacuna, decisão pendente e marco | fechar lacuna sem o dono |
| 3 | ADRs aceitos (`docs/arquitetura/adr/`) | decisão estrutural vigente | criar ADR para “documentar” o legado |
| 4 | `deprecated/`, `deprecated/docs/wyd/`, `w2pp-OpenWYD` | entender comportamento e defeito | copiar texto, número, DDL, regra ou estrutura |
| 5 | `README.md` da raiz | resumo do repositório, reescrito em 2026-11-19 | usá-lo para contagem ou estado fino |

Regra de ouro: **o nível 4 inspira, o nível 1 decide.** Material do legado entra como hipótese `H`,
nunca como fato, e só vira regra depois de reverificado na fonte nova.

---

## 4. Estado medido (2026-11-19)

| Fato | Medição |
|---|---|
| Fase | planejamento; **nada em produção** e nenhum serviço no ar |
| Sistemas | 13 pastas em `planejamento-1.0.0/`; `wyd-db` (piloto), `wyd-protocol` (2º piloto) e `wyd-docs` têm documentação; `wyd-portal`, um prompt |
| `wyd-db` | 76 arquivos; 15 páginas com cabeçalho; 2 de 29 migrations documentadas; 2 ADRs aceitos |
| `wyd-protocol` | plano diretor com teto de 131 páginas, 2 escritas; 20 lacunas abertas; nenhum ADR |
| `wyd-docs` | site MkDocs 2.0 em `wyd-docs/`: 30 páginas montadas, build e verificação verdes; 14 lacunas |
| Lacunas do `wyd-db` | 29 registradas: 14 abertas, 3 externas, 12 fechadas |
| Código Rust | apenas em `deprecated/crates/` (375 arquivos), **ignorado pelo git**; `crates/*` só tem `.gitkeep` |
| Raiz | sem `Cargo.toml`, `rust-toolchain.toml` ou `Makefile` |
| GitHub | público, `develop` padrão, 0 issues abertas, 0 milestones, Projects do repositório desabilitado |
| CI | `.github/workflows/full-workflow.yml` tem só o job `get-info`, e ele está quebrado |
| Cliente | `wyd-client/files/` com 7.147 arquivos, ignorado pelo git |

Números vivem em `planejamento-1.0.0/wyd-1.0.0.md` §1, com comando e data. Não replique contagem em
outro documento: cite a seção.

---

## 5. Invariantes que não se negociam

1. **Planejar antes de codar.** Nenhum arquivo de código, migration ou teste nasce antes de a página
   que o descreve estar `aceita` e o gate do sistema fechar. O `deprecated` é o contraexemplo.
2. **Legado não é verdade.** Nada de `deprecated/` entra por cópia ou paráfrase. Se o dado não foi
   reverificado no planejamento novo, é `H`.
3. **ADR só existe quando uma decisão foi tomada.** Hoje há exatamente **dois** ADRs aceitos:
   `ADR-DB-0001` e `ADR-DB-0002`. Não invente `ADR-0003`, não transforme prática do legado em ADR,
   não numere decisão futura. Decisão sem “sim” do dono é `proposta`, não `aceito`.
4. **Toda afirmação carrega evidência:** `V` (verificado por comando/arquivo, com data), `H`
   (hipótese, com o ID da lacuna que a fecha) ou `P` (planejado). Presente do indicativo sem `V` é
   erro de escrita.
5. **Um fato, um dono.** Número, regra e DDL moram em um arquivo; os demais citam. Se duas páginas
   podem divergir sobre o mesmo número, uma está errada por construção.
6. **`planejamento-1.0.0/` é documentação.** Não crie `.sql`, `.rs`, `.ts`, `.py` nem script
   executável dentro dela. Artefato executável nasce em `crates/`, `wyd-portal/`, `wyd-docs/`,
   `tools/` ou `docker/`.
7. **Segredo nunca aparece.** DSN, token, hash, senha e PIN são descritos por tipo e lugar. Não leia
   `.env`, não cole valor em documento, log, issue ou chat.
8. **Não decida pelo time.** Melhoria, padrão novo, mudança de arquitetura, de escopo ou de
   nomenclatura é proposta com alternativas; implementar só depois de aprovação explícita.
9. **Não escreva fora do pedido.** Não crie issue, label, milestone, board, commit, push, deploy,
   release ou mensagem no GitHub/Slack sem pedido explícito. Análise e pesquisa não autorizam escrita.
10. **Estrutura antes de código.** A árvore de pastas de código e de testes é planejada por sistema
    antes do primeiro arquivo. Mudança de fronteira entre sistemas exige decisão registrada.

---

## 6. Os 13 sistemas

Cada pasta é um sistema e tem planejamento próprio (`docs/`, `src/`, `tests/`, `prompts/`,
`skills/`, dashboard `<sistema>-1.0.0.md`). Vazio significa **não planejado**, não “livre para
inventar”.

| Sistema | Responsabilidade | Estado do planejamento |
|---|---|---|
| `wyd-protocol` | pacotes CPSock, opcodes, offsets, goldens | **2º piloto**, plano aberto |
| `wyd-net` | transporte async, sessão, rate-limit, hardening | vazio |
| `wyd-core` | tick, ECS, combate, cena, IA | vazio |
| `wyd-server` | composição do servidor; um processo por canal | vazio |
| `wyd-sim` | fórmulas, curvas e simulações determinísticas | vazio |
| `wyd-db` | PostgreSQL 17, migrations, repositórios, filas | **piloto**, parcial |
| `wyd-admin` | administração, HTTP/JSON interno, staff | vazio |
| `wyd-portal` | portal Next.js: site, login, cadastro, BFF | só o prompt de produto |
| `wyd-launcher` | launcher Tauri: patch, manifesto, serverlist/itemlist | vazio |
| `wyd-client` | patches e arquivos de configuração do cliente | vazio (cliente em `wyd-client/files/`) |
| `wyd-bots` | bots de QA, carga e convivência | vazio |
| `wyd-docs` | site MkDocs 2.0 de toda a documentação oficial | **site implementado**; plano aberto |
| `wyd-ops` | CI, imagens, deploy, ambientes, monitoramento | proposta de gestão |

Dependência de leitura entre sistemas: `wyd-protocol` → `wyd-net` → `wyd-core` → `wyd-server`;
`wyd-db` serve `wyd-server`, `wyd-portal` e `wyd-admin`; `wyd-ops` opera todos. Essa ordem é hipótese
de planejamento (`H`), não decisão aprovada.

---

## 7. Fluxo de trabalho

### Etapa 0 — Entender e medir

Leia o panorama e o dashboard do sistema. Meça antes de afirmar:

```bash
git status --short --branch
find planejamento-1.0.0 -type f | sort
grep -rl '^> Pergunta:' planejamento-1.0.0/wyd-db/docs --include='*.md' | wc -l
grep -rlE 'ADR-DB-[0-9]{4}' planejamento-1.0.0 --include='*.md'
```

### Etapa 1 — Planejar

Escreva no sistema dono. Use a skill `planejar-sistema` para abrir sistema novo e
`auditar-planejamento` para revisar o que existe. Não crie padrão novo: siga o molde de `wyd-db` e as
regras de `planejamento-1.0.0/wyd-db/docs/convencoes.md`.

### Etapa 2 — Validar

Toda página nova passa pelo gate do sistema: cabeçalho de 5 linhas, estado com data, evidência em
toda afirmação, sem placeholder, sem segredo, link que resolve, dono único e revisão de quem não
escreveu. O plano diretor lista o que reprova uma página (`wyd-db-1.0.0.md` §10 e `convencoes.md`).

Página publicada no site também passa pelo gate de link do `wyd-docs`:

```bash
cd wyd-docs && .venv/Scripts/python.exe preparar.py
.venv/Scripts/mkdocs.exe build --dir .build/docs --output site
.venv/Scripts/python.exe verificar.py site
```

### Etapa 3 — Registrar decisão

Decisão estrutural: proposta → aprovação do dono → ADR com contexto, decisão, consequências,
alternativas e **como verificar** → linha fechada em `lacunas.md` com data e evidência. Decisão que
não muda schema, fronteira ou operação não é ADR: fica no documento dono, com ID.

### Etapa 4 — Implementar

Só depois do gate de planejamento. Código novo nasce com teste (TDD), roda o gate do sistema e
mantém a documentação dona atualizada na mesma entrega. Uma issue por vez, sem refatoração fora do
escopo.

---

## 8. Testes

- Cobertura mínima **85%** por sistema, medida no gate; cobertura de linha não substitui capacidade
  provada.
- Pirâmide: unitário → contrato → integração/sistema → E2E; nada de `SKIP` silencioso.
- E2E: `portal` e `client` com **Playwright**; `server`, `db`, `launcher`, `bots`, `core`,
  `protocol`, `sim`, `net` e `admin` com **Rust**.
- Dado de teste tem factory; proibido id mágico, `sleep` como sincronização e estado global entre
  testes. O dono da estratégia de `wyd-db` é `docs/guia-teste/` (parcialmente escrito).

---

## 9. GitHub, CI e operações

- **Público e gratuito.** Runners padrão do GitHub são gratuitos e ilimitados em repositório
  público; **não** use runner self-hosted em job que executa código de pull request — o próprio
  GitHub desaconselha em repositório público.
- CI é feito de workflows reutilizáveis e actions compostas em `.github/actions/`, com permissões
  mínimas, actions fixadas e cache. Estado atual é stub; o único fluxo previsto é o de documentação
  (`.github/workflows/docs.yml`), descrito em `planejamento-1.0.0/wyd-ops/`.
- O site usa **MkDocs 2.0** (`2.0.dev6`, pré-release) com Python 3.15: sem `mkdocs.yml`, plugin ou
  modo estrito, e o build não roda no Windows. Não crie `mkdocs.yml`; tema, montagem e verificação
  vivem em `wyd-docs/`.
- Segredos só em secret do GitHub ou `.env` local ignorado; nunca em workflow, log ou documento.
- Deploy, release, push e criação de recurso no GitHub exigem pedido explícito.
- Free tier de hospedagem serve para desenvolvimento/homologação, **nunca** para produção de jogo:
  os planos gratuitos hibernam, expiram ou têm compute insuficiente.

---

## 10. Manutenção deste arquivo

Atualize quando um fato medido mudar (sistema planejado, ADR aceito, CI funcionando, issue criada),
sempre com data. Não copie tabela de outro documento: aponte o dono. Este arquivo é normativo para
agentes; divergência entre ele e o planejamento é defeito a corrigir aqui.
