# AGENTS.md — WYD 1.0.0 (Rust)

Instruções para agentes de IA e para quem entra no repositório sem contexto. Este arquivo vale para
todo o repositório. Ele **não** repete regra de sistema: aponta o dono.

**Estado:** planejamento da `1.0.0`, sem código de produção. Medido em **2026-11-19**; estado dos
sistemas e dos ADRs de §4–§5 recontado em **2026-11-25**.

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
| Sistemas | a lista está em §6, que cita o dono; o estado de planejamento de cada um e as contagens por sistema estão em `planejamento-1.0.0/wyd-1.0.0.md` §1 e §7 |
| `wyd-db` | piloto parcial; arquivos, páginas com cabeçalho e migrations documentadas em `planejamento-1.0.0/wyd-1.0.0.md` §1; os ADRs aceitos deste sistema estão no índice de `planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/index.md` §2 |
| `wyd-protocol` | 2º piloto aberto; teto de páginas em `planejamento-1.0.0/wyd-protocol/wyd-protocol-1.0.0.md` §4, escritas e lacunas em `planejamento-1.0.0/wyd-1.0.0.md` §1 e `planejamento-1.0.0/wyd-protocol/lacunas.md` §1; nenhum ADR |
| `wyd-docs` | site MkDocs 2.0 em `wyd-docs/`: páginas montadas e censo por estado em `wyd-docs-1.0.0.md` §9.1 (2026-11-22), build e verificação verdes; lacunas com estado e dono em `planejamento-1.0.0/wyd-docs/lacunas.md` §1 |
| Lacunas do `wyd-db` | registradas por estado em `planejamento-1.0.0/wyd-db/lacunas.md` §1 |
| Código Rust | primeiro slice em `crates/wyd-db/`: `Cargo.toml`, `src/lib.rs`, `src/dsn.rs` e `tests/sistema_pg.rs` (19 testes locais); os outros 8 `crates/*` só têm `.gitkeep`; `deprecated/crates/` segue **ignorado pelo git** (contagem em `planejamento-1.0.0/wyd-1.0.0.md` §1.4) |
| Raiz | sem `Cargo.toml` de workspace, `rust-toolchain.toml` ou `Makefile`; o crate tem `crates/wyd-db/Cargo.toml` próprio |
| GitHub | público, `develop` padrão, Projects habilitado; issues e milestones em `planejamento-1.0.0/wyd-1.0.0.md` §1.4; projeto `WYD 1.0.0` com os campos, as views e as dependências `blocked by` medidos em `planejamento-1.0.0/wyd-ops/proposta-gestao-github-e-ambientes.md` §10.3 |
| CI | o gate de documentação roda em `.github/workflows/docs.yml` (PR e push em `develop`/`main`, disparado por qualquer mudança na árvore `planejamento-1.0.0/` ou em `wyd-docs/`, sem lista de arquivos — 2026-11-22); `code.yml` roda fmt, clippy, os testes unitários e de schema contra PG17 migrado, cobertura informativa sem `--fail-under` e `cargo audit`/`cargo deny` bloqueantes com `deny.toml`; `full-workflow.yml` segue stub de versão. As contagens de teste e o limiar de cobertura têm dono em `planejamento-1.0.0/wyd-ops/docs/planos/estrategia-de-testes.md` §6–§7 (execução, job e log em `planejamento-1.0.0/wyd-ops/` §2.1) |
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
3. **ADR só existe quando uma decisão foi tomada.** O índice lista os ADRs aceitos, com estado e data,
   em `planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/index.md`; esta seção não repete a contagem. Não transforme
   prática do legado em ADR e não numere decisão futura. Decisão sem “sim” do dono é `proposta`, não
   `aceito`.
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

## 6. Os sistemas

Cada pasta de `planejamento-1.0.0/` é um sistema e tem planejamento próprio: dashboard
`<sistema>-1.0.0.md`, `docs/` e `lacunas.md`, inclusive `wyd-ops`. **A lista dos sistemas, a
responsabilidade de cada um e as contagens por pasta têm dono**:
`planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md` §3 (fronteira e mapa por módulo),
`planejamento-1.0.0/wyd-1.0.0.md` §1 e §7 (contagens e estado) e `<sistema>/lacunas.md` §1 (lacunas).
Esta seção não repete a tabela; a árvore reservada de cada pasta é §2 do documento de estrutura, e
onde mora cada artefato é §8 do panorama. **Abertura inicial** (2026-11-22) significa fronteira,
contratos e lacunas nomeadas — sem requisito numerado, sem roadmap com DoD e sem código. Nenhum
sistema está vazio; nenhum está `aceito`.

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

Toda página nova passa pelo gate de aceite do sistema. O formato — cabeçalho, linha de estado,
evidência em toda afirmação, links, dono — e a lista do que reprova uma página são de
`planejamento-1.0.0/wyd-db/docs/convencoes.md` §Cabeçalho de página, §Marcações de evidência e §O que
reprova uma página (`wyd-db-1.0.0.md` §10 aponta as mesmas seções); esta etapa cita, não recopia.

Página publicada no site passa também pelo gate de documentação. O comando único, o que ele mede e
os passos equivalentes no host — os mesmos que o CI executa — estão em
`planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md` §6 (Como verificar); esta etapa não
repete o comando.

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

- A política de teste — pirâmide e níveis, limiar de cobertura, alvo E2E por sistema e anti-padrões
  proibidos — é de `planejamento-1.0.0/wyd-ops/docs/planos/estrategia-de-testes.md` §2, §6 e §8. O
  agente lê a seção e aplica; não recria o critério aqui.
- Dado de teste tem factory, sem id mágico, `sleep` como sincronização ou estado global entre testes
  (centralização em §4 do mesmo plano). O dono da estratégia de `wyd-db` é `docs/guia-teste/`.

---

## 9. GitHub, CI e operações

**Sem arquivo no repositório:** `mkdocs.yml` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

- **Público e gratuito.** Runners padrão do GitHub são gratuitos e ilimitados em repositório
  público; **não** use runner self-hosted em job que executa código de pull request — o próprio
  GitHub desaconselha em repositório público.
- CI é feito de workflows reutilizáveis e actions compostas em `.github/actions/`, com permissões
  mínimas, actions fixadas e cache. O fluxo de documentação (`.github/workflows/docs.yml`) já roda o
  gate em PR e push; o CI de código tem `code.yml` (fmt, clippy, testes, cobertura informativa e audit/deny bloqueantes) e `full-workflow.yml` ainda é stub de versão, descritos em `planejamento-1.0.0/wyd-ops/`.
- O site não se configura por `mkdocs.yml`: versão, build, tema e verificação têm dono em
  `planejamento-1.0.0/wyd-docs/wyd-docs-1.0.0.md` §9 e `wyd-docs/docs/convencoes.md`. Não crie
  `mkdocs.yml`; tema e montagem vivem em `wyd-docs/`.
- Segredos só em secret do GitHub ou `.env` local ignorado; nunca em workflow, log ou documento.
- Deploy, release, push e criação de recurso no GitHub exigem pedido explícito.
- Free tier de hospedagem serve para desenvolvimento/homologação, **nunca** para produção de jogo:
  os planos gratuitos hibernam, expiram ou têm compute insuficiente.

---

## 10. Manutenção deste arquivo

Atualize quando um fato medido mudar (sistema planejado, ADR aceito, CI funcionando, issue criada),
sempre com data. Não copie tabela de outro documento: aponte o dono. Este arquivo é normativo para
agentes; divergência entre ele e o planejamento é defeito a corrigir aqui.
