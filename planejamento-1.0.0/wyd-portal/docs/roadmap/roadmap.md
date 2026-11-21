> Pergunta: "em que ordem o portal é construído e como cada fatia prova que terminou?"
> Público: produto, arquitetura e desenvolvedor novo
> Dono: produto
> Base: `docs/specs/requisitos.md`; `docs/arquitetura/rotas-e-permissoes.md`; `M21`
> Revisado em: 2026-11-20

**Estado:** aceito (2026-11-20)

Fatias de entrega, cada uma com DoD executável. **Nenhuma issue foi criada**: o roadmap planeja
ordem e prova; abrir issue, milestone ou board é ato separado e não pedido nesta rodada.

## 1. Regras do fatiamento

1. Fatia só entra se o DoD tiver comando ou arquivo verificável.
2. Fatia bloqueada por lacuna (`lacunas.md`) não começa antes de a lacuna fechar.
3. Toda fatia administrativa inclui auditoria e teste de permissão negada.
4. Dependência de DDL é explícita: sem a migration aplicada, a fatia não fecha.

## 2. Fatias

### F0 — Fundação do portal (auth e sessão)

| Campo | Valor |
|---|---|
| Fase | 1 |
| Requisitos | `RF-POR-001` a `RF-POR-003` |
| Depende de | `0001`–`0004` aplicadas |
| Entrega | app Next.js, login, cadastro, recuperação, middleware de sessão, layout dos dois temas |
| DoD | E2E de cadastro → verificação → login → logout verde; sessão revogada não acessa `/conta`; `npm run build` e `tsc --noEmit` limpos |
| Fora | área administrativa, WYDpedia, atlas |

### F1 — Conta e personagens

| Campo | Valor |
|---|---|
| Fase | 2 |
| Requisitos | `RF-POR-004` |
| Depende de | F0; `0005`/`0006` aplicadas; view de `0029` |
| Entrega | `/conta`, `/conta/personagens`, `/conta/seguranca`, `/conta/consentimentos`; criação de conta de jogo |
| DoD | sexta conta recusada com mensagem de vaga; personagem apagado não aparece; revogar sessão derruba a lista |
| Fora | edição de personagem e inventário |

### F2 — Portal público: home, notícias e rankings

| Campo | Valor |
|---|---|
| Fase | 3 |
| Requisitos | `RF-POR-005`, `RF-POR-006`, `RF-POR-007` |
| Depende de | F0; views de `0029`; lote das tabelas `portal.news`/`banner` para notícias |
| Entrega | home, `/noticias*`, `/rankings`, `/download` |
| DoD | home com banco vazio mostra `0` e hora da medição; ranking exclui `deleted_at`; rascunho de notícia não aparece |
| Fora | WYDpedia e atlas |

### F3 — WYDpedia e atlas

| Campo | Valor |
|---|---|
| Fase | 3 |
| Requisitos | `RF-POR-008`, `RF-POR-009` |
| Depende de | F2; `0013`–`0019`, `0024` e seus seeds |
| Entrega | `/wydpedia*`, `/atlas*`, busca e tooltip |
| DoD | 404 para item inexistente; busca ignora caixa; mapa sem pins mostra estado vazio; link cruzado mínimo por ficha |
| Fora | simulador de build e calculadora de dano |

### F4 — Admin: NOC, canais, logs e configuração

| Campo | Valor |
|---|---|
| Fase | 4 |
| Requisitos | `RF-POR-010`, `RF-POR-012`, `RF-POR-014` a `RF-POR-018`, `RNF-POR-001`, `RNF-POR-005`, `RNF-POR-007` |
| Depende de | F1; `0007`, `0002`; contrato do `wyd-admin` (`LAC-POR-01`); Prometheus, Loki e Grafana do `wyd-ops` (`LAC-POR-06`, `monitoramento.md` §4) |
| Entrega | `/admin`, `/admin/canais`, `/admin/logs`, `/admin/config`, as seis telas de monitoramento 38–43, middleware de papel |
| DoD | `member` recebe 403 em `/admin`; alteração de canal grava revisão e auditoria; revisão sem mudança é recusada; cada tela de monitoramento tem paridade com o legado (`monitoramento.md` §7); CPU/RAM/latência, logs e painéis vêm das APIs do `wyd-ops` com instante da amostra e viram “não medido” quando a fonte falha; nenhuma URL interna ou credencial chega ao navegador |
| Fora | kick/broadcast como SQL do portal (passam pelo `wyd-admin`); substituir o Grafana ou guardar série no banco |

### F5 — Admin: jogadores, contas e economia

| Campo | Valor |
|---|---|
| Fase | 4 |
| Requisitos | `RF-POR-011` |
| Depende de | F4; `0005`, `0006`, `0008`–`0011`; `wyd-admin` para ação no processo |
| Entrega | `/admin/jogadores*`, `/admin/contas*`, `/admin/economia*`, `/admin/seguranca` |
| DoD | ficha não devolve hash; bloqueio sem motivo recusado; ajuste de moeda vira linha no ledger; toda ação com autor em `audit_log` |
| Fora | alteração de inventário item a item (precisa de decisão de operação) |

### F6 — CMS e launcher

| Campo | Valor |
|---|---|
| Fase | 5 |
| Requisitos | `RF-POR-006` (edição) |
| Depende de | — (`0030` aprovado em 2026-11-20: `news`, `banner`, `guide`, `launcher_release`) |
| Entrega | `/admin/conteudo/noticias`, `/admin/conteudo/wydpedia`, `/admin/launcher` |
| DoD | publicar/despublicar notícia reflete no público em até um ciclo de revalidação; versão de launcher só publica manifesto íntegro |
| Fora | geração de patch e upload de binário (`wyd-ops`) |

## 3. Ordem e dependências

```text
F0 → F1 → F4 → F5
      ↘ F2 → F3
F6 destravado: lote 0030 aprovado em 2026-11-20
```

`F2` pode começar em paralelo a `F4` depois de `F1`, mas notícias só fecham quando o lote de
`portal.news`/`banner`/`guide`/`launcher_release` for aplicado no `wyd-db`.

## 4. Riscos por fatia

| Fatia | Risco | Mitigação |
|---|---|---|
| F0 | sessão de portal confundida com sessão de jogo | teste E2E explícito de que o jogo não depende do portal |
| F2 | ranking caro em tabela quente | `LAC-POR-04`: cache e instante declarado |
| F3 | conteúdo sem seed suficiente | nada de “dado autêntico” inventado; seed é do `wyd-db` |
| F4 | escrita em `game` bloqueada por permissão | ação só pelo `wyd-admin`, nunca SQL local |
| F4 | API de métricas indisponível | card mostra “não medido” e aponta o Grafana; ocupação e capacidade continuam vindo do banco |
| F4 | Loki ou Grafana fora | cada tela degrada isolada (`monitoramento.md` §8); presença e motor continuam e a tela diz qual fonte falhou |
| F4 | telemetria do launcher sem histórico no banco | agregado em processo + log no Loki; a tela declara a janela “desde o boot” e não inventa número |
| F6 | tela sem tabela | não começa antes do lote de schema no `wyd-db` |

## 5. O que este roadmap não faz

- Não estima prazo em dias: não há medição de velocidade do time.
- Não cria issue, milestone ou board.
- Não fixa biblioteca de UI: isso é da fatia, com registro de decisão se mudar fronteira.

*Página de planejamento. Nenhuma fatia foi iniciada.*
