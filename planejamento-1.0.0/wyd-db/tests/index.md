> Pergunta: "como a suíte está montada, que alvo prova o quê e como eu rodo?"
> Público: QA / testador
> Dono: equipe do `wyd-db`
> Base: `../wyd-db-1.0.0.md` §6.17; `../docs/planos/plano-de-testes.md` §2–§4; `../docs/guia-teste/piramide-e-alvos.md`
> Revisado em: 2026-11-24

**Estado:** aceito (2026-11-24) — página escrita; 1 dos 5 alvos implementado e 13 dos 39 cenários executados (2026-11-24).

**Quatro níveis de afirmação convivem aqui:**

- **`P`** — tudo o que a página diz sobre a suíte: só `sistema_pg` existe em `crates/wyd-db/Cargo.toml`;
  os outros quatro alvos, `common/` e `resources/serie/` não têm arquivo escrito.
- **`H`** — dúvida aberta, dita e localizada em §5, com o dono que a fecha.
- **`V`** — o que foi medido nesta árvore em 2026-11-24, com o § ao lado.
- **sem marca** — regra de projeto (`../docs/convencoes.md`) e comportamento do PostgreSQL
  (`../padrao-banco.md` §13).

Esta pasta é o **plano da suíte** do `crates/wyd-db`: cinco alvos, o recurso compartilhado de
`tests/common/`, o dado de apoio de `tests/resources/` e as regras que valem para toda a suíte, em `specs/`.
A matriz de capacidades é [specs/capacidades.md](specs/capacidades.md) e é ela, não esta página, que diz
o que cada alvo prova.

## 1. Árvore planejada de `crates/wyd-db/tests/`

```text
crates/wyd-db/tests/
├── contrato_catalogo.rs      # alvo de contrato: catálogo × série
├── contrato_migrations.rs    # alvo de contrato: forma da série
├── sistema_pg.rs             # alvo de sistema: comportamento no PG17
├── sistema_sessoes.rs        # alvo de sistema: papel, fronteira e view
├── sistema_stack_caps.rs     # meta-alvo: a matriz e o anti-skip
├── common/
│   ├── mod.rs                # reexport dos três auxiliares
│   ├── db.rs                 # provisiona, conecta, derruba
│   ├── factory.rs            # cria dado com chave própria
│   └── serie.rs              # lê a série como texto
└── resources/
    ├── accounts.json         # contas de teste por papel
    └── serie/
        ├── valida/           # série de fixture que passa
        └── invalida/         # uma fixture por defeito do contrato
```

`common/` é o único lugar com helper compartilhado; arquivo dentro de `tests/` que não é alvo nem
recurso vive em `common/` e aparece no inventário de [fixtures-e-factories.md](fixtures-e-factories.md).

## 2. Alvo → arquivo → tipo → precisa de banco

| Alvo (`Cargo.toml`) | Arquivo | Tipo | Precisa de banco | Capacidades |
|---|---|---|---|---|
| `contrato_migrations` | `contrato_migrations.rs` | contrato | **não** | `CAP-01`–`CAP-07` |
| `contrato_catalogo` | `contrato_catalogo.rs` | contrato | **não** | `CAP-08` |
| `sistema_pg` | `sistema_pg.rs` | sistema | **sim** | `CAP-09`–`CAP-19` e `CAP-24`–`CAP-39` |
| `sistema_sessoes` | `sistema_sessoes.rs` | sistema | **sim** | `CAP-20`–`CAP-21` |
| `sistema_stack_caps` | `sistema_stack_caps.rs` | meta | **sim** | `CAP-22`–`CAP-23` |

Um alvo por arquivo, como manda `../wyd-db-1.0.0.md` §5.13. Tipo e fronteira de cada camada estão em
`../docs/guia-teste/piramide-e-alvos.md`; o que cada um **não** prova está na seção 4 de cada página de
`alvos/`.

### 2.1 Porta de entrada — as 12 páginas

O critério de pronto exige que toda página esteja linkada no índice da pasta (`../wyd-db-1.0.0.md` §10,
item 10). As onze abaixo são as demais páginas de `tests/`; esta porta fecha a décima segunda — são
**12** páginas, medido em 2026-11-24. Cada linha
diz **a pergunta que a página responde**, **quando abri-la** e **para onde ir depois** — não é uma lista
de arquivos, é o caminho de leitura.

| Página | Pergunta que responde | Quando abrir | Depois vá para |
|---|---|---|---|
| [specs/capacidades.md](specs/capacidades.md) | que capacidade existe, qual alvo a prova e onde ela é provada | **primeiro**, quando quiser saber o que a suíte promete | a página do alvo que a linha nomeia |
| [alvos/contrato-migrations.md](alvos/contrato-migrations.md) | o que a **forma** da série prova (`CAP-01`–`CAP-07`) | ao mexer em migration ou revisar contrato | [specs/contrato.md](specs/contrato.md), [resources/README.md](resources/README.md) |
| [alvos/contrato-catalogo.md](alvos/contrato-catalogo.md) | a paridade catálogo × série (`CAP-08`) | ao tocar o dicionário; hoje está bloqueada | [specs/contrato.md](specs/contrato.md) |
| [alvos/sistema-pg.md](alvos/sistema-pg.md) | o comportamento contra PG17 (`CAP-09`–`CAP-19` e `CAP-24`–`CAP-39`) | ao escrever teste que abre conexão | [specs/sistema.md](specs/sistema.md), [fixtures-e-factories.md](fixtures-e-factories.md) |
| [alvos/sistema-sessoes.md](alvos/sistema-sessoes.md) | papel, fronteira e leitura por view (`CAP-20`–`CAP-21`) | ao mexer em `GRANT`, view ou auditoria | [specs/sistema.md](specs/sistema.md), [dados-de-teste.md](dados-de-teste.md) |
| [alvos/sistema-stack-caps.md](alvos/sistema-stack-caps.md) | a suíte sobre a suíte: DSN, anti-skip e matriz ligada (`CAP-22`–`CAP-23`) | ao mexer em ambiente de teste ou na matriz | [specs/capacidades.md](specs/capacidades.md) |
| [fixtures-e-factories.md](fixtures-e-factories.md) | o que existe em `common/` e como o dado é limpo | **antes** do primeiro teste de sistema | [dados-de-teste.md](dados-de-teste.md), [resources/README.md](resources/README.md) |
| [dados-de-teste.md](dados-de-teste.md) | que contas existem, o que cada uma prova e o que nunca entra | ao precisar de conta, personagem ou sessão | [resources/README.md](resources/README.md) |
| [resources/README.md](resources/README.md) | formato de `accounts.json` e das fixtures de série | ao criar fixture ou ficha de conta | [alvos/contrato-migrations.md](alvos/contrato-migrations.md) |
| [specs/contrato.md](specs/contrato.md) | as 8 regras da camada **sem** banco | ao escrever teste de contrato | [alvos/contrato-migrations.md](alvos/contrato-migrations.md) |
| [specs/sistema.md](specs/sistema.md) | as 18 regras da camada **com** banco | ao escrever teste de sistema | [alvos/sistema-pg.md](alvos/sistema-pg.md) |

**Ordem de leitura, para quem chega agora:** [specs/capacidades.md](specs/capacidades.md) → a página do
alvo da capacidade que interessa → a regra da camada ([specs/contrato.md](specs/contrato.md) ou
[specs/sistema.md](specs/sistema.md)) → o recurso de que o cenário depende
([fixtures-e-factories.md](fixtures-e-factories.md), [dados-de-teste.md](dados-de-teste.md),
[resources/README.md](resources/README.md)).

## 3. Como rodar

| O que | Comando | Observação |
|---|---|---|
| tudo do crate | `cargo test -p wyd-db` | os dois contratos rodam sem banco |
| um alvo | `cargo test -p wyd-db --test <alvo>` | `<alvo>` é o nome da coluna 1 |
| um cenário | `cargo test -p wyd-db --test <alvo> <cenário>` | o nome do cenário é o da matriz |
| só o contrato | `cargo test -p wyd-db --test contrato_migrations --test contrato_catalogo` | não abre conexão |

**Ambiente:** os três alvos que exigem banco leem **`WYD_DB_URL_TEST`** (DSN do `wyd_migrator` em banco
descartável) e falham **nomeando a variável** quando ela falta — nunca pulam
(`../docs/guia-teste/testes-de-sistema.md` §"DSN de teste"; `CAP-22`). Nenhum binário lê `DATABASE_URL`.

**Pré-requisito dos alvos de sistema:** banco vazio → provisionamento → série. Banco "já quente" não é
ambiente de teste: o banco nasce e morre com a execução.

## 4. O que a suíte não faz

- **Não mede cobertura de linha:** a regra é por capacidade (`CAP-nn`), não por percentual.
- **Não testa interface, E2E, carga ou regra de jogo:** os dois primeiros são dos sistemas consumidores;
  os dois últimos não têm volume nem limiar aprovado (`../docs/planos/plano-de-testes.md` §6).
- **Não substitui o contrato de catálogo do repositório:** o leitor em uso ainda não enxerga
  `CREATE TABLE esquema.tabela` (`../docs/guia-teste/testes-de-contrato.md` §1).
- **Não declara verde:** só o que rodou conta; a matriz registra 39 capacidades e **13** cobertas por
  teste executado (V, 2026-11-24), todas em `sistema_pg`.

## 5. Dúvidas abertas

Duas perguntas do dono mudam o que esta pasta autoriza, e nenhuma está respondida:

| Dúvida | O que a resolve | Dono |
|---|---|---|
| o aceite das 12 páginas libera o código dos cinco alvos, ou o código espera a issue no GitHub? | `Q-BLK-18` — recomendação **A**: o aceite libera e a issue registra o vínculo no lote da onda (`../pendencias-de-fechamento.md` §6.6.22.5) | dono do `wyd-db` |
| a leva de aceite começa por `tests/` ou por `migrations/`? | `Q-BLK-17` — **fechada em A pela execução** (2026-11-24): a leva 1 começou por `tests/`, por razão itens/ato de 7,3 contra 1,1 | dono do `wyd-db` |

Enquanto as duas estiverem abertas, esta pasta é **plano escrito**, não autorização de código: os cinco
alvos continuam fora do `Cargo.toml`.

## 6. O que esta página não decide

- **Não define capacidade:** isso é de [specs/capacidades.md](specs/capacidades.md), dono único da matriz.
- **Não define cenário:** cada página de `alvos/` descreve os seus, com pré-requisito e leitura de falha.
- **Não decide a estratégia de teste:** ela é de `../docs/planos/plano-de-testes.md` e
  `../docs/guia-teste/`.
- **Não cria comando novo de CI:** não existe página de CI em `docs/planos/` do `wyd-db`; o plano do
  projeto é `../../wyd-ops/plano-ci-real.md` (`proposta`, 2026-11-19), e
  `docs/planos/plano-de-testes.md` §81 anuncia a página local para depois do primeiro código. Aqui só o
  comando local por alvo.
