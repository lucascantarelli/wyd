> Pergunta: "que recurso compartilhado a suíte tem, o que é factory e o que é fixture, e como o dado é limpo?"
> Público: QA / testador
> Dono: equipe do `wyd-db`
> Base: `../docs/guia-teste/fixtures-e-factories.md` (regra); `../docs/guia-teste/testes-de-sistema.md` §"Isolamento"; `../wyd-db-1.0.0.md` §6.17
> Revisado em: 2026-11-24

**Estado:** aceito (2026-11-24) — inventário planejado; nenhum auxiliar implementado.

**Quatro níveis de afirmação convivem aqui:**

- **`P`** — tudo o que a página diz sobre `common/`: nenhum dos quatro arquivos foi escrito.
- **`H`** — dúvida aberta, dita e localizada em §5, com o dono que a fecha.
- **`V`** — o que foi medido nesta árvore em 2026-11-24, com o arquivo ao lado.
- **sem marca** — regra de projeto (`../docs/convencoes.md`) e comportamento do PostgreSQL
  (`../padrao-banco.md` §13).

Esta página é o **inventário planejado** de `tests/common/`; a regra de quando usar factory ou fixture é de
`../docs/guia-teste/fixtures-e-factories.md` e não se repete aqui. A diferença de dono é a mesma das
outras duplas: o guia diz **como**, esta página diz **o que existe**.

**Alvo escrito onde roda:** `sistema_pg`, `reversibilidade_pg`, `runner_pg`, `sistema_sessoes` e
`sistema_stack_caps` — os cinco que abrem conexão ([index.md](index.md) §2). Nenhum alvo de contrato usa
`common/`: os dois leem texto e não podem depender de banco (`specs/contrato.md` regra 1).

## 1. Inventário de `tests/common/`
**Sem arquivo no repositório:** `db.rs`, `factory.rs`, `serie.rs` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Arquivo | Papel | O que expõe | Precisa de banco |
|---|---|---|---|
| `mod.rs` | reexport | `db`, `factory` e `serie` em um caminho só | não |
| `db.rs` | ciclo do banco | `provisionar()`, `aplicar_serie()`, `conectar(papel)`, `descartar()` | sim |
| `factory.rs` | criação de dado | uma função por agregado da seção 3, cada uma devolvendo o id criado | sim |
| `serie.rs` | leitura da série | `ler(dir)` devolve arquivos e seções; `fixture(nome)` aponta para `resources/serie/` | não |

**Contrato de `db.rs`:** `provisionar()` roda o provisionamento real (`WYD_PROVISION_DSN`), não um
`CREATE SCHEMA` caseiro; `aplicar_serie()` roda o runner real (`WYD_DB_URL_MIGRATOR`); `descartar()`
derruba o banco inteiro. Teste que monta schema à mão prova um schema que não existe.

**Contrato de `serie.rs`:** lê o diretório **como texto**, com o mesmo parser que o runner usa
(`../src/migrate/spec.md` §2). Não é um segundo parser: se o contrato e o runner discordarem, um dos
dois está errado.

## 2. Factory × fixture, em uma linha cada

| Recurso | O que é | Quando usar |
|---|---|---|
| **factory** | função que cria a entidade com chave própria e devolve o id | todo teste de sistema que precisa de conta, personagem, sessão, grant ou resgate |
| **fixture** | dado estático lido de `resources/`, nunca escrito no banco pela suíte | teste de contrato, que lê a série de `resources/serie/` sem conexão |

Factory que devolve id é a regra que mata o id mágico: o teste usa o que a função devolveu, nunca
`1`, `42` ou `7662`.

## 3. As seis factories planejadas

| Função | Cria | Chave própria | Usada por |
|---|---|---|---|
| `factory::pessoa` | `portal.person` com e-mail único derivado da chave | `chave` do teste | `CAP-13`, `CAP-14` |
| `factory::conta` | `game.account` na vaga pedida (1..5), com papel de jogo | `chave` + vaga | `CAP-10`, `CAP-12`, `CAP-14` |
| `factory::personagem` | `game.character` ligado a uma conta | `chave` | `CAP-11`, `CAP-15` |
| `factory::sessao` | `game.session` viva ou encerrada, opcionalmente com personagem | `chave` | `CAP-11` |
| `factory::grant` | linha em `ops.delivery_queue` no estado pedido (`pending`, `applied`) | `chave` | `CAP-15` |
| `factory::resgate` | `portal.daily_reward_claim` para conta e dia | `chave` + dia | `CAP-16` |

**O que as factories não fazem:** não criam catálogo — o catálogo é da série (`CAP-07`, `CAP-08`); teste
que precisa de item usa o que o seed aplicado trouxe, ou cria a sua própria linha com chave.

## 4. Limpeza

| Regra | Consequência de violar |
|---|---|
| cada teste limpa **só a chave que criou** | `TRUNCATE` na base inteira esconde dependência e quebra o paralelismo |
| a limpeza roda mesmo quando o teste falha | linha órfã faz o teste seguinte passar por acidente |
| o banco inteiro morre no fim do binário | recriar é mais barato que limpar, e o próximo alvo começa limpo |
| ordem de limpeza respeita a FK (filho antes do pai) | `DELETE` que falha por FK vira verde falso se for ignorado |

## 5. Dúvidas abertas
**Sem arquivo no repositório:** `db.rs` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Dúvida | O que a resolve | Dono |
|---|---|---|
| como dois binários de sistema rodam em paralelo sem colidir no banco descartável? | `db.rs` precisa derivar um nome único por binário; não há regra nem medição | autor do alvo, no PR |
| a limpeza por chave sobrevive a falha no meio do teste? | a regra existe (§4); o mecanismo (guard que roda no drop) não está escrito | autor do alvo |

## 6. O que esta página não decide

- **Não define a política de fixture:** é de `../docs/guia-teste/fixtures-e-factories.md`.
- **Não define o dado de conta:** é de [dados-de-teste.md](dados-de-teste.md).
- **Não define o formato dos recursos:** é de [resources/README.md](resources/README.md).
- **Não decide o isolamento entre alvos:** é de `../docs/guia-teste/testes-de-sistema.md` §"Paralelismo",
  que hoje declara a intenção sem medição.
