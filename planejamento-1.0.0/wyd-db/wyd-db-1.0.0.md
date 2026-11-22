# wyd-db 1.0.0 — plano diretor da documentação oficial

> **Estado: RASCUNHO PARA APROVAÇÃO.** Este documento define **o que** a documentação oficial do
> `wyd-db` contém, **em que arquivo** cada assunto mora, **quem** responde por ele e **em que
> ordem** o material é produzido. Esta árvore é o **teto** da documentação, e não o seu estado:
> página escrita se reconhece pelo cabeçalho de §7.1 e pelo estado (`proposto`/`aceito`); o que está
> pronto é medido em `planos/metricas.md`, não aqui.
>
> Tudo o que está em `planejamento-1.0.0/wyd-db/` é material **novo**, escrito para a release
> `1.0.0`. Não há transcrição, cópia nem link para documentação anterior: o que existir aqui é a
> versão final, revisada e detalhada.
>
> Baseline **da série herdada**, medido em 2026-11-17: `crates/wyd-db/` com **53 migrations**,
> **24 arquivos Rust**, **5 alvos de teste** (`Cargo.toml`), **1 seed de desenvolvimento**; schema
> com **87 tabelas** e **625 colunas** conforme `docs/arquitetura/banco-catalogo.md` (gerado em
> 2026-11-15 por `make banco-catalogo`). Esses números descrevem o que a reescrita **substitui**,
> não o alvo: a contagem vigente é a medida depois da escrita e vive em `planos/metricas.md`.

---

## 0. Como ler este documento

| Se você quer… | Vá para |
|---|---|
| entender o propósito e o limite desta documentação | §1 |
| saber para quem escrevemos e que pergunta cada público faz | §2 |
| conhecer as regras editoriais (o que entra, o que não entra) | §3 |
| ver a árvore completa, arquivo por arquivo | §4 |
| entender o papel de cada pasta | §5 |
| ver o conteúdo obrigatório de cada arquivo | §6 |
| saber o formato fixo de cada tipo de página | §7 |
| saber como o material é levantado e revisado | §8 |
| conhecer os identificadores (`RF-DB`, `ADR-DB`, `WYDDB`…) | §9 |
| saber quando uma página está pronta | §10 |
| ver a ordem de produção por onda | §11 |
| aprovar ou recusar decisões | §12 |

---

## 1. Objetivo, escopo e limites

### 1.1 Objetivo

Produzir a documentação **final** do `wyd-db` — o componente de persistência do WYD em
PostgreSQL 17 e Rust — de modo que:

1. **requisitos** (funcionais, regras de negócio, não funcionais) estejam escritos e rastreáveis
   até a tabela, a porta e o teste que os atendem;
2. **arquitetura e modelo de dados** possam ser entendidos e alterados por alguém que nunca viu o
   projeto, sem ler o código primeiro;
3. **cada tabela e cada coluna** tenham explicação verificável, sem que essa explicação seja uma
   segunda cópia do schema;
4. **desenvolvedor, administrador, testador e operação** tenham guias próprios, com comandos que
   funcionam;
5. a documentação seja **publicável** como site (mkdocs) sem reescrita.

### 1.2 Escopo da `1.0.0`

| Dentro | Fora |
|---|---|
| a série de migrations **reescrita do zero** para a `1.0.0`, com a identidade em `0001_bootstrap.sql` (schemas + `content_version`) e `0003_portal_person.sql` (pessoa), e os demais domínios escritos inteiros na mesma série: **32 arquivos**, **97 tabelas** e `SCHEMA_VERSION = 32` na troca — **planejado** em 2026-11-17 e revisto em 2026-11-21 (`0032`), dono único `plano-migrations.md` §3 | desenho de domínio que a identidade não decide: personagem, inventário, economia, conteúdo |
| o crate `crates/wyd-db` (portas, adapter, pool, migrations runner, filas) | `wyd-core`, `wyd-server`, `wyd-net`, `wyd-admin`, portal |
| o seed de desenvolvimento (`seeds/dev.sql`) | dados de jogador reais |
| operação do banco (instalação, backup, retenção, diagnóstico) | operação do jogo e do portal |
| requisitos e regras que o banco precisa cumprir | regra de jogo implementada no tick |
| documentação de teste do crate (contrato e sistema) | harness E2E de cliente/bots |

**Série reescrita:** a `1.0.0` não documenta uma série herdada — ela define a série escrita do zero. Cada arquivo é escrito inteiro, uma vez, criando as tabelas completas do seu domínio; nenhum arquivo altera tabela criada por outro. A decisão e o desenho estão em `plano-migrations.md` §3 e em `padrao-banco.md` §11 (a ADR da fronteira
pessoa × conta ainda não tem número).

**Fronteira de honestidade:** o que não for verificável em código, banco, comando ou artefato
capturado é declarado **hipótese** e marcado como tal. Documento não promove hipótese a regra;
regra nova só entra com decisão registrada (§9.3).

### 1.3 O que esta documentação **não** é

- Não é changelog nem diário de execução — isso é `planos/registro-de-execucao.md`, uma linha por
  entrega, sem narrativa.
- Não é tutorial de instalação do jogo (é do administrador do banco, não do jogador).
- Não é cópia comentada das migrations: o estado atual do schema é lido no catálogo **gerado**.
- Não é material de cliente/rede/protocolo: essas áreas têm documentação própria no repositório.

---

## 2. Público e perguntas

| Público | Pergunta que ele faz | Onde a resposta mora |
|---|---|---|
| **Desenvolvedor novo** | "como este crate é organizado e como eu adiciono uma migration sem quebrar nada?" | `guia-desenvolvedor/` |
| **Desenvolvedor de outro crate** | "qual porta eu uso e o que ela garante?" | `guia-desenvolvedor/portas-e-contratos.md` |
| **Arquiteto** | "por que o banco é assim e onde estão as fronteiras?" | `arquitetura/` + `arquitetura/adr/` |
| **DBA / operação** | "como faço backup, restauro, diagnostico lentidão e controlo retenção?" | `guia-administrador/` |
| **QA / testador** | "o que precisa ser provado, com que dado e em qual alvo?" | `guia-teste/` + `tests/` |
| **Produto / gestão** | "quais requisitos existem, o que falta e o que está em risco?" | `specs/` + `planos/` + `roadmap/` |
| **Auditor / segurança** | "onde ficam segredos, quem escreve o quê, o que é auditado?" | `arquitetura/seguranca.md` + `guia-administrador/configuracao.md` |
| **Agente de IA** | "qual o procedimento exato para esta tarefa?" | `prompts/` + `skills/` |

Regra: **nenhuma página existe sem um público e uma pergunta**. Página que não responde a uma
linha desta tabela não é escrita.

---

## 3. Regras editoriais

1. **Uma informação, um dono.** O fato mora em um arquivo; os demais apontam por link. Se duas
   páginas puderem divergir sobre o mesmo número, uma delas está errada por construção.
2. **Número só com fonte e data.** Toda contagem cita o comando ou o arquivo que a produziu e a
   data da medição. Número sem fonte é hipótese e leva a marca `H`.
3. **Schema não se escreve duas vezes.** Coluna, tipo, nulidade, PK, FK, CHECK e índice vivem no DDL
   de `ddl-confirmado.md` e na página de `migrations/` do arquivo dono — hoje a única fonte da forma, já
   que o catálogo não existe. As páginas por tabela (`M26`) **resumem e ligam**, não copiam o DDL:
   **66 das 97** já estão escritas à mão, em lotes (`0001`–`0012`, `0013`, `0014`, `0019` e `0024`), até 2026-11-22. Texto manual sobre
   coluna é dívida declarada e só é aceito no campo "observação"
   (regra de negócio, provisório, armadilha).
4. **Nível de evidência em toda afirmação de comportamento.** `V` verificado por comando/arquivo,
   `H` hipótese explícita, `P` planejado (ainda não existe). Sem `V`, o verbo é "pretende",
   "deveria", nunca "funciona".
5. **Hipótese não vira requisito.** Campo de semântica desconhecida fica marcado `H` e a página diz
   o que falta para fechar.
6. **Português do Brasil, `snake_case` em identificador técnico, Mermaid em todo fluxo.** Diagrama
   que não renderiza no mkdocs é defeito, não estilo.
7. **Sem duplicação de árvore de código.** Nada de espelhar arquivo `.rs` um a um: página por
   unidade de responsabilidade (§6.16), com contrato e invariantes, não tradução de código.
   **Exceção declarada:** `src/migrate/` e `src/provision/` têm uma página por arquivo de código
   (§6.16.1 e §6.16.2), porque cada arquivo é uma unidade implementável isolada — parser, porta,
   adapter — e o dono pediu que a implementação saia só da documentação.
8. **Sem conteúdo de enfeite.** Proibido: seção "Introdução" que repete o título, tabela de
   conteúdo manual (o mkdocs gera), placeholder `TODO`, captura de tela sem texto alternativo,
   "em construção", adjetivo sem medida ("rápido", "robusto").
9. **Nada de código, migration, script ou teste dentro de `planejamento-1.0.0/wyd-db/`.** Esta
   árvore é documentação; artefato executável nasce em `crates/` ou `tools/`.
10. **Segredo nunca aparece.** DSN, token, hash, senha, PIN e chave são descritos pelo **tipo** e
    pelo **lugar**, jamais pelo valor.
11. **A base de análise é contexto, não fonte.** Ela orienta a pesquisa (§8.4), mas nenhuma frase,
    número ou conclusão entra na documentação sem reverificação na fonte de §8.1. O rastro fica no
    campo `Base:` do cabeçalho (§7.1), que é interno e não é link.

### 3.1 Fronteiras entre páginas que se parecem

Onze pares correm risco de dizer a mesma coisa duas vezes. O dono é sempre o primeiro; o segundo
aponta e não repete.

| Par | Dono | O outro faz |
|---|---|---|
| `arquitetura/migracoes.md` × `migrations/serie-e-regras.md` | arquitetura: **por que** a série é assim | operação: ordem, comandos, verificação |
| `arquitetura/retencao-e-capacidade.md` × `guia-administrador/manutencao-rotineira.md` | arquitetura: política de dado | rotina, vacuum e registro do que foi feito |
| `arquitetura/fluxo-de-dados.md` × `specs/fluxos/*` | arquitetura: mapa geral de dados | sequência de um caso, passo a passo |
| `arquitetura/padroes-postgres.md` × `arquitetura/integridade-e-constraints.md` | padrões: tipo, índice, concorrência | integridade: FK, CHECK, unicidade |
| `guia-desenvolvedor/como-adicionar-tabela.md` × os dois acima | guia: passo a passo | padrão e política |
| `guia-desenvolvedor/portas-e-contratos.md` × `src/repo-*.md` | src: garantia de cada método | guia: qual porta usar e por quê |
| `guia-teste/piramide-e-alvos.md` × `planos/plano-de-testes.md` | plano: escopo e critério de saída | guia: como escolher e escrever |
| `guia-teste/fixtures-e-factories.md` × `tests/fixtures-e-factories.md` | tests: inventário real | guia: regra e padrão |
| `guia-teste/dados-de-teste.md` × `tests/dados-de-teste.md` | tests: catálogo concreto | guia: política e critério |
| `guia-teste/testes-de-contrato.md` × `tests/alvos/contrato-*.md` | alvo: o que aquele alvo prova | guia: como escrever o teste |
| `convencoes.md` × `guia-documentacao/*` | convenções: como **ler** uma página | guia: como **escrever** e publicar |

---

## 4. Árvore completa

Legenda de estado: `▲` a escrever · `◆` gerado por script · `■` dado de apoio (JSON/SQL de teste).

```text
planejamento-1.0.0/wyd-db/
├── wyd-db-1.0.0.md                              ▲ este plano diretor
├── docs/
│   ├── index.md                                 ▲ porta de entrada
│   ├── convencoes.md                            ▲ regras editoriais e marcações
│   ├── glossario.md                             ▲ vocabulário do domínio
│   ├── arquitetura/
│   │   ├── index.md                             ▲
│   │   ├── visao-geral.md                       ▲
│   │   ├── contexto-c4.md                       ▲
│   │   ├── componentes-c4.md                    ▲
│   │   ├── implantacao.md                       ▲
│   │   ├── fronteira-hot-cold.md                ▲
│   │   ├── fluxo-de-dados.md                    ▲
│   │   ├── modelo-conceitual.md                 ▲
│   │   ├── modelo-logico.md                     ▲
│   │   ├── integridade-e-constraints.md         ▲
│   │   ├── padroes-postgres.md                  ▲
│   │   ├── migracoes.md                         ▲
│   │   ├── retencao-e-capacidade.md             ▲
│   │   ├── seguranca.md                         ▲
│   │   └── adr/
│   │       ├── adr-db-0001-provisionamento-de-papel-fora-da-serie.md   ▲
│   │       ├── adr-db-0002-runner-de-migrations-proprio.md             ▲
│   │       └── index.md                                                ▲
│   ├── specs/
│   │   ├── index.md                             ▲
│   │   ├── visao-e-escolpo.md                   ▲
│   │   ├── stakeholders.md                      ▲
│   │   ├── requisitos-funcionais.md             ▲
│   │   ├── regras-de-negocio.md                 ▲
│   │   ├── requisitos-nao-funcionais.md         ▲
│   │   ├── casos-de-uso.md                      ▲
│   │   ├── restricoes-e-premissas.md            ▲
│   │   ├── rastreabilidade.md                   ▲
│   │   ├── fluxos/
│   │   │   ├── login-e-sessao.md                ▲
│   │   │   ├── criacao-e-selecao-de-personagem.md ▲
│   │   │   ├── carregamento-e-save.md           ▲
│   │   │   ├── concessao-de-grant.md            ▲
│   │   │   ├── compra-e-credito.md              ▲
│   │   │   └── carga-de-conteudo.md             ▲
│   │   └── estados/
│   │       ├── conta-e-moderacao.md             ▲
│   │       ├── personagem.md                    ▲
│   │       ├── sessao.md                        ▲
│   │       ├── grant.md                         ▲
│   │       └── item-e-refino.md                 ▲
│   ├── dicionario-dados/
│   │   ├── index.md                             ▲
│   │   ├── como-ler.md                          ▲
│   │   ├── dominio-acesso.md                    ▲
│   │   ├── dominio-personagem.md                ▲
│   │   ├── dominio-inventario.md                ▲
│   │   ├── dominio-social.md                    ▲
│   │   ├── dominio-mundo.md                     ▲
│   │   ├── dominio-catalogo.md                  ▲
│   │   ├── dominio-combate.md                   ▲
│   │   ├── dominio-refino.md                    ▲
│   │   ├── dominio-economia.md                  ▲
│   │   ├── dominio-quests.md                    ▲
│   │   ├── dominio-eventos.md                   ▲
│   │   ├── dominio-arena.md                     ▲
│   │   ├── dominio-progressao.md                ▲
│   │   ├── dominio-canal.md                     ▲
│   │   ├── dominio-moderacao.md                 ▲
│   │   ├── dominio-auditoria.md                 ▲
│   │   ├── dominio-configuracao.md              ▲
│   │   ├── dominio-ponte-web-jogo.md            ▲
│   │   └── gerado/
│   │       ├── index.md                         ◆
│   │       └── <tabela>.md  (uma por tabela: 97) ◆
│   ├── planos/
│   │   ├── index.md                             ▲
│   │   ├── plano-de-documentacao.md             ▲
│   │   ├── plano-de-testes.md                   ▲
│   │   ├── plano-de-implantacao.md              ▲
│   │   ├── plano-de-dados-e-seeds.md            ▲
│   │   ├── plano-de-manutencao.md               ▲
│   │   ├── plano-de-qualidade-e-gates.md        ▲
│   │   ├── metricas.md                          ▲
│   │   ├── riscos.md                            ▲
│   │   └── registro-de-execucao.md              ▲
│   ├── roadmap/
│   │   ├── index.md                             ▲
│   │   ├── fases.md                             ▲
│   │   └── WYDDB-001..WYDDB-040.md              ▲
│   ├── guia-desenvolvedor/
│   │   ├── index.md                             ▲
│   │   ├── setup-ambiente.md                    ▲
│   │   ├── mapa-do-crate.md                     ▲
│   │   ├── portas-e-contratos.md                ▲
│   │   ├── modelos-e-invariantes.md             ▲
│   │   ├── pool-e-transacoes.md                 ▲
│   │   ├── como-adicionar-migration.md          ▲
│   │   ├── como-adicionar-tabela.md             ▲
│   │   ├── como-escrever-teste.md               ▲
│   │   ├── erros-e-logs.md                      ▲
│   │   └── checklist-de-pr.md                   ▲
│   ├── guia-administrador/
│   │   ├── index.md                             ▲
│   │   ├── instalacao.md                        ▲
│   │   ├── configuracao.md                      ▲
│   │   ├── backup-e-restore.md                  ▲
│   │   ├── diagnostico.md                       ▲
│   │   ├── manutencao-rotineira.md              ▲
│   │   ├── recuperacao-de-desastre.md           ▲
│   │   └── sustentacao.md                       ▲
│   ├── guia-teste/
│   │   ├── index.md                             ▲
│   │   ├── piramide-e-alvos.md                  ▲
│   │   ├── fixtures-e-factories.md              ▲
│   │   ├── dados-de-teste.md                    ▲
│   │   ├── testes-de-contrato.md                ▲
│   │   ├── testes-de-sistema.md                 ▲
│   │   └── gates-e-anti-skip.md                 ▲
│   └── guia-documentacao/
│   │   ├── index.md                             ▲
│   │   ├── estrutura-e-navegacao.md             ▲
│   │   ├── estilo-e-linguagem.md                ▲
│   │   ├── markdown-e-mkdocs.md                 ▲
│   │   ├── diagramas.md                         ▲
│   │   ├── links-referencias-e-ids.md           ▲
│   │   ├── imagens-e-evidencias.md              ▲
│   │   └── revisao-e-publicacao.md              ▲
├── migrations/
│   ├── index.md                                 ▲
│   ├── serie-e-regras.md                        ▲
│   └── 0001-….md … 0029-….md  (29)  ▲
├── src/
│   ├── index.md                                 ▲
│   ├── 24 páginas de módulo (raiz, model, pool, repo, transfer) ▲
│   ├── migrate/  (21 páginas, uma por arquivo)  ▲
│   └── provision/ (13 páginas, uma por arquivo) ▲
├── tests/
│   ├── index.md                                 ▲
│   ├── fixtures-e-factories.md                  ▲
│   ├── dados-de-teste.md                        ▲
│   ├── resources/
│   │   ├── README.md                            ▲
│   │   └── accounts.json                        ■
│   ├── alvos/
│   │   ├── contrato-catalogo.md                 ▲
│   │   ├── contrato-migrations.md               ▲
│   │   ├── sistema-pg.md                        ▲
│   │   ├── sistema-sessoes.md                   ▲
│   │   └── sistema-stack-caps.md                ▲
│   └── specs/
│       ├── contrato.md                          ▲
│       ├── sistema.md                           ▲
│       └── capacidades.md                       ▲
├── prompts/
│   ├── README.md                                ▲
│   ├── plan/
│   │   ├── analise-de-requisito.md              ▲
│   │   └── planejamento-de-issue.md             ▲
│   ├── execution/
│   │   ├── execucao-de-issue.md                 ▲
│   │   └── escrita-de-migration.md              ▲
│   ├── review/
│   │   ├── revisao-de-codigo.md                 ▲
│   │   └── revisao-de-documentacao.md           ▲
│   └── test/
│       ├── execucao-de-testes.md                ▲
│       └── revisao-de-testes.md                 ▲
└── skills/
    ├── README.md                                ▲
    ├── plan/levantar-requisito/SKILL.md         ▲
    ├── plan/planejar-onda/SKILL.md              ▲
    ├── execution/implementar-issue/SKILL.md     ▲
    ├── execution/escrever-migration/SKILL.md    ▲
    ├── review/revisar-schema/SKILL.md           ▲
    ├── review/revisar-documentacao/SKILL.md     ▲
    ├── test/provar-schema/SKILL.md              ▲
    └── test/provisionar-dados-de-teste/SKILL.md ▲
```

**Totais:** 1 plano diretor · 3 páginas de raiz em `docs/` · 17 em `arquitetura/` · 20 em
`specs/` · 21 autorais + 97 de tabela em `dicionario-dados/` (escritas à mão, `M26`) · 10 em `planos/` · 42 em `roadmap/` ·
11 em `guia-desenvolvedor/` · 8 em `guia-administrador/` · 7 em `guia-teste/` · 8 em
`guia-documentacao/` · 32 em `migrations/` · 59 em `src/` · 12 em `tests/` · 9 em `prompts/` · 9 em `skills/`
→ **teto de páginas autorais + 97 de tabela + 1 dado de apoio**, mais este plano diretor.

**Fonte dos números (2026-11-18):** os totais que **derivam da série** — **97 páginas de tabela** (uma
por tabela, escritas à mão desde `M26`) e **34 em `migrations/`** (2 páginas de seção + 32 arquivos) — são **planejados** e vêm de
`plano-migrations.md` §3, dono único da contagem. O **teto autoral** e o número **medido** são de
`planos/metricas.md`, escrito só depois que a série existir. Nenhum total desta seção é medição.

**Teto × escrito (2026-11-21):** os totais acima contam páginas **planejadas** e já incluem as
desta árvore — escrever uma página **não muda nenhum deles**; muda o estado no cabeçalho e a
medição de `planos/metricas.md`. **Medido:** **36 páginas com corpo em `docs/`**, todas com o
cabeçalho de §7.1 — as **3 de raiz** (`index`, `convencoes`, `glossario`), **7 em `arquitetura/`**
(`migracoes`, `seguranca`, `adr/index`, `adr/adr-db-0001`, `adr/adr-db-0002`, `padroes-postgres`,
`integridade-e-constraints`), **21 em `dicionario-dados/`** (`index`, `como-ler` e as 19 de domínio),
`planos/plano-de-implantacao.md`, `planos/riscos.md`, **2 em `guia-desenvolvedor/`**
(`como-adicionar-migration`, `como-adicionar-tabela`) e `guia-teste/testes-de-sistema.md`; eram **15**
em 2026-11-18. Os dois índices que não tinham cabeçalho — `docs/index.md` e `arquitetura/adr/index.md`
— receberam o bloco de §7.1 na revisão de 2026-11-18; antes traziam só o título. Os outros **8
arquivos `index.md` são título de uma linha** e não contam como página (§2);
`dicionario-dados/gerado/index.md` é o índice auxiliar das páginas por tabela, também fora da
contagem. Fora de `docs/`: **26 de 32** em `migrations/` e **0 de 59** em `src/` — os dois `spec.md`
de `src/migrate/` e `src/provision/` são desenho de origem, não página da árvore.

**O que dá para escrever agora, página a página:** `proposta-paginas.md` (proposta de 2026-11-18)
mapeia as **147** páginas autorais de `docs/` em **18 escrevíveis**, **24 de recorte** e **69
bloqueadas** — eram 146, 23 e 84 na medição de 2026-11-18, antes de as **19 páginas de domínio**
ficarem escritas (15 delas estavam na lista de bloqueadas) —, com a fonte de cada uma e o
mapa de diluição das seis fontes de planejamento. O
**lote 1** dela (padrão) está escrito: os três títulos contam nas 36 acima.

---

## 5. Papel de cada pasta

Cada pasta tem um dono, uma pergunta e uma fronteira. O que não cabe na fronteira não entra nela
"porque é conveniente": vai para a pasta dona do assunto.

### 5.1 `docs/` — documentação oficial

- **Pergunta:** "o que o `wyd-db` é, o que ele precisa cumprir e como se trabalha com ele?"
- **Dono:** equipe do `wyd-db`.
- **Fronteira:** só conteúdo aprovado entra. Rascunho com dúvida aberta fica marcado `H` e cita a
  decisão pendente em `specs/restricoes-e-premissas.md`.
- **Publicação:** é a fonte do site mkdocs. Nenhum link sai de `docs/` para `migrations/`, `src/`,
  `tests/`, `prompts/` ou `skills/` sem a marca "planejamento" — o site não publica essas pastas.

Arquivos de raiz:

| Arquivo | Responde | Conteúdo obrigatório |
|---|---|---|
| `index.md` | "por onde começo?" | objetivo da doc em 5 linhas, tabela de público (§2), mapa das 10 subpastas em grade de cartões com estado; trilhas em subtítulos; 2 links de ação; emoji por seção e cor por tipo de aviso; 3 links de entrada (requisitos, dicionário, guia do dev) |
| `convencoes.md` | "como leio estas páginas?" | marcações `V`/`H`/`P`, estados de tabela (`pronto`/`provisório`/`planejado`), formato de número com fonte e data, regra de link, padrão de diagrama, o que é proibido (§3) |
| `glossario.md` | "o que significa este termo?" | um verbete por termo de domínio (conta, personagem, slot, refino, Anct, gema, SANC, grant, canal, delivery queue, sessão única, catálogo, conteúdo estático), com definição de 1–2 linhas e link para a página dona |

### 5.2 `docs/arquitetura/` — como o banco e o crate são desenhados

- **Pergunta:** "por que é assim, onde estão as fronteiras e o que acontece se eu mexer?"
- **Fronteira:** desenho e decisão. Regra de negócio fica em `specs/`; operação fica em
  `guia-administrador/`; coluna fica no dicionário.
- **Regra:** toda página de arquitetura tem pelo menos um diagrama Mermaid e uma seção "o que
  quebra se mudar".

Ver §6.2–§6.4.

### 5.3 `docs/specs/` — requisitos e especificações

- **Pergunta:** "o que o sistema precisa fazer, sob quais regras e com que qualidade?"
- **Fronteira:** comportamento exigido, não desenho de tabela. Cada requisito tem ID, critério de
  aceitação verificável e destino na matriz de rastreabilidade.
- **Proibido:** requisito sem critério de aceitação; caso de uso sem fluxo alternativo; regra de
  negócio sem exemplo numérico quando a regra for de cálculo.

Ver §6.5–§6.7.

### 5.4 `docs/dicionario-dados/` — o que cada tabela significa

- **Pergunta:** "para que serve esta tabela, quem escreve, o que é provisório e o que ela exige?"
- **Fronteira:** significado e regra. A **forma** (coluna, tipo, nulidade, índice) é dona das páginas
  de `migrations/`, que trazem o DDL e a tabela de colunas de cada tabela; a página por tabela, se
  existir, resume e aponta para elas.
- **Duas camadas:** 19 páginas por domínio (escritas, com contexto e decisão) + as 97 páginas por
  tabela (`plano-migrations.md` §3), **escritas à mão em lotes** (`M26`, decidida em 2026-11-21);
  nenhuma ferramenta é usada e nada é gerado.

Ver §6.8, onde estão o molde da página manual por tabela e a contagem das já escritas.

### 5.5 `docs/planos/` — como o trabalho é planejado e medido

- **Pergunta:** "o que vamos fazer, em que ordem, como medimos e o que pode dar errado?"
- **Fronteira:** plano e medida. Execução do dia a dia fica em `registro-de-execucao.md`, uma
  linha por entrega.
- **Regra:** `metricas.md` é o **dono único dos números** do planejamento. Nenhuma outra página
  recalcula contagem: ela cita `metricas.md` (ou o comando que gera o número).

Ver §6.9.

### 5.6 `docs/roadmap/` — o trabalho fatiado em issues

- **Pergunta:** "qual é a próxima tarefa, o que ela entrega e como sei que terminou?"
- **Fronteira:** uma página por issue `WYDDB-nnn`, com DoD verificável. Prioridade e fase ficam em
  `fases.md` e em `planos/metricas.md`; issue não repete contagem.
- **Regra:** issue só existe se seu DoD citar comando, arquivo ou documento de saída.

Ver §6.10.

### 5.7 `docs/guia-desenvolvedor/` — como trabalhar no crate

- **Pergunta:** "como eu mexo nisto sem quebrar?"
- **Fronteira:** procedimento e padrão. Referência de coluna fica no dicionário; API detalhada
  fica em `src/`.
- **Regra:** todo comando do guia é copiável e foi executado; todo "faça X" tem o "por que" em uma
  linha.

Ver §6.11.

### 5.8 `docs/guia-administrador/` — como manter o banco vivo

- **Pergunta:** "como instalo, configuro, faço backup, restauro e diagnostico?"
- **Fronteira:** operação do banco. Regra de jogo, desenho de tabela e deploy do portal não entram.
- **Regra:** cada procedimento tem "antes", "passos", "como confirmar", "como reverter" e "o que
  fazer se falhar".

Ver §6.12.

### 5.9 `docs/guia-teste/` — como se prova o banco

- **Pergunta:** "o que preciso provar, com que dado e em qual alvo?"
- **Fronteira:** estratégia e padrão de teste. O detalhe de cada cenário fica em `tests/`.
- **Regra:** nenhum teste sem capacidade associada; nenhuma capacidade sem teste; nada de `SKIP`
  silencioso.

Ver §6.13.

### 5.10 `docs/guia-documentacao/` — como se escreve esta documentação

- **Pergunta:** "como crio, escrevo, diagramo, linko e publico uma página deste projeto?"
- **Fronteira:** padrão de escrita e uso do mkdocs. Regra de negócio não entra: ela vive em
  `specs/`. Regra de **leitura** (as marcações `V`/`H`/`P`) fica em `convencoes.md`, que este guia
  cita em vez de repetir.
- **Regra:** todo padrão tem exemplo certo e exemplo errado; e só se documenta recurso que o site
  do projeto realmente habilita — recurso que não renderiza não entra.

Ver §6.14.

### 5.11 `migrations/` — o histórico aplicável do schema

- **Pergunta:** "o que cada migration cria, insere e como reverte?"
- **Fronteira:** intenção e rollback de cada arquivo. O estado atual do schema **não** é descrito
  aqui: está no catálogo gerado. Não há SQL nesta pasta.
- **Regra:** uma página por migration, ordenada por número; `serie-e-regras.md` é a única que
  descreve a política da série (ordem, `up`/`down`, DDL antes de seed, numeração contígua).

Ver §6.15.

### 5.12 `src/` — contrato de cada módulo do crate

- **Pergunta:** "o que este módulo garante, o que ele expõe e o que ele **não** faz?"
- **Fronteira:** contrato e invariante. Tradução de código, linha a linha, **não**. Arquivos
  `mod.rs` (reexport) não têm página própria: aparecem no `index.md` da pasta.
- **Regra:** toda página declara "responsabilidade", "API pública", "invariantes", "quem depende",
  "testes que cobrem" e "riscos de alteração".

Ver §6.16.

### 5.13 `tests/` — o plano da suíte do crate

- **Pergunta:** "como a suíte está organizada, que recurso ela compartilha e o que cada alvo
  prova?"
- **Fronteira:** organização, fixtures e alvos do crate. E2E de cliente/bots não pertence aqui.
- **Regra:** um arquivo por alvo de teste do `Cargo.toml`; nenhuma fixture com id mágico; nenhuma
  espera por `sleep`.

Ver §6.17.

### 5.14 `prompts/` e `skills/` — procedimento para agente

- **Pergunta:** "qual é o procedimento exato, com entrada, passos e critério de parada?"
- **Fronteira:** instrução operacional do projeto. Regra de negócio não entra: ela vive em
  `specs/` e é citada.
- **Regra:** prompt descreve **o que pedir**; skill descreve **como executar**. Toda skill tem
  "quando usar", "pré-requisitos", "passos", "como verificar", "quando parar e perguntar".

Ver §6.18 e §6.19.

---

## 6. Conteúdo obrigatório de cada arquivo

Formato: **pergunta que responde** e **seções obrigatórias**. Seções além destas são permitidas;
faltar uma delas reprova a página (§10).

### 6.1 `docs/` raiz

Já descrito em §5.1.

### 6.2 `docs/arquitetura/index.md`

- Pergunta: "qual página de arquitetura eu leio para a minha dúvida?"
- Obrigatório: mapa das 13 páginas por pergunta; ordem de leitura sugerida (visão → C4 →
  fronteira → fluxo → modelo → padrões); estado de cada uma; 1 diagrama de navegação.

### 6.3 `docs/arquitetura/*.md`

| Arquivo | Pergunta | Seções obrigatórias |
|---|---|---|
| `visao-geral.md` | "o que é o `wyd-db` em 10 minutos?" | propósito; o que ele guarda e o que não guarda; tecnologias (PostgreSQL 17, `sqlx`, Tokio); 3 números com fonte e data (migrations, tabelas, alvos de teste); diagrama de contexto |
| `contexto-c4.md` | "quem usa o `wyd-db` e com o que ele conversa?" | C4 nível 1 e 2; atores (servidor de jogo, gateway/admin, portal, operação); o que cada um pode ler/escrever; o que o banco **não** decide |
| `componentes-c4.md` | "quais são as peças internas?" | C4 nível 3 com `pool`, `migrate`, `model`, `repo` (portas) e `transfer`; tabela componente → responsabilidade → dependência permitida; o que cada componente não pode importar |
| `implantacao.md` | "como isso roda?" | C4 nível 4; processo, imagem, portas, volume, supervisor; ligação com `docs/rust/05-stack-docker.md` do repositório (sem duplicar) |
| `fronteira-hot-cold.md` | "onde termina o tick e começa o banco?" | regra de ouro (sem I/O no tick); caminho de escrita do jogo; caminho de escrita da web; `delivery_queue` como única ponte; diagrama; o que quebra se violar |
| `fluxo-de-dados.md` | "que dados entram, mudam e saem?" | diagrama de contexto de dados; tabela origem → caminho → destino; dado efêmero × persistente × derivado; ponto de perda possível |
| `modelo-conceitual.md` | "quais entidades existem e como se relacionam?" | ER conceitual por domínio (18), sem coluna; cardinalidades; agregados (conta, personagem, clã, conteúdo); glossário de relação |
| `modelo-logico.md` | "como o conceitual vira tabela?" | ER lógico com nomes de tabela; PK/FK por relação; tabela 1:1, 1:N e N:N; o que é derivado e por quê; aponta para `gerado/` para coluna |
| `integridade-e-constraints.md` | "que regra o banco garante sozinho?" | política de FK (cascade × restrict × sem FK); CHECK por domínio; unicidade; DEFERRABLE; o que é garantido em código e não no banco, e por quê; teste que prova cada grupo |
| `padroes-postgres.md` | "qual é o padrão de modelagem?" | **tipo**: o critério que decide a escolha, citando a lista fechada de `padrao-banco.md` §4; **índice**: o critério que responde quando criar e quando não (invariante × desempenho), citando §6; **concorrência**: transação, versão otimista e `SKIP LOCKED`, cada uma com decisão registrada ou ausência declarada (`H`); **armadilhas**: as famílias de defeito, citando a lista fechada de §11; checklist de revisão de DDL. Citar o dono não dispensa preencher: a seção que só aponta, sem o critério, reprova (§10) |
| `migracoes.md` | "por que a série é assim?" | decisão e desenho: por que `up`/`down` no mesmo arquivo, por que DDL consolidado sem `ALTER`, por que ledger com lock e checksum, o que a série garante; diagrama das camadas de validação. Regras operacionais, comandos e o que fazer em produção ficam em `migrations/serie-e-regras.md` |
| `retencao-e-capacidade.md` | "o que cresce e o que fazer com isso?" | por tabela de crescimento: volume esperado, política de retenção, partição, custo de vacuum; tabela quente × histórica; alerta proposto com limiar |
| `seguranca.md` | "quem pode o quê e o que é auditado?" | papéis do banco (runtime × migração); o que é hash e com que algoritmo; o que nunca é gravado; o que é auditado e onde; ameaça considerada × controle; aponta `guia-administrador/configuracao.md` |

### 6.4 `docs/arquitetura/adr/`

`index.md`: tabela `ADR-DB-nnn | decisão | estado | data | substitui`. Arquivo:
`adr-db-000N-<slug>.md`, numerado na ordem em que a decisão é tomada. Cada ADR segue o mesmo
formato de 5 seções: **contexto**, **decisão**, **consequências** (boas e ruins), **alternativas
descartadas** (com o motivo), **como verificar**. Um ADR só é aceito com "como verificar"
executável.

**Não existe catálogo prévio de ADRs.** A lista cresce uma por decisão tomada, não por
planejamento: hoje há `adr-db-0001-provisionamento-de-papel-fora-da-serie.md` e o `ADR-DB-0002` do
runner, ambos `aceito` (`src/migrate/spec.md` §14). Doze decisões escritas antes de decidir cada uma
seriam intenção, não decisão — e o número muda quando a decisão muda. O
acompanhamento de estado fica em §12.2; a marcação, em `docs/convencoes.md`.

### 6.5 `docs/specs/*.md`

| Arquivo | Pergunta | Seções obrigatórias |
|---|---|---|
| `index.md` | "o que há de requisito aqui?" | mapa das 8 páginas; convenção de ID; estado por área; como um requisito nasce, muda e é retirado |
| `visao-e-escolpo.md` | "o que o `wyd-db` resolve?" | problema; escopo dentro/fora (tabela); premissas de negócio; o que a `1.0.0` entrega |
| `stakeholders.md` | "quem pede o quê?" | ator → interesse → requisito que o atende; quem aprova mudança de requisito |
| `requisitos-funcionais.md` | "o que o banco precisa fazer?" | um bloco por `RF-DB-nnn` com: enunciado, entrada, saída, critério de aceitação verificável, tabela/porta envolvida, issue que entrega |
| `regras-de-negocio.md` | "que regra vale sempre?" | um bloco por `RN-DB-nnn`: enunciado, exemplo numérico, onde é garantida (banco × código), o que acontece na violação |
| `requisitos-nao-funcionais.md` | "com que qualidade?" | um bloco por `RNF-DB-nnn` com métrica, limiar, como medir, condição de reprovação (desempenho, integridade, disponibilidade, segurança, capacidade, retenção) |
| `casos-de-uso.md` | "quem faz o quê, passo a passo?" | um bloco por `UC-DB-nnn`: ator, pré-condição, fluxo principal, fluxos alternativos, pós-condição, requisitos cobertos; diagrama de casos de uso |
| `restricoes-e-premissas.md` | "o que assumimos e o que nos limita?" | restrição técnica/legal/operacional; premissa com o que a invalida; decisão pendente com dono e prazo |
| `rastreabilidade.md` | "quem atende este requisito?" | matriz `RF/RN/RNF/UC → tabela → porta → teste → issue`; itens sem cobertura listados explicitamente; regra de atualização |

### 6.6 `docs/specs/fluxos/*.md`

Um arquivo por fluxo, sempre com: ator, gatilho, diagrama de sequência (Mermaid), passo a passo
com o dado que muda em cada passo, transação/fronteira, falhas previstas e o que acontece,
tabelas tocadas.

| Arquivo | Fluxo |
|---|---|
| `login-e-sessao.md` | autenticação, criação de sessão, sessão única, encerramento |
| `criacao-e-selecao-de-personagem.md` | criação, limite de slot, seleção, recusa |
| `carregamento-e-save.md` | carga do personagem no login, alteração em memória, save em lote, fechamento |
| `concessao-de-grant.md` | pedido da web, fila, drenagem pelo servidor, confirmação, falha |
| `compra-e-credito.md` | compra, crédito idempotente, débito, reconciliação |
| `carga-de-conteudo.md` | leitura do catálogo no boot, versão de conteúdo, recarga |

### 6.7 `docs/specs/estados/*.md`

Máquina de estados com diagrama, tabela `estado | evento | guarda | próximo estado | efeito no
banco`, e estados proibidos.

| Arquivo | Entidade |
|---|---|
| `conta-e-moderacao.md` | conta: ativa, bloqueada, banida; efeito na sessão |
| `personagem.md` | personagem: criado, ativo, deletado (soft), restaurado |
| `sessao.md` | sessão: aberta, única, expirada, encerrada |
| `grant.md` | grant: pendente, aplicado, falho, reenfileirado |
| `item-e-refino.md` | item: normal, refinado, com Anct, com gema; falha de refino |

### 6.8 `docs/dicionario-dados/*.md`

**`index.md`** — pergunta: "onde está a tabela X?". Obrigatório: total de tabelas com fonte e data;
tabela `domínio → página → nº de tabelas` (número gerado, não digitado); como pedir tabela nova;
estado geral (`pronto`/`provisório`).

**`como-ler.md`** — pergunta: "o que significa cada campo da página?". Obrigatório: anatomia de uma
página de tabela; o que é gerado × o que é escrito; marcações `V`/`H`/`P`; o que fazer quando a
página gerada e a migration divergirem (prevalece a migration, e isso é defeito a corrigir).

**19 páginas de domínio** — pergunta: "quais tabelas cuidam deste assunto e como se relacionam?".
Seções obrigatórias, nesta ordem:

1. **Propósito do domínio** e o que ele não cobre.
2. **Diagrama de relações** entre as tabelas do domínio (Mermaid ER).
3. **Tabela de tabelas:** nome, papel em uma linha, quem escreve, quem lê, estado
   (`pronto`/`provisório`), link para `gerado/<tabela>.md`.
4. **Ciclo de vida do dado:** quando nasce, quando muda, quando é apagado, o que fica órfão.
5. **Regras e armadilhas:** o que o schema garante, o que só o código garante, e o que é
   `H` (semântica não verificada) com o que falta para fechar.
6. **Requisitos atendidos:** IDs `RF-DB`/`RN-DB` que dependem deste domínio.

Domínios (um arquivo cada): `acesso`, `personagem`, `inventario`, `social`, `mundo`, `catalogo`,
`combate`, `refino`, `economia`, `quests`, `eventos`, `arena`, `progressao`, `canal`, `moderacao`,
`auditoria`, `configuracao`, `ponte-web-jogo`, `conteudo-portal`.

**`gerado/<tabela>.md` (97 páginas por tabela) — escritas à mão (`M26`, 2026-11-21).** O gerador
`make dicionario-dados` saiu do plano: as páginas nascem em lotes, ao fim do DDL, como qualquer outra
página — sem ferramenta e sem `Makefile` novo. Cada uma traz papel em uma linha, colunas com
tipo/nulidade/default, PK, FKs de saída e de entrada, unicidade, CHECKs, índices, quem escreve/lê e a
migration que criou a tabela, e **liga para a página de `migrations/`**, que é dona da forma, em vez
de copiar o DDL. `gerado/index.md` lista todas com o domínio de cada uma. **66 das 97 existem**
(lotes `0001`–`0012`, `0013`, `0014`, `0019` e `0024`, 2026-11-22) e nada é gerado.

**Por que 19 páginas de domínio existem.** Elas cobrem o que a página por tabela não responde: por que
a tabela existe, o que é provisório, o que quebra se mudar. A divisão prevista é **forma na página da
migration, significado no texto**, e o leitor chega ao campo exato por um link — sem duplicar DDL em
dois lugares que podem divergir.

### 6.9 `docs/planos/*.md`

| Arquivo | Pergunta | Seções obrigatórias |
|---|---|---|
| `index.md` | "qual plano eu leio?" | mapa dos 9 planos; qual é vigente; como um plano é aprovado e revisado |
| `plano-de-documentacao.md` | "como esta documentação é produzida?" | ondas (espelho de §11), dono por pasta, entrada/saída de cada onda, critério de pronto, revisão por par |
| `plano-de-testes.md` | "como provamos o banco?" | escopo; alvos; matriz capacidade × alvo; dado necessário; ambiente; critério de saída da `1.0.0` |
| `plano-de-implantacao.md` | "como isto entra no ar?" | pré-requisitos; ordem (banco → schema → seed → serviço); janela; verificação pós-implantação; rollback; comunicação |
| `plano-de-dados-e-seeds.md` | "que dado inicial existe?" | origem de cada seed; o que é dado de jogo × configuração do dono; como atualizar; como reverter; o que nunca entra em produção |
| `plano-de-manutencao.md` | "o que fazer toda semana/mês?" | rotina (verificação, vacuum, backup, revisão de crescimento); sinais de degradação; quem faz; registro |
| `plano-de-qualidade-e-gates.md` | "quando uma mudança é aceita?" | gate por área (schema, doc, teste, migration); comando de cada gate; condição de reprovação; exceção e quem aprova |
| `metricas.md` | "quanto já está pronto?" | **dono único dos números**: painel por área, item a item, com fonte e data; fórmula do percentual; o que a métrica não mede |
| `riscos.md` | "o que pode dar errado?" | um item por risco com probabilidade, impacto, sinal de que aconteceu, antídoto, dono; risco aceito declarado |
| `registro-de-execucao.md` | "o que já foi entregue?" | uma linha por entrega: data, item, comando/arquivo, resultado. Sem narrativa |

### 6.10 `docs/roadmap/*.md`

**`index.md`** — pergunta: "o que está aberto e em que ordem?". Obrigatório: fila por prioridade
com coluna de esforço (`P`/`M`/`G`); o que está bloqueado e por quê; link para `fases.md` e
`planos/metricas.md`; regra de que `FASE` agrupa escopo e `P-n` ordena prioridade (não são
sinônimos).

**`fases.md`** — pergunta: "como o trabalho é agrupado?". Obrigatório: 7 fases com escopo, pré-
requisito, critério de saída e issues que contém; diagrama de dependência entre fases.

**`WYDDB-nnn.md`** (40 páginas) — pergunta: "o que esta issue entrega?". Formato fixo:
`título`, `fase`, `prioridade`, `contexto` (2–4 linhas), `entregável`, `DoD` (verificável, com
comando/arquivo), `fora de escopo`, `risco`, `depende de`, `evidência ao fechar`. Issue sem DoD
executável não entra na fila.

Distribuição inicial das 40 issues por fase (a numeração é criada na Onda 7, em ordem de execução):

| Fase | Escopo | Issues |
|---|---|---|
| 1 | higiene e baseline (doc comment, contrato, comentários de modelo) | 4 |
| 2 | contratos e erros tipados | 6 |
| 3 | decomposição de portas e separação de SQL | 6 |
| 4 | filas, save e shutdown | 5 |
| 5 | observabilidade e diagnóstico | 5 |
| 6 | testes de lacuna e harness | 8 |
| 7 | migrations e evolução de schema | 6 |

### 6.11 `docs/guia-desenvolvedor/*.md`

| Arquivo | Pergunta | Seções obrigatórias |
|---|---|---|
| `index.md` | "por onde começo como dev?" | trilha de leitura; pré-requisitos; 3 comandos que provam que o ambiente está ok |
| `setup-ambiente.md` | "como preparo a máquina?" | ferramentas e versão; banco local; variáveis de ambiente (nomes, não valores); como rodar a suíte; erros comuns de setup |
| `mapa-do-crate.md` | "o que é cada pasta do código?" | árvore comentada; regra de dependência entre módulos; o que é domínio × infraestrutura; o que não pode ser importado por quem |
| `portas-e-contratos.md` | "que porta eu uso?" | uma seção por porta: propósito, métodos, garantias, erros possíveis, quem implementa, quem chama; tabela de decisão "preciso de X → uso Y" |
| `modelos-e-invariantes.md` | "o que os tipos garantem?" | por tipo: campos, faixa válida, invariante, validação no construtor, o que o banco reforça; tabela de constantes (slots, máscaras, tetos) com a origem da regra |
| `pool-e-transacoes.md` | "como abro conexão e transação?" | configuração e defaults; timeout e saturação; quando usar transação; isolamento; como não vazar conexão; teste que cobre |
| `como-adicionar-migration.md` | "como crio a próxima migration?" | regra da série; passo a passo; o que nunca fazer (`ALTER` no `up`, seed antes do DDL, FK para tabela futura); como regenerar catálogo e dicionário; gate |
| `como-adicionar-tabela.md` | "como modelo uma tabela nova?" | checklist de modelagem (nome, chave, nulidade, CHECK, índice, FK, retenção); quando é 1:1 e quando é coluna; como documentar (domínio + gerado); revisão exigida |
| `como-escrever-teste.md` | "que teste eu escrevo?" | escolha do alvo (contrato × sistema); fixture e factory; proibição de id mágico e `sleep`; como provar isolamento; padrão de nome |
| `erros-e-logs.md` | "como o erro aparece?" | taxonomia de erro; o que vai para log e o que nunca vai; correlação; o que o operador vê; como não vazar segredo em `Display`/`Debug` |
| `checklist-de-pr.md` | "o que revisar antes de abrir PR?" | lista objetiva: fmt, lint, teste, catálogo regenerado, issue vinculada, doc atualizada, sem segredo, sem `unwrap` novo em caminho de produção |

### 6.12 `docs/guia-administrador/*.md`

| Arquivo | Pergunta | Seções obrigatórias |
|---|---|---|
| `index.md` | "o que eu, administrador, preciso saber?" | mapa dos 7 guias; o que é responsabilidade do administrador; nível de acesso exigido por tarefa |
| `instalacao.md` | "como instalo do zero?" | pré-requisitos; passos; como confirmar cada passo; o que fazer se falhar; tempo esperado |
| `configuracao.md` | "quais parâmetros existem?" | tabela `parâmetro → efeito → default → obrigatório → onde é lido`; DSN e segredo por **nome**; o que muda sem reiniciar |
| `backup-e-restore.md` | "como protejo e recupero o dado?" | política (frequência, retenção, onde); comando de backup; ensaio de restore; como medir sucesso (contagem de tabela/coluna/linha); o que **não** fazer |
| `diagnostico.md` | "está lento ou travado, e agora?" | sintoma → causa provável → comando de verificação → ação; locks, conexões, saturação de pool, migration travada, disco |
| `manutencao-rotineira.md` | "o que fazer por rotina?" | diário/semanal/mensal; vacuum e estatística; revisão de crescimento; rotação de log; registro do que foi feito |
| `recuperacao-de-desastre.md` | "perdi o banco, e agora?" | cenários (corrupção, perda de volume, migration destrutiva); decisão de restaurar × corrigir; passos; validação; comunicação; prazo alvo |
| `sustentacao.md` | "como assumo este projeto?" | inventário do que existe; onde está cada coisa; contatos e papéis; o que só o dono anterior sabia (checklist de transferência) |

### 6.13 `docs/guia-teste/*.md`

| Arquivo | Pergunta | Seções obrigatórias |
|---|---|---|
| `index.md` | "como testo este crate?" | mapa dos 6 guias; os 5 alvos e o que cada um prova; comando único de suíte |
| `piramide-e-alvos.md` | "que tipo de teste eu escrevo?" | pirâmide (unit × contrato × sistema × E2E fora do crate); critério de escolha; o que cada nível pode tocar |
| `fixtures-e-factories.md` | "como crio dado de teste?" | factory por agregado; fixture de conteúdo; regra de id; limpeza por chave; proibição de estado global |
| `dados-de-teste.md` | "que dado existe para teste?" | catálogo de contas de teste (papel, o que cada uma prova); seed de desenvolvimento; como provisionar; o que nunca usar em produção |
| `testes-de-contrato.md` | "como provo que o schema e o catálogo batem?" | o que é contrato; como o teste lê as migrations; o que ele reprova; como corrigir quando falha |
| `testes-de-sistema.md` | "como provo contra Postgres real?" | subida do banco; DSN de teste; paralelismo; isolamento entre testes; tempo limite; como evitar flaky |
| `gates-e-anti-skip.md` | "como impeço teste que não roda?" | regra de `SKIP` proibido; como o gate detecta; matriz de capacidades; critério de cobertura por capacidade (não por linha) |

### 6.14 `docs/guia-documentacao/*.md`

| Arquivo | Pergunta | Seções obrigatórias |
|---|---|---|
| `index.md` | "qual regra eu procuro?" | mapa dos 7 guias por tarefa (criar, escrever, diagramar, linkar, ilustrar, revisar, publicar); estado de cada um |
| `estrutura-e-navegacao.md` | "onde este assunto mora?" | árvore e o que pertence a cada pasta; padrão de nome de arquivo (`kebab-case`, sem acento); quando criar seção × página; regra de `index.md` por pasta; como a árvore vira navegação no `preparar.py` |
| `estilo-e-linguagem.md` | "como escrevo?" | voz e tempo verbal; tamanho de frase e de página; o que evitar (adjetivo sem medida, "basta", "simplesmente"); termos preferidos × proibidos; 5 pares de exemplo certo × errado com frase real |
| `markdown-e-mkdocs.md` | "que recurso posso usar?" | recursos que a 2.0 renderiza: tables, footnotes, bloco de código com cópia, `> [!TIPO]` (e a **cor por tipo**: `tip` verde, `warning` âmbar, `danger` vermelho, `abstract` turquesa), `<div class="cartoes"></div>` para grade, emoji Unicode, riscado e **Mermaid** em cerca `mermaid` (um tipo por bloco); o que não usar (admonition `!!!`, abas `=== "…"`, details `???`, attr-list e botões, grid cards, `:material-…:`, captura de tela como prova única); exemplo de cada; quando preferir lista × tabela × aviso |
| `diagramas.md` | "que diagrama uso?" | tipo por pergunta (fluxo, sequência, estado, ER, C4); um tipo por diagrama; legenda em português; tamanho máximo; o que fazer quando o diagrama não renderiza |
| `links-referencias-e-ids.md` | "como aponto para outra página ou para o código?" | link relativo e âncora; citação `arquivo:linha`; prefixos `RF-DB`/`ADR-DB`/`WYDDB`; proibição de link para fora de `docs/`; como marcar link ainda sem destino |
| `imagens-e-evidencias.md` | "como provo com imagem?" | quando a imagem é necessária; nome e pasta; texto alternativo obrigatório; peso e formato; o que nunca aparece na captura (dado de jogador, segredo) |
| `revisao-e-publicacao.md` | "como isto é aprovado e publicado?" | fluxo de revisão; checklist do critério de pronto; gate de link e de build; o que muda entre rascunho e publicado; histórico da página |

### 6.15 `migrations/*.md`

**`index.md`** — pergunta: "qual migration criou o quê?". Obrigatório: tabela da série na ordem, com
`arquivo → páginas de tabela criadas → seed (sim/não) → observação`; total com fonte e data; regra
de que a coluna atual se lê em `gerado/`.

**`serie-e-regras.md`** — pergunta: "quais regras a série obedece e como a aplico?". Obrigatório:
numeração contígua e uma direção; `up`/`down` no mesmo arquivo; DDL antes de seed; sem `ALTER` no
`up`; sem FK para tabela criada depois; todo objeto criado cai no `down` do próprio arquivo; regra
do `down` de seed; o comando de cada camada de verificação (estrutural, contrato, banco real,
ciclo); o que fazer quando uma migration já rodou em produção. O **porquê** de cada regra fica em
`arquitetura/migracoes.md` e nos ADRs correspondentes.

**29 páginas `NNNN-<slug>.md`, uma por arquivo da série nova** (slug aprovado em 2026-11-18, decisão
`P8`; a série, a ordem e a contagem são de `plano-migrations.md` §3)
— pergunta: "o que este arquivo faz e como reverto?". Formato fixo:
`objetivo` (1–3 linhas), `tabelas criadas/alteradas` (com link para `gerado/`), `seeds inseridos`
(origem e reversão), `dependências` (migration anterior exigida), `down` (o que faz exatamente),
`risco de rollback`, `verificação` (comando).

**Nomes de página: fechados em 2026-11-18 (`P8`).** O slug é o nome do arquivo sem o prefixo
numérico, com `_` → `-`, em `kebab-case` sem acento (§6.14, `estrutura-e-navegacao.md`). A página é 1:1
com o arquivo, na mesma ordem lexicográfica da série, e o prefixo de 4 dígitos garante que um `0030`
futuro entre depois sem renomear nada. `index.md` reproduz a coluna **arquivo**; a coluna **página** é
o nome do arquivo em `migrations/`. Nenhuma página foi criada ainda — a escrita é da onda 5 e o
andamento fica em `migrations-checklist.md`.

| # | Arquivo da série | Página |
|---:|---|---|
| 1 | `0001_bootstrap.sql` | `0001-bootstrap.md` |
| 2 | `0002_ops_audit.sql` | `0002-ops-audit.md` |
| 3 | `0003_portal_person.sql` | `0003-portal-person.md` |
| 4 | `0004_portal_access.sql` | `0004-portal-access.md` |
| 5 | `0005_game_account.sql` | `0005-game-account.md` |
| 6 | `0006_game_character.sql` | `0006-game-character.md` |
| 7 | `0007_game_session.sql` | `0007-game-session.md` |
| 8 | `0008_game_skills.sql` | `0008-game-skills.md` |
| 9 | `0009_game_inventory.sql` | `0009-game-inventory.md` |
| 10 | `0010_game_guild.sql` | `0010-game-guild.md` |
| 11 | `0011_game_economy.sql` | `0011-game-economy.md` |
| 12 | `0012_ops_delivery.sql` | `0012-ops-delivery.md` |
| 13 | `0013_content_taxonomy.sql` | `0013-content-taxonomy.md` |
| 14 | `0014_content_item.sql` | `0014-content-item.md` |
| 15 | `0015_content_skill.sql` | `0015-content-skill.md` |
| 16 | `0016_seed_taxonomy.sql` | `0016-seed-taxonomy.md` |
| 17 | `0017_seed_item_catalog.sql` | `0017-seed-item-catalog.md` |
| 18 | `0018_seed_skill_catalog.sql` | `0018-seed-skill-catalog.md` |
| 19 | `0019_content_world.sql` | `0019-content-world.md` |
| 20 | `0020_seed_world.sql` | `0020-seed-world.md` |
| 21 | `0021_content_rules.sql` | `0021-content-rules.md` |
| 22 | `0022_content_overrides.sql` | `0022-content-overrides.md` |
| 23 | `0023_seed_rules.sql` | `0023-seed-rules.md` |
| 24 | `0024_content_maps.sql` | `0024-content-maps.md` |
| 25 | `0025_content_combat.sql` | `0025-content-combat.md` |
| 26 | `0026_content_quests_events.sql` | `0026-content-quests-events.md` |
| 27 | `0027_seed_quests_events.sql` | `0027-seed-quests-events.md` |
| 28 | `0028_game_quests_events.sql` | `0028-game-quests-events.md` |
| 29 | `0029_views.sql` | `0029-views.md` |

A tabela de slugs da **série do rascunho descartada** saiu desta seção quando `P8` fechou: os nomes
dela não correspondiam à série nova (lá o primeiro arquivo era de identidade; aqui `0001_bootstrap.sql`
é schemas + `content_version`) e continuam no histórico do git, não como vocabulário vigente.

**Série fechada (decisão `B2`, `plano-migrations.md` §3):** 32 arquivos, contíguos de `0001` a
`0032`, e `SCHEMA_VERSION = 32` na troca. `0001` absorve `0001`/`0031`/`0034`/`0053` da série herdada e `0030`
vira `0007`, de modo que **quatro** arquivos do legado deixam de existir como arquivo próprio.

### 6.16 `src/*.md`

**`index.md`** — pergunta: "qual página cobre o arquivo que estou lendo?". Obrigatório: árvore de
`crates/wyd-db/src/` com uma linha por arquivo e link; quais arquivos não têm página (`model`, `pool`,
`repo`, `transfer` são `mod.rs` de reexport) e por quê; regra de dependência entre módulos.

**58 páginas de módulo:** 24 no restante do crate (tabela abaixo) e **34** em `src/migrate/` e
`src/provision/`, uma por arquivo de código (§6.16.1 e §6.16.2). As antigas páginas únicas
`migrate.md`/`provision.md` deixaram de existir: em módulo com camadas, a árvore sozinha não permite
implementar arquivo nenhum.

| Página | Arquivo | Pergunta |
|---|---|---|
| `lib.md` | `src/lib.rs` | "o que o crate expõe e qual versão de schema declara?" |
| `error.md` | `src/error.rs` | "quais erros existem e o que cada um significa?" |
| `dsn.md` | `src/dsn.rs` | "como uma DSN é validada e por que ela não vaza em log?" |
| `model-person.md` | `src/model/person.rs` | "o que é uma pessoa no portal e o que ela valida?" |
| `model-game-account.md` | `src/model/game_account.rs` | "o que é uma conta de jogo: vaga, login, PIN e papel?" |
| `model-session.md` | `src/model/session.rs` | "o que é uma sessão viva, de portal e de jogo?" |
| `model-character.md` | `src/model/character.rs` | "o que é um personagem e quais são seus limites?" |
| `model-guild.md` | `src/model/guild.rs` | "o que é um clã e o que ele exige?" |
| `model-audit.md` | `src/model/audit.rs` | "o que é um registro de auditoria?" |
| `pool-config.md` | `src/pool/config.rs` | "quais parâmetros de pool são validados?" |
| `pool-pg.md` | `src/pool/pg.rs` | "como conexões são criadas e devolvidas?" |
| `repo-people.md` | `src/repo/people.rs` | "que operações existem sobre pessoa e credencial do portal?" |
| `repo-game-accounts.md` | `src/repo/game_accounts.rs` | "que operações existem sobre conta de jogo, vaga e moderação?" |
| `repo-portal-sessions.md` | `src/repo/portal_sessions.rs` | "como a sessão do portal é aberta, validada e revogada?" |
| `repo-characters.md` | `src/repo/characters.rs` | "que operações existem sobre personagem?" |
| `repo-sessions.md` | `src/repo/sessions.rs` | "como a sessão é criada, lida e encerrada?" |
| `repo-audit.md` | `src/repo/audit.rs` | "como se grava auditoria?" |
| `repo-grants.md` | `src/repo/grants.rs` | "como um grant entra e sai da fila?" |
| `repo-economy.md` | `src/repo/economy.rs` | "como crédito e débito são registrados?" |
| `repo-config.md` | `src/repo/config.rs` | "como a configuração é lida, escrita e versionada?" |
| `repo-content.md` | `src/repo/content.rs` | "como o catálogo é lido no boot?" |
| `repo-pin-reset.md` | `src/repo/pin_reset.rs` | "como o reset de PIN é emitido e consumido?" |
| `repo-pg.md` | `src/repo/pg.rs` | "onde mora o SQL e como ele está organizado?" |
| `transfer-save.md` | `src/transfer/save.rs` | "como o estado do personagem chega ao banco?" |

Cada página tem, obrigatoriamente: **responsabilidade** (1 parágrafo), **API pública** (tabela
`item → assinatura → garantia`), **invariantes** (o que nunca pode acontecer), **quem depende**,
**testes que cobrem** (link para `tests/alvos/`), **riscos de alteração** e **o que este módulo não
faz**.

#### 6.16.1 `src/migrate/` — 21 páginas, uma por arquivo

Cada página traz: responsabilidade, **API Rust exata** (struct, trait, função, assinatura), invariantes,
variantes de erro com `Display`, dependências de camada permitidas e proibidas, tabela de testes e o
critério "pronto quando". `mod.md` existe porque é nele que moram `pub use` e o `allow` de lint — é
decisão, não reexport trivial. O desenho que elas implementam é `src/migrate/spec.md` §10; a ordem de
leitura é a da tabela.

| # | Página | Arquivo | Pergunta |
|---:|---|---|---|
| 1 | `migrate/mod.md` | `src/migrate/mod.rs` | "o que a fachada expõe: `Command`, `DownTo`, `Report`, `run`?" |
| 2 | `migrate/config.md` | `config.rs` | "quais limites a configuração valida antes de tocar o banco?" |
| 3 | `migrate/error.md` | `error.rs` | "quais erros existem e com que código o processo sai?" |
| 4 | `migrate/migration/mod.md` | `migration/mod.rs` | "o que é modelo puro e o que ele nunca importa?" |
| 5 | `migrate/migration/version.md` | `version.rs` | "o que é uma versão e um slug válidos?" |
| 6 | `migrate/migration/checksum.md` | `checksum.rs` | "que bytes entram no hash e o que a normalização apaga?" |
| 7 | `migrate/migration/sql.md` | `sql.rs` | "que texto pode virar `Sql`, e como recuso transação e marcador?" |
| 8 | `migrate/migration/file.md` | `file.rs` | "como um `.sql` vira `RawMigration` e o que o parser recusa?" |
| 9 | `migrate/migration/set.md` | `set.rs` | "como a série é ordenada e recusada quando tem buraco?" |
| 10 | `migrate/migration/applied.md` | `applied.rs` | "o que o ledger devolve por linha?" |
| 11 | `migrate/migration/plan.md` | `plan.rs` | "aplicar, nada ou parar por divergência?" |
| 12 | `migrate/port/mod.md` | `port/mod.rs` | "quais são os cinco contratos e por que o lint é silenciado?" |
| 13 | `migrate/port/source.md` | `source.rs` | "como o runner recebe os arquivos sem `std::fs`?" |
| 14 | `migrate/port/ledger.md` | `ledger.rs` | "como o runner lê o que já foi aplicado?" |
| 15 | `migrate/port/unit_of_work.md` | `unit_of_work.rs` | "como arquivo e ledger commitam juntos?" |
| 16 | `migrate/port/lock.md` | `lock.rs` | "como dois processos serializam?" |
| 17 | `migrate/app/mod.md` | `app/mod.rs` | "o que a orquestração expõe?" |
| 18 | `migrate/app/runner.md` | `app/runner.rs` | "em que ordem preflight, lock, prepare, plano e execução?" |
| 19 | `migrate/adapter/mod.md` | `adapter/mod.rs` | "por que `sqlx` e `std::fs` só aparecem aqui?" |
| 20 | `migrate/adapter/pg.md` | `adapter/pg.rs` | "como o adapter fala com o PG17?" |
| 21 | `migrate/adapter/fs.md` | `adapter/fs.rs` | "como o diretório da série é lido?" |

#### 6.16.2 `src/provision/` — 13 páginas, uma por arquivo

Mesmo formato de §6.16.1; desenho de origem em `src/provision/spec.md` §5.

| # | Página | Arquivo | Pergunta |
|---:|---|---|---|
| 1 | `provision/mod.md` | `src/provision/mod.rs` | "o que a fachada expõe e por que `MIGRATOR` é reexportado?" |
| 2 | `provision/error.md` | `error.rs` | "quais erros existem e com que código saem?" |
| 3 | `provision/domain/mod.md` | `domain/mod.rs` | "o que a camada pura expõe?" |
| 4 | `provision/domain/spec.md` | `domain/spec.rs` | "quais papéis existem e que texto `render()` emite?" |
| 5 | `provision/domain/observed.md` | `observed.rs` | "como o cluster lido é representado?" |
| 6 | `provision/domain/plan.md` | `plan.rs` | "o que criar, o que está divergente e em que ordem?" |
| 7 | `provision/port/mod.md` | `port/mod.rs` | "quais são os dois contratos de infraestrutura?" |
| 8 | `provision/port/catalog.md` | `catalog.rs` | "como o cluster é lido sem o driver?" |
| 9 | `provision/port/script.md` | `script.rs` | "como o texto de `render()` chega ao cluster?" |
| 10 | `provision/app/mod.md` | `app/mod.rs` | "o que a orquestração expõe?" |
| 11 | `provision/app/runner.md` | `app/runner.rs` | "em que ordem `render`, `plan`, `check` e `apply`?" |
| 12 | `provision/adapter/mod.md` | `adapter/mod.rs` | "quem é o adapter e o que ele reexporta?" |
| 13 | `provision/adapter/pg.md` | `adapter/pg.rs` | "quais consultas e como o script roda numa sessão?" |

**Contratos fixados na revisão de 2026-11-18:** o índice da decisão é `lacunas.md` §1. Assinatura, tipo
e código moram só na **página dona** (a referência do `.rs`); o desenho de origem continua no `spec.md`
do módulo. Este bloco, `lacunas.md` e as ADRs apontam para a página, sem repetir o valor.

| Contrato | Página dona |
|---|---|
| erro de `MigrationTxn::record`/`forget` | `migrate/port/unit_of_work.md` §2 |
| variante de `MigrateError` para falha de `Source::load` e seu código | `migrate/error.md` §2–§3 |
| construtor do adapter do provisionamento | `provision/adapter/pg.md` §2 |
| execução do script de provisionamento | `provision/port/script.md` §3 |

São 21 arquivos em `src/migrate/` e 13 em `src/provision/`; o `Dsn` dos dois é o mesmo arquivo,
`src/dsn.rs`, e por isso não conta em nenhum dos dois. **Regra de dependência:**
`migration`/`domain` não importam `port`, `app` nem `adapter`; `app` não importa `adapter`; `sqlx` e
`std::fs` só existem em `adapter/`. Cada página de `mod.md` declara a fachada (`run`, `Command`,
`Report`, o erro do módulo) e a página do arquivo interno declara o contrato daquele arquivo; tipo
interno (`Plan`, `Step`, `Drift`, `MigrationTxn`, `Catalog`) só aparece na página que o define.

**Sincronia obrigatória:** a página é a referência do `.rs`. Alteração de assinatura, variante de erro,
invariante ou ordem de fluxo no código exige a mesma alteração na página, no mesmo PR — o checklist de
`guia-desenvolvedor/checklist-de-pr.md` cobre isso.

Arquivos sem página própria: `src/model/mod.rs`, `src/pool/mod.rs`, `src/repo/mod.rs` e
`src/transfer/mod.rs` — são reexport; descritos em uma linha no `index.md`.

### 6.17 `tests/*.md`

**`index.md`** — pergunta: "como a suíte está montada?". Obrigatório: árvore de
`crates/wyd-db/tests/`; tabela `alvo (Cargo.toml) → arquivo → tipo → precisa de banco?`; como rodar
tudo; como rodar um alvo.

**`fixtures-e-factories.md`** — pergunta: "que recurso compartilhado existe?". Obrigatório:
inventário de `tests/common/`; regra de criação de dado; o que é factory × fixture; regra de
limpeza.

**`dados-de-teste.md`** — pergunta: "que contas e dados uso?". Obrigatório: tabela de contas
(papel, o que prova, como é criada); relação com `seeds/dev.sql`; o que nunca é usado em produção.

**`resources/README.md`** — pergunta: "o que são os arquivos de recurso?". Obrigatório: formato,
campos, quem consome, regra de que dado sensível é fictício. `resources/accounts.json` é o dado de
apoio (`■`), não documentação: contém contas de teste com papéis distintos e nenhum segredo real.

**`alvos/*.md`** (5 páginas, uma por alvo do `Cargo.toml`) — pergunta: "o que este alvo prova?".
Obrigatório: capacidades cobertas, cenários, pré-requisito de ambiente, como interpretar falha,
tempo esperado.

| Página | Alvo |
|---|---|
| `contrato-catalogo.md` | `contrato_catalogo` |
| `contrato-migrations.md` | `contrato_migrations` |
| `sistema-pg.md` | `sistema_pg` |
| `sistema-sessoes.md` | `sistema_sessoes` |
| `sistema-stack-caps.md` | `sistema_stack_caps` |

**`specs/*.md`** (3 páginas) — pergunta: "o que precisa ser verdade para este tipo de teste
passar?". `contrato.md` (regras que não precisam de banco), `sistema.md` (regras que exigem
Postgres real e como isolar), `capacidades.md` (matriz `CAP-nn → alvo → cenário → requisito`).

### 6.18 `prompts/*.md`

Todos seguem o mesmo formato: **quando usar**, **entrada necessária**, **o que pedir** (passos),
**critério de parada**, **o que não fazer**, **como verificar a saída**.

| Arquivo | Uso |
|---|---|
| `README.md` | índice, relação prompt × skill, regra de que prompt não decide arquitetura |
| `plan/analise-de-requisito.md` | transformar um pedido em requisito com critério de aceitação |
| `plan/planejamento-de-issue.md` | fatiar trabalho em issue com DoD verificável |
| `execution/execucao-de-issue.md` | executar uma issue sem sair do escopo |
| `execution/escrita-de-migration.md` | escrever migration conforme a política da série |
| `review/revisao-de-codigo.md` | revisar diff do crate com foco em invariante e teste |
| `review/revisao-de-documentacao.md` | revisar página contra §7 e §10 |
| `test/execucao-de-testes.md` | rodar a suíte e reportar com evidência |
| `test/revisao-de-testes.md` | revisar teste novo contra `guia-teste/` |

### 6.19 `skills/*/SKILL.md`

Cada skill é uma pasta com `SKILL.md`, no mesmo formato: frontmatter (`name`, `description` em uma
frase com gatilho), **quando usar**, **pré-requisitos**, **passos**, **como verificar**, **quando
parar e perguntar**, **armadilhas**.

| Skill | Entrega |
|---|---|
| `plan/levantar-requisito` | entrevista estruturada → `RF-DB`/`RN-DB` com critério de aceitação |
| `plan/planejar-onda` | onda com arquivos, dependências e critério de pronto |
| `execution/implementar-issue` | código + teste + doc do item, dentro do escopo da issue |
| `execution/escrever-migration` | migration + página em `migrations/` + regeneração do dicionário |
| `review/revisar-schema` | revisão de DDL contra `padroes-postgres.md` e `integridade-e-constraints.md` |
| `review/revisar-documentacao` | revisão de página contra §3, §7 e §10 |
| `test/provar-schema` | execução das camadas estrutural, contrato, banco real e ciclo |
| `test/provisionar-dados-de-teste` | contas e fixtures conforme `tests/dados-de-teste.md` |

---

## 7. Formato fixo por tipo de página

Todo arquivo começa com um bloco de cabeçalho de 5 linhas e segue o molde do seu tipo. Sem molde,
sem página.

### 7.1 Cabeçalho (todos os arquivos)

```text
> Pergunta: <a pergunta que esta página responde>
> Público: <um dos públicos de §2>
> Dono: <papel>
> Base: <bloco de análise consultado | —>
> Revisado em: AAAA-MM-DD
```

`Base:` registra o bloco de §8.4 usado como contexto. Não é link, não é citação e não aparece no
site: é rastro interno de procedência. Página escrita só a partir de código, banco ou comando usa
`—`.

### 7.2 Moldes

| Tipo | Moldes obrigatórios |
|---|---|
| **Requisito** | ID · enunciado · entrada · saída · critério de aceitação (com comando/dado) · tabela e porta · issue |
| **Regra de negócio** | ID · enunciado · exemplo numérico · onde é garantida · efeito da violação |
| **Caso de uso** | ID · ator · pré-condição · fluxo principal · alternativos · pós-condição · requisitos cobertos |
| **ADR** | contexto · decisão · consequências · alternativas · como verificar |
| **Página de domínio** | propósito · diagrama · tabela de tabelas · ciclo de vida · regras e armadilhas · requisitos atendidos |
| **Página de módulo** | responsabilidade · API pública · invariantes · quem depende · testes · riscos · o que não faz |
| **Página de migration** | objetivo · tabelas · seeds · dependências · `down` · risco · verificação |
| **Issue** | contexto · entregável · DoD · fora de escopo · risco · depende de · evidência |
| **Guia operacional** | antes · passos · como confirmar · como reverter · se falhar |
| **Skill** | quando usar · pré-requisitos · passos · como verificar · quando parar · armadilhas |

### 7.3 Diagramas

Mermaid, sempre com um tipo só por diagrama e legenda em português. Mapa de uso: contexto e
componentes em `arquitetura/`, sequência em `specs/fluxos/`, estado em `specs/estados/`, ER em
`arquitetura/modelo-*` e nas páginas de domínio, dependência de fase em `roadmap/fases.md`.

---

## 8. Como a documentação é produzida

O material não é escrito "de memória": cada página nasce de uma fonte verificável e passa por
revisão cruzada.

```text
1. Pergunta      → qual pergunta de §2 esta página responde? (sem pergunta, não escreve)
2. Levantamento  → ler código, schema, comando ou artefato; anotar caminho e linha
3. Redação       → preencher o molde de §7 com o que foi levantado
4. Prova         → rodar o comando/ler o arquivo citado; número ganha fonte e data
5. Revisão       → outra pessoa (ou outra sessão) confere contra §3, §7 e §10
6. Congelamento  → link no index da pasta, estado atualizado em planos/metricas.md
```

### 8.1 Fontes permitidas

| Tipo de afirmação | Fonte que vale |
|---|---|
| schema atual | `crates/wyd-db/migrations/` + `docs/arquitetura/banco-catalogo.md` (gerado) |
| comportamento do crate | `crates/wyd-db/src/` com `arquivo:linha` |
| teste | alvo do `Cargo.toml` + saída do comando |
| operação | comando executado em ambiente de teste, com saída registrada |
| requisito | decisão registrada em `specs/` (com dono e data) |

A base de análise **não** aparece nesta tabela de propósito: ela não é fonte de afirmação, é mapa
de contexto (§8.4).

### 8.2 Regra da divergência

Se a página gerada, a migration e o texto discordarem, **a migration prevalece**; a divergência
vira defeito registrado em `riscos.md` e corrigida na origem (gerador ou migration), nunca
"ajustada" no texto.

### 8.3 Regra do exemplo

Toda regra de cálculo (refino, imposto, teto, crédito) traz **um exemplo numérico completo**, com
entrada, conta e resultado. Regra sem exemplo não é aceita.

### 8.4 Base de análise: mapa de contexto

Existe no repositório uma base de análise técnica do `wyd-db`, em
`.agents/prompts/análises/wyd-db/`. Ela **não é documentação do produto** e não sustenta afirmação
nenhuma sozinha: é o mapa do que já foi levantado, do que já foi provado e de onde cada assunto
foi investigado. Serve para três coisas:

1. não refazer levantamento que já existe;
2. saber o que era prova e o que era hipótese **antes** de escrever;
3. achar rápido o arquivo e a linha por onde começar a verificação.

Mapa dos blocos — o que procurar onde:

| Bloco | O que contém | Usar para escrever |
|---|---|---|
| `00-*` (raiz) | painel de cobertura, rastreabilidade, revisão de risco | saber o que falta e o que é risco |
| `database/` | domínios, padrões de PostgreSQL, decisões de banco, identidade, conteúdo | `arquitetura/` e `specs/` |
| `database/tabelas/` | uma página por tabela (gerada) | ponto de partida de cada domínio do dicionário |
| `database/plan/` | extração bruta, dados e scripts de conferência | insumo de checagem; **não é decisão** |
| `rust/` | portas, pool, migrations, filas, dívidas, issues, prioridades | `src/`, `guia-desenvolvedor/`, `roadmap/` |
| `tests/` | estratégia, mapa da suíte, fixtures | `guia-teste/` e `tests/` |

Regras de uso:

1. **Somente leitura.** Nada é editado, movido ou "corrigido" na base a partir desta árvore.
2. **Nada é copiado.** O texto oficial é escrito de novo, no molde de §7 e na voz deste projeto.
   Copiar frase traz junto ressalvas e marcas de outra estrutura, e a página nasce incoerente.
3. **Toda afirmação é reverificada** na fonte de §8.1 antes de entrar. Um `V` de lá não é `V`
   aqui: o nível é reconquistado com comando, arquivo e data desta rodada.
4. **Números não migram.** Contagem é remedida com o comando desta árvore; se divergir do que a
   base registrava, a divergência vira linha em `planos/riscos.md`.
5. **A base não é publicada nem linkada.** Nenhum link para ela em `docs/`. A ponte é o campo
   `Base:` do cabeçalho (§7.1), interno e textual.
6. **Decisão não se herda.** O que a base tratava como decisão só vale aqui depois de virar
   `ADR-DB-nnn` ou requisito em `specs/`.

### 8.5 Protocolo: da base para a página oficial

```text
1. escolher a página e a pergunta dela (§2)
2. localizar o bloco no mapa de §8.4 e ler SÓ o que responde à pergunta
3. listar as afirmações candidatas, cada uma com arquivo:linha de origem
4. reverificar cada uma nas fontes de §8.1 e reclassificar em V / H / P
5. escrever no molde de §7 e preencher Base:
6. o que não fechar vira linha em planos/riscos.md ou decisão em specs/ — nunca frase vaga
```

### 8.6 Quando a base e o código divergem

Código, migration e banco vencem sempre. A divergência é defeito registrado: uma linha em
`planos/riscos.md` com o que a base afirmava, o que o código mostra e qual página foi corrigida. Se
a base descrevia algo que não existe, a página **não menciona o inexistente** para "explicar a
diferença".

### 8.7 O dia em que a base sair do repositório

A base é insumo temporário; a documentação é permanente. Quando a base for removida, o campo
`Base:` passa a `—` em todas as páginas e nenhuma afirmação pode ficar pendurada nela: ou foi
reverificada e virou `V`, ou virou `H` com o que falta para fechar, ou foi retirada. Esse é o teste
de que a documentação não dependia dela.

---

## 9. Identificadores

### 9.1 Prefixos

| Prefixo | Uso | Exemplo |
|---|---|---|
| `RF-DB-nnn` | requisito funcional | `RF-DB-012` |
| `RN-DB-nnn` | regra de negócio | `RN-DB-004` |
| `RNF-DB-nnn` | requisito não funcional | `RNF-DB-003` |
| `UC-DB-nnn` | caso de uso | `UC-DB-007` |
| `ADR-DB-nnn` | decisão de arquitetura | `ADR-DB-0001` |
| `WYDDB-nnn` | issue do roadmap | `WYDDB-018` |
| `CAP-nn` | capacidade de teste | `CAP-04` |
| `DOM-nn` | domínio do dicionário | `DOM-11` |
| `RSK-nn` | risco | `RSK-06` |
| `FASE-n` | agrupamento de escopo | `FASE-3` |
| `P-n` | ordem de prioridade | `P-2` |

### 9.2 Regras

- Prefixo novo só existe com uma linha nesta tabela.
- `FASE` agrupa escopo; `P` ordena prioridade. Não são sinônimos e nunca se substituem.
- ID não é reutilizado depois de retirado: fica com estado `retirado` e o motivo.
- Toda página que cita ID usa o formato exato, sem abreviar.

### 9.3 Estados

| Estado | Significa |
|---|---|
| `proposto` | escrito, aguardando aprovação |
| `aceito` | aprovado, vigente |
| `substituído` | trocado por outro ID, com link |
| `retirado` | não vale mais, com motivo e data |

---

## 10. Critério de pronto de uma página

Uma página só é `aceito` quando **todos** forem verdadeiros:

1. tem cabeçalho de §7.1 e responde a uma pergunta de §2;
2. todas as seções obrigatórias do seu tipo estão presentes e preenchidas;
3. todo número citado tem fonte e data;
4. toda afirmação de comportamento tem nível `V`, `H` ou `P`, coerente com a evidência;
5. todo link resolve (arquivo existe e âncora existe);
6. diagramas renderizam;
7. nenhum segredo, DSN, token ou hash aparece;
8. nenhum `TODO`, "em construção" ou seção vazia;
9. foi revisada por alguém que não escreveu;
10. está linkada no `index.md` da pasta e contabilizada em `planos/metricas.md`;
11. tem `Base:` preenchido (bloco consultado ou `—`) e nenhuma afirmação pendurada em material
    externo à árvore.

Página que falha em 1–4 volta para rascunho; 5–8 são defeito bloqueante; 9–11 são pendência de
fechamento.

---

## 11. Ordem de produção (ondas)

**Vocabulário (decidido em 2026-11-19):** **Onda** é a trilha de documentação (a tabela abaixo);
**Etapa** é a trilha de código (`plano-migrations.md` §9). A timeline única, com as duas trilhas e o
ponto de junção **Etapa 1 → Onda 3**, fica em `planos/plano-de-documentacao.md`, criado na onda 8.

Cada onda é uma entrega revisável. Nenhuma começa antes de a anterior fechar o critério de saída.
A ordem segue dependência de conteúdo: primeiro o vocabulário, depois o requisito, depois a
tabela, depois o procedimento.

| Onda | Entrega | Arquivos | Critério de saída |
|---|---|---:|---|
| **1. Fundação** | `convencoes`, `glossario`, `docs/index`, `arquitetura/{index,visao-geral,contexto-c4,fronteira-hot-cold}`, `guia-documentacao/` (8) | 15 | vocabulário fechado; fronteira Hot×Cold aprovada; toda página futura já tem molde **e** regra de escrita |
| **2. Modelo** | `arquitetura/{componentes-c4,implantacao,fluxo-de-dados,modelo-conceitual,modelo-logico,integridade-e-constraints,padroes-postgres,seguranca}` | 8 | ER conceitual e lógico revisados; constraints conferidas contra o catálogo |
| **3. Requisitos** | `specs/` completo: 9 páginas + 6 fluxos + 5 estados | 20 | todo `RF`/`RN`/`RNF` com critério de aceitação e destino na matriz |
| **4. Dicionário** | `dicionario-dados/` (2 + 19) + as 97 páginas por tabela, escritas à mão em lotes (`M26`, 2026-11-21) | 21 + 97 | 97 tabelas cobertas quando as páginas forem escritas; `gerado/index.md` é índice auxiliar, fora da contagem; sem gerador; **66 das 97 escritas** em 2026-11-22 |
| **5. Histórico de schema** | `migrations/` (2 + 29) + `arquitetura/migracoes` + `retencao-e-capacidade` | 31 | 29 páginas com `down` e verificação, nomes fechados na decisão `P8` (`plano-migrations.md` §3); política da série em um único dono |
| **6. Operação e processo** | `guia-administrador/` (8) + `planos/{index,implantacao,manutencao,qualidade,riscos,metricas}` | 14 | todo procedimento com "como confirmar" e "como reverter"; métricas com fonte |
| **7. Dev e teste** | `guia-desenvolvedor/` (11) + `guia-teste/` (7) + `src/` (59) + `tests/` (12) | 89 | todo módulo com contrato e teste associado; todo alvo com página |
| **8. Planejamento do trabalho** | `roadmap/` (2 + 40) + `planos/{documentacao,testes,dados,registro}` | 46 | 40 issues com DoD executável; fases com critério de saída |
| **9. Agente e publicação** | `prompts/` (9) + `skills/` (9) + navegação gerada pelo `preparar.py` + as ADRs já aceitas ou encerradas (2 hoje) | 18 + ADRs | prompt e skill sem regra de negócio duplicada; site publica só `docs/` |

**Total:** teto de páginas autorais revisado em `planos/metricas.md` depois de a série nova existir; as **97** páginas de tabela (`plano-migrations.md` §3), escritas à mão em lotes (`M26`), + 1 dado de apoio (`gerado/index.md`, índice auxiliar que não conta como página de tabela); **36 das 97 ainda não existem** em 2026-11-22 e entram em lotes.

**Dependências duras:** Onda 3 depende de 1–2 (requisito cita entidade do modelo); Onda 4 depende
de 2 (domínio cita entidade); Onda 7 depende de 4–5 (módulo cita tabela e migration); Onda 8
depende de 3 (issue cita requisito).

### 11.1 Critério de saída da documentação `1.0.0`

A release de documentação fecha quando **todas** forem verdadeiras ao mesmo tempo:

- todas as páginas contadas em `planos/metricas.md` (autorais + **97** geradas) estão `aceito` ou
  `retirado` com justificativa;
- nenhuma página cita número sem fonte e data;
- nenhum link relativo sem destino (`mkdocs build --strict` verde);
- toda página tem `Base:` preenchido e coerente com o que foi escrito;
- `planos/metricas.md` bate com a contagem real da árvore;
- `RF`/`RN`/`RNF` cobrem todas as tabelas e a matriz de rastreabilidade não tem lacuna sem dono.

---

## 12. Decisões para aprovação

| # | Decisão | Recomendação | Bloqueia |
|---|---|---|---|
| `D-1` | escopo da `1.0.0` = série reescrita do zero, com a identidade em `0001_bootstrap.sql` (schemas + `content_version`) e `0003_portal_person.sql` (pessoa), e os demais domínios escritos inteiros na mesma ordem: **32 arquivos**, **97 tabelas** e `SCHEMA_VERSION = 32` na troca — planejado, fonte `plano-migrations.md` §3 | **aprovada em 2026-11-19** (§1.2) | Ondas 2–9 |
| `D-2` | dicionário em duas camadas: 19 páginas de domínio + **97** geradas | aprovar (§6.8) | Onda 4 |
| `D-3` | **revista em 2026-11-18:** 58 páginas de módulo (§6.16) — 24 no restante do crate e, em `src/migrate/` e `src/provision/`, **uma página por arquivo** (34), com `mod.rs` incluído; o espelho 1:1 continua proibido no resto do crate | aprovar (§6.16) | Onda 7 |
| `D-4` | uma página por arquivo da série nova (**29** arquivos), sem SQL na pasta | aprovar (§6.15) | Onda 5 |
| `D-5` | prefixos de ID de §9.1, independentes de qualquer numeração anterior | aprovar | Ondas 3 e 8 |
| `D-6` | `docs/` é a fonte do site; `migrations/`, `src/`, `tests/`, `prompts/`, `skills/` não são publicados | aprovar | Onda 9 |
| `D-7` | 40 issues iniciais distribuídas nas 7 fases de §6.10 | aprovar a distribuição | Onda 8 |
| `D-8` | o teto de páginas autorais é o medido em `planos/metricas.md` (a série reescrita muda a contagem); página que não responder a §2 é descartada | aprovar | todas |

### 12.1 Checklist de aprovação

- [x] `D-1` escopo reescrito e limites de §1.2 (aprovada em 2026-11-19)
- [ ] `D-2` dicionário em duas camadas
- [ ] `D-3` páginas de módulo sem espelho de código
- [ ] `D-4` página por migration
- [ ] `D-5` prefixos de identificador
- [ ] `D-6` fronteira de publicação
- [ ] `D-7` distribuição das 40 issues por fase
- [ ] `D-8` critério de descarte de página sem valor
- [ ] ondas 1–9 na ordem de §11, uma por vez

### 12.2 Decisões de arquitetura registradas (ADR-DB)

Acompanhamento só: o conteúdo, o motivo e as alternativas moram na página de cada ADR.

| ADR | Status | Dono | Página |
|---|---|---|---|
| `ADR-DB-0001` — provisionamento de papel fora da série versionada | `aceito` | equipe do `wyd-db` | [abrir](docs/arquitetura/adr/adr-db-0001-provisionamento-de-papel-fora-da-serie.md) |
| `ADR-DB-0002` — runner de migrations próprio, com ledger por versão | `aceito` | equipe do `wyd-db` | [abrir](docs/arquitetura/adr/adr-db-0002-runner-de-migrations-proprio.md) |

---

## 13. O que nunca entra nesta árvore

- Código, migration, script, teste, SQL executável, Dockerfile ou configuração de serviço.
- Cópia de documento de planejamento anterior, resumo de histórico ou "lição aprendida".
- Conteúdo importado de outra origem e numeração de issue que não seja `WYDDB-nnn`.
- Changelog, ata de reunião, diário narrativo (o registro é uma linha por entrega).
- Coluna, tipo, índice ou CHECK escrito à mão: isso é gerado.
- Segredo, DSN real, token, hash, captura com dado de jogador.
- Placeholder, "em construção", seção vazia, adjetivo sem medida.

---

*Documento criado em 2026-11-17. Revisão obrigatória ao fechar cada onda e ao aprovar cada `D-n`.*
