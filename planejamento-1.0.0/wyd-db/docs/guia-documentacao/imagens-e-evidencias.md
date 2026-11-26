> Pergunta: "quando uma imagem é necessária e como ela prova algo sem vazar dado?"
> Público: Quem escreve ou revisa página do `wyd-db`
> Dono: equipe do `wyd-db`
> Base: `docs/convencoes.md` §Recursos do site; `wyd-docs/preparar.py`; `AGENTS.md` §5, item 7 e §7; `wyd-db-1.0.0.md` §6.14; `lacunas.md` `M32`
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-24)

Captura de tela **não é prova**: a prova de uma página desta árvore é comando e saída, ou arquivo e linha
(`docs/convencoes.md` §Marcações de evidência). Imagem entra quando o objeto é visual e o texto não o
substitui — e mesmo aí ela acompanha a prova, nunca a substitui. Esta página é a dona da regra de imagem;
o contrato de leitura continua em [`convencoes.md`](../convencoes.md).

## 1. Estado medido: hoje não existe imagem publicada

| Fato | Medição | Fonte |
|---|---|---|
| imagens nas páginas | **0** ocorrências de `![` em `docs/` | `grep -rn '!\[' docs --include='*.md'`, 2026-11-24 |
| o que a montagem copia | só `*.md` (`origem.rglob("*.md")`) e os arquivos do tema (`estilo.css`, `site.js`, `mermaid.js`) | `wyd-docs/preparar.py`, linhas 96 e 215 |
| verificação de link | não trata `src` de imagem | `wyd-docs/gate/checks/links.py` |

**Consequência:** um `.png` colocado em `docs/` **não chega ao site** — a montagem publica página, não
ativo de página. Publicar imagem exige mudança no `preparar.py`, que é do **`wyd-docs`**, e essa mudança
não existe. Enquanto ela não existir, imagem não entra em `docs/`: entra como caminho em `code` com a
marca “planejamento”, igual a link para `migrations/` ou `src/` (`convencoes.md` §Links e citações).

## 2. Quando a imagem é necessária

| Situação | Imagem ajuda? | O que usar em vez dela |
|---|---|---|
| provar que uma migration aplicou | não | `psql -c 'select version, slug from public.schema_migrations'` com a saída |
| mostrar o efeito de um parâmetro | não | tabela parâmetro → efeito (`configuracao.md`) |
| explicar fluxo, estado ou relação entre tabelas | não | Mermaid (`convencoes.md` §Diagramas) |
| mostrar uma tela do `wyd-admin` ou do portal | sim, como apoio | a rota e o contrato C4 que a página cita |
| mostrar um erro que só aparece renderizado | sim, como apoio | o texto do erro e o comando que o produziu |

Regra: **1 imagem por ideia**, ligada a um passo que o texto já descreve. Página que depende da imagem
para ser entendida reprova na revisão — quem lê no terminal ou no leitor de tela não recebe nada.

## 3. Regras para quando a publicação existir

Estas regras valem a partir da mudança de `wyd-docs`; hoje nenhuma imagem é publicada (§1).

| Regra | Critério |
|---|---|
| nome | `kebab-case`, sem acento, dizendo o que a imagem prova (`erro-42501-ao-inserir.png`), nunca `print1.png` — os dois são **exemplos de nome**: não existe imagem com esse nome versionada hoje |
| pasta | junto da página que a usa; nada de imagem em `gerado/`, que é índice auxiliar |
| texto alternativo | obrigatório, e descreve **o que a imagem prova**, não “captura de tela” |
| formato | PNG para tela (texto nítido), WebP quando for foto; sem upscale e sem marca d'água |
| peso | alvo de **até 200 KB** por imagem (`P`, proposto nesta página); acima disso, recortar a região que importa |
| dado | só dado sintético declarado (`M32`, `Q-DP-7 = A`); a captura de ambiente real não entra no repositório |

## 4. O que nunca aparece na imagem

- **Segredo:** DSN com senha, token, PIN, hash, chave de API, `Authorization` — `AGENTS.md` §5, item 7.
- **Dado de jogador:** login, e-mail, nome de personagem, IP, histórico de sessão, saldo nominal.
- **Caminho de máquina:** usuário do sistema, caminho com nome pessoal, hostname interno.
- **Tela de produção com tráfego real:** mesmo “sem nada sensível”, o estado real não vira ilustração.

Antes de commitar, ampliar os cantos da captura e conferir barra de título, terminal e notificação do
sistema — é onde o vazamento aparece. Se a imagem precisa de tarja, a tarja é opaca e não deixa o texto
legível por trás.

## 5. Checklist de revisão

- [ ] A imagem é necessária, ou o comando/tabela/Mermaid já resolve?
- [ ] O texto alternativo diz o que ela prova?
- [ ] Nome em `kebab-case` e pasta junto da página?
- [ ] Formato e peso dentro do alvo?
- [ ] Nenhum segredo, dado de jogador ou caminho de máquina visível?
- [ ] O dado é sintético e está declarado como tal?

## 6. O que esta página não decide

- **Se a montagem vai copiar imagem:** é do `wyd-docs` (`preparar.py`), e hoje não copia (§1).
- **A regra de diagrama:** é de `convencoes.md` §Diagramas; esta página não repete tipo nem sintaxe.
- **A política de dado pessoal:** é de `proposta-dados-pessoais-e-retencao.md` e `M32`; aqui só se aplica o
  que já está decidido.
- **O aceite da página:** é de `planos/plano-de-qualidade-e-gates.md` e de `docs/convencoes.md` §O que
  reprova uma página.
