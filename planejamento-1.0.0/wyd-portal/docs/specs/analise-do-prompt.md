> Pergunta: "o prompt do portal pede o quê, o que dele já está decidido e o que ainda é só intenção?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: produto
> Base: `wyd-portal-prompt.md` (148 linhas, medido em 2026-11-20); `M21`; `AGENTS.md` §5
> Revisado em: 2026-11-20

**Estado:** aceito (2026-11-20)

O `wyd-portal-prompt.md` é **orientação de produto** (`H`), não fonte de verdade: ele descreve o
portal desejado em linguagem de marketing e pede execução em fases. Esta página o lê item a item,
separa o que já foi decidido do que continua intenção, e diz o que o planejamento faz com cada parte.

## 1. O que o prompt pede, por seção

| Seção do prompt | Pede | Vira o quê neste plano |
|---|---|---|
| §1–§2 (persona e missão) | portal AAA com alma do WYD2 antigo e profundidade de Wowhead | direção de produto; **não** é requisito verificável sozinho |
| §3 (descoberta) | auditar Next.js, extrair ativos de `client/`, pesquisar referências | diagnóstico de implementação, não de planejamento; ver §3 abaixo |
| §4 (design system) | tema claro/escuro, tipografia, motion, imagens geradas | `RNF-POR-003` (contraste/foco) e decisão de UI na fatia de implementação |
| §5.1–§5.3 (público) | home, notícias, rankings, WYDpedia, atlas | `RF-POR-005` a `RF-POR-009` e rotas 1–13 |
| §6.1–§6.3 (admin) | NOC, jogadores, economia, CMS, launcher, logs e monitoramento | `RF-POR-010` a `RF-POR-018` e rotas 23–43; as seis telas de monitoramento do legado têm paridade em `monitoramento.md` §7 |
| §7 (padrões) | TypeScript estrito, `zod`, performance, SEO | `RNF-POR-001` a `RNF-POR-006` |
| §8 (workflow) | cinco etapas de execução | `docs/roadmap/roadmap.md`, sem issue criada |
| §9 (escopo 2026-11-20) | portal público **e** admin inteiro, admin configura o servidor | decisão `M21`; base deste plano |

## 2. O que já está decidido (e o plano só cita)

| Assunto | Decisão | Dono |
|---|---|---|
| Portal inclui admin e configuração do servidor | `M21`, 2026-11-20 | produto |
| Papéis de portal `member`/`moderator`/`admin` | `0003` | `wyd-db` |
| Sessão revogável e token de cerimônia | `0004`, `A16`, `M4` | `wyd-db` |
| Configuração com revisão e auditoria | §5.9 de `ddl-confirmado.md`, `M18-A` | `wyd-db` |
| Cash e loja no `portal`; economia de jogo em `game` | `A19` | produto + arquitetura |
| Idempotência por chave única | `M17` | arquitetura |
| Escrita em `game` e ação no processo só pelo `wyd-admin` | `LAC-POR-01`, 2026-11-20 | arquitetura |
| CMS em `portal` (`news`, `banner`, `guide`) e launcher em `portal.launcher_release` | `LAC-POR-02`/`LAC-POR-03`, 2026-11-20 | produto + arquitetura |
| Rankings com `LIMIT` e cache de 60 s no BFF, sem tabela derivada | `LAC-POR-04`, 2026-11-20 | arquitetura |
| Segundo fator de staff por TOTP em `portal.person_authenticator` | `LAC-POR-05`, `A20`, 2026-11-20 | produto + segurança |
| Telemetria de canal por fonte externa (Prometheus do `wyd-ops`), sem tabela | `LAC-POR-06`, 2026-11-20 | `wyd-ops` + arquitetura |
| Monitoramento completo (métrica, log, painel e launcher) por fonte externa, sem tabela | `LAC-POR-06` reconfirmada, 2026-11-20 | `wyd-ops` + arquitetura |

## 3. O que o prompt pede e **não** entra no planejamento

| Pedido | Por que fica fora | Onde entra |
|---|---|---|
| instalar Tailwind, Radix, framer-motion, recharts, cmdk, tanstack-table | escolha de biblioteca é de implementação; o plano fixa requisito, não pacote | fatia de UI, com ADR se mudar fronteira |
| extrair e converter ativos de `client/` | `client/` não é fonte de planejamento e o material é `H`; extração é ferramenta, não schema | `wyd-client`/`wyd-ops`, quando autorizado |
| gerar imagens e banners | produção de ativo, não decisão de sistema | fatia de design |
| “popule com dados autênticos” | dado real de jogador não entra em planejamento; seed é do `wyd-db` | `0016`–`0027` |
| fases 1–5 do workflow | ordem de execução; vira roadmap, não promessa de prazo | `roadmap.md` |

**Nada disso foi descartado por gosto:** o plano só não transforma pedido de ferramenta em contrato
sem dono.

## 4. Tensões reais que o prompt cria

| Tensão | Efeito | Tratamento |
|---|---|---|
| Admin “configura o servidor” × `wyd_portal` sem `USAGE` em `game` | escrita direta em `game` é proibida por permissão | decidido: ação e escrita em `game` vão pelo `wyd-admin` (HTTP/JSON) |
| CMS completo × 92 tabelas do `wyd-db` | não há tabela de notícia, banner, guia ou launcher | decidido: 4 tabelas novas em `portal`, lote `0030` aprovado em 2026-11-20 (92 → 96) |
| “tempo real” no NOC × schema | não há série de CPU/RAM/latência no banco | decidido: fonte externa (Prometheus, Loki e Grafana do `wyd-ops`); as seis telas consultam por API e mostram “não medido” quando a fonte falha (`LAC-POR-06`, `monitoramento.md`) |
| gráficos ricos × escopo do portal | desenhar série no portal duplicaria Prometheus/Loki | decidido: o portal resume e embute o painel do Grafana; gráfico próprio fica fora (`monitoramento.md` §7/§9) |
| atlas com pins × conteúdo | pins vêm de `0019`/`0024`, ainda sem DDL | requisito existe, implementação espera o conteúdo |
| ranking “ao vivo” × custo | consulta pesada em tabela quente | `LAC-POR-04`: cache/`ISR` com instante declarado |

## 5. O que o prompt acerta e o plano mantém

1. **Duas superfícies, um login** — público e admin na mesma pessoa, com papéis distintos.
2. **Admin como operação, não vitrine** — cards agregados levam a detalhe com filtro aplicado.
3. **Trilha de auditoria em toda ação** — já é decisão do `0002`; o plano exige na rota.
4. **WYDpedia como catálogo relacional** — casa com `content` (`0013`–`0019`).
5. **Tema claro e escuro com contraste** — virou `RNF-POR-003`, verificável.

## 6. O que o prompt não resolve e este plano deixa explícito

| Falta | Consequência | Dono |
|---|---|---|
| forma do contrato HTTP do `wyd-admin` | ação existe na direção, falta payload e códigos | planejamento do `wyd-admin` |
| lote das tabelas de CMS/launcher | três rotas sem tabela aplicada | `wyd-db` |
| fonte e cache de ranking | número pode mentir sobre “agora” | `LAC-POR-04` |
| DDL de `portal.person_authenticator` | fechado no lote `0031` (`ddl-confirmado.md` §5.20); implementação começa com a série aplicada | `wyd-db` |
| contrato de métricas, logs e painéis | fontes e nomes de variável estão em `monitoramento.md` §4/§6; valor, intervalo e retenção definitiva ainda sem forma publicada | `wyd-ops` |

## 7. Como este plano usa o prompt daqui para frente

- Requisito novo citado pelo prompt entra em `requisitos.md` com ID, não em comentário de código.
- Tela nova entra em `rotas-e-permissoes.md` antes de existir rota.
- Pedido de ferramenta ou de ativo não vira requisito: vira tarefa da fatia, com dono.

*Página de planejamento. Nenhuma biblioteca foi instalada e nenhum ativo foi extraído.*
