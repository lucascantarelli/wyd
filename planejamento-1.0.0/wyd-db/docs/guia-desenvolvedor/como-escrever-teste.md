> Pergunta: "que teste eu escrevo para esta mudança e o que ele não pode fazer?"
> Público: Desenvolvedor e revisor de PR
> Dono: equipe do `wyd-db`
> Base: `wyd-db-1.0.0.md` §5.9, §6.13 e §6.17; `docs/guia-teste/`; `plano-de-testes.md`
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

Escolha o teste pelo **que ele prova**, não pelo arquivo que você mudou. Contrato prova forma sem
banco; sistema prova comportamento contra PostgreSQL 17; unitário prova uma regra pura. O teste não
pode depender de id mágico, `sleep`, ordem de execução ou estado global. Se o cenário precisa de
fixture que ainda não existe, a página declara a dependência em vez de inventar um dado.

## 1. Escolha do alvo

| O que você mudou | Alvo | Precisa de banco |
|---|---|---|
| parser, versão, checksum, `Sql` ou `plan` | unitário no próprio módulo | não |
| ordem do runner, lock, commit por arquivo | `contrato_migrations` com portas falsas | não |
| aplicação real, drift, `down`, concorrência | `sistema_pg` | sim |
| `GRANT`, leitura cruzada, `permission denied` | `sistema_sessoes` | sim |
| capacidade nova | matriz `CAP-nn` + alvo que a cobre | depende do alvo |

Nenhum teste de unidade abre conexão; nenhum teste de sistema usa `FakeLedger` para provar o banco.
A divisão completa está em [`../guia-teste/piramide-e-alvos.md`](../guia-teste/piramide-e-alvos.md).

## 2. Regras de escrita

| Regra | Por quê | Como revisar |
|---|---|---|
| nada de id mágico | o dado criado em outro teste não existe no seu | a factory devolve o id; o teste usa o retorno |
| nada de `sleep` | tempo de parede não prova ordem | use porta/barreira ou evento observável |
| nada de estado global | testes em paralelo se contaminam | crie e limpe por chave própria |
| nome pelo comportamento | “aplica uma vez e reaplica sem mudar” diz o que prova | o nome descreve o cenário e o resultado |
| falha nomeia o arquivo/versão | o revisor acha a origem sem abrir o teste | asserção contém `Version` ou nome do arquivo |
| `SKIP` proibido | ausência de ambiente não é verde | falhe nomeando a variável; ver `gates-e-anti-skip` |

Um teste que só confirma `is_ok()` sem verificar o efeito não prova contrato. Um teste que lê o mesmo
código que produziu o valor (golden gerado na hora) também não prova nada.

## 3. Fixture e factory

A suíte ainda não tem `tests/common/`, factories ou fixtures publicadas. O desenho mínimo é:

- **factory por agregado:** cria a linha mínima e devolve o identificador;
- **fixture de conteúdo:** um catálogo pequeno, explícito, com versão própria;
- **limpeza por chave:** cada teste remove o que criou; não há `TRUNCATE` global;
- **dado sensível:** fictício, sem PII real e sem segredo;
- **concorrência:** duas conexões reais, sem `sleep` para “dar tempo”.

Enquanto a suíte não existir, a issue de teste declara a dependência; não se cria fixture na página.

## 4. Estrutura de um teste de contrato

```text
given: uma série de fixture com um defeito conhecido
when:  o alvo de contrato lê a série
then:  falha com o código e o arquivo esperados; a mensagem não é genérica
```

O alvo de contrato lê a série como texto e não abre conexão. O teste de sistema usa
`WYD_DB_URL_TEST`; se a variável falta, falha nomeando-a. O `#[ignore]` novo é reprovação, não
solução.

## 5. O que esta página não cobre

- **Lista de cenários por alvo:** é de `tests/alvos/` e ainda não existe.
- **Cobertura percentual:** a regra é por capacidade, não por linha.
- **Tempo esperado e paralelismo:** não medidos; a página de sistema declara o limite quando houver.
- **Dado de produção:** nunca entra em fixture; o catálogo de contas de teste depende de `M19`.
