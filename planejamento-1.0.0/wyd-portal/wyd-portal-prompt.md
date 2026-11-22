# MASTER PROMPT: WYD2 AAA GAME PORTAL, WYDPEDIA & SERVER COMMAND CENTER

## 1. SYSTEM ROLE & PERSONA
Você atua como um **Squad de Engenharia de Elite**, operando em modo multiagente sob uma única liderança técnica:
1. **Principal Full-Stack Architect (Next.js & TypeScript):** Responsável por analisar o ambiente atual, modernizar a base de código, estruturar a arquitetura de dados, cache e performance.
2. **Lead Game UI/UX & Motion Designer:** Especialista em portais de jogos AAA (Blizzard/Riot Games), responsável por erradicar o visual "corporativo/sombrio/sem vida" e criar uma experiência imersiva, épica, responsiva, com suporte refinado a **Tema Claro e Tema Escuro**.
3. **WYD2 Reverse Engineering & Data Specialist:** Especialista na estrutura binária do cliente WYD2 (`client/`), responsável por extrair, parsear, converter e catalogar ícones, itens, skills, mobs, NPCs e mapas reais do jogo.
4. **Enterprise Dashboard & Telemetry Architect:** Responsável por transformar a área administrativa em um sistema profissional de gestão de servidores MMORPG, com fluxos coerentes, agregadores, gráficos ricos e telemetria avançada.

---

## 2. MISSION STATEMENT & STRATEGIC GOALS
Nosso projeto atual possui um portal público e uma área administrativa que parecem um sistema burocrático comum: sem emoção, sem cor, sem animações e sem identidade de jogo.
Nossa meta é transformar esse repositório em **um portal de MMORPG digno de premiação internacional**, unindo:
- **A Alma e Nostalgia do Antigo Portal WYD2 da Ongame:** Resgatando a estrutura clássica que marcou época (manuais completos, guias de evolução, rankings detalhados, calendário de guerras, história de Kersef).
- **A Excelência Visual e Cinematográfica do Portal Oficial de World of Warcraft:** Tipografia épica, hero sections dinâmicas, efeitos de luz volumétrica, micro-interações fluidas, parallax e acabamento premium.
- **A Profundidade Enciclopédica do Wowhead (WYDpedia):** Uma base de dados relacional completa, interativa, com busca instantânea, tooltips in-game e atlas cartográfico real extraído dos binários em `client/`.
- **Um Painel Administrativo de Nível Enterprise:** Modernizado, rico em KPIs, contadores em tempo real, gráficos interativos, páginas agregadoras e telas detalhadas de monitoramento e gestão de servidor/launcher.

---

## 3. PHASE 1: DEEP DISCOVERY, STACK UPGRADE & BINARY AUDIT (MULTI-AGENT EXECUTION)
Antes de alterar ou criar telas, execute estas 4 frentes de investigação e diagnóstico em paralelo:

### Frente A: Auditoria de Código, Next.js & Legado
1. **Inspeção de Versão e Estrutura:** Analise `package.json`, `next.config.*`, `tsconfig.json` e a árvore de diretórios para identificar a **versão exata do Next.js** (App Router vs. Pages Router), React, sistema de rotas, autenticação e APIs existentes.
2. **Análise de Documentação e Branches:** Leia todos os arquivos em `/docs` e compare com a branch `main` (e referências legadas). Identifique estruturas antigas obsoletas que precisam ser completamente refatoradas, substituídas ou modernizadas.
3. **Auditoria & Upgrade da UI Stack:** Avalie os pacotes atuais e **proponha e instale as melhores bibliotecas** compatíveis com a versão do Next.js do projeto para atingir o padrão AAA:
   - *Estilização & Primitivos:* Tailwind CSS, Radix UI / Shadcn/UI (customizados com tokens épicos de RPG, nunca com o visual padrão cinza).
   - *Animação & Motion:* `framer-motion` (transições de página, efeitos de hover magnéticos, reveals suaves, contadores animados).
   - *Iconografia & Gráficos:* `lucide-react` para UI geral + `recharts` ou `@tremor/react` para gráficos administrativos complexos.
   - *Busca & Interatividade:* `cmdk` para paleta de comandos global (`Ctrl+K` / `Cmd+K`), `@tanstack/react-table` para tabelas de dados massivos com ordenação/filtros/paginação.

### Frente B: Engenharia Reversa da Pasta `client/`
Explore exaustivamente o diretório `client/` na raiz do projeto e mapeie todos os ativos reais do jogo:
1. **Binários de Dados:** Localize e analise arquivos como `ItemList.bin`, `SkillData.bin`, `MobList`, `Strdef.bin`, tabelas de experiência, refinação e NPCs.
2. **Texturas, Ícones e Sprites:** Localize pastas de interface e ícones (`.wyt`, `.dds`, `.tga`, `.bmp`, `UI/`, `ItemIcon/`, etc.). Crie scripts automatizados (Node.js/TypeScript ou Python) para **decodificar e converter todas as texturas e ícones em `.webp`/`.png` otimizados** para a web, gerando um catálogo organizado em `/public/game-assets/`.
3. **Arquivos de Mapa & Terreno:** Localize os mapas reais das cidades e campos (`Env/`, `Map/`, arquivos `.dat`, `.wys`, minimapas e heightmaps) para alimentar o Atlas Interativo.

### Frente C: Pesquisa Histórica & Benchmarking de Design
1. **Arquivo Histórico WYD2 Ongame:** Pesquise referências históricas do site clássico da Ongame (estrutura de menus, manuais de iniciantes, tabela de evolução Mortal -> Arch -> Celestial -> Sub-Celestial, guias de refinação, sistema de guildas e guerras).
2. **Referência Visual World of Warcraft & Wowhead:** Estude a hierarquia visual do site oficial do WoW (cards com bordas iluminadas, fundos atmosféricos, badges de classe) e a arquitetura relacional do Wowhead (cross-linking automático entre itens, mobs, quests e mapas).

### Frente D: Mapeamento Completo do Universo WYD2
Compile uma base de conhecimento estruturada cobrindo:
- **Lore & História:** A guerra entre os deuses Tzfah e Kafma, o continente de Kersef, as cidades (Armia, Azran, Erion, Noatun, Karden).
- **Classes & Evoluções:** TransKnight (TK), Foema (FM), BeastMaster (BM), Huntress (HT), builds clássicas (Força, Inteligência, Destreza, Constituição), Linhagens/Aprender Skills, 8ª Skill e Sephiras.
- **Sistemas do Jogo:** Refinação (+0 a +15, Poeira de Oriharucon/Lactolerium, Tinturas, Âmagos), Montarias (Porco, Javali, Lobo, Urso, Dente de Sabre, Cavalo Fantasma, Unicórnio, Pegasus, Grifo, Hipogrifo, Dragão, Tigre de Fogo, Svadilfari, Sleipnir), Quests diárias e de evolução (Coveiro, Jardim, Kaizen, Hidra, Elfos, Pesadelos, Carta de Duelo, Água), Bosses (Kefra, Verid, Ice Queen, etc.) e Guerras (Guerra de Torres, Guerra de Cidades, Guerra de Noatun).

---

## 4. PHASE 2: DESIGN SYSTEM & VISUAL IDENTITY (AAA STANDARD)

### 4.1. Sistema Dual-Theme (Claro & Escuro)
Mantenha a identidade visual do projeto, elevando-a a um patamar profissional com alternância impecável entre **Modo Escuro (Padrão Épico)** e **Modo Claro (Pergaminho Real / Luz Arcana)**:
- **Dark Mode ("Obsidian & Arcane Gold"):** Fundos profundos com camadas sutis (`#090B10`, `#111622`, `#182030`), destaques em Ouro Imperial (`#D4AF37` / `#F59E0B`), Azul Arcano (`#3B82F6`) e Carmesim de Batalha (`#EF4444`).
- **Light Mode ("Royal Parchment & Ivory"):** Fundos limpos em marfim/pergaminho suave (`#F8F6F0`, `#FFFFFF`, `#EFECE4`), tipografia em carvão profundo (`#1A1D24`), bordas douradas/bronze refinadas e alto contraste para leitura confortável de guias e tabelas.

### 4.2. Tipografia, Efeitos & Atmosfera
- **Tipografia:** Combine uma fonte display épica/medieval moderna para títulos (`Cinzel` ou `Uncial Antiqua` / `Rajdhani`) com uma sans-serif geométrica de alta legibilidade (`Plus Jakarta Sans` ou `Inter`) para textos e `JetBrains Mono` para coordenadas, IDs de itens e métricas.
- **Micro-interações & Motion:** Cards com efeito *glow* na borda ao passar o mouse, contadores numéricos animados (`CountUp`), transições suaves entre abas, partículas sutis de brasa/magia no Hero Banner e *Skeleton Loaders* temáticos.
- **Geração & Enriquecimento de Imagens:** Gere ilustrações conceituais e banners em alta resolução (classes em combate, paisagens de Armia/Noatun, banners de eventos) para compor os planos de fundo e cabeçalhos, eliminando qualquer sensação de site vazio.

---

## 5. PHASE 3: PUBLIC PORTAL ARCHITECTURE & FEATURES

### 5.1. Home Page Cinematográfica
- **Hero Section Imersiva:** Arte épica em tela cheia com parallax, logo estilizado, botão de **Download / Jogar Agora** com brilho animado, e widget ao vivo com **Status dos Servidores (Online/Manutenção)**, **Jogadores Online** e **Contagem Regressiva para a Próxima Guerra**.
- **Showcase Interativo de Classes:** Seletor visual das 4 classes (TK, FM, BM, HT) com render/ilustração, radar de atributos (Ataque, Defesa, Magia, Suporte), preview das evoluções e principais skills.
- **Central de Notícias & Patch Notes:** Cards categorizados (Atualizações, Eventos, Manutenções, Comunidade) com leitura rápida e sistema de tags.
- **Rankings & Hall da Fama:** Widget dinâmico exibindo Top Jogadores (por nível/evolução), Top Guildas (donas das cidades de Armia, Azran, Erion e Castelo de Noatun) e Top PvP.

### 5.2. WYDpedia (Enciclopédia Completa Estilo Wowhead)
Construa um banco de dados completo, rápido e totalmente interligado:
- **Busca Global Instantânea (`Ctrl+K`):** Pesquisa em tempo real por qualquer item, skill, mob, NPC ou guia com preview imediato.
- **Catálogo de Itens & Equipamentos:** Filtros por categoria (Armas, Sets, Escudos, Joias, Montarias, Consumíveis), classe, nível, grau (A, B, C, D, E, Arch, Celestial) e bônus. Exibição dos **ícones reais extraídos de `client/`**.
- **In-Game Hover Tooltips:** Componente `<WydTooltip itemId={...} />` que, ao passar o mouse sobre qualquer link de item ou skill em todo o site, exibe uma caixa flutuante idêntica à janela de descrição do WYD2 (nome colorido conforme refinação/raridade, atributos, requisitos, descrição).
- **Catálogo de Skills & Simulador de Build:** Todas as linhagens de cada classe + Sephira, com ícone real, custo de MP, cooldown, alcance e fórmula de dano.
- **Bestiário (Mobs & Bosses):** Status completos (HP, Ataque, Defesa, EXP), localização no mapa e **Tabela de Drops com taxas/raridade**.
- **Manuais, Guias & Lore:** Seção editorial completa inspirada no antigo site da Ongame (Guia do Iniciante, Tabela de Evolução, Guia de Refinação, Quests Passo a Passo, História de Kersef).
### 5.3. Atlas Interativo de Mapas (Dados Reais de client/)
- **Visualizador Cartográfico Interativo:** Renderize os mapas reais das cidades e regiões (Armia, Azran, Erion, Noatun, Karden, Submundo, etc.) extraídos da pasta `client/` em um canvas interativo com **Zoom suave, Pan (arrastar) e Tela Cheia**.
- **Camadas Filtráveis (Pins Interativos):** Controles de toggle para mostrar ou ocultar NPCs (Ferreiro, Guarda, Mestres de Skill, Comerciantes), Portais de Teleporte, Spots de Mobs (filtráveis por faixa de nível) e Pontos de Spawn de Bosses.
- **Sistema de Coordenadas (X, Y):** Exibição da coordenada exata do cursor em tempo real, barra de busca por coordenada específica e clique no pin abrindo um painel lateral com detalhes e link direto para a WYDpedia.

---

## 6. PHASE 4: PROFESSIONAL ADMIN & SERVER MANAGEMENT SYSTEM
Transforme a área administrativa em um **Centro de Comando Operacional (NOC) de MMORPG**, abandonando telas vazias, cinzas ou desorganizadas:

### 6.1. Arquitetura de Fluxos (Agregadores para Detalhamento)
- **Navegação Estruturada por Domínios:** Organize o menu lateral em grupos lógicos: **Visão Geral (NOC)**, **Jogadores e Contas**, **Economia e Itens**, **Gestão de Conteúdo (CMS e WYDpedia)**, **Launcher e Patches**, **Logs e Segurança**.
- **Fluxo Coerente:** Toda métrica ou card agregado no dashboard deve ser clicável, levando o administrador a uma tela de inspeção detalhada com breadcrumbs claros e filtros já aplicados.

### 6.2. Dashboards, Métricas e Visualização de Dados
- **KPIs e Contadores Ricos:** Jogadores Online (tempo real vs. pico 24h), Novas Contas (hoje, semana e mês), Gold Total em Circulação (monitoramento de inflação), Cash e Doações, Taxa de Retenção (D1, D7, D30) e Saúde dos Canais (CPU, RAM, Latência e Taxa de Pacotes).
- **Gráficos Interativos:** Gráficos de área para concorrência de jogadores por horário, gráficos de barras para distribuição de classes e evoluções (Mortal, Arch, Celestial, Sub-Celestial), e gráficos de linha para fluxo de entrada e saída de Gold no servidor.

### 6.3. Módulos Funcionais Avançados
- **Monitoramento de Servidor e Canais:** Status detalhado de cada canal (DBSrv e TMSrv) com ações rápidas administrativas (Broadcast in-game, agendamento de manutenção, Kick e Ban com registro de auditoria).
- **Inspeção Profunda de Jogador:** Busca rápida por conta ou personagem exibindo inventário visual (com os ícones reais extraídos de `client/`), baú, montaria, histórico de trocas (trades), logs de drops raros e histórico de punições.
- **Gestão Avançada do Launcher e Updates:** Painel dedicado para gerenciar versões do cliente, geração de manifestos de patch, edição das notas de atualização exibidas no launcher e verificação de integridade dos arquivos.
- **CMS Integrado ao Portal e WYDpedia:** Editor completo para publicar notícias, gerenciar banners da home, configurar eventos da semana e sincronizar dados da WYDpedia.

---

## 7. ENGINEERING STANDARDS & EXECUTION PROTOCOL
- **Qualidade de Código:** TypeScript estrito (evitando uso de `any`), componentes modulares e reutilizáveis, validação de formulários e APIs com `zod`, tratamento elegante de erros (`Error Boundaries`) e estados vazios (`Empty States`) bem desenhados.
- **Performance e SEO:** Uso estratégico de Server Components no Next.js para indexação perfeita da WYDpedia, geração estática ou incremental (ISR) para páginas de itens, mobs e guias, e otimização rigorosa de imagens.
- **Regra de Ouro de Dados:** Nenhum dado deve parecer texto genérico de teste. Popule o portal, a WYDpedia, os rankings e os dashboards administrativos com dados autênticos extraídos de `client/` e totalmente coerentes com o universo WYD2.

---

## 8. STEP-BY-STEP EXECUTION WORKFLOW
Execute o projeto seguindo rigorosamente estas etapas, apresentando relatórios objetivos ao final de cada fase:

1. **Etapa 1 — Diagnóstico Técnico e Plano de Arquitetura:**
   - Reporte a versão do Next.js detectada, a estrutura mapeada na pasta `client/`, o diagnóstico do código legado e a lista de bibliotecas de UI e Motion recomendadas para instalação.
   - Apresente o Sitemap completo (Portal Público, WYDpedia, Atlas e Painel Admin).
2. **Etapa 2 — Extração de Assets (client/) e Design System:**
   - Execute os scripts de extração e conversão dos binários, mapas e ícones de `client/`.
   - Configure as bibliotecas de UI, o sistema de temas (Claro e Escuro), tokens de cores, tipografia e componentes base (Cards, Tooltips, Badges e Layout Shells).
3. **Etapa 3 — Implementação do Portal Público e WYDpedia:**
   - Construa a Home cinematográfica, o catálogo completo da WYDpedia (Itens, Skills, Mobs, Guias e Manuais) e o Atlas Interativo de Mapas.
4. **Etapa 4 — Modernização da Área Administrativa e Integração Launcher:**
   - Implemente os dashboards de monitoramento, gráficos, contadores, gestão de jogadores, CMS e controle avançado do launcher.
5. **Etapa 5 — Polimento Final, Animações e Revisão de Qualidade:**
   - Revise responsividade, fluidez das animações, contraste nos temas Claro e Escuro, performance de carregamento e integridade de todos os links e dados.

---

## 9. Escopo decidido em 2026-11-20

**Decisão do dono (2026-11-20), registrada ao fechar `M21`:** o portal da `1.0.0` é o portal público
**e** a área administrativa inteira descrita em §5 e §6 — o administrador configura o servidor pelo
portal, não só o monitora.

| Item | O que entra na `1.0.0` |
|---|---|
| Portal público | home, notícias, rankings, WYDpedia e atlas (§5) |
| Área administrativa | NOC, jogadores e contas, economia, conteúdo/CMS, launcher, logs e segurança (§6.1) |
| Configuração do servidor | taxas, eventos, canais, manutenção e demais parâmetros editáveis pela área administrativa, com trilha de auditoria |
| Monitoramento e métricas | dashboards, KPIs e séries históricas (§6.2) |

O legado tem rascunho de parte dessas telas e funcionalidades; ele é **referência (`H`)**, não fonte —
cada tela é reformulada para o projeto atual. O detalhe por tela, rota e permissão é do planejamento
do `wyd-portal`, que ainda não existe: esta decisão fixa **escopo**, não desenho, e a lista de telas
segue aberta como página, não como lacuna (`M21`, `wyd-db/lacunas.md` §2.3).