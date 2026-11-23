# wyd-core 1.0.0 — abertura do planejamento

> Pergunta: "o que o `wyd-core` faz, o que ele consome dos outros sistemas e o que falta para começar o código?"
> Público: Produto, arquitetura e equipe do `wyd-core`
> Dono: equipe do `wyd-core`
> Base: `wyd-protocol-1.0.0.md` §1.4/§1.2; `LPRT-18`; `wyd-ops/proposta-versoes-e-cobertura-funcional.md` §7.2; `AGENTS.md` §6
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22). Abertura inicial: fronteira, contratos e lacunas nomeadas.
**Nenhum código**, nenhum requisito numerado (`RF-CORE`), nenhum roadmap com DoD. O catálogo
funcional — que decidiria o tamanho deste sistema — ainda não existe (`wyd-1.0.0.md` §1.5).

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
| C3 | fórmula de dano, EXP, drop, cooldown | core ← `wyd-sim` | função pura, determinística | mesma entrada, mesma saída | `wyd-sim` | **proposto** |
| C4 | leitura e persistência de estado | core → `wyd-db` | porta de repositório, não SQL no core | nenhuma query fora do adapter | `wyd-db` | **proposto** |
| C5 | comando de processo (kick, broadcast) | core ← `wyd-server` | chamada interna do processo | ack do `wyd-admin` antes de efeito | `wyd-server` | **decidido** no elo (`LAC-ADM-03`) |
| C6 | o que é estado em memória e o que é durável | core decide | a definir | — | `wyd-core` | **lacuna** `LCORE-02` |

## 4. Estado medido (`V`, 2026-11-22)

| Medida | Valor | Comando |
|---|---:|---|
| Páginas de planejamento | 3 | `find planejamento-1.0.0/wyd-core -name '*.md'` |
| Arquivos de código | 0 | `find crates/wyd-core -type f` (só `.gitkeep`) |
| Funcionalidades catalogadas | 0 | `wyd-1.0.0.md` §1.5: catálogo não existe |
| Issues criadas | 0 | nenhuma ação no GitHub |

## 5. O que falta

`lacunas.md` é o dono do estado: `LCORE-01` (o que entra na `1.0.0` sem catálogo), `LCORE-02`
(estado em memória × durável), `LCORE-03` (divisão core × sim na regra de cálculo) e `LCORE-04`
(como o core fala com o banco sem SQL próprio).

## 6. Gate para sair do papel

1. Catálogo funcional do primeiro domínio escrito, com prova (`wyd-1.0.0.md` §4.2).
2. `lacunas.md` sem bloqueio para os requisitos da primeira versão.
3. `RF-CORE` numerados, com critério verificável e dono.
4. Árvore de `src/` e `tests/` planejada antes do primeiro arquivo.
5. Plano de teste determinístico: tick com semente fixa, sem `sleep` como sincronização.

## 7. Próximas páginas

`docs/arquitetura/fronteira.md`, `docs/specs/` com os `RF-CORE` do primeiro domínio,
`docs/planos/plano-de-testes.md` (determinismo) e `docs/roadmap/` com DoD.
