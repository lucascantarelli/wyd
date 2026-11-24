> Pergunta: "como testo este crate, quais alvos existem e o que cada um prova?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `wyd-db-1.0.0.md` §6.17; `plano-de-testes.md`; `tests/` planejado
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

O guia de teste tem cinco alvos planejados e nenhum deles declarado no `Cargo.toml` que já existe. O comando único
da suíte é `P`: não existe um script aprovado que rode tudo. Até a suíte existir, o guia define o que
cada alvo deve provar e proíbe chamar de verde o que não rodou.

## 1. Os cinco alvos

| Alvo | Prova | Banco | Estado |
|---|---|---|---|
| `contrato_catalogo` | documento de catálogo e série descrevem as mesmas tabelas | não | planejado |
| `contrato_migrations` | forma da série: contiguidade, `down`, FK, seed, constraint e tipo | não | planejado |
| `sistema_pg` | aplicar, reaplicar, drift, ciclo de `down`, lock e concorrência | sim | planejado |
| `sistema_sessoes` | privilégio por papel e leitura cruzada por view | sim | planejado |
| `sistema_stack_caps` | matriz de capacidades `CAP-nn` | sim | planejado; matriz escrita (**39** capacidades, **0** cobertas) |

O detalhe de cada alvo está em `tests/alvos/` e a matriz em `tests/specs/capacidades.md` — fora do site,
porque `tests/` não é publicado. A página
[`piramide-e-alvos.md`](piramide-e-alvos.md) diz como escolher o nível; a página
[`gates-e-anti-skip.md`](gates-e-anti-skip.md) diz como impedir teste que não roda.

## 2. Mapa dos guias

| Guia | Pergunta | Estado |
|---|---|---|
| [`piramide-e-alvos.md`](piramide-e-alvos.md) | que tipo de teste escrevo? | recorte escrito |
| [`fixtures-e-factories.md`](fixtures-e-factories.md) | como crio dado? | escrito (2026-11-24) |
| [`dados-de-teste.md`](dados-de-teste.md) | que dado existe? | escrito (2026-11-24) |
| [`testes-de-contrato.md`](testes-de-contrato.md) | como provo a forma sem banco? | escrito |
| [`testes-de-sistema.md`](testes-de-sistema.md) | como provo contra PG real? | escrito |
| [`gates-e-anti-skip.md`](gates-e-anti-skip.md) | como impeço `SKIP`? | recorte escrito |

## 3. Comando único

O comando único da suíte ainda **não existe**. O desenho é `cargo test -p wyd-db` para os alvos sem
banco e `WYD_DB_URL_TEST=... cargo test -p wyd-db --test sistema_pg` para o alvo real. Nenhuma linha
deve ser copiada como “já funciona”: o crate e os alvos são `P`.

## 4. Regras que valem para todos os alvos

- Nenhum teste sem capacidade associada; nenhuma capacidade sem teste.
- `SKIP`, `#[ignore]` novo ou filtro silencioso reprovam o aceite.
- Teste de contrato não abre conexão; teste de sistema não usa falso para provar o banco.
- Dado de teste é fictício, isolado por chave e limpo pelo próprio teste.
- Falha nomeia arquivo, versão ou variável — não uma mensagem genérica.

## 5. O que esta página não cobre

- **Cenários detalhados:** serão de `tests/alvos/`; não há arquivo ainda.
- **Cobertura percentual e tempo:** não medidos; a regra é por capacidade.
- **CI real:** o workflow ainda é stub; o gate de docs não substitui a suíte.
