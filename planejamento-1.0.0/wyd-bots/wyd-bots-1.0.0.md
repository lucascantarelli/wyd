# wyd-bots 1.0.0 — abertura do planejamento

> Pergunta: "que bots o projeto precisa, o que cada um prova e o que falta para começar o código?"
> Público: QA, arquitetura e equipe do `wyd-bots`
> Dono: equipe do `wyd-bots`
> Base: `wyd-protocol-1.0.0.md` §1.4; `AGENTS.md` §8; `wyd-protocol/lib/lacunas` (contrato do codec)
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-22). Abertura inicial: fronteira, contratos e lacunas nomeadas.
**Nenhum código** e nenhum requisito numerado (`RF-BOT`). Nenhum bot existe, e nenhum teste de carga
foi medido.

## 1. O que ele faz

Tem três papéis distintos, e o mesmo codec do servidor em modo cliente:

1. **QA de fio** — conecta, faz login e exercita fluxo para provar comportamento ponta a ponta.
2. **Carga** — N conexões com perfil declarado, para medir limite e degradação.
3. **Convivência** — presença sintética controlada, para observar o servidor com jogadores
   simulados.

Ele não substitui teste de unidade nem de contrato: é o nível mais alto da pirâmide e o mais caro.

## 2. O que ele **não** faz

| Fora | Dono |
|---|---|
| codec e layout do fio | `wyd-protocol` |
| socket, sessão e rate-limit | `wyd-net` |
| regra de jogo | `wyd-core` / `wyd-sim` |
| provisionamento de ambiente e infraestrutura | `wyd-ops` |
| métrica de produção | `wyd-ops` |

## 3. Contratos

| # | Contrato | Direção | Forma | Garantia | Estado |
|---|---|---|---|---|---|
| B1 | codec em modo cliente | bots ← `wyd-protocol` | mesmo `encode`/`decode` do servidor | nenhum atalho de teste no codec | **decidido** (`wyd-protocol` §1.4) |
| B2 | ambiente alvo do cenário | bots ← `wyd-ops` | ambiente efêmero ou homologação | nenhum bot em produção sem autorização | **decidido** `LBOT-01` (`Q-BOT-1` = A, 2026-11-23): CI efêmero, servidor descartável por execução |
| B3 | conta sintética e dado de teste | bots ← `wyd-db` | fixture, não conta real | nenhum dado de jogador | **decidido** `LBOT-02` (`Q-BOT-2` = A, 2026-11-23): seed declarativo em `crates/wyd-db/seeds/dev.sql`, banco recriado por execução |
| B4 | limiar de carga e o que reprova | bots → QA | perfil, duração e limiar medido/congelado; ao atingir, parar, drenar e reportar | número medido, não prometido | **política decidida** `Q-BOT-3` = A (2026-11-23); **`LBOT-03` aberta como medição** |
| B5 | isolamento entre execuções | bots garante | ambiente descartável por execução | sem estado global entre cenários | **decidido** `LBOT-02` (`Q-BOT-2` = A, 2026-11-23): banco recriado por execução, seed reaplicado |

**Organização (D-04 = A): fechada em 2026-11-25.** A auditoria (`wyd-ops/auditoria-de-organizacao.md`) listava para este sistema: árvore de `src/`/`tests/` ausente, `RF-BOT` não numerados, plano de teste próprio ausente, roadmap parcial. Fechado em 2026-11-25 com `src/.gitkeep` + `tests/.gitkeep`, `docs/arquitetura/componentes.md`, `docs/planos/plano-de-testes.md`, `docs/specs/requisitos.md` (6 `RF-BOT` + 5 `RNF-BOT`) e `docs/roadmap/` (index.md + roadmap.md com DoD por fatia). `LBOT-01`/`LBOT-02`/`LBOT-04` fechadas; `LBOT-03` aberta como medição.

## 4. Estado medido (`V`, 2026-11-22)

| Medida | Valor | Comando |
|---|---:|---|
| Páginas de planejamento | 3 | `find planejamento-1.0.0/wyd-bots -name '*.md'` |
| Arquivos de código | 0 | `find crates/wyd-bots -type f` (só `.gitkeep`) |
| Cenários escritos | 0 | nenhum |
| Testes de carga medidos | 0 | nenhum |

## 5. O que falta

`lacunas.md`: `LBOT-01` (ambiente alvo) fechou em 2026-11-23 por resposta do dono (`Q-BOT-1` = A): CI
efêmero, com servidor descartável por execução. `LBOT-02` (conta e dado sintéticos) fechou na mesma data
(`Q-BOT-2` = A): seed declarativo em `crates/wyd-db/seeds/dev.sql`, banco recriado por execução. Segue
aberta `LBOT-03` (limiar de carga), mas com a **política** fixada em 2026-11-23 por `Q-BOT-3` = A:
medir no CI efêmero, congelar o valor com data e, ao atingi-lo, parar de subir, drenar e reportar;
nenhum número antes da medição. `LBOT-04` (o que o bot faz quando o cenário falha — relatório e
artefato) fechou em 2026-11-23 por **regra vigente**: o item 4 do gate (§6) já exige log + captura, e
`AGENTS.md` §5, item 7/`P-22` fixam a higiene. O estado das respostas está em `../lacunas.md`; a
pauta em `../wyd-ops/docs/pauta-de-decisoes-pendentes.md` reúne os ponteiros.

## 6. Gate para sair do papel

1. Primeiro cenário de QA de fio escrito, com fluxo e critério de sucesso; o cenário de carga só
   reprova depois da primeira medição que congela o limiar (`Q-BOT-3` = A).
2. `LBOT-01` decidida em 2026-11-23 (`Q-BOT-1` = A): CI efêmero; nenhum bot roda em produção por
   acidente.
3. `RF-BOT` numerados com critério verificável.
4. Relatório por execução com falha reproduzível (log + captura), sem `sleep` como sincronização.
5. Nenhum segredo de jogador em configuração de bot.

## 7. Próximas páginas
`docs/arquitetura/fronteira.md`, `docs/specs/cenarios.md`, `docs/planos/plano-de-testes.md` e
`docs/roadmap/`.
