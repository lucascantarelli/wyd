"""Gate de alvos de teste: comando que manda rodar um alvo precisa de alvo existente.

O defeito que esta regra impede é o de instrução quebrada: página manda rodar
`cargo test --test <alvo>` e o alvo não está declarado no crate, então o comando falha para quem
segue a documentação. O gate conferia link, âncora e contagem, mas não o nome do alvo.

**Só o comando de teste reprova.** Três formas de citação são lidas em
`planejamento-1.0.0/wyd-db/**/*.md`, e apenas a primeira é asserção dura:

1. **comando** — uma invocação `cargo test` com `--test <nome>`, em qualquer das grafias que
   apontam o crate: sem seletor, com `-p wyd-db` ou com
   `--manifest-path crates/wyd-db/Cargo.toml`. Linha quebrada com `\\` no fim é lida inteira. É a
   forma que quebra quando o alvo não existe.
2. **tabela** — coluna cujo cabeçalho começa por `Alvo`, lendo o nome entre crases de cada linha.
   Vira detalhe: inventário não é instrução, e o comando ao lado já carrega a reprovação.
3. **prosa** — `alvo `x``, `alvos `x` e `y``, com vírgula ou `ou` na lista. Também detalhe. Só entra
   o token com a forma de nome de alvo (`^[a-z][a-z0-9_]{2,}$`): caminho de arquivo, letra solta e
   palavra comum ficam fora, e o que o filtro descarta é impresso em `--details`.

**`--test` fora de comando não some em silêncio.** Linha lógica que cita `--test <nome>` sem
`cargo test` (frase que explica o nome do alvo, lista de flags, exemplo solto) não é comando: ela
nomeia a flag, não manda rodar nada, e por isso não entra na contagem nem reprova. Mesmo assim cada
nome sai em `--details` como descartado, com arquivo e linha — o que o check deixa de contar fica
visível, como os tokens de prosa que o filtro rejeita.

O nome citado em comando é aceito em dois casos:

- existe como `[[test]]` em `crates/wyd-db/Cargo.toml`;
- a tabela de `planejamento-1.0.0/wyd-db/docs/guia-teste/index.md` §1 o declara com `Estado` que
  começa por `planejado`. A lista é **derivada da página dona**, não escrita no código: alvo novo
  entra na tabela, não neste arquivo. O que ela cobre é impresso em `--details`.

Comando que aponta alvo planejado só passa **marcado como futuro** na própria linha (`- [ ]` de DoD
pendente ou `planejad`/`previst`/`ainda não`). Sem a marca, é instrução de runbook e tem de rodar
hoje.

Fica fora do escopo, sem reprovar: `--manifest-path` de outro crate ou com trecho dinâmico (`$c`) e
`-p <outro crate>`. `--test <alvo>` com o nome entre sinais de maior e menor é exemplo genérico, não
citação: fica fora.

Vive em `wyd-docs/` porque `planejamento-1.0.0/` é documentação e não aceita executável (AGENTS §5.6).

Uso:
    python -m gate check alvos --root ..   # a partir de wyd-docs/
"""

from __future__ import annotations

import re
from typing import NamedTuple

from ..model import CheckResult
from ..workspace import Workspace

CARGO = "crates/wyd-db/Cargo.toml"
CRATE = "wyd-db"
BASE = "planejamento-1.0.0/wyd-db"
GUIA = f"{BASE}/docs/guia-teste/index.md"

CMD_RE = re.compile(r"--test\s+`?([A-Za-z_][A-Za-z0-9_]*)")
CARGO_TEST_RE = re.compile(r"cargo\s+test\b", re.IGNORECASE)
MANIFEST_RE = re.compile(r"--manifest-path\s+[`\"']?([^\s`\"']+)")
PKG_RE = re.compile(r"(?:^|\s)-p\s+`?([A-Za-z0-9_-]+)")
PROSA_RE = re.compile(r"\balvos?\s+(`[^`]+`(?:\s*(?:,|e|ou)\s*`[^`]+`)*)", re.IGNORECASE)
TOKEN_RE = re.compile(r"`([^`]+)`")
NOME_RE = re.compile(r"^[a-z][a-z0-9_]{2,}$")
HEADER_RE = re.compile(r"^\|\s*Alvo(?:\s|\||`|\(|$)")
CELL_RE = re.compile(r"`([^`]+)`")
PENDENTE_RE = re.compile(r"^\s*[-*]\s*\[ \]")
FUTURO_RE = re.compile(r"planejad|previst|ainda não|não existe", re.IGNORECASE)
PLANEJADO_RE = re.compile(r"^planejad", re.IGNORECASE)


class Citacao(NamedTuple):
    arquivo: str
    linha: int
    nome: str
    forma: str
    escopo: str = ""


class Erro(Exception):
    """Falha de leitura que impede medir — não é divergência de alvo."""


def declarados(ws: Workspace) -> set[str]:
    """Nomes de `[[test]]` do crate; `[package] name` fica de fora."""
    caminho = ws.root / CARGO
    if not ws.exists(caminho) or not caminho.is_file():
        raise Erro(f"arquivo ausente: {CARGO}")
    nomes: set[str] = set()
    dentro = False
    for linha in ws.read(caminho).splitlines():
        texto = linha.strip()
        if texto.startswith("[["):
            dentro = texto == "[[test]]"
            continue
        if dentro and texto.startswith("name"):
            achado = re.search(r'"([^"]+)"', texto)
            if achado:
                nomes.add(achado.group(1))
    if not nomes:
        raise Erro(f"{CARGO}: nenhum `[[test]]` declarado")
    return nomes


def planejados(ws: Workspace) -> dict[str, str]:
    """Alvos que a tabela de `docs/guia-teste/index.md` §1 marca como `planejado`.

    A lista é lida da página dona: quem declara alvo novo edita a tabela, não o gate.
    """
    caminho = ws.root / GUIA
    if not ws.exists(caminho) or not caminho.is_file():
        raise Erro(f"arquivo ausente: {GUIA}")

    achados: dict[str, str] = {}
    coluna_alvo: int | None = None
    coluna_estado: int | None = None
    for linha in ws.read(caminho).splitlines():
        if coluna_alvo is None:
            if not HEADER_RE.match(linha):
                continue
            celulas = [celula.strip() for celula in linha.strip().strip("|").split("|")]
            coluna_alvo = next(
                (indice for indice, celula in enumerate(celulas) if celula.startswith("Alvo")),
                None,
            )
            coluna_estado = next(
                (indice for indice, celula in enumerate(celulas) if celula.startswith("Estado")),
                None,
            )
            if coluna_estado is None:
                raise Erro(f"{GUIA} §1: tabela de alvos sem coluna `Estado`")
            continue
        if not linha.lstrip().startswith("|"):
            break
        celulas = [celula.strip() for celula in linha.strip().strip("|").split("|")]
        if coluna_alvo >= len(celulas) or coluna_estado >= len(celulas):
            continue
        nomes = CELL_RE.findall(celulas[coluna_alvo])
        if nomes and PLANEJADO_RE.match(celulas[coluna_estado]):
            achados[nomes[0]] = "`docs/guia-teste/index.md` §1"
    if coluna_alvo is None:
        raise Erro(f"{GUIA} §1: tabela de alvos não encontrada")
    # Tabela sem linha `planejado` é estado válido: todos os alvos implementados.
    return achados


def _logicas(linhas: list[str]) -> list[tuple[int, str]]:
    """Linhas lógicas: `\\` no fim junta a continuação, mantendo o número da primeira."""
    saida: list[tuple[int, str]] = []
    indice = 0
    while indice < len(linhas):
        numero = indice + 1
        texto = linhas[indice].rstrip()
        while texto.endswith("\\") and indice + 1 < len(linhas):
            indice += 1
            texto = f"{texto[:-1]} {linhas[indice].strip()}".rstrip()
        saida.append((numero, texto))
        indice += 1
    return saida


def escopo(texto: str) -> str:
    """Motivo de o comando ficar fora do crate, ou vazio quando é do `wyd-db`."""
    manifesto = MANIFEST_RE.search(texto)
    if manifesto:
        caminho = manifesto.group(1).replace("\\", "/").rstrip("\"',;")
        if "$" in caminho or "{" in caminho:
            return f"caminho dinâmico `{caminho}`"
        if not caminho.endswith(f"crates/{CRATE}/Cargo.toml"):
            return f"manifesto `{caminho}`"
        return ""
    pacote = PKG_RE.search(texto)
    if pacote and pacote.group(1) != CRATE:
        return f"pacote `{pacote.group(1)}`"
    return ""


def _citacoes_tabela(linhas: list[str]) -> list[tuple[int, str]]:
    """(número da linha, nome) das linhas de tabela cujo cabeçalho começa por `Alvo`."""
    achados: list[tuple[int, str]] = []
    coluna: int | None = None
    for numero, linha in enumerate(linhas, start=1):
        if coluna is None:
            if HEADER_RE.match(linha):
                celulas = [celula.strip() for celula in linha.strip().strip("|").split("|")]
                coluna = next(
                    (indice for indice, celula in enumerate(celulas) if celula.startswith("Alvo")),
                    None,
                )
            continue
        if not linha.lstrip().startswith("|"):
            coluna = None
            continue
        celulas = [celula.strip() for celula in linha.strip().strip("|").split("|")]
        if coluna >= len(celulas):
            continue
        for nome in CELL_RE.findall(celulas[coluna]):
            achados.append((numero, nome))
    return achados


def _citacoes_prosa(linhas: list[str]) -> tuple[list[tuple[int, str]], list[tuple[int, str]]]:
    """(linha, nome) citados em `alvo `x`` e (linha, token) que o filtro descartou."""
    achados: list[tuple[int, str]] = []
    ignorados: list[tuple[int, str]] = []
    for numero, linha in enumerate(linhas, start=1):
        for trecho in PROSA_RE.findall(linha):
            for token in TOKEN_RE.findall(trecho):
                if NOME_RE.match(token):
                    achados.append((numero, token))
                else:
                    ignorados.append((numero, token))
    return achados, ignorados


def citacoes(
    ws: Workspace,
) -> tuple[list[Citacao], list[tuple[str, int, str]], list[tuple[str, int, str]]]:
    """Citações de alvo, tokens de prosa descartados e `--test` sem `cargo test`.

    Os dois últimos grupos não contam nem reprovam: são o que o check deixou de fora, e vão para
    `--details` para que nenhuma citação suma sem rastro.
    """
    base = ws.root / BASE
    if not ws.is_dir(base):
        raise Erro(f"pasta ausente: {BASE}")
    achados: list[Citacao] = []
    descartados: list[tuple[str, int, str]] = []
    sem_comando: list[tuple[str, int, str]] = []
    for arquivo in ws.files(base, "*.md"):
        relativo = arquivo.relative_to(ws.root).as_posix()
        linhas = ws.read(arquivo).splitlines()
        for numero, texto in _logicas(linhas):
            nomes = CMD_RE.findall(texto)
            if not nomes:
                continue
            if not CARGO_TEST_RE.search(texto):
                sem_comando += [(relativo, numero, nome) for nome in nomes]
                continue
            motivo = escopo(texto)
            for nome in nomes:
                achados.append(Citacao(relativo, numero, nome, "comando", motivo))
        for numero, nome in _citacoes_tabela(linhas):
            achados.append(Citacao(relativo, numero, nome, "tabela"))
        da_prosa, ignorados = _citacoes_prosa(linhas)
        for numero, nome in da_prosa:
            achados.append(Citacao(relativo, numero, nome, "prosa"))
        for numero, token in ignorados:
            descartados.append((relativo, numero, token))
    return achados, descartados, sem_comando


def run(ws: Workspace) -> CheckResult:
    try:
        nomes = declarados(ws)
        plano = planejados(ws)
        achados, descartados, sem_comando = citacoes(ws)
    except Erro as erro:
        return CheckResult("alvos", False, str(erro))

    problemas: list[str] = []
    detalhes: list[str] = []
    citados_planejados = 0
    fora = 0
    for citacao in achados:
        local = f"{citacao.arquivo}:{citacao.linha}"
        # Comando de outro crate não diz nada sobre os alvos do `wyd-db`, mesmo que o nome exista
        # aqui: fica fora antes de qualquer conferência e deixa rastro em `--details`.
        if citacao.escopo:
            fora += 1
            detalhes.append(
                f"ignorado {local}: `{citacao.nome}` citado fora do crate ({citacao.escopo})"
            )
            continue
        if citacao.nome in nomes:
            detalhes.append(f"ok {local}: `{citacao.nome}` ({citacao.forma})")
            continue
        if citacao.nome in plano:
            citados_planejados += 1
            linha = ws.read(ws.root / citacao.arquivo).splitlines()[citacao.linha - 1]
            marcado = bool(FUTURO_RE.search(linha)) or bool(PENDENTE_RE.match(linha))
            if citacao.forma == "comando" and not marcado:
                problemas.append(
                    f"{local}: comando usa o alvo planejado `{citacao.nome}`, que não existe em "
                    f"{CARGO}; sem marca de futuro na linha, a instrução falha hoje"
                )
            else:
                detalhes.append(
                    f"planejado {local}: `{citacao.nome}` ({citacao.forma}; declarado em "
                    f"{plano[citacao.nome]})"
                )
            continue
        if citacao.forma != "comando":
            detalhes.append(
                f"nota {local}: `{citacao.nome}` ({citacao.forma}) não é `[[test]]` nem alvo "
                "planejado; só o comando de teste reprova"
            )
            continue
        problemas.append(
            f"{local}: `cargo test --test {citacao.nome}` não é `[[test]]` em {CARGO} nem está "
            f"planejado na tabela de {GUIA} §1"
        )

    detalhes += [
        f"ignorado {arquivo}:{linha}: `{token}` não tem forma de nome de alvo"
        for arquivo, linha, token in descartados
    ]
    detalhes += [
        f"ignorado {arquivo}:{linha}: `--test {nome}` sem `cargo test` na mesma linha lógica"
        for arquivo, linha, nome in sem_comando
    ]
    resumo = (
        f"{len(achados) - fora} citação(ões) de alvo conferida(s) contra {len(nomes)} `[[test]]`; "
        f"{fora} fora do crate; "
        f"{citados_planejados} de alvo planejado ({len(plano)} em `docs/guia-teste/index.md` §1); "
        f"{len(descartados)} token(s) descartado(s) em prosa; "
        f"{len(sem_comando)} `--test` fora de comando de teste"
    )
    if problemas:
        return CheckResult("alvos", False, resumo, problems=problemas, details=detalhes)
    return CheckResult("alvos", True, resumo, details=detalhes)
