> Pergunta: "que forma uma página publicada no site oficial precisa ter?"
> Público: Desenvolvedor novo
> Dono: equipe do wyd-docs
> Base: `wyd-docs-1.0.0.md` §3, §4, §6; `lacunas.md` §2.2–§2.4; `wyd-db/docs/convencoes.md` (molde)
> Revisado em: 2026-11-19

**Estado:** aceito (2026-11-19)

# wyd-docs — convenções editoriais

**Norma aceita pelo dono de `documentação` em 2026-11-19 (`V`; fase 3 em `wyd-docs-1.0.0.md`
§9).** O contrato abaixo está em vigor desde o aceite. As regras que dependem de decisão pendente
aparecem marcadas com o ID e continuam valendo como estão até a decisão: `LPDOC-03` (publicar
página `proposto`), `LPDOC-06` (redirect ao renomear), `LPDOC-07` (idioma), `LPDOC-08` (quem aprova
sistema novo) e `LPDOC-14` (ponte `compat.py`).

Este é o contrato de **escrita** de quem publica página no site. O contrato de **leitura** do leitor
é do sistema dono: `wyd-db/docs/convencoes.md` e `wyd-protocol/docs/convencoes.md` continuam donos do
que cada marca significa no domínio deles. Aqui só entra o que é do site.

> [!TIP] A regra que resolve quase tudo
> A página nasce no sistema dono, em `planejamento-1.0.0/<sistema>/docs/`, e o site a publica sem editar (`V`, 2026-11-19; páginas montadas e censo em `wyd-docs-1.0.0.md` §7.2). Se a correção precisa acontecer dentro de `wyd-docs/`, a página está no lugar errado.

## 0. Como a fase 3 se divide

A fase 3 é uma entrega em três páginas, cada uma com uma pergunta e um veredito próprio:

| Página | Pergunta que responde | Papel |
|---|---|---|
| `docs/convencoes.md` (esta) | que forma uma página publicada precisa ter | forma |
| `docs/guia-autor/index.md` | como se escreve e entrega uma página | processo |
| `docs/revisao.md` | o que o revisor confere e como o aceite fica registrado | veredito |

As três foram aceitas juntas pelo dono de `documentação` em 2026-11-19 (`V`): aceitar a forma e
recusar os outros dois deixaria autor sem processo ou revisor sem critério. O molde é o do `wyd-db`, que
separa o contrato de leitura (`wyd-db/docs/convencoes.md`) do guia de escrita
(`wyd-db/docs/guia-documentacao/`) — aqui a separação é entre forma, processo e veredito.

## 1. O que o site publica e o que não publica

A tabela origem → destino, com a regra de cada pasta, é de `wyd-docs-1.0.0.md` §3; é lá que ela
muda, e esta página não mantém segunda cópia. A consequência para quem escreve é a da regra acima: o
texto nasce na pasta `docs/` do sistema dono, e o que está fora dela é citado, não linkado.

## 2. Sintaxe permitida em página nova

Página escrita a partir de 2026-11-19 usa **só** o que a 2.0 renderiza sem plugin: GFM, citação de
aviso `> [!TIPO]` e cerca `mermaid` — a regra é de `wyd-docs-1.0.0.md` §6, junto do que a 2.0 não
tem e do que o `compat.py` converte no material antigo. A lista de recursos proibidos não se repete
aqui: é a de `wyd-docs-1.0.0.md` §6.

| Recurso | Sintaxe | Quando usar |
|---|---|---|
| aviso | `> [!TIP]`, `> [!WARNING]`, `> [!NOTE]`, `> [!IMPORTANT]` | regra, risco ou resumo de seção |
| tabela | pipes | comparar 2+ itens por atributo |
| código | cerca com linguagem | todo comando que o leitor vai executar |
| diagrama | cerca `mermaid` | relação que a tabela não mostra |
| link interno | caminho relativo `.md` | outra página publicada |
| caminho fora de `docs/` | `code`, sem link | alvo interno, não publicado |

Copiar a sintaxe antiga para página nova é defeito: ela passa pelo conversor hoje e some quando a
ponte sair (`LPDOC-14`, com estado em `lacunas.md` §1).

## 3. Cabeçalho e estado

Todo arquivo publicado abre com as cinco linhas do cabeçalho e, na linha seguinte, o estado:

```text
> Pergunta: "…"
> Público: …
> Dono: equipe do <sistema>
> Base: …
> Revisado em: AAAA-MM-DD

**Estado:** proposto (AAAA-MM-DD)
```

| Campo | O que promete |
|---|---|
| `Pergunta` | a dúvida única que a página responde |
| `Público` | para quem ela foi escrita; define o vocabulário que pode usar sem explicar |
| `Dono` | papel, nunca nome de pessoa: quem recebe a correção |
| `Base` | seção consultada no sistema dono; `—` quando não houve |
| `Revisado em` | data da última verificação, não a da última edição |

Estados: `proposto`, `aceito`, `substituído`, `retirado`. O site publica a página hoje qualquer que
seja o estado, e o estado viaja visível no topo — é o que `LPDOC-03` registra como situação atual;
se o dono decidir esconder rascunho, esta seção muda na mesma entrega.

## 4. Título, URL e navegação

Título, URL e navegação são de `wyd-docs-1.0.0.md` §4.1–§4.3 e não se redefinem aqui. O que a forma
acrescenta é o efeito sobre quem edita: uma heading `#` por página, porque sem ela `verificar.py`
reprova (`wyd-docs-1.0.0.md` §4.3); e renomear página publicada é mudança de contrato — até a
decisão de `LPDOC-06` (redirect), com estado em `lacunas.md` §1, o link externo quebra em silêncio e
o risco fica registrado na revisão.

## 5. Evidência

Toda afirmação de comportamento carrega `V`, `H` ou `P`, e o que cada marca exige é de
`AGENTS.md` §5, invariante 4 — a definição não se repete aqui. O que a forma acrescenta é onde a fonte fica: na
própria frase da afirmação, com data, e `H` sempre com o ID da lacuna que a reabre.

### 5.1 Números

Número sem fonte e data é hipótese, mesmo quando está correto. Formato:

```text
<N> páginas montadas (gate do site em container Linux, AAAA-MM-DD)
<N> lacunas abertas (`lacunas.md` §1, AAAA-MM-DD)
```

Contagem de página, arquivo ou sistema **não se copia** de outro documento: cita-se a seção dona
(`AGENTS.md` §4; total e censo do site em `wyd-docs-1.0.0.md` §7.2), porque duas cópias divergem na
primeira edição. Quando o número é histórico — o resultado de uma validação de commit antigo —, ele
**não** é atualizado: trocar o número medido numa tabela de validação de `0d765858…` falsificaria a
medição daquele commit.

### 5.2 Links e citações

| Situação | Como aparece |
|---|---|
| outra página publicada | link relativo, ex.: `../guia-autor/index.md` |
| código ou arquivo interno | texto simples em `code`, com caminho: `wyd-docs/preparar.py` |
| página planejada que ainda não existe | caminho em `code`, sem link: `docs/arquitetura/publicacao.md` |
| seção de documento dono | caminho + `§`: `wyd-docs-1.0.0.md` §7.2 |
| âncora dentro da página | `#secao-em-minusculas-com-hifen` |

Link que não resolve é defeito; caminho em `code` significa "vai existir e mora aqui". Trocar um pelo
outro para o build passar é o erro que o próprio gate precisa pegar: se o alvo não existe, mantém-se
`code`.

## 6. Um fato, um dono

| Assunto | Dono |
|---|---|
| o que o site publica, montagem, gate, publicação | `wyd-docs-1.0.0.md` |
| lacuna, dono e estado de decisão do site | `lacunas.md` |
| conteúdo de domínio | `planejamento-1.0.0/<sistema>/docs/` |
| número de página, arquivo e sistema | `AGENTS.md` §4; total e censo do site em `wyd-docs-1.0.0.md` §7.2 |
| marca de evidência no domínio | `convencoes.md` do sistema dono |

Esta página não repete DDL, opcode, regra de jogo nem contagem: aponta a seção. Página que copia o
conteúdo do dono cria a segunda boca — e a que divergir primeiro é a errada, mesmo que esteja certa.

## 7. O que reprova uma página publicada

- [ ] falta o cabeçalho de cinco linhas, ou a `Pergunta` não é respondida pelo corpo;
- [ ] falta `**Estado:** … (AAAA-MM-DD)` ou a data é a da edição, não a da atribuição;
- [ ] afirmação de comportamento sem `V`/`H`/`P`, ou `H` sem o ID que a reabre;
- [ ] número sem fonte e data;
- [ ] sintaxe antiga em página nova (`!!!`, `===`, `???`, `attr_list`, grid cards);
- [ ] link relativo sem destino, ou link para fora de `docs/` em vez de caminho em `code`;
- [ ] placeholder: `TODO`, "em construção", seção vazia, tabela "a preencher";
- [ ] adjetivo sem medida: "rápido", "robusto", "escalável";
- [ ] índice de conteúdo escrito à mão no topo, quando a navegação já o gera;
- [ ] segredo, DSN, token, hash ou dado de jogador no corpo, em exemplo ou em log citado;
- [ ] conteúdo de domínio reescrito aqui em vez de publicado no sistema dono;
- [ ] página sem segunda leitura.

## 8. O que esta página não decide

- Não decide o que é publicado: isso é de `wyd-docs-1.0.0.md` §3.
- Não decide quem aprova página de sistema novo (`LPDOC-08`) nem se página `proposto` aparece
  (`LPDOC-03`); o estado de ambas é de `lacunas.md` §1.
- Não decide redirect, idioma ou busca (`LPDOC-06`, `LPDOC-07`, `LPDOC-13`).
- Não decide sintaxe de domínio: offset, opcode e DDL são do `convencoes.md` de cada sistema.
- Não cria ADR: o aceite vale como norma editorial, não como decisão estrutural.
