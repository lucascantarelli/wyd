# Template de issue/fatia — padrão único

> Pergunta: "qual é o formato único de uma especificação de issue/fatia, quais skills o agente carrega antes de executar e como a issue prova que terminou?"
> Público: arquitetura, `wyd-ops`, desenvolvimento, QA e agentes
> Dono: `wyd-ops` (forma); cada sistema é dono do conteúdo da sua issue
> Base: `.github/ISSUE_TEMPLATE/*`; `docs/github-mapping.md` §7; `docs/estrutura-de-pastas-e-modulos.md` §3.1; `AGENTS.md` §7
> Revisado em: 2026-11-26

**Estado:** proposto (2026-11-26). Este é o molde único das issues/fatias do projeto; a aprovação
editorial é do `wyd-ops` e não muda o conteúdo de nenhuma fatia. As páginas derivadas citam este
arquivo e não repetem o padrão.

## 1. Onde o padrão vale

Vale para toda página `WYD*.md` de `planejamento-1.0.0/<sistema>/docs/roadmap/` e para as issues
`ADM-01`–`ADM-10` do `wyd-admin`. O roadmap do sistema é catálogo: ele cita a página da issue e o
`§Skills`; não repete fase, requisitos, dependência, entregável, DoD ou fora de escopo.

Os arquivos de `.github/ISSUE_TEMPLATE/` são formulários de entrada do GitHub, não uma segunda
especificação. Ao abrir uma issue já especificada, use o formulário do tipo (`tarefa.yml`,
`decisao.yml`, `planejamento.yml` ou `bug.yml`) e linke a página dona; o formulário coleta o mesmo
`skill_principal` e os `skills_complementares` deste padrão.

## 2. Esqueleto obrigatório

Toda página de issue/fatia segue esta ordem. Campo opcional pode ser omitido; seção obrigatória não.

```markdown
# <ID> — <título curto>

> Pergunta: "<o que esta issue entrega e como sei que terminou?>"
> Público: <quem lê>
> Dono: <equipe do sistema>
> Base: <páginas donas, requisitos e decisões>
> Revisado em: <AAAA-MM-DD>

**Estado:** <proposto|aceito|substituído> (<AAAA-MM-DD>)

**Fase:** <fase ou marco; quando houver>
**Fatia:** <identificador da fatia; quando houver>
**Prioridade:** <P1|P2|P3; quando definida>
**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

## Contexto

<por que a issue existe e qual página/requisito ela serve>

## Entregável

<o que existe ao final, em arquivo, comando ou artefato>

## DoD

- [ ] <critério observável com comando ou arquivo>
- [ ] <critério observável com comando ou arquivo>

## Fora de escopo

<o que não entra nesta issue>

## Depende de

<issues, lacunas, capturas ou decisões; escreva "nada" quando não houver>

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `<uma das nove skills da §3>` |
| `skills_complementares` | `<zero ou mais skills da §3>` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

<saída dos comandos do DoD, arquivos e gate verde no PR>
```

## 3. Skills — lista fechada

A lista tem nove nomes. O agente carrega a `skill_principal` **antes** de começar e só usa uma
complementar quando a página declarar que ela é necessária.

| Skill | Onde existe | Quando usar |
|---|---|---|
| `auditar-planejamento` | `.agents/skills/` (disco) | revisar página, contagem, ADR ou afirmação sem evidência |
| `documentation-writer` | cloud skill do projeto | escrever ou reescrever documentação |
| `finishing-a-development-branch` | cloud skill do projeto | fechar uma branch de implementação |
| `frontend-design` | cloud skill do projeto | interface, tema ou experiência visual |
| `github-actions-docs` | cloud skill do projeto | workflow, action, permissão, cache ou OIDC |
| `planejar-sistema` | `.agents/skills/` (disco) | abrir sistema, fronteira, requisitos ou fatia |
| `python-testing-patterns` | cloud skill do projeto | teste Python, fixture ou mock |
| `skill-creator` | cloud skill do projeto | criar, revisar ou medir skill |
| `writing-plans` | cloud skill do projeto | transformar decisão em plano executável |

Duas skills existem em disco e sete são cloud skills do projeto; nenhuma página precisa repetir essa
distinção. Skill fora desta lista reprova o gate.

## 4. Regras de propriedade

1. **Uma issue, uma página.** A página é dona do estado, da fase/fatia, da prioridade, do entregável,
   do DoD, do fora de escopo, da dependência, das skills e da evidência.
2. **Roadmap só cataloga.** `roadmap.md`, `fases.md` e `index.md` apontam a página e o `§Skills`; não
   repetem os campos da §2.
3. **Pré-requisito é explícito.** O que precisa existir antes fica em `## Depende de`; lacuna aberta
   bloqueia a issue e aponta o ID.
4. **DoD é observável.** "Funciona" não é critério; o item nomeia comando, arquivo, status HTTP ou
   teste.
5. **Gate é único.** O gate da issue é o comando do `## DoD` mais `make gate` na raiz; o ciclo curto
   usa `make gate-fast`.
6. **Número não se antecipa.** Issue do GitHub recebe número na criação; a página usa a chave da
   fatia ou o ID já registrado (`WYDDB-nnn` ou `ADM-nn`).
7. **Sem cópia do padrão.** A página cita este template; não replica a explicação das nove skills.
8. **Sem segredo ou dado de jogador.** A issue descreve tipo e lugar, nunca valor.

## 5. Gate de verificação

```bash
# a partir da raiz
make gate-fast          # skills, lacunas, contagens e censos
make gate               # build, links e todos os domínios (container)
```

O gate `skills` confere o bloco `## Skills`, a linha `**Dono do valor:**`, a citação do roadmap e a
lista fechada. O gate `links` confere a página publicada. Uma página de issue sem os campos da §2
reprova; a correção é no arquivo, não no gate.

## 6. O que esta página não cobre

- Criação de issue, label, milestone ou board — ato explícito do dono (`AGENTS.md` §5, item 9).
- Conteúdo técnico, requisito ou decisão de uma fatia — dono é o sistema.
- Numeração de `WYDDB-nnn` ou `WYDPRT-nnn` — dona é a página de criação.
