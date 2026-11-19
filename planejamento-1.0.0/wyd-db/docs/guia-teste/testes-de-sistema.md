> Pergunta: "como provo contra Postgres real?"
> Público: Desenvolvedor do `wyd-db`
> Dono: equipe do wyd-db
> Base: `plano-migrations.md`, `src/provision/spec.md`
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18)

Teste de sistema é o que roda contra um PostgreSQL de verdade, com a série aplicada. Ele existe
para provar o que o teste de contrato não alcança: transação, privilégio, `CHECK` de verdade,
concorrência e `down`. **Não** substitui contrato nem unitário, e não é lugar de testar regra que
não depende do banco — a divisão de camadas e o que cada uma prova são de `padrao-banco.md` §10; aqui
fica como se roda a camada de sistema.

## Subida do banco

| Item | Regra |
|---|---|
| versão | PostgreSQL 17, a mesma da imagem de produção |
| origem | container descartável, sem volume persistente |
| preparo | banco vazio → provisionamento → série; nunca um banco "já quente" |
| limpeza | o banco morre com a execução; recriar é mais barato que limpar |
| pré-requisito | se o provisionamento não rodou, o teste falha dizendo isso — não pula |

## DSN de teste

`WYD_DB_URL_TEST` conecta como `wyd_migrator` num banco descartável (`src/provision/spec.md` §3);
ausente, o teste falha nomeando a variável — nunca passa em silêncio. Ela não é lida por `spec.rs`
nem por binário de produção: só pela suíte.

> [!WARNING] Sem banco, sem verde
> `SKIP` por variável ausente é reprovado pelo gate (`gates-e-anti-skip.md`). Um teste que não
> pode rodar é falha declarada com o motivo, não um resultado neutro.

## Paralelismo

O desenho pretende uma **base por binário de teste**, derivada do mesmo provisionamento, para que
duas suítes não disputem o mesmo ledger nem o mesmo advisory lock. Dentro do binário, os testes
rodam em paralelo porque cada um escreve só com a sua chave. Enquanto o harness da série não
existir, este parágrafo é intenção, não fato — a página diz o que foi medido quando houver medição.

## Isolamento entre testes

| Prática | Regra |
|---|---|
| dado | cada teste cria o seu, com chave própria, e limpa só a sua chave |
| estado global | proibido: nada de `TRUNCATE` na base inteira entre testes |
| ordem | nenhum teste depende de outro ter rodado antes |
| conteúdo | o catálogo é semeado pela série; teste que precisa de contagem fixa usa dado próprio |
| privilégio | teste de fronteira usa a DSN do papel testado, não a do migrator |

## Tempo limite

Todo teste que espera concorrência tem limite declarado. O limite é da operação, não da suíte: se
um teste estoura sozinho com o banco ocioso, ele está errado. A suíte inteira tem teto próprio para
que uma trava de advisory lock apareça como falha, e não como execução pendurada.

## Como evitar flaky

- **Sem `sleep`.** Sincronize por evento, `pg_stat_activity`, `NOTIFY` ou chamada que retorna estado —
  a proibição é do plano diretor (`wyd-db-1.0.0.md` §5.9), junto com o `SKIP`; aqui fica como se
  escreve o teste que não precisa dela.
- **Sem dependência de relógio de parede** para ordem: `timestamptz` grava o instante, o teste compara
  o que foi gravado, não "quantos milissegundos passaram".
- **Sem `ORDER BY` implícito:** consulta sem ordem é resultado sem contrato.
- **Seed determinístico:** mesma série, mesma contagem, mesmo `content_version`.
- **Falha reprodutível:** o teste imprime a chave que criou e o SQL que falhou, para que a segunda
  execução seja igual à primeira.
