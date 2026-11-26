#!/usr/bin/env python3
r"""Gate de censos: mede os quatro números de estado e confere o que as páginas citam.

Medidas (fontes independentes, cada uma com a regra ao lado):

1. **atos de aceite** — páginas que ainda não estão `aceito` nas quatro pastas do `wyd-db`:
   `docs/` (autorais: fora de `dicionario-dados/gerado/` e de `proposta-*`), `src/`, `migrations/`
   (só `NNNN-*.md`; `index.md` não é ato) e `tests/`. Mede a soma, não a citação.
2. **páginas aceitas** — arquivos `.md` de `planejamento-1.0.0/` cuja linha `**Estado:**` diz
   `aceito`, contados por pasta.
3. **células pendentes** — linhas de requisito de `docs/specs/rastreabilidade.md` §2: célula `Teste`
   com `P` e célula `Issue` ainda em `onda N` (sem número de issue). Mede `teste`, `issue` e o total.
4. **`Q-BLK` abertas** — linhas da tabela de status de `pendencias-de-fechamento.md` cuja célula de
   situação começa por `aberta` e cujo ID é `Q-BLK-nn`. Mede a lista, não a contagem citada; **zero
   aberta é medida válida** e imprime `nenhuma`.

Regra das citações: todo `**N** atos de aceite`, `**N** páginas \`aceito\``, `**N** células
pendentes` (ou `**N** células (X \`teste\` + Y \`issue\`)`), `**N** \`Q-BLK\` abertas` e
`**N** células \`Teste\` seguem \`P\`` encontrado em `planejamento-1.0.0/**/*.md` é comparado com a
medida. Citação de outro dono não existe aqui: os quatro números são do `wyd-db`. Linha com marca
explícita de registro histórico (`na medição de então`, `naquele momento`, `naquele lote`, `na leva 1`,
`seguiam abertas`) é impressa como `ignorado` e não reprova — é o dossiê que guarda a passada.

Falha (exit 1) em divergência; exit 2 quando não dá para medir (arquivo ou pasta ausente).
Vive em `wyd-docs/` porque `planejamento-1.0.0/` é documentação e não aceita executável (AGENTS §5.6).

Uso:
    python -m gate check censos --root ..   # a partir de wyd-docs/
"""

from __future__ import annotations

import re
from pathlib import Path
from typing import NamedTuple

from ..model import CheckResult
from ..workspace import Workspace

_WS: Workspace | None = None


def _ws() -> Workspace:
    if _WS is None:
        raise RuntimeError("censos: Workspace não inicializado")
    return _WS

PLANEJAMENTO = "planejamento-1.0.0"
SISTEMA = "planejamento-1.0.0/wyd-db"
RASTREABILIDADE = "planejamento-1.0.0/wyd-db/docs/specs/rastreabilidade.md"
PENDENCIAS = "planejamento-1.0.0/wyd-db/pendencias-de-fechamento.md"

ESTADO_RE = re.compile(r"^[\s>]*\*\*Estado:\*\*[ ]*([^\s(]+)", re.IGNORECASE)
REQUISITO_RE = re.compile(r"^\|\s*`((?:RF|RN|RNF|UC)-DB-[0-9]{3})`\s*\|")
MIGRATION_RE = re.compile(r"[0-9]{4}-[a-z0-9-]+[.]md$")
QBLK_RE = re.compile(r"`(Q-BLK-[0-9]+)`")
ABERTA_RE = re.compile(r"^[ *]*aberta([^a-z]|$)", re.IGNORECASE)

ATOS_RE = re.compile(r"\*\*(\d+)\*\*\s+atos de aceite")
ATOS_TABELA_RE = re.compile(r"atos de aceite pendentes[^|]*\|\s*\*{0,2}(\d+)\*{0,2}")
PAGINAS_RE = re.compile(r"\*\*(\d+)\*\*\s+páginas\s+`aceito`")
PAGINAS_ALT_RE = re.compile(r"\*\*(\d+)\*\*\s+páginas aceitas")
CELULAS_RE = re.compile(r"\*\*(\d+)\*\*\s+células pendentes(?!\s*\()")
CELULAS_PARCIAL_RE = re.compile(
    r"\*\*(\d+)\*\*\s+células[^(]{0,30}\(\s*(\d+)\s*`?teste`?\s*\+\s*(\d+)\s*`?issue`?\s*\)",
    re.IGNORECASE,
)
CELULAS_P_RE = re.compile(r"\*\*(\d+)\*\*\s+células\s+`Teste`\s+seguem\s+`P`", re.IGNORECASE)
QBLK_PRE_RE = re.compile(r"\*\*(\d+)\*\*\s+`Q-BLK`\s+abertas")
QBLK_POS_RE = re.compile(r"`Q-BLK`\s+abertas\s+\*\*(\d+)\*\*")

# Marca explícita de que a linha é registro de uma passada, não censo vigente. A lista é curta e
# impressa quando casa: nada é ignorado sem rastro.
HISTORICO = (
    "na medição de então",
    "naquele momento",
    "naquele lote",
    "na leva 1",
    "seguiam abertas",
    "seguia aberta",
    "antes desta leva",
)


class Citacao(NamedTuple):
    arquivo: str
    linha: int
    valor: int
    unidade: str
    extra: tuple[int, ...] = ()


class Erro(Exception):
    """Falha de leitura que impede medir — não é divergência de censo."""


def ler(raiz: Path, relativo: str) -> str:
    caminho = raiz / relativo
    if not _ws().exists(caminho) or not caminho.is_file():
        raise Erro(f"arquivo ausente: {caminho}")
    return _ws().read(caminho)


def _estado(arquivo: Path) -> str | None:
    for linha in _ws().read(arquivo).splitlines():
        achado = ESTADO_RE.match(linha)
        if achado:
            return achado.group(1).lower()
    return None


def _paginas(pasta: Path) -> list[Path]:
    if not _ws().is_dir(pasta):
        raise Erro(f"pasta ausente: {pasta}")
    return list(_ws().files(pasta, "*.md"))


def medir_atos(raiz: Path) -> dict[str, int]:
    """Páginas sem `aceito`, por pasta, com a mesma regra de §6.6.22.1."""
    base = raiz / SISTEMA
    docs = [
        pagina
        for pagina in _paginas(base / "docs")
        if "/dicionario-dados/gerado/" not in pagina.as_posix()
        and not pagina.name.startswith("proposta-")
    ]
    src = _paginas(base / "src")
    migrations = [
        pagina
        for pagina in _paginas(base / "migrations")
        if MIGRATION_RE.search(pagina.name)
    ]
    tests = _paginas(base / "tests")
    partes = {
        "docs": sum(1 for pagina in docs if _estado(pagina) != "aceito"),
        "src": sum(1 for pagina in src if _estado(pagina) != "aceito"),
        "migrations": sum(1 for pagina in migrations if _estado(pagina) != "aceito"),
        "tests": sum(1 for pagina in tests if _estado(pagina) != "aceito"),
    }
    partes["total"] = sum(partes.values())
    return partes


def medir_aceitas(raiz: Path) -> tuple[int, dict[str, int]]:
    """Páginas `aceito` do `wyd-db`, com o recorte por pasta do sistema."""
    base = raiz / SISTEMA
    if not base.is_dir():
        raise Erro(f"pasta ausente: {base}")
    por_pasta: dict[str, int] = {}
    for pagina in _ws().files(base, "*.md"):
        if _estado(pagina) != "aceito":
            continue
        pasta = pagina.relative_to(base).parts[0]
        por_pasta[pasta] = por_pasta.get(pasta, 0) + 1
    return sum(por_pasta.values()), por_pasta


def medir_celulas(raiz: Path) -> tuple[int, int]:
    """(teste em `P`, issue sem número) das linhas de requisito de §2."""
    teste = issue = 0
    for linha in ler(raiz, RASTREABILIDADE).splitlines():
        if not REQUISITO_RE.match(linha):
            continue
        celulas = [celula.strip() for celula in linha.strip().strip("|").split("|")]
        if len(celulas) < 7:
            raise Erro(f"{RASTREABILIDADE}: linha de requisito com {len(celulas)} células")
        if "`P`" in celulas[4]:
            teste += 1
        if "onda" in celulas[6].lower() and not re.search(r"#\s*[0-9]+", celulas[6]):
            issue += 1
    return teste, issue


def medir_qblk(raiz: Path) -> list[str]:
    """IDs `Q-BLK` cuja situação na tabela de status começa por `aberta` (com ou sem negrito)."""
    abertas: list[str] = []
    for linha in ler(raiz, PENDENCIAS).splitlines():
        if not linha.startswith("|"):
            continue
        ids = QBLK_RE.findall(linha)
        if not ids:
            continue
        celulas = [celula.strip() for celula in linha.strip().strip("|").split("|")]
        if any(ABERTA_RE.match(celula) for celula in celulas):
            abertas.append(ids[0])
    return abertas


def _extra_paginas(linha: str, achado: re.Match[str]) -> tuple[int, ...]:
    return ()


def _historico(linha: str) -> str | None:
    for marca in HISTORICO:
        if marca in linha:
            return marca
    return None


def citacoes(raiz: Path) -> tuple[list[Citacao], list[tuple[Citacao, str]]]:
    """Citações dos quatro censos e as linhas declaradas históricas."""
    vigentes: list[Citacao] = []
    historicas: list[tuple[Citacao, str]] = []
    base = raiz / PLANEJAMENTO
    if not base.is_dir():
        raise Erro(f"pasta ausente: {base}")
    for pagina in _ws().files(base, "*.md"):
        relativo = pagina.relative_to(raiz).as_posix()
        for numero, linha in enumerate(_ws().read(pagina).splitlines(), start=1):
            achados: list[Citacao] = []
            for achado in ATOS_RE.finditer(linha):
                achados.append(Citacao(relativo, numero, int(achado.group(1)), "atos"))
            for achado in ATOS_TABELA_RE.finditer(linha):
                achados.append(Citacao(relativo, numero, int(achado.group(1)), "atos"))
            for achado in PAGINAS_RE.finditer(linha):
                achados.append(Citacao(relativo, numero, int(achado.group(1)), "páginas"))
            for achado in PAGINAS_ALT_RE.finditer(linha):
                achados.append(Citacao(relativo, numero, int(achado.group(1)), "páginas"))
            for achado in CELULAS_RE.finditer(linha):
                achados.append(Citacao(relativo, numero, int(achado.group(1)), "células"))
            for achado in CELULAS_PARCIAL_RE.finditer(linha):
                achados.append(
                    Citacao(
                        relativo,
                        numero,
                        int(achado.group(1)),
                        "células",
                        (int(achado.group(2)), int(achado.group(3))),
                    )
                )
            for achado in CELULAS_P_RE.finditer(linha):
                achados.append(Citacao(relativo, numero, int(achado.group(1)), "teste"))
            for achado in QBLK_PRE_RE.finditer(linha):
                achados.append(Citacao(relativo, numero, int(achado.group(1)), "qblk"))
            for achado in QBLK_POS_RE.finditer(linha):
                achados.append(Citacao(relativo, numero, int(achado.group(1)), "qblk"))
            if not achados:
                continue
            marca = _historico(linha)
            if marca:
                historicas.extend((citacao, marca) for citacao in achados)
            else:
                vigentes.extend(achados)
    return vigentes, historicas


def run(ws: Workspace) -> CheckResult:
    global _WS
    _WS = ws
    raiz = ws.root

    try:
        atos = medir_atos(raiz)
        aceitas, aceitas_por_pasta = medir_aceitas(raiz)
        teste, issue = medir_celulas(raiz)
        qblk = medir_qblk(raiz)
        vigentes, historicas = citacoes(raiz)
    except Erro as erro:
        return CheckResult("censos", False, str(erro))

    medido = {
        "atos": atos["total"],
        "páginas": aceitas,
        "células": teste + issue,
        "teste": teste,
        "qblk": len(qblk),
    }
    problemas: list[str] = []
    for citacao in vigentes:
        esperado = medido[citacao.unidade]
        if citacao.valor != esperado:
            problemas.append(
                f"{citacao.arquivo}:{citacao.linha}: cita {citacao.valor} {citacao.unidade}, "
                f"medido {esperado}"
            )
        if citacao.unidade == "células" and citacao.extra and citacao.extra != (teste, issue):
            problemas.append(
                f"{citacao.arquivo}:{citacao.linha}: cita {citacao.extra[0]} teste + "
                f"{citacao.extra[1]} issue, medido {teste} + {issue}"
            )

    resumo = (
        f"{atos['total']} atos de aceite "
        f"({atos['docs']} docs + {atos['src']} src + {atos['migrations']} migrations + "
        f"{atos['tests']} tests); {aceitas} páginas aceitas "
        f"({', '.join(f'{pasta} {numero}' for pasta, numero in sorted(aceitas_por_pasta.items()))}); "
        f"{teste + issue} células pendentes ({teste} teste + {issue} issue); "
        f"{len(qblk)} Q-BLK abertas ({', '.join(qblk) or 'nenhuma'}); "
        f"{len(vigentes)} citação(ões) conferida(s), {len(historicas)} histórica(s)"
    )
    detalhes = [
        f"ignorado {c.arquivo}:{c.linha}: {c.valor} {c.unidade} ({marca})"
        for c, marca in historicas
    ]
    detalhes += [
        f"ok {c.arquivo}:{c.linha}: {c.valor} {c.unidade}" for c in vigentes
    ]
    if problemas:
        return CheckResult("censos", False, resumo, problems=problemas, details=detalhes)
    return CheckResult("censos", True, resumo, details=detalhes)
