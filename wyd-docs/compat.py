"""Converte a sintaxe herdada do MkDocs 1.x/Material para o Markdown do MkDocs 2.0.

O MkDocs 2.0 (2.0.dev6) habilita apenas `fenced_code`, `footnotes`, `tables`, `toc` e as
extensões internas de link, shortcode e riscado. Não existe plugin, `admonition`,
`pymdownx.tabbed`, `attr_list` nem `md_in_html` — e a configuração de extensões não é
exposta. Esta camada traduz o que as páginas do planejamento já usam para construções
que o GFM do MkDocs 2.0 renderiza (blockquote, título, lista), sem alterar o arquivo
de origem. É uma ponte declarada, não um recurso do site.
"""

from __future__ import annotations

import re

ICONES = {
    "database": "🗄️",
    "book": "📘",
    "books": "📚",
    "code": "💻",
    "console": "⌨️",
    "check": "✅",
    "close": "❌",
    "warning": "⚠️",
    "alert": "🚨",
    "rocket": "🚀",
    "fire": "🔥",
    "bug": "🐛",
    "magnify": "🔍",
    "link": "🔗",
    "clock": "⏱️",
    "map": "🗺️",
    "test-tube": "🧪",
    "account": "👤",
    "shield": "🛡️",
    "tools": "🧰",
}

TIPOS_DE_AVISO = {
    "abstract",
    "bug",
    "caution",
    "danger",
    "example",
    "failure",
    "info",
    "note",
    "question",
    "quote",
    "success",
    "tip",
    "warning",
}

_ABRE_CARTOES = re.compile(r'^\s*<div class="grid cards" markdown>\s*$')
_FECHA_CARTOES = re.compile(r"^\s*</div>\s*$")
_AVISO = re.compile(r'^!!! +([a-z]+)(?: +"(.*)")?\s*$')
_DETALHE = re.compile(r'^\?\?\? +([a-z]+)(?: +"(.*)")?\s*$')
_ABA = re.compile(r'^=== +"(.*)"\s*$')
_TAREFA = re.compile(r"^(\s*)- \[([ xX])\] +")
_ATRIBUTO_DE_LINK = re.compile(r"\]\(([^)]+)\)\{[^}]*\}")
_ICONE = re.compile(r":material-([a-z0-9-]+):")


def _corpo_indentado(linhas: list[str], inicio: int) -> tuple[list[str], int]:
    """Lê o bloco indentado por quatro espaços a partir de `inicio`."""
    corpo: list[str] = []
    indice = inicio
    while indice < len(linhas):
        linha = linhas[indice]
        if not linha.strip():
            corpo.append("")
            indice += 1
            continue
        if linha.startswith("    "):
            corpo.append(linha[4:])
            indice += 1
            continue
        break
    while corpo and not corpo[-1].strip():
        corpo.pop()
    return corpo, indice


def _bloco_de_aviso(tipo: str, titulo: str, corpo: list[str]) -> list[str]:
    cabecalho = f"> [!{tipo.upper()}]"
    if titulo:
        cabecalho = f"{cabecalho} {titulo}"
    saida = [cabecalho]
    for linha in corpo:
        saida.append(">" if not linha.strip() else f"> {linha}")
    return saida


def converter(texto: str) -> str:
    """Traduz a sintaxe herdada preservando o conteúdo e a ordem das seções."""
    linhas = texto.splitlines()
    saida: list[str] = []
    dentro_de_codigo = False
    dentro_de_cartoes = False
    indice = 0

    while indice < len(linhas):
        linha = linhas[indice]

        if linha.lstrip().startswith("```"):
            dentro_de_codigo = not dentro_de_codigo
            saida.append(linha)
            indice += 1
            continue
        if dentro_de_codigo:
            saida.append(linha)
            indice += 1
            continue

        if _ABRE_CARTOES.match(linha):
            dentro_de_cartoes = True
            saida.append('<div class="cartoes"></div>')
            indice += 1
            continue

        if dentro_de_cartoes:
            if _FECHA_CARTOES.match(linha):
                dentro_de_cartoes = False
                indice += 1
                continue
            saida.append(linha[4:] if linha.startswith("    ") else linha)
            indice += 1
            continue

        aviso = _AVISO.match(linha)
        detalhe = _DETALHE.match(linha)
        if aviso or detalhe:
            casamento = aviso or detalhe
            tipo = casamento.group(1).lower()
            titulo = (casamento.group(2) or "").strip()
            corpo, indice = _corpo_indentado(linhas, indice + 1)
            if tipo in TIPOS_DE_AVISO:
                saida.extend(_bloco_de_aviso(tipo, titulo, corpo))
            else:
                saida.extend(_bloco_de_aviso("note", titulo, corpo))
            continue

        aba = _ABA.match(linha)
        if aba:
            corpo, indice = _corpo_indentado(linhas, indice + 1)
            saida.append(f"#### {aba.group(1).strip()}")
            saida.append("")
            saida.extend(corpo)
            continue

        linha = _TAREFA.sub(lambda m: f"{m.group(1)}- {'☑' if m.group(2) in 'xX' else '☐'} ", linha)
        linha = _ATRIBUTO_DE_LINK.sub(r"](\1)", linha)
        linha = _ICONE.sub(lambda m: ICONES.get(m.group(1), ""), linha)
        saida.append(linha)
        indice += 1

    return "\n".join(saida) + ("\n" if texto.endswith("\n") else "")
