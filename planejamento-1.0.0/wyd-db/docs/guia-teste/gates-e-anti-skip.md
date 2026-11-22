> Pergunta: "como impeço um teste de passar sem rodar e como sei que a capacidade foi coberta?"
> Público: Desenvolvedor, QA e revisor de PR
> Dono: equipe do `wyd-db`
> Base: `wyd-db-1.0.0.md` §5.9 e §6.13; `plano-de-testes.md`; `plano-de-qualidade-e-gates.md`
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

A regra é curta: **`SKIP` não é resultado**. Teste que não roda por variável ausente deve falhar
nomeando a variável; teste que não roda por falta de banco não entra no aceite. O gate de capacidade é
por `CAP-nn`, não por percentual de linha. A matriz de capacidades ainda não existe; esta página
declara a lacuna e não inventa cobertura.

## 1. O que reprova

| Sinal | Por que reprova | Correção |
|---|---|---|
| `#[ignore]` novo no caminho de aceite | o teste existe, mas não roda | remover o ignore ou justificar em issue separada, fora do aceite |
| `SKIP`/`skip!` por env ausente | ausência de ambiente vira verde falso | falhar nomeando a variável (`WYD_DB_URL_TEST`) |
| filtro que exclui o alvo | a suíte parece verde sem o cenário | rodar o alvo completo no aceite |
| teste que só verifica `is_ok()` | não prova efeito nem erro esperado | verificar estado, código e mensagem |
| golden gerado na hora | compara o código com ele mesmo | golden versionado e revisado |
| teste que lê a mesma fonte que produz | tautologia | usar fixture independente ou valor esperado explícito |

`SKIP` silencioso inclui `if env::var(...).is_err() { return; }` e variantes que devolvem sucesso. O
teste deve falhar com mensagem que diga qual variável falta e qual alvo precisa dela.

## 2. Como o gate detecta

O gate automático ainda não foi implementado. O desenho mínimo é:

1. procurar `#[ignore]`, `SKIP`, `skip!` e retorno silencioso por ambiente nos alvos de aceite;
2. rodar os alvos de contrato sem banco e falhar se algum for pulado;
3. rodar os alvos de sistema com `WYD_DB_URL_TEST` e exigir que a variável exista;
4. comparar a lista de alvos do `Cargo.toml` com a lista do plano de testes;
5. falhar se uma capacidade `CAP-nn` não tiver alvo/cenário.

Nenhum passo acima tem comando publicado hoje; o crate e os alvos são `P`. O gate de docs não conta
como gate de teste.

## 3. Cobertura por capacidade

| Unidade | O que significa | Como entra no aceite |
|---|---|---|
| `CAP-nn` | uma capacidade observável do banco | linha na matriz, ligada a requisito e alvo |
| alvo | um `Cargo.toml` de teste | precisa cobrir ao menos uma capacidade |
| cenário | um caso nomeado dentro do alvo | prova a capacidade com entrada e resultado |
| requisito | `RF-DB`/`RN-DB`/`RNF-DB` | aponta para a capacidade, não para “teste genérico” |

A matriz `CAP-nn → alvo → cenário → requisito` **não existe**. Sem ela, não há percentual de cobertura
a citar e nenhuma capacidade pode ser declarada coberta por intuição.

## 4. Rotina de aceite

1. Rodar o alvo de contrato e conferir que ele não pulou nenhum arquivo.
2. Rodar o alvo de sistema com banco descartável e DSN nomeada.
3. Conferir que o teste novo falha quando o defeito é reintroduzido (mutação manual mínima).
4. Verificar que não há `#[ignore]`/`SKIP` novo no diff.
5. Registrar a capacidade coberta e a evidência na issue.
6. Se a matriz não existir, declarar a lacuna; não marcar a capacidade como coberta.

## 5. O que esta página não cobre

- **Cobertura percentual e meta numérica:** a regra é por capacidade, não por linha.
- **CI real:** o workflow é stub; a rotina acima é manual até o pipeline existir.
- **Matriz `CAP-nn`:** é lacuna; a página não a substitui por uma tabela inventada.
- **Teste de carga e caos:** não há volume nem ambiente medido.
