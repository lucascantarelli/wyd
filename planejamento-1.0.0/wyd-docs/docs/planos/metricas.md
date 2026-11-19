> Pergunta: "que número o site mede, onde o valor vive e com que comando ele se reproduz?"
> Público: quem mantém o site e a documentação
> Dono: equipe do wyd-docs
> Base: `wyd-docs-1.0.0.md` §7.1–§7.2; `proximos-passos.md` §3
> Revisado em: 2026-11-19

**Estado:** aceito (2026-11-19)

# wyd-docs — métrica do site

**Aceita pelo dono de `documentação` em 2026-11-19 (`V`).** Esta página define **o que** o site
mede; o **valor** de cada número continua com dono único, `wyd-docs-1.0.0.md` §7.2, e não é
copiado aqui. O gate de 2026-11-19 passou com o site montado (`V`).

## 1. O que o site mede

| Número | O que responde | Dono do valor |
|---|---|---|
| páginas montadas | quantas páginas o `preparar.py` colocou no artefato publicado | `wyd-docs-1.0.0.md` §7.2 |
| censo por estado | quantas páginas estão `proposto`, `aceito` ou sem linha de estado | `wyd-docs-1.0.0.md` §7.2 |
| problemas do gate | link, âncora, navegação, título, redirect e sintaxe MkDocs 1.x fora do esperado | `wyd-docs-1.0.0.md` §7.2 |

Os três saem do mesmo artefato: o site montado. A métrica não tem série própria — quem guarda o
número e a data é o §7.2, para que duas páginas não possam divergir sobre o mesmo total.

## 2. Com que comando

O gate de `wyd-docs-1.0.0.md` §7.1 monta, publica e verifica; ele imprime o total de páginas e
reprova com `exit 1` quando há problema. O censo por estado sai do mesmo artefato pelo bloco de
`proximos-passos.md` §3. Os dois comandos não são copiados para cá: esta página aponta o dono de
cada um, e o resultado medido entra só no §7.2, com data.

## 3. O que fica fora da métrica

- **link externo**: o `verificar.py` não resolve destino fora do artefato, então link quebrado de
  fora do site não entra no número;
- **qualidade do texto**: forma e veredito são de `convencoes.md` e `revisao.md`, não de contagem;
- **tráfego e audiência**: não há analítica no site e a `1.0.0` não mede visita;
- **documentação do projeto**: página de sistema, ADR e panorama são medidos por
  `planejamento-1.0.0/wyd-1.0.0.md` §1, não por esta página.

## 4. Quando a métrica muda

Mudou o que o `verificar.py` confere, a regra de montagem do `preparar.py` ou o conjunto de
estados do cabeçalho: esta página e o §7.2 mudam **na mesma entrega**, com data e comando. Número
sem comando que o reproduza não entra, e valor copiado de outra página é defeito.
