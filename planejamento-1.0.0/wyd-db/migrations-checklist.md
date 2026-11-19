> Pergunta: "quais migrations da série 1.0.0 já podem ser documentadas e qual é o bloqueio de cada uma?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §3, `ddl-confirmado.md` §2, `lacunas.md` §2
> Revisado em: 2026-11-18

Este é o **único controle** do andamento da documentação da série `migrations/`. Ele diz, para
cada um dos 29 arquivos planejados, qual página deve existir, quantas tabelas o arquivo cria (`—` quando
o arquivo é só seed) e se há bloqueio para escrevê-la. O plano (`plano-migrations.md`) é o dono da
**ordem e do escopo**; o texto DDL fechado é dono de `ddl-confirmado.md`; a **decisão** de cada
bloqueio é dona de `lacunas.md`. Aqui não se decide nada: aqui se acompanha.

Este arquivo fica na **raiz** de `planejamento-1.0.0/wyd-db/`, fora de `migrations/`: a pasta guarda só
página de migration, uma por arquivo `.sql` da série.

## 1. Como ler este checklist

| Estado | Significa |
|---|---|
| `pronta` | DDL do arquivo inteiro fechado em `ddl-confirmado.md`; a página pode ser escrita agora, do começo ao fim |
| `bloqueada` | falta desenho de coluna, decisão de lacuna ou seed gerado; a página **não** pode ser escrita inteira |
| `parcial` | uma ou mais tabelas do arquivo têm DDL fechado, o resto não — ou todas as tabelas atuais estão fechadas mas o escopo do arquivo pode crescer por lacuna aberta (`0005`); a página só fecha quando o arquivo fecha |
| `concluída` | página escrita em `migrations/`, conferida contra `ddl-confirmado.md` e com os 10 itens de §5 |

Marcar uma linha como concluída exige que a página correspondente exista em `migrations/` com o
conteúdo mínimo de §5. Enquanto o bloqueio não fechar, a linha fica `bloqueada` mesmo que a página
exista em rascunho — não existe estado "quase".

### 1.1 Nomes de página

A estrutura do diretório vem do radical do arquivo `.sql`: a página troca **cada `_` por `-`**
(`kebab-case`, sem acento), e nenhum underscore do arquivo sobrevive no nome — o par arquivo/página
fica inequívoco:

`0004_portal_access.sql` → `0004-portal-access.md`

Nos 21 arquivos que criam tabela em um schema, o radical é `[numero]_[schema]_[nome]`, e a página sai
`[numero]-[schema]-[nome].md`. Dois casos não seguem essa forma: os 6 arquivos de seed, cujo segmento
do meio é `seed` e não schema (`0016_seed_taxonomy.sql` → `0016-seed-taxonomy.md`, que semeia
`content`); e `0001` e `0029`, que não têm schema próprio — `0001` cria os quatro schemas e `0029`
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

## 2. Checklist (29 arquivos, 92 tabelas)

| # | Página | Tab. | Estado | Bloqueio |
|---|---|---:|---|---|
| 1 | `0001-bootstrap.md` | 1 | `concluída` | — |
| 2 | `0002-ops-audit.md` | 4 | `parcial` | `config_revision` e `config_change` só têm nome no plano; falta desenho de coluna. `audit_log` e `login_attempt` estão fechadas. |
| 3 | `0003-portal-person.md` | 3 | `pronta` | — (as três tabelas fechadas em `ddl-confirmado.md` §5.1/§5.6/§5.7; `D5` × `P-22` resolvido por `M7`) |
| 4 | `0004-portal-access.md` | 2 | `concluída` | — |
| 5 | `0005-game-account.md` | 5 | `pronta` | As 5 tabelas estão fechadas: 4 atuais + `game.account_ledger` (`M16`); `A17` decidiu que não há transferência, então nenhum objeto de titularidade. |
| 6 | `0006-game-character.md` | 5 | `bloqueada` | 5 tabelas sem coluna desenhada; os campos vêm do `STRUCT_MOB` e `A14` (soft-delete × sessão viva) muda a semântica do delete. |
| 7 | `0007-game-session.md` | 2 | `pronta` | — (as duas tabelas fechadas em `ddl-confirmado.md` §5.5; `A10`, a chave composta e a fusão `M23` decididas em 2026-11-18) |
| 8 | `0008-game-skills.md` | 5 | `bloqueada` | 5 tabelas sem coluna desenhada. |
| 9 | `0009-game-inventory.md` | 3 | `bloqueada` | 3 tabelas sem coluna desenhada. |
| 10 | `0010-game-guild.md` | 4 | `bloqueada` | 4 tabelas sem coluna desenhada. |
| 11 | `0011-game-economy.md` | 10 | `bloqueada` | 10 tabelas sem coluna desenhada; `A19` decidiu 6 de loja/cash em `portal` e 4 de jogo em `game`; `M17` acrescenta `idempotency_key`. |
| 12 | `0012-ops-delivery.md` | 1 | `bloqueada` | A tabela existe no legado, mas o desenho novo não foi aprovado: a idempotência de crédito (`M17`) ainda não tem dono nem formato. |
| 13 | `0013-content-taxonomy.md` | 5 | `bloqueada` | Nenhum DDL aprovado para o schema `content` (5 tabelas + `fn_fold`). |
| 14 | `0014-content-item.md` | 1 | `bloqueada` | Só a PK e dois detalhes estão definidos; as demais colunas vêm do `STRUCT_MOB` e não foram reescritas no padrão. |
| 15 | `0015-content-skill.md` | 1 | `bloqueada` | Mesma lacuna de `0014`: colunas herdadas do `STRUCT_MOB` sem versão aprovada. |
| 16 | `0016-seed-taxonomy.md` | — | `bloqueada` | Depende do DDL de `0013` e do gerador de seed (`plano-migrations.md` §1, adiado). |
| 17 | `0017-seed-item-catalog.md` | — | `bloqueada` | Sem `0014` fechada e sem gerador de seed liberado; as contagens são as medidas em 2026-11-08/15 e só valem depois de recontadas. |
| 18 | `0018-seed-skill-catalog.md` | — | `bloqueada` | Sem `0015` fechada e sem gerador de seed liberado. |
| 19 | `0019-content-world.md` | 13 | `bloqueada` | 13 tabelas sem coluna desenhada. |
| 20 | `0020-seed-world.md` | — | `bloqueada` | Sem `0019` fechada e sem gerador de seed liberado. |
| 21 | `0021-content-rules.md` | 5 | `bloqueada` | 5 tabelas sem coluna desenhada. |
| 22 | `0022-content-overrides.md` | 3 | `bloqueada` | 3 tabelas sem coluna desenhada. |
| 23 | `0023-seed-rules.md` | — | `bloqueada` | Depende de `0021`/`0022` e do gerador de seed. |
| 24 | `0024-content-maps.md` | 2 | `bloqueada` | 2 tabelas sem coluna desenhada. |
| 25 | `0025-content-combat.md` | 4 | `bloqueada` | 4 tabelas sem coluna desenhada. |
| 26 | `0026-content-quests-events.md` | 6 | `bloqueada` | 6 tabelas sem coluna desenhada. |
| 27 | `0027-seed-quests-events.md` | — | `bloqueada` | Sem `0026` fechada e sem gerador de seed liberado. `M19` (senha de dev do seed) ainda é proposta. |
| 28 | `0028-game-quests-events.md` | 7 | `bloqueada` | 7 tabelas sem coluna desenhada; é o único bloco de `game` com FK para `content`, por isso fecha depois de `0026`. |
| 29 | `0029-views.md` | — | `bloqueada` | As 2 views têm propósito decidido, não lista de colunas; dependem de todos os arquivos anteriores. |

## 3. Bloqueios agrupados

O detalhe de cada decisão fica em `lacunas.md`; aqui está só o que **impede a página de fechar**.

| Bloqueio | O que é | Arquivos afetados | Dono |
|---|---|---|---|
| Desenho de coluna | Nenhuma coluna aprovada em documento para as tabelas do arquivo. É modelagem, não lacuna de produto. | `0006`, `0008`–`0012`, `0013`–`0015`, `0019`, `0021`, `0022`, `0024`, `0025`, `0026`, `0028`, `0029` | arquitetura |
| `A17` | **Fechada em 2026-11-19: não.** Conta não é transferível nem vendável; nenhum objeto em `0005` (`ddl-confirmado.md` §5.2). | `0005` | produto + arquitetura |
| `M16` | `coin` precisa de razão de lançamento (ledger) ou snapshot auditado basta? | `0005`, `0011` | produto + arquitetura |
| `M17` | Chave de idempotência sem dono, formato e janela de replay: trava crédito e entrega. | `0011`, `0012` | arquitetura + frente `wyd-portal` |
| `A19` | Loja e cash existem? Entra em `portal`, fica fora do schema, ou não existe. | `0011` | produto |
| `A14` | Soft-delete de personagem derruba a sessão viva? | `0006` | arquitetura |
| Gerador de seed | Os 6 arquivos de seed são gerados; o PR de ferramentas está adiado (`plano-migrations.md` §1) e não há gerador liberado. | `0016`–`0018`, `0020`, `0023`, `0027` | arquitetura |

**Bloqueios que não impedem DDL** — citados aqui porque aparecem nas páginas, mas não mudam coluna,
constraint nem índice: `B1` (provedor de e-mail: quando `0004`/`0005` passam a ser usadas de verdade),
`M19` (senha de dev documentada nos seeds), `A8` (limiar de bloqueio, que é política de código), `A18`
(cadastro dentro do jogo: decide a **rota**, não a forma; trava a ADR de fronteira pessoa × conta) e
`A21` (troca de e-mail: o `purpose = 'email_change'` já está no CHECK de `0004`; falta o fluxo que emite
o token, que é rota e emissor, não coluna).

## 4. Contagem

| Medida | Valor | Fonte |
|---|---:|---|
| Arquivos planejados | 29 | `plano-migrations.md` §3 |
| Tabelas planejadas | 92 | soma da coluna `Tab.` acima |
| Arquivos de seed | 6 | `plano-migrations.md` §4 |
| Páginas `concluída` hoje | 2 | `0001`, `0004` |
| Páginas `pronta` (a escrever) | 3 | `0003`, `0005`, `0007` |
| Arquivos com DDL parcial | 1 | `0002` (2 de 4) |
| Arquivos bloqueados | 23 | `0006`, `0008`–`0029` (`0007` fechou em 2026-11-18) |
| Soma que fecha | 29 | 2 `concluída` + 3 `pronta` + 1 `parcial` + 23 `bloqueada` |
| Tabelas com DDL fechado | 15 de 92 | `ddl-confirmado.md` §1 |

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
- Arquivo novo na série entra aqui com o próximo número livre e passa por `plano-migrations.md`
  antes; este checklist não cria migration.
- A ordem de escrita não é a ordem da tabela: os arquivos `pronta` primeiro (`0003`, `0005`, `0007`),
  depois o `parcial` que destrava o corte vertical (`0002`), depois o resto.
- Pré-requisito de ambiente (papel de cluster) **não** mora nesta pasta nem altera a contagem de 29
  arquivos: é artefato gerado por `wyd-db provision render`, especificado em
  `src/provision/spec.md` (**planejamento**). Se um pré-requisito passar a mudar schema, ele vira
  migration numerada — com `up`/`down` e decisão em `plano-migrations.md`.

*Controle de documentação. Nenhum SQL foi escrito e nenhum arquivo de `crates/` foi alterado.*
