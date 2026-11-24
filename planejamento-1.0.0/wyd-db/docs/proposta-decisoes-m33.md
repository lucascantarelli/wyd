# Decisões da `M33` — estado de página e aceite da onda A

> Pergunta: "o que uma página do `wyd-db` precisa ter para ser `aceito`, e onde mora o estado de uma
> página de `migrations/`?"
> Público: dono do `wyd-db`, arquitetura, quem escreve página e quem revisa
> Dono: `wyd-db` — a decisão é do dono; esta página formula as perguntas e registra as respostas
> Base: `lacunas.md` §2.3 (`M20`, `M33`); `docs/convencoes.md` §Estados e §Cabeçalho de página;
> `migrations-checklist.md` §1, §1.2, §4 e §5; `wyd-db-1.0.0.md` §10 e §11;
> `proposta-gate-de-aceite-de-pagina.md` §5 e §7; `AGENTS.md` §5, itens 1, 3 e 5
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). Seis perguntas respondidas pelo dono em 2026-11-23; as opções A
foram aprovadas e estão registradas em §9. A página continua `proposto` porque a revisão de aceite
(segunda leitura e checklist de §2) não foi feita; o que mudou foi a decisão, não o estado editorial.
Nenhum número desta página é estimativa: as contagens vêm de arquivo, e a data do estado é a de
atribuição (2026-11-23), não uma medição.

## 1. Fronteira desta rodada

**Fecha:** onde mora o estado de uma página de `migrations/`; se as 27 páginas escritas recebem a
linha agora ou na revisão de cada uma; como o item 10 de §10 convive com `planos/metricas.md`
ausente; se `migrations/index.md` entra agora; se o checklist espelha o estado da página; e como a
onda A atravessa o aceite das 34 páginas.

**Não decide:** o conteúdo de qualquer página; a política da série (`plano-migrations.md` §6, 5ª
onda); número ou fórmula de `metricas.md` (`M20`); retenção (`M22`), fila (`M25`), idempotência
(`M30`), dado pessoal (`M32`); e o aceite de uma página concreta — ato do dono, página a página.

**Não reabre:** `M20` (número medido), `M22` (os três prazos), `M25` (fila), `M30` (idempotência),
`M7` (consentimento versionado), `P-22` (PII em `portal`), `A12` (fechamento anonimiza e-mail) nem
`M32` (as oito `Q-DP`).

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| páginas de migration planejadas | **34** | `plano-migrations.md` §3 |
| escritas | **28** | `migrations-checklist.md` §4 |
| sem linha `**Estado:**` | **0** (antes da aplicação: 27) | `grep -c` de `^\*\*Estado:\*\*` em `migrations/*.md` |
| com linha | **28** — 27 `proposto (2026-11-23)` + `0002-ops-audit.md`, `aceito (2026-11-20)` | idem |
| estados de página definidos | 4 (`proposto`, `aceito`, `substituído`, `retirado`); "página sem essa linha é rascunho" | `docs/convencoes.md` §Estados |
| estados usados pelo checklist | 5 (`pronta`, `aguardando aprovação`, `bloqueada`, `parcial`, `concluída`); **nenhum é `aceito`** | `migrations-checklist.md` §1 |
| itens que §10 exige para `aceito` | 11; o item 10 pede `index.md` da pasta, e a metade de `planos/metricas.md` está suspensa por `Q-M33-3` | `wyd-db-1.0.0.md` §10 |
| `planos/metricas.md` | **não escrito** | `docs/planos/index.md` |
| `migrations/index.md` | **existe** (criado em 2026-11-23); era a 5ª onda de §11 | `migrations/index.md`; `wyd-db-1.0.0.md` §11 |
| `serie-e-regras.md` | 5ª onda, fora da M33 | `wyd-db-1.0.0.md` §11 |
| páginas aceitas hoje | **1 de 28 escritas** | as duas linhas acima |
| DDL e índices de banco | **não tocados** por esta rodada | `migrations/` é documentação; nenhum `CREATE INDEX` entra |

**Não há volume, prazo ou métrica a estimar nesta rodada.** As grandezas são contagens de arquivo, e
a única data em jogo é a de atribuição do estado. `M20` proíbe número medido antes da série, e
`planos/metricas.md` é o dono desse número quando ele existir.

## 3. Já decidido — não reabrir

| Item | Decisão vigente | Fonte | Efeito na M33 |
|---|---|---|---|
| toda página carrega linha de estado | 4 estados; sem a linha, a página é rascunho | `docs/convencoes.md` §Estados | a M33 decide **se `migrations/` é exceção**, não se a regra existe |
| página não aprovada é `proposto` | sem "sim" do dono não há `aceito` | `AGENTS.md` §5, item 3 | a M33 decide **quando** aplicar a linha, não qual estado |
| `metricas.md` só depois da série | número medido, dono único `M20` | `lacunas.md` `M20` | a M33 não escreve `metricas.md`; decide o que fazer com o item 10 enquanto ele falta |
| `index.md` é exigido pelo item 10 | a página precisa estar linkada no índice da pasta | `wyd-db-1.0.0.md` §10 | a M33 decide **quando** criar o índice, não se ele é exigido |
| `serie-e-regras.md` é da 5ª onda | fora das 34 páginas e da M33 | `wyd-db-1.0.0.md` §11 | não entra nesta rodada |
| página antes da migration | `AGENTS.md` §5, item 1 | `AGENTS.md` §5, item 1 | a M33 decide **como a onda A percorre** as 34, não se o critério é por página |
| retenção, fila, idempotência, dado pessoal | `M22`, `M25`, `M30`, `M32` | `lacunas.md` §2.3 | fora da M33; não reabrir |

## 4. As seis decisões

### Q-M33-1 — `migrations/` segue a regra de estado de página ou é exceção?

**Medido.** `convencoes.md` exige a linha em toda página e chama de rascunho a que não tem; em
`migrations/`, **27 de 28** escritas estão sem ela, e a única com linha é `0002-ops-audit.md`. O
checklist tem 5 estados de **arquivo**, nenhum `aceito`. A regra geral já está decidida (seção 3); o
que não está decidido é se `migrations/` fica fora dela.

**O que está em jogo.** Sem linha de estado na página, o `wyd-db` não tem onde registrar que uma
página de migration foi aprovada — e `AGENTS.md` §5, item 1 exige essa aprovação antes de cada `.sql`. Com
exceção, `aceito` deixa de existir para as 34 e o critério de §11.1 ("todas ... `aceito` ou
`retirado`") fica sem objeto.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | `migrations/` segue a regra: `**Estado:** <estado> (data)` em todas; o checklist continua dono do estado do **arquivo** | uma linha em 27 páginas e uma frase de amarração em `convencoes.md` e no checklist; `aceito` passa a existir por página |
| B | Exceção: a página de `migrations/` não tem linha; seu estado é o do checklist, e `convencoes.md` declara a exceção | `aceito` deixa de existir para as 34; §11.1 fica sem objeto; o gate de aceite precisa de caso especial |
| C | Linha só a partir do `aceito` | estado implícito até a aprovação: quem lê não sabe se a página está proposta, esquecida ou reprovada |

**Recomendação: A.** `concluída` no checklist significa "escrita e conferida", não "aprovada"; são
fatos diferentes e podem ter nomes diferentes sem duplicação. **Custo:** 27 linhas e duas frases,
nenhum corpo alterado. **Fontes:** `docs/convencoes.md` §Estados (a regra já vale para página);
`AGENTS.md` §5, item 1 (aprovação por página antes da migration); `AGENTS.md` §5, item 5 (dois estados, dois
donos, sem espelho). **Recusa B** porque retira do `wyd-db` a única forma de registrar a aprovação que
§5.1 exige.

### Q-M33-2 — As 27 páginas recebem a linha agora ou na revisão de cada uma?

**Medido.** As 27 páginas estão escritas e sem linha; pela regra vigente, são rascunho. O estado
`proposto` é o único legal para uma página escrita que segue ativa e não foi aprovada (`AGENTS.md`
§5.3), e `convencoes.md` manda datar o estado, não a edição. Não há número de volume a estimar: são
27 arquivos contados.

**O que está em jogo.** Deixar sem linha mantém a dívida crescendo a cada página nova escrita no
mesmo molde, e o estado real ("escrita, não aprovada") fica invisível. Aplicar agora não aprova
ninguém: só declara o que já é verdade.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Aplicar `**Estado:** proposto (2026-11-23)` nas 27 agora; a data é a de **atribuição** do estado | 27 páginas saem de "rascunho por regra" para "proposto com data"; nenhuma vira `aceito` |
| B | Deixar sem linha até a revisão de cada uma | mantém as 27 como rascunho; a dívida cresce a cada página nova escrita assim |

Aceitar em bloco não entra na tabela: viola `AGENTS.md` §5, item 3 e o item 9 de §10 (revisão por quem não
escreveu), então não é alternativa — é a hipótese que a página existe para impedir.

**Recomendação: A.** É a leitura honesta do que existe: página escrita, não aprovada; a data é de
atribuição, como `convencoes.md` exige. **Custo:** 27 arquivos ganham uma linha; nenhuma outra
mudança, e nenhuma página vira `aceito` por isso. **Fontes:** `docs/convencoes.md` §Estados
("`proposto` sem data não diz há quanto tempo espera revisão"); `AGENTS.md` §5, item 3.

### Q-M33-3 — O item 10 de §10 vale antes de `metricas.md` existir?

**Medido.** O item 10 mistura duas condições: "linkada no `index.md` da pasta" (navegação, objeto de
`Q-M33-4`) e "contabilizada em `planos/metricas.md`" (medição, impossível hoje). `M20` reservou
`metricas.md` ao número medido e ele só existe depois da série; `planos/metricas.md` está listado como
"não escrito". Com §5.1 exigindo página `aceita` antes da migration, o impasse é circular: a série não
começa sem páginas aceitas; as páginas não são aceitas sem `metricas.md`; e `metricas.md` mede a
série.

**O que está em jogo.** Manter o item 10 inteiro bloqueia todo aceite do `wyd-db` até a série
existir; suspender a metade errada libera o aceite sem número medido. O próprio §10 já classifica
9–11 como "pendência de fechamento".

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Suspender **só a segunda metade** ("contabilizada em `planos/metricas.md`") até `M20` destravar; a exigência de `index.md` continua | o `wyd-db` volta a ter caminho para `aceito`; a suspensão fica escrita em §10 e na M33, com data e condição de retomada |
| B | Manter o item 10 inteiro | nenhuma página é aceita antes da série existir; com §5.1, a série não começa |
| C | Suspender o item 10 inteiro para `migrations/` | reabriria a exigência de `index.md`, fixada em §3; a hipótese é recusada e não vira recomendação |

**Recomendação: A.** Uma condição é navegação e a outra é medição; suspender a segunda com data e
condição é diferente de suprimi-la. **Custo:** reescrever um item de §10 e revalidar o aceite de toda
página quando `metricas.md` existir — dívida declarada, não esquecida. **Fontes:** `wyd-db-1.0.0.md`
§10 (itens 9–11 como pendência de fechamento) e §11.1; `lacunas.md` `M20`; `AGENTS.md` §5, item 1.
**Recusa B** porque mantém o impasse circular; **C** porque troca a condição verificável por uma
suspensão total.

### Q-M33-4 — O `migrations/index.md` entra agora e com qual conteúdo?

**Medido.** A pasta tem 28 páginas e **nenhum índice**; o item 10 exige que a página esteja "linkada
no `index.md` da pasta". O conteúdo também é parte da pergunta: o índice não espelha estado; ele traz
títulos, links e a frase que manda o leitor à página. `index.md` e `serie-e-regras.md` são a 5ª onda
de §11; `serie-e-regras` depende da política da série (`plano-migrations.md` §6), que ainda não foi escrita.

**O que está em jogo.** Sem índice, a metade de navegação do item 10 não pode ser satisfeita, e a
pasta não tem porta de entrada. Criar `serie-e-regras` agora antecipa decisão de política que não é
desta rodada.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Criar `migrations/index.md` agora: títulos, links e a frase de que o estado mora na página; `serie-e-regras.md` continua na 5ª onda | o item 10 ganha alvo real; a pasta deixa de ter 28 páginas sem porta de entrada; o índice não duplica o estado (§5.5) |
| B | Adiar o índice; a exigência do item 10 continua e a pasta segue sem porta de entrada até a 5ª onda | o aceite fica sem a metade de navegação; a exigência não muda |
| C | Criar `index.md` **e** `serie-e-regras.md` agora | antecipa a 5ª onda e a política da série, que `plano-migrations.md` §6 ainda não escreveu |

**Recomendação: A.** O índice é navegação e não decide nada; ele traz títulos, links e a frase de que o estado mora na página, sem copiá-lo (§5.5). `serie-e-regras` decide e por isso fica
na onda própria. **Custo:** uma página nova em `migrations/`, fora das 34 posições de
`migrations-checklist.md` §2, então a contagem de páginas escritas não muda. **Fontes:**
`wyd-db-1.0.0.md` §10 item 10 e §11; `plano-migrations.md` §6; `migrations-checklist.md` §2 e §4.
**Recusa C** porque antecipa a 5ª onda.

### Q-M33-5 — O checklist ganha coluna de estado da página?

**Medido.** O checklist tem 5 estados de **arquivo**; `concluída` deriva o estado de tabela em §1.2.
O estado da página é a linha `**Estado:**` dela. Hoje não há coluna que espelhe esse estado, e
`AGENTS.md` §5, item 5 exige um dono por fato.

**O que está em jogo.** Quem lê o checklist pode supor que ali está o aceite. Espelhar o estado na
tabela resolveria a leitura local e criaria dois donos para o mesmo fato.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Sem coluna nova: o checklist continua dono do estado do **arquivo** e §1 ganha uma frase separando os dois estados | um dono por fato; o leitor é mandado à página para saber se está aceita |
| B | Coluna nova espelhando o estado da página | dois lugares com o mesmo estado; quando divergirem, uma das duas está errada por construção |
| C | Remover `concluída` e usar o estado da página no lugar | quebra §1.2, que deriva o estado de **tabela** de `concluída`; a derivação passaria a depender de aprovação, que não é o mesmo fato |

**Recomendação: A.** `concluída` responde "a página existe e foi conferida?"; `aceito` responde "o
dono aprovou?". **Custo:** uma frase em §1 e uma visita a mais para quem quer o estado de aceite.
**Fontes:** `AGENTS.md` §5, item 5; `migrations-checklist.md` §1 e §1.2; `docs/convencoes.md` §Estados.
**Recusa B** porque cria a divergência que §5.5 proíbe; **C** porque muda o que §1.2 deriva.

### Q-M33-6 — Como a onda A atravessa o aceite das 34 páginas?

**Medido.** A onda A (`wyd-1.0.0.md` §7.7) precisa que a série comece; `AGENTS.md` §5, item 1 exige página
`aceita` antes da migration que ela descreve. São 34 páginas, **28 escritas**; 6 dependem de dado de
origem e curadoria de conteúdo, e não existem.

**O que está em jogo.** Aceitar as 34 antes do primeiro `.sql` trava o DDL por assunto que não é DDL;
aceitar por lote esconde página não revisada dentro do agregado.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Por página, na ordem da série: cada página aceita antes da migration que ela descreve, começando pelas 28 escritas | §5.1 ao pé da letra; `0001` pode começar sem esperar `0027` |
| B | As 34 aceitas antes do primeiro `.sql` | as 6 páginas de seed dependem de curadoria de conteúdo e travariam o DDL inteiro |
| C | Aceitar por lote (bloco operacional, identidade, conteúdo...) | um lote aceito esconde página não revisada dentro dele |

**Recomendação: A.** O critério de §5.1 é por página e a dependência real também: `0001` não precisa
que `0027` esteja pronta. **Custo:** 34 atos de aceite, um a um, sem atalho; a série espera o aceite da
página que vai executar, não o das outras. **Fontes:** `AGENTS.md` §5, item 1; `plano-migrations.md` §3 (a
ordem); `migrations-checklist.md` §4 (o painel, sem número novo). **Recusa B** porque transforma
bloqueio de dado em bloqueio de schema; **C** porque troca critério verificável por agregado.

## 5. O quadro que as seis decisões produzem

| Decisão | Efeito se A vencer | Já decidido que não muda |
|---|---|---|
| `Q-M33-1` | linha de estado nas 27 páginas de `migrations/` | a regra de estado e o estado `proposto` |
| `Q-M33-2` | `proposto (2026-11-23)` nas 27, nenhuma `aceito` | §5.3: sem "sim" não há `aceito` |
| `Q-M33-3` | item 10 sem a metade de `metricas.md`, com retomada | `M20`: `metricas.md` só depois da série |
| `Q-M33-4` | `migrations/index.md` criado com títulos, links e a frase de que o estado mora na página; `serie-e-regras` na 5ª onda | §10 item 10: índice obrigatório |
| `Q-M33-5` | checklist sem coluna de página, com frase de separação | §5.5: um fato, um dono |
| `Q-M33-6` | aceite por página, na ordem da série | §5.1: página antes da migration |

## 6. O que esta rodada não decide

- **Não aceita página nenhuma.** Aprovar as opções decide a regra; o aceite é ato do dono, página a
  página (`Q-M33-6`).
- **Não escreve `metricas.md`** nem antecipa número: o dono do número medido é `M20`.
- **Não reabre `M20`, `M22`, `M25`, `M30`, `M7`, `P-22`, `A12` nem `M32`.**
- **Não decide o conteúdo de seed** (`0020`, `0023`, `0027`) nem a curadoria de conteúdo.
- **Não muda o gate do site** nem o `verificar.py`: `migrations/` não é publicado, e a linha de estado
  dessas páginas é regra de planejamento, não de renderização.
- **Não cria índice de banco nem job:** o DDL não é tocado por esta rodada.

## 7. O que muda quando as seis forem respondidas

| # | Entrega | Arquivo | Dono |
|---|---|---|---|
| 1 | frase separando estado de página e estado de arquivo | `docs/convencoes.md` §Estados | `wyd-db` |
| 2 | mesma amarração e linha de estado no conteúdo mínimo de §5 | `migrations-checklist.md` §1 e §5 | `wyd-db` |
| 3 | 27 linhas `**Estado:** proposto (2026-11-23)` | `migrations/*.md` | `wyd-db` |
| 4 | item 10 reescrito com a suspensão datada | `wyd-db-1.0.0.md` §10 | arquitetura |
| 5 | índice da pasta, com títulos, links e a frase de que o estado mora na página | `migrations/index.md` (nova) | `wyd-db` |
| 6 | registro da resposta do dono e atualização do estado de `M33` | `lacunas.md` §2.3 e §1 | `wyd-db` |

A medição de partida é a de `wyd-docs/proposta-gate-de-aceite-de-pagina.md` §5.2 e §7: cabeçalho
`5/5` em 29 páginas, linha de estado em **2 de 29**, placeholder `0` e o gate do site em exit 0.
Depois da aplicação das 27 linhas, a linha de estado passa a **29 de 29** (as 27 novas + `0002` +
`goldens/como-capturar.md`). O número 29 é contagem de arquivo (28 páginas de migration +
`goldens/como-capturar.md`), não estimativa.

## 8. O que continua bloqueado depois desta rodada

- **O aceite de cada página do `wyd-db`** — ato do dono, página a página; a regra que o permite está
  decidida, mas nenhuma página foi aceita nesta rodada.
- **A contabilização em `metricas.md`**, até `M20` destravar.
- **As 6 páginas de migration ainda não escritas** (`0016`–`0018`, `0020`, `0023`, `0027`), que
dependem de dado de origem.
- **`serie-e-regras.md`**, da 5ª onda; o índice foi antecipado por `Q-M33-4` e existe, mas a política
  da série não.
- **O conteúdo dos seis seeds** (`0016`–`0018`, `0020`, `0023`, `0027`), que depende de dado de origem.

## 9. Registro da decisão

**Respondidas em 2026-11-23.** O dono aprovou a opção A nas seis perguntas; a `M33` fecha em
`lacunas.md` §2.3 e as entregas de §7 foram aplicadas nesta rodada. O registro abaixo é o ato do
dono; a página continua `proposto` até a revisão editorial.

| Pergunta | Resposta | Data |
|---|---|---|
| `Q-M33-1` | A — linha de estado na página; checklist dono do estado do arquivo | 2026-11-23 |
| `Q-M33-2` | A — `proposto (2026-11-23)` nas 27 páginas | 2026-11-23 |
| `Q-M33-3` | A — suspender só a metade de `metricas.md` do item 10, com retomada | 2026-11-23 |
| `Q-M33-4` | A — criar `migrations/index.md` agora; `serie-e-regras` na 5ª onda | 2026-11-23 |
| `Q-M33-5` | A — checklist sem coluna de estado da página | 2026-11-23 |
| `Q-M33-6` | A — aceite por página, na ordem da série | 2026-11-23 |
