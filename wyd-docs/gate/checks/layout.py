#!/usr/bin/env python3
r"""Gate de layout: cada artefato no lugar que o planejamento determina.

A página autoritativa é `planejamento-1.0.0/wyd-ops/docs/estrutura-de-pastas-e-modulos.md`
(derivada de `planejamento-1.0.0/wyd-1.0.0.md` §8). Este check transforma a regra em
verificação executável: varre a árvore de trabalho, ignora só o que é gerado/legado e
reprova qualquer caminho fora das zonas previstas.

O que ele mede:

1. topo: só os arquivos e pastas listados em `RAIZ_*`; `scripts/` e `deny.toml` na raiz
   reprovam (os scripts são de `wyd-ops/scripts/` e a política do `cargo deny` é de
   `wyd-ops/deny.toml`);
2. planejamento: só Markdown/TSV de planejamento, dentro de `<sistema>/`, e nenhum
   executável, manifesto, Dockerfile ou Compose;
3. código: crates em `crates/<sistema>/`, portal em `wyd-portal/`, site em `wyd-docs/`,
   operação em `wyd-ops/`, cliente em `wyd-client/`; Dockerfile só em `wyd-ops/`;
4. estruturas obrigatórias: manifesto/lock/lib de cada crate, bins de server/admin/signer,
   migrations do db, página do portal, gate do site e os quatro Dockerfiles;
5. `.gitkeep` só em pasta reservada de verdade (planejamento `src|tests`, módulos ainda
   vazios do portal e `wyd-client/tools/`);
6. Compose: todo `dockerfile:` aponta para arquivo existente e o base não tem `build:`.

O check não lê `.env`, não segue `deprecated/`, `target/`, `node_modules/`, `site/`,
`.build/` nem `wyd-client/files/`. Falha lista `caminho: motivo`; exit 1.
"""

from __future__ import annotations

import os
import re
from pathlib import Path, PurePosixPath

from ..model import CheckResult
from ..workspace import Workspace

SISTEMAS = {
    "wyd-db",
    "wyd-launcher",
    "wyd-protocol",
    "wyd-net",
    "wyd-core",
    "wyd-sim",
    "wyd-server",
    "wyd-admin",
    "wyd-bots",
    "wyd-client",
    "wyd-docs",
    "wyd-ops",
    "wyd-portal",
}
# O `wyd-ops` é frente de processo, não crate; os demais nomes de sistema também não
# são todos crates. A lista de crates é a que existe em `crates/`.
CRATES = {
    "wyd-db",
    "wyd-ops-signer",
    "wyd-launcher",
    "wyd-protocol",
    "wyd-net",
    "wyd-core",
    "wyd-sim",
    "wyd-server",
    "wyd-admin",
    "wyd-bots",
}
CRATES_COM_MAIN = {"wyd-ops-signer", "wyd-server", "wyd-admin"}

RAIZ_ARQUIVOS = {
    ".dockerignore",
    ".env.example",
    ".gitattributes",
    ".gitignore",
    ".pre-commit-config.yaml",
    "AGENTS.md",
    "LICENSE",
    "Makefile",
    "README.md",
    "docker-compose.dev.yml",
    "docker-compose.override.yml",
    "docker-compose.yml",
    "rust-toolchain.toml",
}
RAIZ_DIRS = {
    ".agents",
    ".github",
    ".vscode",
    "crates",
    "planejamento-1.0.0",
    "wyd-client",
    "wyd-docs",
    "wyd-ops",
    "wyd-portal",
}
DOCKERFILES = {
    "wyd-ops/wyd-docs.Dockerfile",
    "wyd-ops/wyd-server.Dockerfile",
    "wyd-ops/wyd-admin.Dockerfile",
    "wyd-ops/wyd-portal.Dockerfile",
}
PORTAL_MODULOS = {
    "admin",
    "auth",
    "banco",
    "bff",
    "cache",
    "monitoramento",
    "observabilidade",
}
PORTAL_TESTES = {"acessibilidade", "contrato", "e2e", "integracao"}
IGNORAR_DIRS = {
    ".git",
    "deprecated",
    "target",
    "node_modules",
    "site",
    ".build",
    ".venv",
    "__pycache__",
    ".next",
    ".tmp",
    ".freebuff",
    "coverage",
    "dist",
    "out",
}
EXECUTAVEIS = {
    ".rs",
    ".sql",
    ".ts",
    ".tsx",
    ".js",
    ".jsx",
    ".mjs",
    ".py",
    ".sh",
    ".ps1",
    ".exe",
    ".dll",
    ".so",
    ".dylib",
}
# Exceções de `.gitkeep`: pasta reservada, não implementada.
RESERVADOS = (
    {f"planejamento-1.0.0/{s}/src/.gitkeep" for s in SISTEMAS}
    | {f"planejamento-1.0.0/{s}/tests/.gitkeep" for s in SISTEMAS}
    | {"wyd-client/tools/.gitkeep", "wyd-portal/public/.gitkeep"}
    | {f"wyd-portal/src/{m}/.gitkeep" for m in PORTAL_MODULOS}
    | {f"wyd-portal/tests/{t}/.gitkeep" for t in PORTAL_TESTES}
)


def _ignorar(rel: PurePosixPath) -> bool:
    partes = rel.parts
    if any(p in IGNORAR_DIRS for p in partes):
        return True
    if rel.parts[:2] == ("wyd-client", "files"):
        return True
    if rel.parts[:2] == (".agents", "memory"):
        return True
    if rel.name == ".env" or (rel.name.startswith(".env.") and rel.name != ".env.example"):
        return True
    return False


def _arquivos(root: Path) -> list[PurePosixPath]:
    encontrados: list[PurePosixPath] = []
    for pasta, subpastas, arquivos in os.walk(root):
        rel_pasta = Path(pasta).relative_to(root)
        # Recalcula com PurePosixPath para a regra de `wyd-client/files/`.
        mantidas = []
        for d in subpastas:
            rel = PurePosixPath(*(rel_pasta.parts + (d,)))
            if not _ignorar(rel):
                mantidas.append(d)
        subpastas[:] = mantidas
        for nome in arquivos:
            rel = PurePosixPath(*(rel_pasta.parts + (nome,)))
            if not _ignorar(rel):
                encontrados.append(rel)
    return sorted(encontrados, key=str)


def _erro_top(rel: PurePosixPath) -> str | None:
    if len(rel.parts) == 1:
        if rel.name in RAIZ_ARQUIVOS:
            return None
        return "arquivo de raiz fora da lista de ferramentas"
    return None


def _gitkeep_reservado(rel: PurePosixPath) -> bool:
    if str(rel) in RESERVADOS:
        return True
    if rel.parts[0] != "planejamento-1.0.0":
        return False
    if len(rel.parts) == 4 and rel.parts[2] in {"src", "tests", "prompts", "skills"}:
        return True
    if len(rel.parts) >= 6 and rel.parts[2:4] == ("docs", "specs") and rel.parts[4] in {"estados", "fluxos"}:
        return True
    return False


def validar_caminho(rel: PurePosixPath) -> str | None:
    """Regra pura de zona; devolve o motivo do desvio ou `None`."""
    if rel.name == ".gitkeep" and not _gitkeep_reservado(rel):
        return "`.gitkeep` fora de pasta reservada"

    top = rel.parts[0]
    if top not in RAIZ_ARQUIVOS and top not in RAIZ_DIRS:
        return "topo fora da árvore prevista"

    if top == "planejamento-1.0.0":
        if len(rel.parts) == 2:
            if rel.name in {"wyd-1.0.0.md", "status-planejamento.md"}:
                return None
            return "arquivo solto na raiz do planejamento; use `<sistema>/`"
        if rel.parts[1] not in SISTEMAS:
            return "sistema de planejamento não previsto"
        if rel.suffix.lower() in EXECUTAVEIS:
            return "artefato executável dentro de planejamento-1.0.0/"
        if rel.name in {"Cargo.toml", "Cargo.lock", "package.json", "docker-compose.yml"}:
            return "manifesto/Compose dentro de planejamento-1.0.0/"
        if "Dockerfile" in rel.name:
            return "Dockerfile dentro de planejamento-1.0.0/"
        return None

    if top == "crates":
        if len(rel.parts) < 2 or rel.parts[1] not in CRATES:
            return "crate fora de `crates/<sistema>/` previsto"
        if "Dockerfile" in rel.name or rel.name in {"docker-compose.yml", "package.json"}:
            return "Dockerfile/Compose/portal dentro de crate"
        return None

    if top == "wyd-portal":
        if len(rel.parts) == 2 and rel.name in {
            "next.config.mjs",
            "package.json",
            "package-lock.json",
        }:
            return None
        if rel.parts[1] not in {"src", "tests", "public"}:
            return "pasta do portal fora de src/tests/public"
        if rel.parts[1] == "src" and len(rel.parts) >= 3:
            modulo = rel.parts[2]
            if modulo != "app" and modulo not in PORTAL_MODULOS:
                return "módulo do portal fora da árvore do sistema"
        if rel.parts[1] == "tests" and len(rel.parts) >= 3:
            if rel.parts[2] not in PORTAL_TESTES:
                return "tipo de teste do portal fora do plano"
        if "Dockerfile" in rel.name or rel.name in {"docker-compose.yml"}:
            return "Dockerfile/Compose dentro do portal; o dono é wyd-ops/"
        return None

    if top == "wyd-docs":
        if rel.parts[1] not in {"docs", "gate", "tema", "tests"} and rel.name not in {
            "gate.sh",
            "preparar.py",
            "requirements.txt",
            "redirects.tsv",
        }:
            if not re.fullmatch(r"verificar.*\.py", rel.name):
                return "arquivo do site fora de docs/gate/tema/tests ou dos scripts de entrada"
        if rel.suffix.lower() in {".md"} and rel.parts[1] != "docs":
            return "Markdown de site fora de wyd-docs/docs/"
        return None

    if top == "wyd-ops":
        if len(rel.parts) == 1:
            return "arquivo solto em wyd-ops/; use Dockerfile/scripts/tests"
        if "Dockerfile" in rel.name or rel.name == "deny.toml":
            return None
        if rel.parts[1] in {"scripts", "tests"}:
            return None
        return "arquivo de ops fora de scripts/, tests/ ou *.Dockerfile"

    if top == "wyd-client":
        if rel.parts[1] not in {"tools"}:
            return "arquivo do cliente fora de tools/ (files/ é ignorado)"
        return None

    if top == ".github":
        if rel.parts[1] == "workflows" and rel.suffix in {".yml", ".yaml"}:
            return None
        if (
            rel.parts[1] == "actions"
            and len(rel.parts) == 4
            and rel.parts[3] == "action.yml"
        ):
            return None
        if rel.parts[1] == "ISSUE_TEMPLATE" and rel.suffix in {".yml", ".yaml"}:
            return None
        if len(rel.parts) == 2 and rel.name in {"dependabot.yml", "pull_request_template.md"}:
            return None
        return "arquivo de .github/ fora de workflows/actions/ISSUE_TEMPLATE"

    if top == ".vscode":
        if rel.suffix == ".json":
            return None
        return "config de IDE fora de JSON"

    if top == ".agents":
        if len(rel.parts) == 2 and rel.name == "README.md":
            return None
        if rel.parts[1] in {"prompts", "skills"} and rel.suffix == ".md":
            return None
        return "arquivo de .agents/ fora de prompts/skills/README"

    return None


def _exigir(root: Path, rel: str, problemas: list[str], motivo: str) -> None:
    if not (root / rel).exists():
        problemas.append(f"{rel}: ausente ({motivo})")


def _checar_estruturas(root: Path, problemas: list[str]) -> None:
    _exigir(root, "planejamento-1.0.0/wyd-1.0.0.md", problemas, "panorama")
    _exigir(root, "planejamento-1.0.0/status-planejamento.md", problemas, "status")
    for sistema in sorted(SISTEMAS):
        _exigir(
            root,
            f"planejamento-1.0.0/{sistema}/{sistema}-1.0.0.md",
            problemas,
            "dashboard do sistema",
        )
    for crate in sorted(CRATES):
        base = f"crates/{crate}"
        _exigir(root, f"{base}/Cargo.toml", problemas, "manifesto do crate")
        _exigir(root, f"{base}/Cargo.lock", problemas, "lock do crate")
        _exigir(root, f"{base}/src/lib.rs", problemas, "API pública do crate")
        if crate in CRATES_COM_MAIN:
            _exigir(root, f"{base}/src/main.rs", problemas, "binário do crate")
    _exigir(root, "crates/wyd-db/migrations", problemas, "série SQL do wyd-db")

    _exigir(root, "wyd-portal/package.json", problemas, "manifesto do portal")
    _exigir(root, "wyd-portal/package-lock.json", problemas, "lock do portal")
    _exigir(root, "wyd-portal/src/app/page.jsx", problemas, "página inicial do portal")
    _exigir(root, "wyd-portal/src/app/layout.jsx", problemas, "layout do portal")

    _exigir(root, "wyd-docs/preparar.py", problemas, "montagem do site")
    _exigir(root, "wyd-docs/gate.sh", problemas, "entrada do gate")
    _exigir(root, "wyd-docs/gate/checks", problemas, "checks do gate")
    _exigir(root, "wyd-docs/tests", problemas, "testes do site")

    for dockerfile in sorted(DOCKERFILES):
        _exigir(root, dockerfile, problemas, "imagem do serviço")
    _exigir(root, "wyd-ops/scripts/ci/db-prepare.sh", problemas, "script de CI")
    _exigir(root, "wyd-ops/scripts/ci/apply-series.sh", problemas, "script de CI")
    _exigir(root, "wyd-ops/scripts/sync-issue-status.mjs", problemas, "script de issues")
    _exigir(root, "wyd-ops/deny.toml", problemas, "política do cargo-deny")
    _exigir(root, "wyd-client/tools/.gitkeep", problemas, "reserva da CLI de captura")

    for nome in ("settings.json", "launch.json", "tasks.json", "extensions.json"):
        _exigir(root, f".vscode/{nome}", problemas, "config de IDE")


def _checar_dockerfiles(root: Path, problemas: list[str]) -> None:
    for caminho in root.rglob("*Dockerfile*"):
        rel = caminho.relative_to(root).as_posix()
        if rel in DOCKERFILES:
            continue
        if _ignorar(PurePosixPath(rel)):
            continue
        problemas.append(f"{rel}: Dockerfile fora de wyd-ops/ com nome previsto")


def _checar_compose(root: Path, problemas: list[str]) -> None:
    for nome in ("docker-compose.yml", "docker-compose.override.yml", "docker-compose.dev.yml"):
        caminho = root / nome
        if not caminho.exists():
            problemas.append(f"{nome}: Compose ausente")
            continue
        texto = caminho.read_text(encoding="utf-8")
        for achado in re.finditer(r"dockerfile:\s*[\"']?([^\"'\s]+)", texto):
            alvo = root / achado.group(1)
            if not alvo.is_file():
                problemas.append(f"{nome}: dockerfile inexistente `{achado.group(1)}`")
    base = root / "docker-compose.yml"
    if base.exists() and re.search(r"^\s*build:", base.read_text(encoding="utf-8"), re.M):
        problemas.append("docker-compose.yml: o Compose base não pode ter `build:`")


def _checar_referencias(root: Path, problemas: list[str]) -> None:
    """Referências que voltariam a apontar para a raiz antiga."""
    nomes = ["Makefile", ".vscode/tasks.json", ".vscode/launch.json"]
    nomes += [str(p.relative_to(root)) for p in sorted((root / ".github/workflows").glob("*.yml"))]
    for nome in nomes:
        caminho = root / nome
        if not caminho.is_file():
            continue
        for numero, linha in enumerate(caminho.read_text(encoding="utf-8").splitlines(), 1):
            if re.search(r"(?<!wyd-ops/)\bscripts/ci/", linha):
                problemas.append(f"{nome}:{numero}: referência à raiz `scripts/ci/` (use wyd-ops/scripts/ci/)")


def run(ws: Workspace) -> CheckResult:
    root = ws.root
    if not (root / "planejamento-1.0.0").is_dir():
        return CheckResult("layout", False, f"não encontrei planejamento-1.0.0/ em {root}")

    problemas: list[str] = []
    arquivos = _arquivos(root)
    for rel in arquivos:
        motivo = _erro_top(rel)
        if motivo is None:
            motivo = validar_caminho(rel)
        if motivo:
            problemas.append(f"{rel}: {motivo}")

    _checar_estruturas(root, problemas)
    _checar_dockerfiles(root, problemas)
    _checar_compose(root, problemas)
    _checar_referencias(root, problemas)

    resumo = f"{len(arquivos)} arquivo(s) classificado(s); {len(problemas)} desvio(s)"
    if problemas:
        return CheckResult("layout", False, resumo, problems=problemas)
    return CheckResult("layout", True, resumo)
