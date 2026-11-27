> Pergunta: "por onde começo a ler a documentação do `wyd-db`?"
> Público: todos os públicos de §2 do plano diretor — é a porta de entrada
> Dono: equipe do `wyd-db`
> Base: —
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25)

# wyd-db — documentação oficial

**A persistência do WYD em PostgreSQL 17 e Rust.** Schema, migrations, portas de domínio, adapter,
pool, filas de transferência e carga de conteúdo — documentados para quem vai entender, alterar ou
operar o banco.

[Começar pelo guia do desenvolvedor](guia-desenvolvedor/index.md)
[Ver os requisitos](specs/index.md)

> [!TIP] ⏱️ Trilha de 10 minutos
> 1. [🏛️ Arquitetura](arquitetura/index.md) — o que o `wyd-db` é e onde ficam as fronteiras.
> 2. [🗂️ Dicionário de dados](dicionario-dados/index.md) — o que cada tabela significa.
> 3. [👩‍💻 Guia do desenvolvedor](guia-desenvolvedor/index.md) — como mexer sem quebrar.

## 🧭 Seções

<div class="cartoes"></div>

-   **🏛️ Arquitetura**

---

Visão geral, contexto C4, componentes, fronteira entre o tick e o banco, modelo conceitual e lógico,
padrões de PostgreSQL e decisões registradas.

[Abrir arquitetura →](arquitetura/index.md)

-   **📋 Especificações e requisitos**

---

Requisitos funcionais, regras de negócio, requisitos não funcionais, casos de uso, fluxos,
estados e a matriz de rastreabilidade.

[Abrir especificações →](specs/index.md)

-   **🗂️ Dicionário de dados**

---

O significado de cada tabela por domínio e a referência de campo gerada a partir das
migrations.

[Abrir dicionário →](dicionario-dados/index.md)

-   **🗺️ Planos**

---

Plano de documentação, testes, implantação, manutenção, qualidade, dados e seeds, métricas, riscos e
registro de execução.

[Abrir planos →](planos/index.md)

-   **🚀 Roadmap**

---

Fases, prioridades e a página de cada issue `WYDDB-nnn` com o seu critério de pronto.

[Abrir roadmap →](roadmap/index.md)

-   **👩‍💻 Guia do desenvolvedor**

---

Ambiente, mapa do crate, portas e contratos, como adicionar migration, como escrever teste e o
checklist de PR.

[Abrir guia do desenvolvedor →](guia-desenvolvedor/index.md)

-   **🛠️ Guia do administrador**

---

Instalação, configuração, backup e restore, diagnóstico, manutenção e recuperação de desastre.

[Abrir guia do administrador →](guia-administrador/index.md)

-   **🧪 Guia de teste**

---

Pirâmide de testes, fixtures, dados de teste, contrato, sistema e os gates que impedem teste
que não roda.

[Abrir guia de teste →](guia-teste/index.md)

-   **✍️ Guia de documentação**

---

Os padrões desta documentação: onde cada assunto mora, como escrever, que recursos de markdown
usar, como diagramar, linkar e publicar.

[Abrir guia de documentação →](guia-documentacao/index.md)


## 🎯 Por onde começar

| Público | Comece por | Depois |
|---|---|---|
| 👩‍💻 **Desenvolvedor novo no crate** | [Guia do desenvolvedor](guia-desenvolvedor/index.md) — `mapa-do-crate.md` | [Arquitetura](arquitetura/index.md) |
| 🧩 **Desenvolvedor de outro crate** | [Guia do desenvolvedor](guia-desenvolvedor/index.md) — `portas-e-contratos.md` | [Arquitetura](arquitetura/index.md) |
| 🏛️ **Arquiteto ou revisor de schema** | [Arquitetura](arquitetura/index.md) | [Decisões (ADR-DB)](arquitetura/adr/index.md) |
| 🛠️ **DBA e operação** | [Guia do administrador](guia-administrador/index.md) | [Arquitetura](arquitetura/index.md) |
| 🧪 **QA e testador** | [Guia de teste](guia-teste/index.md) | [Especificações](specs/index.md) |
| 📋 **Produto e gestão** | [Especificações](specs/index.md) | [Roadmap](roadmap/index.md) |

## 🛤️ Trilhas de leitura

#### 🧠 Entender o banco


1. [Arquitetura](arquitetura/index.md) — `visao-geral.md`, `modelo-conceitual.md`.
2. [Dicionário de dados](dicionario-dados/index.md) — começa em `como-ler.md`.
3. [Especificações](specs/index.md) — as regras que o banco precisa cumprir.

#### 🔧 Alterar o schema


1. [Arquitetura](arquitetura/index.md) — `padroes-postgres.md` e `integridade-e-constraints.md`.
2. [Guia do desenvolvedor](guia-desenvolvedor/index.md) — `como-adicionar-migration.md`.
3. [Guia de teste](guia-teste/index.md) — o que precisa ser provado antes do merge.

#### 🚦 Operar em produção


1. [Guia do administrador](guia-administrador/index.md) — `backup-e-restore.md`, `diagnostico.md`.
2. [Planos](planos/index.md) — `plano-de-implantacao.md`.
## ⚠️ Antes de alterar o schema

> [!WARNING] Três regras que evitam retrabalho
> - **Coluna nasce com a tabela.** Não existe `ALTER TABLE` corretivo: desenho errado se conserta
>   na origem, não com remendo.
> - **DDL antes de seed.** Nenhum seed roda antes de a tabela que ele preenche existir, e o
>   `down` de seed restaura o estado anterior em vez de apagar.
> - **Todo objeto criado cai no `down` do próprio arquivo.** Reverter precisa voltar ao estado
>   exato de antes.
>
> O passo a passo está em [Guia do desenvolvedor](guia-desenvolvedor/index.md); o porquê de cada
> regra está em [Arquitetura](arquitetura/index.md).

## 🧾 Decisões em aberto

As perguntas da rodada `M33` — estado de página e aceite da onda A — estão em
[Decisões da M33](proposta-decisoes-m33.md). O estado da lacuna é de `lacunas.md` §2.3.

As decisões de retenção que a `M32` deixou pendentes estão em
[Retenção — decisões e prazos](proposta-retencao-pendente.md), com a medição do que o repositório já permite saber.

A página [Retenção e capacidade](arquitetura/retencao-e-capacidade.md) existe como `proposto`, com
volume pendente de medição, a matriz de `Q-CAP-2` aplicada e o sinal de `Q-CAP-3` registrado sem
limiar; o histórico das decisões está em [Capacidade — decisões](proposta-decisoes-capacidade.md).

O artefato que deve encerrar a validação jurídica de `Q-DP-2` foi escolhido em **2026-11-24** —
**A: parecer jurídico escrito, por finalidade** — e está registrado em
`pendencias-de-fechamento.md` §3.3.1. O parecer ainda não existe e `Q-DP-2/validação` continua pendente;
`Q-RET-3` foi respondida em **2026-11-24** (A): retenção por ciclo de vida da pessoa, sem expurgo por
tempo. A decisão não transforma a dispensa de parecer em validação jurídica. `Q-RET-7` foi respondida
em **2026-11-24** (A): cópia diária de 30 dias, rotação, criptografia fora do host e restore trimestral;
a rotina, o fornecedor, o RPO/RTO e o ensaio continuam pendentes. `Q-RET-6` foi respondida em
**2026-11-24** (A): financeiro/custódia sem expurgo por tempo e `market_notification` lida por 90 dias;
a aplicação (índice, job e preservação legal) continua pendente. **Não resta decisão de retenção aberta:**
são 7 resolvidas.

O quadro completo de fechamento — 0 decisões de retenção abertas, 6 seeds bloqueados, 1 ato de aceite e a 1
página de `docs/` bloqueada (`planos/metricas.md`) — está em `pendencias-de-fechamento.md`, na raiz de
`planejamento-1.0.0/wyd-db/`. Ele nomeia dono, custo estimado e o que cada item desbloqueia, sem
criar lacuna nova.

O **foco do projeto em 2026-11-24** era a fila das páginas bloqueadas (`Q-FOCO-1 = D`): com o **lote F**, as
quatro que dependiam de decisão foram aprovadas em A e escritas — `rastreabilidade`, `stakeholders`,
`implantacao` e `modelo-conceitual` (`Q-BLK-08` a `Q-BLK-11`) —, e a fila está em **1**: `planos/metricas.md`,
que espera a série aplicada. Ficaram escritos e `proposto` os lotes de entrada na cena, de economia e
custódia, de operação e documentação, de teste e de conexão, cada um com as dependências externas
declaradas na própria página. A1 continua autorizada, mas fora do foco imediato. O registro de execução
está em `pendencias-de-fechamento.md` §6.5 e §6.6.19. Em §6.6 do mesmo arquivo está o pacote de decisões:
`Q-BLK-01` a `Q-BLK-11` foram **aprovadas em 2026-11-24** (o teto autoral foi de 147 para **144**), e
nenhuma pergunta daquele quadro fica aberta. O **lote G** (§6.6.20) escolheu a frente de execução: issues +
suíte de testes — **52** artefatos que não fecham a matriz célula a célula: as **12** páginas de `tests/` fecham
o critério 6 do plano de testes (a matriz `CAP-nn` escrita, com a lacuna de cobertura declarada) e deram
cenário a **15** das 40 linhas, na coluna `Página` — **31** desde `Q-BLK-23` = A, §6.6.29; as **40** issues são
a fila do roadmap, e a coluna `Issue` registra a
onda de cada linha, não um número de issue; as **24** células `Teste` seguem em `P` e **13** estão em `V`
desde 2026-11-24 —, com
as **5** perguntas (`Q-BLK-12` a `Q-BLK-16`) respondidas em **A**. O **lote H** (§6.6.21) executou a primeira
metade: as **12** páginas de `tests/` estão escritas, com a matriz `CAP-nn` de **23** capacidades e **0**
cobertas, e as **40** issues seguem sem artefato no GitHub. O **lote I** (§6.6.22) mediu tudo de novo e
recomenda **aceitar as 12 páginas de `tests/` primeiro** — **12** atos que deixam **5** alvos prontos e,
medido depois do lote I, alcançam **15 das 40** linhas da rastreabilidade —, com o aceite ainda pendente naquele momento (hoje **141**, §6.6.31) e **4** perguntas novas (`Q-BLK-17` a `Q-BLK-20`), todas respondidas em 2026-11-24 —
a `17` pela própria execução (a leva 1 começou por `tests/`) e `18`, `19` e `20` em **A**. Com
`Q-BLK-18` = **A**, a leva 2 foi executada na mesma data: as **37** páginas de `src/` foram revisadas e
estão `aceito (2026-11-24)` (`pendencias` §6.6.27), o que deixou **168** atos de aceite naquele momento e o crate
livre para o primeiro `Cargo.toml`/`.rs` sob o invariante §5.1. Com `Q-BLK-20` e `Q-BLK-22` em **A** e
a leva `migrations/` assinada, as **27** páginas passaram a `aceito (2026-11-24)` e o aceite pendente caiu
para **141**, todos em `docs/` (`pendencias` §6.6.31). Com `Q-BLK-19` = **A**, a mesma data
acrescentou seis requisitos de gate (`RNF-DB-008`–`013`), repontou as **11** capacidades e levou a matriz de
rastreabilidade a **40** linhas (`pendencias` §6.6.28).
O censo de **2026-11-24** (`docs/specs/rastreabilidade.md` §3.4) mediu **25** linhas sem página e **16** sem
cenário — **13** de requisito e **3** `RNF` —, o que abriu `Q-BLK-23` (`pendencias-de-fechamento.md` §6.6.24).
Com `Q-BLK-23` = **A**, o dono mandou escrever os **16** cenários antes da leva de `migrations/`: são as
**16** `CAP-24` a `CAP-39` de `tests/specs/capacidades.md`, em `tests/alvos/sistema-pg.md` (§6.6.29), e o
censo vigente é **9** linhas sem página e **0** sem cenário. `Q-BLK-24` foi aprovada em **A** na mesma data e deu requisito próprio às três capacidades que
apontavam `RNF` alheio: `RNF-DB-006`, `RNF-DB-007` e `RF-DB-017` (§6.6.26 daquele arquivo). A folha de
revisão de `Q-BLK-21` está na raiz do sistema (`revisao-de-q-blk-21.md`, arquivo não publicado): o dono
aprovou **A**, as correções de redação foram aplicadas e as **12** páginas de `tests/` estão `aceito
(2026-11-24)`. O aceite não cobre capacidade por si — mas em 2026-11-24 a leva de `migrations/` aplicou os
**28** `.sql` do zero, 13 cenários de `sistema_pg` e 7 casos de `runner_pg` rodaram verdes: a matriz está **14 de 39** — e o código
dos 6 alvos já tem permissão: `Q-BLK-18` = A.

**A medição do foco do sistema é `foco-e-sequencia.md`**, na raiz deste sistema — arquivo de raiz não é
publicado no site, e por isso é citado aqui como código: ele mantém as seis frentes, a cadeia de
dependência e o que cada elo destrava. O **ranking publicado** das frentes do projeto, que decide qual vem
agora, é `wyd-ops/docs/prioridade-do-projeto.md` §0 e §4. Esta página resume o histórico dos lotes e não é
dona da ordem; onde as duas divergirem, vale a página de prioridade para a ordem e `foco-e-sequencia.md`
para a medição do sistema.

## 📦 Escopo desta edição

> [!NOTE] O que está documentado e o que fica fora
> **Dentro:** o schema da série 1.0.0 (`0001_bootstrap.sql` … `0029_views.sql` — contagem de
> arquivos, tabelas e `SCHEMA_VERSION` em `plano-migrations.md` §3), o crate `wyd-db`
> (portas, adapter, pool, migrations, filas), o seed de desenvolvimento, a operação do banco e os
> requisitos que ele precisa cumprir.
>
> **Fora:** `wyd-core`, `wyd-server`, `wyd-net`, `wyd-admin` e o portal; regra de jogo
> implementada no tick; dados de jogador reais; harness E2E de cliente e bots.

## 📐 Convenções de leitura

> [!ABSTRACT] Como interpretar o texto
> - `V` — **verificado** por comando, arquivo ou artefato.
> - `H` — **hipótese** declarada, com o que falta para fechar.
> - `P` — **planejado**, ainda não existe.
> - **sem marca** — comportamento do PostgreSQL: plataforma, não projeto.
>
> Comportamento sem `arquivo:linha` nunca é apresentado como prova, e todo número cita a fonte e a
> data da medição.
>
> A forma do schema (coluna, tipo, índice, CHECK) vem da referência **gerada**; o texto explica o
> significado em vez de repetir a coluna.
>
> As regras de escrita, marcação e publicação estão no
> [✍️ guia de documentação](guia-documentacao/index.md).

## Mapa de páginas

Este mapa garante que toda página de `docs/` seja alcançável a partir deste índice; a pasta dona continua sendo a fonte do conteúdo.

### raiz

- [Módulos do `wyd-db` — detalhamento](modulos.md)

### roadmap

- [WYDDB-001 — fixar lint e documentação pública do crate](roadmap/WYDDB-001.md)
- [WYDDB-002 — congelar contrato público de migrate](roadmap/WYDDB-002.md)
- [WYDDB-003 — congelar contrato público de provision](roadmap/WYDDB-003.md)
- [WYDDB-004 — documentar invariantes dos modelos de identidade](roadmap/WYDDB-004.md)
- [WYDDB-005 — implementar Dsn com Debug redigido](roadmap/WYDDB-005.md)
- [WYDDB-006 — fechar Command e Report do runner](roadmap/WYDDB-006.md)
- [WYDDB-007 — fechar Command e Report do provisionamento](roadmap/WYDDB-007.md)
- [WYDDB-008 — mapear MigrateError para códigos de saída](roadmap/WYDDB-008.md)
- [WYDDB-009 — mapear ProvisionError para códigos de saída](roadmap/WYDDB-009.md)
- [WYDDB-010 — provar cadeia de erro sem segredo](roadmap/WYDDB-010.md)
- [WYDDB-011 — implementar Source e FsSource](roadmap/WYDDB-011.md)
- [WYDDB-012 — implementar Ledger e MigrationExecutor](roadmap/WYDDB-012.md)
- [WYDDB-013 — implementar AdvisoryLock e timeout](roadmap/WYDDB-013.md)
- [WYDDB-014 — ordenar preflight, lock e apply no runner](roadmap/WYDDB-014.md)
- [WYDDB-015 — separar Catalog e Script no provisionamento](roadmap/WYDDB-015.md)
- [WYDDB-016 — concentrar SQL de repositório em repo/pg.rs](roadmap/WYDDB-016.md)
- [WYDDB-017 — implementar save em lote do personagem](roadmap/WYDDB-017.md)
- [WYDDB-018 — drenar ops.delivery_queue com idempotência](roadmap/WYDDB-018.md)
- [WYDDB-019 — aplicar grants de portal sem duplicar](roadmap/WYDDB-019.md)
- [WYDDB-020 — registrar crédito e débito idempotentes](roadmap/WYDDB-020.md)
- [WYDDB-022 — instrumentar spans e resumo do runner](roadmap/WYDDB-022.md)
- [WYDDB-023 — emitir relatório do provisionamento sem segredo](roadmap/WYDDB-023.md)
- [WYDDB-024 — correlacionar erro por versão e arquivo](roadmap/WYDDB-024.md)
- [WYDDB-025 — expor sinais de lock e drift](roadmap/WYDDB-025.md)
- [WYDDB-026 — documentar diagnóstico de lock e ledger](roadmap/WYDDB-026.md)
- [WYDDB-027 — testar tipos puros de migration](roadmap/WYDDB-027.md)
- [WYDDB-028 — testar contrato da série](roadmap/WYDDB-028.md)
- [WYDDB-029 — testar contrato catálogo ↔ série](roadmap/WYDDB-029.md)
- [WYDDB-030 — testar sistema PostgreSQL e ciclo](roadmap/WYDDB-030.md)
- [WYDDB-031 — testar fronteira de papéis e views](roadmap/WYDDB-031.md)
- [WYDDB-032 — testar stack de capacidades](roadmap/WYDDB-032.md)
- [WYDDB-033 — criar fixtures e factories isoladas](roadmap/WYDDB-033.md)
- [WYDDB-034 — fechar gate anti-skip e matriz CAP](roadmap/WYDDB-034.md)
- [WYDDB-035 — escrever 0001_bootstrap.sql](roadmap/WYDDB-035.md)
- [WYDDB-036 — escrever 0002_ops_audit.sql](roadmap/WYDDB-036.md)
- [WYDDB-037 — escrever 0003 a 0007 do corte vertical](roadmap/WYDDB-037.md)
- [WYDDB-038 — escrever 0008 a 0012 do estado de jogo](roadmap/WYDDB-038.md)
- [WYDDB-039 — escrever 0013 a 0029 de conteúdo e views](roadmap/WYDDB-039.md)

### specs

- [visao-e-escolpo](specs/visao-e-escolpo.md)

### specs/estados

- [grant](specs/estados/grant.md)
- [item-e-refino](specs/estados/item-e-refino.md)
- [personagem](specs/estados/personagem.md)
- [sessao](specs/estados/sessao.md)

### specs/fluxos

- [carga-de-conteudo](specs/fluxos/carga-de-conteudo.md)
- [carregamento-e-save](specs/fluxos/carregamento-e-save.md)
- [compra-e-credito](specs/fluxos/compra-e-credito.md)
- [concessao-de-grant](specs/fluxos/concessao-de-grant.md)
- [criacao-e-selecao-de-personagem](specs/fluxos/criacao-e-selecao-de-personagem.md)
