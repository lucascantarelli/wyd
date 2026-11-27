> Pergunta: "o que são os arquivos de `tests/resources/`, quem os consome e que dado eles podem conter?"
> Público: QA / testador
> Dono: equipe do `wyd-db`
> Base: `../../docs/guia-teste/dados-de-teste.md`; `../../docs/guia-teste/testes-de-contrato.md` §1–§2; `../../wyd-db-1.0.0.md` §6.17
> Revisado em: 2026-11-27

**Estado:** aceito (2026-11-24), atualizado em 2026-11-27 — `serie/` existe e roda em
`contrato_migrations`; `accounts.json` continua planejado.

**Quatro níveis de afirmação convivem aqui:**

- **`P`** — tudo o que a página diz sobre `accounts.json`: o arquivo não existe.
- **`H`** — dúvida aberta, dita e localizada em §5, com o dono que a fecha.
- **`V`** — o que foi medido nesta árvore em 2026-11-24, com o arquivo ao lado.
- **sem marca** — regra de projeto (`../../docs/convencoes.md`) ou comportamento do PostgreSQL
  (`../../padrao-banco.md` §13).

`tests/resources/` guarda **dado de apoio**: o que o teste lê e não cria. É dado, não documentação — a
marca `■` da árvore de `../../wyd-db-1.0.0.md` §6.17 vale para os três itens abaixo.

**Alvo escrito onde roda:** `contrato_migrations` (as oito fixtures de `serie/`, verde desde 2026-11-27) e
`sistema_pg` + `sistema_sessoes` (`accounts.json`, lido antes de conectar). Nenhum recurso é lido por
`contrato_catalogo` hoje: ele depende do documento de catálogo, não de fixture.

## 1. O que existe
**Sem arquivo no repositório:** `accounts.json` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Caminho | O que é | Consumidor | Precisa de banco |
|---|---|---|---|
| `accounts.json` | ficha das contas de teste por papel | `sistema_pg`, `sistema_sessoes` | não (é lido antes de conectar) |
| `serie/valida/` | série mínima que satisfaz os sete invariantes do contrato (4 arquivos) | `contrato_migrations` | não |
| `serie/invalida/<defeito>/` | uma série por defeito, cada uma violando **um** invariante (7 pastas, 9 arquivos) | `contrato_migrations` | não |

## 2. `accounts.json`
**Sem arquivo no repositório:** `accounts.json` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

Array JSON, um objeto por conta, com os campos abaixo e nada além:

| Campo | Tipo | Regra |
|---|---|---|
| `conta` | texto | o nome da conta de teste; único no arquivo |
| `person_role` | texto ou `null` | `member`, `moderator`, `admin` ou `null` para papel de banco |
| `credencial` | texto | `WYD_DB_URL_PORTAL`, `_ADMIN`, `_READ` ou `_TEST` — o **nome** da variável, nunca o valor |
| `senha` | texto | fictícia, declarada como tal, e igual em todos os ambientes de teste |
| `prova` | texto | o que aquela conta demonstra, em uma linha |
| `caps` | lista | os `CAP-nn` atendidos, na forma `["CAP-20"]` |

**O que este arquivo nunca contém:** DSN, senha ou token de ambiente real; e-mail de pessoa real; hash
copiado de qualquer base. O campo `credencial` carrega o nome da variável porque o teste precisa saber
**qual** DSN usar, e o valor chega pelo ambiente.

O catálogo das contas — papéis, o que cada uma prova e como nasce — é de
[../dados-de-teste.md](../dados-de-teste.md); este arquivo é a materialização dele.

## 3. `serie/`

| Fixture | Arquivos | Invariante violado | `CAP` |
|---|---:|---|---|
| `valida/` | 4 | nenhum — é o caso que passa | `CAP-01`–`CAP-07` |
| `invalida/furo/` | 2 | falta o `0002` entre `0001` e `0003` | `CAP-01` |
| `invalida/down-incompleto/` | 1 | `fixture.extra` sobrevive ao `down` do próprio arquivo | `CAP-02` |
| `invalida/fk-futura/` | 2 | `REFERENCES` a `fixture.b`, criada no arquivo posterior | `CAP-03` |
| `invalida/seed-antes/` | 1 | `INSERT` antes do `CREATE TABLE` da tabela | `CAP-04` |
| `invalida/seed-sem-marca/` | 1 | `INSERT` em arquivo de nome sem `_seed_` | `CAP-05` |
| `invalida/constraint-anonima/` | 1 | `PRIMARY KEY` sem `CONSTRAINT nome` | `CAP-06` |
| `invalida/tipo-proibido/` | 1 | coluna `varchar` | `CAP-07` |

Cada fixture tem **um** defeito só: série com dois defeitos prova que o teste reprova, não qual
invariante ele confere. O teste exige **exatamente uma** mensagem de cada pasta `invalida/`.

O alvo aponta para a fixture por caminho constante dentro de `tests/resources/serie/` — **não** há
parâmetro `--dir` nem variável de ambiente, para o alvo ser determinístico
(`../alvos/contrato-migrations.md` §3). Os quatro arquivos da `valida/` são `0001_fixture_bootstrap`,
`0002_fixture_person`, `0003_fixture_wallet` (FK para a tabela do arquivo anterior) e
`0004_fixture_seed_item` (o único `INSERT`, em arquivo com `_seed_`).

## 4. A regra do dado fictício
**Sem arquivo no repositório:** `accounts.json` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

1. **Nada de valor real:** nome de pessoa, e-mail, telefone, senha ou token de qualquer ambiente.
2. **Nada de dado do cliente:** contagem, preço ou nome de item do 7662 é conteúdo de seed, não recurso
   de teste.
3. **Fictício e declarado:** o que é inventado diz que é; a senha de teste é a mesma em todo ambiente e
   será escrita em `accounts.json` como fictícia.
4. **Estável:** recurso versionado não muda de conteúdo entre execuções; mudança de fixture é mudança de
   teste e aparece no PR.

## 5. Dúvidas abertas
**Sem arquivo no repositório:** `accounts.json` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Dúvida | O que a resolve | Dono |
|---|---|---|
| ~~quantos arquivos a série `valida/` tem?~~ | fechada em 2026-11-27: **4** arquivos, o mínimo que cobre schema, PK/`CHECK`, FK para arquivo anterior e seed (§3) | autor do alvo |
| quem valida que todo `CAP-nn` citado em `accounts.json` existe na matriz? | `sistema_stack_caps` (§3 da página do alvo) confere a matriz; a checagem do JSON contra ela ainda não está escrita | autor do alvo |

## 6. O que esta página não decide

- **Não define o catálogo de contas:** é de [../dados-de-teste.md](../dados-de-teste.md).
- **Não define a regra de fixture × factory:** é de
  [../fixtures-e-factories.md](../fixtures-e-factories.md) e do guia.
- **Não decide o formato da série real:** a série é de `../../plano-migrations.md`; as fixtures imitam a
  forma dela, não a substituem.
- **Não cria arquivo:** nada aqui existe ainda, e criar depende do alvo estar implementado.
