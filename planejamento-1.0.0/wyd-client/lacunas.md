# `wyd-client` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-23).** Único lugar onde uma lacuna do `wyd-client` tem dono,
> impacto e estado. Nada aqui está decidido por esta abertura.

## 0. Como ler

Estados: **aberta**, **proposta**, **fechada** (com evidência). `arquitetura` propõe; o dono do
sistema aprova. Lacuna de outro sistema que este consome é citada pelo ID de origem.

## 1. Situação em uma linha

| Grupo | Lacunas | Abertas | Propostas | Fechadas |
|---|---:|---:|---:|---:|
| Medição do cliente | 2 | 2 | 0 | 0 |
| Distribuição | 1 | 1 | 0 | 0 |
| Processo | 1 | 0 | 1 | 0 |
| **Total** | **4** | **3** | **1** | **0** |

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `LCLI-01` | Qual build do cliente é o alvo, e qual é a identidade verificável dele (versão declarada e hash do binário)? | é `LPRT-09` vista do lado do cliente: sem build-alvo, offset e opcode ficam ambíguos | frente `wyd-client` + pesquisa | aberta |
| `LCLI-02` | O que exatamente o patch entrega (`strdef.bin`, assets, executável) e com que versionamento? | é a entrada do manifesto do `wyd-launcher`; sem lista, o patch vira cópia da árvore inteira | `wyd-client` + `wyd-launcher` | aberta |
| `LCLI-03` | Onde mora o registro de captura — ficha, data e quem aprova a segunda leitura? | o processo de `LPRT-10` está decidido, mas o lugar do registro e o aprovador não estão nomeados | QA + arquitetura | aberta |
| `LCLI-04` | Qual ferramenta faz a captura, onde ela mora, quem responde por ela e como impede gravar dado de jogador? | captura sem ferramenta e fronteira aprovadas vira script solto ou grava dado pessoal; `AGENTS.md` §5.6 proíbe executável em `planejamento-1.0.0/` | `wyd-ops` + pesquisa, com `wyd-client` e `wyd-protocol` como partes interessadas | proposta (sem opção escolhida) |

### 2.1 `LCLI-04` — opções para localização, aprovação e privacidade (propostas, 2026-11-23)

Nenhuma alternativa abaixo foi escolhida. Os caminhos são candidatos futuros, não pastas que já
existem nem autorização para criar código. A ferramenta deve capturar apenas ambiente controlado,
seguir `LPRT-10` e nunca gravar bruto com dados de jogador no repositório.

| Opção | Onde moraria | Quem aprova / responde | Como evita dado de jogador | Trade-offs |
|---|---|---|---|---|
| **A — ferramenta específica do cliente** | `wyd-client/tools/` (pasta ainda inexistente; artefato executável na raiz do projeto, não em `planejamento-1.0.0/`) | dono de `wyd-client` aprova localização e manutenção; `wyd-protocol` valida que a saída serve ao processo de golden; QA faz a segunda leitura exigida por `LPRT-10` | executar só com conta/personagem de teste sintéticos, contra servidor local ou proxy controlado, sem sessão de produção ou tráfego de terceiros; capturar somente o cenário necessário; se surgir dado real, interromper, não persistir nem versionar o bruto e descartar a captura. Bruto de sessão sintética, quando necessário para a segunda leitura, fica restrito fora do git; no repositório entra apenas frame anonimizado e ficha resumida | alinhada com a responsabilidade de captura do `wyd-client` e pode evoluir com o cliente; cria ferramenta específica e manutenção no sistema cliente |
| **B — utilitário de operação compartilhado** | `wyd-ops/` (subpasta ainda inexistente; alternativa é o dono implementar ferramenta operacional reutilizável) | dono de `wyd-ops` aprova localização e operação; `wyd-client` define o cenário de captura; `wyd-protocol` valida saída e QA faz a segunda leitura | mesmas barreiras da A: exclusivamente sessão sintética em ambiente controlado, sem produção nem terceiros; parar e descartar se aparecer dado real; bruto de teste restrito fora do git, e somente artefato anonimizado revisado pode ser versionado | centraliza dependências e execução operacional; pode misturar captura específica do cliente com infraestrutura comum e tornar `wyd-client` dependente de `wyd-ops` |
| **C — ferramenta externa, sem código no repositório** | instalada apenas na estação/ambiente de pesquisa, fora da árvore versionada | pesquisa registra a ferramenta e versão; dono de `wyd-client` aprova o uso no cenário; `wyd-protocol` e QA conferem evidência conforme `LPRT-10` | configurar a ferramenta para uma sessão sintética em ambiente controlado, sem produção/terceiros; interromper e descartar se aparecer dado real; bruto de teste restrito fora do repositório, sem versioná-lo; revisar e anonimizar antes de produzir o frame versionado | evita criar/manter executável próprio e permite começar com ferramenta disponível; repetibilidade depende de versão/configuração externa, e o projeto não controla seu ciclo de vida |

**Aprovação mínima a resolver antes de implementar:** o mantenedor do sistema escolhido aprova caminho,
proprietário e execução; se cruzar `wyd-client` e `wyd-ops`, ambos aprovam a fronteira. `wyd-protocol`
aprova a conformidade do artefato com `LPRT-10`, e uma pessoa diferente de quem capturou realiza a
segunda leitura. O nome da pessoa que aprova a ficha permanece em `LCLI-03`; não está decidido aqui.

**Salvaguardas propostas para qualquer opção:** (1) usar apenas conta e personagem de teste, com
valores sintéticos; nunca abrir sessão de produção ou capturar tráfego de outros jogadores; (2) usar
servidor local ou proxy sob controle da equipe e limitar a captura ao cenário de uma ação, com chat
vazio; (3) se a sessão ou o fluxo contiver dado real inesperado, interromper a captura, não persistir
nem versionar o bruto e descartá-lo — se não for possível garantir o ambiente sintético, não capturar;
(4) quando necessário à segunda leitura de `LPRT-10`, manter o bruto exclusivamente fora do
repositório, em local restrito indicado na ficha; (5) revisar independentemente o material e só
versionar frame anonimizado e ficha resumida, sem login, nome, IP, porta ou chat real, conforme
`wyd-protocol/docs/goldens/como-capturar.md` §2 e §6–§7. Anonimizar não substitui a prevenção da
captura de sessão real nem autoriza tráfego de produção.

**Próximo passo:** o dono de `wyd-ops` e a frente de pesquisa recomendam uma alternativa; `wyd-client`,
`wyd-protocol` e QA revisam as fronteiras. Até haver aprovação explícita, `LCLI-04` permanece sem
opção escolhida (e nenhuma ferramenta ou pasta deve ser criada por causa desta proposta).

## 3. Medições herdadas (não são lacunas deste registro)

| ID de origem | O que falta medir | Estado lá |
|---|---|---|
| `LPRT-03` | origem da `KeyWord` no handshake | aberta (`wyd-protocol/lacunas.md`) |
| `LPRT-06` | encoding e terminador de string | aberta (idem) |
| `LPRT-09` | build-alvo e compatibilidade | aberta (idem) |

Estas três fecham no `wyd-protocol`; o `wyd-client` fornece a captura. Duplicar o registro aqui
criaria dois donos para a mesma lacuna.
