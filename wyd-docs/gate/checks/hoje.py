"""Pergunta: todo número que o planejamento apresenta como **vigente hoje** bate com o dono da
medida, e a seção que o apresenta está datada quando é registro?

O `censos`, o `lacunas` e o `capacidades` conferem a citação escrita (`138 atos`, `20 de 39`), mas
não olham o advérbio: `hoje 141 atos` num parágrafo sem data passava como prosa. Esta varredura
fecha esse buraco sem ler o documento à mão.

Uma citação entra quando:

1. o número está **colado a `hoje`** (até 30 caracteres de distância, sem `;` `.` `,` `|` no meio),
   antes ou depois do advérbio; e
2. o número declara a unidade do censo logo depois (`N atos`, `N páginas`, `N células`, `N lacunas`,
   `N capacidades`, `N Q-BLK`) ou a forma `N de M` da matriz de capacidades — ou ainda a unidade da
   `contagens` (`N arquivos`, `N tabelas`, `N fontes`, `N páginas do site`).

Número sem unidade colada (`hoje **138**`, `hoje **1**`) entra quando o **contexto** o atribui a um
único censo: a cláusula (trecho da frase entre `;` ou `|`) cita a unidade do censo, ou nomeia o dono
(`§6.6.22.1` para atos, `lacunas.md` §1, `capacidades.md` §7). A atribuição é recusada — e o número
continua fora — quando a cláusula cita outro dono (outra seção ou outra página), quando há mais de
uma unidade candidata, quando o número é razão de outra medida (`N das 40`), quando está dentro de
crase (identificador como `0016` ou `CAP-08`) ou quando `:`/`=` entre o advérbio e o número anuncia
detalhamento. Cada atribuição por contexto sai em `--details` como `ok`, para não passar em silêncio.

A medida é sempre a do dono: `censos` (atos, páginas aceitas, células, Q-BLK), `lacunas` §1 do
`wyd-db` (39), `capacidades` §7 (20 de 39) e `contagens` (arquivos da série, soma de `Tab.`, fontes
do censo e páginas do site) — nenhum número é copiado aqui. A unidade da `contagens` é genérica
(`arquivos`, `tabelas`, `fontes` medem outras coisas em outras páginas): ela só entra quando a
cláusula não mede outra coisa — outra contagem vizinha (`45 arquivos, 109 citações`), lista de
objetos em crase separada por `/` (`content`/`game`/…) ou caminho de outra árvore (`deprecated/`,
`target/`) recusam —, e `fontes` exige `censo` na cláusula. `arquivos` e `tabelas` só entram com a
marca da série **documentada** (`Tab.`, `DDL`, `checklist`, `plano-migrations.md` ou
`migrations-checklist.md`): `série` e `migrations` também nomeiam a série materializada (28 dos 34
arquivos, em `crates/wyd-db/migrations/`), e uma frase como `a série materializada tem hoje 28
arquivos` mede outro conjunto.
`páginas` continua sendo o censo de aceitas; a medida da `contagens` só é reconhecida como
`páginas do site`.

Isenção, impressa em `--details` como `ignorado`: seção datada antes da medição, data na própria
frase e marca de registro histórico (`naquele momento`, `medição de`, `era`). Seção sem data e sem
marca tem de bater com o dono; divergência reprova com `arquivo:linha`.

Uso:
    python -m gate check hoje --root .. --details   # a partir de wyd-docs/

Exit 0 quando tudo bate, 1 em divergência e 2 quando o dono não é legível.
"""

from __future__ import annotations

import re
from typing import NamedTuple

from ..model import CheckResult
from ..workspace import Workspace
from . import _texto, capacidades, censos, contagens, lacunas

PLANEJAMENTO = "planejamento-1.0.0"

HOJE_RE = re.compile(r"\bhoje\b", re.IGNORECASE)
CORTE_JANELA = 30

# Unidade declarada logo depois do número — só ela identifica o censo da citação.
UNIDADES = {
    "ato": "atos",
    "atos": "atos",
    "página": "páginas",
    "páginas": "páginas",
    "célula": "células",
    "células": "células",
    "lacuna": "lacunas",
    "lacunas": "lacunas",
    "capacidade": "capacidades",
    "capacidades": "capacidades",
    "q-blk": "Q-BLK",
    "tabela": "tabelas",
    "tabelas": "tabelas",
    "arquivo": "arquivos",
    "arquivos": "arquivos",
    "fonte": "fontes",
    "fontes": "fontes",
    "páginas do site": "páginas do site",
}
UNIDADE_RE = re.compile(
    r"^\s*\*{0,2}\s*(páginas\s+do\s+site|atos?|páginas?|células?|lacunas?|capacidades?|Q-BLK|"
    r"tabelas?|arquivos?|fontes?)\b",
    re.IGNORECASE,
)
DE_RE = re.compile(r"^\s+de\s+(\d+)\b")


class Unidade(NamedTuple):
    """Unidade que qualifica um número: canônica e denominador de `N de M`."""

    canonica: str
    de: int | None


def unidade_de(bruto: str) -> str:
    return UNIDADES[re.sub(r"\s+", " ", bruto.lower())]


def unidade_colada(linha: str, fim_numero: int) -> Unidade | None:
    """Unidade escrita logo depois do número, ou a forma `N de M` da matriz de capacidades."""
    achado = UNIDADE_RE.match(linha[fim_numero:])
    if achado:
        return Unidade(unidade_de(achado.group(1)), None)
    de = DE_RE.match(linha[fim_numero:])
    if de:
        return Unidade("capacidades", int(de.group(1)))
    return None


def em_codigo(linha: str, pos: int) -> bool:
    """Dentro de crase o número é identificador (`0016`, `CAP-08`), não contagem."""
    return linha.count("`", 0, pos) % 2 == 1

# Contexto só vale para número vizinho do advérbio; longe dele a atribuição vira adivinhação.
CORTE_CONTEXTO = CORTE_JANELA
# `:` e `=` entre o advérbio e o número anunciam detalhamento, não a medida.
QUEBRA_RE = re.compile(r"[:=→]")
HISTORICO_RE = re.compile(r"naquele|naquela|seguia|seguiam|continuaram|\bera\b", re.IGNORECASE)

UNIDADE_SOLTA_RE = re.compile(
    r"(?<![\w-])(páginas\s+do\s+site|atos?|páginas?|células?|lacunas?|capacidades?|Q-BLK|"
    r"tabelas?|arquivos?|fontes?)(?![\w-])",
    re.IGNORECASE,
)
# Contagem vizinha na mesma enumeração (`45 arquivos, 109 citações`) mede outra coisa.
OUTRA_CONTAGEM_RE = re.compile(
    r"\b(citações?|menções?|commits?|rotas?|requisitos?)\b", re.IGNORECASE
)
# Lista de objetos em crase separada por `/` (`content`/`game`/…) mede outro conjunto.
LISTA_CRASE_RE = re.compile(r"`[^`]+`\s*/\s*`[^`]+`")
# Caminho de outra árvore (`deprecated/`, `target/`) também mede outro conjunto.
OUTRO_CAMINHO_RE = re.compile(r"\b(?:deprecated|target)/")
CENSO_RE = re.compile(r"\bcenso\b", re.IGNORECASE)
SITE_RE = re.compile(r"\bsite\b", re.IGNORECASE)
# `série` e `migrations` também nomeiam a série materializada (28 dos 34 arquivos, em
# `crates/wyd-db/migrations/`) e o rascunho antigo; só a marca da série documentada identifica a checklist.
SERIE_RE = re.compile(
    r"\bTab\.|\bDDL\b|\bchecklist\b|\b(?:plano-migrations|migrations-checklist)\.md\b"
)
UNIDADES_CONTAGENS = {"arquivos", "tabelas", "fontes", "páginas do site"}
CAP_DONO_RE = re.compile(r"capacidades\.md`?[^|;]{0,12}§\s*7\b")
LAC_DONO_RE = re.compile(r"lacunas\.md`?[^|;]{0,12}§\s*1\b")
ATOS_DONO_RE = re.compile(r"§\s*6\.6\.22\.1\b")
SECAO_RE = re.compile(r"§\s*(\d+(?:\.\d+)*)")
MD_RE = re.compile(r"([A-Za-z0-9_./-]+\.md)")
RAZAO_RE = re.compile(r"^\s*(?:de|das?|dos?)\s+\d+")

class Medida:
    """O que cada dono mede hoje, pedido pela API pública de cada check."""

    def __init__(self, ws: Workspace) -> None:
        censo = censos.medidas(ws)
        lacunas_db = lacunas.medidas(ws)
        matriz = capacidades.medir(ws)
        contagem = contagens.medidas(ws)
        self.valores = {
            "atos": censo.atos["total"],
            "páginas": censo.aceitas,
            "células": censo.teste + censo.issue,
            "Q-BLK": len(censo.qblk),
            "lacunas": lacunas_db.total,
            "capacidades": matriz.cobertas,
            "arquivos": contagem.arquivos,
            "tabelas": contagem.tabelas,
            "fontes": contagem.fontes,
            "páginas do site": contagem.paginas_site,
        }
        self.matriz = (matriz.cobertas, matriz.total)
        self.corte = censo.corte

    def esperado(self, unidade: str, de: int | None) -> int | None:
        if unidade == "capacidades" and de is not None:
            return self.matriz[0] if de == self.matriz[1] else None
        return self.valores.get(unidade)


def _numero_colado(linha: str, inicio: int, fim: int) -> list[tuple[int, int, int]]:
    """Números colados ao advérbio: `(valor, começo, fim)` do número, antes ou depois de `hoje`."""
    achados: list[tuple[int, int, int]] = []
    base_antes = max(0, inicio - CORTE_JANELA)
    antes = re.search(r"(\d+)\*{0,2}[^;.,|\d]*$", linha[base_antes:inicio])
    if antes:
        comeco = base_antes + antes.start(1)
        achados.append((int(antes.group(1)), comeco, comeco + len(antes.group(1))))
    depois = re.match(r"\*{0,2}[^;.,|\d]*?(\d+)", linha[fim : fim + CORTE_JANELA])
    if depois:
        comeco = fim + depois.start(1)
        achados.append((int(depois.group(1)), comeco, comeco + len(depois.group(1))))
    return achados


def _clausula(linha: str, pos: int) -> str:
    """Trecho da frase que contém `pos`, cortado em `;` e `|`: célula de tabela é contexto próprio."""
    comeco, fim = 0, len(linha)
    for inicio, termino in _texto.frases(linha):
        if inicio <= pos < termino:
            comeco, fim = inicio, termino
            break
    esquerda = max(linha.rfind(";", comeco, pos), linha.rfind("|", comeco, pos))
    inicio = esquerda + 1 if esquerda >= comeco else comeco
    direitas = [
        achado
        for achado in (linha.find(";", pos, fim), linha.find("|", pos, fim))
        if achado != -1
    ]
    return linha[inicio : min(direitas) if direitas else fim]


def _contexto(linha: str, pos: int, fim_numero: int) -> tuple[str, str] | None:
    """Censo e motivo quando o número não traz unidade: cláusula com um único dono/unidade.

    Recusa (devolve `None`) quando o número é razão de outra medida (`N das 40`), quando a cláusula
    cita outra seção ou outra página, ou quando há mais de um dono ou unidade possível.
    """
    depois = linha[fim_numero : fim_numero + 12]
    if RAZAO_RE.match(depois) or re.match(r"^\s*/\s*\d", depois):
        return None
    trecho = _clausula(linha, pos)
    donos: set[str] = set()
    if CAP_DONO_RE.search(trecho):
        donos.add("capacidades")
    if LAC_DONO_RE.search(trecho):
        donos.add("lacunas")
    if ATOS_DONO_RE.search(trecho):
        donos.add("atos")
    if len(donos) > 1:
        return None
    permitidas = {"6.6.22.1"} if "atos" in donos else set()
    if "capacidades" in donos:
        permitidas.add("7")
    if "lacunas" in donos:
        permitidas.add("1")
    secoes = {achado.group(1) for achado in SECAO_RE.finditer(trecho)}
    if secoes - permitidas:
        return None
    # Só a página dona do censo atribuído pode acompanhar a cláusula; o ledger do censo de atos é
    # `pendencias-de-fechamento.md` §6.6.22.1.
    donas = {f"{dono}.md" for dono in donos}
    if "atos" in donos:
        donas.add("pendencias-de-fechamento.md")
    if set(MD_RE.findall(trecho)) - donas:
        return None
    unidades = {
        unidade_de(achado.group(1)) for achado in UNIDADE_SOLTA_RE.finditer(trecho)
    }
    if donos:
        dono = next(iter(donos))
        if unidades and unidades != {dono}:
            return None
        return dono, "dono na cláusula"
    if len(unidades) == 1:
        return next(iter(unidades)), "unidade na cláusula"
    return None


def _contagens_ok(linha: str, inicio_numero: int, fim_numero: int, unidade: str) -> bool:
    """Unidade da `contagens` só entra quando a cláusula não mede outra coisa."""
    trecho = _clausula(linha, inicio_numero)
    if unidade == "páginas do site":
        return True
    if unidade == "páginas":
        # `páginas` sozinho é o censo de aceitas; com `site` na cláusula a medida é a da `contagens`.
        return not SITE_RE.search(trecho)
    if unidade == "fontes":
        return bool(CENSO_RE.search(trecho))
    if unidade in {"arquivos", "tabelas"} and not SERIE_RE.search(trecho):
        # `N arquivos` e `N tabelas` também medem a série materializada e o rascunho antigo; sem a
        # marca da série documentada, a citação mede outro conjunto e fica fora.
        return False
    vizinhos = linha[fim_numero : fim_numero + 40]
    if OUTRA_CONTAGEM_RE.search(vizinhos):
        return False
    return not (LISTA_CRASE_RE.search(trecho) or OUTRO_CAMINHO_RE.search(trecho))


def citacoes(
    ws: Workspace, medida: Medida
) -> tuple[list[str], list[tuple[str, str]], list[tuple[str, str]], int]:
    """Citações conferidas, isenções históricas e quantas citações de `hoje` têm unidade."""
    problemas: list[str] = []
    historicas: list[tuple[str, str]] = []
    contextuais: list[tuple[str, str]] = []
    conferidas = 0
    raiz = ws.root
    base = raiz / PLANEJAMENTO
    if not base.is_dir():
        raise RuntimeError(f"pasta ausente: {base}")
    corte = medida.corte
    for pagina in ws.files(base, "*.md"):
        relativo = pagina.relative_to(raiz).as_posix()
        pilha: list[tuple[int, str | None]] = []
        for numero, linha in _texto.linhas_logicas(ws.read(pagina)):
            if linha.startswith("#"):
                nivel = len(linha) - len(linha.lstrip("#"))
                while pilha and pilha[-1][0] >= nivel:
                    pilha.pop()
                achado = _texto.DATA_RE.search(linha)
                pilha.append((nivel, achado.group(1) if achado else None))
                continue
            if not HOJE_RE.search(linha):
                continue
            data_secao = _texto.data_da_secao(pilha)
            for advérbio in HOJE_RE.finditer(linha):
                for valor, inicio_numero, fim_numero in _numero_colado(
                    linha, advérbio.start(), advérbio.end()
                ):
                    # Crase é identificador ou exemplo (`0016`, `hoje 138`), não afirmação vigente.
                    if em_codigo(linha, inicio_numero):
                        continue
                    lido = unidade_colada(linha, fim_numero)
                    contexto = ""
                    if lido is None:
                        if inicio_numero >= advérbio.end():
                            distancia = inicio_numero - advérbio.end()
                        else:
                            distancia = advérbio.start() - fim_numero
                        if distancia > CORTE_CONTEXTO or (
                            inicio_numero and linha[inicio_numero - 1] in "/=:"
                        ):
                            continue
                        if inicio_numero >= advérbio.end():
                            entre = linha[advérbio.end() : inicio_numero]
                        else:
                            entre = linha[fim_numero : advérbio.start()]
                        achado_contexto = (
                            None if QUEBRA_RE.search(entre) else _contexto(linha, inicio_numero, fim_numero)
                        )
                        if achado_contexto is None:
                            continue
                        lido = Unidade(achado_contexto[0], None)
                        contexto = achado_contexto[1]
                    unidade, de = lido.canonica, lido.de
                    if unidade in UNIDADES_CONTAGENS or unidade == "páginas":
                        if not _contagens_ok(linha, inicio_numero, fim_numero, unidade):
                            continue
                    esperado = medida.esperado(unidade, de)
                    if esperado is None:
                        continue
                    conferidas += 1
                    if de is not None:
                        escrito = f"{valor} de {de} {unidade}"
                        medido = f"{esperado} de {de} {unidade}"
                    else:
                        escrito = f"{valor} {unidade}"
                        medido = f"{esperado} {unidade}"
                    if valor == esperado:
                        if contexto:
                            contextuais.append(
                                (f"{relativo}:{numero}", f"{escrito} — contexto: {contexto}")
                            )
                        continue
                    motivos: list[str] = []
                    isento = _texto.isencao_por_data(linha, inicio_numero, data_secao, corte)
                    if isento:
                        origem, data = isento
                        motivos.append(
                            f"data na frase ({data})"
                            if origem == "frase"
                            else f"seção datada {data}"
                        )
                    marca = HISTORICO_RE.search(_texto.frase(linha, inicio_numero))
                    if marca:
                        motivos.append(f"marca de registro (`{marca.group(0).strip()}`)")
                    if motivos:
                        if contexto:
                            motivos.append(f"contexto: {contexto}")
                        historicas.append(
                            (f"{relativo}:{numero}", f"{escrito} — isenta: {'; '.join(motivos)}")
                        )
                        continue
                    problemas.append(
                        f"{relativo}:{numero}: diz `hoje {escrito}`; o dono mede {medido}"
                    )
    return problemas, historicas, contextuais, conferidas


def run(ws: Workspace) -> CheckResult:
    try:
        medida = Medida(ws)
        problemas, historicas, contextuais, conferidas = citacoes(ws, medida)
    except (RuntimeError, OSError, censos.Erro, capacidades.Erro, contagens.Erro) as erro:
        return CheckResult("hoje", False, f"erro ao medir: {erro}")
    # Nada é ignorado sem rastro: cada isenção sai com a data ou a marca que a sustenta.
    detalhes = [f"ignorado {local}: {motivo}" for local, motivo in historicas]
    detalhes += [f"ok {local}: {texto}" for local, texto in contextuais]
    resumo = (
        f"{conferidas} citação(ões) de `hoje` conferida(s) contra "
        f"{medida.valores['atos']} atos, {medida.valores['páginas']} páginas, "
        f"{medida.valores['células']} células, {medida.valores['Q-BLK']} Q-BLK, "
        f"{medida.valores['lacunas']} lacunas, "
        f"{medida.matriz[0]} de {medida.matriz[1]} capacidades, "
        f"{medida.valores['arquivos']} arquivos, {medida.valores['tabelas']} tabelas, "
        f"{medida.valores['fontes']} fontes e {medida.valores['páginas do site']} páginas do site; "
        f"{len(contextuais)} por contexto; {len(historicas)} isenta(s)"
    )
    if problemas:
        return CheckResult("hoje", False, resumo, problems=problemas, details=detalhes)
    return CheckResult("hoje", True, resumo, details=detalhes)
