# Sistemas de jogo — mapa de planejamento

> Pergunta: "quais sistemas de jogo (guerras, mobs, NPCs, canais, skills, classes, quests, armas, armaduras, lojas, mercado, crafting, magias, feitiços, poções, pets, companheiros, montarias) estão mapeados, quem é o dono e como cada um será provado?"
> Público: Produto, arquitetura, desenvolvimento e QA
> Dono: `wyd-core` (modelo) + `wyd-db` (catálogo/dado) + `wyd-sim` (fórmulas) + `wyd-net`/`wyd-server` (runtime) + `wyd-portal` (BFF)
> Base: `wyd-core/docs/specs/catalogo-funcional.md`; `wyd-db/docs/dicionario-dados/dominio-catalogo.md`; `wyd-db/docs/dicionario-dados/dominio-combate.md`; `wyd-db/docs/dicionario-dados/dominio-progressao.md`; `wyd-db/docs/dicionario-dados/dominio-quests.md`; `wyd-db/docs/dicionario-dados/dominio-economia.md`; `wyd-db/docs/dicionario-dados/dominio-inventario.md`; `wyd-ops/auditoria-de-organizacao.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Planejamento; nenhum catálogo, fórmula, código ou teste foi criado por este documento.

## 1. Regra de dono

| Camada | Dono | O que decide |
|---|---|---|
| Modelo de jogo | `wyd-core` | entidade, regra de domínio, validação e fronteira |
| Catálogo e dado persistido | `wyd-db` | schema, seed, chave natural, integridade e retenção |
| Fórmula e curva | `wyd-sim` | cálculo determinístico, fixture capturada e `stat_source` |
| Transporte e sessão | `wyd-net`/`wyd-server` | pacote, sessão, rate-limit, canal e hardening |
| Experiência e BFF | `wyd-portal` | rota, permissão, leilão, armaria e mercado |
| Operação | `wyd-ops` | CI, imagem, ambiente, monitoramento, deploy e gate |

Um sistema de jogo não é dono de sua própria tabela: o catálogo e o dado persistido continuam no `wyd-db`, e a fórmula continua no `wyd-sim`.

## 2. Mapa de sistemas de jogo

| Área | Modelo | Catálogo/dado | Fórmula | Runtime | Estado |
|---|---|---|---|---|---|
| Guerra / PvP / canal | `wyd-core` | `wyd-db` (`game.*`, `content.*`) | `wyd-sim` | `wyd-net`/`wyd-server` | proposto |
| Mobs / IA | `wyd-core` | `wyd-db` (`content.mob_template_*`, `content.npc_template`) | `wyd-sim` | `wyd-server` | proposto |
| NPCs / lojas | `wyd-core` | `wyd-db` (`content.npc_*`, `content.item_catalog`) | n/a | `wyd-server` | proposto |
| Classes | `wyd-core` | `wyd-db` (`content.game_class`) | `wyd-sim` | `wyd-server` | proposto |
| Skills / feitiços / magias | `wyd-core`/`wyd-sim` | `wyd-db` (`content.skill_catalog`, `content.skill_lineage`) | `wyd-sim` | `wyd-server` | proposto |
| Quests | `wyd-core` | `wyd-db` (`content.quest*`) | n/a | `wyd-server` | proposto |
| Armas / armaduras / itens | `wyd-core` | `wyd-db` (`content.item_catalog`, `content.item_kind`, `content.item_effect`) | `wyd-sim` quando houver curva | `wyd-server` | proposto |
| Lojas / mercado / crafting | `wyd-core`/`wyd-portal` | `wyd-db` (`content.*`, `market.*`) | `wyd-sim` quando houver curva | `wyd-server`/`wyd-portal` | proposto |
| Poções / buffs | `wyd-core`/`wyd-sim` | `wyd-db` (`content.item_catalog`, `content.item_effect`) | `wyd-sim` | `wyd-server` | proposto |
| Pets / companheiros | `wyd-core` | `wyd-db` (`game.*`, `content.*`) | `wyd-sim` quando houver curva | `wyd-server` | proposto |
| Montarias | `wyd-core` | `wyd-db` (`game.*`, `content.*`) | `wyd-sim` quando houver curva | `wyd-server` | proposto |
| Canais / chat | `wyd-net`/`wyd-server` | `wyd-db` (`content.server_channel` quando aplicável) | n/a | `wyd-net`/`wyd-server` | proposto |

## 3. Fontes de verdade

- Catálogo de item: `wyd-db/docs/dicionario-dados/dominio-catalogo.md`.
- Combate e drop: `wyd-db/docs/dicionario-dados/dominio-combate.md`.
- Progressão, skills aprendidas e maestria: `wyd-db/docs/dicionario-dados/dominio-progressao.md`.
- Quests: `wyd-db/docs/dicionario-dados/dominio-quests.md`.
- Economia, mercado e lojas: `wyd-db/docs/dicionario-dados/dominio-economia.md`.
- Inventário e equipamento: `wyd-db/docs/dicionario-dados/dominio-inventario.md`.
- Mundo, NPCs e mobs: `wyd-db/docs/dicionario-dados/dominio-mundo.md`.
- Fórmulas e curvas: `wyd-sim` (a escrever; hoje sem fórmula nem fixture).
- Captura do cliente 7662: `wyd-client/docs/specs/inventario-da-arvore.md` e `wyd-client/docs/specs/pedido-ao-titular.md`.

## 4. Contratos e fronteiras

| Troca | Dono do contrato | Garantia mínima |
|---|---|---|
| `wyd-core` ↔ `wyd-db` | `wyd-db` | chave natural, integridade, grant e migração versionada |
| `wyd-core` ↔ `wyd-sim` | `wyd-sim` | determinismo, fixture capturada e `stat_source` declarado |
| `wyd-server` ↔ `wyd-net` | `wyd-net` | pacote, sessão, limite, timeout e hardening |
| `wyd-portal` ↔ `wyd-server` | `wyd-admin`/`wyd-portal` | token de serviço por ambiente, ack antes de escrita |
| `wyd-core` ↔ `wyd-portal` | `wyd-portal` | BFF não duplica regra de jogo; apenas rota e permissão |

## 5. Testes por área

| Área | Unitário | Contrato | Integração/sistema | Propriedade |
|---|---|---|---|---|
| Guerra/PvP | regra de dano e alcance | pacote e sessão | cenário de arena | fórmula determinística |
| Mobs/IA | transição de estado | template e drop | spawn e combate | drop por peso |
| NPCs/lojas | validação de compra | catálogo e inventário | fluxo de compra | n/a |
| Classes/skills | aprendizado e maestria | catálogo e progressão | personagem aprende skill | curva de maestria |
| Quests | estado e recompensa | quest e inventário | fluxo de quest | n/a |
| Itens/equipamento | slot e requisito | inventário e catálogo | equipar/desequipar | n/a |
| Mercado/crafting | preço e taxa | mercado e inventário | leilão e craft | economia |
| Poções/buffs | duração e efeito | item e personagem | aplicação e expiração | curva de efeito |
| Pets/companheiros | vínculo e skill | catálogo e personagem | invocação | n/a |
| Montarias | vínculo e deslocamento | personagem e cena | montar/desmontar | n/a |
| Canais/chat | filtro e limite | pacote e sessão | chat em canal | rate-limit |

Regra: teste de linha não substitui capacidade provada. Toda área precisa de capacidade `CAP-nn` ou equivalente antes de ser considerada coberta.

## 6. DoD por área

- **Guerra/PvP:** fórmula reproduz fixture; canal e arena têm cenário verde.
- **Mobs/IA:** template e drop têm teste; `weight` não é lido como porcentagem.
- **NPCs/lojas:** compra valida catálogo, inventário e preço.
- **Classes/skills:** maestria respeita teto da classe e `stat_source` declarado.
- **Quests:** estado, recompensa e idempotência provados.
- **Itens/equipamento:** slot 0–15 e chave natural respeitados.
- **Mercado/crafting:** taxa, escrow e concorrência provados.
- **Poções/buffs:** duração e expiração provadas.
- **Pets/companheiros:** vínculo e skill provados.
- **Montarias:** vínculo e deslocamento provados.
- **Canais/chat:** limite e isolamento por canal provados.

## 7. Hipóteses e lacunas

| Ponto | Marca | O que falta |
|---|---|---|
| contagens de seed (3.220 itens, 154 skills) | `H` | reconferir na fonte |
| semântica de `grade` e `requirements` | `H` | trace no planejamento novo |
| fórmulas de combate | `H` | captura e fixture (`LCLI-02`/`LCLI-03`) |
| por que 3 skills vêm da Kersef | `H` | origem medida, significado não |
| curadoria dos 6 seeds bloqueados | `H` | dado externo |
| balanceamento de guerra, mercado e crafting | `H` | fórmulas e fixtures do `wyd-sim` |

Nenhuma hipótese vira regra sem reverificação na fonte nova.

## 8. Cruzamento com auditoria e especificação

| Referência | Relação |
|---|---|
| `wyd-ops/auditoria-de-organizacao.md` §2–§3 | 13 sistemas organizados; `wyd-ops` mantém fronteira/contratos parciais; este mapa não fecha organização por si só |
| `wyd-core/docs/specs/catalogo-funcional.md` | mapa de fronteira dos sistemas de jogo |
| `wyd-db/docs/dicionario-dados/dominio-catalogo.md` | catálogo de item e skill |
| `wyd-db/docs/dicionario-dados/dominio-combate.md` | stats, equipamento e drop |
| `wyd-db/docs/dicionario-dados/dominio-progressao.md` | maestria e skill aprendida |
| `wyd-db/docs/dicionario-dados/dominio-quests.md` | quests |
| `wyd-db/docs/dicionario-dados/dominio-economia.md` | economia e mercado |
| `wyd-db/docs/dicionario-dados/dominio-inventario.md` | inventário e equipamento |
| `wyd-db/docs/dicionario-dados/dominio-mundo.md` | mundo, NPCs e mobs |
| `wyd-client/docs/specs/mapa-de-funcionalidades.md` | funcionalidades do cliente |

## 9. O que esta página não cobre

- Números de balanceamento: dono `wyd-sim`, com fixture capturada.
- Conteúdo seedado: depende de curadoria.
- Implementação de código, teste ou migration.
- ADR formal: ver `wyd-ops/docs/arquitetura/adr/index.md`.