# `wyd-docs` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-19).** Este é o **único** lugar onde uma lacuna do site oficial
> tem dono, impacto, marco e estado.
>
> **O que este arquivo cobre:** as **14 lacunas** `LPDOC-01`–`LPDOC-14`, **1 fechada** (`LPDOC-01`,
> rollback e restauração validados em 2026-11-19, `wyd-docs-1.0.0.md` §7.3). O site foi montado em
> MkDocs 2.0 pré-release e publicado/validado em GitHub Pages em 2026-11-19. O dono aprovou GitHub
> Pages como hospedagem canônica com publicação contínua.
>
> **O que este arquivo não é:** não decide. Cada linha diz **quem** decide e **o que** precisa ser
> decidido; a decisão entra no documento dono (ou em `ADR-DOC-nnnn`) e volta aqui como linha
> fechada, com data e evidência.

---

## 0. Como ler

### 0.1 Estados

| Estado | Significado |
|---|---|
| **aberta** | sem decisão; a página ou a automação dependente espera |
| **proposta** | existe recomendação escrita; falta o \"sim\" do dono |
| **fechada** | decidida e refletida no arquivo citado em \"evidência\" |
| **externa** | decidida, mas a execução depende de frente fora do site |

### 0.2 Marcos

| Marco | Significa |
|---|---|
| **A** | antes de aprovar este plano |
| **P1** | antes da primeira publicação em ambiente público |
| **P2** | antes de o site receber página de sistema novo |
| **P3** | antes de a documentação ser citada como referência externa |

### 0.3 Donos

| Dono | Decide |
|---|---|
| **operação** | hospedagem, permissão, domínio, deploy |
| **documentação** | padrão editorial, revisão, o que publica |
| **arquitetura** | forma da montagem, URL, versionamento, sintaxe |
| **produto** | o que o visitante vê e em que idioma |

---

## 1. Situação em uma linha

| Grupo | Lacunas | Abertas | Propostas | Fechadas |
|---|---:|---:|---:|---:|
| Publicação e infraestrutura | 3 | 1 (`LPDOC-04`) | 1 (`LPDOC-08`) | 1 (`LPDOC-01`) |
| Conteúdo publicado | 3 | 0 | 3 (`LPDOC-02`, `LPDOC-03`, `LPDOC-09`) | 0 |
| URL, versão e idioma | 3 | 1 (`LPDOC-05`) | 2 (`LPDOC-06`, `LPDOC-07`) | 0 |
| Ferramenta e sintaxe | 5 | 3 (`LPDOC-10`, `LPDOC-12`, `LPDOC-13`) | 2 (`LPDOC-11`, `LPDOC-14`) | 0 |
| **Total** | **14** | **5** | **8** | **1** |

\"Proposta\" não é decisão: descreve o caminho recomendado para o dono aceitar ou recusar.

---

## 2. Lacunas

### 2.1 Publicação e infraestrutura

| ID | Pergunta | Por que importa | Dono | Marco | Estado | Evidência |
|---|---|---|---|---|---|---|
| `LPDOC-01` | Validar rollback da hospedagem canônica aprovada (GitHub Pages)? | confirmar que a publicação contínua pode ser revertida com segurança; hospedar em duas plataformas duplicaria automação | operação | `P1` | fechada | `V` (2026-11-19): dono aprovou GitHub Pages como hospedagem canônica com publicação contínua; PR #44 mesclado, workflow `docs` publicou e o site foi validado. Pré-checagens somente leitura: runs #2 (`44492052127`, SHA `2378b62b`) e #4 (`44498880453`, SHA `0d765858`) em `success` em `develop`, sem execução concorrente; o ambiente `github-pages` só tem política de branch `develop`, sem revisores; e os deployments #6 (`7910251866`) e #8 (`7910399204`) mostraram que um push novo publica por cima do estado anterior, o que exigiu a janela de congelamento; o dono confirmou o run #4 como SHA final e declarou a janela de congelamento. Validação local: os dois commits passaram `preparar.py`, `mkdocs build` e `verificar.py site` em container Linux (30 páginas, exit 0), diferindo só na URL do Highlight.js em `tema/site.js`. Teste na janela declarada: run #2 reexecutado publicou `2378b62b` (deployment `7911886524`, `success` `11:33:08Z`) e o site passou a servir `npm/highlight.js@11.12.0`; run #4 reexecutado restaurou `0d765858` (deployment `7911904845`, `success` `11:34:01Z`), com `cdn-release@11.12.0` servido e `200` na raiz e em páginas internas. Não medidos: erro de console e presença de `hljs`. Detalhe em `wyd-docs-1.0.0.md` §7.3. |
| `LPDOC-04` | O site terá domínio próprio? | URL publicada é contrato; trocar depois exige redirect permanente | operação + produto | `P1` | aberta | nenhuma URL canônica definida; sem `site_url` no MkDocs 2.0 |
| `LPDOC-08` | Quem aprova a publicação de um sistema novo? | sem dono, qualquer pasta vira página pública; com dono único, um sistema pode ficar meses fora do ar | documentação + arquitetura | `P2` | proposta | `preparar.py` publica toda pasta `docs/` que existir, sem aprovação por sistema |

### 2.2 Conteúdo publicado

| ID | Pergunta | Por que importa | Dono | Marco | Estado | Evidência |
|---|---|---|---|---|---|---|
| `LPDOC-02` | `AGENTS.md` e o panorama entram como página? | são normativos e úteis ao contribuidor, mas também expõem processo interno | documentação | `A` | proposta | não são publicados hoje: o MkDocs 2.0 não tem include nem plugin, então exigiria cópia montada por `preparar.py` |
| `LPDOC-03` | Página com estado `proposto` aparece publicada? | esconder rascunho deixa o site incompleto; publicar sem aviso faz leitor citar intenção como regra | documentação | `A` | proposta | hoje publica; o estado viaja no cabeçalho da página |
| `LPDOC-09` | As 91 páginas geradas do `wyd-db` entram no site? | é o maior volume do site e depende de gerador que ainda não existe | arquitetura + documentação | `P3` | proposta | `planejamento-1.0.0/wyd-db/proposta-paginas.md` planeja 91 geradas; nenhuma existe |

### 2.3 URL, versão e idioma

| ID | Pergunta | Por que importa | Dono | Marco | Estado | Evidência |
|---|---|---|---|---|---|---|
| `LPDOC-05` | O site é versionado por release (`/1.0.0/`) ou publica só a branch? | versionar exige gerador de versão — que a 2.0 não tem como plugin | arquitetura | `P3` | aberta | sem mecanismo de versão em `mkdocs 2.0.dev6` |
| `LPDOC-06` | Renomear página publicada exige redirect? | URL é contrato; sem redirect, link externo quebra em silêncio | arquitetura | `P2` | proposta | nenhum redirect implementado; `verificar.py` não detecta link externo quebrado |
| `LPDOC-07` | Um idioma ou i18n? | hoje é português; público internacional exigiria árvores paralelas e tema próprio | produto | `P3` | proposta | todo conteúdo em pt-BR; sem plugin de idioma na 2.0 |

### 2.4 Ferramenta e sintaxe

| ID | Pergunta | Por que importa | Dono | Marco | Estado | Evidência |
|---|---|---|---|---|---|---|
| `LPDOC-10` | O catálogo de opcodes e o dicionário de dados são publicados como página gerada? | catálogo mantido à mão diverge do código; gerado exige pipeline no `preparar.py` | arquitetura | `P3` | aberta | nenhum gerador de página no `wyd-docs`; `LPRT-20` depende desta resposta |
| `LPDOC-11` | Quando atualizar o `mkdocs 2.0.dev6`? | é pré-release sem data de lançamento; atualizar sem teste pode quebrar tema e navegação | arquitetura + operação | `P3` | proposta | `requirements.txt` fixa `2.0.dev6` (2026-09-15); upgrade exige rodar build e `verificar.py` antes |
| `LPDOC-12` | O build é só Linux/CI ou precisa rodar no Windows? | `mkdocs 2.0.dev6` instancia `PosixPath` e falha no Windows; hoje o build local exige Docker | operação | `P1` | aberta | erro reproduzido em 2026-11-19: `UnsupportedOperation: cannot instantiate 'PosixPath' on your system` |
| `LPDOC-13` | Busca de texto completo existe? | a 2.0 não tem plugin de busca; o site tem só filtro de navegação | arquitetura + produto | `P3` | aberta | nenhum índice de busca gerado; `verificar.py` não cobre busca |
| `LPDOC-14` | Até quando a ponte `compat.py` existe? | ela traduz sintaxe do MkDocs 1.x para GFM; enquanto existir, duas sintaxes convivem | arquitetura + documentação | `P2` | proposta | 22 admonitions, 7 abas, 1 detalhe, 3 grids e 67 tasklists convertidos em 2026-11-19 |

---

## 3. Como uma lacuna fecha

1. O dono decide e a decisão é escrita no documento dono (ou em `ADR-DOC-nnnn`, se muda forma do
   site, URL ou operação).
2. A linha ganha `fechada`, **data** e evidência (arquivo ou comando).
3. O plano diretor (§10) e a automação dependente são atualizados na mesma entrega.
4. Lacuna que muda de escopo ganha estado `retirada`, com motivo e data; o ID não é reaproveitado.
