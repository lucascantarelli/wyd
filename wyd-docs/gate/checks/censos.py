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

Regra das citações: todo `N atos` — `**N** atos de aceite`, `**N** atos`, `**N atos**` ou texto
corrido (`141 atos`) —, `**N** páginas \`aceito\``, `**N** células pendentes` (ou `**N** células
(X \`teste\` + Y \`issue\`)`), `**N** \`Q-BLK\` abertas` e `**N** células \`Teste\` seguem
\`P\`` encontrado em `planejamento-1.0.0/**/*.md` é comparado com a medida. O negrito não muda a
regra: o que importa é a citação estar em texto corrente do planejamento. Citação colada a uma pasta
(`141 atos de \`docs/\``, `src/`, `migrations/` ou `tests/`) é contagem daquela pasta, não do total,
e fica fora; o mesmo parágrafo pode citar o total e a pasta, e cada citação é julgada sozinha. Só
`12`, `27` e `37` — os tamanhos de leva conhecidos — ficam fora do censo, e só quando a `leva`
**qualifica o próprio número**, colada a ele (`12 atos da leva`, `leva `tests/` — 12 atos`); `leva`
em outra oração da mesma frase não isenta, e pelo valor sozinho eles são totais como qualquer outro. `34 atos` citado na linha que fala de `M33` é o aceite da série de
migrations, dono próprio, e sai como `escopo \`M33\``. Não há corte relativo ao total: qualquer
citação é conferida, inclusive valores menores que metade do censo. Citação de outro dono não existe
aqui: os quatro números são do `wyd-db`. Linha com marca explícita de registro histórico (`na medição
de então`, `naquele momento`, `naquele lote`, `na leva 1`, `seguiam abertas`) é impressa como
`ignorado` e não reprova — é o dossiê que guarda a passada. Data anterior à medição na mesma frase da
citação, ou na seção que a herda, também isenta a citação de `N atos`, com a data impressa no detalhe;
baseline de dossiê/plano entra assim, datada.

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
from . import _texto

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
# Qualquer `N atos`, com ou sem negrito (`**141** atos`, `**141 atos**`, `141 atos`).
ATOS_ANY_RE = re.compile(r"\*{0,2}\b(\d+)\*{0,2}\s+atos\b(?!\s*/\s*issues)")
ATOS_TABELA_RE = re.compile(r"atos de aceite pendentes[^|]*\|\s*\*{0,2}(\d+)\*{0,2}")
# Colado na própria citação (`141 atos de `docs/``), não na linha: o mesmo parágrafo costuma citar o total.
ATOS_DE_PASTA_RE = re.compile(r"\s+de\s+`(?:docs|src|migrations|tests)/`")
# Tamanhos de leva conhecidos. Só ficam fora do censo quando a frase que contém o número diz que ele é
# de uma leva; pelo valor sozinho, `12`, `27` e `37` são totais como qualquer outro e reprovam se não
# baterem com a medição. Totais de outro tamanho nunca usam esta isenção.
ATOS_LEVAS = frozenset({12, 27, 37})
LEVA_RE = re.compile(r"\blevas?\b", re.IGNORECASE)
# A leva tem de qualificar o próprio número: entre ela e a citação só cabe conectivo, não outra oração.
LEVA_JANELA = 40
LEVA_PALAVRAS = 4
LEVA_CORTE_RE = re.compile(r"[;.,|]")
# `34 atos` é o aceite da série de migrations (`M33`), não o censo do sistema: um fato, um dono.
ESCOPO_M33_RE = re.compile(r"\bM33\b")
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


class Medidas(NamedTuple):
    """O que o censo mede hoje, pelo contrato público usado por quem compõe a medida."""

    atos: dict[str, int]
    aceitas: int
    por_pasta: dict[str, int]
    teste: int
    issue: int
    qblk: list[str]
    corte: str | None


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


def _historico(linha: str) -> str | None:
    for marca in HISTORICO:
        if marca in linha:
            return marca
    return None


def _leva_qualifica(linha: str, inicio: int, fim: int) -> bool:
    """`leva` colada ao número: `12 atos da leva`, `leva `tests/` — 12 atos`, `leva 1 (12 atos)`."""
    for achado in LEVA_RE.finditer(linha):
        if achado.end() <= inicio:
            trecho = linha[achado.end() : inicio]
        elif achado.start() >= fim:
            trecho = linha[fim : achado.start()]
        else:
            continue
        if (
            len(trecho) <= LEVA_JANELA
            and len(trecho.split()) <= LEVA_PALAVRAS
            and not LEVA_CORTE_RE.search(trecho)
        ):
            return True
    return False


def _motivo_atos(
    linha: str, achado: re.Match[str], valor: int, data_secao: str | None, corte: str | None
) -> str | None:
    """Isenção de uma citação de `N atos`: pasta, leva, data na frase ou seção datada."""
    if ATOS_DE_PASTA_RE.match(linha, achado.end()):
        return "atos de uma pasta"
    if ESCOPO_M33_RE.search(linha):
        return "escopo `M33`"
    if valor in ATOS_LEVAS and _leva_qualifica(linha, *achado.span()):
        return "contagem de leva"
    isento = _texto.isencao_por_data(linha, achado.start(), data_secao, corte)
    if isento:
        origem, data = isento
        return f"data {data} na frase" if origem == "frase" else f"seção datada {data}"
    return None


def _corte(raiz: Path) -> str | None:
    datas = _texto.DATA_RE.findall(ler(raiz, PENDENCIAS))
    return max(datas) if datas else None


def citacoes(
    raiz: Path, corte: str | None
) -> tuple[list[Citacao], list[tuple[Citacao, str]]]:
    """Citações dos quatro censos e as linhas declaradas históricas."""
    vigentes: list[Citacao] = []
    historicas: list[tuple[Citacao, str]] = []
    base = raiz / PLANEJAMENTO
    if not base.is_dir():
        raise Erro(f"pasta ausente: {base}")
    for pagina in _ws().files(base, "*.md"):
        relativo = pagina.relative_to(raiz).as_posix()
        pilha: list[tuple[int, str | None]] = []
        for numero, linha in enumerate(_ws().read(pagina).splitlines(), start=1):
            if linha.startswith("#"):
                nivel = len(linha) - len(linha.lstrip("#"))
                while pilha and pilha[-1][0] >= nivel:
                    pilha.pop()
                achado = _texto.DATA_RE.search(linha)
                pilha.append((nivel, achado.group(1) if achado else None))
                continue
            data_secao = _texto.data_da_secao(pilha)
            achados: list[tuple[Citacao, str | None]] = []
            ocupados: list[tuple[int, int]] = []
            for padrao in (ATOS_RE, ATOS_TABELA_RE):
                for achado in padrao.finditer(linha):
                    achados.append((Citacao(relativo, numero, int(achado.group(1)), "atos"), None))
                    ocupados.append(achado.span())
            for achado in ATOS_ANY_RE.finditer(linha):
                if any(inicio <= achado.start() < fim for inicio, fim in ocupados):
                    continue
                valor = int(achado.group(1))
                citacao = Citacao(relativo, numero, valor, "atos")
                achados.append((citacao, _motivo_atos(linha, achado, valor, data_secao, corte)))
            for achado in PAGINAS_RE.finditer(linha):
                achados.append((Citacao(relativo, numero, int(achado.group(1)), "páginas"), None))
            for achado in PAGINAS_ALT_RE.finditer(linha):
                achados.append((Citacao(relativo, numero, int(achado.group(1)), "páginas"), None))
            for achado in CELULAS_RE.finditer(linha):
                achados.append((Citacao(relativo, numero, int(achado.group(1)), "células"), None))
            for achado in CELULAS_PARCIAL_RE.finditer(linha):
                achados.append(
                    (
                        Citacao(
                            relativo,
                            numero,
                            int(achado.group(1)),
                            "células",
                            (int(achado.group(2)), int(achado.group(3))),
                        ),
                        None,
                    )
                )
            for achado in CELULAS_P_RE.finditer(linha):
                achados.append((Citacao(relativo, numero, int(achado.group(1)), "teste"), None))
            for achado in QBLK_PRE_RE.finditer(linha):
                achados.append((Citacao(relativo, numero, int(achado.group(1)), "qblk"), None))
            for achado in QBLK_POS_RE.finditer(linha):
                achados.append((Citacao(relativo, numero, int(achado.group(1)), "qblk"), None))
            if not achados:
                continue
            marca = _historico(linha)
            for citacao, motivo_registro in achados:
                motivo = marca or motivo_registro
                if motivo:
                    historicas.append((citacao, motivo))
                else:
                    vigentes.append(citacao)
    return vigentes, historicas


def medidas(ws: Workspace) -> Medidas:
    """Mede os quatro censos: quem precisa comparar contra eles não toca em `_WS`."""
    global _WS
    _WS = ws
    raiz = ws.root
    atos = medir_atos(raiz)
    aceitas, por_pasta = medir_aceitas(raiz)
    teste, issue = medir_celulas(raiz)
    qblk = medir_qblk(raiz)
    return Medidas(atos, aceitas, por_pasta, teste, issue, qblk, _corte(raiz))


def run(ws: Workspace) -> CheckResult:
    try:
        medida = medidas(ws)
        vigentes, historicas = citacoes(ws.root, medida.corte)
    except Erro as erro:
        return CheckResult("censos", False, str(erro))
    atos = medida.atos
    aceitas = medida.aceitas
    aceitas_por_pasta = medida.por_pasta
    teste, issue = medida.teste, medida.issue
    qblk = medida.qblk

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
    falhas = {
        (c.arquivo, c.linha, c.unidade, c.valor)
        for c in vigentes
        if c.valor != medido[c.unidade]
        or (c.unidade == "células" and c.extra and c.extra != (teste, issue))
    }
    detalhes += [
        f"{'falha' if (c.arquivo, c.linha, c.unidade, c.valor) in falhas else 'ok'} "
        f"{c.arquivo}:{c.linha}: {c.valor} {c.unidade}"
        for c in vigentes
    ]
    if problemas:
        return CheckResult("censos", False, resumo, problems=problemas, details=detalhes)
    return CheckResult("censos", True, resumo, details=detalhes)
