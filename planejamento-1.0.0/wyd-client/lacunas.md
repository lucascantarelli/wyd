# `wyd-client` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-23).** Único lugar onde uma lacuna do `wyd-client` tem dono,
> impacto e estado. `LCLI-04` foi decidida pelo dono em 2026-11-23 e `LCLI-01` fechou com `LPRT-09`
> (`Q-PRT-1`) na mesma data; pendências de execução seguem listadas abaixo.

## 0. Como ler

Estados: **aberta**, **proposta**, **fechada** (com evidência). `arquitetura` propõe; o dono do
sistema aprova. Lacuna de outro sistema que este consome é citada pelo ID de origem.

## 1. Situação em uma linha

| Grupo | Lacunas | Abertas | Propostas | Fechadas |
|---|---:|---:|---:|---:|
| Medição do cliente | 2 | 1 | 0 | 1 |
| Distribuição | 2 | 1 | 1 | 0 |
| Processo | 1 | 0 | 0 | 1 |
| **Total** | **5** | **2** | **1** | **2** |

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `LCLI-01` | Qual build do cliente é o alvo, e qual é a identidade verificável dele (versão declarada e hash do binário)? | é `LPRT-09` vista do lado do cliente: sem build-alvo, offset e opcode ficam ambíguos | frente `wyd-client` + pesquisa | fechada (2026-11-23: `Q-PRT-1`/`LPRT-09` fixou o cliente 7662 com versão e SHA-256 completos; identidade em `registro-do-cliente.md` §4) |
| `LCLI-02` | O que exatamente o patch entrega (`strdef.bin`, assets, executável) e com que versionamento? | é a entrada do manifesto do `wyd-launcher`; sem lista, o patch vira cópia da árvore inteira | `wyd-client` + `wyd-launcher` | aberta |
| `LCLI-03` | Onde mora o registro de captura — ficha, data e quem aprova a segunda leitura? | o processo de `LPRT-10` e o local já estão fixados; falta o nome do QA que assina a segunda leitura | QA + arquitetura | aberta |
| `LCLI-04` | Qual ferramenta faz a captura, onde ela mora, quem responde por ela e como impede gravar dado de jogador? | captura sem ferramenta e fronteira aprovadas vira script solto ou grava dado pessoal; `AGENTS.md` §5.6 proíbe executável em `planejamento-1.0.0/` | dono de `wyd-client`; `wyd-protocol` valida o golden; QA revisa | fechada (opção A em 2026-11-23; D3–D6 decididos; implementação pendente) |
| `LCLI-05` | Que conteúdo do cliente e marcas pode ser copiado, modificado, publicado ou redistribuído, e que evidência autoriza cada uso? | presença de arquivo local não comprova direito; sem escopo e autorização verificáveis, patch/release/site podem incluir material não liberado | dono do produto + responsável por direitos (a nomear), com revisão jurídica; `wyd-client` inventaria; `wyd-launcher`/`wyd-ops` aplicam | proposta (`docs/specs/propriedade-intelectual-e-distribuicao.md`, `Q-IP-1`–`Q-IP-6`) |

### 2.1 `LCLI-04` — decisão da ferramenta e pendências de execução (2026-11-23)

**Decisão do dono: opção A — ferramenta específica do cliente em `wyd-client/tools/`.** A pasta ainda
não existe; o caminho foi aprovado para planejamento e futura implementação, não foi criado código.
A opção B (utilitário compartilhado em `wyd-ops/`) e a C (ferramenta externa sem código versionado)
não foram escolhidas para esta ferramenta.

**Escopo estreito aprovado:** orquestrar a captura específica do cliente e normalizar sua evidência:
configurar o cliente para o `ECAP` ([spec](../wyd-server/docs/specs/endpoint-de-captura.md)), iniciar/parar a sessão,
recortar o frame necessário e produzir a ficha de captura. Não implementa proxy, servidor ou protocolo;
esses componentes permanecem em seus sistemas donos. A ferramenta só captura ambiente controlado,
segue `LPRT-10` e nunca grava bruto com dados de jogador no repositório.

| Opção considerada (registro da comparação) | Onde moraria | Quem aprova / responde | Como evitaria dado de jogador | Trade-offs |
|---|---|---|---|---|
| **A — ferramenta específica do cliente** | `wyd-client/tools/` (pasta ainda inexistente; artefato executável na raiz do projeto, não em `planejamento-1.0.0/`) | dono de `wyd-client` aprova localização e manutenção; `wyd-protocol` valida que a saída serve ao processo de golden; QA faz a segunda leitura exigida por `LPRT-10` | executar só com conta/personagem de teste sintéticos, contra servidor local ou proxy controlado, sem sessão de produção ou tráfego de terceiros; capturar somente o cenário necessário; se surgir dado real, interromper, não persistir nem versionar o bruto e descartar a captura. Bruto de sessão sintética, quando necessário para a segunda leitura, fica restrito fora do git; no repositório entra apenas frame anonimizado e ficha resumida | alinhada com a responsabilidade de captura do `wyd-client` e pode evoluir com o cliente; cria ferramenta específica e manutenção no sistema cliente |
| **B — utilitário de operação compartilhado** | `wyd-ops/` (subpasta ainda inexistente; alternativa é o dono implementar ferramenta operacional reutilizável) | dono de `wyd-ops` aprova localização e operação; `wyd-client` define o cenário de captura; `wyd-protocol` valida saída e QA faz a segunda leitura | mesmas barreiras da A: exclusivamente sessão sintética em ambiente controlado, sem produção nem terceiros; parar e descartar se aparecer dado real; bruto de teste restrito fora do git, e somente artefato anonimizado revisado pode ser versionado | centraliza dependências e execução operacional; pode misturar captura específica do cliente com infraestrutura comum e tornar `wyd-client` dependente de `wyd-ops` |
| **C — ferramenta externa, sem código no repositório** | instalada apenas na estação/ambiente de pesquisa, fora da árvore versionada | pesquisa registra a ferramenta e versão; dono de `wyd-client` aprova o uso no cenário; `wyd-protocol` e QA conferem evidência conforme `LPRT-10` | configurar a ferramenta para uma sessão sintética em ambiente controlado, sem produção/terceiros; interromper e descartar se aparecer dado real; bruto de teste restrito fora do repositório, sem versioná-lo; revisar e anonimizar antes de produzir o frame versionado | evita criar/manter executável próprio e permite começar com ferramenta disponível; repetibilidade depende de versão/configuração externa, e o projeto não controla seu ciclo de vida |

**Decisões de implementação (D3–D6; todas decididas em 2026-11-23):**

| ID | O que falta decidir | Dono / aprovação |
|---|---|---|
| D3 | **decidido em 2026-11-23:** proxy gravador sobre servidor local controlado; a ferramenta em `wyd-client/tools/` configura e recorta, e o endpoint é operado por `wyd-server`, com conta sintética e sem produção | `wyd-client` + `wyd-protocol`; dono de `wyd-protocol` aprova o contrato de captura. Não bloqueia mais `LPRT-03` nem `LPRT-06` (`LPRT-09` está decidido); falta implementar e operar o endpoint |
| D4 | **decidido em 2026-11-23: CLI em Rust** em `wyd-client/tools/`, binário próprio, sem runtime Python; exige Cargo próprio ou workspace; não implementa o proxy | dono de `wyd-client`, com arquitetura |
| D5 | **decidido em 2026-11-23:** bruto fora do repositório, acesso restrito e descarte **30 dias após a segunda leitura**; só frame anonimizado e ficha entram no git | QA propõe; dono de `wyd-client` aprova |
| D6 | **decidido em 2026-11-23:** segunda leitura por **QA ou dono do `wyd-protocol`, nunca quem capturou**; a pessoa é nomeada em `LCLI-03` e assina a ficha | QA + arquitetura; registrar junto de `LCLI-03` |

**O desenho técnico do ECAP e o peer de A1 estão fora de D3–D6.** D3 continua sendo a decisão de
princípio do proxy gravador local, D4 a CLI Rust, D5 o bruto fora do repositório e D6 a segunda leitura.
O endpoint local, o listener e a gravação estão em
[`endpoint-de-captura.md`](../wyd-server/docs/specs/endpoint-de-captura.md); o peer que completa o
handshake foi decidido em `Q-SRV-5`/`LSRV-05` (C, com quarentena), mas a condição foi reprovada na
medição de 2026-11-23: A1 passa a depender da opção A.

A localização e o dono de `wyd-client/tools/` estão decididos; D3–D6 também estão decididos. O
próximo trabalho é implementar o `ECAP` operado por `wyd-server` e a CLI Rust em
`wyd-client/tools/`; depois, executar a captura A1. `LCLI-03` continua aberta para o **nome** da pessoa:
em 2026-11-23 o dono escolheu que **QA designado** assina a segunda leitura e o dono do `wyd-protocol`
é suplente; o nome não foi informado. `wyd-protocol` valida a conformidade do frame com `LPRT-10`; uma
pessoa diferente de quem capturou faz a segunda leitura.

**Salvaguardas adotadas para A:** (1) usar apenas conta e personagem de teste, com valores sintéticos;
nunca abrir sessão de produção nem capturar tráfego de terceiros; (2) usar servidor local ou proxy sob
controle da equipe e limitar a captura ao cenário de uma ação, com chat vazio; (3) se surgir dado real,
interromper, não persistir nem versionar o bruto e descartá-lo; se não for possível garantir ambiente
sintético, não capturar; (4) quando necessário à segunda leitura de `LPRT-10`, manter o bruto
exclusivamente fora do repositório, em local restrito a decidir em D5; (5) revisar independentemente
e só versionar frame anonimizado e ficha resumida, sem login, nome, IP, porta ou chat real, conforme
`wyd-protocol/docs/goldens/como-capturar.md` §2 e §6–§7. Anonimizar não substitui prevenção de
captura de sessão real nem autoriza tráfego de produção.

## 3. Medições herdadas (não são lacunas deste registro)

| ID de origem | O que falta medir | Estado lá |
|---|---|---|
| `LPRT-03` | origem da `KeyWord` no handshake | aberta (`wyd-protocol/lacunas.md`) |
| `LPRT-06` | encoding e terminador de string | aberta (idem) |
| `LPRT-09` | build-alvo e compatibilidade | fechada em 2026-11-23 (`Q-PRT-1`: 7662 + SHA-256); a comparação entre builds segue não medida |

`LPRT-03` e `LPRT-06` fecham no `wyd-protocol`, com a captura que o `wyd-client` fornece; `LPRT-09` fechou
por decisão de alvo. Duplicar o registro aqui criaria dois donos para a mesma lacuna.
