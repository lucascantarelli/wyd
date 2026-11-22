> Pergunta: "o que já foi entregue no `wyd-db`?"
> Público: equipe do `wyd-db`, operação e quem assume o projeto
> Dono: equipe do `wyd-db`
> Base: `wyd-db-1.0.0.md` §6.9; `plano-migrations.md` §9; `docs/planos/plano-de-implantacao.md`
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

Este é o **registro de execução** do `wyd-db`: uma linha por entrega, sem narrativa. Ele responde
“o que já foi entregue?” com data, item, comando ou arquivo e resultado, para que a próxima pessoa
não precise reconstruir a história a partir de commit. Não é diário, não é métrica e não substitui o
plano de implantação.

## 1. Regra

| Regra | Efeito |
|---|---|
| uma linha por entrega | duas entregas no mesmo dia são duas linhas |
| sem narrativa | a linha diz **o que** rodou e **qual** foi o resultado; o porquê fica na issue ou na decisão |
| só entrega verificável | comando, arquivo aplicado ou artefato que outra pessoa consegue repetir |
| append-only | linha registrada não é reescrita; correção entra como linha nova com a data |
| data absoluta | `AAAA-MM-DD`, nunca “ontem” ou “semana passada” |

## 2. Formato

| Data | Item | Comando ou arquivo | Resultado | Prova |
|---|---|---|---|---|
| — | — | — | — | — |

- **Item:** número da etapa de `plano-migrations.md` §9 ou nome curto da entrega.
- **Comando ou arquivo:** o comando exato ou o caminho do arquivo aplicado.
- **Resultado:** `ok`, `falhou` ou o número medido (ex.: `109 tabelas`, `0 ao fim do down`).
- **Prova:** link para a issue, o PR ou o artefato que mostra o resultado.

## 3. Entregas registradas

**Nenhuma entrega do `wyd-db` até 2026-11-22.** A série da `1.0.0` é planejada, não aplicada; nenhum
arquivo `.sql` novo foi escrito e o crate ainda é desenho (`plano-migrations.md`, cabeçalho e §9).
Por isso a tabela de §2 está vazia: registrar uma linha aqui sem comando executado seria inventar
entrega.

## 4. O que entra e o que não entra

| Entra | Não entra |
|---|---|
| `provision render` / `provision apply` executados | decisão de lacuna — é de `lacunas.md` |
| aplicação de `0001`…`0034` e o resultado do ledger | criação de issue ou mudança de roadmap |
| corte vertical de `0001`–`0007` no PG17 efêmero | onda de documentação — é de `proposta-paginas.md` |
| catálogo regenerado e gates de `plano-de-qualidade-e-gates.md` | risco ou defeito de página — é de `riscos.md` |
| troca da série e o `SCHEMA_VERSION` resultante | medição de cobertura funcional — é de `metricas.md` |
| incidente operacional com comando e resultado | opinião, estimativa ou “quase pronto” |

## 5. Como verificar

Cada linha tem de ser reproduzível: o comando pode rodar de novo, o arquivo pode ser conferido no
ledger e o número pode ser recontado. Linha sem prova não é entrega; é intenção. Quando o resultado
for uma falha, a linha registra a falha e a linha seguinte registra a correção — não se apaga a
primeira.

## 6. O que esta página não decide

- **Ordem do trabalho:** é de `plano-migrations.md` §9 e de `proposta-paginas.md` §8.
- **Aceite:** é de [`plano-de-qualidade-e-gates.md`](plano-de-qualidade-e-gates.md).
- **Métrica e percentual:** são de `metricas.md`; esta página conta entrega, não progresso.
- **Risco:** é de [`riscos.md`](riscos.md).
