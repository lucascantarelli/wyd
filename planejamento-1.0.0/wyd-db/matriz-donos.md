# Matriz de donos — seção de raiz × página que enuncia × estado da nota

Uma linha por seção dos seis documentos de raiz, com a página escrita que enuncia aquele conteúdo e o
estado da nota de enunciação. Serve para achar divergência de dono sem reler os seis arquivos e as
quinze páginas.

Seções de `##` e de `###`; um `###` só entra quando tem destino próprio.

## Como ler

| Estado | O que quer dizer |
|---|---|
| **existe** | a seção termina com `**Enunciação (data):**` que nomeia a página dona e o que fica aqui |
| **ponteiro** | a seção é o ponteiro; a nota registra o que saiu e para onde |
| **falta** | há página escrita enunciando o conteúdo e a seção não declara — candidata a nota |
| **—** | nenhuma página escrita enuncia; a nota não é devida (a coluna diz se alguma página **cita** a seção) |

"Cita" não é "enuncia": a página que aponta para a seção como dona não é segunda boca e não gera nota
(`docs/convencoes.md` → *Nota de enunciação*). A nota é citada pela seção que enuncia, nunca por número
de linha — a linha envelhece a cada edição, a seção não.

## `padrao-banco.md`

| § | Título | Página que enuncia | Nota |
|---|---|---|---|
| 0 | Regra de ouro | `docs/arquitetura/padroes-postgres.md` §1 | **existe** (nota de §0) |
| 1 | Decisões estruturais (`P-01`–`P-22`) | — (citada em `como-adicionar-tabela.md` §1, `padroes-postgres.md` §3 e §6) | — |
| 2 | Schemas e fronteiras | — (citada em `seguranca.md`, `plano-de-implantacao.md` §Verificação pós-implantação) | — |
| 3 | Convenções de nome | — (citada em `padroes-postgres.md` §4, `como-adicionar-tabela.md` §1) | — |
| 4 | Tipos por intenção | `docs/arquitetura/padroes-postgres.md` §2 | **existe** (nota de §4) |
| 5 | Integridade (as sete regras) | `docs/arquitetura/integridade-e-constraints.md` §1–§4 e §9 | **existe** (nota de §5) |
| 6 | Índices | `docs/arquitetura/padroes-postgres.md` §5 | **existe** (nota de §6) |
| 7 | Triggers, funções e views | — (citada em `padroes-postgres.md` §1, `integridade-e-constraints.md` §6) | — |
| 8 | Papéis e permissões | `docs/arquitetura/seguranca.md` | **existe** (nota de §8) |
| 9 | Migrations | `docs/arquitetura/migracoes.md` §2 e §4 | **ponteiro** (nota de §9) |
| 10 | Testes e gates | — (regra daqui; `integridade-e-constraints.md` §7 e `testes-de-sistema.md` citam) | — |
| 11 | Antipadrões proibidos | `docs/arquitetura/padroes-postgres.md` §7 (agrupa por sintoma; a lista é daqui) | **—** (nota removida em 2026-11-18) |
| 12 | Dívida documental | — | — |
| 13 | Fontes | — (citada como origem de plataforma em `padroes-postgres.md` e `integridade-e-constraints.md`) | — |

## `plano-migrations.md`

| § | Título | Página que enuncia | Nota |
|---|---|---|---|
| 1 | Ferramentas que recusam o desenho | `como-adicionar-tabela.md` §6 (estado `V` do gate) e `plano-de-implantacao.md` §Pré-requisitos | **existe** (nota de §1) |
| 1.1 | O que muda em cada arquivo | — | — |
| 1.2 | Duas séries convivem | — (citada em `adr-db-0002`, Contexto) | — |
| 1.3 | Critério de pronto do PR | — (citada em `como-adicionar-tabela.md` §6) | — |
| 2 | Formato e numeração | `como-adicionar-migration.md` §Regra da série + `migracoes.md` §2 | **existe** (nota de §2) |
| 3 | A série (31 arquivos, 97 tabelas) | — (a ordem e a contagem são daqui; `migrations-checklist.md` §2 é o estado) | — |
| 3.1 | Por que esta ordem | — (citada em `integridade-e-constraints.md` §6) | — |
| 3.2 | Corte vertical | — | — |
| 4 | Seeds: 26 viram 6 | — (citada em `como-adicionar-migration.md` §Regra da série, `plano-de-implantacao.md` §Ordem) | — |
| 5 | O que sai e o que entra | — | — |
| 6 | Troca da série | — (destino escrito `plano-de-implantacao.md` não enuncia a troca) | — |
| 7 | Gates de aceite | — (delegado, não reenunciado: `como-adicionar-migration.md` §Gate) | — |
| 8 | Riscos | — (`planos/riscos.md` não escrito) | — |
| 9 | Ordem de execução | `docs/planos/plano-de-implantacao.md` §Ordem | **existe** (nota de §9) |
| 10 | O que este plano não decide | — | — |

## `ddl-confirmado.md`

| § | Título | Página que enuncia | Nota |
|---|---|---|---|
| 1 | O critério de "100%" | — (citada em `integridade-e-constraints.md`, `Base`) | — |
| 2 | Veredito por arquivo (29) | — (o estado é de `migrations-checklist.md` §1.2; ver *Achados*) | — |
| 3 | `0001_bootstrap.sql` | — (`migrations/0001-bootstrap.md` cita) | — |
| 4 | `0004_portal_access.sql` | — (`migrations/0004-portal-access.md` cita) | — |
| 5 | DDL das tabelas já desenhadas | `docs/dicionario-dados/dominio-acesso.md` | **existe** (nota de §5) |
| 5.1 | `portal.person` | `dominio-acesso.md` §Tabela de tabelas, §Ciclo de vida do dado | coberta pela nota de §5 |
| 5.2 | Bloco `game` da conta | `dominio-acesso.md`, idem | coberta pela nota de §5 |
| 5.3 | `ops.audit_log` | — (`dominio-auditoria.md` não escrito) | — |
| 5.4 | `ops.login_attempt` | — (`dominio-auditoria.md` não escrito) | — |
| 5.5 | `0007` — canal e sessão | `dominio-acesso.md` (sessão); canal espera `dominio-canal.md` | coberta pela nota de §5 |
| 5.6 | `portal.person_profile` | `dominio-acesso.md`, idem | coberta pela nota de §5 |
| 5.7 | `portal.person_consent` | `dominio-acesso.md`, idem | coberta pela nota de §5 |
| 6 | Adaptações contra o rascunho | — (não se publica: comparação com desenho descartado) | — |
| 7 | Como provar este DDL | — (citada em `testes-de-sistema.md`; contrato não escrito) | — |
| 8 | O que este documento não faz | — | — |

## `migrations-checklist.md`

| § | Título | Página que enuncia | Nota |
|---|---|---|---|
| 1 | Como ler este checklist | — | — |
| 1.1 | Nomes de página | — (citada em `como-adicionar-migration.md` passo 6) | — |
| 1.2 | Estado de tabela — derivado daqui | `docs/convencoes.md` §Estados (significado dos três estados; aponta §1.2 sem copiar o mapeamento) + `dominio-acesso.md` (aplica por domínio) | **existe** (nota de §1.2) |
| 2 | Checklist (31 arquivos, 97 tabelas) | 25 das 31 páginas de `migrations/` escritas; restam os 6 seeds (`migrations-checklist.md` §4) | — |
| 3 | Bloqueios agrupados | — (`specs/restricoes-e-premissas.md` não escrito) | — |
| 4 | Contagem | — (`planos/metricas.md` não escrito) | — |
| 5 | O que toda página de migration tem | — (citada, não reproduzida, em `como-adicionar-migration.md` e `como-adicionar-tabela.md` §4) | — |
| 6 | Regra de manutenção | — | — |

## `lacunas.md`

| § | Título | Página que enuncia | Nota |
|---|---|---|---|
| 0 | Como ler (estados, prazos, donos) | — | — |
| 1 | Situação em uma linha | — (a tabela declara dono por linha, em `Refletida em`) | — |
| 2 | Lacunas | páginas donas de cada item fechado (`como-adicionar-tabela.md` §2 para `M7`/`M23`) | **existe** (nota em §2.4) |
| 2.1 | Bloqueantes | idem §2 | coberta pela nota de §2 |
| 2.2 | Altas | idem §2 | coberta pela nota de §2 |
| 2.3 | Médias | idem §2 | coberta pela nota de §2 |
| 2.4 | Resumo por marco | — (a nota de §2 fecha a seção depois desta) | coberta pela nota de §2 |
| 3 | Divergências do plano diretor | — (`planos/riscos.md` não escrito) | — |
| 4 | Achados estruturais (`D-A`, `D-B`, `D-C`) | — (`D-C` citada em `convencoes.md` e `como-adicionar-tabela.md` §4) | — |
| 5 | O que não fazer enquanto aberto | — | — |
| 6 | Regra de manutenção | — | — |

## `rascunho-accounts.md`

| § | Título | Página que enuncia | Nota |
|---|---|---|---|
| 1 | Como ler | — | — |
| 2 | Inventário de requisitos (`R1`–`R16`) | — (citada em `integridade-e-constraints.md` §6; `specs/` não escrito) | — |
| 3 | Multibox, auto-attack e limite por IP | `docs/arquitetura/integridade-e-constraints.md` §6 | **existe** (nota de §3) |
| 4 | Defeitos do legado | — (`specs/regras-de-negocio.md` não escrito) | — |
| 5 | Estados que o desenho exige | — (`specs/estados/` não escrito) | — |
| 6 | Decisões `D1`–`D14` | — (a tabela declara dono por linha, em `Onde vive hoje`) | — |
| 7 | O que continua UNVERIFIED | — | — |
| 8 | Para onde foram as seções removidas | — | — |

## Contagem

| Documento | Seções | `existe` | `ponteiro` | `falta` | `—` |
|---|---:|---:|---:|---:|---:|
| `padrao-banco.md` | 14 | 5 | 1 | 0 | 8 |
| `plano-migrations.md` | 15 | 3 | 0 | 0 | 12 |
| `ddl-confirmado.md` | 15 | 1 | 0 | 0 | 14 |
| `migrations-checklist.md` | 8 | 1 | 0 | 0 | 7 |
| `lacunas.md` | 11 | 1 | 0 | 0 | 10 |
| `rascunho-accounts.md` | 8 | 1 | 0 | 0 | 7 |
| **total** | **71** | **12** | **1** | **0** | **58** |

Treze notas nos seis documentos (a de §5 do `ddl-confirmado.md` cobre seis subseções). Os outros três
documentos de raiz — `wyd-db-1.0.0.md`, `proposta-paginas.md` e este arquivo — ficam fora: o primeiro é
o plano diretor, o segundo o mapa de diluição, este a matriz.

## Achados

1. **Nenhuma seção está em `falta`.** A última era `plano-migrations.md` §1, fechada em 2026-11-18
   com nota que separa o que as páginas já enunciam — o estado do gate (`V`) e a conferência manual,
   em `como-adicionar-tabela.md` §6, e o que a implantação assume sem ele, em
   `plano-de-implantacao.md` §Pré-requisitos — do que continua sendo da seção: os defeitos de cada
   ferramenta e o critério de pronto de §1.3.
2. **`ddl-confirmado.md` §2 × `migrations-checklist.md` §2 usavam duas escalas para o mesmo arquivo.**
   O primeiro dizia `confirmado`/`aberto`; o segundo diz `concluída`/`pronta`/`parcial`/`bloqueada`.
   **Resolvido em 2026-11-18:** a coluna `Veredito` de §2 passou a citar o `Estado` do checklist arquivo
   a arquivo, e o que resta próprio da página é o que o estado não responde — qual seção fecha o DDL e o
   que falta para fechar, mais o ponteiro para a seção. "Fechado" continua sendo palavra desta página,
   mas como qualidade do **texto** do DDL, nunca como estado de arquivo; a derivação segue só em §1.2.
3. **Três enumerações da série de 29 convivem sem colisão declarada:** `plano-migrations.md` §3 (ordem e
   o que cria), `ddl-confirmado.md` §2 (veredito do DDL) e `migrations-checklist.md` §2 (estado e
   bloqueio). O cruzamento só é seguro porque §3 do plano diz que o andamento é do checklist; o
   veredito do DDL, não.
4. **`lacunas.md` §1 e `rascunho-accounts.md` §6 dispensam nota por estrutura**, não por omissão: as
   colunas `Refletida em` e `Onde vive hoje` apontam o dono linha a linha. Página nova que reenuncie um
   desses itens deve citar o ID, não redigitar a decisão.
5. **Cinco subseções de `ddl-confirmado.md` §5 estão cobertas por uma nota só** (§5.1, §5.2, §5.5,
   §5.6, §5.7). Se `dominio-canal.md` ou `dominio-auditoria.md` forem escritas, a nota de §5 precisa
   mudar junto — hoje ela nomeia esses dois como pendentes.
6. **`§11` perdeu a nota em 2026-11-18, e `§6` manteve.** Decisão sob a leitura estrita da regra 1:
   a página §7 só agrupa os antipadrões por sintoma e diz que a lista é de §11 — chave de leitura, não
   enunciação. Já a página §5 **decide** quando um índice pode existir, então §6 conserva a nota. O
   critério está em `docs/convencoes.md` §*Nota de enunciação*; a seção §11 ganhou um parágrafo sem
   nota, porque a nota registrava uma relação que nenhuma trava de mudança conjunta sustentava.
7. **58 das 71 seções não têm página escrita.** A matriz não mede cobertura de documentação: mede
   divergência de dono. Seção em `—` pode ser lacuna de página, mas não é defeito desta matriz.
