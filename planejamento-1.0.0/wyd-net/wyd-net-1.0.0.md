# wyd-net 1.0.0 — abertura do planejamento

> Pergunta: "o que o `wyd-net` faz, que contrato ele tem com o fio e com o `wyd-core`, e o que falta para começar o código?"
> Público: Produto, arquitetura e equipe do `wyd-net`
> Dono: equipe do `wyd-net`
> Base: `wyd-protocol-1.0.0.md` §1.4 e §10; `LPRT-08`, `LPRT-12`, `LPRT-13`; `AGENTS.md` §6
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22). Abertura inicial: fronteira, contratos e lacunas nomeadas.
**Nenhum código**, nenhum requisito numerado (`RF-NET`), nenhum roadmap com DoD e nenhum plano de teste
por alvo ainda. As decisões de fio que este sistema consome já existem e são citadas, não reabertas.

## 1. O que ele faz

Transporte TCP do CPSock na porta do canal: aceita conexão, mantém a sessão por socket, entrega ao
codec do `wyd-protocol` o buffer de cada conexão e devolve ao `wyd-core` a mensagem já tipada. É o
dono do que acontece com a conexão — timeout, limite por conexão, ordem e reação a erro — e não
decide o que a mensagem significa.

## 2. O que ele **não** faz

| Fora | Dono |
|---|---|
| layout do fio, cabeçalho, ofuscação, checksum, catálogo de `Type` | `wyd-protocol` |
| regra de jogo, tick, combate, cena | `wyd-core` / `wyd-sim` |
| topologia de processo e canal, lifecycle do servidor | `wyd-server` |
| persistência de sessão e de conta | `wyd-db` (`game.session`) |
| comando administrativo (kick, broadcast) | `wyd-admin` → `wyd-server` (`LAC-ADM-03`) |

## 3. Contratos

Estados: **decidido** (decisão registrada em outro documento), **proposto** (esta abertura),
**lacuna** (ver `lacunas.md`).

| # | Contrato | Direção | Forma | Garantia | Dono | Estado |
|---|---|---|---|---|---|---|
| N1 | `decode`/`encode` do codec | net → `wyd-protocol` | `decode(buf) -> Frame`, `encode(frame) -> bytes`, erro tipado | buffer completo por conexão; bytes crus não saem do crate | `wyd-protocol` | **decidido** (`LPRT-18`) |
| N2 | reação a erro de codec | net decide | `HeaderError`/`TransformError`/`PayloadError`, cada um recuperável ou fatal | classificação é do codec, reação é da conexão | `wyd-net` | **decidido** (`LPRT-13`) |
| N3 | `Type` desconhecido | net decide | `UnknownType` tipado, sem decisão no codec | log, descarte ou encerramento é política de conexão | `wyd-net` | **decidido** (`LPRT-08`) |
| N4 | ordem entre mensagens | net garante | por conexão, sem paralelismo de despacho | a ordem é requisito de combate e inventário | `wyd-net` | **decidido** (`LPRT-12`) |
| N5 | entrega ao `wyd-core` | net → core | mensagem tipada e validada em faixa | nenhum offset cru atravessa | `wyd-core` | **proposto** |
| N6 | limite por conexão, timeout e rate-limit | net decide | valores e política | sem valor medido ainda | `wyd-net` | **lacuna** `LNET-01` |
| N7 | versão de protocolo na sessão | net negocia | a definir | `LPRT-09` aberta: build-alvo não confirmado | `wyd-net` + `wyd-client` | **lacuna** `LNET-02` |

## 4. Estado medido (`V`, 2026-11-22)

| Medida | Valor | Comando |
|---|---:|---|
| Páginas de planejamento | 3 | `find planejamento-1.0.0/wyd-net -name '*.md'` |
| Arquivos de código | 0 | `find crates/wyd-net -type f` (só `.gitkeep`) |
| Requisitos com ID | 0 | esta abertura não numera requisito |
| Issues criadas | 0 | nenhuma ação no GitHub |

## 5. O que falta

`lacunas.md` é o dono do estado de cada lacuna: `LNET-01` (limites e política de conexão),
`LNET-02` (versão na sessão), `LNET-03` (modelo de sessão e identidade por conexão) e `LNET-04`
(observabilidade do transporte). Nenhuma tem dono fora do sistema além de `LNET-02`.

## 6. Gate para sair do papel

1. `lacunas.md` sem lacuna aberta que bloqueie requisito.
2. `RF-NET` numerados com critério de aceitação verificável.
3. Árvore de `src/` e `tests/` planejada antes do primeiro arquivo (`AGENTS.md` §5.10).
4. Plano de teste com alvos, dado e comando — incluindo teste de contrato com portas falsas.
5. Fronteira aprovada por arquitetura: nenhum offset cru cruzando para `wyd-core`.

## 7. Próximas páginas

`docs/arquitetura/fronteira.md` (tabela de seis perguntas, molde de `planejar-sistema`), `docs/specs/`
com os `RF-NET`, `docs/planos/plano-de-testes.md` e `docs/roadmap/` com DoD.
