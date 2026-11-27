"""Testes dos verificadores e do contrato do CLI, com fixtures centralizadas."""

from __future__ import annotations

import json
import multiprocessing
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path, PurePosixPath
from unittest import mock

from gate.checks import (
    alvos,
    capacidades,
    censos,
    contagens,
    hoje,
    lacunas,
    layout,
    links,
    skills,
)
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


class LayoutTests(unittest.TestCase):
    def test_scripts_de_ci_so_em_wyd_ops(self):
        self.assertIsNotNone(layout.validar_caminho(PurePosixPath("scripts/ci/db-prepare.sh")))
        self.assertIsNone(layout.validar_caminho(PurePosixPath("wyd-ops/scripts/ci/db-prepare.sh")))

    def test_deny_so_em_wyd_ops(self):
        self.assertIsNotNone(layout.validar_caminho(PurePosixPath("deny.toml")))
        self.assertIsNone(layout.validar_caminho(PurePosixPath("wyd-ops/deny.toml")))

    def test_planejamento_nao_aceita_artefato_solto_nem_executavel(self):
        self.assertIsNotNone(
            layout.validar_caminho(PurePosixPath("planejamento-1.0.0/slice-wyd-db-001-wyd-net-001.md"))
        )
        self.assertIsNotNone(
            layout.validar_caminho(PurePosixPath("planejamento-1.0.0/wyd-db/scripts/schema.py"))
        )
        self.assertIsNone(
            layout.validar_caminho(PurePosixPath("planejamento-1.0.0/wyd-db/lacunas.md"))
        )

    def test_gitkeep_so_em_pasta_reservada(self):
        self.assertIsNone(layout.validar_caminho(PurePosixPath("wyd-portal/src/auth/.gitkeep")))
        self.assertIsNone(layout.validar_caminho(PurePosixPath("wyd-client/tools/.gitkeep")))
        self.assertIsNotNone(layout.validar_caminho(PurePosixPath("crates/wyd-core/.gitkeep")))

    def test_dockerfile_so_em_wyd_ops(self):
        self.assertIsNone(layout.validar_caminho(PurePosixPath("wyd-ops/wyd-server.Dockerfile")))
        self.assertIsNotNone(layout.validar_caminho(PurePosixPath("wyd-server/Dockerfile")))


class LacunaTests(unittest.TestCase):
    def test_fechada_com_evidencia_passa(self):
        resultado = lacunas.run(Workspace(_repo(fixtures.LACUNA_OK)))
        self.assertTrue(resultado.ok, resultado.problems)

    def test_fechada_sem_evidencia_falha(self):
        resultado = lacunas.run(Workspace(_repo(fixtures.LACUNA_SEM_EVIDENCIA)))
        self.assertFalse(resultado.ok)
        self.assertTrue(resultado.problems)

    def test_citacao_de_lacunas_do_wyd_db_passa(self):
        resultado = lacunas.run(Workspace(_repo(fixtures.LACUNA_CITACAO_OK)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("3 citação(ões)", resultado.summary)

    def test_citacao_de_lacunas_velha_reprova(self):
        resultado = lacunas.run(Workspace(_repo(fixtures.LACUNA_CITACAO_VELHA)))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any("cita 35 fechadas" in problema for problema in resultado.problems),
            resultado.problems,
        )

    def test_citacao_de_lacunas_em_secao_datada_nao_reprova(self):
        resultado = lacunas.run(Workspace(_repo(fixtures.LACUNA_CITACAO_HISTORICA)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("2 histórica(s)", resultado.summary)
        self.assertTrue(resultado.details)
        for detalhe in resultado.details:
            self.assertIn("seção datada herdada (2026-11-24)", detalhe)

    def test_citacao_de_lacunas_explica_data_na_frase_e_transicao(self):
        resultado = lacunas.run(Workspace(_repo(fixtures.LACUNA_CITACAO_MOTIVOS)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("2 histórica(s)", resultado.summary)
        self.assertTrue(
            any("data na frase (2026-11-24)" in detalhe for detalhe in resultado.details),
            resultado.details,
        )
        self.assertTrue(
            any("transição (`" in detalhe for detalhe in resultado.details),
            resultado.details,
        )


class SkillTests(unittest.TestCase):
    def test_roadmap_cita_skills_sem_repetir_fase(self):
        resultado = skills.run(Workspace(_repo(fixtures.SKILLS_OK)))
        self.assertTrue(resultado.ok, resultado.problems)

    def test_roadmap_que_repete_fase_falha(self):
        resultado = skills.run(Workspace(_repo(fixtures.SKILLS_REPETE_FASE)))
        self.assertFalse(resultado.ok)
        self.assertTrue(any("Fase" in p for p in resultado.problems))

    def test_formulario_fora_da_lista_fechada_falha(self):
        resultado = skills.run(Workspace(_repo(fixtures.SKILLS_FORMULARIO_DIVERGENTE)))
        self.assertFalse(resultado.ok)
        self.assertTrue(any("skill-inventada" in p for p in resultado.problems))
        self.assertTrue(any("auditar-planejamento" in p for p in resultado.problems))


class ContagemTests(unittest.TestCase):
    def test_medidas_da_serie_coincidem(self):
        root = _repo(fixtures.CONTAGEM_OK)
        medida = contagens.medidas(Workspace(root))
        self.assertEqual((medida.arquivos, medida.tabelas), (1, 2))
        self.assertEqual(contagens.medir_ddl(root), 1)
        self.assertEqual(contagens.medir_plano(root), 1)


class AlvoTests(unittest.TestCase):
    def test_cargo_e_o_comando_concordam(self):
        resultado = alvos.run(Workspace(_repo(fixtures.ALVOS_OK)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("2 `[[test]]`", resultado.summary)

    def test_tabela_nao_reprova_mesmo_com_nome_desconhecido(self):
        resultado = alvos.run(Workspace(_repo(fixtures.ALVOS_TABELA_INEXISTENTE)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertTrue(any("nota" in linha and "contrato_forma" in linha for linha in resultado.details))

    def test_comando_com_alvo_inexistente_reprova(self):
        resultado = alvos.run(Workspace(_repo(fixtures.ALVOS_COMANDO_INEXISTENTE)))
        self.assertFalse(resultado.ok)
        self.assertEqual(len(resultado.problems), 2)
        self.assertIn("contrato_forma", resultado.problems[0])
        self.assertIn("alvo_fantasma", resultado.problems[1])
        self.assertIn("docs/guia-teste/index.md §1", resultado.problems[0])

    def test_prosa_com_lista_e_com_lixo_fora_do_nome_de_alvo(self):
        resultado = alvos.run(Workspace(_repo(fixtures.ALVOS_PROSA_OK)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("3 token(s) descartado(s) em prosa", resultado.summary)
        descartes = [linha for linha in resultado.details if linha.startswith("ignorado")]
        self.assertEqual(len(descartes), 3)

    def test_test_sem_cargo_test_aparece_em_details_sem_reprovar(self):
        resultado = alvos.run(Workspace(_repo(fixtures.ALVOS_TEST_SEM_COMANDO)))
        baseline = alvos.run(Workspace(_repo(fixtures.ALVOS_OK)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertEqual(
            resultado.summary.split(" citação")[0], baseline.summary.split(" citação")[0]
        )
        self.assertIn("3 `--test` fora de comando de teste", resultado.summary)
        descartes = [
            linha for linha in resultado.details if "--test" in linha and "cargo test" in linha
        ]
        self.assertEqual(len(descartes), 3)
        self.assertIn("pagina.md:3", descartes[0])
        self.assertIn("alvo_fantasma", descartes[2])

    def test_prosa_com_alvo_inexistente_vira_detalhe(self):
        resultado = alvos.run(Workspace(_repo(fixtures.ALVOS_PROSA_INEXISTENTE)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertTrue(any("nota" in linha and "contrato_forma" in linha for linha in resultado.details))

    def test_comando_de_alvo_planejado_em_dod_pendente_passa(self):
        resultado = alvos.run(Workspace(_repo(fixtures.ALVOS_COMANDO_PLANEJADO)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertTrue(
            any("WYDDB-028.md:5" in linha and "planejado" in linha for linha in resultado.details)
        )
        self.assertTrue(
            any("docs/guia-teste/index.md` §1" in linha for linha in resultado.details)
        )

    def test_comando_de_alvo_planejado_sem_marca_reprova(self):
        resultado = alvos.run(Workspace(_repo(fixtures.ALVOS_COMANDO_PLANEJADO_SEM_MARCA)))
        self.assertFalse(resultado.ok)
        self.assertIn("alvo planejado `contrato_catalogo`", resultado.problems[0])

    def test_planejado_vem_da_tabela_e_deixa_de_valer_quando_ela_muda(self):
        resultado = alvos.run(Workspace(_repo(fixtures.ALVOS_PLANEJADO_JA_EXECUTADO)))
        self.assertFalse(resultado.ok)
        self.assertIn("contrato_catalogo", resultado.problems[0])
        self.assertIn("nem está planejado", resultado.problems[0])

    def test_comando_quebrado_com_manifest_path_reprova(self):
        resultado = alvos.run(Workspace(_repo(fixtures.ALVOS_MANIFESTO_QUEBRADO)))
        self.assertFalse(resultado.ok)
        self.assertIn("contrato_forma", resultado.problems[0])
        self.assertIn("SKILL.md:4", resultado.problems[0])

    def test_comando_de_outro_crate_fica_fora(self):
        # `sistema_pg` é `[[test]]` do `wyd-db`: mesmo assim, `-p wyd-protocol` tira o comando do
        # escopo e ele não pode sair como `ok` nem reprovar por nome.
        resultado = alvos.run(Workspace(_repo(fixtures.ALVOS_OUTRO_CRATE)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertTrue(
            any("fora do crate" in linha and "wyd-protocol" in linha for linha in resultado.details)
        )
        self.assertFalse(
            any(
                linha.startswith("ok ")
                and "provar/SKILL.md" in linha
                and "sistema_pg" in linha
                for linha in resultado.details
            )
        )
        self.assertIn("1 fora do crate", resultado.summary)

    def test_guia_sem_tabela_de_alvos_acusa(self):
        resultado = alvos.run(Workspace(_repo(fixtures.ALVOS_GUIA_SEM_TABELA)))
        self.assertFalse(resultado.ok)
        self.assertIn("guia-teste/index.md", resultado.summary)

    def test_tabela_sem_linha_planejada_e_estado_valido(self):
        resultado = alvos.run(Workspace(_repo(fixtures.ALVOS_SEM_PLANEJADOS)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("0 em `docs/guia-teste/index.md` §1", resultado.summary)


class CensoTests(unittest.TestCase):
    def test_mede_atos_celulas_e_qblk(self):
        root = _repo(fixtures.CENSO_OK)
        medida = censos.medidas(Workspace(root))
        self.assertEqual(medida.atos["total"], 2)
        self.assertEqual((medida.teste, medida.issue), (1, 1))
        self.assertEqual(medida.qblk, ["Q-BLK-01"])

    def test_atos_sem_de_aceite_reprova(self):
        resultado = censos.run(Workspace(_repo(fixtures.CENSO_ATOS_VELHO)))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any("cita 3 atos" in problema for problema in resultado.problems), resultado.problems
        )

    def test_atos_sem_de_aceite_correto_passa(self):
        resultado = censos.run(Workspace(_repo(fixtures.CENSO_ATOS_CORRETO)))
        self.assertTrue(resultado.ok, resultado.problems)

    def test_citacao_velha_abaixo_da_metade_do_total_reprova(self):
        resultado = censos.run(Workspace(_repo(fixtures.CENSO_ATOS_VELHO_ABAIXO_METADE)))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any("cita 2 atos, medido 5" in problema for problema in resultado.problems),
            resultado.problems,
        )

    def test_contagem_de_leva_fica_fora_do_censo(self):
        resultado = censos.run(Workspace(_repo(fixtures.CENSO_ATOS_LEVA_12)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertTrue(
            any(
                "12 atos" in detalhe and "contagem de leva" in detalhe
                for detalhe in resultado.details
            ),
            resultado.details,
        )

    def test_texto_corrido_reprova(self):
        resultado = censos.run(Workspace(_repo(fixtures.CENSO_ATOS_TEXTO_CORRIDO)))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any("cita 3 atos, medido 2" in problema for problema in resultado.problems),
            resultado.problems,
        )

    def test_negrito_cobrindo_numero_e_palavra_reprova(self):
        resultado = censos.run(Workspace(_repo(fixtures.CENSO_ATOS_NEGRITO_FRASE)))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any("cita 3 atos, medido 2" in problema for problema in resultado.problems),
            resultado.problems,
        )

    def test_texto_corrido_datado_passa(self):
        resultado = censos.run(Workspace(_repo(fixtures.CENSO_ATOS_TEXTO_CORRIDO_DATADO)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertTrue(
            any("2026-11-24" in detalhe and "3 atos" in detalhe for detalhe in resultado.details),
            resultado.details,
        )

    def test_leva_em_outra_oracao_nao_isenta_o_total(self):
        resultado = censos.run(Workspace(_repo(fixtures.CENSO_ATOS_LEVA_DISTANTE)))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any("cita 12 atos, medido 2" in problema for problema in resultado.problems),
            resultado.problems,
        )

    def test_citacao_total_de_12_reprova(self):
        resultado = censos.run(Workspace(_repo(fixtures.CENSO_ATOS_TOTAL_DOZE)))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any("cita 12 atos, medido 2" in problema for problema in resultado.problems),
            resultado.problems,
        )

    def test_atos_em_secao_datada_nao_reprova(self):
        resultado = censos.run(Workspace(_repo(fixtures.CENSO_ATOS_DATADO)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertTrue(
            any("seção datada 2026-11-24" in detalhe for detalhe in resultado.details),
            resultado.details,
        )

    def test_atos_com_data_na_frase_nao_reprova(self):
        resultado = censos.run(Workspace(_repo(fixtures.CENSO_ATOS_DATA_NA_FRASE)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertTrue(
            any("data 2026-11-24 na frase" in detalhe for detalhe in resultado.details),
            resultado.details,
        )


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


class CapacidadeTests(unittest.TestCase):
    def test_citacao_de_acordo_com_o_censo(self):
        resultado = capacidades.run(Workspace(_repo(fixtures.CAPACIDADES_OK)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("2 citação(ões)", resultado.summary)
        self.assertIn("20 cobertas", resultado.summary)

    def test_numero_velho_reprova_com_arquivo_e_linha(self):
        resultado = capacidades.run(Workspace(_repo(fixtures.CAPACIDADES_VELHO)))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any("pagina.md:3" in problema and "cita 14" in problema for problema in resultado.problems),
            resultado.problems,
        )

    def test_secao_datada_antes_da_medicao_fica_fora(self):
        resultado = capacidades.run(Workspace(_repo(fixtures.CAPACIDADES_HISTORICO)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("1 histórica(s)/planejada(s)", resultado.summary)
        self.assertIn("— isenta: seção datada herdada (2026-11-24)", resultado.details[0])
        self.assertNotIn("registro histórico", resultado.details[0])

    def test_meta_de_cobertura_fica_fora(self):
        resultado = capacidades.run(Workspace(_repo(fixtures.CAPACIDADES_PLANEJADO)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("— isenta: transição/planejado (`vai de`)", resultado.details[0])
        self.assertNotIn("data na frase", resultado.details[0])
        self.assertIn("1 histórica(s)/planejada(s)", resultado.summary)

    def test_le_o_censo_do_dono_em_vez_de_fixar_vinte(self):
        resultado = capacidades.run(Workspace(_repo(fixtures.CAPACIDADES_MEDIDA_NOVA)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("21 cobertas", resultado.summary)

    def test_dono_ausente_nao_inventa_medida(self):
        with self.assertRaises(capacidades.Erro):
            capacidades.run(Workspace(_repo(fixtures.CAPACIDADES_SEM_DONO)))

    def test_total_de_capacidades_sem_de_39(self):
        resultado = capacidades.run(Workspace(_repo(fixtures.CAPACIDADES_TOTAL_OK)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("1 citação(ões)", resultado.summary)

    def test_total_velho_de_capacidades_reprova(self):
        resultado = capacidades.run(Workspace(_repo(fixtures.CAPACIDADES_TOTAL_VELHO)))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any("cita 38" in problema and "mede 39" in problema for problema in resultado.problems),
            resultado.problems,
        )

    def test_sem_execucao_fora_da_forma_longa_reprova(self):
        resultado = capacidades.run(Workspace(_repo(fixtures.CAPACIDADES_SEM_EXECUCAO_VELHO)))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any("cita 18" in problema and "mede 19" in problema for problema in resultado.problems),
            resultado.problems,
        )

    def test_recorte_de_cap_nn_nao_conta_como_censo(self):
        resultado = capacidades.run(Workspace(_repo(fixtures.CAPACIDADES_RECORTE)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("0 citação(ões)", resultado.summary)

    def test_subsecao_herda_a_data_do_cabecalho_pai(self):
        resultado = capacidades.run(Workspace(_repo(fixtures.CAPACIDADES_SECAO_HERDADA)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("— isenta: seção datada herdada (2026-11-24)", resultado.details[0])
        self.assertNotIn("registro histórico", resultado.details[0])
        self.assertIn("1 histórica(s)/planejada(s)", resultado.summary)

    def test_recorte_com_cap_nn_ou_que_nao_conta(self):
        resultado = capacidades.run(Workspace(_repo(fixtures.CAPACIDADES_RECORTE_CAP)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("0 citação(ões)", resultado.summary)

    def test_data_na_frase_isenta_e_explica(self):
        resultado = capacidades.run(Workspace(_repo(fixtures.CAPACIDADES_DATA_NA_LINHA)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("— isenta: data na frase (2026-11-24)", resultado.details[0])
        self.assertNotIn("seção datada", resultado.details[0])
        self.assertIn("1 histórica(s)/planejada(s)", resultado.summary)

    def test_data_em_outra_frase_nao_isenta_numero_velho(self):
        resultado = capacidades.run(Workspace(_repo(fixtures.CAPACIDADES_DATA_NAO_QUALIFICA)))
        self.assertFalse(resultado.ok)
        self.assertEqual(1, len(resultado.problems), resultado.problems)
        self.assertTrue(any("cita 38" in problema for problema in resultado.problems), resultado.problems)
        self.assertFalse(any("cita 23" in problema for problema in resultado.problems), resultado.problems)



class HojeTests(unittest.TestCase):
    def test_atos_de_hoje_divergente_reprova(self):
        resultado = hoje.run(Workspace(_repo(fixtures.HOJE_ATOS_VELHO)))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any("hoje 3 atos" in problema and "mede 2 atos" in problema for problema in resultado.problems),
            resultado.problems,
        )

    def test_atos_de_hoje_correto_passa(self):
        resultado = hoje.run(Workspace(_repo(fixtures.HOJE_ATOS_OK)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("1 citação(ões) de `hoje` conferida(s)", resultado.summary)

    def test_secao_datada_isenta_com_rastro(self):
        resultado = hoje.run(Workspace(_repo(fixtures.HOJE_SECAO_DATADA)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertTrue(
            any("seção datada 2026-11-24" in detalhe for detalhe in resultado.details),
            resultado.details,
        )

    def test_capacidades_de_hoje_divergente_reprova(self):
        resultado = hoje.run(Workspace(_repo(fixtures.HOJE_CAPACIDADES_VELHO)))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any("hoje 14 de 39 capacidades" in problema for problema in resultado.problems),
            resultado.problems,
        )

    def test_numero_sem_unidade_fica_fora(self):
        resultado = hoje.run(Workspace(_repo(fixtures.HOJE_SEM_UNIDADE)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("0 citação(ões) de `hoje` conferida(s)", resultado.summary)

    def test_numero_sem_unidade_atribuido_pelo_dono_reprova(self):
        resultado = hoje.run(Workspace(_repo(fixtures.HOJE_CONTEXTO_DONO)))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any("hoje 3 atos" in problema and "mede 2 atos" in problema for problema in resultado.problems),
            resultado.problems,
        )

    def test_numero_sem_unidade_atribuido_pela_unidade_reprova(self):
        resultado = hoje.run(Workspace(_repo(fixtures.HOJE_CONTEXTO_UNIDADE)))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any("hoje 3 atos" in problema for problema in resultado.problems), resultado.problems
        )

    def test_atribuicao_por_contexto_sai_em_details(self):
        resultado = hoje.run(Workspace(_repo(fixtures.HOJE_CONTEXTO_OK)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("1 por contexto", resultado.summary)
        self.assertTrue(
            any("contexto: unidade na cláusula" in detalhe for detalhe in resultado.details),
            resultado.details,
        )

    def test_razao_de_outra_medida_fica_fora(self):
        resultado = hoje.run(Workspace(_repo(fixtures.HOJE_CONTEXTO_AMBIGUO)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("0 citação(ões) de `hoje` conferida(s)", resultado.summary)

    def test_contagens_de_hoje_divergente_reprova(self):
        resultado = hoje.run(Workspace(_repo(fixtures.HOJE_CONTAGENS_VELHO)))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any("hoje 3 arquivos" in problema and "mede 2 arquivos" in problema for problema in resultado.problems),
            resultado.problems,
        )
        self.assertFalse(any("9 tabelas" in problema for problema in resultado.problems))

    def test_fontes_de_hoje_exigem_censo_na_clausula(self):
        resultado = hoje.run(Workspace(_repo(fixtures.HOJE_CONTAGENS_FONTES)))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any("diz `hoje 999 fontes`" in problema for problema in resultado.problems),
            resultado.problems,
        )

    def test_paginas_do_site_de_hoje_reprovam(self):
        resultado = hoje.run(Workspace(_repo(fixtures.HOJE_CONTAGENS_SITE)))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any("diz `hoje 999 páginas do site`" in problema for problema in resultado.problems),
            resultado.problems,
        )

    def test_serie_materializada_fica_fora_da_checklist(self):
        resultado = hoje.run(Workspace(_repo(fixtures.HOJE_CONTAGENS_MATERIALIZADA)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("0 citação(ões) de `hoje` conferida(s)", resultado.summary)

    def test_unidade_generica_com_outra_contagem_vizinha_fica_fora(self):
        resultado = hoje.run(Workspace(_repo(fixtures.HOJE_CONTAGENS_AMBIGUO)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("0 citação(ões) de `hoje` conferida(s)", resultado.summary)

    def test_tabelas_de_outro_conjunto_ficam_fora(self):
        resultado = hoje.run(Workspace(_repo(fixtures.HOJE_CONTAGENS_OUTRO_CONJUNTO)))
        self.assertTrue(resultado.ok, resultado.problems)
        self.assertIn("0 citação(ões) de `hoje` conferida(s)", resultado.summary)
