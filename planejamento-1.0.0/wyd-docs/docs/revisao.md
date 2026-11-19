> Pergunta: "o que o revisor confere, o que bloqueia e como o veredito fica registrado?"
> Público: Revisor de documentação e autor que vai receber a revisão
> Dono: equipe do wyd-docs
> Base: `wyd-docs-1.0.0.md` §9; `wyd-ops/proposta-gestao-github-e-ambientes.md` §3.7; `lacunas.md` §1
> Revisado em: 2026-11-19

**Estado:** aceito (2026-11-19)

# wyd-docs — critério de revisão

**Norma aceita pelo dono de `documentação` em 2026-11-19 (`V`; fase 3 em `wyd-docs-1.0.0.md`
§9).** Este é o critério de revisão do site; o critério de cada sistema dono continua valendo
para o conteúdo dele — por exemplo `planejamento-1.0.0/wyd-db/docs/convencoes.md` e `planejamento-1.0.0/wyd-protocol/docs/convencoes.md`.

> [!NOTE] Por que esta página existe
> A regra atual termina em "página só é `aceito` depois de revisão de alguém que não escreveu"
> (`wyd-docs/docs/contribuir/index.md`). Ela diz **quem**, não **o quê**: sem critério, duas revisões
> da mesma página dão vereditos diferentes e o autor não sabe o que corrigir.

## 1. Quem revisa

| Regra | Valor | Fonte |
|---|---|---|
| Autor não aprova o próprio PR | obrigatório | `wyd-ops` §3.7 |
| Quem revisa | alguém que não escreveu a página, com acesso ao sistema dono | `wyd-ops` §3.7 |
| Quantos | uma leitura | `wyd-ops` §3.7 |
| Aprovação exigida pelo ruleset | **nenhuma** (`required_approving_review_count: 0`) | ruleset `develop — PR obrigatório`, medido em 2026-11-19 |
| Efeito | página fica `proposto` até a segunda leitura | §4, adiante |

O ruleset não exige aprovação porque não há segundo revisor fixo; a decisão está registrada em
`wyd-ops` §3.7 ("não habilite require approvals sem outro revisor disponível: travaria o próprio
dono"). Consequência prática: a revisão é norma de processo, verificável no PR, não portão técnico.

## 2. O que o revisor confere

Quatro blocos, nesta ordem. Cada achado é registrado com página, trecho e a fonte correta — o revisor
aponta, não reescreve: reescrita apaga a autoria e esconde o defeito de quem escreveu.

### 2.1 Forma

A conferência de forma é a lista de reprovação de `convencoes.md` §7, item a item: é a única lista, e
o revisor não mantém segunda cópia. Achado de forma vira **bloqueante** ou **devolve a rascunho**
conforme §3.

### 2.2 Evidência

O que cada marca exige é de `convencoes.md` §5, e número com fonte e data, sem cópia do dono, é de
§5.1. O que é só da revisão: nenhuma conclusão se apoia em `deprecated/` como se fosse fato
(`AGENTS.md` §3).

### 2.3 Dono e escopo

- [ ] o assunto pertence ao sistema da página, não a outro (`AGENTS.md` §6);
- [ ] regra de outro dono é citada por seção, não reescrita — inclusive contagem, que o §5.1 de
  `convencoes.md` já proíbe copiar;
- [ ] a página está dentro do teto do plano do sistema.

### 2.4 Entrega

- [ ] a lacuna correspondente foi atualizada no mesmo PR, se a página fechou decisão;
- [ ] o plano do sistema aponta a página, se ela é nova;
- [ ] o gate foi rodado e a saída está no PR;
- [ ] o PR tem um assunto só.

## 3. Três vereditos

| Veredito | Quando | Efeito |
|---|---|---|
| **bloqueante** | link quebrado, segredo ou dado de jogador, placeholder, sintaxe antiga em página nova, página sem H1 | não entra: corrigir antes do merge |
| **devolve a rascunho** | falta cabeçalho, `Pergunta` não respondida, afirmação sem marca, `H` sem ID, número sem fonte | página volta a `proposto` e o autor corrige |
| **pendência de fechamento** | segunda leitura não registrada, índice da pasta ausente, página não contabilizada | entra, mas não vira `aceito` até fechar |

A ordem importa: um bloqueante não é negociado por estar "quase pronto", e uma pendência de
fechamento não impede o merge — impede o estado `aceito`.

## 4. Como o veredito fica registrado

1. O revisor comenta no PR com o veredito e a lista de achados (página, trecho, fonte correta).
2. O autor corrige no mesmo PR; achado que ele não aceita é respondido com a fonte que o sustenta.
3. Com os bloqueantes resolvidos, o autor muda a linha de estado para `**Estado:** aceito (AAAA-MM-DD)`
   no mesmo PR — a data é a da segunda leitura.
4. O merge não muda o estado: `aceito` é ato de revisão, não consequência de squash.

Página `aceito` que muda de conteúdo relevante volta a `proposto` e pede nova leitura. Correção de
typo, link ou data de revisão não reabre o estado.

## 5. O que a revisão não é

- Não é revisão literária: estilo é do autor, desde que a afirmação seja verificável.
- Não substitui o gate: `verificar.py` verde é pré-requisito, não veredito.
- Não decide lacuna: revisor aponta; quem decide é o dono nomeado em `lacunas.md` — inclusive
  publicação de sistema novo e página `proposto` no ar (`LPDOC-08`, `LPDOC-03`).
- Não cria ADR nem fecha decisão estrutural.

## 6. O que esta página não decide

- Não decide a forma da página: é de `convencoes.md`.
- Não decide o processo de autoria: é de `guia-autor/index.md`.
- Não muda o ruleset nem exige aprovação obrigatória: isso é de `wyd-ops` §3.7 e do dono do
  repositório.
- Não decide conteúdo de domínio: só como uma página é revisada.
