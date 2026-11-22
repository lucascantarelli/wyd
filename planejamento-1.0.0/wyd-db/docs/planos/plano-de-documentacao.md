> Pergunta: "como esta documentação é produzida, em que ordem e com que prova?"
> Público: Equipe do `wyd-db`, arquitetura, produto e quem revisa página
> Dono: equipe do `wyd-db`
> Base: `wyd-db-1.0.0.md` §11; `proposta-paginas.md` §8; `lacunas.md` `D-A`; `plano-migrations.md` §9; `docs/convencoes.md`
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

Esta página é o **plano de produção da documentação**: diz qual é a ordem, o que entra e o que sai
de cada onda, quem responde por cada pasta e qual prova fecha uma página. Ela não escreve página nem
decide conteúdo de arquitetura; a fila do que já tem fonte é de `proposta-paginas.md` §8, e o
significado das marcas e dos estados é de [`../convencoes.md`](../convencoes.md).

## 1. Duas trilhas, uma timeline

A decisão `D-A` separou dois vocabulários que antes competiam:

| Trilha | O que é | Onde está a ordem | O que ela governa |
|---|---|---|---|
| **Onda** | documentação | §2 desta página, espelhando `wyd-db-1.0.0.md` §11 | quando uma página pode ser **aceita** e qual é o critério de saída do conjunto |
| **Etapa** | código | `plano-migrations.md` §9 | quando a série, o runner e o catálogo existem de fato |

A **timeline é única**: a onda e a etapa não são duas filas independentes. A única junção decidida é
**Etapa 1 → Onda 3** (`D-A`, fechada em 2026-11-19): a Onda 3 só fecha quando a Etapa 1 — as
ferramentas de catálogo e o leitor da série — existir. Até lá, páginas da Onda 3 podem ser escritas
com a forma `P`, mas não são declaradas aceitas por causa disso.

A Etapa 1 está **adiada desde 2026-11-17** (`plano-migrations.md` §9); a construção começa na Etapa 2.
Isso não cria outra junção: as demais etapas são execução da série e não movem uma onda por si sós.

## 2. As nove ondas

A tabela é o espelho operacional de `wyd-db-1.0.0.md` §11. A **contagem de arquivos e o conteúdo
detalhado de cada entrega são do §11**; esta página só diz o que precisa existir para abrir a onda e
o que ela precisa entregar para fechar. Nenhuma onda começa antes de a anterior fechar o critério de
saída.

| Onda | Entrada mínima | Saída | Critério de saída |
|---|---|---|---|
| **1. Fundação** | nada; é o vocabulário inicial | 15 páginas: `convencoes`, `glossario`, `docs/index`, o núcleo de `arquitetura/` e o guia de documentação | vocabulário fechado; fronteira Hot×Cold aprovada; toda página futura tem molde e regra de escrita |
| **2. Modelo** | Onda 1 fechada | 8 páginas de `arquitetura/`: componentes, implantação, fluxo, modelos e padrões | ER conceitual e lógico revisados; constraints conferidas contra o catálogo |
| **3. Requisitos** | Ondas 1–2 fechadas | `specs/` completo: 9 páginas de topo, 6 fluxos e 5 estados | todo `RF`/`RN`/`RNF` com critério de aceitação e destino na matriz |
| **4. Dicionário** | Onda 2 fechada + DDL fechado | 21 páginas de domínio + as 109 páginas por tabela, escritas à mão | **109 tabelas cobertas**; `gerado/index.md` é índice auxiliar, fora da contagem |
| **5. Histórico de schema** | Onda 2 fechada + DDL fechado | `migrations/` (índice, regras e 34 páginas) + `arquitetura/migracoes` + `retencao-e-capacidade` | 34 páginas com `down` e verificação, nomes 1:1; política da série em um único dono |
| **6. Operação e processo** | Onda 5 fechada | `guia-administrador/` (8) + `planos/{index,implantacao,manutencao,qualidade,riscos,metricas}` | todo procedimento com “como confirmar” e “como reverter”; métricas com fonte |
| **7. Dev e teste** | Ondas 4–5 fechadas | `guia-desenvolvedor/` (11) + `guia-teste/` (7) + `src/` (59) + `tests/` (12) | todo módulo com contrato e teste associado; todo alvo com página |
| **8. Planejamento do trabalho** | Onda 3 + requisitos com ID | `roadmap/` (2 + 40) + `planos/{documentacao,testes,dados,registro}` | 40 issues com DoD executável; fases com critério de saída |
| **9. Agente e publicação** | Onda 8 fechada | `prompts/` (9) + `skills/` (9) + navegação do `preparar.py` + ADRs encerradas | prompt e skill sem regra de negócio duplicada; o site publica só `docs/` |

As dependências duras são as de §11: Onda 3 depende de 1–2; Onda 4 depende de 2; Onda 7 depende de
4–5; Onda 8 depende de 3. A Onda 5 depende do DDL, não de uma página de operação; a Onda 9 depende de
8 porque a navegação publica o que as ondas anteriores escreveram. A Onda 5 inclui
`retencao-e-capacidade`, que só fecha depois da série medida; enquanto isso, a onda não é declarada
fechada.

## 3. Fila de escrita e o que já foi entregue

`proposta-paginas.md` §8 tem os **cinco lotes** de escrita (padrão, identidade, série e operação,
dicionário, índices). Eles não são uma sexta trilha: são a ordem em que as páginas com fonte já
verificada foram **puxadas para a frente** dentro das ondas. Uma página escrita cedo continua
pertencendo à onda de §11; o que muda é a data de escrita, não o critério de aceite.

**Estado medido em 2026-11-22:** **58 páginas autorais com corpo** em `docs/`, **0 escrevíveis**,
**21 de recorte** e **68 bloqueadas** — **147** páginas autorais. Os cinco lotes estão escritos;
`specs/index` foi antecipado no lote 2; esta página é a primeira fora dos lotes.

**Por que esta página veio depois dos lotes:** `§4` não tinha mais escrevível e, em `§6`,
`plano-de-documentacao.md` era o primeiro item cuja condição de bloqueio já estava resolvida
(`D-A`, fechada em 2026-11-19). Ela pertence à **Onda 8**, mas é escrita antes das outras páginas
da onda porque é o próprio plano que define a timeline. Escrevê-la **não fecha a Onda 8**: o
critério de saída da onda continua sendo os 40 issues com DoD e as outras páginas de planejamento.
Essa antecipação é de **uma página de plano**, não de uma entrega de roadmap.

**O que ainda bloqueia a fila:** os seis seeds `bloqueada` (`0016`–`0018`, `0020`, `0023`, `0027`)
esperam dado de origem; as páginas de `§6` esperam medição da série, suíte, issues ou decisão de
operação. Nenhuma delas é escrita por dedução; a fila e o motivo de cada grupo ficam em
`proposta-paginas.md` §6.

## 4. Dono por pasta

O dono do **conteúdo** é o do campo `Dono:` de cada página. Esta tabela agrupa a competência que
responde pela pasta e a revisão extra que ela exige; não substitui o cabeçalho de cada arquivo.

| Pasta | Dono do conteúdo | Revisão extra |
|---|---|---|
| `docs/` (raiz) | equipe do `wyd-db` | par que não escreveu; arquitetura para fronteira |
| `docs/arquitetura/` | equipe do `wyd-db` | arquitetura e par que não escreveu |
| `docs/specs/` | equipe do `wyd-db` | produto/gestão e arquitetura |
| `docs/dicionario-dados/` | equipe do `wyd-db` | curadoria de conteúdo para seed e catálogo |
| `docs/planos/` | equipe do `wyd-db` | operação para implantação e manutenção; QA para testes |
| `docs/roadmap/` | produto | arquitetura para dependência e DoD |
| `docs/guia-desenvolvedor/` | equipe do `wyd-db` | par que não escreveu |
| `docs/guia-administrador/` | equipe do `wyd-db` com a operação | operação |
| `docs/guia-teste/` | equipe do `wyd-db` | QA |
| `docs/guia-documentacao/` | equipe do `wyd-db` | par que não escreveu |
| `migrations/` | equipe do `wyd-db` | arquitetura e revisor de migration |
| `src/` | equipe do `wyd-db` | arquitetura para contrato de módulo |
| `tests/` | QA com a equipe do `wyd-db` | par que não escreveu |

## 5. Critério de pronto de uma página

Uma página só entra na onda quando passa nos itens abaixo; a prova é lida por quem revisa, não
presumida pelo autor.

| # | Item | Prova | Reprova quando |
|---:|---|---|---|
| 1 | cabeçalho de cinco linhas | `Pergunta`, `Público`, `Dono`, `Base`, `Revisado em` preenchidos | falta campo ou `Base:` vazio sem justificativa |
| 2 | estado com data | `**Estado:** <estado> (AAAA-MM-DD)` logo depois do cabeçalho | estado sem data ou data de edição usada como data de estado |
| 3 | pergunta respondida | a página responde à pergunta do cabeçalho e não abre outra | a resposta exige uma segunda pergunta de topo |
| 4 | evidência | todo comportamento tem `V`, `H` com ID ou `P` | presente do indicativo sem `V`; `H` sem ID |
| 5 | número com dono | toda contagem cita arquivo/comando e data | número solto, estimado ou repetido de outro dono |
| 6 | link e âncora | todo link relativo resolve e toda âncora existe | link para fora de `docs/`; destino inexistente |
| 7 | um fato, um dono | a página aponta o dono em vez de copiar DDL, tabela ou número | duas páginas podem divergir sobre o mesmo fato |
| 8 | sem enfeite | sem `TODO`, “em construção”, seção vazia, TOC manual ou adjetivo sem medida | placeholder ou afirmação sem prova |
| 9 | molde do tipo | página de migration segue os dez itens de `migrations-checklist.md` §5 e o DDL tem `-- migrate:up`/`-- migrate:down`; página de plano segue §6.9 | falta seção obrigatória, `down` ou prova de execução |
| 10 | revisão por par | alguém que não escreveu leu o diff e registrou o veredito | página sem segunda leitura |

Depois dos dez itens, o gate do site é a prova final: `preparar.py`, `mkdocs build`, `verificar.py`
e `verificar-contagens.py` precisam sair com **exit 0**. O comando e o container são os de
`wyd-docs-1.0.0.md` §7.1; a página não repete a linha de comando.

## 6. Revisão por par

A revisão é a de `AGENTS.md` §7 e do [`checklist de PR`](../guia-desenvolvedor/checklist-de-pr.md):
**quem escreveu não aprova a própria página**.

1. O autor escreve, roda o gate e abre o PR com a página, a fonte de cada número e o resultado do
   gate.
2. O revisor — que não escreveu — lê o diff contra §5 e registra os achados com página, trecho e a
   fonte correta.
3. O autor corrige no mesmo PR; achado que ele não aceita é respondido com a fonte que o sustenta.
4. O dono do `wyd-db` decide o estado `aceito`; esta página não aprova página nem fecha onda.
5. Achado que muda schema, fronteira ou operação vira decisão com dono e data; não se resolve
   “ajustando o texto para combinar”.

## 7. O que esta página não decide

- **A fila e o estado de cada página:** são de `proposta-paginas.md` §4–§6 e §8.
- **O conteúdo de arquitetura, requisito, migration ou operação:** é da página dona e das fontes de
  `wyd-db-1.0.0.md` §8.1.
- **A contagem e o percentual:** são de `metricas.md` (planejado); esta página só cita o estado da
  fila.
- **A aprovação de uma página ou de uma onda:** é do dono, com a revisão por par de §6.
- **O calendário:** não há data de onda; a ordem é de dependência, não de sprint.
