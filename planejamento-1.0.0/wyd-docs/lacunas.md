# `wyd-docs` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-19).** Este é o **único** lugar onde uma lacuna do site oficial
> tem dono, impacto, marco e estado.
>
> **O que este arquivo cobre:** as **14 lacunas** `LPDOC-01`–`LPDOC-14`: **0 abertas**,
> **0 propostas** e **14 fechadas**: `LPDOC-01`
> (rollback e restauração validados em 2026-11-19, `wyd-docs-1.0.0.md` §7.3), `LPDOC-04` (sem domínio
> próprio na `1.0.0`, decidido em 2026-11-19), `LPDOC-12` (build suportado em Linux/container,
> decidido em 2026-11-19), `LPDOC-05` (só a branch, sem `/1.0.0/`), `LPDOC-10` (sem catálogo gerado
> na `1.0.0`), `LPDOC-13` (sem busca de texto completo), `LPDOC-06` (redirect decidido e
> implementado em `wyd-docs/redirects.tsv`), `LPDOC-14` (a ponte foi removida em 2026-11-19),
> `LPDOC-08` (arquitetura aprova sistema novo e a entrada em `SISTEMAS`; documentação publica),
> `LPDOC-02` (não publicar `AGENTS.md` nem o panorama), `LPDOC-03` (publicar `proposto` com aviso
> visível), `LPDOC-07` (só pt-BR na `1.0.0`), `LPDOC-09` (gerar e publicar as 97 páginas, uma por tabela planejada) e
> `LPDOC-11` (manter `2.0.dev6` fixo até a próxima dev), decididas em 2026-11-19. O site foi montado
> em MkDocs 2.0 pré-release
> e publicado/validado em GitHub Pages em 2026-11-19. O dono aprovou GitHub Pages como hospedagem
> canônica com publicação contínua.
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
| Publicação e infraestrutura | 3 | 0 | 0 | 3 (`LPDOC-01`, `LPDOC-04`, `LPDOC-08`) |
| Conteúdo publicado | 3 | 0 | 0 | 3 (`LPDOC-02`, `LPDOC-03`, `LPDOC-09`) |
| URL, versão e idioma | 3 | 0 | 0 | 3 (`LPDOC-05`, `LPDOC-06`, `LPDOC-07`) |
| Ferramenta e sintaxe | 5 | 0 | 0 | 5 (`LPDOC-10`, `LPDOC-11`, `LPDOC-12`, `LPDOC-13`, `LPDOC-14`) |
| **Total** | **14** | **0** | **0** | **14** |

\"Proposta\" não é decisão: descreve o caminho recomendado para o dono aceitar ou recusar.

---

## 2. Lacunas

### 2.1 Publicação e infraestrutura

| ID | Pergunta | Por que importa | Dono | Marco | Estado | Evidência |
|---|---|---|---|---|---|---|
| `LPDOC-01` | Validar rollback da hospedagem canônica aprovada (GitHub Pages)? | confirmar que a publicação contínua pode ser revertida com segurança; hospedar em duas plataformas duplicaria automação | operação | `P1` | fechada | `V` (2026-11-19): dono aprovou GitHub Pages como hospedagem canônica com publicação contínua; PR #44 mesclado, workflow `docs` publicou e o site foi validado. Pré-checagens somente leitura: runs #2 (`44492052127`, SHA `2378b62b`) e #4 (`44498880453`, SHA `0d765858`) em `success` em `develop`, sem execução concorrente; o ambiente `github-pages` só tem política de branch `develop`, sem revisores; e os deployments #6 (`7910251866`) e #8 (`7910399204`) mostraram que um push novo publica por cima do estado anterior, o que exigiu a janela de congelamento; o dono confirmou o run #4 como SHA final e declarou a janela de congelamento. Validação local: os dois commits passaram `preparar.py`, `mkdocs build` e `verificar.py site` em container Linux (30 páginas, exit 0), diferindo só na URL do Highlight.js em `tema/site.js`. Teste na janela declarada: run #2 reexecutado publicou `2378b62b` (deployment `7911886524`, `success` `11:33:08Z`) e o site passou a servir `npm/highlight.js@11.12.0`; run #4 reexecutado restaurou `0d765858` (deployment `7911904845`, `success` `11:34:01Z`), com `cdn-release@11.12.0` servido e `200` na raiz e em páginas internas. Não medidos: erro de console e presença de `hljs`. Detalhe em `wyd-docs-1.0.0.md` §7.3. |
| `LPDOC-04` | O site terá domínio próprio? | URL publicada é contrato; trocar depois exige redirect permanente | operação + produto | `P1` | fechada | `V` (2026-11-19): decisão do dono — **sem domínio próprio na `1.0.0`**; publica em `https://lucascantarelli.github.io/wyd/`, e migração para domínio vira projeto separado, com redirect permanente. Sem `site_url` no MkDocs 2.0, a URL é a do Pages; a raiz e `assets/site.js` responderam `200 OK` na medição de §7.3. Plano em §9 e §10 de `wyd-docs-1.0.0.md`. |
| `LPDOC-08` | Quem aprova a publicação de um sistema novo? | sem dono, qualquer pasta vira página pública; com dono único, um sistema pode ficar meses fora do ar | documentação + arquitetura | `P2` | fechada | decisão do dono (`V`, 2026-11-19): **arquitetura aprova o sistema novo — nome, fronteira e entrada em `SISTEMAS`; documentação publica a página**. O ato técnico é a lista fixa de `preparar.py:23-37`: pasta fora dela não é copiada (`preparar.py:196-201`), e `AGENTS.md:100-101` define que pasta vazia é sistema não planejado. Sistema novo entra por PR que altera `SISTEMAS`, com revisão de arquitetura e publicação pela documentação |

### 2.2 Conteúdo publicado

| ID | Pergunta | Por que importa | Dono | Marco | Estado | Evidência |
|---|---|---|---|---|---|---|
| `LPDOC-02` | `AGENTS.md` e o panorama entram como página? | são normativos e úteis ao contribuidor, mas também expõem processo interno | documentação | `A` | fechada | decisão do dono (`V`, 2026-11-19): **não publicar** — processo interno fica no repositório e a cópia montada por `preparar.py` não é criada |
| `LPDOC-03` | Página com estado `proposto` aparece publicada? | esconder rascunho deixa o site incompleto; publicar sem aviso faz leitor citar intenção como regra | documentação | `A` | fechada | decisão do dono (`V`, 2026-11-19): **publicar com aviso visível** — `site.js` insere o aviso "Página em proposta" no topo; o estado continua no cabeçalho |
| `LPDOC-09` | As 97 páginas por tabela do `wyd-db` entram no site? | é o maior volume do site; a publicação está decidida, mas a **produção** das páginas voltou à mesa em 2026-11-21 | arquitetura + documentação | `P3` | fechada (publicação); produção em `M26` | decisão do dono (`V`, 2026-11-19), reconciliada com `plano-migrations.md` §3: **publicar uma página para cada uma das 97 tabelas planejadas**; não há exceção de tabela, e `gerado/index.md` é índice auxiliar fora do teto. O **modo de produção** é a lacuna `M26` do `wyd-db` (`planejamento-1.0.0/wyd-db/lacunas.md` §2.3): o gerador deixou de ser presumido e nenhuma ferramenta é criada sem aprovação. Nenhuma das 97 páginas existe hoje |

### 2.3 URL, versão e idioma

| ID | Pergunta | Por que importa | Dono | Marco | Estado | Evidência |
|---|---|---|---|---|---|---|
| `LPDOC-05` | O site é versionado por release (`/1.0.0/`) ou publica só a branch? | versionar exige gerador de versão — que a 2.0 não tem como plugin | arquitetura | `P3` | fechada | decisão do dono (`V`, 2026-11-19): **só a branch, sem `/1.0.0/`** — uma versão viva; versionar só se houver release de documentação. `preparar.py` monta uma árvore única (`MONTAGEM = .build/docs`) e `_copiar_paginas` copia sem versionar; `requirements.txt` fixa `mkdocs==2.0.dev6`, sem plugin de versão, e o `docs.yml` publica na raiz do Pages |
| `LPDOC-06` | Renomear página publicada exige redirect? | URL é contrato; sem redirect, link externo quebra em silêncio | arquitetura | `P2` | fechada | decisão do dono (`V`, 2026-11-19): **sim** — URL publicada é contrato. Implementado: `wyd-docs/redirects.tsv` (mapa `origem<TAB>destino`), stub gerado no `preparar.py` com `meta refresh` e `canonical` (colisão com página real reprova) e `verificar.py` reprovando origem sem stub, destino inexistente, `refresh`/`canonical` fora do destino, destino que é outro stub e origem duplicada. Teste de 2026-11-19: redirect temporário passou com 35 páginas; destino inexistente e colisão saíram com `exit 1`; com o mapa vazio (só comentários), gate de 34 páginas, `exit 0` |
| `LPDOC-07` | Um idioma ou i18n? | hoje é português; público internacional exigiria árvores paralelas e tema próprio | produto | `P3` | fechada | decisão do dono (`V`, 2026-11-19): **só pt-BR na `1.0.0`**; i18n fica fora desta versão |

### 2.4 Ferramenta e sintaxe

| ID | Pergunta | Por que importa | Dono | Marco | Estado | Evidência |
|---|---|---|---|---|---|---|
| `LPDOC-10` | O catálogo de opcodes e o dicionário de dados são publicados como página gerada? | catálogo mantido à mão diverge do código; gerado exige pipeline no `preparar.py` | arquitetura | `P3` | fechada | decisão do dono (`V`, 2026-11-19): **não publicar catálogo gerado na `1.0.0`**; o catálogo escrito à mão fica como está e o gerador vem depois. `preparar.py` não tem gerador — `_copiar_paginas` só copia `docs/`; a resposta fecha o lado do site do `LPRT-20` (`wyd-protocol/lacunas.md` §2), cuja linha fechou em 2026-11-20 junto com `LPRT-01` |
| `LPDOC-11` | Quando atualizar o `mkdocs 2.0.dev6`? | é pré-release sem data de lançamento; atualizar sem teste pode quebrar tema e navegação | arquitetura + operação | `P3` | fechada | decisão do dono (`V`, 2026-11-19): **manter `2.0.dev6` fixo e revisar quando a próxima dev for publicada**, com build e `verificar.py` antes de subir |
| `LPDOC-12` | O build é só Linux/CI ou precisa rodar no Windows? | `mkdocs 2.0.dev6` instancia `PosixPath` e falha no Windows; hoje o build local exige Docker | operação | `P1` | fechada | decisão do dono (`V`, 2026-11-19): build suportado = Linux/container, Windows fora de escopo; falha reproduzida em 2026-11-19 (`UnsupportedOperation: cannot instantiate 'PosixPath' on your system`) e o mesmo comando passou em container Linux nos commits `2378b62b` e `0d765858` (30 páginas, exit 0); §7.1 declara o escopo |
| `LPDOC-13` | Busca de texto completo existe? | a 2.0 não tem plugin de busca; o site tem só filtro de navegação | arquitetura + produto | `P3` | fechada | decisão do dono (`V`, 2026-11-19): **não ter busca de texto completo na `1.0.0`**; o filtro de navegação é a navegação e busca entra se o volume de páginas justificar. `requirements.txt` sem plugin de busca, `verificar.py` sem checagem de busca (`grep -c 'busca\|search'` = 0, 2026-11-19) e nenhum índice gerado |
| `LPDOC-14` | Até quando a ponte `compat.py` existiu? (removida em 2026-11-19) | ela traduz sintaxe do MkDocs 1.x para GFM; enquanto existir, duas sintaxes convivem | arquitetura + documentação | `P2` | fechada | decidida em 2026-11-19 por arquitetura + documentação: a ponte sai quando `grep -rE '^[[:space:]]*!!!' planejamento-1.0.0/*/docs --include='*.md'` der 0. Medido em 2026-11-19: **0 admonitions (`!!!`), 0 abas (`=== "A"`), 0 detalhes (`???`)** nas páginas publicadas. Restavam 4 abas, 2 botões, 1 grid de cartões, 42 tasklists e 1 ícone que a ponte ainda traduzia; foram convertidos na origem com saída byte a byte idêntica (`.build/docs` contra a linha de base). **`wyd-docs/compat.py` e a chamada em `preparar.py` foram removidos em 2026-11-19**: `_copiar_paginas` copia o arquivo como está e o gate passou em 34 páginas (exit 0). Varredura em 2026-11-19 fora de `.venv`/`.build` (`git grep -InE 'compat\.py|import compat|from compat|compat\.converter'`, mais `git ls-files '*.py' '*.sh' 'Makefile'`): **0 import, 0 chamada, 0 script e 0 teste da ponte**; as únicas ocorrências são este registro, o índice deste arquivo e a linha da fila que anuncia a remoção

---

## 3. Como uma lacuna fecha

1. O dono decide e a decisão é escrita no documento dono (ou em `ADR-DOC-nnnn`, se muda forma do
   site, URL ou operação).
2. A linha ganha `fechada`, **data** e evidência (arquivo ou comando).
3. O plano diretor (§10) e a automação dependente são atualizados na mesma entrega.
4. Lacuna que muda de escopo ganha estado `retirada`, com motivo e data; o ID não é reaproveitado.
