> Pergunta: "como escrevo uma página desta documentação?"
> Público: Quem escreve ou revisa página do `wyd-db`
> Dono: equipe do `wyd-db`
> Base: `wyd-db-1.0.0.md` §3; `docs/convencoes.md`; `docs/glossario.md`; `wyd-docs-1.0.0.md` §6
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

Esta página é o **manual de escrita**: voz, tempo, tamanho, termos e os cinco pares que mostram a
diferença entre uma frase que passa e uma que volta para rascunho. O contrato de **leitura** — o que
`V`, `H` e `P` significam, o que é um estado e como o cabeçalho é lido — é de
[`convencoes.md`](../convencoes.md); aqui se escreve, lá se confere.

## 1. Voz e tempo

| Situação | Forma | Exemplo |
|---|---|---|
| fato verificado (`V`) | presente do indicativo, com a prova e a data | “O portal recebe `permission denied` ao escrever em `game` (`plano-migrations.md` §7, 2026-11-22).” |
| planejado (`P`) | “pretende”, “vai”, “está planejado”; nunca “funciona” | “O runner **pretende** aplicar a série em transação por arquivo.” |
| hipótese (`H`) | “a hipótese é”, com o ID que a reabre | “A hipótese é que o cliente envia o login na mesma caixa (`A11`).” |
| passo a passo | imperativo | “Rode `provision render`; confira o `db/roles.sql`.” |
| decisão do dono | terceira pessoa, com data | “O dono decidiu, em 2026-11-21, que o papel `wyd_admin` tem DSN própria (`M27`).” |

Nunca use primeira pessoa (“eu”, “nós”) para falar pelo time: a página não é ata de reunião. Quando
não há decisão, a frase diz **quem decide** e **o que falta**, não “achamos”.

## 2. Tamanho de frase e de página

Não há limite numérico aprovado; o critério é o da pergunta:

- **Uma página, uma pergunta.** Se a resposta exige uma segunda pergunta do cabeçalho, são duas
  páginas.
- **Um parágrafo, uma ideia.** Parágrafo que mistura regra, exemplo e exceção esconde a regra.
- **Frase que precisa de segunda leitura está longa.** Corte a oração subordinada ou divida em duas.
- **Página acima de ~150 linhas é sinal de divisão**, não gate: verifique se o assunto ainda cabe em
  uma pergunta; se não, abra a página irmã e linke.

O tamanho não substitui o conteúdo: página curta que não responde à pergunta reprova; página longa
que responde a duas também.

## 3. As 11 regras editoriais

| # | Regra | Como aplicar | Reprova quando | Regra em |
|---:|---|---|---|---|
| 1 | uma informação, um dono | o fato mora em um arquivo; os outros linkam | duas páginas podem divergir sobre o mesmo número | `wyd-db-1.0.0.md` §3, item 1 |
| 2 | número com fonte e data | toda contagem cita o comando/arquivo e a data | número solto ou sem data | §3, item 2 |
| 3 | schema não se escreve duas vezes | coluna, tipo e constraint ficam no DDL e na página da migration | página de domínio copia o DDL | §3, item 3 |
| 4 | evidência em toda afirmação | `V`, `H` ou `P` colado na afirmação | presente do indicativo sem `V` | §3, item 4 |
| 5 | hipótese não vira requisito | campo desconhecido fica `H` com o ID que o fecha | hipótese tratada como regra | §3, item 5 |
| 6 | português do Brasil e Mermaid | identificador em `snake_case`; diagrama em cerca `mermaid` | diagrama em bloco `text` ou que não renderiza | §3, item 6 |
| 7 | sem espelho do código | página por responsabilidade, com contrato e invariantes | um arquivo `.rs` por página sem contrato | §3, item 7 |
| 8 | sem conteúdo de enfeite | nada de “Introdução” que repete o título, TOC manual, `TODO`, “em construção” | placeholder ou adjetivo sem medida | §3, item 8 |
| 9 | nada executável na árvore | script, `.sql`, `.rs` e teste nascem fora de `planejamento-1.0.0/` | arquivo executável dentro da árvore | §3, item 9 |
| 10 | segredo nunca aparece | DSN, token e hash pelo tipo e lugar | valor, mesmo em exemplo ou log | §3, item 10 |
| 11 | legado é contexto, não fonte | toda frase do legado é reverificada antes de entrar | cópia ou paráfrase do `deprecated/` | §3, item 11 |

## 4. Termos preferidos × evitados

| Preferido | Evite | Por quê | Fonte |
|---|---|---|---|
| **pessoa** | “usuário” quando se fala de `portal.person` | pessoa é quem autentica; conta de jogo é outra entidade | `glossario.md` |
| **conta de jogo** | “conta” quando há pessoa na frase | “conta” sozinho não diz se é pessoa, conta ou sessão | `glossario.md` |
| **série** | “as migrations” para o conjunto | a série é o conjunto ordenado; migration é o arquivo | `plano-migrations.md` §2 |
| **lacuna** | “pendência” | lacuna tem dono, estado e prazo em `lacunas.md`; “pendência” não | `lacunas.md` §0 |
| **seed** | “dados iniciais” para o arquivo | seed é arquivo da série, com `content_version`; o dado é o conteúdo | `glossario.md` |

Termo de domínio que não está aqui entra no [`glossário`](../glossario.md), não nesta tabela: o
glossário é o dono do significado; esta página só decide a preferência de escrita.

## 5. Cinco pares: errado × certo

| Errado | Certo | Por quê |
|---|---|---|
| “O banco é robusto e rápido.” | “`permission denied` para portal escrevendo em `game` (`plano-migrations.md` §7).” | adjetivo sem medida não prova nada; a frase certa nomeia o efeito e a fonte |
| “A série tem 109 tabelas.” | “34 arquivos e 109 tabelas (`plano-migrations.md` §3, 2026-11-22, `P`).” | número sem fonte e sem data é hipótese, não fato |
| “Basta rodar a migration.” | “A ordem é `provision render` → `provision apply` → `0001`…`0034` (`plano-migrations.md` §1).” | “basta” esconde pré-requisito; a frase certa dá a ordem |
| “A série nova já roda.” | “A troca da série está decidida e a execução pendente (`plano-migrations.md` §6).” | presente do indicativo sem `V` afirma o que não aconteceu |
| “Veja `[spec](../src/migrate/spec.md)`.” | “`src/migrate/spec.md` §2 (planejamento).” | link relativo para fora de `docs/` quebra no site; caminho em `code` não é link |

## 6. Checklist antes de publicar

- [ ] O cabeçalho tem as cinco linhas e o estado com data.
- [ ] Cada afirmação de comportamento tem `V`, `H` (com ID) ou `P`.
- [ ] Todo número tem fonte e data.
- [ ] Todo link relativo resolve; link para fora de `docs/` é caminho em `code` com “planejamento”.
- [ ] Não há `TODO`, “em construção”, seção vazia, TOC manual ou adjetivo sem medida.
- [ ] O fato tem um dono; a página aponta, não redigita.
- [ ] Alguém que não escreveu leu a página.

## 7. O que esta página não decide

- **O significado das marcas e dos estados:** é de [`convencoes.md`](../convencoes.md).
- **O molde de cada tipo de página:** é de `wyd-db-1.0.0.md` §6.
- **O gate de link, build e contagem:** é de
  [`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md) e do `wyd-docs`.
- **A decisão de publicar:** é de arquitetura e documentação (`AGENTS.md` §6).
