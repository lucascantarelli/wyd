> Pergunta: "qual é o próximo passo do site, em ordem de desbloqueio ÷ custo?"
> Público: quem mantém o `wyd-docs`
> Dono: equipe do wyd-docs
> Base: `lacunas.md`; `wyd-docs-1.0.0.md` §9
> Revisado em: 2026-11-19

**Estado:** proposto (2026-11-19)

# wyd-docs — próximos passos

Esta é a **fonte única da fila** do site. O mapa
[roadmap-fase-3-para-4.md](roadmap-fase-3-para-4.md) guarda o critério de saída da fase 4 e aponta
para cá sem repetir esta lista; a ordem de produção é de `wyd-docs-1.0.0.md` §9.

O estado de cada lacuna — com dono, marco e evidência — é de `lacunas.md` §1, que esta página não
reconta.

A coluna **Do dono** diz o que falta de outra pessoa; `—` é trabalho que sai sem perguntar a ninguém.
Nada aqui é commitado, enviado ou mergeado sem a palavra do dono do repositório.

## 1. Fila, ordenada por desbloqueio ÷ custo

| # | O que se faz | Arquivos | Tempo | Desbloqueia imediatamente | Do dono |
|---:|---|---|---|---|---|
| 1 | **Entregar o pacote local**: commit, push e atualização do PR | tudo que `git status` mostra como modificado ou não rastreado — inclui as 3 normas, esta fila, o mapa e o plano de CI | ~10 min | tira do disco o aceite da fase 3, o censo e as propostas de CI | **autorização de commit/push**; o merge do PR #47 é decisão separada |
| 2 | **Passo 4 — `contribuir` aponta para as normas**: as seções de cabeçalho, sintaxe e revisão viraram referência | `wyd-docs/docs/contribuir/index.md` (1.932 bytes, 3 seções) | **cumprido em 2026-11-19** | uma boca só para forma, processo e veredito | — |
| 3 | **`LPDOC-04` respondida**: sem domínio próprio na `1.0.0`; publica em `lucascantarelli.github.io/wyd` | `lacunas.md` (linha fechada); `wyd-docs-1.0.0.md` §9 e §10 | **cumprido em 2026-11-19** | o contrato de URL do redirect existe | — |
| 4 | **Responder `LPDOC-06`** (renomear página exige redirect?) | `lacunas.md`; se "sim", `wyd-docs/verificar.py` e a fase 4 | 1 frase + 1 sessão, se aprovado | abre a implementação do redirect; o contrato de URL já existe | decisão de `arquitetura` |
| 5 | **Responder `LPDOC-13`** (busca de texto completo?) | `lacunas.md` | 1 frase | fecha o segundo item da fase 4 | decisão de `arquitetura + produto` |
| 6 | **Responder `LPDOC-08`** (quem aprova sistema novo no site?) | `lacunas.md` | 1 frase | publicar as páginas do `wyd-protocol` deixa de ser caso a caso | decisão de `documentação + arquitetura` |
| 7 | **Converter `!!!` nas três origens** para `> [!TIPO]` | `docs/index.md:14`, `wyd-protocol/docs/convencoes.md:13`, `wyd-protocol/docs/index.md:13` | **cumprido em 2026-11-19** | essas três páginas deixam de depender da ponte `compat.py` | `LPDOC-14` segue `proposta`: a ponte ainda serve o `wyd-db`; removê-la é decisão de `arquitetura + documentação` |

Itens 4 a 6 são independentes entre si.

## 2. Ganhos rápidos (menos de 10 min)

| O que | Onde | Por que agora |
|---|---|---|
| Rodar o gate antes de cada commit do pacote | `wyd-docs/` (comando do §7.1) | evita commitar página quebrada e refazer o PR |

## 3. Recontar o censo (um comando)

```bash
for f in $(find wyd-docs/.build/docs -name '*.md'); do
  if estado=$(grep -m1 -oE '^\*\*Estado:\*\* *[^ ]+' "$f"); then
    printf '%s\n' "$estado" | sed -E 's/^\*\*Estado:\*\* *([^ ]+).*/\1/'
  else
    echo SEM
  fi
done | sort | uniq -c
```

O resultado não se copia: total de páginas e censo por estado são de `wyd-docs-1.0.0.md` §7.2, o
único dono do número; `convencoes.md`, `AGENTS.md` e o site citam aquela seção. Este comando
reproduz o censo a qualquer momento, sem guardar cópia do resultado.

## 4. Não fazer agora

- **`LPDOC-09` (91 páginas geradas do `wyd-db`)**: aprovar só autoriza; o gerador não existe e o
  volume transformaria o site de 33 em 124 páginas — é projeto, não passo.
- **`LPDOC-02` (publicar `AGENTS.md` e o panorama)**: o MkDocs 2.0 não tem include nem plugin; exigiria
  cópia mantida à mão. Custo alto, ganho de leitor baixo.
- **`LPDOC-05`, `LPDOC-07`, `LPDOC-11`**: versionamento, idioma e upgrade da `2.0.dev6` são `P3`, sem
  consumidor pedindo; `LPDOC-11` depende de o build continuar verde.
- **`LPDOC-10` e o catálogo de opcodes**: o `LPRT-20` espera essa resposta, mas o catálogo depende de
  `LPRT-01` (quais opcodes entram), que é decisão de outro sistema.
- **Métrica da fase 4**: só depois de `LPDOC-06` e `LPDOC-13`, porque é o que sobra da fase.

## 5. Primeira execução recomendada

**Autorizar o item 1** — sem ele, tudo o que está no disco continua invisível para o PR. Em seguida,
`LPDOC-06`: é a decisão que abre o primeiro item da fase 4.

Critério: "quantos itens ficam desbloqueados ÷ minutos", contando primeiro o que não depende de
decisão nenhuma. Hoje esse conjunto está vazio, então o gargalo é autorização, não trabalho.

## 6. O que esta página não decide

- Não decide lacuna: `lacunas.md` continua dono de estado, dono e evidência.
- Não decide fase nem critério de saída: isso é do
  [roadmap-fase-3-para-4.md](roadmap-fase-3-para-4.md).
- Não autoriza commit, push, merge nem mudança de ruleset.
