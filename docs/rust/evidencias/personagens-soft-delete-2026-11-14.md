# Personagens `Cantarelli`, soft delete e limite de slots (2026-11-14)

> Pergunta: por que existem seis linhas com o nome `Cantarelli` e qual delas o cliente carrega?
> Resposta medida no Postgres da stack e no código do canal; nada foi alterado no banco.

## 1. O que existe hoje

```bash
docker exec wyd-db-1 psql -U admin -d wyd -c \
  "select c.id, c.account_id, a.login, c.slot, c.level, c.name,
          (c.deleted_at is not null) as apagado, c.created_at, c.updated_at
     from characters c join accounts a on a.id = c.account_id
    where c.name = 'Cantarelli' order by c.id"
```

| id | conta | slot | nível | apagado (`deleted_at`) | criado |
|---:|---|---:|---:|---|---|
| 57 | `admin` (1) | 0 | 1 | **sim** | 2026-11-10 02:23:21Z |
| 137 | `admin` (1) | 0 | 1 | **sim** | 2026-11-11 00:11:53Z |
| 150 | `admin` (1) | 0 | 1 | **sim** | 2026-11-11 15:46:00Z |
| 151 | `admin` (1) | 0 | 1 | **sim** | 2026-11-11 22:27:34Z |
| 152 | `admin` (1) | 0 | 1 | **sim** | 2026-11-11 23:43:36Z |
| 164 | `admin` (1) | 0 | 1 | **não** | 2026-11-13 23:39:05Z |

Correção do registro anterior: **não são seis linhas vivas**. São seis linhas com o mesmo nome;
**cinco estão soft-deleted** e só a **164** está viva (`select count(*) filter (where name =
'Cantarelli' and deleted_at is null) from characters` → `1`). O total da tabela é 190 linhas,
129 vivas.

## 2. Por que o nome se repete

Os dois índices únicos de `characters` são **parciais** por `deleted_at IS NULL`:

```sql
characters_name_unique         UNIQUE (name)             WHERE deleted_at IS NULL
characters_account_slot_unique UNIQUE (account_id, slot) WHERE deleted_at IS NULL
```

`Characters::soft_delete` faz `UPDATE characters SET deleted_at = now()` — não apaga a linha.
Com isso, apagar libera **nome e slot** ao mesmo tempo, e a próxima criação grava uma linha nova
(id novo) no slot 0. O histórico de cada recriação fica na tabela; a linha antiga nunca volta.

O `_MSG_DeleteCharacter` só marca depois de conferir o Argon2id da conta e o nome enviado no
pacote (`SelectionStore::delete` → `crate::auth::verify_login_password`). As linhas 57…152 são,
portanto, versões anteriores apagadas de propósito (a 152 foi apagada às `23:38:58Z` e a 164
criada 7 s depois, na mesma sessão de 2026-11-13).

## 3. Qual linha o cliente carrega

Só a **164**. O caminho é curto e não tem desempate por “última criada”:

1. `PgBackend::list_by_account` → `WHERE account_id = $1 AND deleted_at IS NULL ORDER BY slot`;
   o `CNFAccountLogin` desenha as posições 0–3 de `STRUCT_SELCHAR` a partir desse roster. As
   cinco apagadas não entram no corpo.
2. O `_MSG_CharacterLogin` traz `slot = 0`; `SelectionStore::select` → `find(account_id, slot)`
   procura no **mesmo** roster filtrado. Slot 0 tem exatamente uma linha viva: a 164.
3. Prova no log do canal 1 (`docker logs wyd-server-canal-1-1`):

```
level=info event=character result=ok action=select account=admin slot=0 name=Cantarelli class=0 level=1 id=164 force=1 entity=6111 client_id=1
```

As linhas 57…152 não são alcançáveis por login enquanto `deleted_at` estiver preenchido; para
voltar a existir um personagem com esse nome é preciso criar outro (id novo).

## 4. Limite por conta e o caso Arch

- **Limite de personagens = 4** (`slot 0..=3`). No código é a constante `MAX_SLOTS = 3`
  (`crates/wyd-db/src/model/character.rs`), usada por `free_slot` em `0..=MAX_SLOTS`; no banco é
  o CHECK `characters_slot` (`slot >= 0 AND slot <= 3`). Não é configuração de conteúdo.
- **Criação**: o servidor honra o slot pedido se estiver livre; se estiver ocupado grava no
  primeiro livre; conta com os quatro ocupados devolve `SelectionError::Full` →
  `_MSG_NewCharacterFail`. O índice parcial é a palavra final contra corrida.
- **Soft delete devolve a vaga**: como o índice de slot é parcial, apagar libera o slot para uma
  criação nova. É isso que permite recriar `Cantarelli` no slot 0 sem `TRUNCATE` nem `DELETE`.
- **Arch precisa de slot livre.** `docs/wyd/classes.md` §4 (CONFIRMADO) diz que Mortal e Arch
  coexistem como **duas entidades de personagem com o mesmo nome**, ligadas por
  `characters.arch_of_id`; cada entidade é uma linha e portanto ocupa um slot dos quatro. Conta
  com os quatro slots ocupados não consegue materializar o Arch sem apagar (soft delete) um
  personagem antes.
- **Conflito de schema ainda não resolvido (débito declarado, não regra):**
  `characters_name_unique` é único **global entre linhas vivas**, não por conta. Duas linhas
  vivas com o mesmo nome — exatamente o caso Mortal + Arch — violam esse índice hoje. `tier`
  (default `mortal`) e `arch_of_id` existem e têm CHECK/FK, mas **nenhum código escreve neles**:
  todas as 190 linhas estão com `arch_of_id IS NULL`. Fechar o Arch exige decisão de produto
  (nome do Arch com sufixo? índice parcial que exclua a linha de Arch? outra chave?) e migração;
  não foi alterado nada nesta investigação.

## 5. O que ficou fora

- Nenhuma escrita no banco: só `SELECT` e leitura de índices/constraints.
- O comportamento do seletor do 7662 com mais de um slot ocupado não foi recapturado aqui; a
  conclusão de “qual linha carrega” é do roster filtrado + `CharacterLogin.slot`, provada pelo
  log acima, não por print.
