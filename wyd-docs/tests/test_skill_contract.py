"""Contrato das skills: código, template, formulários e skills em disco falam a mesma lista."""

from __future__ import annotations

import re
import unittest
from pathlib import Path

from gate.checks import skills

RAIZ = Path(__file__).resolve().parents[2]
TEMPLATE = RAIZ / "planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md"
INVENTARIO = RAIZ / "planejamento-1.0.0/wyd-ops/docs/planos/verificacao-de-skills-e-integracoes.md"
PASTA_SKILLS = RAIZ / ".agents/skills"
FORMULARIOS = RAIZ / ".github/ISSUE_TEMPLATE"


def _skills_do_template() -> set[str]:
    texto = TEMPLATE.read_text(encoding="utf-8")
    secao = texto.split("## 3. Skills", 1)[1].split("## 4.", 1)[0]
    return set(re.findall(r"^\|\s*`([^`]+)`\s*\|", secao, re.M))


class ContratoDeSkillsTests(unittest.TestCase):
    def test_lista_do_gate_e_do_template_coincidem(self):
        self.assertEqual(_skills_do_template(), skills.SKILLS)

    def test_todo_formulario_oferece_a_lista_fechada(self):
        formularios = sorted(
            arquivo for arquivo in FORMULARIOS.glob("*.yml") if arquivo.name != "config.yml"
        )
        for formulario in formularios:
            texto = formulario.read_text(encoding="utf-8")
            for nome in sorted(skills.SKILLS):
                self.assertIn(nome, texto, f"{formulario.name} não oferece {nome}")

    def test_skills_locais_tem_nome_e_frontmatter(self):
        pastas = {pasta.name for pasta in PASTA_SKILLS.iterdir() if pasta.is_dir()}
        self.assertEqual(pastas, {"auditar-planejamento", "planejar-sistema"})
        for nome in sorted(pastas):
            texto = (PASTA_SKILLS / nome / "SKILL.md").read_text(encoding="utf-8")
            achado = re.search(r"^name:\s*(\S+)\s*$", texto, re.M)
            self.assertIsNotNone(achado, f"{nome}/SKILL.md sem frontmatter name")
            self.assertEqual(achado.group(1), nome)

    def test_inventario_cita_todas_as_skills(self):
        texto = INVENTARIO.read_text(encoding="utf-8")
        for nome in sorted(skills.SKILLS):
            self.assertIn(f"`{nome}`", texto)

    def test_pr_template_exige_registro_da_skill(self):
        texto = (RAIZ / ".github/pull_request_template.md").read_text(encoding="utf-8")
        self.assertIn("## Skills carregadas", texto)
        self.assertIn("Skill obrigatória da issue", texto)


if __name__ == "__main__":
    unittest.main()
