"""Testes dos verificadores e do contrato do CLI, com fixtures centralizadas."""

from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

from gate.checks import censos, contagens, lacunas, links, skills
from gate.workspace import Workspace
from tests import fixtures


def _repo(dados: dict[str, str]) -> Path:
    pasta = Path(tempfile.mkdtemp(prefix="gate-fixture-"))
    for relativo, conteudo in dados.items():
        destino = pasta / relativo
        destino.parent.mkdir(parents=True, exist_ok=True)
        destino.write_text(conteudo, encoding="utf-8")
    return pasta


class WorkspaceTests(unittest.TestCase):
    def test_indice_e_cache_compartilham_a_mesma_leitura(self):
        root = _repo(fixtures.LACUNA_OK)
        ws = Workspace(root)
        caminho = root / "planejamento-1.0.0/wyd-teste/evidencia.md"
        self.assertIs(ws.read(caminho), ws.read(caminho))
        self.assertEqual(ws.find("planejamento-1.0.0/wyd-teste/evidencia.md"), caminho)
        self.assertEqual(ws.find("evidencia.md"), caminho)


class LacunaTests(unittest.TestCase):
    def test_fechada_com_evidencia_passa(self):
        resultado = lacunas.run(Workspace(_repo(fixtures.LACUNA_OK)))
        self.assertTrue(resultado.ok, resultado.problems)

    def test_fechada_sem_evidencia_falha(self):
        resultado = lacunas.run(Workspace(_repo(fixtures.LACUNA_SEM_EVIDENCIA)))
        self.assertFalse(resultado.ok)
        self.assertTrue(resultado.problems)


class SkillTests(unittest.TestCase):
    def test_roadmap_cita_skills_sem_repetir_fase(self):
        resultado = skills.run(Workspace(_repo(fixtures.SKILLS_OK)))
        self.assertTrue(resultado.ok, resultado.problems)

    def test_roadmap_que_repete_fase_falha(self):
        resultado = skills.run(Workspace(_repo(fixtures.SKILLS_REPETE_FASE)))
        self.assertFalse(resultado.ok)
        self.assertTrue(any("Fase" in p for p in resultado.problems))


class ContagemTests(unittest.TestCase):
    def test_medidas_da_serie_coincidem(self):
        root = _repo(fixtures.CONTAGEM_OK)
        contagens._WS = Workspace(root)
        arquivos, tabelas = contagens.medir_checklist(root)
        self.assertEqual((arquivos, tabelas), (1, 2))
        self.assertEqual(contagens.medir_ddl(root), 1)
        self.assertEqual(contagens.medir_plano(root), 1)


class CensoTests(unittest.TestCase):
    def test_mede_atos_celulas_e_qblk(self):
        root = _repo(fixtures.CENSO_OK)
        censos._WS = Workspace(root)
        atos = censos.medir_atos(root)
        self.assertEqual(atos["total"], 2)
        self.assertEqual(censos.medir_celulas(root), (1, 1))
        self.assertEqual(censos.medir_qblk(root), ["Q-BLK-01"])


class LinkTests(unittest.TestCase):
    def _rodar(self, dados: dict[str, str]):
        root = _repo(dados)
        site = root / "site"
        site.mkdir(exist_ok=True)
        for nome, conteudo in dados.items():
            (site / nome).write_text(conteudo, encoding="utf-8")
        redirects = root / "wyd-docs/redirects.tsv"
        redirects.parent.mkdir(parents=True, exist_ok=True)
        redirects.write_text("", encoding="utf-8")
        return links.run(Workspace(root), site)

    def test_link_interno_existente_passa(self):
        resultado = self._rodar(fixtures.SITE_OK)
        self.assertTrue(resultado.ok, resultado.problems)

    def test_link_interno_quebrado_falha(self):
        resultado = self._rodar(fixtures.SITE_QUEBRADO)
        self.assertFalse(resultado.ok)
        self.assertTrue(any("falta.html" in p for p in resultado.problems))


class CliTests(unittest.TestCase):
    def test_check_json_devolve_contrato_legivel(self):
        root = _repo(fixtures.SKILLS_OK)
        processo = subprocess.run(
            [sys.executable, "-m", "gate", "check", "skills", "--root", str(root), "--json"],
            cwd=Path(__file__).resolve().parents[1],
            capture_output=True,
            text=True,
            check=False,
        )
        self.assertEqual(processo.returncode, 0, processo.stderr)
        payload = json.loads(processo.stdout)
        self.assertTrue(payload["ok"])
        self.assertEqual(payload["domains"][0]["domain"], "skills")
        self.assertIn("duration_ms", payload["domains"][0])


if __name__ == "__main__":
    unittest.main()
