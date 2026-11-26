# Pauta de decisões pendentes — o que falta aprovar para fechar o planejamento

> Pergunta: "o que ainda depende de um sim do dono para o planejamento fechar, quem responde e o que cada resposta libera?"
> Público: dono do produto, arquitetura, `wyd-ops`, QA, `wyd-db`, revisão jurídica e agentes
> Dono: equipe do `wyd-ops` (consolidação); cada decisão pertence ao dono listado na linha
> Base: `docs/lacunas.md` §4; `docs/pacote-aprovacao-o1-o5.md` §7; `docs/pacote-aprovacao-decisoes-fora-o1-o5.md` §7; `docs/arquitetura/adr/candidatos.md`; varredura de 2026-11-25 descrita em §4
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Ponto de entrada único das decisões de dono. Ele **não decide, não aprova e não envia nada**: cada linha é um resumo que aponta para a mensagem canônica do §7 do pacote de origem, e o registro de estado continua sendo `docs/lacunas.md` §4. Em 2026-11-25 o dono respondeu **às doze** — duas na primeira rodada e dez na segunda (§2.1) —; `Q-SERIE-1` foi implementada e provada como ciclo SQL em PG17, e `Q-SEP-1` foi **reaberta** porque a resposta A não parseia 26 dos 28 nomes.

## 1. Como usar

| Elemento | Onde vive |
|---|---|
| Pergunta completa, forma de responder e impacto detalhado | §7 do pacote de origem, ancorado na coluna **Mensagem** |
| Estado da lacuna, dono e ADR | `docs/lacunas.md` §4 |
| Texto pronto para enviar ao dono | §7 do pacote indicado na linha |
| Esta página | índice de leitura: uma linha por decisão, sem repetir o texto da mensagem |

Status possíveis: **bloqueada** (sem o sim) ou **fechada** com evidência. Enquanto a linha estiver bloqueada, nada do que ela libera pode ser implementado, publicado ou provisionado.

## 2. As 12 decisões de dono

| ID em `lacunas.md` §4 | Decisão | Dono | Pergunta objetiva | O que a aprovação libera | ADR | Se ficar aberta | Mensagem |
|---|---|---|---|---|---|---|---|
| `OPS-03.1` | O1 — ambiente efêmero | arquitetura + `wyd-ops` + `wyd-bots` | Postgres e servidor descartáveis por execução, com ciclo, cleanup e artefato, estão aprovados? | ambiente isolado dos bots; execução de `LBOT-03` no CI; plano de teste dos bots | não | bots sem ambiente aprovado e medição sem CI | [§7.4](pacote-aprovacao-o1-o5.md#74-wyd-bots-o1) e [§7.3](pacote-aprovacao-o1-o5.md#73-wyd-ops-o1-a-o5) |
| `OPS-03.2` | O2 — métricas e logs | arquitetura + `wyd-ops` + `wyd-server` + `wyd-net` | Nomes e labels de métrica, intervalo, retenção e schema de log estão aprovados? | `RF-OPS-004` implementável; dashboards, alertas e diagnóstico | não | monitoramento sem formato; incidente manual | [§7.5](pacote-aprovacao-o1-o5.md#75-wyd-server-o2) e [§7.6](pacote-aprovacao-o1-o5.md#76-wyd-net-o2) |
| `OPS-03.3` | O3 — publicação do site | arquitetura + `wyd-ops` + `wyd-docs` | Artefato, workflow, janela de publicação e rollback estão aprovados? | publicação reproduzível e rollback exigível | não | cada publicação vira exceção manual | [§7.7](pacote-aprovacao-o1-o5.md#77-wyd-docs-o3) |
| `OPS-03.4` | O4 — segredos | arquitetura + `wyd-ops` | Inventário, emissor, consumidor, armazenamento e rotação estão aprovados? | deploy de servidor e admin; auditoria de segurança | sim, após o sim | deploy bloqueado; segredo sem dono | [§7.1](pacote-aprovacao-o1-o5.md#71-arquitetura-o1-a-o5) e [§7.3](pacote-aprovacao-o1-o5.md#73-wyd-ops-o1-a-o5) |
| `OPS-03.5` | O5 — deploy e rollback | produto + arquitetura + `wyd-ops` | Staging, gatilho, aprovação, artefato e rollback de servidor e launcher estão aprovados? | primeiro deploy real; release com critério de volta | sim, após o sim | sem staging nem rollback ensaiado | [§7.2](pacote-aprovacao-o1-o5.md#72-produto-o5) |
| `OPS-04` | Hospedagem `Q-OPS-2` | produto + `wyd-ops` | Provedor, contrato, região, backup e RPO/RTO estão definidos? | produção provisionada; rotina de backup e restauração | não | produção inexistente | [§7.1](pacote-aprovacao-decisoes-fora-o1-o5.md#71-produto-ops-04-ops-05-e-d-06) |
| `OPS-05` | Board e novas issues | produto | Autoriza criar o board e as novas issues do `github-mapping.md`? | board montado e fatias restantes criadas | não | **executada em 2026-11-26** — 61 fatias (#129–#189) e 10 lacunas (#190–#199) com `area/*`; board pendente | [§7.1](pacote-aprovacao-decisoes-fora-o1-o5.md#71-produto-ops-04-ops-05-e-d-06) |
| `D-06` | Próxima frente | produto | Qual frente vem depois de `D-05`? | ordem de execução explícita | não | cada agente escolhe a própria ordem | [§7.1](pacote-aprovacao-decisoes-fora-o1-o5.md#71-produto-ops-04-ops-05-e-d-06) |
| `Q-SERIE-1` | Política de `down` | arquitetura + `wyd-db` | Qual política para os arquivos que só têm `up`? Opções e recomendação em `planejamento-1.0.0/wyd-db/migrations/serie-e-regras.md` §9 | política de reversão da série de migrations | sim, se a opção dispensar `down` ou marcar baseline | **decidida em B** — 28 `down` escritos e ciclo SQL provado em PG17 | [§7.2](pacote-aprovacao-decisoes-fora-o1-o5.md#72-arquitetura-e-wyd-db-q-serie-1-e-q-sep-1) |
| `Q-SEP-1` | Separador/slug de migration | arquitetura + `wyd-db` | Decidir A′/B′/C′ para 26 slugs com hífen recusados por `Slug`; detalhe em §9.1 | nomenclatura estável e série carregável | não | **decidida em A′ e aplicada em 2026-11-25** — 28 nomes com `_`; todos parseiam | [§7.2](pacote-aprovacao-decisoes-fora-o1-o5.md#72-arquitetura-e-wyd-db-q-serie-1-e-q-sep-1) |
| `LNET-01`/`LBOT-03` | Medições de rede e carga | QA + `wyd-ops` | Autoriza a implementação mínima e a execução da medição no CI efêmero? | valores medidos; limites e carga com base | não | **autorizada em 2026-11-25** — falta executar a medição; sem execução não há valor medido nem P50/P99 citável | [§7.3](pacote-aprovacao-decisoes-fora-o1-o5.md#73-qa-e-wyd-ops-lnet-01-e-lbot-03) |
| `LCLI-02`/`LCLI-05` | Direitos do cliente | produto + revisão jurídica | Há liberação do titular para distribuir o cliente e para a classificação jurídica? | distribuição e patch do cliente | não | **autorizada em 2026-11-25** (opção A de cada bloco; pedido ao titular aprovado para envio) — `LCLI-02` foi executada e fechada em 2026-11-25 (lista, manifesto e verificador); falta a evidência do titular para `LCLI-05` e o cliente segue congelado | [§7.4](pacote-aprovacao-decisoes-fora-o1-o5.md#74-juridico-lcli-02-e-lcli-05) |

**Total: 12 decisões de dono; todas têm resposta registrada em 2026-11-25** — duas na primeira rodada e
dez na segunda (§2.1) —, mais **3 escolhas técnicas** da série (`Q-ALVO-1`–`03`, §2.2). Nenhuma se fecha
por documentação: resta execução (`LNET-01`/`LBOT-03`, board), `CAP-38`/`CAP-39`, ADR (`OPS-03.4`,
`OPS-03.5`), artefato de terceiro (`LCLI-05`) ou o adiamento declarado (`OPS-04`).

## 2.1 Respostas de 2026-11-25 (segunda rodada)

| Decisão | Resposta do dono | O que muda |
|---|---|---|
| `D-06` | **A** — finalizar o `wyd-db` | a ordem das frentes deixa de ser implícita; `D-05` vem depois |
| `Q-SERIE-1` | **escrever os 28 `down` reais** | mantém `ADR-DB-0002`; os 28 `down` e o ciclo `up → down → up` passaram em PG17 por `psql`; `CAP-37` foi fechada por `runner_pg` |
| `Q-SEP-1` | **ler `-` e `_`, usar `_` nos arquivos novos** — depois substituída por **A′** | `Slug` recusava hífen em 26 slugs; A′ renomeou os 28 para `_` e todos parseiam (§9.1) |
| `OPS-03.1`–`OPS-03.3` | **aprovados na forma** | `LBOT-03` ganha ambiente, `RF-OPS-004` ganha formato, a publicação ganha contrato e rollback exigível |
| `OPS-03.4`, `OPS-03.5` | **aprovados na forma, com ADR antes de implementar** | segredos e deploy ficam a um ADR de distância; nada bloqueado por falta de “sim” |
| `OPS-05` | **executado em 2026-11-26** | 61 fatias (#129–#189) e 10 lacunas (#190–#199) criadas com labels `area/*`; board pendente |
| `OPS-04` | **adiada** para depois do primeiro deploy interno | provedor, backup e RPO/RTO continuam indefinidos, sem número inventado |

## 2.2 Escolhas técnicas da prova do `PgAdapter` (2026-11-25)

Não são decisões de dono do produto: são as três perguntas da
`planejamento-1.0.0/wyd-db/proposta-alvo-pgadapter.md` §6, respondidas pela equipe do `wyd-db` e
arquitetura, e implementadas no alvo `runner_pg`.

| Decisão | Resposta | O que mudou |
|---|---|---|
| `Q-ALVO-1` — transporte | **A1** — porta publicada em `127.0.0.1` | job `runner` do `code.yml` usa `-p 127.0.0.1::5432`; nunca `0.0.0.0` |
| `Q-ALVO-2` — série | **B1** — fixture contígua agora | `tests/resources/fixture/` (4 arquivos) e `fixture_falha/` (2) |
| `Q-ALVO-3` — papel | **`wyd_migrator` com `LOGIN` só no teste** | sem `allow_superuser(true)`; o preflight é exercitado de verdade |

Resultado: `cargo test --test runner_pg` **5/5** em PG17 (log `/tmp/wyd-runner-pg.log`), fechando
`CAP-37`. Detalhe em `../../wyd-db/tests/alvos/runner-pg.md`.

## 3. Quem responde o quê

| Dono | Decisões |
|---|---|
| produto | `OPS-03.5`, `OPS-04`, `OPS-05`, `D-06`, `LCLI-02`/`LCLI-05` |
| arquitetura | as 12 (aprova a forma de cada uma) |
| `wyd-ops` | as 12 (é dono do contrato ou da operação) |
| `wyd-bots` | `OPS-03.1` |
| `wyd-server` | `OPS-03.2` |
| `wyd-net` | `OPS-03.2` |
| `wyd-docs` | `OPS-03.3` |
| `wyd-db` | `Q-SERIE-1`, `Q-SEP-1` |
| QA | `LNET-01`/`LBOT-03` |
| revisão jurídica | `LCLI-02`/`LCLI-05` |

## 4. Varredura: o que não é decisão de dono

Critério medido em 2026-11-25: busca por "decisão pendente", "pendente de aprovação", "sem resposta", "não decidid", "em aberto", "espera o dono", "aguarda o dono", "continua aberta" e "segue proposta" em `planejamento-1.0.0/**/*.md`, mais os estados de cada `lacunas.md` e o backlog de `docs/arquitetura/adr/candidatos.md`. Resultado: **nenhuma decisão de dono fora das 12 acima**; os dois achados da varredura de 2026-11-25 estão nas linhas abaixo — `Q-CLI-2` já foi decidido e o pedido ao titular ainda não foi enviado. O que a varredura encontrou são bloqueios de execução, evidência ou terceiro — não pedidos de aprovação —, listados aqui para que não pareçam esquecidos:

| Item | Por que não é decisão de dono | Onde está |
|---|---|---|
| `LPRT-03` e `LPRT-06` | captura A1 do pacote 7662 já autorizada em 2026-11-23; falta executar a captura e registrar o golden | `planejamento-1.0.0/wyd-protocol/lacunas.md` §1 |
| `LCLI-03` | segunda leitura com QA definido; falta a ficha assinada com o nome do operador | `planejamento-1.0.0/wyd-client/lacunas.md` §1 |
| `Q-DP-2/validação` | o artefato foi escolhido em 2026-11-24 (parecer jurídico por finalidade); falta produzir o parecer | `planejamento-1.0.0/wyd-db/pendencias-de-fechamento.md` §3.3.1 |
| `Q-RET-7/aplicação` | a rotina foi aprovada em 2026-11-24; fornecedor, RPO/RTO e ensaio de restore dependem da decisão `OPS-04`, já nesta pauta | `planejamento-1.0.0/wyd-db/pendencias-de-fechamento.md` §3.3 |
| `Q-CAP-3` | sinal de capacidade sem limiar; o limite só existe depois da medição | `planejamento-1.0.0/wyd-db/docs/arquitetura/retencao-e-capacidade.md` |
| seis seeds de conteúdo | bloqueio de conteúdo do piloto, com dono no `wyd-db`; não pede aprovação de arquitetura | `planejamento-1.0.0/wyd-db/pendencias-de-fechamento.md` |
| `Q-CLI-2` (o que o patch entrega e como versiona) | decisão aprovada em 2026-11-25, registrada em `planejamento-1.0.0/wyd-client/lacunas.md` §2; falta entregar o manifesto/lista — não há pergunta ao dono | `planejamento-1.0.0/wyd-client/lacunas.md` §2 |
| envio do pedido ao titular (`Q-IP-1`/`Q-IP-2`) | o pedido está pronto e **não enviado**: falta o dono confirmar signatário e dados e abrir o ticket; é ato do dono, não escolha de arquitetura, e precede qualquer resposta do titular de `LCLI-05` | `planejamento-1.0.0/wyd-client/docs/specs/pedido-ao-titular.md` §7 |

Medições futuras de `adr/candidatos.md` (limites medidos, captura validada, primeiro deploy) são gatilhos de ADR, não decisões pendentes: elas só existem depois de executar o que as linhas acima liberam.

## 5. Como esta pauta se mantém

1. O dono responde na mensagem do §7 indicado.
2. A resposta é registrada em `docs/lacunas.md` §4, com data e evidência; só então a linha daqui muda de status.
3. Se aprovada e o ADR for exigido, o ADR é escrito antes da implementação, sem numerar antes do sim.
4. Se a resposta for "não", a objeção volta ao dono e a linha continua bloqueada — esta página não arbitra.

## 6. Limites

- Não cria issue, board, commit, push, deploy, ADR ou mensagem; nada foi enviado.
- Não substitui `docs/lacunas.md` §4 nem o §7 dos dois pacotes: só reúne os ponteiros.
- Não reabre decisão já fechada nem conta como resposta o que não está registrado.
