# Gate de aceite de uma página de planejamento

> Pergunta: "o que precisa ser verdade, e como se prova, para uma página de planejamento passar de `proposto` a `aceito`?"
> Público: Autor, revisor, dono de sistema e agente que escreve página de planejamento
> Dono: equipe do `wyd-docs` (gate editorial); a aprovação de cada página é do dono do sistema dela
> Base: `AGENTS.md` §5 e §7; `wyd-docs/docs/convencoes.md` §3 e §7; `wyd-docs/docs/revisao.md` §1–§4; `wyd-db-1.0.0.md` §10; `wyd-protocol-1.0.0.md` §8; `wyd-db/docs/convencoes.md` §Estados; `wyd-db/docs/planos/plano-de-qualidade-e-gates.md` §4; `wyd-1.0.0.md` §7.7
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). Não cria lista nova de reprovação: projeta as listas existentes em
comandos e mede o resultado. Não aceita página nenhuma — `aceito` é ato do dono do sistema, com data.

## 1. A regra em quatro condições

Uma página passa de `proposto` para `aceito` quando **as quatro** valem na mesma entrega:

| # | Condição | Quem garante | Fonte |
|---|---|---|---|
| 1 | **zero falha mecânica** nos itens de §2 | qualquer pessoa ou o CI; a saída do comando vai na entrega | esta página |
| 2 | **zero falha de julgamento** nos itens de §3, assinada por quem conferiu | revisor que não escreveu a página | `wyd-docs/docs/revisao.md` §2 |
| 3 | **segunda leitura registrada** com nome e data, no PR | revisor | `revisao.md` §1 e §4 |
| 4 | **"sim" do dono do sistema**, com data, e a linha de estado atualizada na mesma entrega | dono nomeado em `lacunas.md` | `AGENTS.md` §5.3 e §7 |

Três consequências que costumam ser esquecidas:

- **O agente não dá `aceito`.** Decisão sem "sim" do dono é `proposta`, não `aceito` (`AGENTS.md` §5.3);
  escrever `**Estado:** aceito` sem a condição 4 é falsificar estado.
- **Merge não aceita.** `aceito` é ato de revisão, não consequência de squash (`revisao.md` §4.4).
- **`aceito` não é permanente.** Mudança de conteúdo relevante devolve a página a `proposto`; correção de
  typo, link ou data de revisão não reabre (`revisao.md` §4, último parágrafo).

## 2. O que a máquina prova

Os comandos abaixo rodam da raiz do repositório e são a projeção dos itens já normatizados. São
**comandos de terminal**, não script versionado: `planejamento-1.0.0/` não recebe executável (`AGENTS.md`
§5.6).

| Item | Comando | Passa quando | Dono do item |
|---|---|---|---|
| cabeçalho de 5 linhas, na ordem | ver bloco abaixo | `5/5` | `wyd-docs/docs/convencoes.md` §3; `wyd-db-1.0.0.md` §10.1 |
| linha de estado com data | `grep -c '^\*\*Estado:\*\*' arquivo` | `1`, com `(AAAA-MM-DD)` | `convencoes.md` §Estados (`proposto`/`aceito`/`substituído`/`retirado`) |
| placeholder | `grep -nE '\bTODO\b\|\bTBD\b\|em constru\|a preencher\|XXX'` | `0` linhas | `wyd-db-1.0.0.md` §10.8; `convencoes.md` §7 |
| link, âncora, título, navegação e sintaxe 1.x | `docker compose --profile gate run --rm gate` | `exit 0` | `wyd-docs/docs/convencoes.md` §7; `verificar.py` |
| adjetivo sem medida | `grep -nE '\b(rápido\|robusto\|escalável)\b'` | `0` linhas | `convencoes.md` §7 |

O probe do cabeçalho, copiável:

```bash
for f in <arquivos>; do
  h=0
  for c in 'Pergunta' 'Público' 'Dono' 'Base' 'Revisado em'; do
    head -6 "$f" | grep -q "^> $c:" && h=$((h+1))
  done
  echo "$h/5 $f"
done
```

**O que o comando não pode fazer.** O `grep` de placeholder é sensível a maiúsculas de propósito: com
`-i`, `TODO` casa com `todo`/`todos` do português e produz falso positivo em quase toda página que fala
"todos os arquivos". Um gate que reprova por engano ensina o time a ignorá-lo — a primeira execução
deste probe, em 2026-11-23, deu `TODO` em seis páginas do `wyd-db` e todas eram a palavra "todo(s)"
(`V`, saída do comando). O comando corrigido deu zero.

**Citar o placeholder não é usá-lo.** O probe normativo roda fora de cerca e de `code span`, como
`_sem_code_span` já faz no gate de sintaxe (`wyd-docs/verificar.py`): esta própria página contém `TODO` e
`TBD` entre crases, como exemplo, e o comando bruto as contaria. Para o veredito, descarte a linha cujo
único acerto está entre crases; a regra existe justamente para que a documentação possa citar o que proíbe.

**O que nenhum comando prova, e não está nesta tabela de propósito:** fonte de número, coerência de
marca `V`/`H`/`P`, se a página responde à `Pergunta`, se reescreveu regra alheia e se nenhum segredo
aparece em exemplo. São os itens de §3 — chamá-los de "automáticos" seria mentira útil.

## 3. O que exige julgamento — e quem assina

| Item | O que se confere | Quem assina |
|---|---|---|
| `Pergunta` respondida | o corpo responde à pergunta do cabeçalho e não a outra | revisor |
| evidência | toda afirmação de comportamento tem `V`, `H` ou `P`, e `H` tem o ID que a reabre (`AGENTS.md` §5.4) | revisor |
| número com fonte e data | o formato de `convencoes.md` §5.1, sem copiar contagem do dono | revisor |
| um fato, um dono | regra de outro sistema citada por seção, não reescrita (`AGENTS.md` §5.5) | revisor |
| molde do tipo | as seções obrigatórias do plano do sistema estão presentes e preenchidas (`wyd-db-1.0.0.md` §6; `wyd-protocol-1.0.0.md` §6) | dono do sistema |
| legado como fonte | nenhuma conclusão se apoia em `deprecated/` como fato (`AGENTS.md` §3) | revisor |
| segredo e dado de jogador | nem no corpo, nem em exemplo, nem em log citado (`AGENTS.md` §5.7) | revisor |
| dentro do teto | a página pertence à árvore planejada e à contagem do sistema | dono do sistema |

## 4. As listas existentes e quem vence

Esta página não substitui nenhuma delas. O gate do projeto é a **união**; em divergência, vence o dono do
sistema da página.

| Lista | Itens | Escopo |
|---|---:|---|
| `wyd-docs/docs/convencoes.md` §7 | 12 | forma de página **publicada** |
| `wyd-docs/docs/revisao.md` §2–§3 | 4 blocos, 3 vereditos | processo de revisão e registro do veredito |
| `wyd-db-1.0.0.md` §10 | 11 | página do `wyd-db`, com severidade: 1–4 devolvem a rascunho, 5–8 bloqueiam, 9–11 são pendência de fechamento |
| `wyd-protocol-1.0.0.md` §8 | 8 | página do `wyd-protocol` |
| `wyd-db/docs/convencoes.md` §Estados e §Números | — | significado dos estados e formato de número |

**Divergência já medida, que o time precisa resolver.** `wyd-db-1.0.0.md` §10, item 10, exige que a página
esteja "contabilizada em `planos/metricas.md`" — e `metricas.md` não existe: `docs/planos/index.md` (2026-11-22)
o lista como **não escrito**, e `M20` decidiu que ele só pode ser escrito depois da série existir
(`V`, `wyd-db/lacunas.md` §2.3). Enquanto isso valer, **nenhuma página de `wyd-db` satisfaz §10 por
completo**, mesmo perfeita nos outros dez itens. Ou o item 10 ganha uma exceção declarada, ou o estado
`aceito` fica indisponível para o sistema — e as duas são decisão do dono, não desta página.

A divergência e a questão gêmea do estado de página de `migrations/` (§5.2 mede; §5.3, item 2, formula a decisão) estão registradas juntas em
`M33` (`wyd-db/lacunas.md` §2.3, proposta desde 2026-11-23). O registro é o dono da decisão; esta página é a
medição que a motivou.

## 5. Aplicação à onda A (2026-11-23)

**O que é a onda A.** A onda **A — fio e dado** de `wyd-1.0.0.md` §7.7: `protocol` `0.1.0` (quadro e DTO,
sem opcode de payload) e `db` com `SCHEMA_VERSION = 34`; gate de saída: round-trip verde e migrations que
aplicam do zero e revertem.

**O conjunto de páginas que a onda A exige**, e não outro:

| Grupo | Páginas | Existem | Ausentes |
|---|---:|---:|---:|
| `wyd-protocol/docs/wire/` — `index`, `header`, `enquadramento`, `keyword-e-checksum`, `limites-e-faixas`, `flags-de-direcao` (onda 2 de `wyd-protocol-1.0.0.md` §9) | 6 | 0 | 6 |
| `wyd-protocol/docs/goldens/como-capturar.md` — pré-requisito da captura que fecha `LPRT-03` | 1 | 1 | 0 |
| `wyd-db/migrations/` — `index.md`, `serie-e-regras.md` e as 34 páginas de arquivo | 36 | 28 | 8 |
| **Total** | **43** | **29** | **14** |

As 8 ausentes do `wyd-db` são 2 páginas de seção e as 6 de seed (`0016`, `0017`, `0018`, `0020`, `0023`,
`0027`), `bloqueada` por dado de origem (`migrations-checklist.md` §4, `V`). As 6 de `wire/` não têm
arquivo: `wyd-protocol/docs/` tem só `index.md`, `convencoes.md`, `goldens/` e `roadmap/` (`V`, `ls`,
2026-11-23).

### 5.1 Veredito

| Grupo | Veredito | Item que decide |
|---|---|---|
| `wire/` (6) | **reprova por ausência** | não há página para aceitar; escrever depende de `LPRT-03` e do golden do fio |
| `goldens/como-capturar.md` (1) | **não muda**: pendência de fechamento | forma ok e estado com data; falta segunda leitura e o "sim" do dono (`revisao.md` §1, §4) |
| `migrations/` escritas (28) | **27 reprovam**; 1 já era `aceito` | falta a linha de estado na página: pela regra de `convencoes.md` §Estados, "página sem essa linha é rascunho"; `0002-ops-audit.md` é `aceito` desde 2026-11-20 |
| `migrations/` seeds ausentes (6) | **reprova por ausência**, com bloqueio declarado | dado de origem não conferido (`migrations-checklist.md` §4) |
| `migrations/index.md` e `serie-e-regras.md` (2) | **reprova por ausência** | seção não escrita |

**Nenhuma página muda para `aceito` nesta aplicação.** Uma já estava aceita, uma depende de revisão, 27
não podem ser aceitas na forma atual e 14 não existem. "Não muda" não é juízo de valor sobre o conteúdo:
é o retrato do gate.

**Por que isso não é só dívida de documentação.** `AGENTS.md` §5.1 diz que nenhuma migration nasce antes de
a página que a descreve estar `aceita`. Lido ao pé da letra: das 34 páginas de migration, **1 está aceita**
(`0002`), **27 estão em rascunho** por falta da linha de estado e **6 não existem** — então a onda A trava
no processo de aceite antes de travar na captura. O bloqueio de captura (`LPRT-03`) é real e conhecido;
este é o que não estava visível.

### 5.2 O que foi medido

| Medida | Resultado | Comando |
|---|---|---|
| cabeçalho de 5 linhas | **29 de 29** com `5/5` | probe de §2 |
| linha `**Estado:**` | **2 de 29** — só `0002-ops-audit.md` e `goldens/como-capturar.md` | `grep -c '^\*\*Estado:\*\*'` |
| placeholder real | **0** nas 29 | `grep -nE '\bTODO\b\|\bTBD\b\|em constru\|a preencher\|XXX'` |
| sintaxe 1.x (`!!!`) | **28 de 29** | `grep -cE '^\s*!!!'` |
| gate do site | **exit 0**, 269 páginas, 267 fontes | `docker compose --profile gate run --rm gate` |
| `metricas.md` | não existe | `docs/planos/index.md` (2026-11-22) |

Sobre o `!!!`: as 28 páginas de migration usam admonição do MkDocs 1.x, e o gate do site **não as vê**
porque `_checar_sintaxe` percorre só `<sistema>/docs/**` e `migrations/` não é publicado
(`V`, `wyd-docs/verificar.py`; `wyd-db-1.0.0.md` §5.1). Hoje não é defeito de renderização; vira defeito no
dia em que a página for promovida a `docs/`, porque lá a regra de `convencoes.md` §2–§7 se aplica e a 2.0
não renderiza `!!!`.

### 5.3 O caminho mínimo até a primeira aceitação da onda A

| # | Ação | Dono | Por que vem antes |
|---|---|---|---|
| 1 | registrar segunda leitura de `goldens/como-capturar.md` e pedir o "sim" | revisor + dono de `wyd-protocol` | é a única página do conjunto que cumpre a forma e ainda não foi aceita |
| 2 | decidir o que é o estado de uma página de migration: linha `**Estado:**` ou o estado do checklist | dono de `wyd-db` | sem isso, 27 páginas ficam em rascunho por regra, não por conteúdo |
| 3 | aplicar a decisão de 2 às 27 páginas, com data | dono de `wyd-db` | é forma, não aceite: `proposto` com data é a leitura honesta do estado atual |
| 4 | declarar a exceção de `metricas.md` ou criar o arquivo | dono de `wyd-db` | sem isso, o item 10 de §10 bloqueia qualquer aceite do sistema |
| 5 | escrever as 6 páginas de `wire/` e aceitá-las com os goldens | `wyd-protocol` | a onda A não fecha sem o fio provado por captura |
| 6 | escrever as 8 páginas ausentes de `migrations/` | `wyd-db` | 6 esperam dado de origem; 2 são seção |

A ordem importa: os passos 1–4 são de processo e cabem numa sessão; 5 depende de captura, que depende de
endpoint; 6 depende de dado de origem conferido.

## 6. O que esta proposta não decide

- Não aceita página, não atribui data de aceite e não muda a linha de estado de nenhuma página.
- Não reescreve as listas de §4: em conflito, vale a lista do dono do sistema.
- Não decide se página de `migration` tem linha de estado própria ou se o estado do checklist basta — é de
  `wyd-db` (`convencoes.md` §Estados × `migrations-checklist.md` §1.2).
- Não cria `planos/metricas.md` nem decide a exceção do item 10 de `wyd-db-1.0.0.md` §10.
- Não decide severidade de defeito: 1–4 devolvem a rascunho, 5–8 bloqueiam e 9–11 são pendência, como em
  `wyd-db-1.0.0.md` §10 e `revisao.md` §3.
- Não muda o gate do site, o `verificar.py` nem o Compose.
- Não define o que é a onda A: isso é de `wyd-1.0.0.md` §7.7.
- Não unifica o vocabulário de estado dos arquivos da raiz: os `proposta-*.md` escrevem `proposta` e esta
  página escreve `proposto`, o termo de `convencoes.md` §Estados. A unificação é decisão do dono do
  `wyd-docs`.

## 7. Como re-medir

```bash
cd planejamento-1.0.0
for f in wyd-protocol/docs/wire/*.md wyd-protocol/docs/goldens/como-capturar.md \
         wyd-db/migrations/*.md; do
  [ -f "$f" ] || continue
  h=0
  for c in 'Pergunta' 'Público' 'Dono' 'Base' 'Revisado em'; do
    head -6 "$f" | grep -q "^> $c:" && h=$((h+1))
  done
  e=$(grep -c '^\*\*Estado:\*\*' "$f")
  p=$(grep -cE '\bTODO\b|\bTBD\b|em constru|a preencher|XXX' "$f")
  s=$(grep -cE '^\s*!!!' "$f")
  printf '%-34s cabeçalho=%s/5 estado=%s placeholder=%s sintaxe1x=%s\n' \
    "$(basename "$f")" "$h" "$e" "$p" "$s"
done
```

O laço acima é o **probe bruto**: ele conta acerto em `code span`, então uma página que cita o placeholder
como exemplo aparece suja. Para o veredito, a leitura é a de §2 — fora de cerca e de `code span`, como
`_sem_code_span` faz em `verificar.py`. O resultado de 2026-11-23 está em §5.2, e foi lido assim. Quem
re-medir depois precisa atualizar §5 com a data nova — número de gate sem data envelhece como qualquer
outro (`convencoes.md` §5.1).
