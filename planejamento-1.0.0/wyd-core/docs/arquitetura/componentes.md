> Pergunta: "quais unidades de código o `wyd-core` tem, qual é a API de cada uma e o que ela não pode violar?"
> Público: Desenvolvedor do `wyd-core`
> Dono: equipe do wyd-core
> Base: `wyd-core-1.0.0.md` §1, §2, §3, §6; `AGENTS.md` §6; `wyd-admin/docs/arquitetura/componentes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum arquivo de código existe. A árvore abaixo descreve unidades de responsabilidade, não arquivos 1:1.

## `tick`

- **Responsabilidade:** avançar a simulação em passos discretos, determinísticos, com semente fixa.
- **API:** `step(dt)`, `now()`, `seed`.
- **Invariantes:** sem `sleep` como sincronização; mesma semente → mesma sequência; relógio no core.
- **Depende de:** `entities`, `scene`, `events`.
- **Testes:** determinismo com semente fixa; avanço de tempo.
- **Risco:** alto — tick é a espinha dorsal.

## `entities`

- **Responsabilidade:** entidades e componentes (ECS) em memória; criação, remoção, query.
- **API:** `spawn`, `despawn`, `get`, `query`.
- **Invariantes:** sem I/O; estado efêmero reconstruível a partir do durável + `content_version`.
- **Depende de:** `tick`.
- **Testes:** CRUD de componente; query por filtro.
- **Risco:** médio.

## `scene`

- **Responsabilidade:** cena, mapa, colisão, resolução de posição.
- **API:** `load_map`, `collide`, `resolve_position`.
- **Invariantes:** posição é durável eventual; checkpoint no release; sem checkpoint periódico na primeira fatia.
- **Depende de:** `entities`, `wyd-sim` (deslocamento puro).
- **Testes:** colisão; resolução de posição.
- **Risco:** médio.

## `combat`

- **Responsabilidade:** aplicar regra de combate: validar pré-condição, consumir sorteio, aplicar efeito, emitir evento.
- **API:** `attack`, `apply_effect`, `emit`.
- **Invariantes:** fórmula vem do `wyd-sim` (não recalcula); sorteio no core; duplicação de expressão é defeito.
- **Depende de:** `entities`, `wyd-sim`, `events`, `persistence`.
- **Testes:** ataque com repositório falso; sorteio consumido.
- **Risco:** alto — combate é regra.

## `ai`

- **Responsabilidade:** comportamento de NPC/mob: decisão de alvo, momento, estado.
- **API:** `decide`, `update`.
- **Invariantes:** decisão depende de estado; mudar estado muda decisão; sem fórmula duplicada.
- **Depende de:** `entities`, `scene`, `wyd-sim`.
- **Testes:** decisão com estado falso.
- **Risco:** médio.

## `events`

- **Responsabilidade:** barramento de eventos internos (combate, movimento, quest).
- **API:** `publish`, `subscribe`.
- **Invariantes:** ordem por conexão preservada; sem paralelismo de despacho.
- **Depende de:** `tick`.
- **Testes:** ordem de publicação; entrega.
- **Risco:** baixo.

## `persistence`

- **Responsabilidade:** porta `CoreRepository` (`load_session`, `load_character`, `commit(UnitOfWork)`); sem SQL.
- **API:** trait/porta; implementada pelo `wyd-db`.
- **Invariantes:** sem `sqlx`, `SELECT`, `INSERT`, `UPDATE`, `DELETE`, `game.`; nenhum tipo de persistência cruza para `wyd-sim`; ausência é `None`.
- **Depende de:** `wyd-db` (adapter, contrato).
- **Testes:** contrato com repositório falso; UoW na ordem; falha sem escrita; `CommitUncertain` → nova leitura.
- **Risco:** alto — fronteira de persistência.

## `validation`

- **Responsabilidade:** validar pré-condições (sessão, personagem, conta) antes de aplicar.
- **API:** `check_session`, `check_character`.
- **Invariantes:** falha fechada como durável crítico na dúvida.
- **Depende de:** `persistence`.
- **Testes:** pré-condição violada rejeita.
- **Risco:** médio.

## `observability`

- **Responsabilidade:** métricas e logs de metadados; sem payload/PII.
- **API:** `metrics`, `log_meta`.
- **Invariantes:** nunca payload/PII em log.
- **Depende de:** todas.
- **Testes:** log sem payload.
- **Risco:** baixo.

## Árvore de código e de teste

```text
src/                      # planejamento; código real mora em wyd-core/ (raiz) ou crates/wyd-core/
├── tick/                 # passos discretos, semente
├── entities/             # ECS em memória
├── scene/                # mapa, colisão, posição
├── combat/               # regra de combate
├── ai/                   # comportamento NPC/mob
├── events/               # barramento interno
├── persistence/          # porta CoreRepository, UoW
├── validation/           # pré-condições
└── observability/        # métricas/logs
tests/
├── unitario/            # tick, entities, validation
├── contrato/            # repositório falso, sim
├── integracao/          # cena + combate
├── sistema/             # PostgreSQL + UoW
├── e2e/                 # ponta a ponta
└── propriedade/         # determinismo com semente
```

*Página de planejamento. Nenhuma unidade foi implementada.*