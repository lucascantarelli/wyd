> Pergunta: "como leio estas páginas e o que cada marca significa?"
> Público: Desenvolvedor novo
> Dono: equipe do wyd-db
> Base: —
> Revisado em: 2026-11-17

Esta página é o contrato de **leitura** da documentação do `wyd-db`. Ela diz o que cada marca
significa e o que você tem direito de concluir de uma frase. As regras de **escrita** ficam em
`guia-documentacao/`, pasta que ainda não foi escrita; aqui interessa o que o leitor pode exigir.

!!! tip "A regra que resolve quase tudo"
    Toda afirmação de comportamento carrega um nível de evidência. Se a frase fala de
    comportamento no presente do indicativo e não tem `V` no contexto, trate como hipótese:
    o verbo correto seria "pretende", não "faz".

## Marcações de evidência

| Marca | Significa | O que você pode concluir |
|---|---|---|
| `V` | verificado por comando, arquivo, schema ou banco, com caminho e data | o fato foi medido nesta árvore; pode apoiar decisão |
| `H` | hipótese explícita | não é regra; a página diz o que falta para fechar |
| `P` | planejado: a coisa ainda não existe | descreve intenção; não conte com isso no código |

Uma marca vale para a afirmação a que está colada, não para a página inteira. Quando a página
mistura os três níveis, cada bloco traz a sua marca — um `V` no topo não empresta força ao que vem
depois.

## Níveis de afirmação na página

Toda página que mistura planejamento, hipótese e fato medido abre com a legenda dos níveis, antes da
primeira afirmação. Ela existe hoje em duas formas — "Três níveis de afirmação convivem aqui" em
`padroes-postgres.md` e `integridade-e-constraints.md`, com `V` acrescentado em
`como-adicionar-tabela.md` — e as demais páginas usam as marcas sem declarar o que "sem marca" quer
dizer.

**Forma:** lista dos níveis que a página usa, na ordem `P`, `H`, `V`, com `sem marca` por último — cada
um dizendo o que o nível cobre **nesta** página e de onde vem a fonte. A legenda fica logo depois da
linha de estado. O significado das marcas é o da tabela acima; o que a legenda acrescenta é a **fonte
local** de cada nível:

| Nível | Onde a fonte está |
|---|---|
| `P` | `plano-migrations.md`, `migrations-checklist.md` |
| `H` | `lacunas.md`, item citado |
| `V` | comando, arquivo ou banco, com caminho e data |
| sem marca | `padrao-banco.md` §13 ou documentação oficial |

Quatro regras:

- **Página de um nível só não precisa da legenda.** Se tudo é `P`, a linha de estado já diz isso
  (`proposto`), e quatro itens para repetir "tudo aqui é planejado" é ruído.
- **`H` sem ID não é hipótese, é omissão.** A marca vale quando nomeia a lacuna que a fecha; sem ela,
  o leitor não tem o que cobrar.
- **"Sem marca" não é dispensa de fonte.** Comportamento do PostgreSQL entra sem marca porque é
  plataforma, e a prova é `padrao-banco.md` §13 — não porque ninguém verificou.
- **A legenda não se repete no corpo.** Depois dela, a marca vai colada na afirmação (seção acima),
  não em cada parágrafo.

!!! warning "`V` emprestado é o erro que a legenda evita"
    Um `V` medido num arquivo não empresta verificação ao parágrafo seguinte sobre o banco. Quando o
    nível muda, o texto diz onde muda; página que não sabe dizer qual afirmação é qual não está
    pronta para revisão.

## Estados

**Estado da página** — vale para o arquivo como um todo.

| Estado | Significa | Você pode citar? |
|---|---|---|
| `proposto` | escrito, aguardando aprovação | com ressalva |
| `aceito` | aprovado e vigente | sim |
| `substituído` | trocado por outro ID, com link para o novo | só o substituto |
| `retirado` | não vale mais; a página diz motivo e data | não |

**Forma:** primeira linha depois do cabeçalho, `**Estado:** <estado> (AAAA-MM-DD)`, com a data em que
o estado foi atribuído — não a da última edição. Página sem essa linha é rascunho, e `proposto` sem
data não diz há quanto tempo espera revisão.

**Estado de tabela e coluna** — vale para o dicionário de dados.

| Estado | Significa |
|---|---|
| `pronto` | coluna, tipo e constraint existem na migration aplicada (`schema_migrations` com o checksum vigente); a página de `gerado/`, quando existir, só reflete isso |
| `provisório` | existe, mas há decisão pendente ou valor herdado do legado sem confirmação; a página diz qual |
| `planejado` | prevista e ainda não aplicada ao schema |

Esta página define **o significado** das três palavras, não o valor de cada tabela. A **derivação** —
qual estado de arquivo vira qual estado de tabela — é de `migrations-checklist.md` §1.2, dono único, e
não se copia aqui. O valor de cada tabela também não: página de domínio que precise dele cita a seção
e a linha do arquivo.

## Números

Número sem fonte e data é hipótese. Todo número aparece com onde foi medido e quando, neste
formato:

```text
<N> migrations (crates/wyd-db/migrations/, AAAA-MM-DD)
<N> tabelas (crates/wyd-db/migrations/, AAAA-MM-DD)
5 alvos de teste (crates/wyd-db/Cargo.toml, 2026-11-17)
SCHEMA_VERSION = <N> (crates/wyd-db/src/lib.rs:29, AAAA-MM-DD)
```

Os exemplos acima eram `53`, `87` e `53` enquanto a série herdada estava vigente. Com a série
reescrita do zero (plano diretor §1.2), o número só entra **depois** da medição: `N` sem data é
hipótese, não exemplo.

Se a página gerada, a migration e o texto discordarem, **a migration prevalece**. A divergência é
defeito registrado, não interpretação: corrija a origem, nunca o número no texto.

## Links e citações

| Situação | Como aparece |
|---|---|
| outra página desta árvore | link relativo, ex.: `../arquitetura/visao-geral.md` |
| código do crate | texto simples, não link: `crates/wyd-db/src/lib.rs:29` |
| página planejada que ainda não existe | caminho em `code`, sem link: `dicionario-dados/dominio-personagem.md` |
| âncora dentro da página | `#secao-em-minusculas-com-hifen` |

O plano (§5.1) permite link para fora de `docs/` (`migrations/`, `src/`, `tests/`, `prompts/`,
`skills/`) **com a marca "planejamento"**, porque o site publica só `docs/` e o alvo não é
publicado. Na prática isso é caminho em `code` precedido da marca, nunca link relativo — link
relativo para fora de `docs/` reprova (`mkdocs build --strict`), e link para `.agents/` é proibido
em qualquer forma (§8.4).

!!! warning "Link que não resolve é defeito"
    Caminho em `code` significa "vai existir e é aqui que ele mora". Link significa "abra agora".
    Trocar um pelo outro para o build passar é o erro mais comum: se o alvo não existe, mantenha
    `code`.

## Nota de enunciação

Quando o valor de uma seção de arquivo de raiz passa a ser enunciado numa página de `docs/`, a seção
**não some e não se cala**: ganha uma nota de enunciação ao fim, antes do `---`. O padrão existe
hoje em seis seções de `padrao-banco.md` (§0, §4, §5, §6, §8 e §9) e vale para qualquer arquivo de
raiz cuja seção tenha casa nova. Enquanto a árvore de planejamento existir, as notas
valem; quando ela sair, elas saem com ela — não são conteúdo, são o registro de uma mudança de dono.

**Forma:** `**Enunciação (AAAA-MM-DD):**` em negrito, no começo da linha, seguida de até seis linhas
de parágrafo. A data é a da mudança de dono, não a da última edição da seção.

A nota responde a três perguntas, nesta ordem:

| Pergunta | O que a nota diz |
|---|---|
| **quem enuncia?** | a página e a **seção** dela, pelo nome — `docs/arquitetura/padroes-postgres.md` §2, nunca "a página de padrões" |
| **o que fica aqui?** | o que a página não enuncia e esta seção continua sendo dona: a lista fechada, o modelo de permissão, o porquê das regras |
| **como as duas mudam?** | a trava: mudança de decisão altera as duas na mesma entrega, e divergência é defeito a corrigir na página — não interpretação |

Quatro regras que a nota não pode quebrar:

- **Nota é para seção cuja enunciação foi substituída**, não para seção apenas citada — nem para
  seção que a página só ensina a reconhecer. O teste é a **norma**, não o tema: se a página enuncia o
  que **decide** (o critério, a proibição, o modelo), a nota é devida; se ela dá apenas a **chave de
  leitura** — como agrupar, como reconhecer no diff, em que ordem revisar —, a seção segue dona única
  e não há nota. Por isso `padrao-banco.md` §6 tem nota (a página §5 diz quando um índice pode
  existir) e §11 não tem (a página §7 só agrupa os antipadrões em quatro famílias de sintoma e declara
  que a lista é de lá); §7 também não tem, porque nenhuma página tomou o inventário de objetos.
- **"Versão de bolso" não é versão aceita.** Se a nota precisa dizer que a lista "ficou resumida
  aqui", o resumo é o defeito: ou a seção aponta e não redigita, ou a lista é dela e a página é que
  aponta. Foi assim que §9 de `padrao-banco.md` colapsou a ponteiro.
- **A nota não autoriza seção vazia.** Citar o dono não dispensa preencher o critério próprio — o
  molde da página continua exigindo o que ele exige, e seção que só aponta reprova.
- **Sem ponteiro prometido.** Se a nota diz que a página aponta para cá, a página aponta de fato: com
  a seção no `Base:` ou numa citação do corpo. Promessa de mão dupla que não existe é defeito.

!!! warning "Nota desatualizada é pior que nota ausente"
    Quando o dono muda de novo, a nota antiga passa a afirmar o contrário do arranjo vigente. Foi o que
    aconteceu quando §9 virou ponteiro: a nota ainda dizia que as regras da série ficavam ali, e a
    primeira correção foi a própria nota. Ao mudar um dono, revise a nota que o declarava **antes** de
    revisar o corpo.

## Coluna de dono

Tabela que enuncia regra, decisão ou estado de outro dono termina com uma coluna que nomeia esse dono.
É o que impede a linha de virar parágrafo de regra: a linha diz **o quê**, a coluna diz **onde mora**.
O nome é **`Regra em`** (`padroes-postgres.md` §1 e §7, `como-adicionar-tabela.md` §1,
`integridade-e-constraints.md` §3). As variantes `Enunciada em` e `Regra de origem` foram convergidas
em 2026-11-18, sem mudança de conteúdo.

**Forma:** última coluna, cabeçalho `Regra em`, célula com dono **e seção** — `padrao-banco.md` §5,
item 3; `integridade-e-constraints.md` §3; `M7` quando a decisão está em `lacunas.md`.

| A linha é… | A célula leva |
|---|---|
| regra de arquivo de raiz | arquivo + `§` + item: `padrao-banco.md` §5, item 3 |
| critério de outra página | página + `§`: `padroes-postgres.md` §2 |
| decisão fechada | ID + estado: `M23`, fechada |
| ausência de decisão | `H` + o ID que a reabre |

Regras:

- **A célula não repete a regra.** Predicado, tipo e texto de constraint são da página dona; a linha
  que os copiasse criaria a segunda boca que a *Nota de enunciação* proíbe.
- **Sem `§` a célula é ambígua.** "`padrao-banco.md` §5" vale; "o padrão de banco" não diz onde
  corrigir.
- **Tabela de registro não usa esse nome.** `lacunas.md` §1 e `rascunho-accounts.md` §6 indexam
  decisão com `Refletida em` e `Onde vive hoje` — mesma função, nome próprio, porque a linha é
  índice e não regra. Esses dois nomes não convergem para `Regra em`.

!!! warning "Coluna preenchida com parágrafo é redigitação"
    Célula que explica a regra em vez de apontá-la faz a tabela divergir do dono na primeira mudança.
    Se a explicação é necessária, ela é da página dona e não da célula.

## Cabeçalho de página

Todo arquivo começa com cinco linhas. O plano (§7) escreve "bloco de cabeçalho de 4 linhas" e
lista cinco campos: vale a lista, e a contagem é divergência do plano, registrada e não corrigida
nesta página. Cada campo promete uma coisa:

| Campo | O que promete |
|---|---|
| `Pergunta` | a dúvida única que a página responde; se você chegou com outra, está na página errada |
| `Público` | para quem a página foi escrita; define o vocabulário que ela pode usar sem explicar |
| `Dono` | **papel**, não nome de pessoa nem caminho: quem responde pela exatidão e recebe a correção; nestas páginas, "equipe do `wyd-db`" |
| `Base` | bloco de análise consultado; rastro interno, sem link — `—` quando não houve |
| `Revisado em` | data da última verificação; número mais velho que a migration que ele descreve está vencido |

Página sem esse cabeçalho não é `aceito`, mesmo que o texto esteja correto.

## Bloco `Base:`

`Base:` é a quarta linha do cabeçalho e o plano (§7.1) a chama de rastro de procedência. Na prática
as páginas a usam para mais: listam as **seções** de onde o conteúdo veio, e é por ela que a mão
dupla da *Nota de enunciação* fecha — a página que recebe uma seção a nomeia ali.

**Forma:** `> Base: dono §seção, dono §seção` ou `> Base: —`. Nomeia seção, não só arquivo
(`padrao-banco.md` §0–§7, §11); intervalo é aceito. Página planejada entra em `code` com a marca
"planejamento" (seção *Links e citações*). Nunca link.

Regras:

- **`Base:` é o que foi consultado, não o que é citado.** Citação vive no corpo; `Base` é
  procedência.
- **Seção de raiz enunciada pela página aparece no `Base:`** — é o que cumpre a quarta regra da
  nota, a do ponteiro de mão dupla.
- **"O plano" não é `Base`.** Sem seção, o rastro não permite achar a origem; `§` é obrigatório
  quando o alvo é página desta árvore.
- **`—` é declaração, não esquecimento.** Só a página que não nasceu de outra página (`convencoes`,
  `glossario`, `index`) usa; as demais nomeiam o que leram.

## Seção "O que esta página não decide"

Página que muda o dono de uma decisão fecha com a lista do que ela **não** decide. Ela existe em
`padroes-postgres.md` §10, `integridade-e-constraints.md` §9 e `como-adicionar-tabela.md` §7, e o
molde do plano (§7.2) não a prevê: é convenção de prática, não campo obrigatório.

**Forma:** última seção numerada, título fixo `O que esta página não decide`, de quatro a seis itens;
cada item é **uma fronteira + o dono** — "Não enuncia DDL. Predicado e nome são de
`ddl-confirmado.md` §3–§5".

Regras:

- **Item sem dono é desabafo.** "Não decide produto" não serve; "o que é permitido ao jogador é de
  `specs/` (planejado)" serve.
- **Não repete o que a página já disse.** A seção só fecha o que ficou fora; item que reenuncia o
  corpo é ruído.
- **Não substitui seção obrigatória.** Ter a lista não dispensa o critério próprio — é a terceira
  regra da nota aplicada ao molde.
- **A ausência também informa.** Página de tipo que não decide nada fora do próprio assunto (ADR,
  issue) não ganha a seção por simetria.

## Diagramas

O padrão é **Mermaid**, um tipo por diagrama e legenda em português. O fence precisa estar habilitado
no config do site que publica esta árvore — item da frente `wyd-docs`, que ainda não existe (`EXT`):

```yaml
markdown_extensions:
  - pymdownx.superfences:
      custom_fences:
        - name: mermaid
          class: mermaid
          format: !!python/name:pymdownx.superfences.fence_code_format
```

Regra para o próximo diagrama:

| Pergunta | Tipo | Onde mora |
|---|---|---|
| quem fala com quem, em que ordem? | `sequenceDiagram` | `specs/fluxos/*` |
| que estados a entidade tem? | `stateDiagram-v2` | `specs/estados/*` |
| como as tabelas se relacionam? | `erDiagram` | `arquitetura/modelo-*`, `dicionario-dados/*` |
| como o trabalho flui? | `flowchart` | `arquitetura/*`, `planos/*` |
| quem usa o quê? | `C4Context` | `arquitetura/contexto-c4.md` |

- Um tipo por diagrama; nada de fluxograma dentro de diagrama de estado.
- Rótulos em português; identificador técnico pode ficar em `snake_case`.
- Bloco `mermaid` que não renderiza no build é **defeito bloqueante** (`wyd-db-1.0.0.md` §10, item 6), não estilo.
- Diagrama que só repete a tabela ao lado não entra: escolhe-se um dos dois.

## Recursos do site (habilitados e de uso obrigatório)

O site habilita os recursos abaixo, e usar o recurso certo é parte da página, não enfeite: regra
escrita em parágrafo onde cabia `!!! warning` está escondida do leitor.

| Recurso | Sintaxe | Uso obrigatório quando |
|---|---|---|
| Admonition | `!!! tip` / `warning` / `danger` / `abstract` | regra, risco, decisão ou resumo de seção — cor por tipo: `tip` verde, `warning` âmbar, `danger` vermelho, `abstract` turquesa |
| Tabela | pipes | comparar 2+ itens por atributo; nunca lista disfarçada de tabela |
| Content tabs | `=== "Rust"` | alternativas do mesmo passo (comando por sistema, cliente × servidor) |
| Details | `??? note` | detalhe que quebraria o fluxo de leitura |
| Footnotes | `[^1]` | fonte externa citada no texto |
| Código com cópia | cerca com linguagem | todo comando que o leitor vai executar |
| Botão | `<a class="md-button" href="...">` | ação única no topo da página (abrir painel, rodar gate) |
| Grid cards | `<div class="grid cards" markdown>` | índice de pasta com 4+ páginas |
| Emoji e ícone Material | `:material-database:` | sinalizar tipo de conteúdo; um por título, nunca no meio da frase |
| Mermaid | cerca `mermaid` | qualquer diagrama (seção acima) |

Página que desenha diagrama em bloco `text`, ou que lista em parágrafo o que deveria ser
tabela/admonition, volta para rascunho na revisão.

## O que reprova uma página

Use esta lista para julgar o que está lendo:

- [ ] falta o cabeçalho, ou a pergunta do cabeçalho não é respondida pelo corpo;
- [ ] afirmação de comportamento sem marca `V`/`H`/`P`; página que mistura os níveis sem a
  legenda; `H` sem o ID que o reabre;
- [ ] número sem fonte ou sem data;
- [ ] link relativo sem destino, ou link para fora de `docs/` sem a marca "planejamento";
- [ ] nota de enunciação ausente onde a enunciação mudou de dono, ou nota que promete ponteiro que a
  página não faz;
- [ ] tabela que enuncia regra sem a coluna `Regra em`, ou célula que copia o texto da regra;
- [ ] placeholder: `TODO`, "em construção", seção vazia, tabela com coluna "a preencher";
- [ ] adjetivo sem medida: "rápido", "robusto", "escalável";
- [ ] índice de conteúdo escrito à mão no topo;
- [ ] regra de negócio em página de arquitetura, ou operação em página de requisito;
- [ ] espelho 1:1 do código, arquivo por arquivo, sem contrato nem invariante;
- [ ] segredo, DSN, token ou hash no corpo, num exemplo ou num log citado;
- [ ] sem revisão de alguém que não escreveu — página sem segunda leitura não fecha;
- [ ] fora do `index.md` da pasta, ou não contabilizada em `planos/metricas.md`;
- [ ] `Base:` vazio, `Base:` que nomeia arquivo sem a seção quando a página enuncia seção de raiz, ou
  afirmação que só se sustenta em material fora da árvore.

Falha nos quatro primeiros critérios de `wyd-db-1.0.0.md` §10 devolve a página a rascunho; falta de link, diagrama
quebrado, segredo ou placeholder é defeito bloqueante; revisão, índice, métrica e `Base:` são
pendência de fechamento.

## Quando a página está errada

Registre o defeito com página, trecho e fonte correta. O destino é `planos/riscos.md`, que ainda
não existe: até lá, o registro vai na issue do roadmap (`WYDDB-nnn`) que cobre a página.
