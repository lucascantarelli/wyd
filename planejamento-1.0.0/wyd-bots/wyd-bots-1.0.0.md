# wyd-bots 1.0.0 — abertura do planejamento

> Pergunta: "que bots o projeto precisa, o que cada um prova e o que falta para começar o código?"
> Público: QA, arquitetura e equipe do `wyd-bots`
> Dono: equipe do `wyd-bots`
> Base: `wyd-protocol-1.0.0.md` §1.4; `AGENTS.md` §8; `wyd-protocol/lib/lacunas` (contrato do codec)
> Revisado em: 2026-11-22

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
| B2 | ambiente alvo do cenário | bots ← `wyd-ops` | ambiente efêmero ou homologação | nenhum bot em produção sem autorização | **lacuna** `LBOT-01` |
| B3 | conta sintética e dado de teste | bots ← `wyd-db` | fixture, não conta real | nenhum dado de jogador | **lacuna** `LBOT-02` |
| B4 | limiar de carga e o que reprova | bots → QA | perfil, duração e limiar | número medido, não prometido | **lacuna** `LBOT-03` |
| B5 | isolamento entre execuções | bots garante | ambiente descartável por execução | sem estado global entre cenários | **proposto** |

## 4. Estado medido (`V`, 2026-11-22)

| Medida | Valor | Comando |
|---|---:|---|
| Páginas de planejamento | 3 | `find planejamento-1.0.0/wyd-bots -name '*.md'` |
| Arquivos de código | 0 | `find crates/wyd-bots -type f` (só `.gitkeep`) |
| Cenários escritos | 0 | nenhum |
| Testes de carga medidos | 0 | nenhum |

## 5. O que falta

`lacunas.md`: `LBOT-01` (ambiente alvo), `LBOT-02` (conta e dado sintético), `LBOT-03` (limiar de
carga) e `LBOT-04` (o que o bot faz quando o cenário falha — relatório e artefato).

## 6. Gate para sair do papel

1. Primeiro cenário de QA de fio escrito, com fluxo e critério de sucesso.
2. `LBOT-01` decidida: nenhum bot roda em produção por acidente.
3. `RF-BOT` numerados com critério verificável.
4. Relatório por execução com falha reproduzível (log + captura), sem `sleep` como sincronização.
5. Nenhum segredo de jogador em configuração de bot.

## 7. Próximas páginas

`docs/arquitetura/fronteira.md`, `docs/specs/cenarios.md`, `docs/planos/plano-de-testes.md` e
`docs/roadmap/`.
