> Pergunta: "quais unidades de código o `wyd-sim` tem, qual é a API de cada uma e o que ela não pode violar?"
> Público: Desenvolvedor do `wyd-sim`
> Dono: equipe do wyd-sim
> Base: `wyd-sim-1.0.0.md` §1, §2, §3, §6; `AGENTS.md` §6; `wyd-admin/docs/arquitetura/componentes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum arquivo de código existe. A árvore abaixo descreve unidades de responsabilidade, não arquivos 1:1.

## `damage`

- **Responsabilidade:** cálculo de dano físico/mágico a partir de entrada tipada.
- **API:** `physical_damage(input) -> output`, `magical_damage(input) -> output`.
- **Invariantes:** função pura total; sem I/O, estado, relógio, RNG; aritmética inteira; `formula_version` no cabeçalho.
- **Depende de:** `arithmetic`, `version`.
- **Testes:** vetor de entrada/saída; determinismo.
- **Risco:** alto — dano é regra de jogo.

## `accuracy`

- **Responsabilidade:** chance de acerto a partir de entrada tipada.
- **API:** `hit_chance(input) -> probability`.
- **Invariantes:** pura; percentual em bps; sem RNG no sim.
- **Depende de:** `arithmetic`, `version`.
- **Testes:** vetor; limite de chance.
- **Risco:** médio.

## `drop`

- **Responsabilidade:** chance/quantidade de drop a partir de entrada tipada.
- **API:** `drop_roll(input) -> result`.
- **Invariantes:** pura; limiar/probabilidade, não sorteio (sorteio no core).
- **Depende de:** `arithmetic`, `version`.
- **Testes:** vetor; limiar.
- **Risco:** médio.

## `economy`

- **Responsabilidade:** fórmula de economia (preço, taxa, conversão) em unidades menores inteiras.
- **API:** `price(input) -> amount`.
- **Invariantes:** dinheiro inteiro (`P-07`); percentual em bps; sem f64 em estado.
- **Depende de:** `arithmetic`, `version`.
- **Testes:** vetor; arredondamento meio-para-longe-do-zero.
- **Risco:** alto — economia é dinheiro.

## `movement`

- **Responsabilidade:** deslocamento puro do passo 6 (entrada tipada → deslocamento).
- **API:** `displacement(input) -> delta`.
- **Invariantes:** pura; sem colisão (colisão é do core).
- **Depende de:** `arithmetic`, `version`.
- **Testes:** vetor; determinismo.
- **Risco:** médio.

## `arithmetic`

- **Responsabilidade:** primitivas de aritmética inteira compartilhadas (bps, divisão, arredondamento).
- **API:** `bps`, `div_round`, `overflow_checked`.
- **Invariantes:** overflow verificado; arredondamento meio-para-longe-do-zero uma vez por fórmula.
- **Depende de:** nenhum.
- **Testes:** overflow; arredondamento.
- **Risco:** alto — base de todas as fórmulas.

## `version`

- **Responsabilidade:** `formula_version` próprio por fórmula + `content_version` usado.
- **API:** `formula_version() -> u32`, `content_version_used() -> u32`.
- **Invariantes:** mudança de código que altere resultado incrementa versão; mudança de conteúdo mantém versão.
- **Depende de:** nenhum.
- **Testes:** versão declarada.
- **Risco:** baixo.

## `fixtures`

- **Responsabilidade:** carregar/validar fixtures JSON versionadas em `tests/fixtures/sim/<formula>.json`.
- **API:** `load_fixture(name)`.
- **Invariantes:** entrada, saída esperada, versão de fórmula; reprova quando diverge.
- **Depende de:** `version`.
- **Testes:** fixture carrega; divergência reprova.
- **Risco:** baixo.

## Árvore de código e de teste

```text
src/                      # planejamento; código real mora em wyd-sim/ (raiz) ou crates/wyd-sim/
├── damage/               # dano físico/mágico
├── accuracy/             # chance de acerto
├── drop/                 # drop
├── economy/              # economia
├── movement/             # deslocamento puro
├── arithmetic/           # primitivas inteiras
├── version/              # formula_version + content_version
└── fixtures/             # carga de fixtures
tests/
├── unitario/            # arithmetic, version
├── contrato/            # fixtures JSON
├── integracao/          # core × sim
├── sistema/             # (n/a)
├── e2e/                 # (n/a)
└── propriedade/         # determinismo com vetor
tests/fixtures/sim/       # fixtures JSON versionadas
```

*Página de planejamento. Nenhuma unidade foi implementada.*
