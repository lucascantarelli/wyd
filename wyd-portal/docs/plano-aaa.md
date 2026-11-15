# Plano AAA — Etapa 1: diagnóstico, sitemap e stack

> **Data:** 2026-11-15 · **Escopo:** portal público, WYDpedia, Atlas e Centro de Comando.
> **Natureza desta etapa:** diagnóstico e plano. Nenhuma tela foi alterada.
> Documento irmão de [`identidade-visual.md`](identidade-visual.md) (direção de arte) e de
> [`../docs/audit/21-portal-aaa-fase1.md`](../docs/audit/21-portal-aaa-fase1.md) (medições).

---

## 1. Diagnóstico técnico

### 1.1 Versões reais (lidas de `node_modules`, não do `package.json`)

| Pacote | Instalado |
|---|---|
| `next` | **16.4.6** — App Router, `output: "standalone"` |
| `react` / `react-dom` | **19.3.1** |
| `typescript` | **7.0.2** (strict) |
| `lucide-react` | 1.72.0 |
| `jose` / `zod` | 6.2.17 / 4.6.7 |

- Rotas: 28 `page.tsx` + 18 `route.ts` (BFF) em `src/app`, grupos `(admin)` e `(public)`.
- Auth: cookie `wyd_session` httpOnly HS256; `src/proxy.ts` guarda de rota; browser nunca vê
  token de staff (ADR-0008).
- Estilo: CSS autoral, 3.414 linhas (globals 2.474 · publico 703 · guias 237). Sem Tailwind.
- Fontes: `Cinzel` (display) + `Manrope` (operação) via `next/font` — **já coincide com o
  briefing**; falta apenas a mono (coordenadas/IDs/métricas): `JetBrains Mono`.
- Sem `framer-motion`, `cmdk`, `@tanstack/react-table`, `recharts`, Radix.

### 1.2 Ativos do jogo disponíveis (Frente B)

| Ativo | Medição | Uso no portal |
|---|---|---|
| `client/ItemList.bin` | 6.500 × 140 B | catálogo (já no seed `0017`) |
| `client/SkillData.bin` | 124 × 192 B | catálogo de skills |
| `client/itemicon.bin` | **6.500 × int32 LE**; `-1`=sem ícone (524), `0..940` (913 ids) | **ícone por item** |
| `client/UI/itemicon01..10.wyt` | 350×350, 32bpp = **10×10 células de 35 px** | atlas real dos ícones (1.000 slots) |
| `client/UI/itemicon.wyt` | 400×3550 — **não** é atlas (textura de slot vazio que se repete) | nenhum |
| `client/UI` | 341 arquivos | telas/HUD |
| `client/NUI` | 8 telas | login, seleção de personagem |
| `tools/client/wyt.mjs` | `parseWyt` · `toPng` · `toBmp` | conversão para web |

**Grade do atlas (medida e confirmada):** `itemicon01..10.wyt` são 350×350 = **10×10 células
de 35 px**. O arquivo `itemicon.wyt` (400×3550) **não** é o atlas: as células de 35 px dele são
byte-idênticas (é uma textura de slot vazio que se repete). O mapeamento `icon id → arquivo`
é `1 + floor(id/100)`, `célula = id % 100` — **fortemente indicado** (10 arquivos = 1.000
slots para 941 ids; `weapon` 238 em `0100s`, `accessory` 45/46 em `0000s`), mas a ordem
célula a célula **não está provada**: `Dianus` (garra, id 602) caiu num sprite de runa.

**Pipeline pronto:** `node tools/client/extrair-icones.mjs` gera
`wyd-portal/public/game-assets/itens/atlas-01..10.png` + `icones.json` (mapa item→ícone).
`--amostra <ids> <pasta>` recorta ids para conferência visual.

---

## 2. Sitemap alvo

### 2.1 Portal público

| Rota | Estado |
|---|---|
| `/` | existe — vira **hero cinematográfico** (parallax, status ao vivo, contagem de guerra) |
| `/noticias`, `/noticias/[slug]` | existe — ganha categorias/tags |
| `/ranking` | existe — ganha abas (nível, evolução, PvP, guildas) |
| `/downloads` | existe |
| `/guias`, `/guias/[slug]` | existe |
| `/login`, `/conta` | existe |

### 2.2 WYDpedia (novo)

| Rota | Conteúdo |
|---|---|
| `/wydpedia` | hub com busca instantânea (`Ctrl+K`) |
| `/wydpedia/itens` | catálogo 3.220 itens, filtros por tipo/grade/nível/classe |
| `/wydpedia/itens/[index]` | ficha do item + `<WydTooltip>` + relações (drop/venda/refino) |
| `/wydpedia/skills`, `/wydpedia/skills/[id]` | 151 skills, custo/cooldown/alcance |
| `/wydpedia/mobs`, `/wydpedia/mobs/[id]` | bestiário com drops |
| `/wydpedia/npcs` | NPCs e lojas |
| `/wydpedia/manuais` | guias editoriais (Ongame) |
| `/wydpedia/atlas` | canvas com zoom/pan, camadas e coordenadas X,Y |

> `/enciclopedia` atual permanece como redirect 308 para `/wydpedia/itens`.

### 2.3 Centro de Comando (admin)

| Grupo | Rotas |
|---|---|
| **Visão Geral (NOC)** | `/painel` (KPIs clicáveis → detalhe) |
| **Jogadores e Contas** | `/contas`, `/contas/[id]` (inventário visual com ícones reais, baú, montaria, trades) |
| **Economia e Itens** | `/economia`, `/financeiro`, `/itens` |
| **Conteúdo (CMS/WYDpedia)** | `/conteudo` (novo) |
| **Launcher e Patches** | `/configuracoes/launcher`, `/launcher/patches` (novo) |
| **Logs e Segurança** | `/auditoria`, `/monitoramento/*` (6 telas existentes) |

**Regra preservada:** menu não anuncia rota sem contrato no `wyd-admin`. Rotas novas de
**leitura** já têm contrato (WEB-05/WEB-06, ver `README.md` do portal). Tela nova de
**escrita** exige handler no gateway — entra como issue separada, não como tela vazia.

---

## 3. Stack de UI recomendada

| Necessidade | Pacote | Por que |
|---|---|---|
| Primitivos acessíveis | `radix-ui` (ou `@radix-ui/react-*`) | tooltip/dialog/tabs/select sem reinventar foco e ARIA; já existe `<dialog>` nativo, migração incremental |
| Motion | `motion` (sucessor do `framer-motion`) | reveals, contadores, hover magnético; respeita `prefers-reduced-motion` |
| Paleta de comandos | `cmdk` | `Ctrl+K` da WYDpedia |
| Tabelas massivas | `@tanstack/react-table` | 3.220 itens + contas: ordenação/filtro/paginação sem virtualizar à mão |
| Gráficos | `recharts` | área/linha/barra para o NOC; dados só de Prometheus/gateway |
| Tipografia mono | `next/font` → `JetBrains Mono` | coordenadas, IDs, métricas |

### 3.1 Tailwind: sim, mas subordinado aos tokens

`tests/contrato/identidade.test.ts` reprova **qualquer HEX de 6 dígitos no `globals.css`**
que não venha de `docs/arquitetura/identidade.tokens`. Instalar Shadcn "padrão" criaria uma
segunda paleta e quebraria o teste — ou pior, seria burlado.

**Integração proposta (Tailwind v4, CSS-first):** o tema do Tailwind **não declara cor
literal**; cada cor aponta para a variável dos tokens.

```css
@theme {
  --color-surface: var(--surface);
  --color-accent: var(--accent);
  /* …uma entrada por token, sem nenhum HEX */
}
```

Assim o teste continua sendo a guarda, o launcher continua lendo o mesmo arquivo e o
Tailwind vira só açúcar de utilitário. Componentes Radix/Shadcn são reescritos com essas
classes — nenhum cinza padrão entra.

---

## 4. Tema claro e escuro (decisão D1 — precisa de ADR)

O briefing pede dual-theme com HEXs próprios. O arquivo de tokens é **compartilhado com o
launcher Rust** (`include_str!` em `crates/wyd-launcher/src/identidade.rs`, com teste de
contrato em `crates/wyd-launcher/tests/contrato/identidade.rs`). Alterar `cor.bg`/`cor.accent`
muda a skin do launcher e exige mexer em `crates/`.

**Recomendação: mudança puramente aditiva**, sem tocar `crates/`:

| Token novo | Valor | Papel |
|---|---|---|
| `cor.claro.bg` | `#F8F6F0` | fundo pergaminho |
| `cor.claro.surface` | `#FFFFFF` | cartão |
| `cor.claro.surface-2` | `#EFECE4` | camada |
| `cor.claro.text` | `#1A1D24` | carvão |
| `cor.claro.muted` | `#5C6270` | legenda |
| `cor.claro.border` | `#D9D3C6` | filete bronze |
| `cor.ouro-imperial` | `#D4AF37` | acento cerimonial (ambos os temas) |
| `cor.arcano` | `#3B82F6` | azul arcano |
| `cor.batalha` | `#EF4444` | carmesim de batalha |

Com `[data-tema="claro"]` remapeando `--bg`/`--surface`/… para os valores claros, o teste de
identidade continua verde (todos os HEXs estão no arquivo canônico) e o launcher não muda.

**Não recomendado:** sobrescrever `cor.bg` (`#0f1115`) pelos `#090B10` do briefing — a
diferença é imperceptível e obrigaria a regerar a skin do launcher. Se for desejado, entra
como ADR próprio.

---

## 5. Ordem de execução

1. **Etapa 1** — este documento ✔
2. **Etapa 2** — extração de ativos → `public/game-assets/`; tokens (`claro.*`, ouro, arcano,
   batalha); Tailwind mapeado; primitivos (Card, Badge, Tooltip, Skeleton)
3. **Etapa 3** — Home cinematográfica + WYDpedia + Atlas
4. **Etapa 4** — NOC, jogadores, launcher/patches, CMS
5. **Etapa 5** — polimento, contraste, performance, links

## 6. Pendências declaradas

| Ponto | Estado |
|---|---|
| Geometria final da célula de ícone (33×33 / passo 35) | **Fortemente indicado** — confirmar com amostras renderizadas |
| Mapas de terreno (`Env/`, heightmaps) para o Atlas | **Não localizado** ainda — Frente B.3 |
| Taxa de drop por item | **Não existe no cliente** (é config do servidor/TMSrv) — o bestiário não pode inventar |
| Curva de EXP | **Não existe no cliente** (lógica do servidor) |
| Contagem de jogadores online / guerra | Depende de contrato no gateway — verificar antes de prometer widget |
