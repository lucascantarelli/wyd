#!/usr/bin/env python3
"""Gate de contagens da série 1.0.0: mede dos arquivos e confere o que os dashboards citam.

Medidas (fontes independentes):

1. soma da coluna `Tab.` de `migrations-checklist.md` §2;
2. arquivos da série: linhas de §2 e nomes `NNNN-*.sql` distintos em `plano-migrations.md`;
3. tabelas: `CREATE TABLE` em `ddl-confirmado.md`;
4. páginas do site: todo `.md` de `wyd-docs/docs/` e de `planejamento-1.0.0/<sistema>/docs/`
   (a mesma regra de `preparar.py`).
5. censo por estado: `aceito`, `proposto` e sem linha de estado nas fontes de sistema que o site
   publica (`planejamento-1.0.0/<sistema>/docs/**/*.md`), com o total de fontes censadas.
6. **páginas de issue** — arquivos `WYD*.md` de cada `docs/roadmap/`, sem `roadmap.md`, `fases.md`
   nem `index.md`; `status-planejamento.md` cita o total, um dashboard cita o do próprio sistema.
7. **fatias** — no dashboard, as páginas de issue do sistema; o `wyd-admin` não tem página por fatia e
   a medida é o número de IDs `ADM-nn` do `roadmap.md` dele (exceção declarada em `FATIAS_ESPECIAIS`).
8. **páginas escritas** e **páginas no `wyd-db`** — páginas `.md` de `<sistema>/docs/`; a frase nomeia
   o sistema em `código` e a medida é a daquele sistema.
9. **fontes** — o total do censo, conferido na citação (`N páginas / N fontes`).
10. **páginas em `docs/` e por pasta** — `N páginas em `docs/`` confere com as páginas do sistema; cada
    item `**N na raiz**` / `**N em `<pasta>/`**` confere com a contagem recursiva daquela pasta. O total
    e a soma das pastas precisam fechar; pasta que não existe no sistema é ignorada.
11. **estrutura de pastas** — `(N sistemas)` confere com os `*/src/.gitkeep`; `docs/modulos.md em N
    sistemas` com os arquivos `docs/modulos.md` existentes.
12. **páginas de módulo** — são planejadas: a frase que cita `N páginas de módulo` precisa do marcador
    `planejadas`/`planejado`, senão reprova (o repositório não tem `docs/src/`).
13. **sistemas restantes** — `os N restantes` numa linha que fala de `docs/modulos.md` é
    (sistemas com dashboard) − (arquivos `docs/modulos.md`): o denominador é a lista de dashboards,
    a mesma que `_dashboards()` usa.
14. **planos com CI** — `todos os N planos declaram` confere com os `*/docs/planos/plano-de-testes.md`
    que citam CI/`integração contínua`/`github actions`, `cobertura` e `monitoramento`/`monitorar`.
15. **planos de teste por sistema** — `` `plano-de-testes.md` (nos|de) N sistemas`` confere com os
    arquivos `*/docs/planos/plano-de-testes.md` existentes (13).
16. **série WYDDB e fatias** — `N (páginas )?`WYDDB-nnn`` confere com as páginas de issue do `wyd-db`
    (40); `N por fatia` com o total de páginas de issue menos as do `wyd-db`; `nos N sistemas` numa
    linha com `por fatia` com os sistemas que têm página de fatia (11, sem `wyd-db` e sem `wyd-admin`).
17. **páginas de domínio e de migration** — `N páginas de domínio` confere com
    `docs/dicionario-dados/dominio-*.md`; `N páginas `NNNN-`` exige o marcador `planejadas`, porque
    `docs/migrations/` não existe no repositório.
18. **inventário de páginas/arquivos** — `alvos/*.md (N páginas)` confere com os arquivos de
    `<sistema>/tests/alvos/` e `specs/*.md (N páginas)` com os de `<sistema>/tests/specs/`; `N páginas,
    uma por arquivo|alvo` e `N arquivos em `src/migrate/`` exigem `planejadas`/`previsto`, porque essas
    pastas ainda não existem (`N páginas, uma por alvo` na linha de `alvos/*.md` é medida, não planejada). Linha de árvore (com `▲`) fica fora.

Percentual de alcançabilidade não é publicado: `wyd-docs/verificar.py` confere link, âncora e
navegação lateral de cada página, mas não calcula a razão alcançáveis/total. Enquanto não houver essa
linha no gate, documento nenhum afirma percentual — e o status não repete “0 órfãos”.

Número que não é medível localmente — estado do board no GitHub — não é publicado como contagem: a
frase nomeia a fonte e a data da leitura e diz que o gate não a mede. Data entre parênteses na linha
(`(2026-11-22)`) marca registro histórico e a linha não reprova.

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

SISTEMAS = "planejamento-1.0.0"
STATUS = "planejamento-1.0.0/status-planejamento.md"
CHECKLIST = "planejamento-1.0.0/wyd-db/migrations-checklist.md"
DDL = "planejamento-1.0.0/wyd-db/ddl-confirmado.md"
PLAN = "planejamento-1.0.0/wyd-db/plano-migrations.md"
CENSO = "planejamento-1.0.0/wyd-docs/wyd-docs-1.0.0.md"

CLAIM_RE = re.compile(r"([0-9]+)[ ]+(tabelas|arquivos)")
SITE_PAGE_RE = re.compile(
    r"(?:site passa com|passou com)[ ]+\*{0,2}[ ]*([0-9]+)[ ]*\*{0,2}[ ]*páginas"
)
PLAN_FILE_RE = re.compile(r"[0-9]{4}[-_][a-z0-9_-]+[.]sql")
ESTADO_RE = re.compile(r"^[\s>]*\*\*Estado:\*\*[ ]*([^\s(]+)", re.IGNORECASE)
CENSO_FONTES_RE = re.compile(r"censo das ([0-9]+) fontes")
ISSUE_PAGE_RE = re.compile(r"([0-9]+)\s+p\u00E1ginas de issue")
PAGINAS_FONTES_RE = re.compile(r"([0-9]+)\s+p\u00E1ginas\s*/\s*([0-9]+)\s+fontes")
FATIAS_RE = re.compile(r"([0-9]+)\s+fatias")
PAGINAS_ESCRITAS_RE = re.compile(r"([0-9]+)\s+p\u00E1ginas escritas")
PAGINAS_DB_RE = re.compile(r"([0-9]+)\s+p\u00E1ginas no `wyd-db`")
PAGINAS_CORPO_RE = re.compile(r"([0-9]+)\s+páginas com corpo em `docs/`")
SISTEMAS_GITKEEP_RE = re.compile(r"\(([0-9]+) sistemas\)")
COMPONENTES_RE = re.compile(r"componentes.md`? em ([0-9]+) sistemas")
INV_ARQUIVO_RE = re.compile(r"[0-9]+ páginas(?: planejadas)?, uma por (?:arquivo|alvo)")
INV_ALVOS_RE = re.compile(r"alvos/[*][.]md`[*][*] [(]([0-9]+) páginas")
INV_SPECS_RE = re.compile(r"specs/[*][.]md`[*][*] [(]([0-9]+) páginas")
INV_SRC_RE = re.compile(r"[0-9]+ arquivos em `src/migrate/`")
PREVISTO_RE = re.compile(r"previst|planejad", re.IGNORECASE)
DOMINIO_RE = re.compile(r"([0-9]+) páginas (?:de|por) domínio")
MIGRACAO_PAG_RE = re.compile(r"[0-9]+ páginas `NNNN[-_]")
PLANO_SISTEMAS_RE = re.compile(r"`plano-de-testes.md` (?:nos|de) ([0-9]+) sistemas")
WYDDB_PAGINAS_RE = re.compile(r"([0-9]+) (?:páginas )?`WYDDB-nnn`")
FATIA_SPLIT_RE = re.compile(r"([0-9]+) por fatia")
FATIA_SISTEMAS_RE = re.compile(r"nos ([0-9]+) sistemas")
MODULOS_RE = re.compile(r"`docs/modulos\.md` em ([0-9]+) sistemas")
PAGINAS_DOCS_RE = re.compile(r"([0-9]+) p\u00E1ginas em `docs/`")
FOLDER_RE = re.compile(r"\*\*([0-9]+) (?:na raiz|em `([a-z0-9-]+)/`)\*\*")
PLANEJADO_RE = re.compile(r"planejad", re.IGNORECASE)
RESTANTES_RE = re.compile(r"os ([0-9]+) restantes")
PLANOS_CI_RE = re.compile(r"todos os ([0-9]+) planos declaram")
ORFAOS_RE = re.compile(r"0 órfãos")
MONTADAS_RE = re.compile(r"([0-9]+) páginas montadas")
MODULO_PAG_RE = re.compile(r"[0-9]+ p.ginas de m.dulo")
FONTES_RE = re.compile(r"([0-9]+)\s+fontes")
SISTEMA_NA_LINHA = re.compile(r"`(wyd-[a-z]+)`")
# Linha que declara medição de passada anterior: é registro histórico, não contagem atual.
HISTORICO_RE = re.compile(
    r"terceira passada|históric|Medido em 2026-\d\d-\d\d|Teto × escrito|medição de então|naquele|naquela|revista em 20\d\d-",
    re.IGNORECASE,
)
HISTORICO_DATA_RE = re.compile(r"\(20\d\d-\d\d-\d\d\)")
# Sistemas cujo roadmap não tem página por fatia: a medida é outra e a exceção imprime o motivo.
FATIAS_ESPECIAIS = {
    "wyd-admin": "roadmap.md lista `ADM-nn` em tabela; não há página de fatia",
}

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


def _paginas_md(base: Path) -> int:
    return sum(1 for _ in base.rglob("*.md"))


def medir_paginas_sistema(raiz: Path, sistema: str) -> int:
    base = raiz / SISTEMAS / sistema / "docs"
    if not base.is_dir():
        raise Erro(f"pasta de docs ausente: {base}")
    return _paginas_md(base)


def _paginas_issue(base: Path) -> int:
    return sum(
        1
        for arq in base.glob("*.md")
        if arq.name.startswith("WYD")
        and arq.name not in ("roadmap.md", "fases.md", "index.md")
    )


def medir_paginas_issue(raiz: Path, sistema: str | None = None) -> int:
    if sistema:
        return _paginas_issue(raiz / SISTEMAS / sistema / "docs/roadmap")
    total = 0
    for base in sorted((raiz / SISTEMAS).glob("*/docs/roadmap")):
        total += _paginas_issue(base)
    return total


def _conta_nome(raiz: Path, nome: str) -> int:
    return sum(1 for _ in (raiz / SISTEMAS).glob("*/" + nome))


def medir_pastas_gitkeep(raiz: Path) -> int:
    return sum(1 for _ in (raiz / SISTEMAS).glob("*/src/.gitkeep"))


def medir_paginas_pasta(raiz: Path, sistema: str, pasta: str | None) -> int:
    base = raiz / SISTEMAS / sistema / "docs"
    if pasta is None:
        return sum(1 for _ in base.glob("*.md"))
    destino = base / pasta
    if not destino.is_dir():
        raise Erro(f"pasta de docs ausente: {destino}")
    return sum(1 for _ in destino.rglob("*.md"))
def medir_planos_teste(raiz: Path) -> int:
    return sum(1 for _ in (raiz / SISTEMAS).glob("*/docs/planos/plano-de-testes.md"))


def medir_sistemas_com_fatias(raiz: Path) -> int:
    """Sistemas com página de fatia: exclui o `wyd-db` (série `WYDDB-nnn`, contada à parte)."""
    total = 0
    for base in sorted((raiz / SISTEMAS).glob("*/docs/roadmap")):
        if base.parent.parent.name == "wyd-db":
            continue
        if _paginas_issue(base) > 0:
            total += 1
    return total


def medir_planos_com_ci(raiz: Path) -> int:
    """Planos de teste que declaram CI, cobertura e monitoramento (as três grandezas da frase)."""
    padroes = (
        re.compile(r"(^|[^A-Za-z])CI([^A-Za-z]|$)|integração contínua|github actions", re.IGNORECASE),
        re.compile(r"cobertura", re.IGNORECASE),
        re.compile(r"monitoramento|monitorar", re.IGNORECASE),
    )
    total = 0
    for plano in sorted((raiz / SISTEMAS).glob("*/docs/planos/plano-de-testes.md")):
        texto = plano.read_text(encoding="utf-8")
        if all(padrao.search(texto) for padrao in padroes):
            total += 1
    return total


def medir_fatias(raiz: Path, sistema: str) -> int:
    if sistema in FATIAS_ESPECIAIS:
        roadmap = raiz / SISTEMAS / sistema / "docs/roadmap/roadmap.md"
        if not roadmap.is_file():
            raise Erro(f"roadmap ausente: {roadmap}")
        return len(set(re.findall(r"`(?:ADM)-[0-9]{2}`", roadmap.read_text(encoding="utf-8"))))
    return _paginas_issue(raiz / SISTEMAS / sistema / "docs/roadmap")


def _dashboards(raiz: Path) -> list[tuple[str, str]]:
    alvos: list[tuple[str, str]] = []
    for pasta in sorted((raiz / SISTEMAS).glob("*/")):
        for arq in sorted(pasta.glob("*-1.0.0.md")):
            alvos.append((arq.relative_to(raiz).as_posix(), pasta.name))
    if not alvos:
        raise Erro("nenhum dashboard `<sistema>-1.0.0.md`")
    return alvos


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
    for relativo, _ in _dashboards(raiz):
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


def _checar_arquivo(
    raiz: Path, relativo: str, sistema: str | None, fontes: int, paginas_site: int
) -> tuple[list[str], list[str]]:
    """Confere as cita\u00E7\u00F5es de p\u00E1ginas/fatias de um dashboard ou do status."""
    problemas: list[str] = []
    ok: list[str] = []
    db_paginas = medir_paginas_sistema(raiz, "wyd-db")
    for numero, linha in enumerate(ler(raiz, relativo).splitlines(), start=1):
        if HISTORICO_RE.search(linha) or HISTORICO_DATA_RE.search(linha):
            continue
        citacoes: list[tuple[int, int, str]] = []
        achado = ISSUE_PAGE_RE.search(linha)
        if achado:
            alvo_issue = sistema
            if alvo_issue:
                citacoes.append(
                    (
                        int(achado.group(1)),
                        medir_paginas_issue(raiz, alvo_issue),
                        "p\u00E1ginas de issue",
                    )
                )
        achado = FATIAS_RE.search(linha)
        if achado:
            alvo = sistema
            if alvo is None:
                nome = SISTEMA_NA_LINHA.search(linha) or re.search(r"\b(wyd-[a-z]+)\b", linha)
                alvo = nome.group(1) if nome else None
            if alvo:
                citacoes.append((int(achado.group(1)), medir_fatias(raiz, alvo), "fatias"))
        achado = PAGINAS_ESCRITAS_RE.search(linha)
        if achado:
            nome = SISTEMA_NA_LINHA.search(linha)
            if nome:
                citacoes.append(
                    (
                        int(achado.group(1)),
                        medir_paginas_sistema(raiz, nome.group(1)),
                        "p\u00E1ginas escritas",
                    )
                )
        achado = PAGINAS_DB_RE.search(linha)
        if achado:
            citacoes.append((int(achado.group(1)), db_paginas, "p\u00E1ginas no `wyd-db`"))
        achado = PAGINAS_CORPO_RE.search(linha)
        if achado and sistema:
            citacoes.append(
                (
                    int(achado.group(1)),
                    medir_paginas_sistema(raiz, sistema),
                    "páginas com corpo em docs/",
                )
            )
        achado = PAGINAS_FONTES_RE.search(linha)
        if achado:
            citacoes.append((int(achado.group(1)), paginas_site, "p\u00E1ginas"))
            citacoes.append((int(achado.group(2)), fontes, "fontes"))
        elif "p\u00E1ginas" in linha:
            achado = FONTES_RE.search(linha)
            if achado:
                citacoes.append((int(achado.group(1)), fontes, "fontes"))

        achado = SISTEMAS_GITKEEP_RE.search(linha)
        if achado:
            citacoes.append(
                (int(achado.group(1)), medir_pastas_gitkeep(raiz), "sistemas com src/.gitkeep")
            )
        achado = COMPONENTES_RE.search(linha)
        if achado:
            citacoes.append(
                (
                    int(achado.group(1)),
                    _conta_nome(raiz, "docs/arquitetura/componentes.md"),
                    "sistemas com componentes.md",
                )
            )
        achado = MODULOS_RE.search(linha)
        if achado:
            citacoes.append(
                (
                    int(achado.group(1)),
                    _conta_nome(raiz, "docs/modulos.md"),
                    "sistemas com docs/modulos.md",
                )
            )
        achado = PAGINAS_DOCS_RE.search(linha)
        if achado and sistema:
            citacoes.append(
                (int(achado.group(1)), medir_paginas_sistema(raiz, sistema), "p\u00E1ginas em docs/")
            )
        if sistema:
            for pasta_valor, pasta_nome in FOLDER_RE.findall(linha):
                try:
                    medido_pasta = medir_paginas_pasta(raiz, sistema, pasta_nome or None)
                except Erro:
                    continue
                citacoes.append(
                    (int(pasta_valor), medido_pasta, "p\u00E1ginas por pasta")
                )
        achado = RESTANTES_RE.search(linha)
        if achado and "modulos.md" in linha:
            restantes = len(_dashboards(raiz)) - _conta_nome(raiz, "docs/modulos.md")
            citacoes.append((int(achado.group(1)), restantes, "sistemas restantes"))
        achado = MONTADAS_RE.search(linha)
        if achado:
            citacoes.append((int(achado.group(1)), paginas_site, "páginas montadas"))
        achado = PLANO_SISTEMAS_RE.search(linha)
        if achado:
            citacoes.append((int(achado.group(1)), medir_planos_teste(raiz), "planos de teste"))
        achado = WYDDB_PAGINAS_RE.search(linha)
        if achado:
            citacoes.append(
                (int(achado.group(1)), medir_paginas_issue(raiz, "wyd-db"), "páginas WYDDB")
            )
        achado = FATIA_SPLIT_RE.search(linha)
        if achado:
            fatias = medir_paginas_issue(raiz) - medir_paginas_issue(raiz, "wyd-db")
            citacoes.append((int(achado.group(1)), fatias, "páginas por fatia"))
        achado = FATIA_SISTEMAS_RE.search(linha)
        if achado and "por fatia" in linha:
            citacoes.append(
                (int(achado.group(1)), medir_sistemas_com_fatias(raiz), "sistemas com fatia")
            )
        achado = DOMINIO_RE.search(linha)
        if achado and sistema:
            dominios = sum(
                1
                for _ in (raiz / SISTEMAS / sistema / "docs/dicionario-dados").glob("dominio-*.md")
            )
            citacoes.append((int(achado.group(1)), dominios, "páginas de domínio"))
        if MIGRACAO_PAG_RE.search(linha) and not PLANEJADO_RE.search(linha):
            problemas.append(
                f"{relativo}:{numero}: páginas de migration sem o marcador de planejado "
                "(o `docs/migrations/` não existe)"
            )
        achado = INV_ALVOS_RE.search(linha)
        if achado and sistema:
            medido = len(list((raiz / SISTEMAS / sistema / "tests/alvos").glob("*.md")))
            citacoes.append((int(achado.group(1)), medido, "páginas de alvo"))
        achado = INV_SPECS_RE.search(linha)
        if achado and sistema:
            medido = len(list((raiz / SISTEMAS / sistema / "tests/specs").glob("*.md")))
            citacoes.append((int(achado.group(1)), medido, "páginas de spec"))
        if (
            INV_ARQUIVO_RE.search(linha)
            and "alvos/*.md" not in linha
            and not PLANEJADO_RE.search(linha)
            and "▲" not in linha
        ):
            problemas.append(
                f"{relativo}:{numero}: inventário de páginas sem o marcador de planejado"
            )
        if INV_SRC_RE.search(linha) and not PREVISTO_RE.search(linha):
            problemas.append(
                f"{relativo}:{numero}: inventário de arquivos de `src/` sem previsto/planejado"
            )
        if ORFAOS_RE.search(linha):
            problemas.append(
                f"{relativo}:{numero}: afirma 0 órfãos sem medição "
                "(nenhum gate publica alcançabilidade)"
            )
        achado = PLANOS_CI_RE.search(linha)
        if achado:
            citacoes.append((int(achado.group(1)), medir_planos_com_ci(raiz), "planos com CI"))
        if MODULO_PAG_RE.search(linha) and not PLANEJADO_RE.search(linha):
            if not linha.lstrip().startswith("|") and "\u25B2" not in linha:
                problemas.append(
                    f"{relativo}:{numero}: páginas de módulo sem o marcador de planejado"
                )
        for citado, medido, unidade in citacoes:
            if citado != medido:
                problemas.append(f"{relativo}:{numero}: cita {citado} {unidade}, medido {medido}")
            else:
                ok.append(f"{relativo}:{numero}: {citado} {unidade}")
    return problemas, ok


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
        declaradas = censo_declarado(raiz)
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

    de_paginas: list[str] = []
    for relativo, sistema in _dashboards(raiz):
        achados, oks = _checar_arquivo(raiz, relativo, sistema, censo["fontes"], paginas)
        problemas += achados
        de_paginas += oks
    achados, oks = _checar_arquivo(raiz, STATUS, None, censo["fontes"], paginas)
    problemas += achados
    de_paginas += oks

    censos_ok: list[str] = []
    for citacao in declaradas:
        esperado = censo[citacao.unidade]
        if citacao.valor != esperado:
            problemas.append(
                f"{citacao.arquivo}:{citacao.linha}: censo cita {citacao.valor} "
                f"{citacao.unidade}, medido {esperado}"
            )
        else:
            censos_ok.append(
                f"{citacao.arquivo}:{citacao.linha}: {citacao.valor} {citacao.unidade} no censo"
            )

    print(
        f"contagens: {arquivos} arquivos, `Tab.` {soma_tab}, DDL {tabelas_ddl}, "
        f"plano {arquivos_plano}, site {paginas} páginas; "
        f"censo {censo['fontes']} fontes = {censo['aceito']} aceito + {censo['proposto']} proposto + "
        f"{censo['sem linha']} sem linha + {len(especiais)} fora do censo; "
        f"{len(da_serie) + len(declaradas)} citação(ões) da série, {len(ignoradas)} de outro dono, "
        f"{len(de_paginas)} de páginas/fatias"
    )
    for relativo, motivo in especiais:
        print(f"  fora do censo {relativo} ({motivo})")
    for citacao, dono in ignoradas:
        print(f"  ignorado {citacao.arquivo}:{citacao.linha}: {citacao.valor} {citacao.unidade} ({dono})")
    for linha in [f"  ok {c.arquivo}:{c.linha}: {c.valor} {c.unidade}" for c in da_serie]:
        print(linha)
    for linha in de_paginas:
        print(f"  ok {linha}")
    for linha in censos_ok:
        print(f"  ok {linha}")

    if problemas:
        for problema in problemas:
            print(f"  divergência: {problema}")
        print(f"contagens: {len(problemas)} problema(s)")
        return 1
    print("contagens: medidas e citações coincidem")
    print("censos: medidas e citações coincidem")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
