> Pergunta: "quem atende este requisito — tabela, porta, teste e issue?"
> Público: Desenvolvedor, revisor de migration, líder de onda e auditor de cobertura
> Dono: equipe do `wyd-db`
> Base: `requisitos-funcionais.md`; `regras-de-negocio.md`; `requisitos-nao-funcionais.md`; `casos-de-uso.md`; `../dicionario-dados/index.md`; `../planos/plano-de-documentacao.md`
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-24)

Esta página responde "quem atende este requisito?" para os **40** requisitos nomeados do banco: **17**
`RF`, **7** `RN`, **13** `RNF` e **3** `UC`. Em 2026-11-24, `Q-BLK-24` = A acrescentou `RNF-DB-006`,
`RNF-DB-007` e `RF-DB-017`, e `Q-BLK-19` = A acrescentou `RNF-DB-008` a `RNF-DB-013` — um por gate de
série que governava capacidade na matriz. Ela é matriz, não enunciado — o texto de cada requisito
continua na página dona, e a coluna, o tipo e o `CHECK` continuam no dicionário e na migration. A decisão
`Q-BLK-08` = A (2026-11-24) fixou a forma: **5** colunas, com `teste` como **alvo planejado** e `issue` como
"a alocar na onda", sem esperar a suíte das ondas 4–6.

## 1. Como ler a matriz

| Coluna | O que significa | Quem é o dono |
|---|---|---|
| `ID` | identificador do requisito, na série da página dona | página que o enuncia |
| `Tabela` | tabela que materializa o requisito; `—` quando a regra não vive no banco | dicionário e migration |
| `Porta` | porta do contrato C4 que expõe o dado; `—` quando não há porta | `../arquitetura/componentes-c4.md` |
| `Teste` | **alvo** do teste e sua natureza; `V` = **executado e verde** em 2026-11-24 (`sistema_pg`, §3.1); `P` = previsto — alvo escrito na matriz `CAP-nn`, teste **não executado** | `../planos/plano-de-testes.md`; `tests/specs/capacidades.md` |
| `Página` | a página de `tests/` que descreve o cenário que prova a linha; caminho de pasta, porque `tests/` não é publicada no site | a própria página de `tests/`; aqui é só o ponteiro |
| `Issue` | onda em que a issue do roadmap deve ser alocada | `../roadmap/` (40 issues) |

**14 das 37 células de `Teste` têm teste executado** desde 2026-11-25: as 13 capacidades `CAP-24`–`CAP-36`
rodaram em `crates/wyd-db/tests/sistema_pg.rs` (13 verdes, §3.1) e `CAP-37` rodou em
`crates/wyd-db/tests/runner_pg.rs` (5/5). As outras **20** seguem em `P`; as **3** restantes são `—` por
regra, porque não são teste do banco (`RF-DB-015` e `RF-DB-016`, frente `wyd-server`, e `RN-DB-003`, regra de
cultura). Das 20 com `P`, **16** têm `CAP-nn` escrita na matriz (`CAP-01`–`CAP-23`, `CAP-38`–`CAP-39`) e **4**
não (`RF-DB-003`, `RF-DB-006`, `RF-DB-014` e `RNF-DB-001`, que é gate de CI). As seis linhas de gate de série (`RNF-DB-008`–`RNF-DB-013`) seguem medidas em
[`../planos/plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) §1, sem alvo Rust.

**Células pendentes da matriz: 64** = **24** na coluna `Teste` (as que seguem em `P`) + **40** na coluna `Issue` (onda
sem número de issue). As **13** células de `Teste` em `V` saíram da conta; as **3** que sobram das 40 linhas
são `—` por regra, e não entram. Medido em 2026-11-24 por leitura das 40 linhas de §2 mais a execução de
`sistema_pg`; é este o número que as outras páginas citam. A medição é **0** de **40** issues criadas
(`../planos/riscos.md` `R-5`) e **1** alvo de sistema no repositório (13 cenários). A coluna não promete
cobertura; ela torna a ausência contável.

### 1.1 Onde a linha vai rodar — os alvos escritos

A coluna `Teste` usa a **natureza**; o alvo que a executa é um dos cinco escritos em `tests/alvos/` e
nomeados no `Cargo.toml` planejado (`tests/index.md` §2). A tabela liga as duas coisas, e não é uma
segunda matriz de capacidade: quem liga requisito a `CAP-nn` é `tests/specs/capacidades.md`.

**Natureza não basta para a coluna `Página`:** a página precisa **nomear** o requisito em uma `CAP-nn`;
onde ela não nomeia, o ponteiro vira `—` e o caso entra em §3.4. Foi o que a revisão de 2026-11-24
corrigiu em 15 linhas que apontavam `sistema-pg` só por serem de natureza `sistema`.

| Natureza na coluna `Teste` | Alvo escrito que executa | Página (coluna `Página`) | Observação |
|---|---|---|---|
| `sistema` | `sistema_pg` para as linhas que a matriz `CAP-nn` nomeia; `sistema_sessoes` quando a linha prova papel ou fronteira (`RF-DB-011`) | `tests/alvos/sistema-pg.md`; `tests/alvos/sistema-sessoes.md` em `RF-DB-011`; `—` quando nenhuma `CAP-nn` nomeia a linha | a natureza sozinha não decide: quem decide é o cenário escrito |
| `integração` | `sistema_pg` quando a matriz `CAP-nn` nomeia a linha (`RF-DB-005`, `RF-DB-009`) | `tests/alvos/sistema-pg.md` em todas as linhas de `integração` (`RF-DB-007` e `RF-DB-008` ganharam `CAP-27` e `CAP-28`) | não existe alvo `integração` entre os cinco; a verificação roda no alvo de sistema que a cobre |
| `contrato` | `contrato_migrations` ou `contrato_catalogo` | `tests/alvos/contrato-migrations.md` ou `tests/alvos/contrato-catalogo.md` | nenhuma linha usa `contrato` hoje |
| `gate` | `sistema_stack_caps` (`RNF-DB-003`), o gate de cobertura no CI (`RNF-DB-001`) e o gate de série de `RNF-DB-008` a `RNF-DB-013` | `tests/alvos/sistema-stack-caps.md`; `—` em `RNF-DB-001` e nos seis de série | `RNF-DB-001` não é alvo Rust e os seis de série são conferência do gate (`plano-de-qualidade-e-gates.md` §1) |
| `instrumentação` | `sistema_pg` (`CAP-19`, `CAP-38`) | `tests/alvos/sistema-pg.md` | espera por conexão e reuso, `RNF-DB-007`; p95 medido e registrado, `RNF-DB-004` |
| `boot` | `sistema_pg` (`CAP-18`, `CAP-39`) | `tests/alvos/sistema-pg.md` | versão única de conteúdo, `RF-DB-017`; tempo de boot medido e registrado, `RNF-DB-005` |

**O caminho de cada linha, em um salto:** a coluna `Página` é o ponteiro materializado desta tabela —
**29** linhas apontam `tests/alvos/sistema-pg.md` (as **13** de antes — `RF-DB-003`, `005`, `006`, `009`,
`014`, `017`; `RN-DB-001`–`005`; `RNF-DB-006`, `007` — mais as **16** de 2026-11-24: `RF-DB-001`, `002`,
`004`, `007`, `008`, `010`, `012`, `013`; `RN-DB-006`, `007`; `UC-DB-001`–`003`; `RNF-DB-002`, `004`, `005`),
**1** aponta `tests/alvos/sistema-sessoes.md` (`RF-DB-011`), **1** aponta `tests/alvos/sistema-stack-caps.md`
(`RNF-DB-003`) e **9** não têm página: **2** de outro sistema (`RF-DB-015` e `RF-DB-016`, `wyd-server`), **1**
de gate de CI (`RNF-DB-001`) e **6** de gate de série (`RNF-DB-008`–`013`). Nenhuma das 40 usa as páginas de
contrato: as duas provam a **forma** da série, não um requisito nomeado.

**O que falta para `P` virar nome de teste:** o código dos outros quatro alvos. O aceite das 12 páginas de
`tests/` existe desde 2026-11-24 (`Q-BLK-21` = A); em 2026-11-24 `sistema_pg` nasceu e as 13 linhas de
`CAP-24`–`CAP-36` viraram `V`. `crates/wyd-db/` tem `Cargo.toml`, `src/dsn.rs` e `tests/sistema_pg.rs`; os
outros quatro alvos e `tests/common/` seguem vazios. Até o código nascer, a promessa é a página que **nomeia**
a linha em uma `CAP-nn`; onde não há, a coluna diz `—` e o caso está em §3.4.

## 2. A matriz

### 2.1 Requisitos funcionais — 17

| ID | Enunciado (resumo) | Tabela | Porta | Teste | Página | Issue |
|---|---|---|---|---|---|---|
| `RF-DB-001` | login de 1 a 16 caracteres | `game.account` | `accounts` | sistema · `V` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RF-DB-002` | falha de PIN acumula e bloqueia | `game.account` | `accounts` | sistema · `V` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RF-DB-003` | saldo da conta em inteiro de 32 bits | `game.account_wallet` | `wallet` | sistema · `P` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RF-DB-004` | cofre pertence à conta | `game.vault_item` | `inventory` | sistema · `V` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RF-DB-005` | um resgate diário por conta e dia | `portal.daily_reward_claim` | `rewards` | integração · `P` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RF-DB-006` | uma sessão viva por conta | `game.session` | `session` | sistema · `P` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RF-DB-007` | bloqueio exige motivo, autor e instante | `game.account_moderation` | `moderation` | integração · `V` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RF-DB-008` | crédito e débito de donate idempotentes | `portal.donate_credit`/`donate_debit` | `donate` | integração · `V` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RF-DB-009` | grant pendente impede apagar conta | `ops.delivery_queue` | `delivery` | integração · `P` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RF-DB-010` | token de reset só existe como hash | `portal.person_token`, `game.pin_reset` | `identity` | sistema · `V` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RF-DB-011` | auditoria append-only, sem FK | `ops.audit_log` | `audit` | sistema · `P` | `tests/alvos/sistema-sessoes.md` | onda 8 |
| `RF-DB-012` | papel de jogo é fechado | `game.account` | `accounts` | sistema · `V` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RF-DB-013` | senha e PIN só como hash argon2id | `portal.person`, `game.account` | `identity` | sistema · `V` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RF-DB-014` | cinco contas da mesma pessoa online | `game.session` | `session` | sistema · `P` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RF-DB-015` | contas da mesma pessoa trocam entre si | — (regra de jogo) | — | — (frente `wyd-server`) | — (frente `wyd-server`) | onda 8 |
| `RF-DB-016` | limite por IP comporta cinco clientes | — (`cpsock/limits.rs`) | — | — (frente `wyd-server`) | — (frente `wyd-server`) | onda 8 |
| `RF-DB-017` | uma versão corrente de conteúdo, referenciada por FK | `content.content_version` | — | sistema · `P` | `tests/alvos/sistema-pg.md` | onda 8 |

### 2.2 Regras de negócio — 7

| ID | Enunciado (resumo) | Tabela | Porta | Teste | Página | Issue |
|---|---|---|---|---|---|---|
| `RN-DB-001` | vaga de conta é 1..5, a fechada é reutilizada | `game.account` | `accounts` | integração · `P` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RN-DB-002` | uma sessão por conta e por personagem | `game.session` | `session` | integração · `P` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RN-DB-003` | cinco contas por pessoa, sem barreira por IP | — (o banco permite) | — | — (regra de cultura) | `tests/alvos/sistema-pg.md` | onda 8 |
| `RN-DB-004` | fechar pessoa anonimiza o e-mail | `portal.person` | `identity` | integração · `P` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RN-DB-005` | moeda é inteira de 32 bits | `game.account_wallet`, `game.account_ledger` | `wallet` | integração · `P` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RN-DB-006` | bloqueio é moderação, não ciclo de vida | `game.account_moderation`, `game.account` | `moderation` | integração · `V` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RN-DB-007` | limiar de PIN é política, o estado é do banco | `game.account` | `accounts` | sistema · `V` | `tests/alvos/sistema-pg.md` | onda 8 |

### 2.3 Requisitos não funcionais — 13

| ID | Enunciado (resumo) | Tabela | Porta | Teste | Página | Issue |
|---|---|---|---|---|---|---|
| `RNF-DB-001` | cobertura de linha ≥ 85% no código do banco | — | — | gate · `P` | — (gate de CI) | onda 8 |
| `RNF-DB-002` | espera máxima por DDL de `10 s`, com dois processos | — | — | sistema · `P` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RNF-DB-003` | teste que não roda é falha, não passa | — | — | gate · `P` | `tests/alvos/sistema-stack-caps.md` | onda 8 |
| `RNF-DB-004` | p95 da leitura de personagem, **a medir** | — | — | instrumentação · `P` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RNF-DB-005` | tempo de boot, **a medir** | — | — | boot · `P` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RNF-DB-006` | escrita concorrente sem `40P01` | — | — | sistema · `P` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RNF-DB-007` | espera por conexão finita em `30 s` e reuso | — | — | instrumentação · `P` | `tests/alvos/sistema-pg.md` | onda 8 |
| `RNF-DB-008` | série contígua e reversível (gate `estrutura`) | — | — | gate · `P` | — (gate de série) | onda 8 |
| `RNF-DB-009` | constraint nomeada e violada em teste (gate `constraints`) | — | — | gate · `P` | — (gate de série) | onda 8 |
| `RNF-DB-010` | nenhum tipo proibido na série (gate `tipos`) | — | — | gate · `P` | — (gate de série) | onda 8 |
| `RNF-DB-011` | catálogo e série com o mesmo schema (gate `catálogo`) | — | — | gate · `P` | — (gate de série) | onda 8 |
| `RNF-DB-012` | fronteira entre papéis provada (gate `fronteira`) | — | — | gate · `P` | — (gate de série) | onda 8 |
| `RNF-DB-013` | ambiente nomeado, sem default (gate `ambiente`) | — | — | gate · `P` | — (gate de série) | onda 8 |

### 2.4 Casos de uso — 3

| ID | Enunciado (resumo) | Tabela | Porta | Teste | Página | Issue |
|---|---|---|---|---|---|---|
| `UC-DB-001` | autenticar no portal | as de `RF-DB-010`, `RF-DB-013`, `RN-DB-004` | `identity` | sistema · `V` | `tests/alvos/sistema-pg.md` | onda dos `RF` cobertos |
| `UC-DB-002` | criar conta na vaga | as de `RF-DB-001`, `RF-DB-003`, `RF-DB-012`, `RN-DB-001`, `RN-DB-005` | `accounts`, `wallet` | sistema · `V` | `tests/alvos/sistema-pg.md` | onda dos `RF` cobertos |
| `UC-DB-003` | abrir sessão de jogo | as de `RF-DB-002`, `RF-DB-006`, `RF-DB-014`, `RN-DB-002`, `RN-DB-003` | `session` | sistema · `V` | `tests/alvos/sistema-pg.md` | onda dos `RF` cobertos |

## 3. Itens sem cobertura

### 3.1 Sem teste

**13 das 40 linhas** têm teste executado, e **0 de 40** issues do roadmap existem: `RF-DB-001`, `002`, `004`,
`007`, `008`, `010`, `012`, `013`; `RN-DB-006`, `007`; `UC-DB-001`, `002`, `003` — todos por
`CAP-24`–`CAP-36` em `crates/wyd-db/tests/sistema_pg.rs`, 13 verdes em 2026-11-24 contra a base migrada. As
outras **24** seguem em `P`: **20** com cenário escrito e **4** sem `CAP-nn` (`RF-DB-003`, `RF-DB-006`,
`RF-DB-014` e `RNF-DB-001`); as **3** restantes são `—` e estão em §3.4.

### 3.2 Sem requisito nomeado

**16** das **109** tabelas aparecem em `RF` ou `RN`: `content.content_version`, `game.account`,
`game.account_wallet`, `game.account_ledger`, `game.account_moderation`, `game.character`,
`game.pin_reset`, `game.session`, `game.vault_item`, `ops.audit_log`, `ops.delivery_queue`,
`portal.daily_reward_claim`, `portal.donate_credit`, `portal.donate_debit`, `portal.person`,
`portal.person_token`. As outras **93** têm dicionário e página de migration, e **nenhum** requisito
nomeado — o que é o vazio que esta matriz torna visível.

| Domínio | Tabelas | Com `RF`/`RN` | Sem |
|---|---:|---:|---:|
| acesso | 9 | 6 | 3 |
| canal | 1 | 0 | 1 |
| auditoria | 3 | 1 | 2 |
| configuração | 5 | 1 | 4 |
| personagem | 5 | 1 | 4 |
| progressão | 5 | 0 | 5 |
| inventário | 3 | 1 | 2 |
| social | 4 | 0 | 4 |
| mundo | 13 | 0 | 13 |
| catálogo | 7 | 0 | 7 |
| combate | 4 | 0 | 4 |
| refino | 8 | 0 | 8 |
| economia | 22 | 4 | 18 |
| quests | 4 | 0 | 4 |
| eventos | 5 | 0 | 5 |
| arena | 4 | 0 | 4 |
| moderação | 1 | 1 | 0 |
| ponte web↔jogo | 1 | 1 | 0 |
| conteúdo do portal | 5 | 0 | 5 |

A contagem por domínio é de [`../dicionario-dados/index.md`](../dicionario-dados/index.md); a coluna
"com `RF`/`RN`" é esta matriz. **Os 93 são lacuna de requisito, não de documentação:** cada um tem
tabela, DDL e dicionário, e nenhum `RF`/`RN` que o nomeie. `content.content_version` entrou na conta em
2026-11-24 com `RF-DB-017`.

### 3.3 Sem dono de issue

**As 40 linhas** têm onda de alocação e **nenhuma** tem número de issue — o roadmap tem 40 issues
abertas (#82–#121, 2026-11-22) e nenhuma vinculada (`../planos/riscos.md` `R-5`). Enquanto a alocação
não acontece, `Issue` é promessa de onda, não vínculo.

**`onda 8` não é alocação de execução.** As **37** linhas com `onda 8` e as **3** `UC` com a onda dos `RF`
cobertos repetem a nota de `requisitos-funcionais.md`: a issue **nasce** na Onda 8 de
[`../planos/plano-de-documentacao.md`](../planos/plano-de-documentacao.md) §2, a que produz os **40**
`WYDDB-nnn`. A fase que executa cada requisito é do roadmap e não está nesta coluna, porque
[`../roadmap/fases.md`](../roadmap/fases.md) agrupa o trabalho por **camada** (baseline, contratos, portas,
filas, observabilidade, testes, série), não por requisito.

**Os seis de gate não têm issue que os nomeie.** `RNF-DB-008` a `RNF-DB-013` são gates de série: a prova é a
de [`../planos/plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) §1, rodada **depois**
da série (`plano-migrations.md` §5 — `provision render`, `apply`, `0001`…`0034`, catálogo, gates). Cinco deles
são medidos por alvos da fase 6 (`contrato_migrations` e `contrato_catalogo` em `WYDDB-028`/`WYDDB-029`,
`sistema_pg` em `WYDDB-030`, `sistema_sessoes` em `WYDDB-031`); `RNF-DB-013` não tem alvo Rust — prova-se
executando sem a variável. Nenhuma das **40** páginas `WYDDB-nnn` cita os seis IDs, e por isso o `onda 8`
deles é a mesma promessa de criação das outras linhas, **não** a leva de aceite de `migrations/`, que é ato de
página, não execução da série.

### 3.4 Sem página de teste

**9** das 40 linhas não apontam página, e o motivo é medido, não editorial:

| Motivo | Quantas | Quais | O que fecha |
|---|---:|---|---|
| regra de outro sistema | **2** | `RF-DB-015`, `RF-DB-016` | `wyd-server`, dono do fio |
| gate de CI, não alvo Rust | **1** | `RNF-DB-001` | `wyd-ops` (gate de cobertura) |
| gate de série, não cenário | **6** | `RNF-DB-008`–`RNF-DB-013` | o próprio gate (`../planos/plano-de-qualidade-e-gates.md` §1) |

**Nenhuma linha com `P` fica sem cenário:** as **18** que seguem em `P` têm página, e **14** das **16** que
ganharam cenário em 2026-11-24 com `Q-BLK-23` = A (`CAP-24` a `CAP-39`, em
`tests/alvos/sistema-pg.md` e `tests/alvos/runner-pg.md`) já rodaram e viraram `V` — `CAP-37` inclusive; as
duas restantes (`CAP-38`–`CAP-39`) dependem de código que não existe. Os três `RNF` que perderam o ponteiro falso para `sistema-pg` em `Q-BLK-24` = A (`RNF-DB-002`,
`004`, `005`) voltaram a apontar página na mesma data: o cenário mede e registra, e não fixa limiar que o
requisito não tem. A coluna `Teste` marca natureza **e** resultado — `V` onde o cenário rodou e passou.

## 4. Regra de atualização

| Quando muda | O que esta página exige na mesma entrega |
|---|---|
| requisito nasce, muda ou é retirado | a linha correspondente nasce, muda ou desaparece; a página dona é atualizada antes |
| tabela entra, muda de nome ou é retirada | a coluna `Tabela` acompanha `../dicionario-dados/` e a migration |
| porta do contrato muda | a coluna `Porta` acompanha `../arquitetura/componentes-c4.md` |
| teste é escrito | `P` vira o nome do teste e o `CAP-nn` da matriz, ou `P` desaparece quando o alvo não existe mais |
| cenário é executado e passa | `Teste` troca `P` por `V`, com o `CAP-nn`, o alvo e a data; execução sem verde não muda a célula |
| alvo de teste nasce ou muda de nome | §1.1 acompanha `tests/index.md` §2 e o `Cargo.toml`; a natureza da linha continua a mesma |
| issue é criada | a coluna `Issue` recebe o número; a onda deixa de ser a única informação |

Atualizar a matriz **não** cria requisito nem muda limiar: quem faz isso é a página dona, pela regra de
[`index.md`](index.md) §"Como um requisito nasce, muda e é retirado".

## 5. O que esta página não decide

- **Não enuncia requisito:** repete só o resumo; o texto, o limiar e o critério de aceitação ficam na
  página dona.
- **Não cria issue nem escolhe onda:** a alocação é do roadmap e da fila de §4 de
  `pendencias-de-fechamento.md`, na raiz do sistema.
- **Não mede cobertura de código:** percentual de linha é do gate; aqui o que se conta é requisito sem
  teste e tabela sem requisito.
- **Não declara requisito coberto:** `P` é alvo previsto; cobertura só existe com teste executado.
- **Não decide o que fazer com os 94:** o vazio é registrado aqui; transformá-lo em requisito é trabalho
  de quem enuncia, e lacuna se registra em `lacunas.md`, dono único, na raiz do sistema.
