> Pergunta: "quais unidades de código o `wyd-bots` tem, qual é a API de cada uma e o que ela não pode violar?"
> Público: Desenvolvedor do `wyd-bots`
> Dono: equipe do wyd-bots
> Base: `wyd-bots-1.0.0.md` §1, §2, §3, §6; `AGENTS.md` §6; `wyd-admin/docs/arquitetura/componentes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum arquivo de código existe. A árvore abaixo descreve unidades de responsabilidade, não arquivos 1:1.

## `qa-bot`

- **Responsabilidade:** conectar, logar e exercitar fluxo para provar comportamento ponta a ponta.
- **API:** `run_scenario(scenario) -> Result`.
- **Invariantes:** codec em modo cliente (mesmo `encode`/`decode`); nenhum atalho de teste no codec.
- **Depende de:** `wyd-protocol`, `wyd-net`, `scenario-runner`.
- **Testes:** cenário de login; cenário de entrada na cena.
- **Risco:** alto — é o nível mais alto da pirâmide.

## `load-bot`

- **Responsabilidade:** N conexões com perfil declarado, para medir limite e degradação.
- **API:** `run_load(profile, duration) -> Report`.
- **Invariantes:** perfil, duração e limiar medidos/congelados; ao atingir, parar, drenar, reportar.
- **Depende de:** `wyd-protocol`, `wyd-net`, `scenario-runner`, `reporting`.
- **Testes:** degrau de carga; saturação.
- **Risco:** alto — medição, não promessa.

## `coexistence-bot`

- **Responsabilidade:** presença sintética controlada para observar o servidor com jogadores simulados.
- **API:** `run_coexistence(count, duration)`.
- **Invariantes:** ambiente descartável por execução; sem estado global entre cenários.
- **Depende de:** `qa-bot`, `scenario-runner`.
- **Testes:** presença estável.
- **Risco:** médio.

## `scenario-runner`

- **Responsabilidade:** executar cenário: setup, ações, assert, teardown.
- **API:** `run(scenario) -> Outcome`.
- **Invariantes:** sem `sleep` como sincronização; timeout explícito.
- **Depende de:** `wyd-protocol`, `wyd-net`.
- **Testes:** timeout; falha reproduzível.
- **Risco:** médio.

## `reporting`

- **Responsabilidade:** relatório por execução: log + captura, sem segredo de jogador.
- **API:** `report(outcome) -> Artifact`.
- **Invariantes:** artefato só metadados/frame anonimizado; sem PII.
- **Depende de:** `anonymization` (contrato).
- **Testes:** relatório sem PII; artefato anexado.
- **Risco:** médio.

## `environment`

- **Responsabilidade:** provisionar ambiente efêmero (Postgres + servidor descartáveis por execução).
- **API:** `up()`, `down()`.
- **Invariantes:** CI efêmero; sem estado entre cenários; seed declarativo aplicado.
- **Depende de:** `wyd-ops`, `wyd-db` (seed).
- **Testes:** up/down; seed aplicado.
- **Risco:** médio.

## `config`

- **Responsabilidade:** perfil, duração, limiar, cenário, ambiente.
- **API:** `load() -> Config`.
- **Invariantes:** nenhum segredo de jogador em configuração.
- **Depende de:** nenhum.
- **Testes:** config inválida recusa.
- **Risco:** baixo.

## Árvore de código e de teste

```text
src/                      # planejamento; código real mora em wyd-bots/ (raiz)
├── qa-bot/               # QA de fio
├── load-bot/             # carga
├── coexistence-bot/      # convivência
├── scenario-runner/      # executor de cenário
├── reporting/            # relatório
├── environment/          # ambiente efêmero
└── config/               # configuração
tests/
├── unitario/            # config, runner
├── contrato/            # codec em modo cliente
├── integracao/          # cenário completo
├── sistema/             # ambiente efêmero
├── e2e/                 # (n/a)
└── propriedade/         # fuzz de cenário
```

*Página de planejamento. Nenhuma unidade foi implementada.*
