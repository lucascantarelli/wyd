"""Contrato das skills: código, template, formulários e skills em disco falam a mesma lista."""

from __future__ import annotations

import re
import tempfile
import unittest
from pathlib import Path

from gate.checks import skills
from gate.workspace import Workspace

RAIZ = Path(__file__).resolve().parents[2]
TEMPLATE = RAIZ / "planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md"
INVENTARIO = RAIZ / "planejamento-1.0.0/wyd-ops/docs/planos/verificacao-de-skills-e-integracoes.md"
PASTA_SKILLS = RAIZ / ".agents/skills"
FORMULARIOS = RAIZ / ".github/ISSUE_TEMPLATE"


def _skills_do_template() -> set[str]:
    texto = TEMPLATE.read_text(encoding="utf-8")
    secao = texto.split("## 3. Skills", 1)[1].split("## 4.", 1)[0]
    return set(re.findall(r"^\|\s*`([^`]+)`\s*\|", secao, re.M))


def _arvore(arquivos: dict[str, str]) -> Path:
    raiz = Path(tempfile.mkdtemp(prefix="skill-contract-"))
    for relativo, conteudo in arquivos.items():
        destino = raiz / relativo
        destino.parent.mkdir(parents=True, exist_ok=True)
        destino.write_text(conteudo, encoding="utf-8")
    return raiz


def _formulario(*nomes: str) -> str:
    opcoes = "".join(f"        - {nome}\n" for nome in nomes)
    return (
        "name: Tarefa\nbody:\n  - type: dropdown\n    id: skill_principal\n"
        f"    attributes:\n      options:\n{opcoes}    validations:\n      required: true\n"
    )


def _template(*nomes: str) -> str:
    linhas = "".join(f"| `{nome}` | disco |\n" for nome in nomes)
    return f"# Template\n\n## 3. Skills — lista fechada\n\n| Skill | Onde existe |\n|---|---|\n{linhas}\n## 4. Regras\n"


class ContratoDeSkillsTests(unittest.TestCase):
    def test_gate_le_o_template_e_os_formularios_sem_divergencia(self):
        problemas: list[str] = []
        nomes = skills.lista_fechada(Workspace(RAIZ), problemas)
        self.assertEqual(problemas, [])
        self.assertEqual(nomes, _skills_do_template())

    def test_formulario_que_oferece_a_mais_reprova(self):
        raiz = _arvore(
            {
                "planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md": _template(
                    "planejar-sistema"
                ),
                ".github/ISSUE_TEMPLATE/tarefa.yml": _formulario(
                    "planejar-sistema", "skill-inventada"
                ),
            }
        )
        problemas: list[str] = []
        skills.lista_fechada(Workspace(raiz), problemas)
        self.assertTrue(any("skill-inventada" in p and "fora da lista" in p for p in problemas))

    def test_formulario_que_omite_a_skill_do_template_reprova(self):
        raiz = _arvore(
            {
                "planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md": _template(
                    "planejar-sistema", "auditar-planejamento"
                ),
                ".github/ISSUE_TEMPLATE/tarefa.yml": _formulario("planejar-sistema"),
            }
        )
        problemas: list[str] = []
        skills.lista_fechada(Workspace(raiz), problemas)
        self.assertTrue(any("auditar-planejamento" in p and "não oferece" in p for p in problemas))

    def test_formulario_sem_o_campo_skill_principal_reprova(self):
        raiz = _arvore(
            {
                "planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md": _template(
                    "planejar-sistema"
                ),
                ".github/ISSUE_TEMPLATE/tarefa.yml": (
                    "name: Tarefa\nbody:\n  - type: input\n    id: contexto\n"
                    "    attributes:\n      label: Contexto\n"
                ),
            }
        )
        resultado = skills.run(Workspace(raiz))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any(
                "tarefa.yml" in p and "sem o campo `skill_principal`" in p
                for p in resultado.problems
            ),
            resultado.problems,
        )

    def test_secao_de_skills_vazia_reprova(self):
        raiz = _arvore(
            {
                "planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md": (
                    "# Template\n\n## 3. Skills — lista fechada\n\n"
                    "Nenhuma skill cadastrada ainda.\n\n## 4. Regras\n"
                ),
                ".github/ISSUE_TEMPLATE/tarefa.yml": _formulario("planejar-sistema"),
            }
        )
        resultado = skills.run(Workspace(raiz))
        self.assertFalse(resultado.ok)
        self.assertTrue(
            any(
                "template-de-issue.md" in p and "não lista nenhuma skill" in p
                for p in resultado.problems
            ),
            resultado.problems,
        )

    def test_todo_formulario_oferece_a_lista_fechada(self):
        nomes = skills.lista_fechada(Workspace(RAIZ), [])
        formularios = sorted(
            arquivo for arquivo in FORMULARIOS.glob("*.yml") if arquivo.name != "config.yml"
        )
        for formulario in formularios:
            texto = formulario.read_text(encoding="utf-8")
            for nome in sorted(nomes):
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
        for nome in sorted(skills.lista_fechada(Workspace(RAIZ), [])):
            self.assertIn(f"`{nome}`", texto)

    def test_pr_template_exige_registro_da_skill(self):
        texto = (RAIZ / ".github/pull_request_template.md").read_text(encoding="utf-8")
        self.assertIn("## Skills carregadas", texto)
        self.assertIn("Skill obrigatória da issue", texto)


if __name__ == "__main__":
    unittest.main()
