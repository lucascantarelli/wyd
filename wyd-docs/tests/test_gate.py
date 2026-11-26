"""Testes dos verificadores e do contrato do CLI, com fixtures centralizadas."""

from __future__ import annotations

import json
import multiprocessing
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

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
            destino = site / nome
            destino.parent.mkdir(parents=True, exist_ok=True)
            destino.write_text(conteudo, encoding="utf-8")
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

    def test_link_para_diretorio_resolve_index(self):
        resultado = self._rodar(fixtures.SITE_DIRETORIO)
        self.assertTrue(resultado.ok, resultado.problems)

    def test_link_com_pai_e_normalizado(self):
        resultado = self._rodar(fixtures.SITE_RELATIVO)
        self.assertTrue(resultado.ok, resultado.problems)

    def test_diretorio_sem_index_falha(self):
        dados = dict(fixtures.SITE_DIRETORIO)
        dados.pop("pasta/index.html")
        resultado = self._rodar(dados)
        self.assertFalse(resultado.ok)
        self.assertTrue(any("pasta" in p for p in resultado.problems))


class SintaxeTests(unittest.TestCase):
    def _rodar(self, dados: dict[str, str]):
        root = _repo(dados)
        links._WS = Workspace(root)
        return links._checar_sintaxe(root / "planejamento-1.0.0")

    def test_sintaxe_citada_em_code_span_passa(self):
        problemas, arquivos = self._rodar(fixtures.SINTAXE_1X_CITADA)
        self.assertEqual((problemas, arquivos), ([], 1))

    def test_sintaxe_fora_de_code_span_reprova(self):
        problemas, _ = self._rodar(fixtures.SINTAXE_1X_USADA)
        self.assertTrue(any("sintaxe MkDocs 1.x (admonition)" in p for p in problemas))


class ParaleloTests(unittest.TestCase):
    @unittest.skipIf(
        sys.platform == "win32",
        "skip apenas do Windows: não há fork; Linux e macOS provam o caminho paralelo",
    )
    def test_paralelo_e_sequencial_iguais_em_falhas(self):
        """Linux e macOS (fork) executam os dois caminhos; só o Windows é skip.

        O teste é deliberadamente sensível à ordem: não ordena nem normaliza a lista
        antes de comparar. O volume de 32 páginas garante que `_analisar_todas` use
        processos em vez de cair no caminho sequencial. No Linux e no macOS, a ausência
        de `fork` reprova em vez de pular, para o gate não ficar verde sem provar o
        caminho.
        """
        self.assertIn(
            "fork",
            multiprocessing.get_all_start_methods(),
            "o gate Linux/macOS exige fork para provar o caminho paralelo",
        )
        dados = fixtures.site_com_falhas(32)
        root = _repo({"wyd-docs/redirects.tsv": ""})
        site = root / "site"
        for nome, conteudo in dados.items():
            (site / nome).parent.mkdir(parents=True, exist_ok=True)
            (site / nome).write_text(conteudo, encoding="utf-8")

        ws = Workspace(root)
        original = links._analisar_todas

        def sequencial(paginas, trabalhadores=None):
            return original(paginas, trabalhadores=1)

        def paralelo(paginas, trabalhadores=None):
            return original(paginas, trabalhadores=4)

        with mock.patch.object(links, "_analisar_todas", sequencial):
            esperado = links.run(ws, site)
        with mock.patch.object(links, "_analisar_todas", paralelo):
            obtido = links.run(ws, site)

        self.assertFalse(esperado.ok)
        self.assertEqual(obtido.ok, esperado.ok)
        self.assertEqual(len(esperado.problems), 32 * 3)
        self.assertEqual(obtido.problems, esperado.problems)

    def test_exportar_importar_preserva_coletor(self):
        root = _repo({})
        caminho = root / "pagina.html"
        caminho.write_text(fixtures.SITE_OK["index.html"], encoding="utf-8")
        links._WS = Workspace(root)
        original = links._analisar(caminho)
        copia = links.Coletor.importar(original.exportar())
        self.assertEqual(copia.exportar(), original.exportar())

    def test_cli_processa_site_grande_e_mantem_resumo(self):
        dados = fixtures.site_repetido(36)
        root = _repo({"wyd-docs/redirects.tsv": ""})
        site = root / "site"
        for nome, conteudo in dados.items():
            (site / nome).parent.mkdir(parents=True, exist_ok=True)
            (site / nome).write_text(conteudo, encoding="utf-8")
        processo = subprocess.run(
            [
                sys.executable,
                "-m",
                "gate",
                "check",
                "links",
                "--root",
                str(root),
                "--site",
                str(site),
                "--json",
            ],
            cwd=Path(__file__).resolve().parents[1],
            capture_output=True,
            text=True,
            check=False,
        )
        self.assertEqual(processo.returncode, 0, processo.stderr)
        payload = json.loads(processo.stdout)
        self.assertEqual(payload["domains"][0]["summary"], "36 páginas, 0 fontes")


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
