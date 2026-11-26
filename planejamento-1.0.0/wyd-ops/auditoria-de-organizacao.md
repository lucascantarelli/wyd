# Auditoria de organização definida — D-04 = A

> Pergunta: "quais sistemas já têm organização definida sob o checklist D-04 = A e o que falta em cada um?"
> Público: dono do produto, arquitetura, `wyd-ops` e equipes dos 13 sistemas
> Dono: `wyd-ops` (auditoria); a decisão D-04 é do dono do produto
> Base: `wyd-1.0.0.md` §8.7; `prioridade-do-projeto.md` §0.1; `AGENTS.md` §5
> Revisado em: 2026-11-26

**Estado:** auditoria executada em **2026-11-25**; D-04 = **A** aprovada. **Revisão 2026-11-25 (segunda passada):** os 11 sistemas pendentes receberam a árvore `src/`/`tests/` (`.gitkeep`), `docs/arquitetura/componentes.md`, `docs/planos/plano-de-testes.md`, `docs/specs/requisitos.md` (RF/RNF numerados) e roadmap com DoD; **13 de 13** passam no checklist, com `wyd-ops` ainda com fronteira/contratos pendentes (fechado na quarta passada). **Terceira passada (2026-11-25):** as pastas vazias `wyd-db/prompts/` e `wyd-db/skills/` receberam `.gitkeep`, cada `docs/index.md` passou a alcançar todas as páginas da pasta (0 órfãos medidos), os quatro templates de issue exigem `skill_principal` em lista fechada de nove skills, os índices locais de ADR passaram a `aceito` e os 13 planos de teste declaram CI, cobertura e monitoramento. **Quarta passada (2026-11-26):** `OPS-03.4`/`OPS-03.5` foram formalizados em `ADR-OPS-0011` e no `ADR-OPS-0007` revisado; `OPS-03` fechou e restam `OPS-04`/`OPS-05`. Nenhum código, issue, commit ou deploy foi criado por esta auditoria.

---

## 1. Critério aplicado

O checklist é o de `wyd-1.0.0.md` §8.7. Um sistema só é declarado **organizado** quando tem, no
documento dono:

1. fronteira em uma página: o que faz, o que não faz e o dono de cada exclusão;
2. contratos com forma, direção, erro, timeout e dono;
3. árvore de `src/`, `tests/` e operação — ou ausência justificada de operação própria;
4. requisitos com ID e critério de aceitação verificável;
5. plano de teste com alvos, dado, isolamento e comando;
6. roadmap com DoD executável por issue;
7. dashboard que mede páginas, requisitos, issues e gate;
8. comando de gate que falha de verdade.

**Marcas:** `sim` = existe e é verificável; `parcial` = existe incompleto ou só como lista de
próximas páginas; `não` = ausente. O veredito não mede implementação: um sistema organizado ainda
pode ter código vazio, e um sistema implementado (`wyd-docs`) ainda pode reprovar no checklist.

## 2. Resultado por sistema

| Sistema | Fronteira | Contratos | Árvore | RF/RNF | Teste | Roadmap | Gate | Dashboard | Veredito |
|---|---|---|---|---|---|---|---|---|---|
| `wyd-db` | sim | sim | sim | sim (18/14) | sim | sim (42) | sim | sim | **organizado** |
| `wyd-admin` | sim | sim | sim | sim (15/5) | sim | sim (10) | sim | sim | **organizado** |
| `wyd-portal` | sim | sim | sim | sim (35/10) | sim | sim | sim | sim | **organizado** |
| `wyd-protocol` | sim | sim | sim | sim (6/6) | sim | sim | sim | sim | **organizado** |
| `wyd-net` | sim | sim (6 decididos) | sim | sim (8/6) | sim | sim | sim | sim | **organizado** |
| `wyd-core` | sim | sim | sim | sim (8/6) | sim | sim | sim | sim | **organizado** |
| `wyd-server` | sim | sim | sim | sim (7/6) | sim | sim | sim | sim | **organizado** |
| `wyd-sim` | sim | sim | sim | sim (7/6) | sim | sim | sim | sim | **organizado** |
| `wyd-launcher` | sim | sim | sim | sim (8/5) | sim | sim | sim | sim | **organizado** |
| `wyd-client` | sim | sim | sim | sim (7/5) | sim | sim | sim | sim | **organizado** |
| `wyd-bots` | sim | sim | sim | sim (6/5) | sim | sim | sim | sim | **organizado** |
| `wyd-docs` | sim | sim | sim (site implementado) | sim (7/5) | sim | sim | sim | sim | **organizado** |
| `wyd-ops` | sim | sim | sim | sim (8/7) | sim | sim | parcial | sim | **organizado (gate parcial; implementação pendente)** |

**Somas:** 13 organizados (12 plenos + `wyd-ops` com gate parcial e implementação pendente; os
contratos O1–O5 estão aprovados). `wyd-db` e `wyd-admin` seguem como referência do checklist.

## 3. O que falta em cada sistema (fora da organização)

A tabela abaixo não repete o checklist fechado em §2. Ela lista o que ainda falta **depois** do carimbo de organização: captura, medição, implementação ou aprovação. Nenhum item desta tabela reabre o D-04.

| Sistema | O que já existe | O que ainda falta (não é organização) |
|---|---|---|
| `wyd-db` | fronteira, contratos, 18 `RF` + 14 `RNF`, 37 páginas de `src/`, 12 de `tests/`, 42 de roadmap, gate | **141** atos de `docs/` pendentes de aceite editorial; seis seeds de conteúdo bloqueados; código limitado às páginas `aceitas`. |
| `wyd-admin` | fronteira, contrato C4, 15 `RF` + 5 `RNF`, árvore de `src/`/`tests/` em `componentes.md`, plano de testes, 10 fatias com DoD, gate | Revisão do contrato pelo BFF do portal; implementação do canal de controle. |
| `wyd-portal` | fronteira, contratos, 33 `RF` + 10 `RNF`, roadmap com fatias, gate, 9 páginas de `docs/`, árvore e plano de teste na segunda passada | BFF, agendador, alerta e schema de leilão/armaria não implementados. |
| `wyd-protocol` | fronteira, 6 contratos de fio, 6 páginas de `docs/`, roadmap de fases, gate, `RF-PRT`/`RNF-PRT` e árvore na segunda passada | `LPRT-03` e `LPRT-06` dependem de captura A1; 5 de 131 páginas escritas. |
| `wyd-net` | fronteira, 7 contratos (6 decididos), 3 páginas, gate, `RF-NET`/`RNF-NET`, árvore, teste e roadmap na segunda passada | `LNET-01` continua medição; tarefa em `docs/planos/medicoes-lnet-01-lbot-03.md`. |
| `wyd-core` | fronteira, 6 contratos (C4 decidida), 4 páginas, gate, `RF-CORE`/`RNF-CORE`, árvore, teste e roadmap na segunda passada | Catálogo funcional do primeiro domínio; fórmulas/fixtures do `wyd-sim`. |
| `wyd-server` | fronteira, 7 contratos, 4 páginas, spec do ECAP, gate, `RF-SRV`/`RNF-SRV`, árvore, teste e roadmap na segunda passada | A1 e o peer continuam implementação pendente. |
| `wyd-sim` | fronteira, 4 contratos, 3 páginas, gate, `RF-SIM`/`RNF-SIM`, árvore, teste e roadmap na segunda passada | Fórmulas e fixtures determinísticas não escritas. |
| `wyd-launcher` | fronteira, 6 contratos (4 decididos), 3 páginas, gate, `RF-LCH`/`RNF-LCH`, árvore, teste e roadmap na segunda passada | Artefato de release (instalador/updater) não produzido. |
| `wyd-client` | fronteira, 5 contratos, 5 páginas, inventário e registro medidos, gate, `RF-CLI`/`RNF-CLI`, árvore, teste e roadmap na segunda passada | `LCLI-03`/`LCLI-05` e a segunda leitura seguem abertas; `LCLI-02` fechou em 2026-11-25; distribuição congelada. |
| `wyd-bots` | fronteira, 5 contratos (B1–B5), 3 páginas, gate, `RF-BOT`/`RNF-BOT`, árvore, teste e roadmap na segunda passada | `LBOT-03` continua medição; tarefa em `docs/planos/medicoes-lnet-01-lbot-03.md`. |
| `wyd-docs` | site implementado, contratos, gate de build/publicação, 5 páginas de `docs/`, `RF-DOC`/`RNF-DOC`, teste e roadmap na segunda passada | Aceite editorial e publicação contínua; nenhum deploy novo. |
| `wyd-ops` | dashboard, `RF-OPS`/`RNF-OPS`, plano de teste, roadmap, árvore e gate na segunda passada | Contratos O1–O5 aprovados (O4 em `ADR-OPS-0011`, O5 no `ADR-OPS-0007` revisado); CI, imagem, ambiente, monitoramento, segredo e deploy não implementados; `Q-OPS-2` não aplicado e board pendente. Ver `docs/lacunas.md`. |

## 4. O que a aprovação de D-04 = A muda

- **Muda o critério, não o estado:** “organizado” passa a ser um resultado auditável de planejamento,
  não sinônimo de “sistema aberto”. `wyd-db` e `wyd-admin` são o molde; os outros **11** receberam o
  carimbo na segunda passada (§2) e o que resta em cada um, fora da organização, está em §3.
- **Não autoriza código:** a árvore de `src/`/`tests/` é planejada antes do primeiro arquivo, mas a
  implementação continua subordinada à página `aceita` e ao gate do sistema (`AGENTS.md` §5, item 1).
- **Não muda D-01:** o foco continua finalizar o `wyd-db`. A auditoria só define o padrão que os
  outros sistemas devem cumprir quando forem trabalhados; não abre 11 frentes em paralelo.
- **Não muda D-02:** cobertura informativa no `code.yml` e segurança bloqueante continuam como estão.
- **Dá destino ao que estava aberto:** os **11** sistemas agora têm uma lista explícita do que falta;
  a próxima decisão é de prioridade (qual sistema auditar primeiro), não de critério. Os contratos
  O1–O5 do `wyd-ops` estão aprovados; o que resta é implementação e `OPS-04`/`OPS-05`.

## 5. Próximos passos por sistema (depois da organização)

| Ordem sugerida | Sistema | Ação | Por que primeiro |
|---:|---|---|---|
| 1 | `wyd-db` | fechar o aceite editorial dos 141 atos e revisar os seeds bloqueados | é o piloto e o foco D-01 |
| 2 | `wyd-server`/`wyd-protocol` | concluir A1, captura e peer; fechar `LPRT-03`/`LPRT-06` | destrava o fio e o runtime |
| 3 | `wyd-net`/`wyd-bots` | executar `docs/planos/medicoes-lnet-01-lbot-03.md` e congelar os valores | as políticas estão decididas; falta medição |
| 4 | `wyd-core`/`wyd-sim` | escrever o catálogo do primeiro domínio e as fórmulas/fixtures | a onda jogável depende das regras |
| 5 | `wyd-portal`/`wyd-admin`/`wyd-launcher`/`wyd-client` | implementar BFF, canal de controle, release e respeitar direitos | são frentes de produto e distribuição |
| 6 | `wyd-docs`/`wyd-ops` | aceite editorial, aplicação de `OPS-04` e board (`OPS-05`) — O1–O5 já aprovados | são processos; não bloqueiam a onda jogável, mas fecham operação e publicação |

## 6. Como reproduzir a medição

```bash
# 1. páginas, cabeçalhos e sistemas
find planejamento-1.0.0 -maxdepth 2 -name '*-1.0.0.md' | sort
grep -rl '^> Pergunta:' planejamento-1.0.0 --include='*.md' | wc -l

# 2. requisitos por sistema (o número do quadro)
grep -rhoE 'RF-[A-Z]+-[0-9]{3}' planejamento-1.0.0/<sistema> | sort -u | wc -l
grep -rhoE 'RNF-[A-Z]+-[0-9]{3}' planejamento-1.0.0/<sistema> | sort -u | wc -l

# 3. árvore e teste
find planejamento-1.0.0/<sistema> -maxdepth 2 -type d \( -name src -o -name tests -o -name roadmap \)
find planejamento-1.0.0/<sistema> -iname '*plano*teste*' -o -iname '*testes*'

# 4. gate do sistema e site
docker compose --profile gate run --rm gate
```

## 7. O que esta auditoria não decide

- Não aceita página, não abre issue e não cria código.
- Não escolhe qual frente vem primeiro; isso é prioridade do dono (D-01 continua `wyd-db`; D-06 = A foi decidida em 2026-11-25, com a frente `wyd-db`).
- Não fecha `LPRT-03`, `LPRT-06`, `LCLI-03`, `LCLI-05`, `LNET-01`, `LBOT-03`, `A11`, `A20` ou `M21`.
- Não substitui o checklist de `wyd-1.0.0.md` §8.7 nem a decisão D-04 em
  `prioridade-do-projeto.md` §0.1.

## 8. O que resta nas nove pendências bloqueadas (2026-11-25)

Esta seção foi reformulada contra os ledgers e o briefing: **não há aqui aprovação a repetir**. A1 foi
autorizada em 2026-11-23 e as políticas das dez linhas originais foram aceitas em 2026-11-25, na opção A
de cada bloco; `LCLI-02` tinha decisão de conteúdo (`Q-CLI-2`) e `LCLI-05` tem decisão de escopo, com o
pedido de direitos aprovado para envio. Em 2026-11-25 a execução avançou o que não dependia de terceiro:
uma linha fechou (§8.1) e duas foram provadas bloqueadas por artefato inexistente (§8.2). Restam nove
pendências: execução (§8.3), ato do dono que não é pergunta de desenho (§8.4), dependência de terceiro
(§8.5) e a única escolha real desta auditoria (§8.6).

### 8.1 Fechado em 2026-11-25 — `LCLI-02`

O patch da `1.0.0` está decidido e verificado: a lista fechada tem **três arquivos de tabela de
strings** com tamanho e `sha256` medidos (`wyd-client/registro-do-cliente.md` §5.1), a entrada existe em
`wyd-launcher/docs/specs/patch-1.0.0.json` no formato do `ADR-LCH-0001` e o verificador está em
`crates/wyd-launcher` (`src/manifest.rs`, `src/verify.rs`), com testes de hash divergente, tamanho
divergente e arquivo ausente (`tests/manifesto.rs`) e um teste que confere o manifesto real contra a
árvore local (`tests/patch_1_0_0.rs`). Nenhum binário de terceiro entra no pacote. A linha está
`fechada` em `wyd-client/lacunas.md` §2; a **distribuição continua congelada** por `LCLI-05`, porque a
lista é metadado e a evidência do titular ainda não existe.

### 8.2 Bloqueado por artefato que não existe — `LPRT-03` e `LPRT-06`

Tentei executar as duas capturas e o ambiente não as comporta; o bloqueio está medido, não suposto:

- o procedimento (`wyd-protocol/docs/goldens/como-capturar.md` §2, regra 8) exige `ECAP` + `PEER-A1`
  locais, e nenhum dos dois existe — `crates/wyd-server`, `crates/wyd-protocol` e `crates/wyd-net`
  contêm só `.gitkeep` (0 `Cargo.toml`, 0 `src`) e `wyd-client/tools/` não existe;
- sem endpoint operado, o cliente 7662 não tem para onde conectar: captura de handshake exige um peer do
  outro lado do socket, e é justamente o handshake que se quer medir (`registro-do-cliente.md` §7);
- `LPRT-06` depende do mesmo endpoint mais um personagem sintético criado em servidor, que também não
  existe.

Nenhuma medição foi inventada nem reaproveitada de número antigo; as duas linhas continuam
`bloqueada como prova`, com o bloqueio escrito na coluna de estado de `wyd-protocol/lacunas.md` §2.
**Quem destrava:** a frente `wyd-server`/`wyd-client` implementa `ECAP` + CLI (autorizado desde
2026-11-23); em seguida o operador nomeado em `LCLI-03` executa a captura e o QA designado faz a segunda
leitura.

### 8.3 Execução pendente — decisão dada, falta o artefato

| Linha | O que já está decidido (data) | Artefato que falta | Quem executa |
|---|---|---|---|
| `A11` | ASCII estrito `[A-Za-z0-9_]` em 1–16 bytes (2026-11-25) | captura `G-LOGIN` com acento, segunda leitura, golden, `CHECK` no DDL e teste de borda | `wyd-server` + `wyd-client`, no mesmo `ECAP` de §8.2 |
| `LNET-01` | `Q-NET-1` = A em `ADR-NET-0001` (aceito) e execução autorizada em 2026-11-25 | esqueleto do transporte, escada de carga, `medicoes-lnet-01-lbot-03.md` §4.6 preenchido e teste de recusa por motivo | QA + `wyd-ops`/`wyd-net` |
| `LBOT-03` | `Q-BOT-3` = A e execução autorizada em 2026-11-25 | esqueleto dos bots, execução datada e job de CI que reprova abaixo do congelado | QA + `wyd-ops`/`wyd-bots` |
| `A20` | MFA obrigatório para staff antes de publicar (2026-11-18, reafirmado em 2026-11-25); segue `externa` | ADR de segurança do portal, fluxo de inscrição/recuperação e teste que recusa staff sem segundo fator | `wyd-portal` |
| `M21` | fechar por fatia, rota a rota (2026-11-25); segue `externa` | telas das fatias `WYDPOR-F0`–`F11` com E2E verde e relatório de execução | `wyd-portal` |

Duas dessas execuções não podem começar hoje: `LNET-01`/`LBOT-03` esperam a implementação do ambiente
efêmero de `OPS-03.1`, cujo contrato já foi aprovado e está na
pauta em `docs/pauta-de-decisoes-pendentes.md`. As outras não esperam aprovação nenhuma.

### 8.4 Ato do dono — não é pergunta de desenho

| Ato | Por que não é escolha | Onde está |
|---|---|---|
| nomear o operador da primeira ficha de captura (`LCLI-03`) | a cadeia de aceite já está decidida — revisor **Lucas Cantarelli**, operador distinto — e falta só o nome; a ficha depende do `ECAP` de §8.2 | `wyd-client/lacunas.md` §4 |
| enviar o pedido ao titular (`Q-IP-1`/`Q-IP-2`, `LCLI-05`) | o pedido está redigido e aprovado para envio; falta confirmar signatário e abrir o ticket | `wyd-client/docs/specs/pedido-ao-titular.md` §7 |

### 8.5 Dependência de terceiro

- **Resposta do titular** (`Q-IP-1`/`Q-IP-2`): única saída de `LCLI-05`; sem ela não existe o parecer
  datado de `registro-do-cliente.md` §6 e a distribuição continua congelada.
- **`ECAP`/`PEER-A1`**: não é terceiro, é implementação nossa — mas é pré-requisito das capturas de
  §8.2 e da ficha de `LCLI-03`.

### 8.6 A escolha real do dono nesta auditoria

1. **Qual frente vem depois do `wyd-db` (`D-06`)?** Recomendação: implementar o `ECAP`/peer e concluir
   as capturas de §8.2, que destravam mais sistemas por esforço. Impacto: define a ordem das ondas; não
   fecha lacuna por si.

As demais decisões de dono do projeto (`OPS-04`, `OPS-05`, `D-06` e a `Q-SEP-1`) não pertencem a
esta auditoria; seu estado está na
pauta em `docs/pauta-de-decisoes-pendentes.md`. `Q-SERIE-1` = B foi implementada e provada como ciclo SQL
em PG17, `Q-SEP-1` = A′ renomeou os 28 nomes e `CAP-37` foi fechada por `runner_pg` no mesmo dia.

---

## 9. Decisão registrada — cobertura do classificador de citações (2026-11-25)

**DEC-CIT-01.** Não é ADR: não muda schema, fronteira nem operação; fica no documento dono, com ID
(`AGENTS.md` §7, Etapa 3).

**Contexto.** O classificador de citações (ferramenta de medição, fora do repositório) nasceu lendo
backticks e caminhos com `/`. Duas classes ficaram declaradas em prosa sem medição: caminho **sem
extensão** (`Makefile`, `Dockerfile`) e nome **não-ASCII** (`grão.json`); **não existe** nenhum
desses arquivos no repositório — são o vocabulário da ferramenta, não artefatos do projeto.

**Medição (V, 2026-11-25).** Fixture descartável com 18 defeitos — 10 de nome sem extensão em prosa,
tabela, lista, blockquote, ADR, `AGENTS.md`, `.agents/`, `SKILL.md` e cerca; 8 de nome não-ASCII nos
mesmos contextos — mais 3 controles que declaram a ausência. Antes do ajuste: **0/18 (0 %)**. Depois:
**18/18 (100 %)**, controles **3/3** não acusados. Revalidação dos quatro conjuntos anteriores, todos
em 100 %: 5 citações antigas (overlay `DEMO`), 10 defeitos da passagem anterior, fixture de
links/HTML/prosa com `/` (11/11) e fixture de caminhos nus sem `/` (13/13, já com um nome
não-ASCII entre os defeitos).

**Decisão.** (a) Nome sem extensão é extraído por **lista fechada** — `Makefile`, `Dockerfile*`,
`LICENSE`/`LICENCE`, `CODEOWNERS`, `Jenkinsfile`, `Procfile`, `Gemfile`, `Rakefile`, `Justfile`,
`rust-toolchain`, `.gitignore`, `.dockerignore`, `.editorconfig`, `.gitattributes`, `.mailmap` — nunca
por heurística de maiúscula, que acusaria nome próprio do texto. (b) Nome não-ASCII passa a ser
extraído como qualquer outro, com `\p{L}` na fronteira do token.

**Medição da forma `arquivo.md` sem barra (V, 2026-11-25).** Extraídas **146 citações** nessa forma
nos 613 documentos; **135 resolvem**, **11 estão declaradas** (`mkdocs.yml`, com nota no próprio
local) e **0 são acusadas**. Discriminador lexical de "exemplo de nome" (`exemplo`/`ilustra`/
`nome do arquivo`) encontraria **0 linhas** e foi **recusado**: esconderia citação real introduzida
por "por exemplo, ver …". Os exemplos de nome que existem hoje são resolvidos por outra via — série
placeholder (`0001-….md`), exemplo de sintaxe em code span (`` `](pagina.md)` ``) e nota de ausência.

**Custo medido.** Varredura do repositório: **124 menções** a nome sem extensão em documento; **0
acusações** — todas resolvem (`LICENSE` na raiz; `Makefile` e `Dockerfile.*` existem só em
`deprecated/`, contados como existentes pela resolução por basename) ou estão declaradas; nome
não-ASCII citado: **0**. Nome sem extensão **fora da lista** não é extraído: medido **0 caso** hoje,
e é o limite conhecido desta decisão — acrescentar nome novo à lista é uma linha no classificador,
não muda documento.
