# Versões no board e cobertura funcional da `1.0.0` — proposta

> **Estado: proposta (`P`) com direção decidida em 2026-11-20; nada executado.** O dono escolheu as
> quatro opções recomendadas (§9). A migração de versões continua **não executada**: os 6 milestones
> `M0`–`M5`, os campos do project, as labels e os formulários continuam como em
> `proposta-gestao-github-e-ambientes.md` §10. Em 2026-11-22 foram criadas 7 milestones de fase
> (`F1`–`F7`) e as 40 issues `WYDDB` #82–#121; isso não altera o eixo de versão, que segue pendente.
>
> Este documento pertence a `wyd-ops` porque trata de board, marco e sprint. Ele **não** decide
> arquitetura, funcionalidade de jogo nem escopo de produto; a lista de funcionalidades de §7 é
> levantamento a fazer, não catálogo pronto. A decisão é do dono de operação, com produto.

---

## 1. O que existe hoje

| Item | Estado | Fonte |
|---|---|---|
| Milestones | 13 criados: `M0`–`M5` (nº 1–6) + `F1`–`F7` (nº 7–13) | `proposta-gestao-github-e-ambientes.md` §10 |
| Campos do project `WYD 1.0.0` | `Sistema` (13), `Fase` (0–5), `Prioridade` (P0–P3), `Marco` (M0–M5), `Status` | idem |
| Labels | 13 `area/*`, 3 `estado/*`, `origem/legado`, 2 `risco/*` | idem |
| Issues | **40** `WYDDB` (#82–#121) no project `WYD 1.0.0` | `wyd-1.0.0.md` §1.4 |
| Série de versões | só no `wyd-protocol`, `0.1.0` → `1.0.0` | `wyd-protocol-1.0.0.md` §9.1 |

O board não tem hoje nenhum eixo de versão: versão existe em um sistema e não aparece no trabalho.

---

## 2. Premissa declarada pelo dono (2026-11-20)

> Todos os sistemas e funcionalidades do jogo oficial devem estar **100% funcionais na `1.0.0`**.

Isso é premissa desta proposta, não decisão registrada em plano: `wyd-1.0.0.md` §4 ainda descreve
fases de **planejamento** e a `1.0.0` como meta de integração. Mudar o alvo para “jogo completo” é
decisão de produto e precisa ser escrita no panorama, com o critério de saída refeito. Esta proposta
não faz essa mudança; ela mostra o que muda no board se ela for tomada.

Consequência prática: o eixo de versão deixa de ser detalhe do `wyd-protocol` e passa a organizar
**todo** o trabalho — daí a pergunta de §3.

---

## 3. Como a série aparece no board — quatro opções

| Opção | Como aparece | Muda `M0`–`M5`? | Ganha | Perde |
|---|---|---|---|---|
| **A — milestone por versão** | 10 milestones `v0.1.0` … `v1.0.0`; fases viram labels `fase/0`–`fase/5` e o campo `Fase` continua | sim, quando executada | progresso % por versão, semântica de release, uma issue = uma versão | vocabulário de marco muda no panorama e nos documentos que citam `M0`–`M5` |
| **B — campo `Versão` no project** | single select `0.1.0`…`1.0.0` + view “Por versão” | não | zero migração, reversível, agrupa e filtra | sem barra de progresso; versão não fecha sozinha, só por view |
| **C — label `versao/0.x.0`** | 10 labels + auto-add filtrado | não | entra no auto-add e na busca | ordenação não existe, progresso não existe, mais 10 labels para manter |
| **D — sprint (`Iteration`)** | campo de iteração por sprint | não | cadência e burndown | sprint é tempo, não entrega; depende de o campo existir no plano — **verificar antes**, não assumir |

Sprint e versão não competem: versão é **o que** entrega, sprint é **quando** se trabalha. Uma versão
pode ocupar mais de um sprint, e um sprint pode cruzar duas versões pequenas.

---

## 4. Recomendação — decidida em 2026-11-20

**Decidida: opção A, migração única e imediata.** O dono escolheu o milestone por versão e a
reformulação agora, com zero issues. Com zero issues o custo é hoje; depois da primeira issue, cada
migração vira retrabalho de board. A opção B (campo `Versão`) fica descartada como passo permanente,
mas continua sendo o substituto reversível se a execução atrasar.

Sequência aprovada (não executada):

1. criar as labels `fase/0` … `fase/5`, preservando os nomes `M0`–`M5` no texto da label;
2. criar 10 milestones `v0.1.0` … `v1.0.0`, com o critério de fechamento citado de §9.1 do plano do
   protocolo para as versões dele e, para as demais, o gate do sistema dono;
3. trocar o campo `Marco` por `Versão` no project e nos 4 formulários de issue, mantendo `Fase`;
4. fechar os 6 milestones `M0`–`M5` — sem deleção, para o histórico continuar legível;
5. atualizar as referências a `M0`–`M5` em `wyd-1.0.0.md` §4, na proposta de gestão e no README do
   project.

Custo estimado: 6 milestones, 10 novos, 6 labels, 1 campo e 4 formulários — sem issue para migrar.
Risco: perder o vocabulário de fase no board; mitigação é a label `fase/*` e o campo `Fase`.

---

## 5. O que muda com a reformulação decidida

| Hoje | Depois |
|---|---|
| marco = fase do projeto | marco = versão de entrega |
| fase no campo `Fase` e no milestone | fase no campo `Fase` e na label `fase/*` |
| `1.0.0` é meta de integração | `1.0.0` é o jogo completo, com cobertura funcional provada |
| board mede fase | board mede versão, e a soma das versões é a `1.0.0` |

Nada disso cria issue. A criação continua dependendo de página com DoD e de pedido explícito
(`AGENTS.md` §5, item 9).

---

## 6. Sprints

Sprint só entra depois de duas coisas: inventário funcional (§7) e versão definida no board. Sem o
inventário, sprint é caixa de tempo sem conteúdo verificável.

Proposta, se o dono quiser cadência: usar `Iteration` no project **se o campo existir na conta** —
verificar na criação, não presumir — e nomear sprint por versão e foco (`v0.3.0 — movimento`). O
sprint não substitui o gate da versão e não recebe data neste documento.

---

## 7. Cobertura funcional — o que existe e o que falta

**Resposta direta: não, o jogo não está mapeado.** O que existe são **13 sistemas técnicos**
(`wyd-protocol`, `wyd-net`, `wyd-core`, `wyd-server`, `wyd-sim`, `wyd-db`, `wyd-admin`, `wyd-portal`,
`wyd-launcher`, `wyd-client`, `wyd-bots`, `wyd-docs`, `wyd-ops`), quase todos vazios. Eles dizem
**onde** o código mora, não **quais** funcionalidades o jogador tem. Não há hoje catálogo de classes,
skills, refinamento, guerras, cidadania, estados especiais, montarias, invocações, transformações,
magias, quests ou guildas — nem contagem, nem fluxo, nem prova.

O que dá para afirmar hoje: o `deprecated/` tem código legado de parte disso (referência `H`, nível 4
de `AGENTS.md` §3) e o `wyd-client/files/` tem o cliente, ignorado pelo git. **Nenhuma das duas é
fonte de verdade**; a fonte é o cliente 7662 medido, como já decidido em `LPRT-04`.

### 7.1 Estrutura proposta do catálogo funcional

Uma entrada por funcionalidade, com sete campos — o mesmo molde de campo de `payloads/`, aplicado a
comportamento:

| Campo | Conteúdo |
|---|---|
| `id` | identificador estável, nunca reaproveitado |
| `domínio` | classe, skill, guerra, guilda, estado especial, montaria… |
| `sistema dono` | qual dos 13 responde pela regra |
| `fluxos` | passos do jogador e do servidor, com estados |
| `prova` | golden, captura datada ou leitura do cliente que sustenta a entrada |
| `versão` | em que versão da série entra |
| `estado` | `H`, `P` ou `aceito` — `aceito` só com prova e segunda leitura |

**Gate da `1.0.0`:** todas as entradas `aceito`, com fluxo e prova; nenhuma entrada aberta. O total é
medido pelo catálogo, não estimado antes do levantamento.

### 7.2 Onde o catálogo mora — decidido em 2026-11-20

**Decidida: dividir por natureza, em vez de criar sistema novo** — `wyd-core` para comportamento,
`wyd-sim` para fórmula, `wyd-db` para conteúdo, `wyd-protocol` para mensagem e `wyd-admin` para
operação. `wyd-content` não entra.

| Parte | Dono recomendado |
|---|---|
| comportamento: classe, skill, combate, guerra, guilda, estados | `wyd-core` |
| fórmulas, curvas, chance, dano, drop | `wyd-sim` |
| conteúdo: itens, mapas, NPCs, quests, tabelas | `wyd-db` |
| mensagens e opcodes de cada fluxo | `wyd-protocol` |
| operação de GM e staff | `wyd-admin` |

Criar um `wyd-content` é alternativa legítima, mas é fronteira nova: exige aprovação de arquitetura
antes de qualquer página (`LPDOC-08`) e não é recomendada agora.

### 7.3 Domínios a levantar — lista do dono, não catálogo

Os itens abaixo foram citados pelo dono em 2026-11-20. São **domínios a mapear**, sem contagem e sem
afirmar que existem no cliente: toda entrada nasce `H` até a leitura.

| Grupo | Exemplos citados |
|---|---|
| progressão | classes, níveis, atributos, evolução |
| habilidades | skills, magias, evocações, transformações |
| itens | refinamento, upgrade, crafting, drop, loja |
| reinos | cidadania, reinos, impostos, teleporte |
| guerra e social | guerras, guildas, PvP, chat |
| estados especiais | mortal, arch, celestial, subcelestial, hardcore |
| criaturas | montarias, invocações, pets, NPCs |
| conteúdo | quests, eventos, mapas, spawns |
| operação | launcher, patch, portal, painel de GM |

### 7.4 Como levantar sem inventar

1. escolher um domínio e listar o que o **cliente** expõe (mensagens, telas, arquivos de configuração);
2. comparar com o legado, marcando cada achado como `H` e citando arquivo:linha;
3. capturar um fluxo por funcionalidade, com o procedimento de `goldens/como-capturar.md`;
4. escrever a entrada com prova, dono, fluxo e versão;
5. só então a funcionalidade entra no board como issue, com DoD.

Sem esse levantamento não existe plano de sprint honesto para a `1.0.0` — só uma lista de desejos.

---

## 8. O que esta proposta não faz

- não cria milestone, label, campo, view, sprint ou issue;
- não altera `M0`–`M5` nem o project;
- não decide o dono do catálogo funcional nem a fonte de verdade;
- não afirma contagem de funcionalidades, classes, skills ou quests;
- não muda o alvo da `1.0.0` no panorama — isso é decisão de produto, a registrar lá.

---

## 9. Decisões pedidas e resposta do dono (2026-11-20)

| # | Decisão | Resposta | Dono |
|---|---|---|---|
| 1 | eixo de versão no board | **decidida:** milestone por versão, migração única; campo `Versão` só como substituto se a execução atrasar | operação |
| 2 | reformular `M0`–`M5` | **decidida:** sim, em migração única enquanto não há issue | operação + produto |
| 3 | dono do catálogo funcional | **decidida:** dividir — `wyd-core` (comportamento) + `wyd-sim` (fórmulas) + `wyd-db` (conteúdo), sem sistema novo | arquitetura |
| 4 | fonte de verdade do catálogo | **decidida:** cliente 7662 medido; legado e `w2pp` só `H` | produto + arquitetura |
| 5 | alvo da `1.0.0` | **declarada:** “jogo completo”, com gate de cobertura 100%; falta registrar no panorama | produto |

## 10. O que ainda falta para executar

1. **Pedido explícito** para escrever no GitHub — labels `fase/*`, 10 milestones `v0.1.0`–`v1.0.0`,
   troca do campo `Marco` por `Versão` e ajuste dos 4 formulários. Nenhuma chamada foi feita.
2. **Registrar no panorama** a mudança de alvo da `1.0.0` (jogo completo), com o critério de saída
   refeito; hoje `wyd-1.0.0.md` §4 ainda mede fases de planejamento.
3. **Planejar `wyd-core` e `wyd-sim`** antes de qualquer página de catálogo funcional: as duas pastas
   já têm três páginas cada (2026-11-22); o que falta é o catálogo funcional e os requisitos, não o
   plano de sistema.
4. **Levantar o primeiro domínio** do catálogo, como piloto, com entradas `H` e prova por captura; o
   levantamento completo é frente própria, não um passo de board.
