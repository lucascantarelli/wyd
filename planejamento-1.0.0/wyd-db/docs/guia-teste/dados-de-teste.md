> Pergunta: "que contas e que dado existem para testar, e o que nunca pode ser usado?"
> Público: Desenvolvedor, QA e operação
> Dono: equipe do `wyd-db` com QA
> Base: `AGENTS.md` §8; `lacunas.md` `M19`; `pendencias-de-fechamento.md` §6.6.5 (`Q-BLK-05` = A, 2026-11-24); `docs/planos/plano-de-dados-e-seeds.md`
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-24)

Cinco contas cobrem os cinco papéis que o teste precisa provar. O seed de desenvolvimento ainda **não
existe** (code `seeds/dev.sql` é planejado) e a senha única é a decidida em `M19` — esta página não
reproduz o valor.

## 1. Catálogo de contas

Duas coisas diferentes: a **conta** é linha em `portal.person` com um `role`; a **credencial** é a DSN do
processo que age em nome dela. O teste de fronteira usa a DSN do papel testado, não a do migrator.

| Conta | Papel de produto | Credencial usada | O que prova | O que não pode |
|---|---|---|---|---|
| jogador | `role = member` | `WYD_DB_URL_GAME` (`wyd_game`) | login, sessão, personagem, inventário | ler `portal.person` cru; escrever em `portal` |
| staff | `role = moderator` | `WYD_DB_URL_ADMIN` (`wyd_admin`) | moderação, tabelas do C4, `ops.audit_log` | `DELETE`; DDL |
| administrador | `role = admin` | `WYD_DB_URL_ADMIN` | ação administrativa e a trilha dela | `DELETE`; DDL |
| operador | não é conta de jogo | `WYD_PROVISION_DSN` e a credencial administrativa | migration, expurgo, vacuum, backup | entrar como jogador |
| bot | `role = member` | `WYD_DB_URL_GAME` | carga e convivência (`wyd-bots`) | rodar contra produção |

Os valores de `role` são os do DDL (`member`, `moderator`, `admin` —
[`dicionario-dados/person`](../dicionario-dados/gerado/person.md)); staff e administrador são papéis
distintos, e juntá-los faria o teste atribuir ao staff uma ação que só `admin` faz.

## 2. Seed de desenvolvimento

- Um arquivo declarativo, code `seeds/dev.sql`, aplicado em banco **recriado** — não em banco com estado.
- Cria as cinco contas, **um personagem para cada conta de jogo** e a versão de conteúdo mínima.
- A senha única de desenvolvimento é a de `lacunas.md` `M19`; o valor mora no seed ou no `.env`, nunca
  nesta página, em log ou em fixture versionada (`AGENTS.md` §5, item 7).
- Não usa valor do cliente 7662: as 6 seeds da série são dado de origem e seguem bloqueadas
  (`pendencias-de-fechamento.md` §4.2).

## 3. Como provisionar

| Passo | O que fazer | Prova de que deu certo |
|---|---|---|
| 1 | criar banco descartável | o banco existe e está vazio |
| 2 | aplicar a série | ledger com uma entrada por arquivo da série |
| 3 | aplicar `seeds/dev.sql` | 5 contas e 1 personagem por conta de jogo |
| 4 | conferir a contagem declarada no próprio seed | contagem igual à do arquivo |
| 5 | rodar a suíte | alvos verdes; hoje os alvos são `P` |

A DSN de provisionamento é `WYD_DB_TEST_URL` (`wyd_migrator`, banco descartável —
[`testes-de-sistema.md`](testes-de-sistema.md), seção “DSN de teste”). Recriar o banco entre execuções é o padrão, não a
exceção.

## 4. O que nunca usar

- **Dado do cliente 7662**, em fixture, seed ou snapshot: é dado de origem, não do projeto novo.
- **Conta real de jogador** e qualquer export de produção como fixture (`M32`).
- **`seeds/dev.sql` em produção** e a senha de desenvolvimento fora de desenvolvimento.
- **Bot contra produção**: o bot é de carga e convivência, e o ambiente dele é o de teste (`wyd-bots`
  `LBOT-01`).
- **DSN de runtime para limpar dado**: os papéis não têm `DELETE`; insistir vira `42501`, não solução.

## 5. O que esta página não decide

- **Os nomes das contas:** ficam no `seeds/dev.sql` quando ele existir; esta página não inventa login.
- **O formato do seed e a ferramenta que o aplica:** decisão da onda de código.
- **Cobertura, gate e regra de `SKIP`:** é [`gates-e-anti-skip.md`](gates-e-anti-skip.md).
- **Quantas contas o `wyd-bots` cria por cenário:** é do planejamento do `wyd-bots` (`Q-BOT-2`).
