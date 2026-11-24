# AGENTS.md — WYD 1.0.0 (Rust)

Instruções para agentes de IA e para quem entra no repositório sem contexto. Este arquivo vale para
todo o repositório. Ele **não** repete regra de sistema: aponta o dono.

**Estado:** planejamento da `1.0.0`, sem código de produção. Medido em **2026-11-19**; estado dos
sistemas de §6 recontado em **2026-11-22**.

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
| Sistemas | os sistemas, um por pasta de `planejamento-1.0.0/`, e o estado de planejamento de cada um estão em §6; as contagens por sistema, em `planejamento-1.0.0/wyd-1.0.0.md` §1 |
| `wyd-db` | piloto parcial; arquivos, páginas com cabeçalho e migrations documentadas em `planejamento-1.0.0/wyd-1.0.0.md` §1; 2 ADRs aceitos (§5.3) |
| `wyd-protocol` | 2º piloto aberto; teto de páginas em `planejamento-1.0.0/wyd-protocol/wyd-protocol-1.0.0.md` §4, escritas e lacunas em `planejamento-1.0.0/wyd-1.0.0.md` §1 e `planejamento-1.0.0/wyd-protocol/lacunas.md` §1; nenhum ADR |
| `wyd-docs` | site MkDocs 2.0 em `wyd-docs/`: páginas montadas e censo por estado em `wyd-docs-1.0.0.md` §9.1 (2026-11-22), build e verificação verdes; lacunas com estado e dono em `planejamento-1.0.0/wyd-docs/lacunas.md` §1 |
| Lacunas do `wyd-db` | registradas por estado em `planejamento-1.0.0/wyd-db/lacunas.md` §1 |
| Código Rust | apenas em `deprecated/crates/`, **ignorado pelo git**; `crates/*` só tem `.gitkeep` (contagem em `planejamento-1.0.0/wyd-1.0.0.md` §1.4) |
| Raiz | sem `Cargo.toml`, `rust-toolchain.toml` ou `Makefile` |
| GitHub | público, `develop` padrão, Projects habilitado; issues e milestones em `planejamento-1.0.0/wyd-1.0.0.md` §1.4; projeto `WYD 1.0.0` (40 itens com os campos preenchidos, 5 views, dependências como `blocked by`) em `planejamento-1.0.0/wyd-ops/proposta-gestao-github-e-ambientes.md` §10.3 |
| CI | o gate de documentação roda em `.github/workflows/docs.yml` (PR e push em `develop`/`main`, disparado por qualquer mudança na árvore `planejamento-1.0.0/` ou em `wyd-docs/`, sem lista de arquivos — 2026-11-22); o CI de código ainda é stub: `full-workflow.yml` entrega `version`/`commit` vazios, sem build, teste ou cobertura (execução, job e log em `planejamento-1.0.0/wyd-ops/` §2.1) |
| Imagem e Compose | `.github/workflows/docs-image.yml` está versionado mas **nunca rodou e nunca publicou** (2026-11-23): o arquivo não está no remoto e o pacote no GHCR não existe. `docker compose pull docs` / `up -d docs` consomem a imagem publicada (`:latest`, com `build:` de fallback); `docker-compose.override.yml` (automático) traz só o serviço `gate` sob perfil e `docker-compose.dev.yml` (opt-in, `-f`) constrói a árvore com `:dev`. Build, fumaça e contagem provados localmente; `pull` reprova com exit 1 enquanto não houver pacote. Estado medido e o que falta para publicar em `planejamento-1.0.0/wyd-1.0.0.md` §8; decisão, DoD e medição em `planejamento-1.0.0/wyd-ops/plano-imagem-e-compose-do-site.md` |
| Cliente | `wyd-client/files/`, ignorado pelo git; contagem, tamanho e hash do binário medidos em `planejamento-1.0.0/wyd-client/registro-do-cliente.md` (2026-11-22) |

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
   executável dentro dela. Artefato executável nasce na pasta real do projeto, na raiz; qual pasta,
   qual sistema é dono e quais arquivos ficam na raiz por exigência de ferramenta estão em
   `planejamento-1.0.0/wyd-1.0.0.md` §8 — não repita a tabela aqui.
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

Cada pasta é um sistema e tem planejamento próprio: dashboard `<sistema>-1.0.0.md`, `docs/` e
`lacunas.md` — exceto `wyd-ops`, que ainda tem só `docs/` e propostas, sem plano diretor. As pastas
`src/`, `tests/`, `prompts/` e `skills/` só existem onde o dono abriu pergunta própria (hoje
`wyd-db` e `wyd-admin`); onde mora cada artefato é §8 do panorama. **Abertura inicial** (2026-11-22) significa fronteira,
contratos e lacunas nomeadas — sem requisito numerado, sem roadmap com DoD e sem código. Nenhum
sistema está vazio; nenhum está `aceito`.

| Sistema | Responsabilidade | Estado do planejamento |
|---|---|---|
| `wyd-protocol` | pacotes CPSock, opcodes, offsets, goldens | **2º piloto**, plano aberto; 2 lacunas de captura (`LPRT-03`, `LPRT-06`); A1 autorizada em 2026-11-23, captura pendente |
| `wyd-net` | transporte async, sessão, rate-limit, hardening | **aberto (2026-11-22)**: fronteira, 7 contratos (6 decididos) e 4 lacunas (3 fechadas em 2026-11-23, uma delas por regra vigente; 1 aberta como medição, com a política de `Q-NET-1` = A); sem código |
| `wyd-core` | tick, ECS, combate, cena, IA | **aberto (2026-11-22)**: fronteira, 6 contratos (C4 decidida em 2026-11-23) e 4 lacunas (0 abertas); escopo depende do catálogo funcional |
| `wyd-server` | composição do servidor; um processo por canal | **aberto (2026-11-22)**: fronteira, 7 contratos (o de controle decidido por `LAC-ADM-03`; `S7` de captura com A1 autorizada em 2026-11-23) e 5 lacunas (0 abertas; `LSRV-05` em C com a condição reprovada — peer de A1 pela opção A, implementação pendente) |
| `wyd-sim` | fórmulas, curvas e simulações determinísticas | **aberto (2026-11-22)**: fronteira, 4 contratos e 4 lacunas (**todas fechadas em 2026-11-23** por resposta explícita do dono: `LSIM-01`, `LSIM-02`, `LSIM-03` e `LSIM-04`); nenhuma fórmula ou fixture escrita |
| `wyd-db` | PostgreSQL 17, migrations, repositórios, filas | **piloto**, parcial |
| `wyd-admin` | administração, HTTP/JSON interno, staff | **planejado**: contrato C4 e custódia de item propostos (2026-11-22), com a autenticação portal → admin (token de serviço por ambiente) e o canal de controle do `wyd-server` (HTTP síncrono com ack) decididos na mesma data; nenhuma lacuna aberta; código vazio |
| `wyd-portal` | portal Next.js: site, login, cadastro, BFF | **planejado**: 11 páginas de planejamento, 33 `RF-POR`, armaria e mercado em F7–F11 (2026-11-22); **0 abertas, 0 propostas e 9 fechadas** (`LAC-POR-09` fechou em 2026-11-23: BFF fecha leilão, `wyd-ops` agenda/alerta), mas BFF, agendador, alerta e schema não implementados; código vazio |
| `wyd-launcher` | launcher Tauri: patch, manifesto, serverlist/itemlist | **aberto (2026-11-22)**: fronteira, 6 contratos e 4 lacunas (**0 abertas, 0 propostas, 4 fechadas**; `LLCH-01`–`LLCH-04` fecharam por decisão do dono em 2026-11-23) |
| `wyd-client` | patches e arquivos de configuração do cliente; frente de captura | **aberto (2026-11-22)**: fronteira, 5 contratos, 5 lacunas próprias (uso/distribuição **em proposta**, com responsável nominal, distribuição congelada, `Q-IP-3`–`Q-IP-6` decididas e `Q-IP-1`/`Q-IP-2` com pedido externo iniciado, `Q-IP-2` no escopo B, árvore inteira; inventário item a item (`docs/specs/inventario-da-arvore.md`) e pacote do pedido (`docs/specs/pedido-ao-titular.md`) prontos, pedido não enviado; `LCLI-02` aberta) + 3 medições herdadas; **registro medido** do cliente em `registro-do-cliente.md`; cliente em `wyd-client/files/`; A1 autorizada em 2026-11-23, CLI/peer pendentes; segunda leitura com QA definido (`LCLI-03`: Lucas Cantarelli, que não é o operador; operador na ficha) |
| `wyd-bots` | bots de QA, carga e convivência | **aberto (2026-11-22)**: fronteira, 5 contratos e 4 lacunas (**1 aberta como medição, 3 fechadas em 2026-11-23** — `LBOT-01` e `LBOT-02` por decisão do dono, `LBOT-04` por regra vigente; `LBOT-03` tem política decidida por `Q-BOT-3` = A, mas o número só fecha com a primeira execução) |
| `wyd-docs` | site MkDocs 2.0 de toda a documentação oficial | **site implementado**; plano aberto |
| `wyd-ops` | CI, imagens, deploy, ambientes, monitoramento | **propostas** de gestão de GitHub/ambientes e de versões/cobertura + imagem do site e Compose (2026-11-22); sem plano diretor |

Publicação no site é ato à parte: **arquitetura aprova** o sistema novo (nome, fronteira e entrada
em `SISTEMAS` de `wyd-docs/preparar.py`); **documentação publica**. Decisão de 2026-11-19
(`LPDOC-08`, fechada em `planejamento-1.0.0/wyd-docs/lacunas.md`).

Dependência de leitura entre sistemas: `wyd-protocol` → `wyd-net` → `wyd-core` → `wyd-server`;
`wyd-db` serve `wyd-server`, `wyd-portal` e `wyd-admin`; `wyd-ops` opera todos. O mapa vigente —
grafo, contratos com dono e estado, o que já pode ser ligado e as ondas de execução — é
`planejamento-1.0.0/wyd-1.0.0.md` §7 (2026-11-22); esta frase não é dona da dependência.

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

Página publicada no site também passa pelo gate de link do `wyd-docs`. Em um comando, no container —
o mesmo que o CI valida:

```bash
docker compose --profile gate run --rm gate
```

No host, sem container, são os mesmos três passos:

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
  mínimas, actions fixadas e cache. O fluxo de documentação (`.github/workflows/docs.yml`) já roda o
  gate em PR e push; o CI de código ainda é stub, descrito em `planejamento-1.0.0/wyd-ops/`.
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
