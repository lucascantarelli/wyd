#!/usr/bin/env python3
r"""Gate de skills: a página de issue é dona do valor e o roadmap só cita o bloco §Skills.

Medida (fonte: os arquivos de `planejamento-1.0.0/<sistema>/docs/roadmap/`, lidos como texto):

1. **roadmap não declara skill por nome** — nenhuma linha de roadmap pode ter a célula
   `Skill principal` / `Skills complementares` (ou o cabeçalho com essas colunas). O roadmap cita o
   link para `§Skills` da página; o nome da skill vive só lá.
2. **citação resolve** — cada `[<ID>](ID.md) §Skills` aponta para arquivo existente na mesma pasta,
   o arquivo tem `## Skills`, o ID do título do alvo é o mesmo do link e, se a citação trouxer um
   segundo identificador em `código` (o valor), esse valor existe no bloco da página.
3. **página de issue segue o padrão único** — todo `WYD*.md` da pasta de roadmap tem o cabeçalho de
   cinco campos, `**Estado:**`, o campo `**Gate de verificação:**`, uma fase ou fatia, as sete seções
   do template, `## Skills` com skills da lista fechada e, depois dele, a linha `**Dono do valor:**`.
   A explicação das nove skills não pode ser repetida na página; ela vive no template.
4. **arquivo de IA cita o fato normativo, não o publica** — AGENTS.md e as skills não repetem o
   comando do gate, a lista/contagem de sistemas, contagem de ADR, de migration/tabela/seed, o
   limiar de cobertura nem o formato do cabeçalho: cada um tem dono em documento de planejamento e
   a linha que o republicar com valor reprova aqui.
5. **roadmap não repete o detalhe da fatia** — fase, requisitos, dependência, entregável, DoD e
   fora de escopo são da página da issue; nenhuma linha de `roadmap.md` ou `fases.md` pode ter a
   célula `Fase`, `Requisitos`, `Depende de`, `Entrega`, `DoD` ou `Fora` com valor ao lado. O
   `roadmap.md` cita a página de cada fatia na tabela de ordem, com `§Skills`, e toda página
   `WYD*-F<numero>.md` da pasta aparece nessa citação. `index.md` é catálogo: não repete fila, fase,
   prioridade, bloqueio nem estado — aponta o campo da página.

O que estiver fora de `docs/roadmap/` não é medido: `lacunas.md`, `proposta-*` e as páginas de
sistema continuam livres para narrar a decisão. Não há número de planejamento aqui — só a regra de
propriedade do bloco de skills.

Falha (exit 1) listando `arquivo:linha: motivo`; exit 2 quando a pasta de planejamento não existe.

Uso:
    python -m gate check skills --root ..   # a partir de wyd-docs/
"""

from __future__ import annotations

import re
from pathlib import Path

from ..model import CheckResult
from ..workspace import Workspace

PLANEJAMENTO = "planejamento-1.0.0"
PASTA_ROADMAP = "docs/roadmap"

_WS: Workspace | None = None


def _ws() -> Workspace:
    if _WS is None:
        raise RuntimeError("skills: Workspace não inicializado")
    return _WS

# Célula de skill declarada por nome no roadmap: é exatamente o que a regra proíbe.
CELULA_SKILL = re.compile(r"^\s*\|\s*Skills? (?:principal|complementares)\s*\|", re.IGNORECASE)
# Cabeçalho de tabela com a coluna de skill por nome.
COLUNA_SKILL = re.compile(r"\|\s*Skills? (?:principal|complementares)\s*\|", re.IGNORECASE)
# Célula que repete o detalhe da fatia: o dono é a página da issue.
CELULA_DETALHE = re.compile(
    r"^\s*\|\s*(Fase|Requisitos|Depende de|Entrega|DoD|Fora)\s*\|\s*[^|]+\|\s*$",
    re.IGNORECASE,
)
# Link para outra página da mesma pasta.
LINK = re.compile(r"\]\(([^)\s#]+\.md)\)")
PAGINA_FATIA = re.compile(r"^WYD.*-F\d+\.md$")
# Arquivos que são o roteiro do sistema; `index.md` é catálogo e não entra na regra de detalhe.
ROTEIROS = {"roadmap.md", "fases.md"}
# Arquivos de IA: instrução de operação do agente, não cópia do fato normativo.
ARQUIVOS_DE_IA = ["AGENTS.md"]
PASTA_SKILLS = ".agents/skills"
# Fato com dono: (padrão que republica o valor, dono, motivo curto).
FATOS_COM_DONO = [
    (
        re.compile(r"docker compose --profile gate run --rm gate"),
        "planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md \u00A76",
        "comando do gate",
    ),
    (
        re.compile(r"\b1[13-5]\s+ADRs?\b|\bdois\s+ADRs?\b"),
        "planejamento-1.0.0/wyd-ops/docs/arquitetura/adr/index.md \u00A72",
        "contagem de ADRs",
    ),
    (
        re.compile(r"\b13\s+sistemas\b"),
        "planejamento-1.0.0/wyd-1.0.0.md \u00A71",
        "lista/contagem de sistemas",
    ),
    (
        re.compile(r"\b\d+\s+(?:migrations?|tabelas?|seeds?)\b"),
        "planejamento-1.0.0/wyd-db/plano-migrations.md \u00A73",
        "contagem de migration/tabela/seed",
    ),
    (
        re.compile(r"\b85\s*%"),
        "planejamento-1.0.0/wyd-ops/docs/planos/estrategia-de-testes.md \u00A76",
        "limiar de cobertura",
    ),
    (
        re.compile(r"cabe\u00E7alho de (?:cinco|5) linhas|cinco linhas do cabe\u00E7alho"),
        "planejamento-1.0.0/wyd-db/docs/convencoes.md \u00A7Cabe\u00E7alho de p\u00E1gina",
        "formato do cabe\u00E7alho",
    ),
]
# Citação: [texto](alvo.md) §Skills
CITACAO = re.compile(r"\[([^\]]*)\]\(([^)\s#]+\.md)\)\s*((?:`[^`]+`\s*)*)§Skills")
CODIGO = re.compile(r"`([^`]+)`")
TITULO = re.compile(r"^#\s+([^\s—]+)\s+—")
SECAO_SKILLS = "## Skills"
PRINCIPAL = re.compile(r"\|\s*`skill_principal`\s*\|\s*`([^`]+)`\s*\|")
COMPLEMENTARES = re.compile(r"\|\s*`skills_complementares`\s*\|\s*(.+?)\s*\|")
DONO = "**Dono do valor:**"
GATE = "**Gate de verificação:**"
CABECALHO = (
    r"^> Pergunta:",
    r"^> Público:",
    r"^> Dono:",
    r"^> Base:",
    r"^> Revisado em:",
)
SECOES = (
    "## Contexto",
    "## Entregável",
    "## DoD",
    "## Fora de escopo",
    "## Depende de",
    SECAO_SKILLS,
    "## Evidência ao fechar",
)
SKILLS = {
    "auditar-planejamento",
    "documentation-writer",
    "finishing-a-development-branch",
    "frontend-design",
    "github-actions-docs",
    "planejar-sistema",
    "python-testing-patterns",
    "skill-creator",
    "writing-plans",
}
EXPLICACAO_REPETIDA = "A skill principal é a que o agente carrega"


def _linha(texto: str, posicao: int) -> int:
    return texto.count("\n", 0, posicao) + 1


def _checar_roadmap(
    caminho: Path, nome: str, problemas: list[str], detalhe: bool = True
) -> int:
    texto = _ws().read(caminho)
    linhas = texto.splitlines()
    citacoes = 0
    for numero, linha in enumerate(linhas, start=1):
        repetido = CELULA_DETALHE.match(linha) if detalhe else None
        if repetido:
            problemas.append(
                f"{nome}:{numero}: roadmap repete `{repetido.group(1)}` da fatia; "
                "a página da issue é dona do valor e o roadmap cita o link"
            )
        if CELULA_SKILL.match(linha) or (
            COLUNA_SKILL.search(linha) and not CITACAO.search(linha)
        ):
            problemas.append(
                f"{nome}:{numero}: roadmap declara a skill por nome; cite o link para §Skills da página"
            )
        for achado in CITACAO.finditer(linha):
            citacoes += 1
            etiqueta, alvo = achado.group(1), achado.group(2)
            destino = (caminho.parent / alvo).resolve()
            if not _ws().exists(destino) or not destino.is_file():
                problemas.append(f"{nome}:{numero}: citação para página inexistente {alvo}")
                continue
            pagina = _ws().read(destino)
            if SECAO_SKILLS not in pagina:
                problemas.append(f"{nome}:{numero}: {alvo} não tem bloco {SECAO_SKILLS}")
                continue
            tokens = CODIGO.findall(etiqueta)
            if not tokens:
                problemas.append(f"{nome}:{numero}: citação sem o ID da página em `código`")
                continue
            titulo = TITULO.search(pagina)
            if not titulo:
                problemas.append(f"{nome}:{numero}: {alvo} não tem título `# ID — nome`")
                continue
            if tokens[0] != titulo.group(1):
                problemas.append(
                    f"{nome}:{numero}: citação de `{tokens[0]}` aponta para {alvo}, cujo título é `{titulo.group(1)}`"
                )
            bloco = pagina.split(SECAO_SKILLS, 1)[1]
            aceitos: set[str] = set()
            principal = PRINCIPAL.search(bloco)
            if principal:
                aceitos.add(principal.group(1))
            complementares = COMPLEMENTARES.search(bloco)
            if complementares:
                aceitos.update(CODIGO.findall(complementares.group(1)))
            for valor in tokens[1:] + CODIGO.findall(achado.group(3)):
                if valor not in aceitos:
                    problemas.append(
                        f"{nome}:{numero}: valor `{valor}` não é a skill do bloco {SECAO_SKILLS} de {alvo}"
                    )
    return citacoes


def _checar_paginas(caminho: Path, nome: str, problemas: list[str]) -> int:
    texto = _ws().read(caminho)
    for padrao in CABECALHO:
        if not re.search(padrao, texto, re.M):
            problemas.append(
                f"{nome}:1: página de issue sem o campo de cabeçalho `{padrao[2:-1]}`"
            )
    if "**Estado:**" not in texto:
        problemas.append(f"{nome}:1: página de issue sem `**Estado:**`")
    if GATE not in texto:
        problemas.append(f"{nome}:1: página de issue sem `{GATE}`")
    if "**Fase:**" not in texto and "**Fatia:**" not in texto:
        problemas.append(f"{nome}:1: página de issue sem `**Fase:**` ou `**Fatia:**`")
    for secao in SECOES:
        if secao not in texto:
            problemas.append(f"{nome}:1: página de issue sem a seção `{secao}`")
    if SECAO_SKILLS not in texto:
        return 0
    if DONO not in texto:
        linha = _linha(texto, texto.index(SECAO_SKILLS))
        problemas.append(f"{nome}:{linha}: página de issue sem a linha {DONO} do bloco de skills")
    if EXPLICACAO_REPETIDA in texto:
        linha = _linha(texto, texto.index(EXPLICACAO_REPETIDA))
        problemas.append(
            f"{nome}:{linha}: página de issue repete a explicação das skills; "
            "o padrão e a lista fechada são do template-de-issue §3"
        )
    principal = PRINCIPAL.search(texto)
    if not principal:
        problemas.append(f"{nome}:1: página de issue sem `skill_principal`")
    elif principal.group(1) not in SKILLS:
        problemas.append(
            f"{nome}:{_linha(texto, principal.start())}: skill principal "
            f"`{principal.group(1)}` fora da lista fechada"
        )
    complementares = COMPLEMENTARES.search(texto)
    if complementares:
        for valor in CODIGO.findall(complementares.group(1)):
            if valor not in SKILLS:
                problemas.append(
                    f"{nome}:{_linha(texto, complementares.start())}: skill complementar "
                    f"`{valor}` fora da lista fechada"
                )
    return 1


def _checar_arquivos_de_ia(raiz: Path, problemas: list[str]) -> int:
    arquivos: list[Path] = []
    for nome in ARQUIVOS_DE_IA:
        caminho = raiz / nome
        if _ws().exists(caminho) and caminho.is_file():
            arquivos.append(caminho)
    pasta = raiz / PASTA_SKILLS
    if pasta.is_dir():
        arquivos.extend(_ws().files(pasta, "*.md"))
    for arquivo in arquivos:
        texto = _ws().read(arquivo)
        rel = arquivo.relative_to(raiz).as_posix()
        for padrao, dono, motivo in FATOS_COM_DONO:
            for achado in padrao.finditer(texto):
                problemas.append(
                    f"{rel}:{_linha(texto, achado.start())}: arquivo de IA publica {motivo} "
                    f"(\"{achado.group(0)}\") com valor; o dono é {dono} e aqui se cita, não se repete"
                )
    return len(arquivos)


def run(ws: Workspace) -> CheckResult:
    global _WS
    _WS = ws
    raiz = ws.root
    base = raiz / PLANEJAMENTO
    if not base.is_dir():
        return CheckResult("skills", False, f"não encontrei {PLANEJAMENTO}/ em {raiz}")

    problemas: list[str] = []
    arquivos_ia = _checar_arquivos_de_ia(raiz, problemas)
    roadmaps = 0
    citacoes = 0
    paginas = 0
    for sistema in sorted(p for p in base.iterdir() if p.is_dir()):
        pasta = sistema / PASTA_ROADMAP
        if not pasta.is_dir():
            continue
        for arquivo in sorted(pasta.glob("*.md")):
            nome = f"{PLANEJAMENTO}/{sistema.name}/{PASTA_ROADMAP}/{arquivo.name}"
            if arquivo.name.startswith("WYD"):
                paginas += _checar_paginas(arquivo, nome, problemas)
            else:
                roadmaps += 1
                citacoes += _checar_roadmap(
                    arquivo, nome, problemas, arquivo.name in ROTEIROS
                )
                if arquivo.name == "roadmap.md":
                    texto = ws.read(arquivo)
                    ligados = {alvo for alvo in LINK.findall(texto)}
                    for fatia in sorted(pasta.glob("*.md")):
                        if not PAGINA_FATIA.match(fatia.name):
                            continue
                        if fatia.name not in ligados:
                            problemas.append(
                                f"{nome}:1: página de fatia {fatia.name} não é citada pelo roadmap"
                            )

    resumo = (
        f"{roadmaps} roadmap(s) e {paginas} página(s) de issue "
        f"({citacoes} citações de §Skills; {arquivos_ia} arquivo(s) de IA)"
    )
    if problemas:
        return CheckResult("skills", False, resumo, problems=problemas)
    return CheckResult("skills", True, resumo)
