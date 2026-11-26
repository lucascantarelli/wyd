"""Contrato de resultado de um domínio do gate."""

from __future__ import annotations

from dataclasses import dataclass, field


@dataclass
class CheckResult:
    """Resultado de um domínio, com resumo curto e detalhes sob demanda."""

    domain: str
    ok: bool
    summary: str
    problems: list[str] = field(default_factory=list)
    details: list[str] = field(default_factory=list)
    duration_ms: int = 0

    @property
    def code(self) -> int:
        return 0 if self.ok else 1
