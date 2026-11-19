"""Monta a árvore de fontes do site e a navegação para o MkDocs 2.0.

O MkDocs 2.0 não tem plugin nem hook: só existe um diretório de entrada. Este passo copia
as páginas de cada sistema (`planejamento-1.0.0/<sistema>/docs/`) para
`.build/docs/sistemas/<sistema>/`, aplica a ponte de sintaxe de `compat.py`, copia o tema
e gera `templates/navegacao.html` a partir da árvore real. Nenhuma página é editada no
sistema dono.
"""

from __future__ import annotations

import html
import shutil
from pathlib import Path

import compat

BASE = Path(__file__).resolve().parent
REPO = BASE.parent
PLANEJAMENTO = REPO / "planejamento-1.0.0"
MONTAGEM = BASE / ".build" / "docs"
TEMA = BASE / "tema"

SISTEMAS = [
    "wyd-protocol",
    "wyd-net",
    "wyd-core",
    "wyd-server",
    "wyd-sim",
    "wyd-db",
    "wyd-admin",
    "wyd-portal",
    "wyd-launcher",
    "wyd-client",
    "wyd-bots",
    "wyd-docs",
    "wyd-ops",
]

ROTULOS = {
    "arquitetura": "Arquitetura",
    "specs": "Requisitos e especificações",
    "wire": "Fio",
    "payloads": "Payloads",
    "goldens": "Goldens",
    "dicionario-dados": "Dicionário de dados",
    "migrations": "Migrations",
    "src": "Código",
    "tests": "Testes",
    "planos": "Planos",
    "roadmap": "Roadmap",
    "guia-desenvolvedor": "Guia do desenvolvedor",
    "guia-administrador": "Guia do administrador",
    "guia-teste": "Guia de teste",
    "guia-documentacao": "Guia de documentação",
    "guia-autor": "Guia do autor",
    "guia-operador": "Guia do operador",
    "adr": "Decisões (ADR)",
    "fluxos": "Fluxos",
    "estados": "Estados",
    "opcodes": "Opcodes",
    "alvos": "Alvos",
    "golden": "Goldens",
}


def _bonito(nome: str) -> str:
    return nome.replace("-", " ").replace("_", " ").strip().capitalize()


def _titulo(caminho: Path, padrao: str) -> str:
    try:
        for linha in caminho.read_text(encoding="utf-8").splitlines():
            if linha.startswith("# "):
                return linha[2:].strip()
    except OSError:
        pass
    return padrao


def _url(rel: str) -> str:
    """Reproduz a URL que o MkDocs 2.0 gera para o caminho relativo."""
    partes = rel.split("/")
    nome = partes[-1]
    if nome.lower() in ("index.md", "readme.md"):
        pasta = "/".join(partes[:-1])
        return f"/{pasta}/" if pasta else "/"
    return "/" + "/".join(partes[:-1] + [Path(nome).stem.lower()]) + "/"


def _pagina(destino: Path, rel: str, titulo: str) -> dict[str, str]:
    return {"rel": rel, "url": _url(rel), "titulo": titulo, "destino": str(destino)}


def _copiar_paginas(origem: Path, destino: Path, prefixo: str) -> list[dict[str, str]]:
    paginas: list[dict[str, str]] = []
    for arquivo in sorted(origem.rglob("*.md")):
        rel = arquivo.relative_to(origem).as_posix()
        alvo = destino / rel
        alvo.parent.mkdir(parents=True, exist_ok=True)
        alvo.write_text(compat.converter(arquivo.read_text(encoding="utf-8")), encoding="utf-8")
        padrao = "Visão geral" if Path(rel).name == "index.md" else _bonito(Path(rel).stem)
        paginas.append(_pagina(alvo, f"{prefixo}/{rel}" if prefixo else rel, _titulo(arquivo, padrao)))
    return paginas


def _arvore(paginas: list[dict[str, str]]) -> dict:
    raiz: dict = {}
    for pagina in paginas:
        partes = pagina["rel"].split("/")
        no = raiz
        for parte in partes[:-1]:
            no = no.setdefault(parte, {})
        no[partes[-1]] = pagina
    return raiz


def _render(no: dict, raiz: bool = False) -> str:
    paginas = [(chave, valor) for chave, valor in no.items() if "url" in valor]
    pastas = [(chave, valor) for chave, valor in no.items() if "url" not in valor]
    paginas.sort(key=lambda par: (par[0] != "index.md", par[0].lower()))

    itens: list[str] = []
    for nome, pagina in paginas:
        titulo = "Visão geral" if raiz and nome == "index.md" else pagina["titulo"]
        itens.append(f'<li><a href="{{{{ \'{pagina["url"]}\' | url }}}}">{html.escape(titulo)}</a></li>')
    for nome, sub in pastas:
        rotulo = html.escape(ROTULOS.get(nome, _bonito(nome)))
        itens.append(
            f'<li class="grupo"><span>{rotulo}</span><ul>{_render(sub)}</ul></li>'
        )
    return "".join(itens)


def _navegacao(proprias: list[dict[str, str]], sistemas: dict[str, list[dict[str, str]]]) -> str:
    inicio = "<li><a href=\"{{ '/' | url }}\">Início</a></li>"
    contribuir = "".join(
        f'<li><a href="{{{{ \'{pagina["url"]}\' | url }}}}">{html.escape(pagina["titulo"])}</a></li>'
        for pagina in proprias
        if pagina["rel"].startswith("contribuir/")
    )
    grupos = []
    for sistema, paginas in sistemas.items():
        if not paginas:
            continue
        grupos.append(
            f'<li class="grupo"><span>{sistema}</span><ul>{_render(_arvore(paginas), raiz=True)}</ul></li>'
        )
    secao = f'<li class="grupo"><span>Sistemas</span><ul>{"".join(grupos)}</ul></li>' if grupos else ""
    return f'<ul class="lista">{inicio}{secao}{contribuir}</ul>'


def main() -> int:
    if MONTAGEM.exists():
        shutil.rmtree(MONTAGEM)
    MONTAGEM.mkdir(parents=True)

    proprias = _copiar_paginas(BASE / "docs", MONTAGEM, "")
    sistemas: dict[str, list[dict[str, str]]] = {}
    for sistema in SISTEMAS:
        origem = PLANEJAMENTO / sistema / "docs"
        if not origem.is_dir():
            sistemas[sistema] = []
            continue
        sistemas[sistema] = _copiar_paginas(origem, MONTAGEM / "sistemas" / sistema, f"sistemas/{sistema}")

    templates = MONTAGEM / "templates"
    templates.mkdir(exist_ok=True)
    shutil.copyfile(TEMA / "base.html", templates / "base.html")
    (templates / "navegacao.html").write_text(
        _navegacao(proprias, sistemas), encoding="utf-8"
    )

    assets = MONTAGEM / "assets"
    assets.mkdir(exist_ok=True)
    for nome in ("estilo.css", "site.js", "mermaid.js"):
        shutil.copyfile(TEMA / nome, assets / nome)

    total_paginas = len(proprias) + sum(len(paginas) for paginas in sistemas.values())
    total_sistemas = sum(1 for paginas in sistemas.values() if paginas)
    print(f"montagem: {total_paginas} páginas de {total_sistemas} sistemas em {MONTAGEM}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
