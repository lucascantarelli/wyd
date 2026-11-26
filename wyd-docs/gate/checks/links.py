"""Verifica o site gerado: link interno, âncora, tema e navegação.

O MkDocs 2.0 não tem modo estrito nem validação de link — link relativo para página
inexistente é copiado como está e vira 404 em produção. Este script é o gate do site:
varre `site/**/*.html`, resolve cada `href`/`src` local e falha quando o destino, a
âncora ou a navegação não existem. Também confere `redirects.tsv`: a origem vira
página-stub com `meta refresh`/`canonical`, o destino existe e não é outro stub. Também varre as
fontes publicadas (`planejamento-1.0.0/*/docs`) e falha na sintaxe do MkDocs 1.x que a 2.0 não
renderiza — `preparar.py` copia a página como está, então o defeito tem de ser barrado na origem.
"""

from __future__ import annotations

import multiprocessing
import os
import re
from concurrent.futures import ProcessPoolExecutor
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urlsplit

from ..model import CheckResult
from ..workspace import Workspace

_WS: Workspace | None = None
_ARVORE: "_Arvore | None" = None


def _ws() -> Workspace:
    if _WS is None:
        raise RuntimeError("links: Workspace não inicializado")
    return _WS


class _Arvore:
    """Índice imutável do site gerado.

    O site não muda durante a verificação, então existência e tipo de cada destino
    podem ser respondidos por conjunto em vez de um `stat` por link. A navegação
    lateral repete os mesmos destinos em todas as páginas: sem o índice, o gate
    fazia ~146 mil chamadas ao sistema de arquivos. Caminho ausente do índice cai
    no `Workspace`, que responde exatamente como antes (inclusive fora da raiz).
    """

    def __init__(self, raiz: Path) -> None:
        # Chave string com `normcase`: mesma igualdade que o sistema de arquivos usa
        # e muito mais barata que o hash de `Path` no Windows (~20 us por consulta).
        self.arquivos: set[str] = set()
        self.diretorios: set[str] = set()
        for caminho in raiz.rglob("*"):
            chave = _chave(caminho)
            if caminho.is_dir():
                self.diretorios.add(chave)
            else:
                self.arquivos.add(chave)


# Construções do MkDocs 1.x que a 2.0 não renderiza. A varredura ignora bloco e code span, para que
# a documentação possa citar a sintaxe proibida como exemplo.
_SINTAXE_1X = (
    ("admonition", re.compile(r"^\s*!!!(?:\s|$)")),
    ("aba", re.compile(r'^\s*===\s+"')),
    ("detalhe", re.compile(r"^\s*\?\?\?(?:\s|$)")),
    ("atributo de link", re.compile(r"\]\([^)]*\)\{[^}]*\}")),
    ("grid cards", re.compile(r'<div class="grid cards" markdown>')),
)


class Coletor(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.links: list[str] = []
        self.ids: set[str] = set()
        self.tem_navegacao = False
        self.titulo = ""
        self.refresh = ""
        self.canonical = ""

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        # Um laço só evita o `dict(attrs)` em cada uma das ~260 mil tags do site.
        # O último valor vence, como no `dict` original, e `href` vem antes de `src`.
        identificador = href = src = ""
        for chave, valor in attrs:
            if not valor:
                continue
            if chave == "id":
                identificador = valor
            elif chave == "href":
                href = valor
            elif chave == "src":
                src = valor
        if identificador:
            self.ids.add(identificador)
        if href:
            self.links.append(href)
        if src:
            self.links.append(src)
        if tag == "aside" and "barra" in (dict(attrs).get("class") or ""):
            self.tem_navegacao = True
        if tag == "meta":
            dados = dict(attrs)
            if (dados.get("http-equiv") or "").lower() == "refresh":
                self.refresh = dados.get("content") or ""
        elif tag == "link":
            dados = dict(attrs)
            if (dados.get("rel") or "").lower() == "canonical":
                self.canonical = dados.get("href") or ""

    def handle_data(self, data: str) -> None:
        if not self.titulo and data.strip():
            self.titulo = data.strip()

    def exportar(self) -> tuple[list[str], set[str], bool, str, str, str]:
        """Dados serializáveis para o processo principal (sem estado do parser)."""
        return (
            self.links,
            self.ids,
            self.tem_navegacao,
            self.titulo,
            self.refresh,
            self.canonical,
        )

    @classmethod
    def importar(cls, dados: tuple[list[str], set[str], bool, str, str, str]) -> "Coletor":
        coletor = cls()
        (
            coletor.links,
            coletor.ids,
            coletor.tem_navegacao,
            coletor.titulo,
            coletor.refresh,
            coletor.canonical,
        ) = dados
        return coletor


def _analisar(caminho: Path) -> Coletor:
    coletor = Coletor()
    coletor.feed(_ws().read(caminho))
    return coletor


def _coletar(caminho: str) -> tuple[list[str], set[str], bool, str, str, str]:
    """Analisa uma página em outro processo, sem compartilhar o `Workspace`."""
    coletor = Coletor()
    coletor.feed(Path(caminho).read_text(encoding="utf-8"))
    return coletor.exportar()


def _contexto_processos() -> multiprocessing.context.BaseContext | None:
    """Contexto de cópia sob demanda, ou `None` onde ele não existe.

    O Python 3.15 usa `forkserver` como padrão no Linux; ele reimporta o módulo
    principal e custa mais que o ganho de 480 páginas. O `fork` explícito copia o
    processo já inicializado, sem reimportar nada. No Windows não há `fork`: o parse
    fica sequencial, que é o mesmo resultado, só sem paralelismo. O macOS expõe
    `fork`, então o caminho paralelo vale lá também; o padrão do macOS é `spawn`, mas
    este código escolhe `fork` explicitamente quando ele está disponível.
    """
    if "fork" in multiprocessing.get_all_start_methods():
        return multiprocessing.get_context("fork")
    return None


def _analisar_todas(
    paginas: list[Path], trabalhadores: int | None = None
) -> dict[Path, Coletor]:
    """Analisa as páginas na mesma ordem, em paralelo quando há volume e `fork`."""
    contexto = _contexto_processos()
    if trabalhadores is None:
        trabalhadores = min(8, os.cpu_count() or 1)
        if len(paginas) < 32 or contexto is None:
            trabalhadores = 1
    if trabalhadores <= 1 or contexto is None:
        return {pagina: _analisar(pagina) for pagina in paginas}
    with ProcessPoolExecutor(max_workers=trabalhadores, mp_context=contexto) as pool:
        dados = list(pool.map(_coletar, [str(pagina) for pagina in paginas], chunksize=8))
    return {pagina: Coletor.importar(dado) for pagina, dado in zip(paginas, dados)}


def _chave(caminho: Path | str) -> str:
    return os.path.normcase(str(caminho))


def _existe(caminho: Path | str) -> bool:
    arvore = _ARVORE
    if arvore is not None:
        chave = _chave(caminho)
        if chave in arvore.arquivos or chave in arvore.diretorios:
            return True
    return _ws().exists(Path(caminho))


def _e_diretorio(caminho: Path | str) -> bool:
    arvore = _ARVORE
    if arvore is not None and _chave(caminho) in arvore.diretorios:
        return True
    return _ws().is_dir(Path(caminho))


def _destino_str(pagina: Path, raiz: Path, caminho: str) -> str:
    """Destino absoluto normalizado, como string, sem consultar o disco."""
    if not caminho:
        return str(pagina)
    if caminho.startswith("/"):
        alvo = os.path.normpath(os.path.join(str(raiz), caminho.lstrip("/")))
    else:
        alvo = os.path.normpath(os.path.join(str(pagina.parent), caminho))
    if _e_diretorio(alvo):
        alvo = os.path.join(alvo, "index.html")
    return alvo


def _destino(pagina: Path, raiz: Path, caminho: str) -> Path | None:
    return Path(_destino_str(pagina, raiz, caminho))


def _sem_code_span(linha: str) -> str:
    """Remove `code` da linha: sintaxe citada como exemplo não é uso."""
    return re.sub(r"`[^`]*`", "", linha)


def _checar_sintaxe(raiz: Path) -> tuple[list[str], int]:
    """Varre as fontes publicadas e aponta sintaxe 1.x fora de bloco de código."""
    problemas: list[str] = []
    arquivos = 0
    if not raiz.is_dir():
        return problemas, arquivos
    for sistema in sorted(caminho for caminho in raiz.iterdir() if caminho.is_dir()):
        docs = sistema / "docs"
        if not docs.is_dir():
            continue
        for arquivo in _ws().files(docs, "*.md"):
            arquivos += 1
            em_codigo = False
            for numero, linha in enumerate(_ws().read(arquivo).splitlines(), 1):
                if linha.lstrip().startswith("```"):
                    em_codigo = not em_codigo
                    continue
                if em_codigo:
                    continue
                # A limpeza é por linha, não por padrão: sem backtick nada muda, e
                # a linha é a mesma para os cinco padrões.
                if "`" in linha:
                    linha = _sem_code_span(linha)
                for nome, padrao in _SINTAXE_1X:
                    if padrao.search(linha):
                        relativo = arquivo.relative_to(raiz.parent).as_posix()
                        problemas.append(f"{relativo}:{numero}: sintaxe MkDocs 1.x ({nome})")
    return problemas, arquivos


def _checar_redirects(raiz: Path, ws: Workspace, analisados: dict[Path, Coletor]) -> list[str]:
    """Confere o mapa: origem vira stub, destino existe e nao e outro stub, sem ciclo."""
    problemas: list[str] = []
    redirects = ws.root / "wyd-docs" / "redirects.tsv"
    if not ws.exists(redirects):
        return problemas
    vistos: set[str] = set()
    for numero, linha in enumerate(ws.read(redirects).splitlines(), 1):
        linha = linha.strip()
        if not linha or linha.startswith("#"):
            continue
        campos = [campo.strip() for campo in linha.split("\t")]
        if len(campos) != 2 or not all(campos):
            problemas.append(f"redirects.tsv:{numero}: esperado origem<TAB>destino")
            continue
        origem = campos[0].strip("/").removesuffix(".md")
        destino = campos[1].strip("/").removesuffix(".md")
        if origem in vistos:
            problemas.append(f"redirects.tsv:{numero}: origem duplicada {origem}")
        vistos.add(origem)
        if origem == destino:
            problemas.append(f"redirects.tsv:{numero}: origem igual ao destino {origem}")
            continue
        stub = raiz / origem / "index.html"
        alvo = (raiz / destino / "index.html").resolve()
        if not _existe(stub):
            problemas.append(f"redirects.tsv:{numero}: origem sem pagina-stub {origem}")
            continue
        if not _existe(alvo):
            problemas.append(f"redirects.tsv:{numero}: destino inexistente {destino}")
            continue
        coletor = analisados.get(stub) or _analisar(stub)
        if not coletor.refresh:
            problemas.append(f"redirects.tsv:{numero}: stub sem meta refresh {origem}")
            continue
        url = coletor.refresh.split("url=", 1)[1].strip() if "url=" in coletor.refresh else ""
        resolvido = _destino(stub, raiz, unquote(urlsplit(url).path)) if url else None
        if resolvido is None or resolvido != alvo:
            problemas.append(f"redirects.tsv:{numero}: stub aponta para {url or 'nada'}, esperado {destino}")
        if not coletor.canonical:
            problemas.append(f"redirects.tsv:{numero}: stub sem canonical {origem}")
        elif _destino(stub, raiz, unquote(urlsplit(coletor.canonical).path)) != alvo:
            problemas.append(f"redirects.tsv:{numero}: canonical fora do destino {destino}")
        alvo_coletor = analisados.get(alvo) or _analisar(alvo)
        if alvo_coletor.refresh:
            problemas.append(f"redirects.tsv:{numero}: destino e outro stub {destino}")
    return problemas


def run(ws: Workspace, site: Path | None = None) -> CheckResult:
    global _WS, _ARVORE
    _WS = ws
    raiz = site.resolve() if site is not None else ws.root / "wyd-docs" / "site"
    if not raiz.is_dir():
        return CheckResult("links", False, f"saída não encontrada em {raiz}")
    _ARVORE = _Arvore(raiz)

    paginas = sorted(raiz.rglob("*.html"))
    problemas: list[str] = []
    analisados = _analisar_todas(paginas)

    for pagina in paginas:
        coletor = analisados[pagina]
        relativo = pagina.relative_to(raiz)
        if not coletor.refresh:
            if not coletor.tem_navegacao:
                problemas.append(f"{relativo}: sem navegação lateral")
            if not coletor.titulo:
                problemas.append(f"{relativo}: sem título")

    for pagina, coletor in analisados.items():
        relativo = pagina.relative_to(raiz)
        for link in coletor.links:
            partes = urlsplit(link)
            if partes.scheme or partes.netloc or link.startswith("//"):
                continue
            if partes.path == "" and partes.fragment:
                if partes.fragment not in coletor.ids:
                    problemas.append(f"{relativo}: âncora inexistente #{partes.fragment}")
                continue
            alvo = _destino_str(pagina, raiz, unquote(partes.path))
            if not _existe(alvo):
                problemas.append(f"{relativo}: destino inexistente {partes.path}")
                continue
            if partes.fragment:
                pagina_alvo = Path(alvo)
                if pagina_alvo not in analisados:
                    analisados[pagina_alvo] = _analisar(pagina_alvo)
                if partes.fragment not in analisados[pagina_alvo].ids:
                    problemas.append(f"{relativo}: âncora inexistente {partes.path}#{partes.fragment}")

    sintaxe, fontes = _checar_sintaxe(ws.root / "planejamento-1.0.0")
    problemas.extend(sintaxe)

    problemas.extend(_checar_redirects(raiz, ws, analisados))

    resumo = f"{len(paginas)} páginas, {fontes} fontes"
    if problemas:
        return CheckResult("links", False, resumo, problems=problemas)
    return CheckResult("links", True, resumo)
