"""Leitura compartilhada dos checks de citação: linha lógica, frase e data que qualifica.

Os checks de citação (`capacidades`, `censos`, `hoje`, `lacunas`) precisam das mesmas coisas:
juntar a prosa quebrada em várias linhas sem perder o número da primeira, delimitar a frase de uma
citação (ponto seguido de espaço ou fim, para `arquivo.md` e `§6.6.22.1` não cortarem) e decidir se
uma data anterior à medição do dono está na mesma frase ou na seção do número.

A gramática de "número seguido da unidade" (`138 atos`, `20 de 39 capacidades`) não é compartilhada
aqui: o `hoje` e o `censos` leem unidades diferentes (o `hoje` todas, com atribuição contextual; o
`censos` só `atos`, com negrito e recorte de pasta/leva), então cada check mantém a sua. O que é
comum e fica aqui é a isenção por data.
"""

from __future__ import annotations

import re

DATA_RE = re.compile(r"\b(20\d\d-\d\d-\d\d)\b")


def linhas_logicas(texto: str) -> list[tuple[int, str]]:
    """Junta a quebra de linha de prosa; citação partida no fim da linha continua citada."""
    brutas = texto.splitlines()
    saida: list[tuple[int, str]] = []
    i = 0
    while i < len(brutas):
        linha = brutas[i]
        numero = i + 1
        i += 1
        if not linha.strip() or linha.startswith(("#", "|", ">", "```")):
            saida.append((numero, linha))
            continue
        while i < len(brutas):
            seguinte = brutas[i]
            if (
                not seguinte.strip()
                or seguinte.startswith(("#", "|", ">", "```"))
                or re.match(r"\s*(?:[*-]\s|\d+\.\s)", seguinte)
            ):
                break
            if re.search(r"[.!?:;]\s*$", linha):
                break
            linha = linha.rstrip() + " " + seguinte.strip()
            i += 1
        saida.append((numero, linha))
    return saida


def data_da_secao(pilha: list[tuple[int, str | None]]) -> str | None:
    """Data mais próxima na pilha de cabeçalhos; subseção herda a do pai."""
    for _, data in reversed(pilha):
        if data:
            return data
    return None


def frases(linha: str) -> list[tuple[int, int]]:
    """Spans de frase: ponto só fecha quando seguido de espaço ou fim."""
    spans: list[tuple[int, int]] = []
    comeco = 0
    for indice, caractere in enumerate(linha):
        if caractere in ".!?" and (indice + 1 == len(linha) or linha[indice + 1].isspace()):
            spans.append((comeco, indice + 1))
            comeco = indice + 1
    if comeco < len(linha):
        spans.append((comeco, len(linha)))
    return spans


def frase(linha: str, inicio: int) -> str:
    for comeco, fim in frases(linha):
        if comeco <= inicio < fim:
            return linha[comeco:fim]
    return linha


def datas_que_qualificam(linha: str, inicio: int) -> list[str]:
    """Só a frase que contém a citação qualifica; data em outra frase não isenta."""
    return DATA_RE.findall(frase(linha, inicio))


def isencao_por_data(
    linha: str, inicio: int, data_secao: str | None, corte: str | None
) -> tuple[str, str] | None:
    """`(origem, data)` quando a data anterior ao corte está na frase ou na seção da citação."""
    if not corte:
        return None
    anteriores = [data for data in datas_que_qualificam(linha, inicio) if data < corte]
    if anteriores:
        return "frase", min(anteriores)
    if data_secao and data_secao < corte:
        return "seção", data_secao
    return None
