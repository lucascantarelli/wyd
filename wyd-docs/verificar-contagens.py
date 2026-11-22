#!/usr/bin/env python3
"""Gate de contagens da série 1.0.0: mede dos arquivos e confere o que os dashboards citam.

Medidas (fontes independentes):

1. soma da coluna `Tab.` de `migrations-checklist.md` §2;
2. arquivos da série: linhas de §2 e nomes `NNNN_*.sql` distintos em `plano-migrations.md`;
3. tabelas: `CREATE TABLE` em `ddl-confirmado.md`;
4. páginas do site: todo `.md` de `wyd-docs/docs/` e de `planejamento-1.0.0/<sistema>/docs/`
   (a mesma regra de `preparar.py`).
5. censo por estado: `aceito`, `proposto` e sem linha de estado nas fontes de sistema que o site
   publica (`planejamento-1.0.0/<sistema>/docs/**/*.md`), com o total de fontes censadas.

Regra das citações: **todo número `N tabelas` / `N arquivos` é da série** até ser classificado como de
outro dono em `OUTROS_DONOS`. A lista é curta, cada entrada nomeia o dono, e o que ela exclui é impresso
como `ignorado` — nenhuma citação some sem rastro. É o inverso de uma lista de marcadores: número novo
sem marcador **reprova** em vez de passar calado, e número alheio só passa porque foi classificado.

`N páginas` só conta quando a frase é do gate do site (`site passa com` / `passou com`): a mesma linha
pode citar páginas do dicionário, e medida histórica (“34 páginas montadas”) não é contagem atual.

O censo por estado é lido de `wyd-docs-1.0.0.md` §9.1 — o dono único do número — e reprova quando
qualquer uma das quatro grandezas (`aceito`, `proposto`, sem linha, fontes) diverge da montagem. Fonte
cujo estado não é editorial é exceção declarada em `CENSO_ESPECIAL`, impressa com o motivo; estado
fora de `aceito`/`proposto` sem exceção reprova.

Falha (exit 1) em divergência; exit 2 quando não dá para medir (arquivo ou pasta ausente).
Vive em `wyd-docs/` porque `planejamento-1.0.0/` é documentação e não aceita executável (AGENTS §5.6).

Uso:
    python verificar-contagens.py            # a partir de wyd-docs/
    python verificar-contagens.py --root .   # raiz do repositório explícita
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path
from typing import NamedTuple

DASHBOARDS = (
    "planejamento-1.0.0/wyd-db/wyd-db-1.0.0.md",
    "planejamento-1.0.0/wyd-portal/wyd-portal-1.0.0.md",
    "planejamento-1.0.0/wyd-docs/wyd-docs-1.0.0.md",
)
CHECKLIST = "planejamento-1.0.0/wyd-db/migrations-checklist.md"
DDL = "planejamento-1.0.0/wyd-db/ddl-confirmado.md"
PLAN = "planejamento-1.0.0/wyd-db/plano-migrations.md"
CENSO = "planejamento-1.0.0/wyd-docs/wyd-docs-1.0.0.md"

CLAIM_RE = re.compile(r"([0-9]+)[ ]+(tabelas|arquivos)")
SITE_PAGE_RE = re.compile(
    r"(?:site passa com|passou com)[ ]+\*{0,2}[ ]*([0-9]+)[ ]*\*{0,2}[ ]*páginas"
)
PLAN_FILE_RE = re.compile(r"[0-9]{4}_[a-z0-9_]+[.]sql")
ESTADO_RE = re.compile(r"^[\s>]*\*\*Estado:\*\*[ ]*([^\s(]+)", re.IGNORECASE)
CENSO_FONTES_RE = re.compile(r"censo das ([0-9]+) fontes")

# Grandezas do censo de §9.1 e como cada uma é declarada na seção. O nome é a chave de `medido`.
CENSO_ESTADOS = (
    ("aceito", re.compile(r"([0-9]+)[ ]*`aceito`")),
    ("proposto", re.compile(r"([0-9]+)[ ]*`proposto`")),
    ("sem linha", re.compile(r"([0-9]+)[ ]+sem linha de estado")),
)

# Fonte publicada cujo estado não é editorial: entra no total de fontes, não no censo. Cada entrada
# nomeia o motivo, e o arquivo é impresso — nada some sem rastro.
CENSO_ESPECIAL = {
    "planejamento-1.0.0/wyd-db/docs/dicionario-dados/gerado/index.md":
        "índice de navegação: o estado é a contagem de páginas escritas",
}

# Citações que NÃO são da série. Cada padrão isola o número com a unidade e o dono real, para não
# esconder outra citação da mesma linha. Tabela de exceções, não de marcadores: cresce só com prova.
OUTROS_DONOS = (
    (re.compile(r"[0-9]+[ ]+arquivos[ ]+Rust"), "crate Rust"),
    (re.compile(r"[0-9]+[ ]+tabelas[^|]*banco-catalogo"), "catálogo medido (`banco-catalogo.md`)"),
    (re.compile(r"[0-9]+[ ]+arquivos em `?src/(?:migrate|provision)/"), "arquivos de módulo do crate"),
    (re.compile(r"[0-9]+[ ]+arquivos[^|]*[.]gitkeep"), "inventário da pasta do sistema"),
    (re.compile(r"[0-9]+[ ]+arquivos[^|]*fora de `docs/`"), "arquivos do projeto do site (2026-11-19)"),
)


class Citacao(NamedTuple):
    arquivo: str
    linha: int
    valor: int
    unidade: str


class Erro(Exception):
    """Falha de leitura que impede medir — não é divergência de contagem."""


def ler(raiz: Path, relativo: str) -> str:
    caminho = raiz / relativo
    if not caminho.is_file():
        raise Erro(f"arquivo ausente: {caminho}")
    return caminho.read_text(encoding="utf-8")


def medir_checklist(raiz: Path) -> tuple[int, int]:
    """(arquivos, soma de `Tab.`) da tabela de §2, lida célula a célula."""
    arquivos = soma = 0
    for linha in ler(raiz, CHECKLIST).splitlines():
        celulas = [celula.strip() for celula in linha.strip().strip("|").split("|")]
        if len(celulas) < 3:
            continue
        numero, pagina, tabelas = celulas[:3]
        if not numero.isdigit() or not pagina.startswith("`") or ".md`" not in pagina:
            continue
        # `—` na coluna `Tab.` = arquivo sem tabela (seed bloqueado, view): conta como arquivo.
        if tabelas != "—" and not tabelas.isdigit():
            continue
        arquivos += 1
        if tabelas.isdigit():
            soma += int(tabelas)
    if not arquivos:
        raise Erro(f"{CHECKLIST}: nenhuma linha de arquivo em §2")
    return arquivos, soma


def medir_ddl(raiz: Path) -> int:
    return sum(1 for linha in ler(raiz, DDL).splitlines() if linha.startswith("CREATE TABLE "))


def medir_plano(raiz: Path) -> int:
    return len(set(PLAN_FILE_RE.findall(ler(raiz, PLAN))))


def medir_paginas_site(raiz: Path) -> int:
    bases = [raiz / "wyd-docs" / "docs"]
    bases += sorted((raiz / "planejamento-1.0.0").glob("*/docs"))
    for base in bases:
        if not base.is_dir():
            raise Erro(f"pasta de páginas ausente: {base}")
    return sum(1 for base in bases for _ in base.rglob("*.md"))


def _estado(arquivo: Path) -> str | None:
    """Valor da linha `**Estado:**`, em minúsculas, ou `None` quando a página não tem a linha."""
    for linha in arquivo.read_text(encoding="utf-8").splitlines():
        achado = ESTADO_RE.match(linha)
        if achado:
            return achado.group(1).lower()
    return None


def medir_censo(raiz: Path) -> tuple[dict[str, int], list[tuple[str, str]]]:
    """Censo por estado das fontes de sistema publicadas, mais as exceções declaradas.

    A pasta é a mesma que `preparar.py` monta em `sistemas/<sistema>/`; as duas páginas próprias do
    site (`wyd-docs/docs/`) ficam fora, como declara `wyd-docs-1.0.0.md` §9.1.
    """
    bases = sorted((raiz / "planejamento-1.0.0").glob("*/docs"))
    if not bases:
        raise Erro("nenhuma fonte em `planejamento-1.0.0/*/docs`")
    contagem = {"aceito": 0, "proposto": 0, "sem linha": 0, "fontes": 0}
    especiais: list[tuple[str, str]] = []
    for base in bases:
        for arquivo in sorted(base.rglob("*.md")):
            relativo = arquivo.relative_to(raiz).as_posix()
            estado = _estado(arquivo)
            contagem["fontes"] += 1
            if relativo in CENSO_ESPECIAL:
                if estado in ("aceito", "proposto"):
                    raise Erro(f"{relativo}: exceção de censo com estado editorial `{estado}`")
                especiais.append((relativo, CENSO_ESPECIAL[relativo]))
            elif estado is None:
                contagem["sem linha"] += 1
            elif estado in ("aceito", "proposto"):
                contagem[estado] += 1
            else:
                raise Erro(
                    f"{relativo}: estado `{estado}` fora de `aceito`/`proposto` e sem exceção em "
                    "CENSO_ESPECIAL"
                )
    return contagem, especiais


def _linhas_de_secao(raiz: Path, titulo: str) -> list[tuple[int, str]]:
    """Linhas (número, texto) da seção cujo cabeçalho começa por `titulo`, até a próxima heading."""
    dentro = False
    linhas: list[tuple[int, str]] = []
    for numero, linha in enumerate(ler(raiz, CENSO).splitlines(), start=1):
        if linha.startswith("#"):
            if dentro:
                break
            dentro = linha.startswith(titulo)
            continue
        if dentro:
            linhas.append((numero, linha))
    if not linhas:
        raise Erro(f"{CENSO}: seção `{titulo}` sem corpo")
    return linhas


def censo_declarado(raiz: Path) -> list[Citacao]:
    """As quatro grandezas declaradas em `wyd-docs-1.0.0.md` §9.1."""
    linhas = _linhas_de_secao(raiz, "### 9.1")
    declaradas: list[Citacao] = []
    for unidade, padrao in CENSO_ESTADOS + (("fontes", CENSO_FONTES_RE),):
        for numero, linha in linhas:
            for achado in padrao.finditer(linha):
                declaradas.append(Citacao(CENSO, numero, int(achado.group(1)), unidade))
    faltando = {unidade for unidade, _ in CENSO_ESTADOS} | {"fontes"}
    faltando -= {declarada.unidade for declarada in declaradas}
    if faltando:
        raise Erro(f"{CENSO} §9.1: censo sem declaração de {', '.join(sorted(faltando))}")
    return declaradas


def _dono_alheio(linha: str, inicio: int, fim: int) -> str | None:
    """Dono declarado quando uma exceção cobre exatamente este número."""
    for padrao, dono in OUTROS_DONOS:
        for excecao in padrao.finditer(linha):
            if excecao.start() <= inicio and fim <= excecao.end():
                return dono
    return None


def citacoes(raiz: Path) -> tuple[list[Citacao], list[tuple[Citacao, str]]]:
    """Citações da série e citações ignoradas (com o dono declarado)."""
    da_serie: list[Citacao] = []
    ignoradas: list[tuple[Citacao, str]] = []
    for relativo in DASHBOARDS:
        for numero, linha in enumerate(ler(raiz, relativo).splitlines(), start=1):
            for achado in CLAIM_RE.finditer(linha):
                citacao = Citacao(relativo, numero, int(achado.group(1)), achado.group(2))
                dono = _dono_alheio(linha, achado.start(), achado.end())
                if dono:
                    ignoradas.append((citacao, dono))
                else:
                    da_serie.append(citacao)
            for achado in SITE_PAGE_RE.finditer(linha):
                da_serie.append(Citacao(relativo, numero, int(achado.group(1)), "páginas"))
    return da_serie, ignoradas


def main() -> int:
    parser = argparse.ArgumentParser(description="Confere as contagens da série 1.0.0.")
    parser.add_argument(
        "--root",
        default=str(Path(__file__).resolve().parent.parent),
        help="raiz do repositório (default: pasta acima deste script)",
    )
    raiz = Path(parser.parse_args().root).resolve()

    try:
        arquivos, soma_tab = medir_checklist(raiz)
        tabelas_ddl = medir_ddl(raiz)
        arquivos_plano = medir_plano(raiz)
        paginas = medir_paginas_site(raiz)
        censo, especiais = medir_censo(raiz)
        da_serie, ignoradas = citacoes(raiz)
        da_serie += censo_declarado(raiz)
    except Erro as erro:
        print(f"contagens: {erro}", file=sys.stderr)
        return 2

    problemas: list[str] = []
    if soma_tab != tabelas_ddl:
        problemas.append(
            f"`Tab.` soma {soma_tab}, mas `ddl-confirmado.md` tem {tabelas_ddl} `CREATE TABLE`"
        )
    if arquivos != arquivos_plano:
        problemas.append(
            f"§2 lista {arquivos} arquivos, mas `plano-migrations.md` cita {arquivos_plano}"
        )

    medido = {"tabelas": soma_tab, "arquivos": arquivos, "páginas": paginas, **censo}
    for citacao in da_serie:
        esperado = medido[citacao.unidade]
        if citacao.valor != esperado:
            problemas.append(
                f"{citacao.arquivo}:{citacao.linha}: cita {citacao.valor} "
                f"{citacao.unidade}, medido {esperado}"
            )

    print(
        f"contagens: {arquivos} arquivos, `Tab.` {soma_tab}, DDL {tabelas_ddl}, "
        f"plano {arquivos_plano}, site {paginas} páginas; "
        f"censo {censo['fontes']} fontes = {censo['aceito']} aceito + {censo['proposto']} proposto + "
        f"{censo['sem linha']} sem linha + {len(especiais)} fora do censo; "
        f"{len(da_serie)} citação(ões) da série, {len(ignoradas)} de outro dono"
    )
    for relativo, motivo in especiais:
        print(f"  fora do censo {relativo} ({motivo})")
    for citacao, dono in ignoradas:
        print(f"  ignorado {citacao.arquivo}:{citacao.linha}: {citacao.valor} {citacao.unidade} ({dono})")
    for citacao in da_serie:
        print(f"  ok {citacao.arquivo}:{citacao.linha}: {citacao.valor} {citacao.unidade}")

    if problemas:
        for problema in problemas:
            print(f"  divergência: {problema}")
        print(f"contagens: {len(problemas)} problema(s)")
        return 1
    print("contagens: medidas e citações coincidem")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
