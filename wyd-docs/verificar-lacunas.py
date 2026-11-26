"""Pergunta: o placar de lacunas que o gate imprime é o mesmo que as tabelas do ledger declaram,
e cada lacuna fechada cita um artefato que existe **e trata do item**?

Três medidas na mesma passada:

1. **Placar.** Lê `planejamento-1.0.0/*/lacunas.md`, conta as linhas da seção dona do registro
   (a seção 2 ou, no `wyd-admin`, a tabela `## Fechadas`), descarta seções de resumo/lista
   (`Situação`, `Resumo`, `Medições herdadas`, `O que não é lacuna`, `Divergências`, `Achados`,
   `Lacunas fechadas`) e deduplica por id entre ledgers — o mesmo id em dois registros é uma
   lacuna, não duas. O estado vem do marcador da linha (`**fechada**`, `(bloqueada)`, …) ou do
   título da seção quando a linha não declara estado (`### LAC-POR-07 — … (fechada)`).
2. **Evidência.** Toda lacuna fechada precisa citar um arquivo que exista e cujo texto mencione um
   termo-âncora da própria lacuna (identificador em `código` da linha) ou a seção `§N` citada.
   Arquivo que existe mas não fala do item não é prova — é ponteiro genérico.
3. **Números publicados.** Número em negrito da linha (`**58 páginas**`, `**34 arquivos**`) precisa
   coincidir com a medição da seção `§N` do arquivo citado: páginas = linhas de tabela com página
   mais os `N páginas` dos títulos das subseções; arquivos, tabelas, fontes e rotas = o número que
   o título da seção declara. Contagem histórica em prosa (sem negrito, com data) não é medida
   vigente e não reprova; o número corrente é o que a linha afirma.

Além disso, confere o placar publicado: toda linha `lacunas medidas: N = F fechadas + B bloqueadas`,
toda fórmula `N = a + p + f + e` fora de trecho histórico (`após`, `estado anterior`, `antes do
briefing`) e as tabelas de lacunas — linha por sistema e linha `**Total**` — têm de bater com a
medição; divergência reprova. Nas tabelas, a coluna `Fechadas` é comparada com as fechadas medidas e
`Abertas + Propostas + Externas` com as bloqueadas.

Uso:
    python verificar-lacunas.py [--root CAMINHO]

Sai com 0 quando placar e evidências fecham, 1 quando algo diverge (imprime `arquivo:linha` e o
motivo) e 2 quando não há registro para medir.
"""

from __future__ import annotations

import argparse
import os
import re
import sys
from pathlib import Path

PLANEJAMENTO = "planejamento-1.0.0"
REGISTRO = "lacunas.md"
IGNORAR_DIR = {".git", "target", "node_modules", "site", ".build", "dist", "coverage"}

LINHA_LACUNA = re.compile(r"^\| `([A-Z][A-Z0-9]*-[A-Z0-9-]*\d+)`")
LINHA_LACUNA_NEGRITO = re.compile(r"^\| \*\*([A-Z][A-Z0-9-]*\d+)\*\*")
TITULO_LACUNA = re.compile(r"^#{3,4}\s+`?([A-Z][A-Z0-9]*-[A-Z0-9-]*\d+)`?\s*—")
TITULO_SECAO = re.compile(r"^(#{2,4})\s+(.*)")

# O estado é o que a coluna Estado diz; a palavra solta numa justificativa ("a coleta segue
# bloqueada") não reclassifica a lacuna.
FECHADA = re.compile(r"(\*\*fechada|\|\s*fechada\s*\||\(fechada\)|fechada \(|fechada em |fechada;|fechada,)")
BLOQUEADA = re.compile(r"(\*\*bloqueada|\|\s*bloqueada\s*\||\(bloqueada\)|bloqueada como|bloqueada —)")
FECHADA_TEXTO = re.compile(r"(?<!não )(?<!nao )fechad", re.IGNORECASE)
BLOQUEADA_TEXTO = re.compile(r"bloquead", re.IGNORECASE)

SECAO_RESUMO = re.compile(
    r"^(situa[çc][ãa]o|resumo|medi[çc][õo]es herdadas|o que n[ãa]o [ée] lacuna|diverg[êe]ncias|"
    r"achados|o que depende|como uma lacuna fecha|como ler|regra de manuten[çc][ãa]o|"
    r"o que fazer|prazos|donos|estados|marcos|baseline)",
    re.IGNORECASE,
)
SECAO_FECHADAS = re.compile(r"lacunas? fechadas?", re.IGNORECASE)
NUMERACAO = re.compile(r"^\d+(?:\.\d+)*\.?\s*")

ARTEFATO = re.compile(
    r"`([^`\s]*\.(?:md|rs|sql|py|yml|yaml|toml|json|tsv|sh|html|css|js))(?::\d+)?`"
)
PERGUNTA = re.compile(r"`(Q-[A-Z]+-\d+|WYDDB-\d{3})`")
ANCORA = re.compile(r"`([A-Za-z_][A-Za-z0-9_.:#$/<>{}\[\]-]{3,})`")
NUMERO_CODIGO = re.compile(r"^[\d.]+$")
PLACAR_CITADO = re.compile(
    r"lacunas medidas:\s*(\d+)\s*=\s*(\d+)\s*fechadas?\s*\+\s*(\d+)\s*bloqueadas?"
)

# Somas publicadas em tabela/formula alem da linha canonica. Cada conta e conferida contra a
# medicao: total, fechadas e o bloco das nao fechadas (abertas + propostas + externas).
TABELA_TOTAL = re.compile(
    r"^\|\s*\*\*Total\*\*\s*\|\s*\*\*(\d+)\*\*\s*\|\s*\*\*(\d+)\*\*\s*\|"
    r"\s*\*\*(\d+)\*\*\s*\|\s*\*\*(\d+)\*\*\s*\|\s*\*\*(\d+)\*\*"
)
TABELA_SISTEMA = re.compile(
    r"^\|\s*`(wyd-[a-z]+)`\s*\|\s*(\d+)\s*\|\s*(\d+)\s*\|\s*(\d+)\s*\|"
    r"\s*(\d+)\s*\|\s*(\d+)\s*\|"
)
SOMA_QUATRO = re.compile(r"(\d+)\s*=\s*(\d+)\s*\+\s*(\d+)\s*\+\s*(\d+)\s*\+\s*(\d+)")
PALAVRAS_SOMA = re.compile(
    r"\b(abertas?|abertos?|propostas?|proposta|fechadas?|fechada|externas?|externa)\b",
    re.IGNORECASE,
)
HISTORICO_SOMA = re.compile(r"\bap[oó]s\b|estado anterior|antes do briefing|naquela", re.IGNORECASE)

def _indice(root: Path) -> dict[str, Path]:
    indice: dict[str, Path] = {}
    for pasta, subpastas, arquivos in os.walk(root):
        subpastas[:] = [d for d in subpastas if d not in IGNORAR_DIR]
        for nome in arquivos:
            indice.setdefault(nome, Path(pasta) / nome)
    return indice


def _perguntas_registradas(root: Path, registros: list[Path]) -> set[str]:
    vistos: set[str] = set()
    for caminho in (root / PLANEJAMENTO).rglob("*.md"):
        if caminho in registros:
            continue
        vistos.update(PERGUNTA.findall(caminho.read_text(encoding="utf-8", errors="ignore")))
    return vistos


def _existe(root: Path, indice: dict[str, Path], citado: str) -> Path | None:
    alvo = citado.split("#", 1)[0]
    direto = root / alvo
    if direto.is_file():
        return direto
    return indice.get(Path(alvo).name)


def _estado(texto: str, secao: str) -> str | None:
    if FECHADA.search(texto):
        return "fechada"
    if BLOQUEADA.search(texto):
        return "bloqueada"
    if SECAO_FECHADAS.search(secao):
        return "fechada"
    if re.match(r"^fechadas\b", NUMERACAO.sub("", secao), re.IGNORECASE):
        return "fechada"
    if BLOQUEADA_TEXTO.search(texto) and not FECHADA_TEXTO.search(texto):
        return "bloqueada"
    if FECHADA_TEXTO.search(texto):
        return "fechada"
    return None


def _ancoras(texto: str) -> list[str]:
    achados: list[str] = []
    for bruto in ANCORA.findall(texto):
        if "/" in bruto or bruto.endswith((".md", ".rs", ".sql", ".py", ".yml", ".toml", ".json")):
            continue
        if NUMERO_CODIGO.match(bruto) or bruto.startswith("planejamento"):
            continue
        achados.append(bruto)
    return achados


def _corpo_por_numero(texto: str, numero: str) -> str:
    linhas = texto.splitlines()
    for indice, linha in enumerate(linhas):
        achado = re.match(rf"^(#{{1,6}})\s*{re.escape(numero)}\b", linha)
        if not achado:
            continue
        nivel = len(achado.group(1))
        corpo = [linha]
        for seguinte in linhas[indice + 1 :]:
            titulo = re.match(r"^(#{1,6})\s", seguinte)
            if titulo and len(titulo.group(1)) <= nivel:
                break
            corpo.append(seguinte)
        return "\n".join(corpo)
    return ""


def _corpo_por_titulo(texto: str, titulo: str) -> str:
    linhas = texto.splitlines()
    for indice, linha in enumerate(linhas):
        achado = re.match(r"^(#{1,6})\s+(.*)", linha)
        if not achado or titulo.lower() not in achado.group(2).lower():
            continue
        nivel = len(achado.group(1))
        corpo = [linha]
        for seguinte in linhas[indice + 1 :]:
            outro = re.match(r"^(#{1,6})\s", seguinte)
            if outro and len(outro.group(1)) <= nivel:
                break
            corpo.append(seguinte)
        return "\n".join(corpo)
    return ""


DECISAO = re.compile(
    r"(decid|decis|fechad|fecham|aprovad|vigente|regra:|exige|impede|garante|obriga)", re.IGNORECASE
)
IMPLEMENTACAO = (".sql", ".rs", ".js", ".py", ".toml", ".yml", ".yaml")


def _registra_decisao(conteudo: str, ancoras: list[str]) -> bool:
    """O termo-âncora aparece num trecho que enuncia decisão, não numa menção de passagem."""
    for ancora in ancoras:
        for achado in re.finditer(re.escape(ancora), conteudo, re.IGNORECASE):
            janela = conteudo[max(0, achado.start() - 500) : achado.end() + 500]
            if DECISAO.search(janela):
                return True
    return False


UNIDADES = {
    "página": "paginas",
    "páginas": "paginas",
    "arquivo": "arquivos",
    "arquivos": "arquivos",
    "tabela": "tabelas",
    "tabelas": "tabelas",
    "fonte": "fontes",
    "fontes": "fontes",
    "rota": "rotas",
    "rotas": "rotas",
}
NEGRITO = re.compile(r"\*\*(.+?)\*\*", re.DOTALL)
CLAIM = re.compile(
    r"(\d+)\s*(páginas?|arquivos?|tabelas?|fontes?|rotas?)\b", re.IGNORECASE
)


def _claims(texto: str) -> list[tuple[int, str, str]]:
    """Números publicados em negrito: só o que está afirmado como estado, não história em prosa."""
    achados: list[tuple[int, str, str]] = []
    for trecho in NEGRITO.findall(texto):
        for numero, unidade in CLAIM.findall(trecho):
            achados.append((int(numero), UNIDADES[unidade.lower()], f"{numero} {unidade}"))
    return achados


def _medicoes_da_secao(corpo: str) -> dict[str, int]:
    """Mede o que a própria seção declara: páginas listadas e números do título."""
    medidas: dict[str, int] = {}
    paginas = len(re.findall(r"^\|\s*`[^`]+\.md`", corpo, re.MULTILINE))
    for achado in re.finditer(r"^#{3,6}.*?(\d+)\s+páginas?\b", corpo, re.MULTILINE):
        paginas += int(achado.group(1))
    if paginas:
        medidas["paginas"] = paginas
    cabecalho = "\n".join(re.findall(r"^#{2,6}\s+.*$", corpo, re.MULTILINE))
    for achado in re.finditer(r"(\d+)\s+(arquivos?|tabelas?|fontes?|rotas?)\b", cabecalho):
        medidas.setdefault(UNIDADES[achado.group(2).lower()], int(achado.group(1)))
    return medidas


def _confere_numeros(
    root: Path, indice: dict[str, Path], texto_linha: str, lacuna: str
) -> str:
    """Número publicado na linha fechada precisa coincidir com a medição da seção citada."""
    declarados = _claims(texto_linha)
    if not declarados:
        return ""
    if "Refletida em" in texto_linha:
        texto_linha = texto_linha.split("Refletida em", 1)[1]
    for citado in ARTEFATO.findall(texto_linha):
        caminho = _existe(root, indice, citado)
        if caminho is None:
            continue
        try:
            conteudo = caminho.read_text(encoding="utf-8", errors="ignore")
        except OSError:
            continue
        for numero in re.findall(r"§(\d+(?:\.\d+)*)", texto_linha):
            medidas = _medicoes_da_secao(_corpo_por_numero(conteudo, numero))
            for valor, unidade, escrito in declarados:
                if unidade in medidas and valor != medidas[unidade]:
                    return (
                        f"lacuna {lacuna}: número publicado {escrito} diverge da medição "
                        f"{medidas[unidade]} {unidade} em {citado} §{numero}"
                    )
    return ""


def _confere_citacao(
    root: Path, indice: dict[str, Path], texto_linha: str, ancoras: list[str], lacuna: str = ""
) -> tuple[bool, str]:
    # Só o ponteiro declarado conta: menção de passagem antes dele não é registro.
    if "Refletida em" in texto_linha:
        texto_linha = texto_linha.split("Refletida em", 1)[1]
    citados = ARTEFATO.findall(texto_linha)
    if not citados:
        return False, "sem artefato citado"
    lidos: list[tuple[str, str]] = []
    for citado in citados:
        caminho = _existe(root, indice, citado)
        if caminho is None:
            continue
        try:
            lidos.append((citado, caminho.read_text(encoding="utf-8", errors="ignore")))
        except OSError:
            continue
    if not lidos:
        return False, "artefato citado não existe"
    titulos = re.findall(r"§\"([^\"]+)\"", texto_linha)
    for citado, conteudo in lidos:
        baixo = conteudo.lower()
        variantes = ancoras + [a.split(".")[-1] for a in ancoras if "." in a]
        if lacuna:
            # O registro cita o próprio id da lacuna quando enuncia a decisão.
            variantes = variantes + [lacuna]
        if any(a.lower() in baixo for a in variantes):
            if Path(citado).suffix in IMPLEMENTACAO or _registra_decisao(conteudo, variantes):
                return True, ""
        for titulo in titulos:
            corpo = _corpo_por_titulo(conteudo, titulo.strip().strip("`"))
            if corpo and _registra_decisao(corpo, variantes):
                return True, ""
        for numero in re.findall(r"§(\d+(?:\.\d+)*)", texto_linha):
            corpo = _corpo_por_numero(conteudo, numero)
            if corpo and _registra_decisao(corpo, variantes):
                return True, ""
    return False, f"citação não trata do item ({', '.join(c for c, _ in lidos)})"


def _conta(por_sistema: dict[str, dict[str, int]], sistema: str, estado: str) -> None:
    linha = por_sistema.setdefault(sistema, {"fechada": 0, "bloqueada": 0})
    linha[estado] = linha.get(estado, 0) + 1


def _confere_somas(
    root: Path,
    fechadas: int,
    bloqueadas: int,
    por_sistema: dict[str, dict[str, int]],
    problemas: list[str],
) -> None:
    total = fechadas + bloqueadas
    for caminho in sorted((root / PLANEJAMENTO).rglob("*.md")):
        rel = caminho.relative_to(root).as_posix()
        for numero, linha in enumerate(
            caminho.read_text(encoding="utf-8", errors="ignore").splitlines(), start=1
        ):
            achado = TABELA_TOTAL.match(linha)
            if achado:
                t, a, p, f, e = (int(g) for g in achado.groups())
                if (t, f, a + p + e) != (total, fechadas, bloqueadas):
                    problemas.append(
                        f"{rel}:{numero}: soma da tabela {t} = {a} + {p} + {f} + {e} "
                        f"diverge da medicao {total} = {fechadas} fechadas + {bloqueadas} "
                        "nao fechadas"
                    )
                elif t != a + p + f + e:
                    problemas.append(
                        f"{rel}:{numero}: soma da tabela {t} != {a} + {p} + {f} + {e}"
                    )
                continue
            achado = TABELA_SISTEMA.match(linha)
            if achado:
                sistema = achado.group(1)
                t, a, p, f, e = (int(g) for g in achado.groups()[1:])
                medida = por_sistema.get(sistema)
                if medida is None:
                    problemas.append(f"{rel}:{numero}: sistema `{sistema}` sem ledger medido")
                    continue
                mf = medida.get("fechada", 0)
                mb = medida.get("bloqueada", 0)
                if (t, f, a + p + e) != (mf + mb, mf, mb):
                    problemas.append(
                        f"{rel}:{numero}: linha de `{sistema}` cita {t} = {a} + {p} + {f} + {e}; "
                        f"medido {mf + mb} = {mf} fechadas + {mb} nao fechadas"
                    )
                elif t != a + p + f + e:
                    problemas.append(
                        f"{rel}:{numero}: linha de `{sistema}`: {t} != {a} + {p} + {f} + {e}"
                    )
                continue
            if HISTORICO_SOMA.search(linha):
                continue
            normal = PALAVRAS_SOMA.sub("", linha)
            for achado in SOMA_QUATRO.finditer(normal):
                t, a, p, f, e = (int(g) for g in achado.groups())
                if t != total:
                    continue
                if (f, a + p + e) != (fechadas, bloqueadas):
                    problemas.append(
                        f"{rel}:{numero}: soma citada {t} = {a} + {p} + {f} + {e} diverge da "
                        f"medicao {total} = {fechadas} fechadas + {bloqueadas} nao fechadas"
                    )


def verificar(root: Path) -> tuple[int, int, int, list[str]]:
    base = root / PLANEJAMENTO
    registros = sorted(base.glob(f"*/{REGISTRO}"))
    if not registros:
        raise SystemExit(f"verificar-lacunas: nenhum {PLANEJAMENTO}/*/{REGISTRO} em {root}")

    indice = _indice(root)
    perguntas = _perguntas_registradas(root, registros)
    problemas: list[str] = []
    vistas: dict[str, str] = {}
    por_sistema: dict[str, dict[str, int]] = {}

    for registro in registros:
        nome = registro.relative_to(root).as_posix()
        linhas = registro.read_text(encoding="utf-8").splitlines()
        secao = ""
        linhas_da_secao: list[int] = []
        for numero, linha in enumerate(linhas, start=1):
            titulo = TITULO_SECAO.match(linha)
            if titulo:
                secao = titulo.group(2).strip()
                continue
            if SECAO_RESUMO.match(NUMERACAO.sub("", secao)):
                continue
            achado = LINHA_LACUNA.match(linha) or LINHA_LACUNA_NEGRITO.match(linha)
            if not achado:
                continue
            linhas_da_secao.append(numero)
            lacuna = achado.group(1)
            if lacuna in vistas:
                continue
            estado = _estado(linha, secao)
            if estado is None:
                continue
            vistas[lacuna] = estado
            _conta(por_sistema, registro.parent.name, estado)
            if estado == "bloqueada":
                continue
            ancoras = _ancoras(linha)
            ok, motivo = _confere_citacao(root, indice, linha, ancoras, lacuna)
            if not ok:
                problemas.append(f"{nome}:{numero}: lacuna {lacuna}: {motivo}")

        # Registro em prosa: a lacuna é uma seção `### \`ID\` — …`, com o estado no título.
        secoes = list(re.finditer(r"^#{3,4}\s+.*$", "\n".join(linhas), re.MULTILINE))
        for achado in re.finditer(
            r"^(#{3,4})\s+`?([A-Z][A-Z0-9]*-[A-Z0-9-]*\d+)`?\s*—(.*)$",
            "\n".join(linhas),
            re.MULTILINE,
        ):
            lacuna = achado.group(2)
            if lacuna in vistas:
                continue
            inicio = achado.end()
            seguinte = next((s.start() for s in secoes if s.start() > achado.start()), len(linhas))
            corpo = "\n".join(linhas)[inicio:seguinte]
            estado = _estado(achado.group(3), "") or _estado(corpo, "")
            if estado is None:
                continue
            vistas[lacuna] = estado
            _conta(por_sistema, registro.parent.name, estado)
            if estado == "bloqueada":
                continue
            numero = "\n".join(linhas)[: achado.start()].count("\n") + 1
            ancoras = _ancoras(achado.group(3) + corpo)
            ok, motivo = _confere_citacao(root, indice, achado.group(3) + corpo, ancoras, lacuna)
            if not ok:
                problemas.append(f"{nome}:{numero}: lacuna {lacuna}: {motivo}")

    fechadas = sum(1 for e in vistas.values() if e == "fechada")
    bloqueadas = sum(1 for e in vistas.values() if e == "bloqueada")

    # Placar publicado: toda citação tem de coincidir com a medição.
    for caminho in sorted((root / PLANEJAMENTO).rglob("*.md")):
        for numero, linha in enumerate(
            caminho.read_text(encoding="utf-8", errors="ignore").splitlines(), start=1
        ):
            for achado in PLACAR_CITADO.finditer(linha):
                total, f, b = (int(g) for g in achado.groups())
                if (total, f, b) != (fechadas + bloqueadas, fechadas, bloqueadas):
                    rel = caminho.relative_to(root).as_posix()
                    problemas.append(
                        f"{rel}:{numero}: placar citado {total} = {f} + {b} diverge da medição "
                        f"{fechadas + bloqueadas} = {fechadas} + {bloqueadas}"
                    )

    # Número publicado no ledger coincide com a medição da seção citada, inclusive nas linhas
    # das seções de divergência, que ficam fora do placar mas publicam contagem.
    for registro in registros:
        nome = registro.relative_to(root).as_posix()
        for numero, linha in enumerate(
            registro.read_text(encoding="utf-8").splitlines(), start=1
        ):
            if "Refletida em" not in linha:
                continue
            achado = LINHA_LACUNA.match(linha) or LINHA_LACUNA_NEGRITO.match(linha)
            divergencia = _confere_numeros(
                root, indice, linha, achado.group(1) if achado else ""
            )
            if divergencia:
                problemas.append(f"{nome}:{numero}: {divergencia}")
    _confere_somas(root, fechadas, bloqueadas, por_sistema, problemas)

    return len(vistas), fechadas, bloqueadas, problemas


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--root", default="..", help="raiz do repositório (padrão: ..)")
    args = parser.parse_args(argv[1:])

    total, fechadas, bloqueadas, problemas = verificar(Path(args.root).resolve())

    if problemas:
        print(f"lacunas: {len(problemas)} problema(s) em {total} medida(s) "
              f"({fechadas} fechadas + {bloqueadas} bloqueadas)")
        for problema in problemas:
            print(f" - {problema}")
        return 1

    print(f"lacunas medidas: {total} = {fechadas} fechadas + {bloqueadas} bloqueadas")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
