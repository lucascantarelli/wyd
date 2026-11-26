# Tarefa de medição — `LNET-01` e `LBOT-03`

> Pergunta: "qual é a tarefa concreta, com dono, método e critério de aceite, que fecha as medições pendentes de `LNET-01` e `LBOT-03`?"
> Público: QA, `wyd-ops`, `wyd-net`, `wyd-bots`, arquitetura e agentes
> Dono: QA (execução) + `wyd-ops` (CI e telemetria) + arquitetura (revisão e congelamento)
> Base: `wyd-net/lacunas.md` §2; `wyd-bots/lacunas.md` §2; `wyd-net/docs/proposta-decisoes-net.md` §4; `wyd-bots/docs/proposta-decisoes-bots.md` §4; `wyd-1.0.0.md` §7.5 e §7.6; `AGENTS.md` §5, §8 e §9
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Planejamento de medição; nenhum código, workflow, execução, número ou fechamento de lacuna foi produzido por esta página.

## 1. Resultado esperado

Produzir **uma execução válida e reproduzível** no CI efêmero que entregue, no mesmo relatório:

- os valores de `LNET-01`: balde por conexão, porte por IP, teto total por shard, timeout de handshake e timeout ocioso;
- o limiar de `LBOT-03`: degrau de saturação, P50/P99 de ida e volta, taxa de sucesso e recusas por motivo;
- evidência anexada e congelada com ambiente, data, commit e perfil de teste.

A tarefa **não** escolhe os números por estimativa. Ela mede, registra e só então propõe o congelamento ao dono.

## 2. Dono e papéis

| Papel | Responsabilidade |
|---|---|
| QA | escreve o cenário, executa a escada de carga, valida o relatório e classifica a saturação |
| `wyd-ops` | fornece o job de CI efêmero, Postgres descartável, retenção do artefato e coleta de `/metrics` |
| `wyd-net` | expõe os contadores de conexão, recusa tipada, timeout e latência usados na medição |
| `wyd-bots` | executa o cliente de carga e entrega o relatório de `LBOT-04` no ponto de falha |
| arquitetura | revisa método, ambiente e resultado; propõe o congelamento ao dono |
| dono do sistema | aprova o congelamento e autoriza o fechamento de `LNET-01` e `LBOT-03` |

Sem os quatro primeiros papéis, a execução é **exploratória** e não fecha lacuna.

## 3. Pré-requisitos

1. A1 autorizada e captura concluída (`LPRT-03`/`LPRT-06`), porque o cenário usa o mesmo fio do cliente.
2. Fatia do `wyd-server` disponível no CI efêmero, com Postgres e servidor descartáveis por execução.
3. Seed declarativo de `Q-BOT-2` em `crates/wyd-db/seeds/dev.sql` aplicado no banco recriado.
4. Contadores de `LNET-04` expostos em `/metrics`, sem payload ou PII.
5. Job de CI de `Q-BOT-1` com runner hospedado, sem self-hosted em PR (`AGENTS.md` §9).
6. Perfil de teste declarado: número de origens, conexões por origem e duração de cada degrau.

Se qualquer pré-requisito faltar, a tarefa fica **bloqueada**; não se substitui por medição local ou número provisório.

## 4. Método

### 4.1 Preparação

- Criar um banco novo por execução e aplicar migrations + `seeds/dev.sql`.
- Subir um servidor descartável com o mesmo build do commit medido.
- Declarar no relatório: commit, imagem, runner, CPU/memória, sistema operacional, versão do Postgres, número de origens e duração.
- Separar as origens por IP ou declarar explicitamente que o limite por IP pode mascarar a saturação do shard.
- Não usar produção, homologação compartilhada ou banco com dado de jogador.

### 4.2 Escada de carga

Subir em degraus, mantendo cada degrau estável antes do próximo. A escada inicial é **proposta**, não limiar:

| Degrau | Conexões simultâneas | Duração | Objetivo |
|---:|---:|---:|---|
| 1 | 1 | 60 s | linha de base e handshake |
| 2 | 10 | 60 s | concorrência baixa |
| 3 | 25 | 60 s | concorrência média |
| 4 | 50 | 60 s | pressão moderada |
| 5 | 100 | 60 s | pressão alta |
| 6 | 200 | 60 s | saturação provável |

A escada pode ser ajustada ao runner, mas o ajuste e o motivo entram no relatório. Não se pula degrau nem se escolhe o degrau vencedor depois de ver o resultado.

### 4.3 O que medir

| Métrica | Origem | Uso |
|---|---|---|
| conexões aceitas, recusadas e fechadas | `/metrics` do `wyd-net` | limite por conexão, IP e shard |
| recusas por motivo (`RateLimited`, `TooManyPerIp`, `TooManyTotal`, timeout) | contador do `wyd-net` | distingue abuso de saturação |
| P50 e P99 de ida e volta | cliente de carga | limiar de `LBOT-03` |
| taxa de sucesso por degrau | cliente de carga | limiar de `LBOT-03` |
| timeout de handshake e ociosidade | relógio do servidor | valores de `LNET-01` |
| CPU, memória e file descriptors | runner/`/metrics` | explicação da saturação |
| erros de codec e desconexões | log de metadados | separa falha de carga de falha de protocolo |

Latência de ida e volta é medida no cliente; latência interna é medida no servidor. As duas não se misturam no mesmo P99.

### 4.4 Critério de saturação

Saturação é o **primeiro degrau** em que uma das condições ocorre:

- as recusas deixam de ser zero; ou
- o P99 cresce e não volta ao patamar do degrau anterior após a janela de estabilização.

O degrau anterior é o último ponto saudável. O valor congelado é o par **(último ponto saudável, primeiro ponto de saturação)**, não um número isolado.

### 4.5 Repetição

Executar a escada **duas vezes** no mesmo ambiente e no mesmo commit. Se o ponto de saturação mudar de degrau, repetir uma terceira vez e registrar a variância. O relatório declara qual execução foi usada e por quê.

### 4.6 Execução de prontidão (2026-11-25)

Antes de tentar a carga, foi executada uma medição de prontidão no repositório local. Ela **não** produz
latência nem conexões; produz a evidência de que o runtime necessário não existe.

| Comando | Resultado medido |
|---|---|
| `find crates -maxdepth 2 -type f` | 9 diretórios; 1 `Cargo.toml` (`wyd-db`), 1 `Cargo.lock` (`wyd-db`), 9 `.gitkeep` |
| `test -f crates/wyd-net/Cargo.toml` | **não** |
| `test -f crates/wyd-server/Cargo.toml` | **não** |
| `test -f crates/wyd-bots/Cargo.toml` | **não** |
| `test -f crates/wyd-protocol/Cargo.toml` | **não** |
| `test -f Cargo.toml` (raiz) | **não** |
| `find crates -path '*/tests/*' -type f` | **1** arquivo (do `wyd-db`) |
| `ss -ltn` / `netstat -ltn` | **0** listeners do projeto |

**Conclusão medida:** não há servidor executável, cliente de carga, workspace ou cenário; portanto
`LNET-01` e `LBOT-03` **não podem** ser fechadas com P50/P99, conexões ou recusas. Nenhum número foi
inventado. A execução fica bloqueada até a fatia do `wyd-server` e o job de `Q-BOT-1` existirem; a
pergunta está consolidada em `../lacunas.md` §4, item 7.

## 5. Critérios de aceite

A tarefa só é aceita quando todos os itens forem verdadeiros:

1. **Ambiente declarado:** commit, imagem, runner, CPU/memória, Postgres, origens e duração estão no relatório.
2. **Duas execuções:** a escada rodou duas vezes; a variância e a execução escolhida estão registradas.
3. **Métricas completas:** P50, P99, sucesso, recusas por motivo, timeouts, CPU, memória e file descriptors existem por degrau.
4. **Saturação classificada:** último ponto saudável e primeiro ponto de saturação estão explícitos, com a regra de §4.4.
5. **`LNET-01` mensurado:** balde por conexão, porte por IP, teto total por shard, timeout de handshake e timeout ocioso têm valor medido e unidade.
6. **`LBOT-03` mensurado:** limiar de carga tem degrau, P50/P99, sucesso e recusas; nenhum número foi inventado.
7. **Artefato seguro:** relatório e captura não contêm payload, credencial, nome de personagem, e-mail ou dado de jogador (`AGENTS.md` §5, item 7).
8. **Rastreável:** o relatório aponta para o artifact do CI, o commit e a data; o artifact é retido pelo período definido em `wyd-ops`.
9. **Revisão:** arquitetura revisou o método e o resultado; o dono aprovou o congelamento.
10. **Fechamento:** `LNET-01` e `LBOT-03` são atualizadas em `lacunas.md` com data, valores, ambiente e link do relatório; o gate do sistema passa.

## 6. Entregáveis

| Entregável | Onde | Dono |
|---|---|---|
| cenário de carga | `crates/wyd-bots` (implementação futura) | QA |
| job de CI efêmero | `.github/workflows/` (implementação futura) | `wyd-ops` |
| contadores de limite/timeout | `crates/wyd-net` (implementação futura) | `wyd-net` |
| relatório de medição | artifact do CI + página dona | QA + arquitetura |
| valores congelados | `wyd-net/lacunas.md` e `wyd-bots/lacunas.md` | dono do sistema |
| evidência de gate | `docker compose --profile gate run --rm gate` | `wyd-ops` |

## 7. Fora de escopo

- Implementar o job, o servidor, o bot, o seed ou os contadores nesta tarefa.
- Definir limite de produção antes da medição.
- Medir produção, homologação compartilhada ou máquina local como gate.
- Fechar `LPRT-03`, `LPRT-06`, `LCLI-02` ou qualquer lacuna fora de `LNET-01`/`LBOT-03`.
- Criar ADR: o congelamento é decisão do dono e pode virar ADR se mudar fronteira ou operação.

## 8. Riscos e mitigação

| Risco | Mitigação |
|---|---|
| runner saturado antes do servidor | registrar CPU/memória e separar limite de infraestrutura de comportamento do jogo |
| limite por IP mascarar o shard | usar origens distintas ou declarar o perfil |
| P99 instável | duas execuções e janela de estabilização |
| artefato com PII | scrubbing e retenção fora do git |
| número usado como gate antes da aprovação | estado `exploratório` até o dono congelar |

## 9. Cruzamento com auditoria e especificação

| Referência | Relação |
|---|---|
| `wyd-ops/auditoria-de-organizacao.md` §2–§3 | `wyd-net` e `wyd-bots` organizados; `wyd-ops` mantém fronteira/contratos parciais; esta tarefa não fecha organização por si só |
| `wyd-net/lacunas.md` §2 | `LNET-01` aberta como medição |
| `wyd-bots/lacunas.md` §2 | `LBOT-03` aberta como medição |
| `wyd-net/docs/proposta-decisoes-net.md` §4 | política de `Q-NET-1` = A |
| `wyd-bots/docs/proposta-decisoes-bots.md` §4 | política de `Q-BOT-3` = A |
| `wyd-1.0.0.md` §7.5 e §7.6 | ligação e estado de teste/medição |
| `AGENTS.md` §5, §8 e §9 | evidência, gate medido e runner hospedado |

## 10. Skills que o agente deve carregar

- `planejar-sistema` — antes de escrever cenário, contrato ou critério.
- `auditar-planejamento` — antes de aceitar o relatório e fechar as lacunas.

## 11. Decisões abertas

1. **Degraus iniciais** — a escada de §4.2 é proposta.
   - Opções: A) manter 1/10/25/50/100/200; B) ajustar ao runner; C) usar degraus menores.
   - Recomendação: A como ponto de partida, com ajuste registrado no relatório.
2. **Retenção do artefato** — ainda não definida.
   - Opções: A) 30 dias; B) 90 dias; C) período do `wyd-ops`.
   - Recomendação: C, citando a política dona.
3. **Quem congela** — QA mede; arquitetura revisa; o dono aprova.
   - Recomendação: manter essa separação para não transformar medição em autoaprovação.