> Pergunta: "o que `content.event_schedule` guarda de cada janela e por que o horário é minuto do dia, não TIME?"
> Público: Desenvolvedor do `wyd-db` e revisor de schema
> Dono: equipe do wyd-db
> Base: `ddl-confirmado.md` §5.26; `migrations-checklist.md` §2; `plano-migrations.md` §3
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — DDL confirmado, série não aplicada; o seed `0027` segue `bloqueada`.

## Papel

Janela semanal de um evento: dia da semana, minuto de início, duração, fuso e periodicidade em semanas.
É filha de `content.world_event` e o único lugar da série que diz **quando** um evento acontece.

## Colunas

| Coluna | Tipo | Nulo | Default | Observação |
|---|---|---|---|---|
| `id` | `BIGINT` identity | não | gerado | identidade da linha (`P-03`) |
| `event_code` | `TEXT` | não | — | evento agendado; FK `CASCADE` |
| `weekday` | `SMALLINT` | não | — | dia da semana, 0–6 |
| `start_minute` | `SMALLINT` | não | — | minuto do dia, 0–1439 |
| `duration_minutes` | `SMALLINT` | não | — | duração, maior que zero |
| `timezone` | `TEXT` | não | `'America/Sao_Paulo'` | fuso IANA da janela |
| `every_n_weeks` | `SMALLINT` | não | `1` | periodicidade, 1–8 |
| `enabled` | `BOOLEAN` | não | `true` | desliga sem apagar |

Sem carimbos: definição de conteúdo, seed-only.

## Chave primária

`pk_event_schedule (id)` — identidade sintética. A agenda não tem chave natural: a mesma hora pode
repetir em semanas diferentes, e a identidade evita promover `(evento, dia, minuto)` a PK quando a
periodicidade muda.

## FKs de saída e de entrada

- **Saída, `CASCADE` (1 FK):** `fk_event_schedule_event`: `event_code → content.world_event (code)`,
  `ON UPDATE RESTRICT`, `ON DELETE CASCADE` — a janela não existe sem o evento.
- **Entrada:** nenhuma; nenhuma tabela da série aponta para a agenda.

## Unicidade

`uq_event_schedule UNIQUE (event_code, weekday, start_minute)` — um evento não tem duas janelas no mesmo
dia e minuto. O único **ignora `timezone`**: duas linhas iguais em fusos diferentes são recusadas, e não há
como agendar o mesmo horário em dois fusos para o mesmo evento.

## `CHECK`s

| Constraint | Recusa |
|---|---|
| `ck_event_schedule_weekday` | `weekday` fora de 0–6 |
| `ck_event_schedule_inicio` | `start_minute` fora de 0–1439 |
| `ck_event_schedule_duracao` | `duration_minutes` menor ou igual a zero |
| `ck_event_schedule_fuso` | `timezone` vazio, só com espaço, ou acima de 64 caracteres |
| `ck_event_schedule_periodo` | `every_n_weeks` fora de 1–8 |

O `CHECK` de fuso só limita tamanho: nome IANA inválido passa no schema e falha no runtime.

## Índices

Nenhum além do índice da PK e do criado pelo `uq_event_schedule`. A leitura por evento usa o prefixo
`event_code` do único; não há índice em `weekday` nem em `start_minute` isolado.

## Armadilha de leitura

`start_minute` é **minuto do dia**, não hora: 19h30 é `1170`, e 1439 é 23h59. `weekday` é 0–6 e o
significado do zero não está no schema — o domínio diz domingo (`dominio-eventos.md`). O fuso é **por
linha**: dois eventos do mesmo dia podem ter fusos diferentes, e o horário não anda quando o host muda.
`duration_minutes` pode atravessar a meia-noite e o schema não impede; se a janela termina no dia seguinte
é leitura do runtime. E `every_n_weeks > 1` não cria exceção de semana: o schema guarda o número, não a
data de referência.

## Papéis

- **Escreve:** `wyd_migrator`, no seed de quests e eventos (`0027`, `bloqueada`,
  `migrations-checklist.md` §2); nenhum `GRANT` de escrita em runtime.
- **Leem:** `wyd_game`, `wyd_portal` e `wyd_read` — `SELECT` herdado do default privilege de `content`
  (`0001_bootstrap.md` §7).
- **Não lê:** `wyd_admin` — `0032` não concede `content` (`migrations/0032_admin_grants.md` §5).

## Migration

Forma (dona): `migrations/0026_content_quests_events.md` — arquivo `0026_content_quests_events.sql`.
