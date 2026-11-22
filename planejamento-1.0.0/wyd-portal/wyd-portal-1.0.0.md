# wyd-portal 1.0.0 — plano diretor do portal e da área administrativa

> **Estado: APROVADO em 2026-11-20.** Este documento abre o planejamento do `wyd-portal` e mede o
> que existe; ele não é a especificação de tela nem substituto de `docs/`. O escopo foi decidido pelo
> dono em 2026-11-20 (`M21`, `wyd-db/lacunas.md` §2.3): portal público **e** área administrativa
> inteira, com o administrador configurando o servidor pelo portal. O desenho por rota, permissão e
> tabela vive nas páginas de `docs/`, também aprovadas nessa data.
>
> Base medida em 2026-11-20: `planejamento-1.0.0/wyd-portal/` tinha **2 arquivos** (`.gitkeep` e
> `wyd-portal-prompt.md`, 148 linhas) e nenhuma página de planejamento. Nada de código existe em
> `wyd-portal/`.
>
> Referência, nunca fonte: o prompt do portal (`wyd-portal-prompt.md`) é **orientação de produto**
> (`H`); o que vale é o que está decidido no planejamento novo e no `wyd-db`.

---

## 1. O que este sistema é

O `wyd-portal` é a aplicação **Next.js** que serve duas superfícies com o mesmo login:

1. **Portal público** — home, notícias, rankings, WYDpedia (itens, skills, mobs, guias), atlas de
   mapas, armaria do personagem (perfil público, somente leitura) e mercado global com leilão em
   crédito do portal, com dados de conteúdo do schema `content` e estado agregado do jogo;
2. **Área administrativa** — NOC, jogadores e contas, economia, conteúdo/CMS, launcher, logs e
   segurança; é onde o staff **opera e configura** o servidor, com trilha de auditoria.

Ele não é o servidor de jogo e não implementa regra de combate, drop ou progressão: lê e escreve
estado pelos contratos do `wyd-db` e, quando a operação exigir ação no processo do jogo, pelo
`wyd-admin`.

## 2. Fronteira (resumo; dono é `docs/arquitetura/fronteira.md`)

| Pergunta | Resposta curta |
|---|---|
| O que faz | portal público + área administrativa, autenticação de pessoa, BFF e leitura/escrita de estado |
| O que não faz | tick, combate, drop, progressão, protocolo de fio, patch do cliente, operação de banco |
| Quem consome | jogador (navegador), staff (navegador), launcher (notícias e manifesto, quando houver contrato) |
| O que consome | `wyd-db` (SQL pelo adapter), `wyd-admin` (HTTP/JSON interno: contrato C4 e custódia de item propostos em 2026-11-22), `content` (catálogo) |
| Estado dele | sessão de portal (`portal.person_session`), token de cerimônia (`portal.person_token`), cache de página |
| O que quebra se cair | login e área administrativa param; o jogo **não** para, porque sessão de jogo mora em `game.session` |

## 3. Escopo da `1.0.0`

| Dentro | Fora |
|---|---|
| login, cadastro, recuperação de senha e sessão de portal (`0003`/`0004`) | autenticação do cliente 7662 (é `wyd-net`/`wyd-server`) |
| home, notícias, rankings, WYDpedia e atlas | renderização de mapa 3D e cliente do jogo |
| NOC, jogadores, contas, economia, CMS, launcher, logs e configuração | regra de jogo (tick, dano, drop) — `wyd-core`/`wyd-sim` |
| leitura de estado do jogo e escrita administrativa com auditoria | migrations e DDL — `wyd-db` |
| BFF do portal (rotas de servidor) | API interna de staff — `wyd-admin` |
| armaria pública read-only e mercado global com leilão (F7–F11, decididos em 2026-11-22) | edição de personagem e inventário pelo jogador: armaria é só leitura |

**Decidido em 2026-11-20 (`M21`):** o administrador configura o servidor pelo portal, incluindo
taxas, eventos, canais e manutenção, com trilha de auditoria. Isso não é monitoramento só: é escrita.

## 4. Números deste planejamento

Medido em 2026-11-20, neste diretório (a contagem de páginas foi recontada em 2026-11-22):

| Medida | Valor | Fonte |
|---|---:|---|
| Páginas de planejamento escritas | 11 | `find planejamento-1.0.0/wyd-portal -name '*.md'` (exclui o prompt; inclui a proposta de 2026-11-22) |
| Páginas em `docs/` | 8 | `docs/index.md`, 4 de `arquitetura/`, 2 de `specs/`, 1 de `roadmap/` |
| Requisitos funcionais | 33 | `docs/specs/requisitos.md` (18 de 2026-11-20 + 15 de armaria e mercado) |
| Requisitos não funcionais | 10 | `docs/specs/requisitos.md` §5 (7 de 2026-11-20 + 3 de armaria e mercado) |
| Lacunas abertas | 0 | `lacunas.md` §1 (8 fechadas, `LAC-POR-01` a `LAC-POR-08`; `LAC-POR-09` em proposta com `P-POR-09`) |
| Rotas mapeadas | 50 | `docs/arquitetura/rotas-e-permissoes.md` §2–§3 (43 até 2026-11-22 + 7 de armaria/mercado) |
| Tabelas do `wyd-db` citadas | 52 (51 com DDL) + 2 views; as 11 do mercado têm DDL no lote `0034`; quatro views seguem em §7.1 | `docs/arquitetura/consumo-do-banco.md` §7 e §7.1 |
| Issues criadas | **0** | nenhuma, por decisão desta rodada |

Número sem fonte e data não entra: os valores acima saem dos arquivos citados, não de memória.

## 5. Estrutura do planejamento

```text
planejamento-1.0.0/wyd-portal/
├── wyd-portal-1.0.0.md          # este dashboard
├── lacunas.md                   # registro único de lacuna e dono
├── armaria-e-mercado.md         # armaria e mercado: D1–D10 decididas, fatias F7–F11 (2026-11-22)
├── wyd-portal-prompt.md         # orientação de produto recebida (H)
└── docs/
    ├── index.md                 # porta de entrada
    ├── arquitetura/
    │   ├── fronteira.md         # contrato com wyd-db e wyd-admin
    │   ├── rotas-e-permissoes.md# telas por rota e nível de acesso
    │   ├── consumo-do-banco.md  # o que cada tela lê e escreve
    │   └── monitoramento.md     # fontes, paridade com o legado e degradação
    ├── specs/
    │   ├── requisitos.md        # RF-POR e RNF-POR com critério de aceitação
    │   └── analise-do-prompt.md # o prompt lido contra o que está decidido
    └── roadmap/
        └── roadmap.md           # fatias com DoD, sem issue criada
```

`src/` e `tests/` **não** foram criados: árvore de código se planeja quando a primeira fatia for
aprovada, e pasta vazia por simetria é enfeite (`AGENTS.md` §5.10).

## 6. O que falta para o planejamento sair de rascunho

| Pendência | Dono | Efeito |
|---|---|---|
| ~~Aprovar o plano diretor e as páginas~~ — **aprovado em 2026-11-20** | dono do produto | páginas em `aceito` |
| ~~`LAC-POR-01`~~ — **decidida:** escrita e ação no jogo só pelo `wyd-admin` | arquitetura | BFF não tem `USAGE` em `game`; contrato HTTP ainda sem forma |
| ~~`LAC-POR-02`/`03`~~ — **decididas:** `news`, `banner`, `guide` e `launcher_release` em `portal` | arquitetura + produto | lotes `0030`/`0031` aprovados no `wyd-db` em 2026-11-20, mais o `0032` (só `GRANT`, 2026-11-21); série 34 arquivos / 109 tabelas |
| ~~`LAC-POR-04`~~ — **decidida:** ranking direto de `game` com `LIMIT` e cache BFF de 60 s | arquitetura | sem tabela/view materializada nova |
| ~~`LAC-POR-05`~~ — **decidida:** TOTP em `portal.person_authenticator`, só staff | produto + segurança | `RF-POR-013`; DDL no lote `0031` do `wyd-db` |
| ~~`LAC-POR-06`~~ — **decidida:** telemetria vem do `/metrics` Prometheus do `wyd-server`, coletado pelo `wyd-ops`; o portal consulta a API | `wyd-ops` + arquitetura | NOC mostra CPU/RAM/latência com instante da amostra; sem tabela nova no `wyd-db` |
| Planejar o `wyd-admin` | arquitetura | forma do contrato C4 (rotas, payload, códigos) |
| Contrato de observabilidade do `wyd-ops` | `wyd-ops` | valor de URL/credencial, intervalo de coleta, retenção definitiva e dashboards; fontes e nomes estão em `docs/arquitetura/monitoramento.md` |
| ~~Decidir armaria e mercado~~ — **decididas:** D1–D10 em 2026-11-22 | produto + arquitetura | `armaria-e-mercado.md`, `requisitos.md` §4, `roadmap.md` F7–F11; schema no lote `0034` (`LAC-ADM-07` fechada) |
| ~~Executor da transferência de item~~ — **fechada em 2026-11-22** | arquitetura + `wyd-admin` | contrato nas quatro rotas do C4 (`contrato-c4.md` §4.9–§4.13) e fatia `ADM-10`; lote `0034` aprovado (`LAC-ADM-07` fechada); F8 espera aplicar a série |
| ~~Origem de dano, alcance e durabilidade~~ — **fechada em 2026-11-22** | `wyd-server` + `wyd-db` | `LAC-POR-08`: ficha publica só o provado; durabilidade não existe; captura de dano/alcance em `P-POR-08` |
| Dono do fechamento de leilão | arquitetura + `wyd-ops` | `LAC-POR-09` em **proposta**: `P-POR-09` recomenda BFF fecha e `wyd-ops` agenda/alerta; falta o “sim” de operação |

## 7. Gate

```bash
# cabeçalho em toda página de docs/
grep -rL '^> Pergunta:' planejamento-1.0.0/wyd-portal/docs --include='*.md'
# nenhum código dentro do planejamento
find planejamento-1.0.0/wyd-portal -type f \( -name '*.ts' -o -name '*.tsx' -o -name '*.sql' \)
# site
cd wyd-docs && python preparar.py && mkdocs build --dir .build/docs --output site && python verificar.py site
```

O `wyd-portal` **já está** em `SISTEMAS` de `wyd-docs/preparar.py`: o gate publicou
  `sistemas/wyd-portal/` e, na última medição (2026-11-22), passou com **183 páginas** e **181 fontes**
  verificadas. As páginas de
  `docs/` são publicadas; o dashboard e `lacunas.md` ficam na raiz do sistema e são citados por
  nome, não por link.
