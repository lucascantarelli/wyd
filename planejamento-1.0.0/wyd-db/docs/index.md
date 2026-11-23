> Pergunta: "por onde começo a ler a documentação do `wyd-db`?"
> Público: todos os públicos de §2 do plano diretor — é a porta de entrada
> Dono: equipe do `wyd-db`
> Base: —
> Revisado em: 2026-11-22

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

Os prazos de retenção que a `M32` deixou pendentes estão em
[Retenção — prazos pendentes](proposta-retencao-pendente.md), com a medição do que o repositório já permite saber.

O que a página de retenção e capacidade pode afirmar sem volume medido — e quem responde pelo
alerta — está em [Capacidade — decisões](proposta-decisoes-capacidade.md).

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