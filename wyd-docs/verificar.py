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

import re
import sys
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urlsplit

BASE = Path(__file__).resolve().parent
REDIRECTS = BASE / "redirects.tsv"
FONTES = BASE.parent / "planejamento-1.0.0"

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
        dados = dict(attrs)
        if tag == "meta" and (dados.get("http-equiv") or "").lower() == "refresh":
            self.refresh = dados.get("content") or ""
        if tag == "link" and (dados.get("rel") or "").lower() == "canonical":
            self.canonical = dados.get("href") or ""
        if "id" in dados and dados["id"]:
            self.ids.add(dados["id"])
        classe = dados.get("class") or ""
        if tag == "aside" and "barra" in classe:
            self.tem_navegacao = True
        for chave in ("href", "src"):
            valor = dados.get(chave)
            if valor:
                self.links.append(valor)

    def handle_data(self, data: str) -> None:
        if not self.titulo and data.strip():
            self.titulo = data.strip()


def _analisar(caminho: Path) -> Coletor:
    coletor = Coletor()
    coletor.feed(caminho.read_text(encoding="utf-8"))
    return coletor


def _destino(pagina: Path, raiz: Path, caminho: str) -> Path | None:
    if not caminho:
        return pagina
    if caminho.startswith("/"):
        alvo = (raiz / caminho.lstrip("/")).resolve()
    else:
        alvo = (pagina.parent / caminho).resolve()
    if alvo.is_dir():
        alvo = alvo / "index.html"
    return alvo


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
        for arquivo in sorted(docs.rglob("*.md")):
            arquivos += 1
            em_codigo = False
            for numero, linha in enumerate(arquivo.read_text(encoding="utf-8").splitlines(), 1):
                if linha.lstrip().startswith("```"):
                    em_codigo = not em_codigo
                    continue
                if em_codigo:
                    continue
                for nome, padrao in _SINTAXE_1X:
                    if padrao.search(_sem_code_span(linha)):
                        relativo = arquivo.relative_to(raiz.parent).as_posix()
                        problemas.append(f"{relativo}:{numero}: sintaxe MkDocs 1.x ({nome})")
    return problemas, arquivos


def _checar_redirects(raiz: Path, analisados: dict[Path, Coletor]) -> list[str]:
    """Confere o mapa: origem vira stub, destino existe e nao e outro stub, sem ciclo."""
    problemas: list[str] = []
    if not REDIRECTS.exists():
        return problemas
    vistos: set[str] = set()
    for numero, linha in enumerate(REDIRECTS.read_text(encoding="utf-8").splitlines(), 1):
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
        if not stub.exists():
            problemas.append(f"redirects.tsv:{numero}: origem sem pagina-stub {origem}")
            continue
        if not alvo.exists():
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


def main(argv: list[str]) -> int:
    raiz = Path(argv[1]).resolve() if len(argv) > 1 else BASE / "site"
    if not raiz.is_dir():
        print(f"verificação: saída não encontrada em {raiz}", file=sys.stderr)
        return 2

    paginas = sorted(raiz.rglob("*.html"))
    problemas: list[str] = []
    analisados: dict[Path, Coletor] = {}

    for pagina in paginas:
        coletor = _analisar(pagina)
        analisados[pagina] = coletor
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
            alvo = _destino(pagina, raiz, unquote(partes.path))
            if alvo is None:
                continue
            if not alvo.exists():
                problemas.append(f"{relativo}: destino inexistente {partes.path}")
                continue
            if partes.fragment:
                if alvo not in analisados:
                    analisados[alvo] = _analisar(alvo)
                if partes.fragment not in analisados[alvo].ids:
                    problemas.append(f"{relativo}: âncora inexistente {partes.path}#{partes.fragment}")

    sintaxe, fontes = _checar_sintaxe(FONTES)
    problemas.extend(sintaxe)

    problemas.extend(_checar_redirects(raiz, analisados))

    if problemas:
        print(f"verificação: {len(problemas)} problema(s) em {len(paginas)} página(s)")
        for problema in problemas:
            print(f" - {problema}")
        return 1

    print(
        f"verificação: {len(paginas)} páginas, links, âncoras, navegação e sintaxe ok "
        f"({fontes} fontes verificadas)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
