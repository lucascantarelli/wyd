# Decisões registradas — projeto WYD 1.0.0

> Pergunta: "quais decisões existem, em que estado e quem é o dono?"
> Público: Arquitetura, desenvolvimento e QA
> Dono: arquitetura
> Base: `AGENTS.md` §3, §5; `lacunas.md` de cada sistema; `planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/index.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Registro consolidado; não substitui `lacunas.md` do sistema dono.

## 1. Regra

Um fato, um dono (AGENTS.md §5, item 5). A decisão mora no documento dono; este arquivo **cita** por
ID e aponta o dono, nunca duplica o texto. Decisão estrutural (schema, fronteira, operação) vira ADR
quando aprovada (ver `adr/index.md`).

## 2. Decisões aceitas/decididas (resumo)

| ID | Sistema | Resumo | Estado | Dono | Onde |
|---|---|---|---|---|---|
| `ADR-DB-0001` | wyd-db | papel fora da série | aceito | wyd-db | adr/ |
| `ADR-DB-0002` | wyd-db | runner de migrations próprio | aceito | wyd-db | adr/ |
| `ADR-NET-0001` | wyd-net | política de limites (Q-NET-1 = A) | aceito (2026-11-25) | wyd-net | adr/ |
| `ADR-CORE-0001` | wyd-core | decisão C4 | aceito (2026-11-25) | wyd-core | adr/ |
| `ADR-SRV-0001` | wyd-server | captura S7 (peer de A1) | aceito (2026-11-25) | wyd-server | adr/ |
| `ADR-ADM-0001` | wyd-admin | canal de controle HTTP síncrono com ack | aceito (2026-11-25) | wyd-admin | adr/ |
| `ADR-POR-0001` | wyd-portal | leilão: BFF fecha, wyd-ops agenda/alerta | aceito (2026-11-25) | wyd-portal | adr/ |
| `ADR-DOC-0001` | wyd-docs | publicação é ato à parte | aceito (2026-11-25) | wyd-docs | adr/ |
| `ADR-CLI-0001` | wyd-client | uso e distribuição (Q-IP-3..Q-IP-6) | aceito (2026-11-25) | wyd-client | adr/ |
| `ADR-OPS-0001` | wyd-ops | workspace Cargo único, toolchain e versões (D-05 = A) | aceito (2026-11-25) | wyd-ops | adr/ |
| `ADR-OPS-0002` | wyd-ops | hospedagem de produção em VPS paga com Compose (Q-OPS-2 = A) | aceito (2026-11-25) | wyd-ops | adr/ |
| `ADR-LCH-0001` | wyd-launcher | manifesto canônico + assinatura Ed25519 | aceito (2026-11-23) | wyd-launcher | adr/ |
| `ADR-SIM-0001` | wyd-sim | aritmética inteira em bps, `f64` só em exibição | aceito (2026-11-23) | wyd-sim | adr/ |
| `ADR-PRT-0001` | wyd-protocol | build-alvo 7662 | aceito (2026-11-23) | wyd-protocol | adr/ |
| `ADR-BOT-0001` | wyd-bots | ambiente CI efêmero com seed descartável | aceito (2026-11-23) | wyd-bots | adr/ |
| `D-05` = A | wyd-ops | workspace, toolchain, Makefile, lock único e digests | decidida 2026-11-25 | wyd-ops | dossie-d-05.md; ADR-OPS-0001 |
| `Q-OPS-2` = A | wyd-ops | VPS paga com processo persistente e Compose | decidida 2026-11-24 | wyd-ops | proposta-decisoes-ops.md; ADR-OPS-0002 |
| `LNET-01` | wyd-net | limites (medição) | **aberta como medição** (política decidida) | wyd-net | proposta-decisoes-net.md |
| `LSRV-05` | wyd-server | captura (peer de A1) | decidido (2026-11-25) | wyd-server | wyd-server-1.0.0.md |
| `LPRT-03`,`LPRT-06` | wyd-protocol | captura do cliente 7662 | decididos (2026-11-25) | wyd-protocol | lacunas.md |
| `LCLI-02`,`LCLI-03`,`LCLI-05` | wyd-client | uso/distribuição | decididos (2026-11-25) | wyd-client | lacunas.md |
| `LBOT-03` | wyd-bots | carga (medição) | **aberta como medição** (política decidida) | wyd-bots | lacunas.md |
| `LPDOC-12` | wyd-docs | Windows | decidido (2026-11-25) | wyd-docs | lacunas.md |
| `Q-IP-1`,`Q-IP-2` | wyd-client | pedido externo iniciado | decididos (2026-11-25) | wyd-client | lacunas.md |
| `Q-NET-1` = A | wyd-net | política de limites | decidido | wyd-net | proposta-decisoes-net.md |
| `C4` | wyd-core | decidida 2026-11-23 | decidido | wyd-core | wyd-core-1.0.0.md |
| `LAC-ADM-03` | wyd-admin | canal de controle HTTP síncrono com ack | decidido | wyd-admin | contrato-c4.md |
| `LAC-POR-09` | wyd-portal | BFF fecha leilão; wyd-ops agenda/alerta | fechada 2026-11-23 | wyd-portal | lacunas.md |
| `LPDOC-08` | wyd-docs | publicação é ato à parte | fechada 2026-11-19 | wyd-docs | lacunas.md |
| `Q-IP-3`–`Q-IP-6` | wyd-client | uso/distribuição | decididas | wyd-client | propriedade-intelectual-e-distribuicao.md |
| `LSIM-01`–`04` | wyd-sim | todas fechadas 2026-11-23 | fechadas | wyd-sim | lacunas.md |
| `LLCH-01`–`04` | wyd-launcher | fechadas 2026-11-23 | fechadas | wyd-launcher | lacunas.md |
| `LBOT-01`,`02`,`04` | wyd-bots | fechadas 2026-11-23 | fechadas | wyd-bots | lacunas.md |
| `D-04` = A | todos | checklist de organização | fechada 2026-11-25 | todos | auditoria-de-organizacao.md |

## 3. Decisões abertas/propostas (não bloqueiam organização)

As decisões que ainda exigem resposta ou execução estão consolidadas em
`wyd-ops/docs/lacunas.md` §4: `D-06`, `Q-SERIE-1`, `Q-SEP-1`, `LNET-01`/`LBOT-03` (medição),
`LCLI-02`/`LCLI-05` (direitos) e `OPS-03`–`OPS-05`. Esta página não duplica a lista.

## 4. O que esta página não cobre

- **Texto integral da decisão**: mora no documento dono (citado acima).
- **ADR formal**: 15 existem; candidatos listados em `adr/index.md` §4.

## 5. Páginas de issue

As **101** páginas de issue planejadas (40 `WYDDB` + 61 por fatia dos 11 sistemas) não
são decisão: vivem em `<sistema>/docs/roadmap/` e estão indexadas no índice de roadmap de cada
sistema, com o inventário medido em `github-mapping.md` §7.
