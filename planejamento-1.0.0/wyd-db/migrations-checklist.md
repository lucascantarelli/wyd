> Pergunta: "quais migrations da série 1.0.0 já podem ser documentadas e qual é o bloqueio de cada uma?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3, `ddl-confirmado.md` §2, `lacunas.md` §2
> Revisado em: 2026-11-23

Este é o **único controle** do andamento da documentação da série `migrations/`. Ele diz, para
cada um dos 34 arquivos planejados, qual página deve existir, quantas tabelas o arquivo cria (`—` quando
o arquivo é só seed) e se há bloqueio para escrevê-la. O plano (`plano-migrations.md`) é o dono da
**ordem e do escopo**; o texto DDL fechado é dono de `ddl-confirmado.md`; a **decisão** de cada
bloqueio é dona de `lacunas.md`. Aqui não se decide nada: aqui se acompanha.

Este arquivo fica na **raiz** de `planejamento-1.0.0/wyd-db/`, fora de `migrations/`: a pasta guarda as
páginas de migration, uma por arquivo `.sql` da série, e a nota transversal `serie-e-regras.md`, que
não é página numerada nem entra nas contagens.

## 1. Como ler este checklist

| Estado | Significa |
|---|---|
| `pronta` | DDL do arquivo inteiro fechado em `ddl-confirmado.md`; a página pode ser escrita agora, do começo ao fim |
| `aguardando aprovação` | DDL do arquivo inteiro **desenhado** e registrado em `ddl-confirmado.md` como proposta; espera o "sim" do dono. Não conta como `pronta` e não é `parcial`, porque não falta desenho. Nenhum arquivo está neste estado em 2026-11-22 |
| `bloqueada` | falta desenho de coluna, decisão de lacuna ou dado de seed verificado; a página **não** pode ser escrita inteira |
| `parcial` | uma ou mais tabelas do arquivo têm DDL fechado, o resto não — ou todas as tabelas atuais estão fechadas mas o escopo do arquivo pode crescer por lacuna aberta (`0005`); a página só fecha quando o arquivo fecha |
| `concluída` | página escrita em `migrations/`, conferida contra `ddl-confirmado.md` e com os 10 itens de §5 |

Marcar uma linha como concluída exige que a página correspondente exista em `migrations/` com o
conteúdo mínimo de §5. Enquanto o bloqueio não fechar, a linha fica `bloqueada` mesmo que a página
exista em rascunho — não existe estado "quase". A única exceção é `aguardando aprovação`, criada em
2026-11-20: o desenho está inteiro, mas o lote não vale como fechado até o dono responder "sim".

**Estado da página × estado do arquivo.** `concluída` diz que a página existe e foi conferida; não
diz que o dono a aprovou. O estado de aceite mora na linha `**Estado:**` da própria página; este
checklist não o espelha (`M33`, Q-M33-5). Para saber se uma página está `aceito`, leia a página, não
esta tabela.

### 1.1 Nomes de página

A estrutura do diretório vem do radical do arquivo `.sql`: a página troca **cada `_` por `-`**
(`kebab-case`, sem acento), e nenhum underscore do arquivo sobrevive no nome — o par arquivo/página
fica inequívoco:

`0004_portal_access.sql` → `0004_portal_access.md`

Nos 26 arquivos que criam tabela em um schema, o radical é `[numero]-[schema]-[nome]`, igual para o
`.sql` e para a página: `0005_game_account.sql` → `0005_game_account.md`. A série **versionada** em
`crates/wyd-db/migrations/` já usa essa forma com hífen; a grafia com underscore (`0005_game_account.sql`)
que circulou nos rascunhos do plano não corresponde a nenhum arquivo do repositório e foi corrigida em
todas as páginas em 2026-11-25. Dois casos não seguem essa forma: os 6 arquivos de seed, cujo segmento
do meio é `seed` e não schema (`0016-seed-taxonomy.sql` → `0016-seed-taxonomy.md`, página planejada que
semeia `content` e cujo estado de escrita fica em §2); e `0001` e `0029`, que não têm schema próprio — `0001` cria os quatro schemas e `0029`
cria views de dois deles —, e por isso caem em `[numero]-[nome].md`. A exceção de `0001`/`0029` está
dita no topo da própria página.

Páginas `0000_*` **não entram** nesta pasta: pré-requisito de ambiente (papel de cluster e estado do
schema `public`) é artefato gerado, especificado em
`planejamento-1.0.0/wyd-db/src/provision/spec.md` (**planejamento**) e decidido em ADR-DB-0001. Aqui
só mora página que corresponde a um arquivo `.sql` da série.

### 1.2 Estado de tabela — derivado daqui

O estado de tabela (`pronto`/`provisório`/`planejado`, definido em `docs/convencoes.md`) **não se
digita** em página nenhuma. Ele deriva do estado do **arquivo** na tabela de §2:

| Estado do arquivo | Estado das tabelas que ele cria |
|---|---|
| `pronta` | `planejado` — DDL inteiro fechado, série não aplicada |
| `aguardando aprovação` | `provisório` — desenho completo, mas não aprovado nem aplicado |
| `parcial` | `provisório` — o DDL da tabela pode estar fechado, mas o escopo do arquivo ainda cresce |
| `bloqueada` | `provisório` — nome conhecido, forma não desenhada |
| `concluída` | `planejado` — a página existe; a migration segue não aplicada |

`pronto` **não sai desta tabela**: exige a migration aplicada com o checksum vigente no ledger
(`src/migrate/spec.md` §3), e por isso só passa a existir depois da troca de série (`plano-migrations.md`
§6).
Página de domínio que precise do estado cita esta seção e a linha do arquivo — não copia o valor.

**Enunciação (2026-11-18):** o **significado** dos três estados é de `docs/convencoes.md` §Estados,
que aponta para cá como dono único da derivação e não reproduz o mapeamento; `dominio-acesso.md`
aplica a derivação por domínio e cita esta seção, nunca o valor. Aqui ficam o **mapeamento** arquivo →
tabela e o **estado de cada arquivo** (§2). Regra nova de derivação entra só aqui; estado novo (a
palavra) é decisão de `convencoes` e aparece nas duas na mesma entrega.

## 2. Checklist (34 arquivos; 109 tabelas)

O `0012` fechou em 2026-11-20 com o desenho de `ddl-confirmado.md` §5.16 e a decisão `M25` (dreno por
`SKIP LOCKED`, expurgo de 30 dias), e o `0030` fechou na mesma data, com o desenho de §5.15 aprovado
pelo dono. `0013`, `0014` e `0015` fecharam em 2026-11-20 (`ddl-confirmado.md` §5.17, §5.18 e §5.19);
`0019` e `0021` fecharam em 2026-11-20 (§5.21 e §5.22) e o `0024` em 2026-11-21 (§5.24). Em
**2026-11-21** o dono aprovou os cinco lotes restantes — `0022` (§5.23), `0025` (§5.25), `0026`
(§5.26), `0028` (§5.28) e `0029` (§5.29) —, então os 25 arquivos de DDL estão `concluída` e só os 6
seeds seguem `bloqueada`, por dado de origem não verificado (`plano-migrations.md` §4). Em
**2026-11-21** o dono decidiu `M27` e o `0032` entrou só com `GRANT`s: os arquivos `concluída` passam a
26 e `Tab.` continua 97. Em **2026-11-22** o dono fechou `M30` e `M31` (`LAC-ADM-02`, `LAC-ADM-06` e
`LAC-ADM-07`): os lotes `0033` e `0034` entram com 1 e 11 tabelas, os arquivos `concluída` passam a 28 e
`Tab.` a 109. Em **2026-11-23** o dono confirmou que a fonte dos 6 seeds é o cliente 7662 medido, que
eles ficam fora da onda A (`Q-M33-6`) e que nenhum valor herdado é tratado como verificado; a série de
28 pode andar e os seeds continuam `bloqueada` por dado. O pedido de curadoria que fecha esse bloqueio —
item por item e com o formato da medição — está em `pedido-de-curadoria-dos-seeds.md`, na raiz do sistema.

| # | Página | Tab. | Estado | Bloqueio |
|---|---|---:|---|---|
| 1 | `0001_bootstrap.md` | 1 | `concluída` | — |
| 2 | `0002_ops_audit.md` | 4 | `concluída` | — (DDL de §5.9 aprovado em 2026-11-20; página escrita e conferida) |
| 3 | `0003_portal_person.md` | 3 | `concluída` | — (página escrita em 2026-11-20 contra `ddl-confirmado.md` §5.1/§5.6/§5.7) |
| 4 | `0004_portal_access.md` | 2 | `concluída` | — |
| 5 | `0005_game_account.md` | 5 | `concluída` | — (página escrita em 2026-11-20 contra `ddl-confirmado.md` §5.2, com o ledger de `M16`) |
| 6 | `0006_game_character.md` | 5 | `concluída` | — (DDL de §5.10 aprovado em 2026-11-20; página escrita em 2026-11-20) |
| 7 | `0007_game_session.md` | 2 | `concluída` | — (página escrita em 2026-11-20 contra `ddl-confirmado.md` §5.5) |
| 8 | `0008_game_skills.md` | 5 | `concluída` | — (DDL de §5.11 aprovado em 2026-11-20; página escrita e conferida em 2026-11-20) |
| 9 | `0009_game_inventory.md` | 3 | `concluída` | — (DDL de §5.12 aprovado em 2026-11-20; página escrita e conferida) |
| 10 | `0010_game_guild.md` | 4 | `concluída` | — (DDL de §5.13 aprovado em 2026-11-20; página escrita e conferida) |
| 11 | `0011_game_economy.md` | 10 | `concluída` | — (DDL de §5.14 aprovado em 2026-11-20; página escrita e conferida) |
| 12 | `0012_ops_delivery.md` | 1 | `concluída` | — (DDL de §5.16 aprovado em 2026-11-20, com `M25`: dreno por `SKIP LOCKED` e expurgo de 30 dias; página escrita e conferida) |
| 13 | `0013_content_taxonomy.md` | 5 | `concluída` | — (DDL de §5.17 aprovado em 2026-11-20: `slug` como PK e `item_effect` como look-up; página escrita e conferida) |
| 14 | `0014_content_item.md` | 1 | `concluída` | — (DDL de §5.18 aprovado em 2026-11-20: colunas com consumidor provado, `kind_slug` anulável e PK `item_index`; página escrita e conferida) |
| 15 | `0015_content_skill.md` | 1 | `concluída` | — (DDL de §5.19 aprovado em 2026-11-20: tempos brutos, vínculos anuláveis e PK `skill_index`; página escrita e conferida) |
| 16 | `0016-seed-taxonomy.md` | — | `bloqueada` | O DDL de `0013` fechou em 2026-11-20; falta reconferir na fonte as contagens de taxonomia listadas em `plano-migrations.md` §4. |
| 17 | `0017-seed-item-catalog.md` | — | `bloqueada` | O DDL de `0014` fechou em 2026-11-20; as contagens são de 2026-11-08/15 e faltam ser recontadas na fonte (`plano-migrations.md` §4). |
| 18 | `0018-seed-skill-catalog.md` | — | `bloqueada` | O DDL de `0015` fechou em 2026-11-20; falta a curadoria de classe e linhagem das skills, que é dado e não DDL (`plano-migrations.md` §4). |
| 19 | `0019_content_world.md` | 13 | `concluída` | — (DDL de §5.21 aprovado em 2026-11-20; página escrita e conferida) |
| 20 | `0020-seed-world.md` | — | `bloqueada` | O DDL de `0019` fechou em 2026-11-20; faltam as contagens de templates, NPCs, lojas, níveis e regiões verificadas na fonte (`plano-migrations.md` §4). A produção tem exceção datada (2026-11-27, `M34`) e as 4 cidades entram em `game.city` por este seed (`Q-M34-3`). |
| 21 | `0021_content_rules.md` | 5 | `concluída` | — (DDL de §5.22 aprovado em 2026-11-20; página escrita e conferida) |
| 22 | `0022_content_overrides.md` | 3 | `concluída` | — (DDL de §5.23 aprovado em 2026-11-21; página escrita e conferida) |
| 23 | `0023-seed-rules.md` | — | `bloqueada` | O DDL de `0021` e o de `0022` fecharam (2026-11-20 e 2026-11-21); faltam os valores de refino, gemas, SANC e composição verificados na fonte (`plano-migrations.md` §4). |
| 24 | `0024_content_maps.md` | 2 | `concluída` | — (DDL de §5.24 aprovado em 2026-11-21; página escrita e conferida) |
| 25 | `0025_content_combat.md` | 4 | `concluída` | — (DDL de §5.25 aprovado em 2026-11-21, com `RESTRICT` no item e teto `1.000.000` em `rate_bps`; página escrita e conferida) |
| 26 | `0026_content_quests_events.md` | 6 | `concluída` | — (DDL de §5.26 aprovado em 2026-11-21, com `start_minute` e `id` identity na recompensa; página escrita e conferida) |
| 27 | `0027-seed-quests-events.md` | — | `bloqueada` | O DDL de `0026` fechou em 2026-11-21; faltam a cadeia de quests do 7662 e o calendário de eventos verificados na fonte (`plano-migrations.md` §4). `M19` fechou em 2026-11-19. |
| 28 | `0028_game_quests_events.md` | 7 | `concluída` | — (DDL de §5.28 aprovado em 2026-11-21; página escrita e conferida) |
| 29 | `0029_views.md` | — | `concluída` | — (DDL de §5.29 aprovado em 2026-11-21, sem `GRANT` novo; página escrita e conferida) |
| 30 | `0030_portal_content.md` | 4 | `concluída` | — (DDL de §5.15 aprovado em 2026-11-20; página escrita e conferida) |
| 31 | `0031_portal_mfa.md` | 1 | `concluída` | — (DDL de §5.20 aprovado em 2026-11-20; página escrita e conferida) |
| 32 | `0032_admin_grants.md` | — | `concluída` | — (`M27` decidida em 2026-11-21: `GRANT`s de §5.32 ao papel `wyd_admin`, sem tabela; página escrita e conferida; emendada em 2026-11-22 com os `GRANT` da custódia) |
| 33 | `0033_ops_idempotency.md` | 1 | `concluída` | — (`M30` decidida em 2026-11-22: `ops.idempotency_record`, 7 dias de retenção; DDL de §5.33; página escrita e conferida) |
| 34 | `0034_portal_market.md` | 11 | `concluída` | — (`M31` decidida em 2026-11-22: dez tabelas em `portal` e `game.market_escrow`; DDL de §5.34; página escrita e conferida) |

## 3. Bloqueios agrupados

O detalhe de cada decisão fica em `lacunas.md`; aqui está só o que **impede a página de fechar**.

| Bloqueio | O que é | Arquivos afetados | Dono |
|---|---|---|---|
| `A17` | **Fechada em 2026-11-19: não.** Conta não é transferível nem vendável; nenhum objeto em `0005` (`ddl-confirmado.md` §5.2). | `0005` | produto + arquitetura |
| `M16` | `coin` precisa de razão de lançamento (ledger) ou snapshot auditado basta? | `0005`, `0011` | produto + arquitetura |
| `M17` | **Fechada em 2026-11-19:** `UUID` gerado pelo BFF, único em `0011`/`0012`, replay de 24 h. Nenhum arquivo trava por ela. | `0011` (fechada), `0012` (desenho de §5.16) | arquitetura + frente `wyd-portal` |
| `M30` | **Fechada em 2026-11-22:** registro de idempotência em `ops.idempotency_record`, escrito e fechado na transação da ação, retido 7 dias; `GET /actions/{key}` devolve `404` depois do expurgo. Nenhum arquivo trava por ela. | `0033` (fechada) | `wyd-db` + `wyd-ops` |
| `M31` | **Fechada em 2026-11-22:** lote `0034_portal_market.sql`, dez tabelas em `portal` e uma em `game`, com a emenda de `GRANT` em `0032`. Nenhum arquivo trava por ela. | `0034` (fechada) | `wyd-db` + arquitetura |
| `M25` | **Fechada em 2026-11-20:** o dreno reivindica com `SELECT … FOR UPDATE SKIP LOCKED` em transação curta; `applied`/`failed` expurgadas 30 dias após `closed_at`, `pending` nunca. Nenhum arquivo trava por ela. | `0012` (fechada) | arquitetura + frente `wyd-server` |
| `A19` | Loja e cash existem? Entra em `portal`, fica fora do schema, ou não existe. | `0011` | produto |
| `A14` | Soft-delete de personagem derruba a sessão viva? | `0006` | arquitetura |
| Dado de seed não verificado | Os 6 seeds são escritos à mão, com os valores conferidos no cliente 7662; ficam fora da onda A (`Q-M33-6`), não há gerador previsto — **exceto a exceção pontual de 2026-11-27 (`M34`) para `0017`/`0020`, com script descartável** — e valor herdado não é verdade sem reconferência (`plano-migrations.md` §4). O que falta é dado, não código. | `0016`–`0018`, `0020`, `0023`, `0027` | arquitetura + curadoria de conteúdo |

**Bloqueios que não impedem DDL** — citados aqui porque aparecem nas páginas, mas não mudam coluna,
constraint nem índice: `B1` (provedor de e-mail: quando `0004`/`0005` passam a ser usadas de verdade),
`M19` (senha de dev documentada nos seeds), `A8` (limiar de bloqueio, que é política de código), `A18`
(cadastro dentro do jogo: decide a **rota**, não a forma; trava a ADR de fronteira pessoa × conta) e
`A21` (troca de e-mail: o `purpose = 'email_change'` já está no CHECK de `0004`; falta o fluxo que emite
o token, que é rota e emissor, não coluna).

## 4. Contagem

| Medida | Valor | Fonte |
|---|---:|---|
| Arquivos planejados | 34 | `plano-migrations.md` §3 (29 originais + os adendos `0030`, `0031` e `0032` + os lotes `0033` e `0034`) |
| Tabelas planejadas | 109 | soma da coluna `Tab.` acima |
| Arquivos de seed | 6 | `plano-migrations.md` §4 |
| Páginas `concluída` hoje | 28 | `0001` a `0015` + `0019` + `0021` + `0022` + `0024` + `0025` + `0026` + `0028` + `0029` + `0030` + `0031` + `0032` + `0033` + `0034` |
| Páginas `pronta` (a escrever) | 0 | — |
| Arquivos aguardando aprovação | 0 | os cinco lotes de 2026-11-21 foram aprovados |
| Arquivos com DDL parcial | 0 | — |
| Arquivos bloqueados | 6 | `0016`, `0017`, `0018`, `0020`, `0023` e `0027` (dado de seed não verificado) |
| Soma que fecha | 34 | 28 `concluída` + 0 `pronta` + 0 `aguardando aprovação` + 0 `parcial` + 6 `bloqueada` |
| Tabelas com DDL fechado | 109 de 109 | `ddl-confirmado.md` §1 |
| Tabelas desenhadas e não confirmadas | 0 | — (as 20 de `0022`, `0025`, `0026` e `0028` foram aprovadas em 2026-11-21 e as 12 de `0033`/`0034` em 2026-11-22) |

Número é **planejado**, não medido: a contagem real sai do catálogo depois que a série existir
(`M20`). Nenhum arquivo `.sql` desta série foi escrito e nenhum Postgres rodou.

## 5. O que toda página de migration precisa ter

O critério é o do pedido original: **o desenvolvedor executa a migration só com a página**, sem abrir
outro documento. Página sem qualquer um destes itens não é `concluída`:

1. **Cabeçalho** de cinco campos (`Pergunta`, `Público`, `Dono`, `Base`, `Revisado em`), como em
   `docs/convencoes.md`.
2. **Identificação**: número, nome do arquivo `.sql`, schema, posição na ordem e arquivos de que
   depende (com a razão da dependência).
3. **Objetivo em uma frase**: o que passa a existir no banco depois deste arquivo.
4. **DDL completo do `up`**, pronto para colar, com toda constraint e todo índice nomeados — o texto
   é o de `ddl-confirmado.md` quando o arquivo está fechado ali, e nunca uma paráfrase.
5. **DDL completo do `down`**, na ordem inversa exata do `up`, com o que é derrubado e o que é
   revogado.
6. **Tabela de colunas**: nome, tipo, nulidade, default e o que a coluna significa — a frase que o
   dicionário de dados vai reaproveitar.
7. **Contratos e invariantes**: cada constraint com o estado que ela proíbe e o código de erro que o
   chamador recebe (`23505`, `23503`, `23514`).
8. **Permissões**: quem lê, quem escreve, quem **não** pode — com o **efeito por papel**. `GRANT`/`REVOKE`
   explícitos **quando o arquivo tiver**; quando não tiver, a página diz **por que** (herda do
   `ALTER DEFAULT PRIVILEGES` do `0001`) e qual é o efeito resultante em cada papel. Página que só
   lista papéis sem dizer o efeito não cumpre o item.
9. **Prova de execução**: o comando `psql` que aplica o arquivo e as consultas que devolvem o
   esperado (contagem por schema, violação proposital, `permission denied`).
10. **O que este arquivo não decide**, com o ID da lacuna quando houver.

Enquanto o item 4 não existir inteiro, a linha deste checklist fica `bloqueada` — é o que impede
página escrita por dedução virar DDL sem decisão.

## 6. Regra de manutenção

- Fecha-se uma linha por vez, com a página criada em `migrations/` e o DDL conferido contra
  `ddl-confirmado.md`. Linha sem página não fecha.
- Quando uma lacuna de §3 fechar em `lacunas.md`, este checklist muda o **estado** e a coluna
  `Bloqueio` — nunca a coluna `Tab.` nem a contagem de arquivos, que vêm do plano.
- `aguardando aprovação` vira `pronta` (página ainda não escrita) ou `concluída` (página já escrita e
  conferida) só com o "sim" do dono registrado em `ddl-confirmado.md`; sem ele, o lote continua
  desenhado e a contagem de DDL fechado não sobe. Reprovado, a linha volta a `parcial` com o motivo da
  recusa. O `0030` fez o segundo caminho em 2026-11-20 e o `0024` em 2026-11-21.
- Arquivo novo na série entra aqui com o próximo número livre e passa por `plano-migrations.md`
  antes; este checklist não cria migration.
- A ordem de escrita não foi a ordem da tabela: os arquivos `pronta` primeiro (`0003`, `0005`, `0007`),
  depois o lote de `0002`, que destrava o corte vertical, depois o resto. Hoje só restam os 6 seeds.
- Pré-requisito de ambiente (papel de cluster) **não** mora nesta pasta nem altera a contagem de 30
  arquivos: é artefato gerado por `wyd-db provision render` (hoje 34), especificado em
  `src/provision/spec.md` (**planejamento**). Se um pré-requisito passar a mudar schema, ele vira
  migration numerada — com `up`/`down` e decisão em `plano-migrations.md`.

*Controle de documentação. Nenhum SQL foi escrito e nenhum arquivo de `crates/` foi alterado.*
