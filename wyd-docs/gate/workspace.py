"""Índice compartilhado da árvore, com cache de texto e de listas de arquivos.

Os verificadores liam a mesma árvore dezenas de vezes. O `Workspace` é criado uma
vez pelo CLI e emprestado a cada check; a árvore não muda durante uma execução,
então o cache é seguro e não altera nenhuma regra.

O índice por nome de arquivo existe para resolver citações do tipo
`caminho/arquivo.md` sem varrer o repositório a cada lacuna. A resolução direta
(`root / caminho`) tem prioridade; o índice é apenas o fallback que o gate já
usava.
"""

from __future__ import annotations

import os
from pathlib import Path

IGNORAR_DIR = {".git", "target", "node_modules", "site", ".build", "dist", "coverage"}


class Workspace:
    """Visão imutável da raiz, com caches por caminho e por glob."""

    def __init__(self, root: Path | str) -> None:
        self.root = Path(root).resolve()
        self._textos: dict[tuple[Path, str], str] = {}
        self._arquivos: dict[tuple[Path, str], tuple[Path, ...]] = {}
        self._existe: dict[Path, bool] = {}
        self._diretorio: dict[Path, bool] = {}
        self._indice: dict[str, Path] | None = None

    def read(self, caminho: Path, errors: str = "strict") -> str:
        chave = (caminho, errors)
        if chave not in self._textos:
            self._textos[chave] = caminho.read_text(encoding="utf-8", errors=errors)
        return self._textos[chave]

    def files(self, base: Path, pattern: str = "*.md") -> tuple[Path, ...]:
        chave = (base, pattern)
        if chave not in self._arquivos:
            self._arquivos[chave] = tuple(sorted(base.rglob(pattern)))
        return self._arquivos[chave]

    def exists(self, caminho: Path) -> bool:
        if caminho not in self._existe:
            self._existe[caminho] = caminho.exists()
        return self._existe[caminho]

    def is_dir(self, caminho: Path) -> bool:
        if caminho not in self._diretorio:
            self._diretorio[caminho] = caminho.is_dir()
        return self._diretorio[caminho]

    def index(self) -> dict[str, Path]:
        if self._indice is None:
            indice: dict[str, Path] = {}
            for pasta, subpastas, arquivos in os.walk(self.root):
                subpastas[:] = [d for d in subpastas if d not in IGNORAR_DIR]
                for nome in arquivos:
                    indice.setdefault(nome, Path(pasta) / nome)
            self._indice = indice
        return self._indice

    def find(self, citado: str) -> Path | None:
        """Resolve a citação relativa como o gate sempre fez: direto, depois por nome."""
        alvo = citado.split("#", 1)[0]
        direto = self.root / alvo
        if self.exists(direto) and direto.is_file():
            return direto
        return self.index().get(Path(alvo).name)

    def relative(self, caminho: Path) -> str:
        return caminho.relative_to(self.root).as_posix()
