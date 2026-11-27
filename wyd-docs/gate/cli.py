"""CLI única do gate de documentação.

Contrato:
- `full` monta o site, verifica links e roda todos os domínios;
- `fast` roda só os checks nativos (layout, alvos, skills, lacunas, contagens, censos,
  capacidades, hoje);
- `check` roda um ou mais domínios isolados.

Sem `--json`, imprime uma linha por domínio com estado, tempo e resumo curto.
Com `--json`, devolve o mesmo contrato em JSON legível por agente. Problemas
completos só aparecem com `--details`; falhas sempre mostram a contagem e uma
amostra curta para não despejar centenas de linhas no terminal.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import time
from pathlib import Path

from .checks import alvos, capacidades, censos, contagens, hoje, lacunas, layout, links, skills
from .model import CheckResult
from .workspace import Workspace

DOCS = Path(__file__).resolve().parents[1]
ROOT_PADRAO = DOCS.parent
DOMINIOS = (
    "layout",
    "alvos",
    "skills",
    "lacunas",
    "links",
    "contagens",
    "censos",
    "capacidades",
    "hoje",
)


def _runner(nome: str, ws: Workspace, site: Path | None = None) -> CheckResult:
    inicio = time.perf_counter()
    try:
        if nome == "layout":
            resultado = layout.run(ws)
        elif nome == "alvos":
            resultado = alvos.run(ws)
        elif nome == "skills":
            resultado = skills.run(ws)
        elif nome == "lacunas":
            resultado = lacunas.run(ws)
        elif nome == "links":
            resultado = links.run(ws, site)
        elif nome == "contagens":
            resultado = contagens.run(ws)
        elif nome == "censos":
            resultado = censos.run(ws)
        elif nome == "capacidades":
            resultado = capacidades.run(ws)
        elif nome == "hoje":
            resultado = hoje.run(ws)
        else:
            raise ValueError(f"domínio desconhecido: {nome}")
    except (SystemExit, Exception) as erro:  # o gate nunca esconde uma falha de leitura
        resultado = CheckResult(nome, False, f"erro ao executar: {erro}")
    resultado.duration_ms = int((time.perf_counter() - inicio) * 1000)
    return resultado


def _build(ws: Workspace) -> CheckResult:
    inicio = time.perf_counter()
    comandos = (
        [sys.executable, "preparar.py"],
        ["mkdocs", "build", "--dir", ".build/docs", "--output", "site"],
    )
    for comando in comandos:
        try:
            processo = subprocess.run(
                comando,
                cwd=DOCS,
                capture_output=True,
                text=True,
                check=False,
            )
        except FileNotFoundError:
            return CheckResult(
                "build",
                False,
                f"comando não encontrado: {comando[0]}",
                duration_ms=int((time.perf_counter() - inicio) * 1000),
            )
        if processo.returncode != 0:
            detalhe = (processo.stderr or processo.stdout or "").strip()
            return CheckResult(
                "build",
                False,
                f"falhou: {' '.join(comando)}",
                problems=[detalhe] if detalhe else [],
                duration_ms=int((time.perf_counter() - inicio) * 1000),
            )
    return CheckResult(
        "build",
        True,
        "montagem e MkDocs concluídos",
        duration_ms=int((time.perf_counter() - inicio) * 1000),
    )


def _executar(
    comando: str, dominios: tuple[str, ...], ws: Workspace, site: Path | None
) -> list[CheckResult]:
    resultados: list[CheckResult] = []
    if comando == "full":
        build = _build(ws)
        resultados.append(build)
        if not build.ok:
            return resultados
        site = site or (DOCS / "site")
    for nome in dominios:
        if nome == "links" and comando == "fast":
            continue
        resultados.append(_runner(nome, ws, site))
    return resultados


def _imprimir_humano(resultados: list[CheckResult], modo: str, detalhes: bool) -> None:
    ok = all(resultado.ok for resultado in resultados)
    total_ms = sum(resultado.duration_ms for resultado in resultados)
    print(f"gate: {'PASS' if ok else 'FAIL'} ({modo}, {total_ms / 1000:.1f}s)")
    for resultado in resultados:
        estado = "PASS" if resultado.ok else "FAIL"
        print(f"  {resultado.domain:<10} {estado:<4} {resultado.duration_ms / 1000:>6.1f}s  {resultado.summary}")
        if resultado.problems:
            print(f"    {len(resultado.problems)} problema(s); use --details para a lista completa")
            if detalhes or len(resultados) > 1:
                limite = None if detalhes else 5
                for problema in resultado.problems[:limite]:
                    print(f"    - {problema}")
                if limite is not None and len(resultado.problems) > limite:
                    print(f"    ... {len(resultado.problems) - limite} omitido(s)")
        if detalhes:
            for linha in resultado.details:
                print(f"    {linha}")


def _json(resultados: list[CheckResult], modo: str, detalhes: bool) -> str:
    payload = {
        "ok": all(resultado.ok for resultado in resultados),
        "mode": modo,
        "duration_ms": sum(resultado.duration_ms for resultado in resultados),
        "domains": [
            {
                "domain": resultado.domain,
                "ok": resultado.ok,
                "summary": resultado.summary,
                "duration_ms": resultado.duration_ms,
                "problems": resultado.problems,
                **({"details": resultado.details} if detalhes else {}),
            }
            for resultado in resultados
        ],
    }
    return json.dumps(payload, ensure_ascii=False, indent=2)


def _parse(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        prog="python -m gate",
        description="Gate único de documentação: resumo curto, --json e detalhes sob demanda.",
    )
    sub = parser.add_subparsers(dest="command", required=True)
    for nome, ajuda in (
        ("full", "monta o site e roda todos os domínios"),
        ("fast", "roda só os checks nativos, sem MkDocs/links"),
        ("check", "roda um ou mais domínios isolados"),
    ):
        cmd = sub.add_parser(nome, help=ajuda)
        cmd.add_argument("--root", type=Path, default=ROOT_PADRAO, help="raiz do repositório")
        cmd.add_argument("--json", action="store_true", help="saída JSON para agente")
        cmd.add_argument("--details", action="store_true", help="inclui detalhes completos")
        if nome == "check":
            cmd.add_argument("domains", nargs="+", choices=DOMINIOS)
        if nome != "fast":
            cmd.add_argument("--site", type=Path, default=None, help="diretório do site gerado")
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = _parse(list(sys.argv[1:] if argv is None else argv))
    ws = Workspace(args.root)
    if args.command == "check":
        dominios = tuple(args.domains)
    elif args.command == "fast":
        dominios = (
            "layout",
            "alvos",
            "skills",
            "lacunas",
            "contagens",
            "censos",
            "capacidades",
            "hoje",
        )
    else:
        dominios = DOMINIOS
    resultados = _executar(args.command, dominios, ws, getattr(args, "site", None))
    if args.json:
        print(_json(resultados, args.command, args.details))
    else:
        _imprimir_humano(resultados, args.command, args.details)
    return 0 if all(resultado.ok for resultado in resultados) else 1


if __name__ == "__main__":
    raise SystemExit(main())
