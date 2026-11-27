"""Fixtures mínimas, centralizadas, para cada regra do gate.

Cada dicionário é `caminho relativo -> conteúdo`. Os testes criam a árvore em um
diretório temporário; nenhuma fixture depende do repositório real.
"""

LACUNA_OK = {
    "planejamento-1.0.0/wyd-teste/lacunas.md": """# Lacunas de teste

## 2. Lacunas

| ID | Pergunta | Evidência |
|---|---|---|
| `LT-01` | o item foi decidido? | **fechada** (2026-01-01: decidido); evidência em `planejamento-1.0.0/wyd-teste/evidencia.md` |
""",
    "planejamento-1.0.0/wyd-teste/evidencia.md": """# Evidência

`LT-01` decidido: o artefato existe e trata do item.
""",
}

LACUNA_SEM_EVIDENCIA = {
    "planejamento-1.0.0/wyd-teste/lacunas.md": LACUNA_OK[
        "planejamento-1.0.0/wyd-teste/lacunas.md"
    ],
}

SKILLS_OK = {
    "planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md": """# Template

## 3. Skills — lista fechada

| Skill | Onde existe |
|---|---|
| `planejar-sistema` | `.agents/skills/` |
| `auditar-planejamento` | `.agents/skills/` |

## 4. Regras

Uma issue, uma página.
""",
    ".github/ISSUE_TEMPLATE/tarefa.yml": """name: Tarefa
body:
  - type: dropdown
    id: skill_principal
    attributes:
      options:
        - auditar-planejamento
        - planejar-sistema
    validations:
      required: true
""",
    "planejamento-1.0.0/wyd-teste/docs/roadmap/roadmap.md": """# Roadmap

| Página |
|---|
| [`WYDX-F1`](WYDX-F1.md) §Skills |
""",
    "planejamento-1.0.0/wyd-teste/docs/roadmap/WYDX-F1.md": """# WYDX-F1 — fatia de teste

> Pergunta: "a fatia de teste entrega o contrato mínimo?"
> Público: Desenvolvimento e agentes
> Dono: `wyd-teste`
> Base: fixture do gate
> Revisado em: 2026-01-01

**Estado:** proposto
**Fase:** F1
**Gate de verificação:** `python -m gate fast --root ..`

## Contexto

Fixture mínima para a regra de skills.

## Entregável

Uma página de issue no padrão único.

## DoD

- [ ] o gate `skills` passa.

## Fora de escopo

Implementação de produção.

## Depende de

Nada.

## Skills

| campo | valor |
|---|---|
| `skill_principal` | `planejar-sistema` |

**Dono do valor:** equipe de teste

## Evidência ao fechar

Saída verde do gate `skills`.
""",
}

SKILLS_REPETE_FASE = {
    **SKILLS_OK,
    "planejamento-1.0.0/wyd-teste/docs/roadmap/roadmap.md": """# Roadmap

| Página |
|---|
| [`WYDX-F1`](WYDX-F1.md) §Skills |

| Fase | F1 |
""",
}

# O formulário oferece uma skill fora da §3 e omite a outra: as duas direções reprovam.
SKILLS_FORMULARIO_DIVERGENTE = {
    **SKILLS_OK,
    ".github/ISSUE_TEMPLATE/tarefa.yml": """name: Tarefa
body:
  - type: dropdown
    id: skill_principal
    attributes:
      options:
        - planejar-sistema
        - skill-inventada
    validations:
      required: true
""",
}

CONTAGEM_OK = {
    "planejamento-1.0.0/wyd-db/migrations-checklist.md": """# Checklist

## 2. Arquivos

| Nº | Página | Tab. |
|---|---|---|
| 0001 | `0001_bootstrap.md` | 2 |
""",
    "planejamento-1.0.0/wyd-db/ddl-confirmado.md": """# DDL

CREATE TABLE exemplo (id integer);
""",
    "planejamento-1.0.0/wyd-db/plano-migrations.md": """# Plano

0001_bootstrap.sql
""",
    "planejamento-1.0.0/wyd-db/docs/guia.md": """# Guia

**Estado:** aceito
""",
    "wyd-docs/docs/index.md": """# Portal
""",
}

CENSO_OK = {
    "planejamento-1.0.0/wyd-db/docs/a.md": """# A

**Estado:** proposto
""",
    "planejamento-1.0.0/wyd-db/src/b.md": """# B

**Estado:** aceito
""",
    "planejamento-1.0.0/wyd-db/migrations/0001_bootstrap.md": """# Migration

**Estado:** proposto
""",
    "planejamento-1.0.0/wyd-db/tests/t.md": """# Teste

**Estado:** aceito
""",
    "planejamento-1.0.0/wyd-db/docs/specs/rastreabilidade.md": """# Rastreabilidade

## 2. Requisitos

| Requisito | História | Ator | Estado | Teste | Issue | Entrega | Nota |
|---|---|---|---|---|---|---|---|
| `RF-DB-001` | login | ator | ok | `P` | — | onda 1 | nota |
""",
    "planejamento-1.0.0/wyd-db/pendencias-de-fechamento.md": """# Pendências

| ID | Situação |
|---|---|
| `Q-BLK-01` | aberta |
""",
}

CENSO_ATOS = {
    **CENSO_OK,
    "planejamento-1.0.0/wyd-db/pendencias-de-fechamento.md": """# Pendências

| ID | Situação |
|---|---|
| `Q-BLK-01` | aberta |

Medido em 2026-11-27.
""",
}

CENSO_ATOS_VELHO = {
    **CENSO_ATOS,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

O sistema tem **3** atos pendentes.
""",
}

CENSO_ATOS_CORRETO = {
    **CENSO_ATOS,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

O sistema tem **2** atos pendentes.
""",
}

CENSO_ATOS_VELHO_ABAIXO_METADE = {
    **CENSO_ATOS,
    "planejamento-1.0.0/wyd-db/docs/c.md": """# C

**Estado:** proposto
""",
    "planejamento-1.0.0/wyd-db/docs/d.md": """# D

**Estado:** proposto
""",
    "planejamento-1.0.0/wyd-db/docs/e.md": """# E

**Estado:** proposto
""",
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

O sistema tem **2** atos pendentes.
""",
}

CENSO_ATOS_LEVA_12 = {
    **CENSO_ATOS,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

A leva de `docs/` tem **12** atos pendentes.
""",
}

CENSO_ATOS_LEVA_DISTANTE = {
    **CENSO_ATOS,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

A leva de `docs/` foi fechada e o sistema tem **12** atos pendentes.
""",
}

CENSO_ATOS_TOTAL_DOZE = {
    **CENSO_ATOS,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

O sistema tem **12** atos pendentes.
""",
}

CENSO_ATOS_TEXTO_CORRIDO = {
    **CENSO_ATOS,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

O sistema tem 3 atos pendentes.
""",
}

CENSO_ATOS_NEGRITO_FRASE = {
    **CENSO_ATOS,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

O sistema tem **3 atos** pendentes.
""",
}

CENSO_ATOS_TEXTO_CORRIDO_DATADO = {
    **CENSO_ATOS,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

## Lote (2026-11-24)

O sistema tinha 3 atos pendentes.
""",
}

CENSO_ATOS_DATADO = {
    **CENSO_ATOS,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

## Lote (2026-11-24)

O sistema tinha **3** atos pendentes.
""",
}

CENSO_ATOS_DATA_NA_FRASE = {
    **CENSO_ATOS,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

O sistema tinha **3** atos em 2026-11-24; hoje a fila é outra.
""",
}

SITE_OK = {
    "index.html": """<html><body><aside class="barra"></aside><h1>Home</h1>
<a href="alvo.html">alvo</a></body></html>""",
    "alvo.html": """<html><body><aside class="barra"></aside><h1>Alvo</h1></body></html>""",
}

SITE_QUEBRADO = {
    **SITE_OK,
    "index.html": """<html><body><aside class="barra"></aside><h1>Home</h1>
<a href="falta.html">quebrado</a></body></html>""",
}

# Link para diretório resolve `index.html`, como o site publicado.
SITE_DIRETORIO = {
    "index.html": """<html><body><aside class="barra"></aside><h1>Home</h1>
<a href="pasta/">pasta</a></body></html>""",
    "pasta/index.html": """<html><body><aside class="barra"></aside><h1>Pasta</h1></body></html>""",
}

# Link relativo com `..` é normalizado antes de existir.
SITE_RELATIVO = {
    "index.html": """<html><body><aside class="barra"></aside><h1>Home</h1>
<a href="sub/pagina.html">sub</a></body></html>""",
    "alvo.html": """<html><body><aside class="barra"></aside><h1>Alvo</h1></body></html>""",
    "sub/pagina.html": """<html><body><aside class="barra"></aside><h1>Sub</h1>
<a href="../alvo.html">alvo</a></body></html>""",
}

# Sintaxe 1.x citada em code span não é uso; fora dele reprova.
SINTAXE_1X_CITADA = {
    "planejamento-1.0.0/wyd-teste/docs/pagina.md": """# Página

O bloco `!!! nota` é citado como exemplo.
""",
}

SINTAXE_1X_USADA = {
    "planejamento-1.0.0/wyd-teste/docs/pagina.md": """# Página

!!! nota

Conteúdo.
""",
}


def site_repetido(paginas: int) -> dict[str, str]:
    """Site válido com N páginas e link cruzado; volume força o parse em processos."""
    return {
        f"p{indice:02d}.html": (
            '<html><body><aside class="barra"></aside>'
            f"<h1>Página {indice}</h1>"
            f'<a href="p{(indice + 1) % paginas:02d}.html">próxima</a>'
            "</body></html>"
        )
        for indice in range(paginas)
    }


def site_com_falhas(paginas: int) -> dict[str, str]:
    """Site com N páginas, cada uma com link quebrado e duas âncoras inexistentes.

    O volume (`>= 32`) força o caminho paralelo quando há `fork`; o link remoto aponta
    para uma página que também está no lote, então o alvo já foi analisado e a ordem dos
    problemas não depende de análise sob demanda.
    """
    return {
        f"p{indice:02d}.html": (
            '<html><body><aside class="barra"></aside>'
            f"<h1>Página {indice}</h1>"
            f'<a href="falta-{indice:02d}.html">quebrado</a>'
            '<a href="#nao-existe">âncora local</a>'
            f'<a href="p{(indice + 1) % paginas:02d}.html#sumiu">âncora remota</a>'
            "</body></html>"
        )
        for indice in range(paginas)
    }


ALVOS_CARGO = """[package]
name = "wyd-db"

[[test]]
name = "sistema_pg"
path = "tests/sistema/sistema_pg.rs"

[[test]]
name = "contrato_api_migrate"
path = "tests/contrato/contrato_api_migrate.rs"
"""

ALVOS_GUIA = """# Guia de teste

## 1. Os alvos

| Alvo | Prova | Banco | Estado |
|---|---|---|---|
| `sistema_pg` | schema por `psql` | sim | 13/13 em 2026-11-25 |
| `contrato_api_migrate` | API de `migrate` | não | 1 passed em 2026-11-26 |
| `contrato_catalogo` | catálogo × série | não | planejado |
| `sistema_sessoes` | papel e view | sim | planejado |
"""

ALVOS_OK = {
    "crates/wyd-db/Cargo.toml": ALVOS_CARGO,
    "planejamento-1.0.0/wyd-db/docs/guia-teste/index.md": ALVOS_GUIA,
    "planejamento-1.0.0/wyd-db/tests/index.md": """# Índice da suíte

| Alvo (`Cargo.toml`) | Arquivo |
|---|---|
| `sistema_pg` | `sistema/sistema_pg.rs` |
| `contrato_migrations` | `contrato/contrato_migrations.rs` |

`cargo test -p wyd-db --test sistema_pg`
""",
}

ALVOS_COMANDO_INEXISTENTE = {
    **ALVOS_OK,
    "planejamento-1.0.0/wyd-db/tests/index.md": """# Índice da suíte

| Alvo (`Cargo.toml`) | Arquivo |
|---|---|
| `sistema_pg` | `sistema/sistema_pg.rs` |

`cargo test -p wyd-db --test contrato_forma --test alvo_fantasma`
""",
}

ALVOS_TABELA_INEXISTENTE = {
    **ALVOS_OK,
    "planejamento-1.0.0/wyd-db/tests/index.md": """# Índice da suíte

| Alvo (`Cargo.toml`) | Arquivo |
|---|---|
| `sistema_pg` | `sistema/sistema_pg.rs` |
| `contrato_forma` | `contrato/contrato_forma.rs` |
""",
}

ALVOS_PROSA_OK = {
    **ALVOS_OK,
    "planejamento-1.0.0/wyd-db/docs/guia-teste/pagina.md": """# Página

O alvo `sistema_pg` abre conexão; os alvos `sistema_pg` e `contrato_migrations` são citados
na mesma frase. Ficam fora o alvo `P`, o alvo `crates/wyd-db/tests/sistema/sistema_pg.rs` e o
alvo `integração`, que não são nomes de alvo.
""",
}

ALVOS_PROSA_INEXISTENTE = {
    **ALVOS_OK,
    "planejamento-1.0.0/wyd-db/docs/guia-teste/pagina.md": """# Página

Aplicar no alvo `contrato_forma` antes do corte.
""",
}

ALVOS_TEST_SEM_COMANDO = {
    **ALVOS_OK,
    "planejamento-1.0.0/wyd-db/docs/guia-teste/pagina.md": """# Página

O nome do alvo não mudou na reorganização (`--test sistema_pg`, `--test contrato_forma`,
`--test alvo_fantasma`).
""",
}

ALVOS_COMANDO_PLANEJADO = {
    **ALVOS_OK,
    "planejamento-1.0.0/wyd-db/docs/roadmap/WYDDB-028.md": """# WYDDB-028

## DoD

- [ ] O alvo é declarado e roda com `cargo test -p wyd-db --test contrato_catalogo`.
""",
}

ALVOS_COMANDO_PLANEJADO_SEM_MARCA = {
    **ALVOS_OK,
    "planejamento-1.0.0/wyd-db/docs/roadmap/WYDDB-028.md": """# WYDDB-028

Rode `cargo test -p wyd-db --test contrato_catalogo` antes do corte.
""",
}

ALVOS_PLANEJADO_JA_EXECUTADO = {
    **ALVOS_OK,
    "planejamento-1.0.0/wyd-db/docs/guia-teste/index.md": ALVOS_GUIA.replace(
        "| `contrato_catalogo` | catálogo × série | não | planejado |",
        "| `contrato_catalogo` | catálogo × série | não | 12/12 em 2026-11-27 |",
    ),
    "planejamento-1.0.0/wyd-db/docs/roadmap/WYDDB-028.md": """# WYDDB-028

## DoD

- [ ] O alvo é declarado e roda com `cargo test -p wyd-db --test contrato_catalogo`.
""",
}

ALVOS_MANIFESTO_QUEBRADO = {
    **ALVOS_OK,
    "planejamento-1.0.0/wyd-db/skills/test/provar/SKILL.md": """# Provar

```bash
cargo test --manifest-path crates/wyd-db/Cargo.toml \\
    --test contrato_forma -- --test-threads=1
```
""",
}

ALVOS_OUTRO_CRATE = {
    **ALVOS_OK,
    "planejamento-1.0.0/wyd-db/skills/test/provar/SKILL.md": """# Provar

```bash
cargo test -p wyd-protocol --test sistema_pg
```
""",
}

ALVOS_SEM_PLANEJADOS = {
    **ALVOS_OK,
    "planejamento-1.0.0/wyd-db/docs/guia-teste/index.md": ALVOS_GUIA.replace(
        "| não | planejado |", "| não | 12/12 em 2026-11-27 |"
    ).replace("| sim | planejado |", "| sim | 4/4 em 2026-11-27 |"),
}

ALVOS_GUIA_SEM_TABELA = {
    **ALVOS_OK,
    "planejamento-1.0.0/wyd-db/docs/guia-teste/index.md": """# Guia de teste

## 1. Os alvos

Sem tabela: a lista de alvos saiu da página.
""",
}

CAPACIDADES_DONO = """# Matriz de capacidades

**Estado:** aceito (2026-11-27).

## 7. Censo e dívida

**Medido em 2026-11-27**, com a fonte de cada linha:

| Medida | Número | Fonte |
|---|---:|---|
| capacidades na matriz | **39** | §2–§6 |
| cobertas por teste executado | **20** | `sistema_pg`, `runner_pg`, `contrato_migrations` |
| alvos | **7** | `../index.md` §2 |
"""

CAPACIDADES_OK = {
    "planejamento-1.0.0/wyd-db/tests/specs/capacidades.md": CAPACIDADES_DONO,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

A matriz está **20 de 39** capacidades cobertas.

As **19 de 39** capacidades sem execução seguem abertas.
""",
}

CAPACIDADES_VELHO = {
    **CAPACIDADES_OK,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

A matriz está **14 de 39** capacidades cobertas.
""",
}

CAPACIDADES_HISTORICO = {
    **CAPACIDADES_OK,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

## 2. Medição de 2026-11-24

A matriz estava em **13 de 39** cobertas.
""",
}

CAPACIDADES_PLANEJADO = {
    **CAPACIDADES_OK,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

A cobertura vai de **20 a 22 de 39** se os dois passarem.
""",
}

CAPACIDADES_MEDIDA_NOVA = {
    "planejamento-1.0.0/wyd-db/tests/specs/capacidades.md": CAPACIDADES_DONO.replace(
        "| **20** | `sistema_pg`", "| **21** | `sistema_pg`"
    ),
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

A matriz está **21 de 39** capacidades cobertas.
""",
}

CAPACIDADES_SEM_DONO = {
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

A matriz está **20 de 39** capacidades cobertas.
""",
}

CAPACIDADES_TOTAL_OK = {
    **CAPACIDADES_OK,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

As **39 capacidades** da matriz têm requisito nomeado.
""",
}

CAPACIDADES_TOTAL_VELHO = {
    **CAPACIDADES_OK,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

As **38 capacidades** da matriz têm requisito nomeado.
""",
}

CAPACIDADES_SEM_EXECUCAO_VELHO = {
    **CAPACIDADES_OK,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

**18 capacidades sem execução** seguem abertas.
""",
}

CAPACIDADES_RECORTE = {
    **CAPACIDADES_OK,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

As **11 capacidades** governadas por gate passaram a citar `RNF-DB-008`.
""",
}

CAPACIDADES_SECAO_HERDADA = {
    **CAPACIDADES_OK,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

## 2. Lote de 2026-11-24

### 2.1 Detalhe do lote

A matriz tinha **23 capacidades**.
""",
}

CAPACIDADES_RECORTE_CAP = {
    **CAPACIDADES_OK,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

O alvo: **27 capacidades que só existem contra PostgreSQL 17 real**.

Já rodaram **20 capacidades** (`CAP-24`–`CAP-37`).
""",
}

CAPACIDADES_DATA_NA_LINHA = {
    **CAPACIDADES_OK,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

A matriz nasceu em 2026-11-24 com **23 capacidades** e nenhuma coberta.
""",
}

CAPACIDADES_DATA_NAO_QUALIFICA = {
    **CAPACIDADES_OK,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

Em 2026-11-24 a matriz tinha **23** capacidades. Hoje ela tem **38 capacidades**.
""",
}

LACUNA_WYD_DB = {
    "planejamento-1.0.0/wyd-db/lacunas.md": """# Lacunas de teste

Medido em 2026-11-27.

## 1. Situação em uma linha

| Grupo | Quantas | Bloqueadas | Fechadas |
|---|---:|---:|---:|
| Lacunas (`B`/`A`/`M`) | 39 | 3 (`A11`) | 36 (`B1`) |

## 2. Lacunas

| ID | Pergunta | Evidência |
|---|---|---|
| `B1` | o item foi decidido? | **fechada** (2026-01-01: decidido); evidência em `planejamento-1.0.0/wyd-db/evidencia.md` |
""",
    "planejamento-1.0.0/wyd-db/evidencia.md": """# Evidência

`B1` decidido: o artefato existe e trata do item.
""",
}

LACUNA_CITACAO_OK = {
    **LACUNA_WYD_DB,
    "planejamento-1.0.0/wyd-db/docs/pagina.md": """# Página

O `wyd-db` tem 39 lacunas: 0 abertas, 36 fechadas e 3 externas.
""",
}

LACUNA_CITACAO_VELHA = {
    **LACUNA_WYD_DB,
    "planejamento-1.0.0/wyd-db/docs/pagina.md": """# Página

O `wyd-db` tem 39 lacunas: 0 abertas, 35 fechadas e 3 externas.
""",
}

LACUNA_CITACAO_HISTORICA = {
    **LACUNA_WYD_DB,
    "planejamento-1.0.0/wyd-db/docs/pagina.md": """# Página

## 2. Lote (2026-11-24)

As 38 lacunas seguem 0/0/35/3.
""",
}

LACUNA_CITACAO_MOTIVOS = {
    **LACUNA_WYD_DB,
    "planejamento-1.0.0/wyd-db/docs/pagina.md": """# Página

Em 2026-11-24 o `wyd-db` tinha 38 lacunas.

As 35 lacunas fechadas subiram de 35 para 36 no lote seguinte.
""",
}


HOJE_BASE = {
    **CENSO_ATOS,
    **LACUNA_WYD_DB,
    "planejamento-1.0.0/wyd-db/tests/specs/capacidades.md": CAPACIDADES_DONO,
    # Medidas da `contagens`: a série (2 arquivos, 9 `Tab.`), o censo de fontes e as páginas do site.
    "planejamento-1.0.0/wyd-db/migrations-checklist.md": """# Checklist

## 2. Arquivos

| # | Página | Tab. |
|---|---|---|
| 0033 | `0033_x.md` | 4 |
| 0034 | `0034_y.md` | 5 |
""",
    "wyd-docs/docs/index.md": "# Site\n",
}

HOJE_ATOS_VELHO = {
    **HOJE_BASE,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

Hoje o sistema tem **3** atos pendentes.
""",
}

HOJE_ATOS_OK = {
    **HOJE_BASE,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

Hoje o sistema tem **2** atos pendentes.
""",
}

HOJE_SECAO_DATADA = {
    **HOJE_BASE,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

## Lote (2026-11-24)

Hoje o sistema tinha **3** atos pendentes.
""",
}

HOJE_CAPACIDADES_VELHO = {
    **HOJE_BASE,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

Hoje a matriz está **14 de 39** capacidades cobertas.
""",
}

HOJE_SEM_UNIDADE = {
    **HOJE_BASE,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

Hoje o sistema tem **3** pendências.
""",
}

HOJE_CONTEXTO_DONO = {
    **HOJE_BASE,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

A leva `docs/` tinha **141** atos; hoje **3** no sistema, `pendencias-de-fechamento.md` §6.6.22.1.
""",
}

HOJE_CONTEXTO_UNIDADE = {
    **HOJE_BASE,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

Hoje o total de atos é **3**.
""",
}

HOJE_CONTEXTO_OK = {
    **HOJE_BASE,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

Hoje o total de atos é **2**.
""",
}

HOJE_CONTEXTO_AMBIGUO = {
    **HOJE_BASE,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

Hoje **31** das 40 linhas têm página, `docs/specs/rastreabilidade.md` §2.
""",
}

HOJE_CONTAGENS_VELHO = {
    **HOJE_BASE,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

Hoje a checklist da série mede 3 arquivos e 9 `Tab.`, não 2.
""",
}

HOJE_CONTAGENS_FONTES = {
    **HOJE_BASE,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

O censo tem hoje **999** fontes publicadas.
""",
}

HOJE_CONTAGENS_SITE = {
    **HOJE_BASE,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

O site publica hoje **999** páginas do site.
""",
}

HOJE_CONTAGENS_MATERIALIZADA = {
    **HOJE_BASE,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

A série materializada tem hoje **28** arquivos, seis sem página escrita.
""",
}

HOJE_CONTAGENS_AMBIGUO = {
    **HOJE_BASE,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

Recortes não fecham: hoje 45 arquivos, 109 citações e 49 menções.
""",
}

HOJE_CONTAGENS_OUTRO_CONJUNTO = {
    **HOJE_BASE,
    "planejamento-1.0.0/wyd-ops/docs/pagina.md": """# Página

Hoje **111** tabelas em `content`/`game`/`portal`/`ops`.
""",
}
