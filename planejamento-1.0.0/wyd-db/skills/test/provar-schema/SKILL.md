---
name: provar-schema
description: Prova que um arquivo da série do wyd-db faz o que a página diz, nas quatro camadas — estrutural, contrato, banco real e ciclo up/down/up. Use antes de fechar issue de migration, ao reproduzir falha de CI com Postgres 17, e quando for preciso saber se a série inteira sobe do zero.
---

# Provar schema

## Quando usar

- Migration nova ou corrigida vai para merge.
- O CI falhou e é preciso reproduzir local com PostgreSQL 17.

Não use para testar comportamento de runtime (`game`/`portal`): ali a prova é de aplicação.

## Pré-requisitos

- Docker disponível; `make db-up` sobe o serviço `db` (perfil `db` do Compose).
- Série contígua em `crates/wyd-db/migrations/`.
- Papéis de `db/roles.sql` criados no banco de teste (`wyd_owner`, `wyd_migrator`, `wyd_game`,
  `wyd_portal`, `wyd_admin`, `wyd_read`) — o preflight do runner recusa outro papel.

## Passos

1. **Camada estrutural** — o arquivo aplica num banco vazio, na ordem da série, sem `ERROR`:

   ```bash
   make db-up
   for f in crates/wyd-db/migrations/*.sql; do
     awk '/^-- migrate:up/{on=1;next} /^-- migrate:down/{on=0} on' "$f" \
       | PGPASSWORD=test psql -h 127.0.0.1 -U postgres -v ON_ERROR_STOP=1 -X -q || break
   done
   ```

2. **Camada de contrato** — o que a página promete existe e tem o tipo prometido:
   `\d content.<tabela>`, contagem de índices, constraints com o nome do padrão.
3. **Camada de banco real** — os testes de integração do crate rodam contra o PG17: `cargo test -p wyd-db`.
4. **Camada de ciclo** — aplicar `down` e aplicar `up` de novo devolve o mesmo estado; se sobrar
   objeto, o `down` está incompleto.
5. **Registre a evidência** na página (§ prova de execução) e no comentário da issue: comando, saída
   relevante e data.

## Como verificar

| Alegação | Prova |
|---|---|
| série sobe do zero | laço do passo 1 sem `ERROR` |
| ordem aceita pelo runner | `cargo test -p wyd-db` sem `NotContiguous` |
| `down` reverte | `to_regclass` nulo depois do `down` |
| nada quebrou no resto | `make gate-fast` |

## Quando parar e perguntar

- A falha é de dado, não de DDL (constraint recusando valor real): pare e leve ao dono — schema não
  se ajusta ao dado sem decisão.
- A falha só reproduz no CI: registre a diferença de ambiente (imagem, papel, versão) antes de mexer.

## Armadilhas

- **Provar com `psql -f` do arquivo inteiro**: os dois blocos rodam juntos e o `down` desfaz o teste.
- **Teste de integração sem banco**: verde falso; a camada 3 exige PG17 de verdade.
- **Concluir pelo `up`**: sem o ciclo, `down` quebrado passa.
- **`make gate` completo em vez de `gate-fast`** a cada passo: use o curto no ciclo e o completo ao fim.
