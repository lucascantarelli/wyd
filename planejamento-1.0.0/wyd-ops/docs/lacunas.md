# `wyd-ops` — lacunas e decisões pendentes (registro único)

> Pergunta: "quais lacunas do `wyd-ops` estão abertas, quais fecharam e o que depende do dono ou de credencial externa?"
> Público: arquitetura, `wyd-ops`, dono do produto e agentes
> Dono: equipe do `wyd-ops`; aprovação de fronteira/contrato é da arquitetura
> Base: `wyd-ops/auditoria-de-organizacao.md`; `wyd-ops/wyd-ops-1.0.0.md`; `docs/specs/requisitos.md`; `docs/roadmap/roadmap.md`; `docs/arquitetura/componentes.md`; `AGENTS.md` §5 e §9
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Este é o único lugar onde uma lacuna do `wyd-ops` tem dono, impacto, estado e desbloqueio. Cada linha termina em `fechada` (com a evidência no repositório) ou `bloqueada` (com o item do pacote O1–O5 que a desbloqueia); nenhuma fica sem status. A criação deste arquivo fecha a lacuna de registro; não fecha fronteira, contrato, implementação ou decisão humana.

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
| Fronteira e contratos (`OPS-03`, 5 decisões) | 1 | 0 | 1 |
| Aplicação e operação | 2 | 0 | 2 |
| **Total** | **5** | **2** | **3** |

As cinco fecham assim: duas com evidência em §5 e três bloqueadas com o desbloqueio nomeado em §4 — nenhuma fica sem status.

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `OPS-01` | O `wyd-ops` tem dashboard, requisitos, plano de teste e roadmap com DoD? | sem esses artefatos o sistema não passa no checklist D-04 | arquitetura + `wyd-ops` | fechada (2026-11-25: `wyd-ops-1.0.0.md`, `docs/specs/requisitos.md` com 7 `RF-OPS` + 5 `RNF-OPS`, `docs/planos/plano-de-testes.md` e `docs/roadmap/index.md`/`roadmap.md` existem; gate verde) |
| `OPS-02` | O sistema tem registro único de lacuna e documentos sem afirmações obsoletas sobre artefatos ausentes? | sem registro, lacuna não tem dono; texto que diz "não existe" quando existe induz o agente ao erro | `wyd-ops` | fechada (2026-11-25: este `lacunas.md` criado; `wyd-ops-1.0.0.md` §4–§7, `docs/index.md` §"O que falta", `docs/arquitetura/componentes.md`, `docs/planos/plano-de-testes.md`, `docs/proposta-decisoes-ops.md` §2 e `docs/estrutura-de-pastas-e-modulos.md` alinhados; gate verde) |
| `OPS-03` | A fronteira e os contratos O1–O5 estão aprovados pelos donos e pelos consumidores? | contrato sem aprovação produz duas implementações incompatíveis; O4 e O5 ainda não têm forma fechada | arquitetura + `wyd-ops` + donos consumidores | bloqueada (2026-11-25: O1–O3 decididos na origem, O4 parcial e O5 sem forma fechada; as cinco decisões estão em §4, `OPS-03.1`–`OPS-03.5`, cada uma com o item do §7 do pacote que a desbloqueia) |
| `OPS-04` | A hospedagem de produção (`Q-OPS-2` = A) foi aplicada com provedor, contrato, VPS e backup? | a decisão escolheu VPS paga, mas nada foi provisionado; produção não existe | dono do produto + `wyd-ops` | bloqueada (2026-11-25: decisão registrada em `docs/proposta-decisoes-ops.md` §4/§6 e formalizada em `ADR-OPS-0002`; provedor, contrato, VPS, backup e RPO/RTO não executados; pedido em §7.1 do [pacote fora de O1–O5](pacote-aprovacao-decisoes-fora-o1-o5.md)) |
| `OPS-05` | A criação do board e das novas issues no GitHub foi autorizada? | o repositório já tem issues e milestones, mas criar board ou novas fatias é ato explícito | dono do produto | bloqueada (2026-11-25: leitura do GitHub achou **40 issues abertas** (`WYDDB-001`–`040`) e milestones existentes; `docs/github-mapping.md` está pronto; 0 boards e 0 issues criados nesta rodada; pedido em §7.1 do [pacote fora de O1–O5](pacote-aprovacao-decisoes-fora-o1-o5.md)) |

## 3. O que não é lacuna deste sistema

- Regra de jogo: dono é `wyd-core`/`wyd-sim`.
- Layout do fio: dono é `wyd-protocol`.
- Escrita administrativa: dono é `wyd-admin`.
- Conteúdo de portal: dono é `wyd-portal`.
- Implementação de código de cada sistema: cada sistema é dono; `wyd-ops` opera o pipeline.

## 4. Perguntas e recomendações consolidadas

Esta seção é a lista única para itens que exigem decisão humana, aprovação ou credencial externa. Eles **não** são fechados por documentação. A coluna **Status** é a definitiva: `bloqueada` com o item do §7 do [pacote O1–O5](pacote-aprovacao-o1-o5.md) ou do [pacote fora de O1–O5](pacote-aprovacao-decisoes-fora-o1-o5.md) que a desbloqueia — toda entrada bloqueada tem a sua mensagem. Os pacotes só organizam a leitura por dono e a [pauta de decisões pendentes](pauta-de-decisoes-pendentes.md) reúne os doze ponteiros; nenhum deles é um segundo registro.

| # | Item | Pergunta | Dono | Recomendação | ADR? | Lacuna | Status |
|---:|---|---|---|---|---|---|---|
| `OPS-03.1` | Contrato O1 — ambiente efêmero | O contrato `ops → wyd-bots` aprova Postgres e servidor descartáveis, ciclo de vida, cleanup e artefato por execução? | arquitetura + `wyd-ops` + `wyd-bots` | Aprovar a forma e o dono do ciclo; registrar em `wyd-ops-1.0.0.md` §3 e no plano de teste dos bots | **Não** — `LBOT-01`/`LBOT-02` já decidiram a política; falta o contrato operacional | `OPS-03` | bloqueada — [§7.4](pacote-aprovacao-o1-o5.md#74-wyd-bots-o1) (`wyd-bots`) com [§7.1](pacote-aprovacao-o1-o5.md#71-arquitetura-o1-a-o5) e [§7.3](pacote-aprovacao-o1-o5.md#73-wyd-ops-o1-a-o5) |
| `OPS-03.2` | Contrato O2 — métricas e logs | O contrato `wyd-server`/`wyd-net → ops` aprova nomes e labels de métrica, intervalo de coleta, retenção e schema de log? | arquitetura + `wyd-ops` + `wyd-server` + `wyd-net` | Aprovar a forma; retenção e alertas vêm do baseline de operação, não de número inventado | **Não** — `LNET-04` e `LAC-POR-06` já decidiram a política; falta a forma do contrato | `OPS-03` | bloqueada — [§7.5](pacote-aprovacao-o1-o5.md#75-wyd-server-o2) (`wyd-server`) e [§7.6](pacote-aprovacao-o1-o5.md#76-wyd-net-o2) (`wyd-net`) com [§7.1](pacote-aprovacao-o1-o5.md#71-arquitetura-o1-a-o5) e [§7.3](pacote-aprovacao-o1-o5.md#73-wyd-ops-o1-a-o5) |
| `OPS-03.3` | Contrato O3 — publicação do site | O contrato `ops → Pages` aprova artefato, workflow, janela de publicação e rollback? | arquitetura + `wyd-ops` + `wyd-docs` | Aprovar o contrato e exigir rollback ensaiado antes de publicar | **Não** — `LPDOC-01` e `ADR-DOC-0001` já decidiram a publicação como ato à parte; falta o contrato operacional | `OPS-03` | bloqueada — [§7.7](pacote-aprovacao-o1-o5.md#77-wyd-docs-o3) (`wyd-docs`) com [§7.1](pacote-aprovacao-o1-o5.md#71-arquitetura-o1-a-o5) e [§7.3](pacote-aprovacao-o1-o5.md#73-wyd-ops-o1-a-o5) |
| `OPS-03.4` | Contrato O4 — segredos | Quais segredos existem, quem emite, quem consome, onde ficam e como rotacionam? | arquitetura + `wyd-ops` | Inventariar antes de qualquer deploy; separar segredo de controle, DSN, chave de assinatura e token de serviço | **Sim** — `LAC-ADM-03` §3.3 cobre só o token de controle; inventário, armazenamento e rotação são decisão estrutural de segurança | `OPS-03` | bloqueada — [§7.1](pacote-aprovacao-o1-o5.md#71-arquitetura-o1-a-o5) e [§7.3](pacote-aprovacao-o1-o5.md#73-wyd-ops-o1-a-o5); ADR só depois do "sim" |
| `OPS-03.5` | Contrato O5 — deploy de servidor e launcher | Qual o contrato de staging, gatilho, aprovação, artefato e rollback do servidor e do launcher? | produto + arquitetura + `wyd-ops` | Definir o fluxo antes do primeiro deploy real; rollback precisa de comando e dono | **Sim** — operação e rollback de produção não estão cobertos por O1–O4 | `OPS-03` | bloqueada — [§7.2](pacote-aprovacao-o1-o5.md#72-produto-o5) (produto) com [§7.1](pacote-aprovacao-o1-o5.md#71-arquitetura-o1-a-o5) e [§7.3](pacote-aprovacao-o1-o5.md#73-wyd-ops-o1-a-o5); ADR só depois do "sim" |
| `OPS-04` | Hospedagem `Q-OPS-2` | Qual provedor, contrato, região e rotina de backup serão usados? | produto + `wyd-ops` | Escolher um provedor de VPS com Compose e definir RPO/RTO antes do provisionamento | **Não** — `ADR-OPS-0002` já decidiu a forma; falta aplicação, custo e credencial | `OPS-04` | bloqueada — [§7.1](pacote-aprovacao-decisoes-fora-o1-o5.md#71-produto-ops-04-ops-05-e-d-06) (produto + `wyd-ops`) |
| `OPS-05` | GitHub | Autorizar a criação do board e das novas issues a partir de `github-mapping.md`? | produto | O repositório já tem 40 issues e milestones; montar o board e criar as fatias restantes só depois de autorização explícita | **Não** — convenção de gestão, não arquitetura | `OPS-05` | bloqueada — [§7.1](pacote-aprovacao-decisoes-fora-o1-o5.md#71-produto-ops-04-ops-05-e-d-06) (produto) |
| `D-06` | Foco do projeto | Qual frente vem depois de `D-05`? | produto | Manter `wyd-db` até o gate e então `wyd-server`/`wyd-net` | **Não** — prioridade, não arquitetura | — | bloqueada — [§7.1](pacote-aprovacao-decisoes-fora-o1-o5.md#71-produto-ops-04-ops-05-e-d-06) (produto) |
| `Q-SERIE-1` | Série de migrations | Qual política de `down` para os 28 arquivos? | arquitetura + `wyd-db` | A) `down` só onde há reversão provada; B) nenhum `down`; C) todos com `down` | **Sim** — política de migrations e reversão é estrutural | — | bloqueada — [§7.2](pacote-aprovacao-decisoes-fora-o1-o5.md#72-arquitetura-e-wyd-db-q-serie-1-e-q-sep-1) (arquitetura + `wyd-db`) |
| `Q-SEP-1` | Separador de migration | Qual separador usar nos nomes? | arquitetura + `wyd-db` | A) `_` (já usado); B) `-` | **Não** — nomenclatura sem impacto estrutural | — | bloqueada — [§7.2](pacote-aprovacao-decisoes-fora-o1-o5.md#72-arquitetura-e-wyd-db-q-serie-1-e-q-sep-1) (arquitetura + `wyd-db`) |
| `LNET-01`/`LBOT-03` | Medições | Autorizar a implementação mínima de servidor/bot e a execução da medição? | QA + `wyd-ops` | Executar `docs/planos/medicoes-lnet-01-lbot-03.md` no CI efêmero | **Não** — política já decidida; falta medição, não ADR | — | **autorizada em 2026-11-25** (opção A) — falta executar a medição; tarefa em `docs/planos/medicoes-lnet-01-lbot-03.md`, que depende do ambiente de `OPS-03.1`; [§7.3](pacote-aprovacao-decisoes-fora-o1-o5.md#73-qa-e-wyd-ops-lnet-01-e-lbot-03) (QA + `wyd-ops`) |
| `LCLI-02`/`LCLI-05` | Direitos | Há liberação para distribuir o cliente e para a classificação jurídica? | produto + revisão jurídica | Manter congelado até resposta do titular | **Não** — direito e contrato, não arquitetura | — | **autorizada em 2026-11-25** (opção A de cada bloco; pedido `Q-IP-1`/`Q-IP-2` aprovado para envio) — `LCLI-02` fechou em 2026-11-25 (lista com hash, manifesto e verificador); falta a evidência do titular para `LCLI-05`; [§7.4](pacote-aprovacao-decisoes-fora-o1-o5.md#74-juridico-lcli-02-e-lcli-05) (produto + jurídico); `LCLI-02`/`LCLI-05` em `planejamento-1.0.0/wyd-client/lacunas.md` §1 |

## 5. Evidência de fechamento

| Lacuna | Evidência | Data |
|---|---|---|
| `OPS-01` | artefatos citados em §2; `docker compose --profile gate run --rm gate` verde | 2026-11-25 |
| `OPS-02` | este arquivo; documentos citados em §2; gate verde | 2026-11-25 |

## 6. O que esta página não cobre

- Implementação de CI, imagem, ambiente, monitoramento, segredos ou deploy.
- Decisão de provedor, contrato ou backup.
- Criação de issue, milestone, board, commit, push ou deploy.
- Envio de mensagem: os textos do §7 do pacote estão prontos e **não enviados**.
- Lacunas de outros sistemas: cada uma tem `lacunas.md` próprio.
