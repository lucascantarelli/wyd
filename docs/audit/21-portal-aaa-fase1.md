# Fase 1 — Descoberta do portal AAA, WYDpedia e Command Center

> **Executada em:** 2026-11-15 · **Natureza:** investigação **somente leitura** no código.
> Nenhum arquivo de `crates/`, `wyd-portal/src/`, `client/` ou `roadmap-rust/` foi alterado.
> O único artefato novo é este documento.
> **Objetivo:** responder às Frentes A e B do briefing antes de criar ou alterar qualquer tela.

Este documento **não substitui** [`04-client-analysis.md`](04-client-analysis.md),
[`08-reverse-engineering.md`](08-reverse-engineering.md) nem `docs/cliente/`: ele os cita,
acrescenta o que foi medido agora e registra as **decisões de contrato** que bloqueiam a
Fase 2. Onde a documentação e o código divergem, o código vence.

---

## 1. Resumo executivo

1. O portal **não é** um "sistema burocrático sem identidade". Ele já tem direção de arte
   própria ("laca e ouro": obsidiana + ouro velho, serifa lapidar, cantos vivos, densidade de
   mesa de operação) e ela é **contratual**: `docs/arquitetura/identidade.tokens` é fonte única
   consumida pelo portal **e** pelo launcher (embutida no binário), com teste de paridade.
2. Por isso, três pedidos do briefing **colidem com contratos existentes** — Tailwind/Shadcn
   com "tokens épicos", tema claro, e animação por `framer-motion`. Nenhum é impossível, mas
   nenhum pode ser feito "instalando a lib": cada um exige ADR (§5).
3. A Frente B (engenharia reversa de `client/`) **já está feita e versionada**: 21 documentos em
   `docs/audit/`, 23 em `docs/cliente/`, 9 artefatos gerados e uma toolchain que roda (§4).
4. O que **falta de verdade** para a WYDpedia é a camada de **ícone e tooltip**: o catálogo
   publicado tem nome, preço, grade, nível, tipo e efeitos — **não tem ícone**, embora a tabela
   de índices e o atlas existam e tenham sido medidos agora (§4.3).
5. O briefing chega **truncado** no meio da Frente B ("…ItemList.bin, Skil"). As Frentes C e D
   e as Fases 2+ não estão no texto recebido. Este documento cobre A e B e presume **nada** do
   resto.

---

## 2. Frente A — stack real do portal (medido)

### 2.1 Versões instaladas (não as declaradas)

| Pacote | Declarado (`package.json`) | Instalado (`node_modules`) |
|---|---|---|
| `next` | `^16.4.6` | **16.4.6** |
| `react` / `react-dom` | `^19.1.0` | **19.3.1** |
| `typescript` | `^7.0.2` | **7.0.2** |
| `lucide-react` | `^1.70.0` | **1.72.0** |
| `jose` | `^6.2.17` | **6.2.17** |
| `zod` | `^4.6.7` | **4.6.7** |

- **Router:** App Router, com grupos `(admin)` e `(public)`. Não há Pages Router.
- **Saída:** `output: "standalone"` (imagem Docker roda `.next/standalone/server.js`).
- **Autenticação:** BFF puro; cookie `wyd_session` httpOnly HS256 (`jose`), `src/proxy.ts`
  como guarda de rota com matcher literal coberto por teste. O browser nunca fala com o
  gateway nem vê token de staff (ADR-0008).
- **Estilização:** CSS Modules globais escritos à mão. **Sem Tailwind, sem Radix, sem Shadcn.**
- **Animação/gráficos/tabelas:** **sem** `framer-motion`, `recharts`, `@tremor/react`, `cmdk`
  ou `@tanstack/react-table`. Tabela própria (`DataTable`), gráficos via embed do Grafana.

### 2.2 Tamanho da superfície

| Medida | Valor |
|---|---:|
| `page.tsx` em `src/app` | 28 |
| `route.ts` (BFF) em `src/app` | 18 |
| `globals.css` | 2.474 linhas |
| `publico.css` | 703 linhas |
| `guias.css` | 237 linhas |
| Suítes em `tests/` | 5 (`unidade`, `contrato`, `integracao`, `componente`, `e2e`) |

### 2.3 O que o contrato de identidade realmente impõe

`wyd-portal/tests/contrato/identidade.test.ts` reprova quando:

1. uma cor `cor.*` de `identidade.tokens` não existe no CSS com **o mesmo valor**;
2. a escala `espaco.*` diverge;
3. `raio`/`tipografia` divergem;
4. aparece **qualquer HEX de 6 dígitos no `globals.css` que não esteja em `identidade.tokens`**;
5. a tela de login usa estilo inline ou HEX;
6. o nome de servidor em `servidor.N` passa de 8 bytes (campo de 9 B do `sn.bin`).

> **Consequência direta:** ampliar a paleta para "AAA" é editar
> `docs/arquitetura/identidade.tokens` — que é **compartilhado com o launcher Rust**
> (`crates/wyd-launcher/src/identidade.rs` via `include_str!` e
> `crates/wyd-launcher/tests/contrato/identidade.rs`). Não é um arquivo do portal; é uma
> fronteira entre linguagens.

---

## 3. O que o portal público e o admin já têm

Extraído do `README.md` do portal (contrato real do gateway, não intenção):

- **Público:** `/` (home com hero, notícias, status, ranking), `/noticias` e `/noticias/[slug]`,
  `/ranking`, `/downloads`, `/guias` e `/guias/[slug]`, `/enciclopedia` e `/enciclopedia/[index]`,
  `/login`, `/conta` (sessões, histórico, PIN numérico).
- **Staff:** `/painel`, `/contas`, `/itens`, `/auditoria`, `/economia`, `/correio`, `/entregas`,
  `/financeiro`, `/configuracoes` (+ `/configuracoes/launcher`), e monitoramento com
  `/monitoramento`, `/monitoramento/servidor`, `/monitoramento/grafana`,
  `/monitoramento/prometheus`, `/monitoramento/logs`, `/monitoramento/launcher`.
- **Regra dura:** menu não anuncia rota sem contrato no `wyd-admin`. Tela nova exige
  `opcode`/handler no gateway — o portal não inventa dado.

**Gap para "Command Center":** o que o briefing pede (CPU, memória, rede, processos, players e
mobs separados, DB, alertas) depende de fonte no Prometheus/Grafana/gateway. O README registra
que `/monitoramento` já lê Prometheus + Loki + Grafana + presença por canal; o que **não** existe
é a página customizada (o briefing chama de "não importar o Grafana inteiro"). Isso é Fase 2.

---

## 4. Frente B — engenharia reversa de `client/`

### 4.1 Já existe e roda (verificado nesta árvore)

| Peça | Evidência |
|---|---|
| `tools/client/` | 22 módulos: `pe.mjs`, `strings.mjs`, `keytable.mjs`, `formats.mjs`, `wyt.mjs`, `serverlist.mjs`, `extract-tables.mjs`, `strdef.mjs`, `map-skills.mjs`, `captura*.mjs` |
| Artefatos | `docs/cliente/gerado/`: `pe.json`, `strings.json`, `evidencia.json`, `formatos.json`, `tabelas.json`, `wyt.json`, `skills-map.json`, `goldens.json`, `ancoras.txt` |
| Dossiê | `docs/cliente/`: 23 documentos (formatos, tabelas, pacotes, customização, mapa, inventário) |
| Auditoria | `docs/audit/`: 21 documentos + `systems/` com 9 |
| Conteúdo | `client/UI` = 341 arquivos · `client/NUI` = 8 telas |

**Execuções feitas agora (saída real, não presumida):**

```text
node tools/client/wyt.mjs client/NUI/main.wyt --json
→ client/NUI/main.wyt: 616x300 32bpp · 42936 cores · alfa opaco 1

node tools/client/extract-tables.mjs
→ SkillData.bin    124 × 192 B   enums=21  vazias=6
→ ItemList.bin    6500 × 140 B   enums=0   vazias=6
→ ausente: taxa de drop por item (é configuração do servidor/TMSrv, não do cliente)
→ ausente: curva de EXP por nível (lógica do servidor)
```

Ou seja: o decodificador `.wyt` (magic `WT10`, raster puro) e o extrator de tabelas
**funcionam**. Não é preciso reconstruir a Frente B.

### 4.2 Tabelas binárias — registros medidos

| Arquivo | Bytes | Registro | Registros |
|---|---:|---:|---:|
| `ItemList.bin` | 910.004 | 140 B | **6.500** |
| `SkillData.bin` | 23.812 | 192 B | **124** |
| `itemicon.bin` | 26.000 | **4 B (int32 LE)** | **6.500** |

### 4.3 Descoberta nova — `itemicon.bin` (o elo que falta para o tooltip)

Medido nesta árvore, pela primeira vez de forma explícita:

```text
bytes: 26000   registros @4: 6500
-1 (sem ícone): 524      zero: 3314      positivo: 5976
mín/máx positivo: 0 .. 940      ids distintos: 913
amostra idx 400..410: 1,1,1,1,1,2,2,2,2,2,3
```

**Confirmado:** `itemicon.bin` é uma tabela de **6.500 registros `int32` little-endian**,
paralela a `ItemList.bin` (mesma contagem), onde `-1` significa "item sem ícone" e os valores
`0..940` indexam um atlas — **913 ids distintos** para 6.500 itens, coerente com reuso de ícone
por família.

**Atlas localizado e decodificado (corrigido em 2026-11-15):**

```text
client/UI/itemicon01..10.wyt → 350 × 350, 32bpp = 10×10 células de 35 px (1.000 slots)
client/UI/itemicon.wyt       → 400 × 3550 — NÃO é o atlas de ícones
```

> **Correção de uma conclusão anterior deste documento.** Eu havia tratado
> `itemicon.wyt` (400×3550) como o atlas, por causa de calhas de fundo a cada 35 px. A
> hipótese foi **refutada por medição**: as células de 35 px daquele arquivo são
> **byte-idênticas entre si** (`cel(36,16) == cel(71,16) == cel(36,51)`), ou seja, é uma
> textura de slot vazio que se repete — não arte de item. Os atlas reais são os arquivos
> numerados: 350 = 10×35 exato, e todas as células são distintas.

**Evidência do mapeamento `icon id → arquivo`:** 10 arquivos × 100 = **1.000 slots** para
**941** ids usados. A distribuição por categoria bate com a arte: `accessory` 45/46 e
`consumable` 36 em `0000s` (itemicon01 traz anéis/amuletos/poções), `weapon` 238 em `0100s`
e 72 em `0200s` (itemicon02/03 são armas).

**O que continua NÃO confirmado:** a ordem célula a célula. Amostras renderizadas divergiram
em pelo menos um caso (`Dianus`, garra, id 602 → um sprite de runa). Por isso o artefato
`icones.json` declara `confianca.ordem = "fortemente indicado"` e o extrator tem
`--amostra` para conferência. **Não promover a contrato sem prova visual no cliente.**

### 4.4 O que a WYDpedia publica hoje

`wyd-portal/src/server/enciclopedia.ts` lê `src/content/itens.json` (gerado por
`tools/content/gen-enciclopedia.mjs` a partir do seed `0017`) e expõe por item:
`index, name, price, grade, reqLevel, kind, system, eventOnly, tradeable, effects[]`.

**Não expõe:** ícone, descrição, requisitos de atributo, durabilidade, refino, classe permitida,
drops, NPCs que vendem. Busca por nome com dobra de acento (`fold`) e filtro por `kind`
existem; `limit` padrão 120 e **sem paginação no servidor**.

Para o padrão "Wowhead" pedido, as lacunas reais são: **ícone + tooltip** (§4.3), **página de
item com relações** (drop/venda/refino) e **busca instantânea com paleta de comandos**.

---

## 5. Conflitos entre o briefing e os contratos — exigem decisão/ADR

| # | Pedido do briefing | Contrato existente | O que acontece se instalar direto |
|---|---|---|---|
| **C1** | Tailwind + Radix/Shadcn "com tokens épicos" | `identidade.tokens` = fonte única; teste reprova HEX fora dela; `globals.css` com 2.474 linhas de CSS autoral | Fonte de verdade dupla. Shadcn traz paleta e raio próprios → o teste de identidade quebra ou passa a ser burlado |
| **C2** | "Suporte refinado a Tema Claro e Tema Escuro" | `wyd-portal/docs/identidade-visual.md` lista **"tema claro"** em *"O que deliberadamente não existe"* | Contradiz decisão escrita. Exige ADR **e** uma segunda paleta em `identidade.tokens`, que o launcher Rust também lê |
| **C3** | `framer-motion` para transições/micro-interações | O mesmo doc recusa "animações decorativas"; `prefers-reduced-motion` já é respeitado no CSS | Dependência nova + orçamento de JS; precisa de decisão sobre o que anima e o que não |
| **C4** | `recharts`/`@tremor/react` para KPIs | Hoje o admin lê Prometheus/Loki por HTTP e embute Grafana | Gráfico só pode desenhar o que o gateway/Prometheus entrega; risco de número inventado |

Pela invariante **#7** do `AGENTS.md` (dependência pesada e mudança de fronteira exigem ADR em
`docs/arquitetura/adr/`), **C1–C4 não podem ser resolvidos por instalação**. E, pelo §2.5 do
próprio briefing, a Fase 1 é investigativa.

**Observação de arquitetura:** para o padrão AAA pedido, o caminho de menor atrito **não é**
substituir o CSS autoral por Tailwind — é **estender `identidade.tokens`** (mais degraus de
superfície, estado e tipografia) e manter o teste de paridade como guarda. O que dá a sensação
"AAA" (hero cinematográfico, parallax, revelação) é composição e movimento, não uma lib de
componentes.

---

## 6. O que falta para fechar a Fase 1

| Frente | Estado |
|---|---|
| **A — código/stack/legado** | **Concluída** nesta árvore (§2–§3) |
| **B — binários do `client/`** | **Concluída** por `docs/audit/` + `docs/cliente/`, com o acréscimo de §4.3 |
| **C — ?** | **Não recebida** (briefing truncado no meio da Frente B) |
| **D — ?** | **Não recebida** |

## 7. Próximos passos propostos (nenhum iniciado)

1. **Decidir C1–C4** (§5) e, se aprovado, escrever um ADR de identidade/UI antes de tocar telas.
2. **Fechar a geometria do ícone** (`itemicon.wyt`) com um teste de fatiamento, depois ligar
   ícone ao item em `gen-enciclopedia.mjs` → `itens.json` → `enciclopedia.ts`.
3. **Tooltip de item** (componente puro, sem nova dependência) reaproveitando `effects[]`.
4. **Command Center**: consolidar CPU/memória/rede/players/mobs/DB a partir de fonte que exista
   no Prometheus/gateway; nunca derivar número sem contrato.
5. Receber o **restante do briefing** para planejar as Fases 2+.
