> Pergunta: "o que reviso antes de abrir um PR do `wyd-db`?"
> Público: Desenvolvedor do `wyd-db` e revisor
> Dono: equipe do `wyd-db`
> Base: `wyd-db-1.0.0.md` §6.11; `padrao-banco.md` §10–§11; `plano-migrations.md` §7; `docs/convencoes.md`
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

O checklist é curto de propósito: cada item tem uma prova e um motivo de reprovação. Ele vale para
PR que toca código, migration ou página; PR só de planejamento usa o gate de documentação e os itens
de documentação deste checklist. **Nenhum comando de código roda hoje** — o crate é desenho e o CI de código é stub —, então
os itens marcados `P` são critério para quando o alvo existir, não verde atual.

## 1. Checklist

| # | Item | Como provar | Reprova quando |
|---:|---|---|---|
| 1 | escopo e issue | o PR diz qual issue/fase fecha e não mistura refatoração | muda arquivo fora do escopo ou não tem DoD |
| 2 | formatação | `cargo fmt --check` (`P`) | diff de formato no PR |
| 3 | lint | `cargo clippy --all-targets -- -D warnings` (`P`) | warning novo ou supressão sem justificativa |
| 4 | teste | `make test` (`P`): a `lib` e os dois contratos de API, sem banco | teste falha, `#[ignore]` novo ou `SKIP` silencioso |
| 5 | contrato e sistema | os alvos de [`testes-de-contrato`](../guia-teste/testes-de-contrato.md) e de sistema (`P`) | invariante sem prova ou ciclo `up/down/up` sujo |
| 6 | catálogo | regenerar o catálogo e conferir a contagem (`P`) | documento e série divergem |
| 7 | página de migration | os dez itens de `migrations-checklist.md` §5 | falta `down`, coluna, invariante ou prova de execução |
| 8 | documentação | página dona atualizada, estado e `Revisado em` com data, link conferido | número sem fonte, link quebrado ou página órfã |
| 9 | segredo | `git diff` sem DSN, token, hash, senha ou `.env` | valor de segredo em qualquer arquivo ou log |
| 10 | sem `unwrap` novo | `rg 'unwrap|expect'` no diff de produção (`P`) | `unwrap`/`expect` novo em caminho de produção sem justificativa |
| 11 | tipos proibidos | `rg` na migration por `serial`, `timestamp`, `money`, `varchar`, `enum` | qualquer tipo da lista de `padrao-banco.md` §11 |
| 12 | revisão | alguém que não escreveu leu o diff | PR sem segunda leitura |

## 2. Ordem de execução

1. **Formata e lint** antes de testar: erro de forma esconde erro de lógica.
2. **Teste unitário e contrato** antes do sistema: o contrato não precisa de banco.
3. **Sistema e fronteira** com PostgreSQL 17 efêmero; sem banco, a falha é declarada, não pulada.
4. **Catálogo e documentação** na mesma entrega da migration; número e link mudam juntos.
5. **Segredo e `unwrap`** por último, no diff completo — é onde o hábito falha.

## 3. O que o checklist não cobre

- **Cobertura de 85%:** é do gate do sistema (`AGENTS.md` §8), não deste checklist.
- **Desempenho e carga:** não há medição nem limiar decidido.
- **Deploy, release e operação:** são de `wyd-ops` e do
  [`plano-de-implantacao`](../planos/plano-de-implantacao.md).
- **Decisão de arquitetura:** mudança estrutural é proposta com ADR; o checklist não a aprova.

## 4. Exceção

Não existe exceção silenciosa. Item que não pode rodar é **falha declarada com o motivo**; abrir
exceção exige decisão do dono do `wyd-db`, com data e o que substitui o item até ele voltar. Sem esse
registro, o PR não passa.
