# wyd-core 1.0.0 — abertura do planejamento

> Pergunta: "o que o `wyd-core` faz, o que ele consome dos outros sistemas e o que falta para começar o código?"
> Público: Produto, arquitetura e equipe do `wyd-core`
> Dono: equipe do `wyd-core`
> Base: `wyd-protocol-1.0.0.md` §1.4/§1.2; `LPRT-18`; `wyd-ops/proposta-versoes-e-cobertura-funcional.md` §7.2; `AGENTS.md` §6
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22). Abertura inicial: fronteira, contratos e lacunas nomeadas.
**Nenhum código**, nenhum requisito numerado (`RF-CORE`), nenhum roadmap com DoD. O catálogo
funcional ainda não existe (`wyd-1.0.0.md` §1.5), mas o escopo de `LCORE-01` foi decidido em
2026-11-23: `1.0.0` é o catálogo completo, e o primeiro domínio é a entrada na cena.

## 1. O que ele faz

É a simulação do mundo em memória: o tick, as entidades e componentes, a cena, o combate e a IA.
Recebe a mensagem já decodificada do `wyd-net`, aplica a regra do jogo — chamando as fórmulas do
`wyd-sim` quando a regra é cálculo — e pede ao `wyd-db` o que precisa ser lido ou persistido. É o
dono do **comportamento**: classes, skills, combate, guerra, guilda e estados especiais, conforme a
divisão de catálogo decidida em 2026-11-20.

## 2. O que ele **não** faz

| Fora | Dono |
|---|---|
| byte, cabeçalho, opcode e encoding do fio | `wyd-protocol` |
| socket, timeout, rate-limit, ordem de entrega | `wyd-net` |
| fórmula, curva, chance, dano e drop como cálculo | `wyd-sim` |
| DDL, migrations, SQL e transação | `wyd-db` |
| processo por canal, bind e ciclo de vida do servidor | `wyd-server` |
| escrita administrativa e custódia de item | `wyd-admin` |
| patch e arquivo do cliente | `wyd-client` / `wyd-launcher` |

## 3. Contratos

| # | Contrato | Direção | Forma | Garantia | Dono | Estado |
|---|---|---|---|---|---|---|
| C1 | payload tipado por família | core ← `wyd-protocol` | tipo já validado em faixa | offset cru não entra | `wyd-core` | **decidido** (`LPRT-18`) |
| C2 | entrega de mensagem e ordem | core ← `wyd-net` | mensagem tipada, em ordem por conexão | nenhuma mensagem fora de ordem | `wyd-net` | **decidido** (`LPRT-12`) |
| C3 | fórmula de dano, EXP, drop, cooldown | core ← `wyd-sim` | função pura, determinística | mesma entrada, mesma saída | `wyd-sim` | **decidido** (`LCORE-03`); fórmulas em `LSIM-01`/`LSIM-04` |
| C4 | leitura e persistência de estado | core → `wyd-db` | porta `CoreRepository` com `load_session`, `load_character` e `commit(UnitOfWork)`; sem SQL no core | `wyd-db` executa a transação; nenhuma query fora do adapter | `wyd-core` (forma), `wyd-db` (adapter) | **decidido** (`LCORE-04`) |
| C5 | comando de processo (kick, broadcast) | core ← `wyd-server` | chamada interna do processo | ack do `wyd-admin` antes de efeito | `wyd-server` | **decidido** no elo (`LAC-ADM-03`) |
| C6 | o que é estado em memória e o que é durável | core decide | durável crítico, durável eventual e memória | escrita crítica antes do ack; posição no release; sem checkpoint periódico na primeira fatia | `wyd-core` | **decidido** (`LCORE-02`) |

### 3.1 Fronteira `core` × `sim` (`LCORE-03`, 2026-11-23)

**Regra.** Uma responsabilidade é do `wyd-sim` quando pode ser escrita como função **total, pura e
determinística** `f(entrada tipada e imutável) -> saída tipada`, sem ler ou mutar estado de mundo,
sessão ou banco, sem relógio, sem RNG e sem I/O; o mesmo vetor de entrada produz sempre o mesmo vetor
de saída. É do `wyd-core` quando precisa ler ou alterar estado, escolher alvo/ordem/momento, validar
pré-condição, consumir sorteio, aplicar efeito no mundo, emitir evento ou persistir.

**Consequências verificáveis.**

1. `sim` não recebe `&mut World`, sessão, repositório, `Clock`, `Rng` nem faz I/O; sua API é entrada
   tipada -> saída tipada.
2. `core` chama `sim` para o número/curva/limiar e **não recalcula** a fórmula; a duplicação de uma
   expressão de dano, EXP, drop ou chance no core é defeito de revisão.
3. Sorteio pertence ao core: o `sim` devolve probabilidade/limiar ou recebe o resultado do sorteio
   como entrada; o estado do RNG não entra no `sim`.
4. A prova é dupla: vetor de entrada/saída no `sim` (`LSIM-03`) e teste de estado/decisão no core com
   repositório ou mundo falso; mudar só o estado muda a decisão do core, não a saída do `sim` para o
   mesmo vetor.
5. No passo 6, o core valida a sessão/personagem, resolve mapa e colisão, aplica a posição resultante
   e emite o resultado; o `sim` calcula apenas o deslocamento puro a partir da entrada tipada. A
   fórmula concreta e a aritmética continuam em `LSIM-01` e `LSIM-04`.

### 3.2 Fronteira de persistência (`LCORE-02`/`LCORE-04`, 2026-11-23)

**Regra C6 — classificação do estado.** Todo estado do core pertence a uma das três classes abaixo.
A classe decide quando o estado é gravado e qual perda é aceitável; ela não decide schema, coluna ou
frequência de job.

| Classe | O que entra | Quando é gravado | Perda aceitável |
|---|---|---|---|
| **Durável crítico** | identidade/auth, sessão, personagem, inventário, economia, progressão, quests e auditoria | antes do ack que depende do estado; a operação só responde depois do commit | nenhuma perda observável; falha impede o ack |
| **Durável eventual** | posição e cena do personagem | aplicação em memória no passo; checkpoint no release da sessão | desde o último release; não há checkpoint periódico na primeira fatia |
| **Memória** | buffers, caches, entidades, estado efêmero e derivados de durável + `content_version` | não é gravado; é reconstruído ou descartado | total, desde que o durável e o `content_version` permitam reconstruir |

**Regra de dado novo.** Se a perda for observável ou o dado não puder ser reconstruído a partir do
durável + `content_version`, ele é **durável**; se for reconstruível, é **memória**. Na dúvida, a
classificação falha fechada como **durável crítico**. Todo estado novo entra nesta tabela antes de
aparecer em código; a classificação não é inferida pela tabela do banco.

**Posição hoje.** `game.character` não tem coluna de coordenada atual — só `recall_x`/`recall_y`
(`wyd-db/migrations/0006-game-character.md` §6.1, `V` 2026-11-22). A posição é **durável eventual**,
mas a primeira fatia não prova save e esta decisão **não cria coluna**. O checkpoint de posição no
release depende de schema futuro; não se inventa DDL aqui.

**Contrato C4 — porta `CoreRepository`.** O core define a forma da porta; o `wyd-db` implementa o
adapter. A composição do `wyd-server` injeta o adapter no core. O core não importa `wyd-db`, `sqlx`,
conexão, transação, linha ou nome de tabela.

| Operação | Entrada | Saída | Garantia |
|---|---|---|---|
| `load_session` | identificador da sessão | `Option<SessionSnapshot>` ou erro tipado | leitura de uma sessão; ausência é `None`, nunca erro |
| `load_character` | identificador do personagem e da conta | `Option<CharacterSnapshot>` ou erro tipado | leitura escopada à conta; personagem de outra conta ou excluído é `None` |
| `commit` | `UnitOfWork` consumida | `CommitReceipt` ou erro tipado | única operação de escrita; abre, aplica na ordem, faz commit ou rollback |

`SessionSnapshot` contém apenas o que o core precisa para decidir: `session_id`, `account_id`,
`character_id`, `group_id`, `channel`, `client_version`, `opened_at`, `released_at` e
`released_reason`. `CharacterSnapshot` contém `character_id`, `account_id`, `deleted_at`,
`content_version` e os campos de estado que a regra do primeiro domínio exigir; não é uma entidade
lazy nem um registro de tabela. `CommitReceipt` devolve `session_id`, `character_id`, `released_at` e
o estado final observado.

**`UnitOfWork` e operações transacionais.** A unidade de trabalho é do core: ele escolhe as operações,
a ordem e o conteúdo. O `wyd-db` é dono da transação, do commit/rollback, do isolamento, da execução
na ordem recebida e do mapeamento de constraint para erro. O core não recebe `begin`, `rollback`,
`Transaction`, `Connection`, `Row` ou `Pool`.

| Operação da UoW | Efeito esperado | Falha sem escrita |
|---|---|---|
| `OpenSession` | cria a sessão viva com `account_id`, canal, `client_version` e `character_id` nulo; devolve o `session_id` gerado pelo banco | conta já tem sessão viva, canal inexistente, versão inválida ou FK inválida |
| `SelectCharacter` | grava `character_id` somente se a sessão está viva, ainda sem personagem, pertence à conta e o personagem não está excluído | sessão ausente, já liberada, já com personagem, personagem de outra conta ou excluído |
| `ReleaseSession` | grava `released_at` e `released_reason` uma vez; repetir a mesma liberação é idempotente e não sobrescreve o instante | sessão ausente ou já liberada com motivo diferente |

A primeira fatia usa **uma operação de escrita por comando**, mas a UoW aceita mais de uma operação
quando a atomicidade entre elas for necessária. Todas as operações de C4 vivem em `game`; nenhuma UoW
do core cruza `portal`, `ops` ou `content` na primeira fatia. Uma falha em qualquer operação desfaz
todas as anteriores: não existe commit parcial.

**Leitura e erro.** `load_session` e `load_character` são leituras independentes; não abrem uma
transação longa nem garantem snapshot comum entre duas chamadas. A ausência é `None`; o core decide se
ela é sessão inexistente, personagem não pertencente à conta ou personagem excluído. O adapter não
devolve linha crua e não expõe SQLSTATE ao core.

| Erro tipado | Quando | Efeito no core |
|---|---|---|
| `NotFound` | uma operação de atualização aponta para linha que não existe mais | trata como estado obsoleto; não repete a UoW cegamente |
| `VersionConflict` | a pré-condição de estado da operação não vale mais: sessão já selecionada, já liberada ou estado esperado divergente | recarrega o estado e decide de novo; não sobrescreve |
| `InvariantViolation` | constraint ou invariante: sessão viva duplicada (`AlreadyLive`), personagem de outra conta/excluído, canal inexistente, motivo inválido | rejeita a operação; nada é gravado |
| `Unavailable` | conexão, timeout ou conflito de concorrência antes do commit; carrega `retryable` | pode reconstruir a UoW após nova leitura; não há retry automático |
| `CommitUncertain` | o commit pode ter sido aplicado, mas a resposta não chegou | **nova leitura obrigatória** de `load_session`/`load_character`; nunca reenvia a UoW às cegas |

**Proibições verificáveis.**

1. O core não contém `sqlx`, `SELECT`, `INSERT`, `UPDATE`, `DELETE`, `game.` ou nome de tabela; a
   única porta de persistência é `CoreRepository`.
2. O `sim` não importa `CoreRepository`, `UnitOfWork`, `StoreError`, snapshot nem qualquer tipo de
   persistência; ele continua função pura de entrada/saída.
3. O core não faz I/O fora da porta: sem arquivo, socket, pool ou conexão; o relógio e o RNG continuam
   no core, mas não viram tipos de banco.
4. O `wyd-db` não decide regra de jogo: ele executa a UoW, aplica a transação e mapeia constraint; a
   ordem e o conteúdo são do core.
5. O adapter é o único ponto de SQL; a prova de revisão é o grep e o teste de contrato com repositório
   falso, não a inspeção visual.

**Prova mínima.**

- teste de contrato com repositório falso: `load_session`/`load_character` ausentes devolvem `None`;
  `commit` recebe as operações na ordem e falha sem chamar o adapter quando a pré-condição do core já
  rejeitou;
- teste de sistema com PostgreSQL: uma UoW com duas operações grava as duas ou nenhuma; constraint
  vira `InvariantViolation`; segunda sessão viva vira `InvariantViolation(AlreadyLive)`;
- teste de ambiguidade: adapter devolve `CommitUncertain`; o core faz nova leitura e não reenvia a UoW;
- `grep -R "sqlx\|SELECT\|INSERT\|UPDATE\|DELETE\|game\." crates/wyd-core` não encontra nada;
- `grep -R "CoreRepository\|UnitOfWork\|StoreError" crates/wyd-sim` não encontra nada.

**O que esta seção não decide.** Schema, coluna de posição, DDL, isolation level, timeout, pool,
política de retry do adapter e tabelas de outros domínios. A classificação de estado é C6; a porta é
C4; ambas foram decididas em 2026-11-23.

## 4. Estado medido (`V`, 2026-11-22)

| Medida | Valor | Comando |
|---|---:|---|
| Páginas de planejamento | 4 | `find planejamento-1.0.0/wyd-core -name '*.md'` |
| Arquivos de código | 0 | `find crates/wyd-core -type f` (só `.gitkeep`) |
| Funcionalidades catalogadas | 0 | `wyd-1.0.0.md` §1.5: catálogo não existe |
| Issues criadas | 0 | nenhuma ação no GitHub |

## 5. O que falta

`lacunas.md` é o dono do estado: as quatro lacunas fecharam em 2026-11-23. `LCORE-01` fixou escopo
(catálogo completo; primeiro domínio: entrada na cena), `LCORE-03` fixou a fronteira `core` × `sim` e
`LCORE-02`/`LCORE-04` fixaram a fronteira de persistência em §3.2 (classificação C6 e porta C4). O que
falta é implementar a porta, o adapter e o teste de contrato, não decidir.

## 6. Gate para sair do papel

1. Catálogo funcional do primeiro domínio escrito, com prova (`wyd-1.0.0.md` §4.2).
2. `lacunas.md` sem bloqueio para os requisitos da primeira versão.
3. `RF-CORE` numerados, com critério verificável e dono.
4. Árvore de `src/` e `tests/` planejada antes do primeiro arquivo.
5. Plano de teste determinístico: tick com semente fixa, sem `sleep` como sincronização.

## 7. Próximas páginas

`docs/arquitetura/fronteira.md`, `docs/specs/` com os `RF-CORE` do primeiro domínio,
`docs/planos/plano-de-testes.md` (determinismo) e `docs/roadmap/` com DoD.
