# `wyd-ops` — lacunas e decisões pendentes (registro único)

> Pergunta: "quais lacunas do `wyd-ops` estão abertas, quais fecharam e o que depende do dono ou de credencial externa?"
> Público: arquitetura, `wyd-ops`, dono do produto e agentes
> Dono: equipe do `wyd-ops`; aprovação de fronteira/contrato é da arquitetura
> Base: `wyd-ops/auditoria-de-organizacao.md`; `wyd-ops/wyd-ops-1.0.0.md`; `docs/specs/requisitos.md`; `docs/roadmap/roadmap.md`; `docs/arquitetura/componentes.md`; `AGENTS.md` §5 e §9
> Revisado em: 2026-11-26

**Estado:** proposto (2026-11-26). Este é o único lugar onde uma lacuna do `wyd-ops` tem dono, impacto, estado e desbloqueio. Cada linha termina em `fechada` (com a evidência no repositório) ou `bloqueada` (com o item do pacote O1–O5 que a desbloqueia); nenhuma fica sem status. A criação deste arquivo fecha a lacuna de registro; não fecha fronteira, contrato, implementação ou decisão humana.

## 0. Como ler

| Estado | Significado |
|---|---|
| **fechada** | decidida ou produzida, com evidência no arquivo citado; não volta a ser perguntada |
| **bloqueada** | sem o "sim" do dono; o desbloqueio aponta a mensagem do §7 do [pacote O1–O5](pacote-aprovacao-o1-o5.md) ou do [pacote fora de O1–O5](pacote-aprovacao-decisoes-fora-o1-o5.md) |

Dono: `arquitetura` propõe; o dono do sistema aprova. Fechar lacuna sem o dono é proibido (`AGENTS.md` §5, item 8).

## 1. Situação em uma linha

| Grupo | Lacunas | Fechadas | Bloqueadas |
|---|---:|---:|---:|
| Organização e registro | 2 | 2 | 0 |
| Fronteira e contratos (`OPS-03`, 5 decisões) | 1 | 1 | 0 |
| Aplicação e operação | 2 | 0 | 2 |
| **Total** | **5** | **3** | **2** |

As cinco fecham assim: três com evidência em §5 e duas bloqueadas com o desbloqueio nomeado em §4 — nenhuma fica sem status. Em 2026-11-25 o dono respondeu as cinco decisões de `OPS-03` e as duas de aplicação (`OPS-04`, `OPS-05`): `OPS-03.1`–`OPS-03.5` fecharam, `OPS-04` ficou adiada e `OPS-05` teve as issues e labels executados em 2026-11-26, com o board pendente.

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `OPS-01` | O `wyd-ops` tem dashboard, requisitos, plano de teste e roadmap com DoD? | sem esses artefatos o sistema não passa no checklist D-04 | arquitetura + `wyd-ops` | fechada (2026-11-25: `wyd-ops-1.0.0.md`, `docs/specs/requisitos.md` com 7 `RF-OPS` + 5 `RNF-OPS`, `docs/planos/plano-de-testes.md` e `docs/roadmap/index.md`/`roadmap.md` existem; gate verde) |
| `OPS-02` | O sistema tem registro único de lacuna e documentos sem afirmações obsoletas sobre artefatos ausentes? | sem registro, lacuna não tem dono; texto que diz "não existe" quando existe induz o agente ao erro | `wyd-ops` | fechada (2026-11-25: este `lacunas.md` criado; `wyd-ops-1.0.0.md` §4–§7, `docs/index.md` §"O que falta", `docs/arquitetura/componentes.md`, `docs/planos/plano-de-testes.md`, `docs/proposta-decisoes-ops.md` §2 e `docs/estrutura-de-pastas-e-modulos.md` alinhados; gate verde) |
| `OPS-03` | A fronteira e os contratos O1–O5 estão aprovados pelos donos e pelos consumidores? | contrato sem aprovação produz duas implementações incompatíveis; O4 e O5 ainda não tinham ADR | arquitetura + `wyd-ops` + donos consumidores | fechada (2026-11-26: `OPS-03.1`–`OPS-03.3` fechadas em 2026-11-25; `OPS-03.4` formalizada em `ADR-OPS-0011` e `OPS-03.5` no `ADR-OPS-0007` revisado; implementação continua em F4/OPS-04) |
| `OPS-04` | A hospedagem de produção (`Q-OPS-2` = A) foi aplicada com provedor, contrato, VPS e backup? | a decisão escolheu VPS paga, mas nada foi provisionado; produção não existe | dono do produto + `wyd-ops` | bloqueada (2026-11-25: decisão registrada em `docs/proposta-decisoes-ops.md` §4/§6 e formalizada em `ADR-OPS-0002`; **em 2026-11-25 o dono adiou** provedor, região, backup e RPO/RTO para depois do primeiro deploy interno e informou que não há valores definidos; nada provisionado) |
| `OPS-05` | A criação do board e das novas issues no GitHub foi autorizada? | o repositório já tem issues e milestones, mas criar board ou novas fatias é ato explícito | dono do produto | **bloqueada** — autorizada em 2026-11-25 e executada em 2026-11-26 para as 61 fatias (#129–#189) e 10 lacunas (#190–#199), com labels `area/*`; falta o board do Project |

## 3. O que não é lacuna deste sistema

- Regra de jogo: dono é `wyd-core`/`wyd-sim`.
- Layout do fio: dono é `wyd-protocol`.
- Escrita administrativa: dono é `wyd-admin`.
- Conteúdo de portal: dono é `wyd-portal`.
- Implementação de código de cada sistema: cada sistema é dono; `wyd-ops` opera o pipeline.

## 4. Perguntas e recomendações consolidadas

Esta seção é o **ledger de estado** dos itens que exigem decisão humana, aprovação ou credencial externa. Eles **não** são fechados por documentação. A coluna **Status** é a definitiva: `bloqueada` com o item do §7 do [pacote O1–O5](pacote-aprovacao-o1-o5.md) ou do [pacote fora de O1–O5](pacote-aprovacao-decisoes-fora-o1-o5.md) que a desbloqueia — toda entrada bloqueada tem a sua mensagem. Neste sistema, restam duas bloqueadas (`OPS-04` e `OPS-05`); as cinco decisões de contrato fecharam. Os pacotes só organizam a leitura por dono e a [pauta canônica](pauta-de-decisoes-pendentes.md) reúne as **onze pendências abertas** com opções, recomendação e critério de pronto; esta seção registra o estado e não é um segundo registro de decisão.

| # | Item | Pergunta | Dono | Recomendação | ADR? | Lacuna | Status |
|---:|---|---|---|---|---|---|---|
| `OPS-03.1` | Contrato O1 — ambiente efêmero | O contrato `ops → wyd-bots` aprova Postgres e servidor descartáveis, ciclo de vida, cleanup e artefato por execução? | arquitetura + `wyd-ops` + `wyd-bots` | Aprovar a forma e o dono do ciclo; registrar em `wyd-ops-1.0.0.md` §3 e no plano de teste dos bots | **Não** — `LBOT-01`/`LBOT-02` já decidiram a política; falta o contrato operacional | `OPS-03` | **fechada em 2026-11-25** — forma aprovada pelo dono; contrato em `wyd-ops-1.0.0.md` §3; a execução do CI efêmero é de `WYDOPS-F*` e `LBOT-03` |
| `OPS-03.2` | Contrato O2 — métricas e logs | O contrato `wyd-server`/`wyd-net → ops` aprova nomes e labels de métrica, intervalo de coleta, retenção e schema de log? | arquitetura + `wyd-ops` + `wyd-server` + `wyd-net` | Aprovar a forma; retenção e alertas vêm do baseline de operação, não de número inventado | **Não** — `LNET-04` e `LAC-POR-06` já decidiram a política; falta a forma do contrato | `OPS-03` | **fechada em 2026-11-25** — forma aprovada; `RF-OPS-004` implementável; retenção e alerta vêm do baseline de operação |
| `OPS-03.3` | Contrato O3 — publicação do site | O contrato `ops → Pages` aprova artefato, workflow, janela de publicação e rollback? | arquitetura + `wyd-ops` + `wyd-docs` | Aprovar o contrato e exigir rollback ensaiado antes de publicar | **Não** — `LPDOC-01` e `ADR-DOC-0001` já decidiram a publicação como ato à parte; falta o contrato operacional | `OPS-03` | **fechada em 2026-11-25** — forma aprovada; rollback ensaiado exigido antes de publicar |
| `OPS-03.4` | Contrato O4 — segredos | Quais segredos existem, quem emite, quem consome, onde ficam e como rotacionam? | arquitetura + `wyd-ops` | Inventariar antes de qualquer deploy; separar segredo de controle, DSN, chave de assinatura e token de serviço | **Sim** — `LAC-ADM-03` §3.3 cobre só o token de controle; inventário, armazenamento e rotação são decisão estrutural de segurança | `OPS-03` | **fechada em 2026-11-26** — aprovada na forma em 2026-11-25 e formalizada em `ADR-OPS-0011`, com inventário por classe, armazenamento e gatilhos de rotação; nenhum valor ou provedor foi inventado |
| `OPS-03.5` | Contrato O5 — deploy de servidor e launcher | Qual o contrato de staging, gatilho, aprovação, artefato e rollback do servidor e do launcher? | produto + arquitetura + `wyd-ops` | Definir o fluxo antes do primeiro deploy real; rollback precisa de comando e dono | **Sim** — operação e rollback de produção não estão cobertos por O1–O4 | `OPS-03` | **fechada em 2026-11-26** — aprovada na forma em 2026-11-25 e formalizada no `ADR-OPS-0007` revisado (artefato imutável, deploy manual com aprovação, rollback por digest/manifesto); staging, provedor e janela continuam em `OPS-04` |
| `OPS-04` | Hospedagem `Q-OPS-2` | Qual provedor, contrato, região e rotina de backup serão usados? | produto + `wyd-ops` | Escolher um provedor de VPS com Compose e definir RPO/RTO antes do provisionamento | **Não** — `ADR-OPS-0002` já decidiu a forma; falta aplicação, custo e credencial | `OPS-04` | **adiada em 2026-11-25** pelo dono para depois do primeiro deploy interno; provedor, região, backup e RPO/RTO indefinidos (o dono informou não haver valores) |
| `OPS-05` | GitHub | Autorizar a criação do board e das novas issues a partir de `github-mapping.md`? | produto | O repositório já tem 40 issues e milestones; montar o board e criar as fatias restantes só depois de autorização explícita | **Não** — convenção de gestão, não arquitetura | `OPS-05` | **bloqueada** — autorizada em 2026-11-25 e parcialmente executada em 2026-11-26: 61 fatias (#129–#189) e 10 lacunas (#190–#199) criadas com `area/*`; falta o board do Project |
| `D-06` | Foco do projeto | Qual frente vem depois de `D-05`? | produto | Manter `wyd-db` até o gate e então `wyd-server`/`wyd-net` | **Não** — prioridade, não arquitetura | — | **decidida em 2026-11-25: A** — foco no `wyd-db` (142 atos, 64 células, 40 issues); D-05 executada depois |
| `Q-SERIE-1` | Série de migrations | Qual política para arquivos sem `down`? Opções e recomendação em `planejamento-1.0.0/wyd-db/migrations/serie-e-regras.md` §9 | arquitetura + `wyd-db` | Ler a recomendação e responder sim ou apontar objeção | **Sim** — política estrutural | — | **B decidida, implementada e provada em PG17 via `psql` em 2026-11-25**; a porta foi provada depois por `runner_pg` e `CAP-37` fechou |
| `Q-SEP-1` | Separador/slug de migration | A resposta original A foi incompatível: `Slug` rejeita hífen interno em 26/28 slugs; alternativas revisadas em `serie-e-regras.md` §9.1 | arquitetura + `wyd-db` | Decidir A′/B′/C′ antes de renomear arquivo ou alterar parser | **Não** — escolha de compatibilidade/nomenclatura | — | **A′ decidida e aplicada em 2026-11-25**: 28 arquivos renomeados para `_`; todos parseiam; parser intacto |
| `LNET-01`/`LBOT-03` | Medições | Autorizar a implementação mínima de servidor/bot e a execução da medição? | QA + `wyd-ops` | Executar `docs/planos/medicoes-lnet-01-lbot-03.md` no CI efêmero | **Não** — política já decidida; falta medição, não ADR | — | **autorizada em 2026-11-25** (opção A) — falta executar a medição; tarefa em `docs/planos/medicoes-lnet-01-lbot-03.md`, que depende do ambiente de `OPS-03.1`; [§7.3](pacote-aprovacao-decisoes-fora-o1-o5.md#73-qa-e-wyd-ops-lnet-01-e-lbot-03) (QA + `wyd-ops`) |
| `LCLI-02`/`LCLI-05` | Direitos | Há liberação para distribuir o cliente e para a classificação jurídica? | produto + revisão jurídica | Manter congelado até resposta do titular | **Não** — direito e contrato, não arquitetura | — | **autorizada em 2026-11-25** (opção A de cada bloco; pedido `Q-IP-1`/`Q-IP-2` aprovado para envio) — `LCLI-02` fechou em 2026-11-25 (lista com hash, manifesto e verificador); falta a evidência do titular para `LCLI-05`; [§7.4](pacote-aprovacao-decisoes-fora-o1-o5.md#74-juridico-lcli-02-e-lcli-05) (produto + jurídico); `LCLI-02`/`LCLI-05` em `planejamento-1.0.0/wyd-client/lacunas.md` §1 |

## 5. Evidência de fechamento

| Lacuna | Evidência | Data |
|---|---|---|
| `OPS-01` | artefatos citados em §2; `docker compose --profile gate run --rm gate` verde | 2026-11-25 |
| `OPS-02` | este arquivo; documentos citados em §2; gate verde | 2026-11-25 |
| `OPS-03.1`–`OPS-03.3` | aprovação do dono registrada em §4 e contrato em `wyd-ops-1.0.0.md` §3; gate verde | 2026-11-25 |
| `OPS-03.4` | `ADR-OPS-0011` aceito, com inventário por classe, armazenamento e gatilhos de rotação; referências conferidas nos workflows | 2026-11-26 |
| `OPS-03.5` | `ADR-OPS-0007` revisado com o contrato O5 (artefato, aprovação, rollback); staging/provedor em `OPS-04` | 2026-11-26 |

## 6. O que esta página não cobre

- Implementação de CI, imagem, ambiente, monitoramento, segredos ou deploy.
- Decisão de provedor, contrato ou backup.
- Criação de issue, milestone, board, commit, push ou deploy.
- Envio de mensagem: os textos do §7 do pacote estão prontos e **não enviados**.
- Lacunas de outros sistemas: cada uma tem `lacunas.md` próprio.
