> Pergunta: "o que ainda não está decidido no portal, quem decide e o que trava enquanto isso?"
> Público: arquitetura, produto e desenvolvedor novo
> Dono: arquitetura
> Base: `docs/arquitetura/fronteira.md`; `docs/arquitetura/consumo-do-banco.md`; `M21`; `AGENTS.md` §5.4
> Revisado em: 2026-11-20

**Estado:** aceito (2026-11-20)

Registro único de lacuna do `wyd-portal`. Lacuna sem dono não entra; lacuna fechada leva data e
evidência. Este arquivo é o único lugar onde o ID da lacuna é definido — as páginas citam, não
redefinem.

**Nota de compatibilidade:** `wyd_portal` não recebe `USAGE` em `game` (`P-17`). Portanto “consulta direta” significa que a fonte lógica é `game.character`/`game.guild`, sem materializar cópia; a execução é pelo adapter do `wyd-db` sob o papel de serviço autorizado, não por conexão SQL do BFF com `game`. A forma exata desse contrato pertence ao planejamento do `wyd-db`/`wyd-admin`.

## 1. Resumo

| Estado | Quantas | Quais |
|---|---:|---|
| Aberta | 0 | — |
| Fechada | 6 | `LAC-POR-01` a `LAC-POR-06` |

## 2. Registro das lacunas

### LAC-POR-04 — Fonte e cache dos rankings (`fechada`)

- **Decisão do dono (2026-11-20):** consulta direta às tabelas `game`, limitada por `LIMIT`, com
  cache de **60 segundos no BFF**.
- **Consequência:** `game.character`/`game.guild` continuam fonte autoritativa; não se cria tabela nem
  view materializada, nem se depende de job. A consulta usa `LIMIT` e o BFF aplica cache com TTL de
  60 segundos. O endpoint expõe o instante de leitura/atualização do cache e o limite aplicado, sem
  chamar o resultado de “ao vivo” além dessa janela.
- **Compatibilidade com `P-17`:** `wyd_portal` continua sem `USAGE` em `game`. A decisão define a
  fonte lógica e a janela de cache, não concede permissão SQL ao portal; o acesso às tabelas de origem
  deve respeitar a fronteira aprovada e seu contrato autorizado de leitura.
- **Por que:** a consulta limitada evita tabela derivada e mantém o ranking alinhado com os dados de
  jogo; o TTL curto reduz consultas repetidas sem impor uma política de consistência entre instâncias
  que o dono não escolheu.
- **Dono:** arquitetura. **Evidência:** resposta do dono a `ask_questions`, 2026-11-20; refletida em
  `docs/arquitetura/consumo-do-banco.md` §4 e `docs/specs/requisitos.md` (`RF-POR-007`).
- **Reconfirmada em 2026-11-20, com a expansão do monitoramento:** o dono manteve a consulta limitada
  com cache de 60 s no BFF e recusou view materializada em `game` e tabela de cache em `portal`.
  Consequência: nenhuma tabela nem view nova pelo monitoramento; com o lote `0031`, a série está em
  31/97 e o catálogo de tabelas citadas, em 40 (39 com DDL) + 2 views.

### LAC-POR-05 — Segundo fator para staff (`fechada`)

- **Decisão do dono (2026-11-20):** confirmada a `A20` do `wyd-db`: segundo fator **TOTP**, em
  **credencial separada ligada a `portal.person`**, obrigatório para `role IN ('admin','moderator')`
  **antes** de o portal ir a público; `member` fica fora da `1.0.0`.
- **Consequência:** o login de staff passa a ter dois passos (senha + código TOTP); a sessão só nasce
  depois do segundo passo. O autenticador é da pessoa do portal, nunca de `game.account`, e não abre
  nem amplia acesso ao jogo (`P-17` intacta). Recuperação do segundo fator é fluxo operado por staff,
  nunca autoatendimento do titular (`A20`).
- **DDL fechado em 2026-11-20:** `A20` reservou nome, colunas conceituais e o `kind` inicial; o lote
  `0031` (`ddl-confirmado.md` §5.20) desenhou colunas, constraints, índices, permissão e revogação, e a
  página `migrations/0031-portal-mfa.md` está `concluída`. O requisito `RF-POR-013` pode ser implementado
  quando a série for aplicada; a escolha de mecanismo e de dono não foi reaberta.
- **Opções apresentadas:** (1) confirmar `A20` com autenticador separado em `portal` — **escolhida**;
  (2) reabrir `A20` e usar `game.account` — recusada.
- **Dono:** produto + segurança. **Evidência:** resposta do dono a `ask_questions`, 2026-11-20;
  `wyd-db/lacunas.md` `A20`; refletida em `docs/specs/requisitos.md` (`RF-POR-013`),
  `docs/arquitetura/rotas-e-permissoes.md` (§3/§6) e `docs/arquitetura/consumo-do-banco.md` (§3/§7).
- **Reconfirmada em 2026-11-20:** o dono manteve TOTP em credencial separada em `portal`, ligada a
  `portal.person`, e recusou segundo fator em `game.account` e o adiamento para a 1.1. O DDL de
  `portal.person_authenticator` está fechado no lote `0031` do `wyd-db` (`ddl-confirmado.md` §5.20,
  aprovado em 2026-11-20); a tabela entra na conta, que passa a 97.

### LAC-POR-06 — Telemetria de canal (CPU, RAM, latência) (`fechada`)

- **Decisão do dono (2026-11-20):** fonte **externa**. O `wyd-server` expõe `/metrics` no formato
  Prometheus; a pilha do `wyd-ops` (Prometheus + Grafana) coleta, retém e alerta; o portal **consulta
  a API HTTP do Prometheus** pelo BFF e **não** guarda série.
- **Consequência:** nenhuma tabela nova em `portal` nem em `ops` e nenhum DDL de telemetria; o
  catálogo de `docs/arquitetura/consumo-do-banco.md` §7 fica em **40 (39 com DDL) + 2 views**. O
  BFF chama `/api/v1/query` (e `query_range` para a janela do card) com timeout curto, sem credencial
  de banco, e cada card mostra o instante da amostra. Quando a consulta falha ou a métrica não
  existe, o card mostra **“não medido”** — nunca `0`, nunca valor estimado.
- **O que fica fora desta decisão:** intervalo de coleta, retenção, dashboards e alertas são do
  `wyd-ops`; a rede e a credencial entre o BFF e a API de métricas são contrato do `wyd-ops` ainda
  sem forma publicada. O portal não substitui o Grafana: o NOC resume e o Grafana investiga.
- **Paridade com o legado (`H`):** `deprecated/docs/arquitetura/adr/0018-telemetria-tracing.md` já
  declara `/metrics` Prometheus, métricas `wyd_*` e `WYD_METRICS_ADDR`, e `deprecated/docker/monitoring/`
  reúne Prometheus, Grafana, Loki, Alloy, Alertmanager e blackbox. A decisão **amplia** o legado: a
  mesma fonte passa a alimentar o NOC do portal, sem perder o que já existe.
- **Opções apresentadas:** (1) fonte externa — `/metrics` Prometheus + pilha do `wyd-ops`, portal por
  API — **escolhida**; (2) tabela de série em `ops` alimentada por coletor — recusada por duplicar no
  banco uma medição que o Prometheus já guarda, com retenção própria e risco de divergir; (3) BFF
  chamando o endpoint do servidor direto — recusada por não ter histórico nem a pilha que o legado já
  opera.
- **Por que:** uma medição, uma fonte e um dono; o portal ganha histórico e alerta sem criar schema e
  sem perder função em relação ao legado.
- **Dono:** `wyd-ops` + arquitetura. **Evidência:** resposta do dono a `ask_questions`, 2026-11-20;
  refletida em `docs/specs/requisitos.md` (`RF-POR-010`), `docs/arquitetura/consumo-do-banco.md`
  (§2/§8), `docs/arquitetura/rotas-e-permissoes.md` (§3/§6), `docs/arquitetura/fronteira.md` (§4/§8),
  `docs/specs/analise-do-prompt.md` e `docs/roadmap/roadmap.md` (F4).
- **Reconfirmada em 2026-11-20, com a expansão do monitoramento:** as seis telas do portal legado
  (resumo, servidor, Prometheus, painéis, logs e telemetria do launcher) usam Prometheus, Loki e
  Grafana do `wyd-ops`; o dono recusou tabela de eventos em `portal` e recusou reduzir o escopo.
  Contrato, paridade e degradação em `docs/arquitetura/monitoramento.md`; nenhuma tabela nova.

## 3. Lacunas fechadas

| ID | Decisão (2026-11-20) | Onde ficou registrada |
|---|---|---|
| `LAC-POR-01` | O portal **não** escreve estado de jogo: toda ação e escrita em `game` passa pelo `wyd-admin` por HTTP/JSON, com chave de idempotência; o BFF só lê pelo adapter do `wyd-db` e grava `portal`/`ops`. | `docs/arquitetura/fronteira.md` §3 (C4) |
| `LAC-POR-02` | CMS em `portal`, com **três** tabelas: `news`, `banner` e `guide`, com `slug`, estado de publicação e autor. | `docs/arquitetura/consumo-do-banco.md` §5 |
| `LAC-POR-03` | Versão, notas e **URL do manifesto assinado** em tabela do `portal`; o binário e a assinatura ficam no `wyd-ops`. | `docs/arquitetura/consumo-do-banco.md` §5 |
| `LAC-POR-04` | Rankings consultam `game` diretamente com `LIMIT` e cache de 60 s no BFF; nenhuma tabela/view materializada nova. | `docs/arquitetura/consumo-do-banco.md` §4 |
| `LAC-POR-05` | MFA de staff: TOTP em `portal.person_authenticator`, ligado a `portal.person`, obrigatório para `admin`/`moderator`; `member` fora da `1.0.0`. | `docs/specs/requisitos.md` (`RF-POR-013`) |
| `LAC-POR-06` | Telemetria de canal vem da fonte externa: `/metrics` Prometheus do `wyd-server`, coletado pela pilha do `wyd-ops`; o portal consulta a API do Prometheus e mostra “não medido” quando falha. Nenhuma tabela. | `docs/arquitetura/consumo-do-banco.md` §2 |

**Reconfirmação de 2026-11-20 (expansão do monitoramento):** as três decisões de arquitetura foram
levadas de novo ao dono, com alternativas, e mantidas — ranking com `LIMIT` e cache de 60 s, TOTP em
credencial separada de `portal.person` e telemetria por fonte externa. A expansão **não** mudou tabela,
rota pública nem permissão de banco: acrescentou seis rotas de staff (38–43) e cinco requisitos
(`RF-POR-014` a `RF-POR-018`), com o contrato em `docs/arquitetura/monitoramento.md`.

**Consequência declarada para o `wyd-db`:** as três de CMS mais a tabela de launcher são **4 tabelas
novas** em `portal`, que não existem nas 92 de `plano-migrations.md` §3. O lote
`0030_portal_content.sql` foi desenhado e aprovado em 2026-11-20 — texto em
`planejamento-1.0.0/wyd-db/migrations/0030-portal-content.md` e `ddl-confirmado.md` §5.15 —, e a série
passa a **31 arquivos / 97 tabelas**, com as cinco tabelas dos dois adendos entre as 77 de 97 com DDL fechado. As duas
contagens citadas aqui concordam com as do `wyd-db`.
`LAC-POR-06` **não** acrescenta tabela: a telemetria vem do `wyd-ops` e o `wyd-db` não ganha schema de
métrica. O monitoramento inteiro (métrica, log, painel e evento de launcher) segue a mesma regra; a
presença vem da view `game.v_live_session` de `0029`. Por isso o catálogo de tabelas citadas por este
planejamento fica em **40 (39 com DDL) + 2 views** (`docs/arquitetura/consumo-do-banco.md` §7), sem
contradizer as 97 tabelas da série.

## 4. O que **não** é lacuna

| Assunto | Por que não |
|---|---|
| papéis `member`/`moderator`/`admin` | decidido em `0003` |
| sessão revogável e token de cerimônia | decidido em `0004` (`A16`, `M4`) |
| configuração com revisão e auditoria | decidido em `0002` §5.9 (`M18-A`) |
| cash e loja no `portal`; jogo em `game` | decidido em `A19` |
| chave de idempotência | decidida em `M17` |
| caminho da escrita no jogo | decidido em 2026-11-20 (`LAC-POR-01`) |
| dono da escrita de `game.account_moderation` | decidido em 2026-11-21: leitura no portal, escrita no `wyd-admin` (C4 §4.1); `docs/arquitetura/consumo-do-banco.md` §3 e `RF-POR-011` corrigidos, sem mudança de DDL |
| CMS e launcher | decididos em 2026-11-20 (`LAC-POR-02`, `LAC-POR-03`) |
| monitoramento (seis telas) | decidido em `LAC-POR-06`, reconfirmada em 2026-11-20; fonte externa, sem tabela |

## 5. Como fechar

1. Dono responde a pergunta com a opção escolhida e a data.
2. A decisão vai para a página dona (`fronteira.md`, `consumo-do-banco.md` ou `rotas-e-permissoes.md`),
   não para este arquivo.
3. Aqui, a linha muda para `fechada` com data e link da evidência.
