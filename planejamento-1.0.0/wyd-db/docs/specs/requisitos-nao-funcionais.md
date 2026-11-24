> Pergunta: "com que qualidade o banco precisa operar, e o que ainda não tem limiar aprovado?"
> Público: Produto / gestão, QA e quem opera
> Dono: equipe do `wyd-db`
> Base: `AGENTS.md` §8; `docs/planos/plano-de-qualidade-e-gates.md` §1; `docs/guia-teste/gates-e-anti-skip.md`; `docs/guia-administrador/configuracao.md` §2–§3; `docs/guia-desenvolvedor/pool-e-transacoes.md` §1 e §3; `docs/arquitetura/retencao-e-capacidade.md`; `docs/arquitetura/seguranca.md`; `docs/roadmap/WYDDB-040.md`; `tests/specs/capacidades.md` §2–§6; `pendencias-de-fechamento.md` §6.6.7 (`Q-BLK-07` = A), §6.6.22.5 (`Q-BLK-19` = A) e §6.6.26 (`Q-BLK-24` = A)
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-24)

**Decisão:** `Q-BLK-07` = **A**, `Q-BLK-24` = **A** e `Q-BLK-19` = **A** (todas em 2026-11-24). A página
nasce com **limiar só onde há fonte** e **“a medir”** onde não há. A leva de `Q-BLK-07` são **três** `RNF-DB`
com número rastreável e **dois** com métrica declarada e sem limiar até `M20` executar; `Q-BLK-24`
acrescenta **dois** blocos (§6 e §7), porque concorrência de escrita e espera por conexão não eram
enunciadas por nenhum dos cinco anteriores; `Q-BLK-19` acrescenta **seis** (§8–§13), um por gate de série
que já governava capacidade na matriz. Nenhum número foi escolhido por estimativa: cinco já valem no
projeto, dois ficam **a medir**, um — o limiar de `RNF-DB-006` — entra **declarado sem página de fonte**
(§6), e os seis de §8–§13 são **limiares do gate que os mede** (0 violação ou 100 %), não número novo.

**A regra que fecha a página:** medir e publicar um número **sem** limiar aprovado **reprova**. Quem quiser
p95 ou tempo de boot com teto traz a medição para aprovação — não fixa o número na página.

## 1. `RNF-DB-001` — cobertura de teste

| Campo | Conteúdo |
|---|---|
| Métrica | percentual de cobertura por sistema, medida no gate |
| Limiar | **≥ 85 %** por sistema |
| Como medir | gate do sistema (comando ainda não existe: o crate tem `Cargo.toml` e o alvo `sistema_pg`, mas nenhuma medição de cobertura de linha) |
| Reprova | cobertura abaixo de 85 %, ou cobertura de linha apresentada como prova de capacidade |
| Fonte | `AGENTS.md` §8 |

Cobertura de linha **não** substitui capacidade provada (`AGENTS.md` §8): o limiar é piso, não critério de
pronto. Estado do número: `V` (regra vigente); estado da medição: `P`.

## 2. `RNF-DB-002` — espera máxima por DDL

| Campo | Conteúdo |
|---|---|
| Métrica | tempo de espera pelo advisory lock da série |
| Limiar | **10 s** (faixa aceita `1 s`–`10 min`) |
| Como medir | valor do `MigrateConfig` no runner e o teste de concorrência com dois processos |
| Reprova | valor fora da faixa (o construtor recusa) e espera acima de 10 s que **não** vire erro |
| Fonte | [`configuracao.md`](../guia-administrador/configuracao.md) §2; `src/migrate/config.md` |

O limite existe para o segundo processo **desistir**, não para esperar mais: passados 10 s, a migration
concorrente falha em vez de ficar pendurada. É o único limiar de tempo que o projeto fixa hoje.

## 3. `RNF-DB-003` — teste ignorado

| Campo | Conteúdo |
|---|---|
| Métrica | contagem de `SKIP`/`#[ignore]` novos no gate |
| Limiar | **0** |
| Como medir | regra do gate de teste, que reprova `SKIP` silencioso |
| Reprova | qualquer teste novo ignorado sem regra que o autorize |
| Fonte | [`gates-e-anti-skip.md`](../guia-teste/gates-e-anti-skip.md) |

O limiar é zero porque o defeito não é “pouco teste”: é teste que existe e **não roda**, dando a impressão
de cobertura. A matriz de capacidades (`gates-e-anti-skip.md`) é quem diz o que cada alvo precisa provar.

## 4. `RNF-DB-004` — latência de leitura de personagem (**a medir**)

| Campo | Conteúdo |
|---|---|
| Métrica | **p95** da consulta de leitura de personagem, na porta C4, em banco com volume de teste conhecido |
| Limiar | **não aprovado** — nenhum limiar de latência existe no projeto |
| Como medir | instrumentar a consulta e registrar a série em `planos/metricas.md`, quando ele existir (`M20`) |
| Reprova | **medir e publicar número como se fosse limiar**, ou trocar a métrica por média sem dizer |
| Fonte | [`retencao-e-capacidade.md`](../arquitetura/retencao-e-capacidade.md) (“Existe limiar de latência ou capacidade? **não**”) |

A métrica fica declarada para que a medição futura tenha forma: p95, não média; leitura de personagem, não
“o banco”. O número entra quando `M20` rodar — e entra por decisão, não por esta página.

## 5. `RNF-DB-005` — tempo de boot (**a medir**)

| Campo | Conteúdo |
|---|---|
| Métrica | tempo entre o início do processo e o pool aceitar a primeira conexão |
| Limiar | **não aprovado** |
| Como medir | instrumentar o boot e registrar em `planos/metricas.md` (`M20`) |
| Reprova | fixar prazo de boot sem medição; hoje [`WYDDB-040`](../roadmap/WYDDB-040.md) cita o boot no DoD **sem prazo** |
| Fonte | `WYDDB-040.md` (boot citado, sem número) |

**Por que não há número:** o único evento de boot que o planejamento menciona é “a série herdada não é lida
no boot”, que é **critério funcional**, não prazo. Tempo de boot depende de volume e de hardware que ninguém
mediu; inventar 10 s seria repetir o erro que a decisão A recusou.

## 6. `RNF-DB-006` — escrita concorrente sem deadlock

| Campo | Conteúdo |
|---|---|
| Métrica | ocorrência de `SQLSTATE` **`40P01`** (`deadlock detected`) em escritas concorrentes na ordem trocada |
| Limiar | **0** — qualquer `40P01` reprova |
| Como medir | cenário `concorrencia_4` do alvo `sistema_pg` (`tests/alvos/sistema-pg.md` §2), com **quatro** sessões; a falha é o erro do PostgreSQL, não o relógio |
| Reprova | deadlock detectado; e teto de tempo do harness apresentado como limiar deste requisito |
| Fonte | **sem página de fonte** — ver o parágrafo abaixo |

O limiar é **zero** pelo mesmo motivo de `RNF-DB-003`: o defeito não é lentidão, é a escrita abortar por
conflito de ordem. **Este bloco é o único da página sem página que o enuncie.** Varredura de 2026-11-24 na
árvore viva (`deadlock`, `40P01`, `SQLSTATE`) não achou página de `docs/` que trate deadlock como falha:
`padrao-banco.md` §13 é **“Fontes”** (bibliografia), e `padroes-postgres.md` §6 fixa três garantias de
concorrência que não incluem esta. O limiar se apoia **só** no SQLSTATE `40P01` e na decisão
`Q-BLK-24` = A; entra declarado assim, sem citação de empréstimo.

**O que este bloco não fixa:** número de sessões e tempo limite. As **quatro** sessões são parâmetro do
cenário (`CAP-17`), e o teto de tempo é do harness, que exige limite declarado sem número aprovado
(`docs/guia-teste/testes-de-sistema.md` §"Tempo limite"). Teto de tempo **não** é limiar deste `RNF`.

## 7. `RNF-DB-007` — espera por conexão finita e conexão reutilizada

| Campo | Conteúdo |
|---|---|
| Métrica | tempo de espera por conexão com o pool esgotado, e se a conexão devolvida volta a ser usada |
| Limiar | **30 s** (`acquire_timeout`); esgotado, **erro de aquisição** — nunca espera infinita |
| Como medir | esgotar um pool de **1** conexão com duas sessões e medir a espera; devolver e pedir de novo para observar o reuso (cenário `pool_espera_reuso`, `tests/alvos/sistema-pg.md` §2) |
| Reprova | espera sem teto; falha que não nomeia aquisição; conexão descartada a cada uso |
| Fonte | [`configuracao.md`](../guia-administrador/configuracao.md) §3; [`pool-e-transacoes.md`](../guia-desenvolvedor/pool-e-transacoes.md) §1 e §3 |

O **30 s** já valia: é o default do `sqlx` (`V`, §1 daquele guia) e o projeto o declarou em 2026-11-24
(`Q-BLK-06` = A). O que não existia era requisito que o enunciasse — `CAP-19` apontava para `RNF-DB-004`,
que mede p95 de leitura de personagem. Este bloco não inventa limiar: torna verificável um valor já
declarado.

**Não é threshold de pool.** O bloco fixa a espera por **uma** conexão, não ocupação, concorrência nem
disco — esses continuam fora de escopo por `WYDDB-026` (`pool-e-transacoes.md` §8). O orçamento de **53 de
97** conexões é de §2 daquele guia; aqui só se exige que a espera termine.

## 8. `RNF-DB-008` — a série é contígua e reversível (gate `estrutura`)

| Campo | Conteúdo |
|---|---|
| Métrica | conformidade estrutural da série: `0001`…`0034` sem furo nem repetição; `down` de cada arquivo desfazendo o seu `up`; nenhuma `REFERENCES` para arquivo posterior; nenhum `INSERT` de seed antes do `CREATE TABLE` da tabela |
| Limiar | **0** violação nas quatro condições; ao fim do `down` de todos, **0** tabela e **0** view nos quatro schemas |
| Como medir | gate `estrutura` ([`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) §1): `psql` no PG17 efêmero, aplicar, `down` de todos, contar `pg_tables`/views; e `contrato_migrations` (`serie_contigua`, `down_completo`, `fk_sem_futuro`, `seed_depois_da_tabela`, `apply_reapply_drift`) |
| Reprova | arquivo faltando, lacuna na numeração, tabela ou view sobrando ao fim do `down`, FK apontando para frente, seed antes da tabela |
| Fonte | gate `estrutura` de [`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) §1; os **34** arquivos têm dono em `plano-migrations.md` §3 |

Governa **6** capacidades: `CAP-01`–`CAP-05` e `CAP-09` (`tests/specs/capacidades.md` §2 e §3). O gate já existia e já reprovava; o que faltava era o requisito que a matriz pudesse citar em vez
do nome solto do gate (`Q-BLK-19` = A).

## 9. `RNF-DB-009` — toda constraint é nomeada e violável em teste (gate `constraints`)

| Campo | Conteúdo |
|---|---|
| Métrica | proporção de `PRIMARY KEY`, `UNIQUE`, `CHECK` e `FOREIGN KEY` com `CONSTRAINT nome`, e de constraints com um teste que as viole recebendo o erro esperado |
| Limiar | **100 %** nomeadas e **0** constraint sem teste que a viole |
| Como medir | gate `constraints` ([`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) §1): `contrato_migrations` para o nome e `sistema_pg` para a violação com o SQLSTATE esperado |
| Reprova | constraint anônima, ou invariante sem teste que a recuse |
| Fonte | gate `constraints` de [`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) §1; a regra “constraint sem teste é comentário” é de `padrao-banco.md` §10 |

Governa `CAP-06`. O limiar é **100 %/0** porque o gate não aceita parcial: constraint nomeada sem teste
continua sendo invariante não provada.

## 10. `RNF-DB-010` — nenhum tipo proibido na série (gate `tipos`)

| Campo | Conteúdo |
|---|---|
| Métrica | ocorrências de `serial`, `timestamp` sem fuso, `time`, `money`, `varchar` e `enum` em coluna da série |
| Limiar | **0** ocorrência de qualquer um dos seis |
| Como medir | gate `tipos` ([`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) §1): conferência manual hoje; o gate automático é do PR de ferramentas, ainda em espera |
| Reprova | qualquer tipo proibido em coluna, inclusive em tabela nova |
| Fonte | gate `tipos` de [`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) §1; a lista de tipos é a mesma de `padrao-banco.md` §4 |

Governa `CAP-07`. A conferência manual é **falha declarada** enquanto o gate automático não existe —
não é “verde por ausência” ([`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) §6).

## 11. `RNF-DB-011` — catálogo e série descrevem o mesmo schema (gate `catálogo`)

| Campo | Conteúdo |
|---|---|
| Métrica | tabelas e views do documento gerado contra as da série, com schema, nota e fonte |
| Limiar | **109** tabelas e **2** views idênticas entre documento e série; **0** divergência |
| Como medir | gate `catálogo` ([`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) §1): `node tools/content/check-migrations.mjs` (previsto) e `contrato_catalogo` |
| Reprova | tabela ausente, tabela sem schema, tabela sem nota ou contagem divergente |
| Fonte | gate `catálogo` de [`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) §1; as **109** tabelas e **2** views têm dono em `plano-migrations.md` §3; a página do catálogo é `Q-BLK-20` |

Governa `CAP-08`, que hoje **não tem ambiente**: o documento `dicionario-dados/banco-catalogo.md` não
existe e o leitor atual não entende `CREATE TABLE esquema.tabela` (`tests/alvos/contrato-catalogo.md` §3).
O requisito existe e é verificável no dia em que o gate rodar; o que não existe é o insumo. Em 2026-11-24,
`Q-BLK-20` = **A** declarou `CAP-08` **não executável na `1.0.0`**; o requisito segue válido para depois da
release, quando o dicionário e o leitor existirem.

## 12. `RNF-DB-012` — a fronteira entre papéis é provada (gate `fronteira`)

| Campo | Conteúdo |
|---|---|
| Métrica | escrita de um papel em schema de outro, e leitura cruzada fora de view |
| Limiar | **0** escrita cruzada aceita (todo `INSERT`/`UPDATE`/`DELETE` de `wyd_portal` em `game` devolve `42501`) e **0** leitura crua de `portal.person` pelo papel de jogo |
| Como medir | gate `fronteira` ([`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) §1): `sistema_sessoes` e `psql` com a DSN de cada papel |
| Reprova | a escrita passa, ou a leitura crua funciona |
| Fonte | gate `fronteira` de [`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) §1; o mapa de privilégio por papel é de [`seguranca.md`](../arquitetura/seguranca.md) |

Governa `CAP-20`; `CAP-21` já tem requisito próprio (`RF-DB-011`) e usa o mesmo gate como prova adicional.

## 13. `RNF-DB-013` — o ambiente é nomeado, não adivinhado (gate `ambiente`)

| Campo | Conteúdo |
|---|---|
| Métrica | comportamento com a DSN ausente e leitura de `DATABASE_URL` pelo binário |
| Limiar | **0** default silencioso e **0** leitura de `DATABASE_URL`; toda execução sem a variável falha **nomeando-a** |
| Como medir | gate `ambiente` ([`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) §1): executar sem a variável e conferir a mensagem; busca por `DATABASE_URL` no código |
| Reprova | erro genérico, default silencioso ou leitura de `DATABASE_URL` |
| Fonte | gate `ambiente` de [`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) §1; os nomes das variáveis têm dono em `src/provision/spec.md` §3 |

Governa `CAP-22`. O limiar não fixa o texto da mensagem: exige que ela **nomeie** a variável faltante.

## 14. Cobertura por dimensão

A tabela diz **onde cada dimensão mora** hoje, para a página de RNF não virar segunda boca de regra que já
tem dono.

| Dimensão | O que existe | Dono |
|---|---|---|
| desempenho | `RNF-DB-004` e `RNF-DB-005`, ambos **a medir** | esta página; número em `planos/metricas.md` |
| concorrência | `RNF-DB-006`: escrita concorrente sem `40P01` | esta página; cenário em `tests/alvos/sistema-pg.md` |
| saturação do pool | `RNF-DB-007`: espera finita de `30 s` e reuso | esta página; número de conexões em `pool-e-transacoes.md` §2 |
| integridade | `CHECK`, FK, índice único e `RESTRICT` já desenhados | [`integridade-e-constraints.md`](../arquitetura/integridade-e-constraints.md) |
| disponibilidade | RPO 24 h e RTO 4 h | [`recuperacao-de-desastre.md`](../guia-administrador/recuperacao-de-desastre.md) §1 |
| segurança | papel por serviço, segredo por tipo e lugar, `42501` provado | [`seguranca.md`](../arquitetura/seguranca.md) |
| capacidade | teto de **97** conexões e orçamento de **53** | [`pool-e-transacoes.md`](../guia-desenvolvedor/pool-e-transacoes.md) §2 |
| retenção | prazos por categoria, já decididos | [`retencao-e-capacidade.md`](../arquitetura/retencao-e-capacidade.md) |
| gate de série | `RNF-DB-008` a `RNF-DB-013`: forma, constraints, tipos, catálogo, fronteira e ambiente | esta página; prova em [`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) §1 |

Nenhuma dessas linhas cria requisito que não esteja enunciado acima: cada uma aponta o dono. O que esta página acrescenta é o
**limiar com número** e a **pendência declarada** — nada além.

## 15. O que esta página não decide

- **p95 e boot com número.** Dependem de `M20` executado; o dono do número é `planos/metricas.md`.
- **Número de sessões concorrentes e teto de tempo do teste.** São parâmetro de cenário e de harness (§6).
- **RNF de outros sistemas.** `RNF-POR`, `RNF-ADM` e equivalentes são dos sistemas donos; esta página não os enuncia.
- **Critério de aceite por página.** É de `M33` e da revisão de quem não escreveu; aqui só entram os limiares de qualidade do banco.
- **Retenção e capacidade.** Prazos e teto já têm dono (§8); esta página os cita, não os redefine.
- **Desempenho medido do servidor de jogo.** É do `wyd-server`; o `wyd-db` responde pela consulta, não pela latência de rede.
