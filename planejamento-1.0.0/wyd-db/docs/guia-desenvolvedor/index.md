> Pergunta: "por onde começo como desenvolvedor do `wyd-db` e o que já posso executar?"
> Público: Desenvolvedor novo, revisor e autor de migration
> Dono: equipe do `wyd-db`
> Base: `wyd-db-1.0.0.md` §6.11; `src/migrate/spec.md`; `src/provision/spec.md`; `plano-migrations.md` §9
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

O guia tem uma trilha curta: entender a fronteira, preparar o ambiente, achar o arquivo, usar a porta
certa e provar com teste. Ele é para quem mexe no crate; instalação do banco e operação ficam no
[guia do administrador](../guia-administrador/index.md).

## 1. Trilha de leitura

1. [`../arquitetura/visao-geral.md`](../arquitetura/visao-geral.md) — o que o banco guarda e o que não guarda.
2. [`../arquitetura/componentes-c4.md`](../arquitetura/componentes-c4.md) — as peças de `migrate` e `provision`.
3. [`setup-ambiente.md`](setup-ambiente.md) — o que existe e o que ainda falta para rodar.
4. [`mapa-do-crate.md`](mapa-do-crate.md) — onde cada arquivo mora e a regra de dependência.
5. [`portas-e-contratos.md`](portas-e-contratos.md) — qual porta usar e o que ela garante.
6. [`modelos-e-invariantes.md`](modelos-e-invariantes.md) — o que os tipos recusam.
7. [`como-escrever-teste.md`](como-escrever-teste.md) — como provar sem inventar fixture.
8. [`erros-e-logs.md`](erros-e-logs.md) — como o erro aparece e o que nunca vai para o log.

## 2. Pré-requisitos

| Pré-requisito | Estado |
|---|---|
| PostgreSQL 17 para o alvo de sistema | planejado; a versão é a de `padrao-banco.md` §1 |
| Rust e Cargo | planejado; versão mínima não foi decidida em documento |
| DSNs por papel | nomes em `src/provision/spec.md` §3; valores são do ambiente |
| crate `crates/wyd-db` | **não existe**; a árvore é a de `wyd-db-1.0.0.md` §6.16 |
| suíte de testes | planejada; os alvos ainda não têm `Cargo.toml` |

## 3. Os três comandos que provam o ambiente

**Este recorte não tem os três comandos.** Eles dependem do crate e de um `Cargo.toml` que ainda não
existem; escrever comandos que não rodam seria pior que declarar a falta. Quando a fase 1 do roadmap
fechar, o guia ganha três comandos executáveis e a saída esperada de cada um. Até lá, o critério é:

- o crate compila;
- o alvo de contrato roda sem PostgreSQL;
- o alvo de sistema roda com `WYD_DB_URL_TEST` e um banco descartável.

Nenhum dos três é "verde" hoje; a evidência é a issue `WYDDB-001` e as seguintes.

## 4. Mapa dos guias

| Guia | Pergunta | Estado |
|---|---|---|
| [`setup-ambiente.md`](setup-ambiente.md) | como preparo a máquina? | recorte: variáveis e banco; versões e comandos faltam |
| [`mapa-do-crate.md`](mapa-do-crate.md) | o que é cada pasta? | recorte: árvore planejada |
| [`portas-e-contratos.md`](portas-e-contratos.md) | que porta eu uso? | recorte: sete portas desenhadas |
| [`modelos-e-invariantes.md`](modelos-e-invariantes.md) | o que os tipos garantem? | recorte: tipos de identidade e runner |
| [`como-escrever-teste.md`](como-escrever-teste.md) | que teste eu escrevo? | recorte: alvo e regra; fixture depende da suíte |
| [`erros-e-logs.md`](erros-e-logs.md) | como o erro aparece? | recorte: `migrate` e `provision` |
| [`como-adicionar-migration.md`](como-adicionar-migration.md) | como crio a próxima migration? | escrito |
| [`como-adicionar-tabela.md`](como-adicionar-tabela.md) | como modelo uma tabela nova? | escrito |
| [`checklist-de-pr.md`](checklist-de-pr.md) | o que revisar antes do PR? | escrito |

## 5. O que esta página não cobre

- **Instalação de serviço, volume e porta:** é do guia do administrador e do plano de implantação.
- **Regra de negócio, coluna e constraint:** são de `specs/` e do dicionário.
- **CI real:** será aplicado depois do primeiro código; o guia não promete pipeline.
