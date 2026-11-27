#!/usr/bin/env python3
r"""Gate de capacidades: confere toda citação de censo contra `capacidades.md` §7.

O dono do número é `planejamento-1.0.0/wyd-db/tests/specs/capacidades.md` §7. De lá saem duas
medidas: **capacidades na matriz** (o `N` da citação) e **cobertas por teste executado** (o `X`
esperado quando a citação fala de cobertura). A citação de "sem execução" espera o complemento
`N - cobertas`, que não é um número independente: é a mesma medida lida do outro lado.

Regra das citações: todo `X de N` em `planejamento-1.0.0/**/*.md` é comparado com a medida, e o
mesmo censo confere as formas que não usam `de N`:

- `N capacidades sem execução` (ou `sem teste`, `ainda não têm teste`) → `N - cobertas`;
- `N capacidades cobertas` → `cobertas`;
- `N capacidades` sem qualificador → total, **quando `N` chega ao número sem execução**; abaixo
dele é recorte de `CAP-nn` ("as 11 capacidades governadas por gate"), não censo, e fica fora.
  Seguido de `que ...` ou de `` (`CAP-...`) ``, também é recorte: "27 capacidades que só existem
  contra PostgreSQL 17" e "20 capacidades (`CAP-24`–`CAP-37`)" não são o tamanho da matriz.

Ficam fora da reprovação, e são impressos como `histórico/planejado`:

- seção cujo título traz data anterior à medição do dono (ex.: `### 6.6.29 ... (2026-11-24)`),
  inclusive herdada por subseção;
- data anterior à medição na **frase da citação** — data em outra frase do mesmo bloco lógico não
  isenta: número velho ao lado de uma data antiga continua reprovando;
- linha com marca de registro histórico (`naquele momento`, `passou a`, `passou de`, `continuava`,
  `era`, `medição de`, `históric`);
- linha com transição ou meta (`→`, `de X para Y`, `a Y de N`, `vai de`, `meta`, `planejad`),
  porque ali o número é destino, não medição.

Falha (exit 1) quando a citação atual diverge; erro de leitura do dono reprova o check em vez de
inventar medida. Vive em `wyd-docs/` porque `planejamento-1.0.0/` é documentação e não aceita
executável (AGENTS §5.6).

Uso:
    python -m gate check capacidades --root ..   # a partir de wyd-docs/
"""

from __future__ import annotations

import re
from typing import NamedTuple

from ..model import CheckResult
from ..workspace import Workspace
from . import _texto

OWNER = "planejamento-1.0.0/wyd-db/tests/specs/capacidades.md"
BASE = "planejamento-1.0.0"
SECAO = "## 7."
ROTULO_MATRIZ = "capacidades na matriz"
ROTULO_COBERTAS = "cobertas por teste executado"

MEDIDO_RE = re.compile(r"Medido em\s+\*{0,2}(\d{4}-\d\d-\d\d)\*{0,2}")
LINHA_TABELA_RE = re.compile(r"^\|\s*([^|]+?)\s*\|\s*\*\*(\d+)\*\*\s*\|")
CITACAO_RE = re.compile(r"\b(\d+)\s+de\s+(\d+)\b")
CAPACIDADES_RE = re.compile(r"\b(\d+)\*{0,2}\s+capacidades\b")
DAS_RE = re.compile(r"\b(\d+)\*{0,2}\s+d[ae]s\s+\*{0,2}(\d+)\*{0,2}\s+capacidades\b")
PLANEJADO_RE = re.compile(
    r"→|->|\bde\s+\d+\s+para\s+\d+|\ba\s+\d+\s+de\s+\d+\b|vai de|planejad|previst|\bmeta\b|objetivo",
    re.IGNORECASE,
)
HISTORICO_RE = re.compile(
    r"naquele momento|passou a\b|passou de\b|continuava|\bera\b|medição de|históric",
    re.IGNORECASE,
)
SEM_EXECUCAO_RE = re.compile(r"sem execução|sem teste|ainda não|restantes|não têm teste", re.IGNORECASE)
COBERTAS_RE = re.compile(r"cobert", re.IGNORECASE)


class Censo(NamedTuple):
    total: int
    cobertas: int
    medido_em: str | None


class Citacao(NamedTuple):
    caminho: str
    linha: int
    valor: int
    esperado: int
    rotulo: str
    forma: str
    motivo: str = ""


class Erro(Exception):
    """O dono do número não está legível; o gate não inventa medida."""


def _secao(linhas: list[str]) -> list[str]:
    corpo: list[str] = []
    dentro = False
    for linha in linhas:
        if linha.startswith("#"):
            if dentro:
                break
            dentro = linha.startswith(SECAO)
            continue
        if dentro:
            corpo.append(linha)
    return corpo


def medir(ws: Workspace) -> Censo:
    caminho = ws.root / OWNER
    if not caminho.is_file():
        raise Erro(f"{OWNER} não encontrado")
    corpo = _secao(ws.read(caminho).splitlines())
    if not corpo:
        raise Erro(f"{OWNER}: seção `{SECAO}` não encontrada")

    total = cobertas = None
    for linha in corpo:
        achado = LINHA_TABELA_RE.match(linha)
        if not achado:
            continue
        rotulo = achado.group(1).strip().lower()
        valor = int(achado.group(2))
        if rotulo.startswith(ROTULO_MATRIZ):
            total = valor
        elif rotulo.startswith(ROTULO_COBERTAS):
            cobertas = valor

    if total is None or cobertas is None:
        raise Erro(f"{OWNER} §7: faltam as linhas `{ROTULO_MATRIZ}`/`{ROTULO_COBERTAS}`")
    if not 0 <= cobertas <= total:
        raise Erro(f"{OWNER} §7: cobertas {cobertas} fora de 0..{total}")

    achado = MEDIDO_RE.search("\n".join(corpo))
    return Censo(total, cobertas, achado.group(1) if achado else None)


def _esperado(linha: str, fim: int, censo: Censo) -> tuple[int, str]:
    # O que vem logo depois da citação decide o lado: "de 39 capacidades sem execução" é o
    # complemento; "de 39 capacidades cobertas" (ou sem qualificador) é a cobertura. Uma linha
    # pode citar os dois lados — por isso a janela é curta e olha só para a direita.
    depois = re.split(r"[;|()]", linha[fim : fim + 60])[0]
    if SEM_EXECUCAO_RE.search(depois):
        return censo.total - censo.cobertas, "sem execução"
    return censo.cobertas, "cobertas"


def _esperado_capacidades(linha: str, fim: int, valor: int, censo: Censo) -> tuple[int, str] | None:
    # Só o que vem imediatamente depois do número qualifica a citação; "39 capacidades em 5 alvos,
    # 0 cobertas" continua sendo o total, não a cobertura.
    inicio = linha[fim : fim + 30]
    if SEM_EXECUCAO_RE.search(inicio):
        return censo.total - censo.cobertas, "sem execução"
    if re.match(r"\s*\*{0,2}\s*cobert", inicio):
        return censo.cobertas, "cobertas"
    if re.match(r"\s*\*{0,2}\s*(?:\(`?CAP-|\bque\b)", inicio):
        return None  # "27 capacidades que só existem...", "20 capacidades (`CAP-24`–`CAP-37`)": recorte
    if valor <= censo.total - censo.cobertas:
        return None  # recorte de `CAP-nn` ou abreviatura das sem execução, não censo
    return censo.total, "matriz"


def _janela(linha: str, inicio: int, fim: int) -> str:
    return linha[max(0, inicio - 30) : fim + 30]


def _registrar(
    problemas: list[Citacao],
    historicas: list[Citacao],
    item: Citacao,
    linha: str,
    inicio: int,
    fim: int,
    data_secao: str | None,
    censo: Censo,
) -> None:
    if item.valor == item.esperado:
        return
    janela = _janela(linha, inicio, fim)
    motivos: list[str] = []
    for rotulo, padrao in (
        ("transição/planejado", PLANEJADO_RE),
        ("registro histórico", HISTORICO_RE),
    ):
        achado = padrao.search(janela)
        if achado:
            motivos.append(f"{rotulo} (`{achado.group(0).strip()}`)")
    if censo.medido_em:
        datas = _texto.datas_que_qualificam(linha, inicio)
        if datas and min(datas) < censo.medido_em:
            motivos.append(f"data na frase ({min(datas)})")
        if data_secao and data_secao < censo.medido_em:
            motivos.append(f"seção datada herdada ({data_secao})")
    if motivos:
        historicas.append(item._replace(motivo="; ".join(motivos)))
    else:
        problemas.append(item)


def varrer(ws: Workspace, censo: Censo) -> tuple[list[Citacao], list[Citacao], int]:
    problemas: list[Citacao] = []
    historicas: list[Citacao] = []
    total = 0
    for arquivo in ws.files(ws.root / BASE, "*.md"):
        texto = ws.read(arquivo)
        caminho = arquivo.relative_to(ws.root).as_posix()
        pilha: list[tuple[int, str | None]] = []
        for numero, linha in _texto.linhas_logicas(texto):
            if linha.startswith("#"):
                nivel = len(linha) - len(linha.lstrip("#"))
                while pilha and pilha[-1][0] >= nivel:
                    pilha.pop()
                achado = _texto.DATA_RE.search(linha)
                pilha.append((nivel, achado.group(1) if achado else None))
                continue
            data_secao = _texto.data_da_secao(pilha)
            for achado in CITACAO_RE.finditer(linha):
                valor, referencia = int(achado.group(1)), int(achado.group(2))
                if referencia != censo.total:
                    continue
                total += 1
                esperado, rotulo = _esperado(linha, achado.end(), censo)
                _registrar(
                    problemas,
                    historicas,
                    Citacao(caminho, numero, valor, esperado, rotulo, f"de {censo.total}"),
                    linha,
                    achado.start(),
                    achado.end(),
                    data_secao,
                    censo,
                )
            for achado in DAS_RE.finditer(linha):
                valor, referencia = int(achado.group(1)), int(achado.group(2))
                if referencia != censo.total:
                    continue
                total += 1
                esperado, rotulo = _esperado(linha, achado.end(), censo)
                _registrar(
                    problemas,
                    historicas,
                    Citacao(caminho, numero, valor, esperado, rotulo, f"das {censo.total}"),
                    linha,
                    achado.start(),
                    achado.end(),
                    data_secao,
                    censo,
                )
            for achado in CAPACIDADES_RE.finditer(linha):
                # `X de N capacidades` e `X das N capacidades` já foram contados.
                if re.search(
                    r"(?:de|d[ae]s)\s+\*{0,2}$", linha[max(0, achado.start() - 12) : achado.start()]
                ):
                    continue
                valor = int(achado.group(1))
                esperado = _esperado_capacidades(linha, achado.end(), valor, censo)
                if esperado is None:
                    continue
                total += 1
                _registrar(
                    problemas,
                    historicas,
                    Citacao(caminho, numero, valor, esperado[0], esperado[1], "capacidades"),
                    linha,
                    achado.start(),
                    achado.end(),
                    data_secao,
                    censo,
                )
    return problemas, historicas, total


def run(ws: Workspace) -> CheckResult:
    censo = medir(ws)
    problemas, historicas, total = varrer(ws, censo)
    resumo = (
        f"{total} citação(ões) de censo de capacidades conferida(s) contra §7 "
        f"({censo.total} capacidades, {censo.cobertas} cobertas); "
        f"{len(historicas)} histórica(s)/planejada(s)"
    )
    problemas_texto = [
        f"{item.caminho}:{item.linha}: cita {item.valor} {item.forma}, §7 mede {item.esperado} ({item.rotulo})"
        for item in problemas
    ]
    detalhes = problemas_texto + [
        f"{item.caminho}:{item.linha}: {item.valor} {item.forma} — isenta: {item.motivo}"
        for item in historicas
    ]
    return CheckResult("capacidades", not problemas, resumo, problemas_texto, detalhes)
