"""Verifica o site gerado: link interno, âncora, tema e navegação.

O MkDocs 2.0 não tem modo estrito nem validação de link — link relativo para página
inexistente é copiado como está e vira 404 em produção. Este script é o gate do site:
varre `site/**/*.html`, resolve cada `href`/`src` local e falha quando o destino, a
âncora ou a navegação não existem.
"""

from __future__ import annotations

import sys
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urlsplit

BASE = Path(__file__).resolve().parent


class Coletor(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.links: list[str] = []
        self.ids: set[str] = set()
        self.tem_navegacao = False
        self.titulo = ""

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        dados = dict(attrs)
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

    if problemas:
        print(f"verificação: {len(problemas)} problema(s) em {len(paginas)} página(s)")
        for problema in problemas:
            print(f" - {problema}")
        return 1

    print(f"verificação: {len(paginas)} páginas, links, âncoras e navegação ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
