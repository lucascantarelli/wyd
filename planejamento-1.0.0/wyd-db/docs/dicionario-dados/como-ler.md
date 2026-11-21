> Pergunta: "como leio uma página do dicionário, o que cada marca significa e quem manda quando o texto e o DDL divergem?"
> Público: Desenvolvedor novo e revisor de documentação do `wyd-db`
> Dono: equipe do wyd-db
> Base: `wyd-db-1.0.0.md` §6.8, §10; `docs/convencoes.md`; `lacunas.md` §2.3 (`M26`)
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21) — as páginas por tabela são escritas à mão; quantas das 97 existem diz
[`gerado/index.md`](gerado/index.md), dono único da contagem.

## Duas camadas, um dono cada

O dicionário tem **duas camadas** e elas não competem:

| Camada | Responde | Onde |
|---|---|---|
| domínio | por que a tabela existe, como o dado nasce e morre, o que quebra se mudar | `dominio-<assunto>.md` |
| tabela | o que a tabela tem: colunas, chaves, FKs, únicos, `CHECK`s, índices, papéis | `gerado/<tabela>.md` |
| forma | o DDL palavra por palavra | `migrations/<arquivo>.md` |

A regra é **forma na migration, significado no texto**. A página por tabela **resume e liga**; ela não
copia o DDL, porque duas cópias do mesmo DDL divergem e uma passa a mentir.

## Anatomia de `gerado/<tabela>.md`

Sempre nesta ordem:

1. **papel em uma linha** — o que a tabela é, não o que ela tem;
2. **colunas** — nome, tipo, nulidade e default, com observação só quando o nome não basta;
3. **PK** — a chave e o que ela impede;
4. **FKs de saída** — para onde aponta e com qual `ON DELETE`;
5. **FKs de entrada** — quem aponta para ela, e se isso a torna impossível de apagar;
6. **unicidade** — índices únicos, inclusive os parciais e os `NULLS NOT DISTINCT`;
7. **`CHECK`s** — o que o schema recusa, em uma linha cada;
8. **índices** — os de leitura, com a consulta que os justifica;
9. **papéis** — quem escreve, quem lê, quem não lê;
10. **migration** — o arquivo que criou a tabela, em `code`.

O cabeçalho de 5 campos (`Pergunta`, `Público`, `Dono`, `Base`, `Revisado em`) e a linha `Estado` com
data são obrigatórios.

## O que cada marca significa

- **`V`** — verificado por comando ou arquivo, com a data. É o único que pode aparecer no presente do
  indicativo como fato.
- **`H`** — hipótese, com o ID da lacuna que a fecha. Sem o ID, não é `H`: é afirmação sem dono.
- **`P`** — planejado: desenhado e ainda não aplicado.

Campo sem marca em página de tabela é leitura direta do DDL confirmado — e o DDL é `P` até a série
rodar. A distinção está em [`../convencoes.md`](../convencoes.md).

## Quando o texto e o DDL divergem

**O DDL vence.** Constraint, tipo e índice são forma, e a forma é da migration. Divergência não se
resolve "corrigindo o texto para combinar": abre-se lacuna, o dono decide qual dos dois está errado, e
só então um dos dois muda. Corrigir o texto para esconder a diferença é o defeito que este dicionário
existe para evitar.

## O que a página por tabela não faz

- não decide coluna, `CHECK` nem índice — isso é `ddl-confirmado.md` + `lacunas.md`;
- não mantém escala de estado: `pronto`/`provisório`/`planejado` derivam de `migrations-checklist.md`
  §1.2, dono único;
- não descreve código que ainda não existe (`repo/`, runtime);
- não guarda segredo, DSN, hash ou PII.

## Como as páginas nascem

**Escritas à mão, em lotes** (`M26`, 2026-11-21): sem gerador, sem `Makefile` e sem script. Um lote por
domínio, depois do DDL do arquivo fechado, e cada página entra conferida contra a página de
`migrations/` dona da forma. `gerado/index.md` é o índice auxiliar, fora da contagem das 97.

## Checklist de pronto de uma página

1. cabeçalho de 5 campos e `Estado` com data;
2. tabela e arquivo citados existem e batem com `ddl-confirmado.md`;
3. toda afirmação tem `V`, `H` (com ID) ou `P`;
4. a forma é **ligada** à migration, não copiada;
5. sem PII, sem segredo, e link que resolve no site.
