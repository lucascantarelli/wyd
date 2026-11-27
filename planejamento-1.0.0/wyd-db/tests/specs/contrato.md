> Pergunta: "que regras valem para todo teste de contrato, sem exceção por autor?"
> Público: QA / testador
> Dono: equipe do `wyd-db`
> Base: `../../docs/guia-teste/testes-de-contrato.md`; `../specs/capacidades.md` §2–§3; `../../src/migrate/spec.md` §2
> Revisado em: 2026-11-24

**Estado:** aceito (2026-11-24) — regras escritas; nenhum dos **dois alvos desta página** implementado
(`contrato_migrations` e `contrato_catalogo`); os contratos de API `contrato_api_migrate` e
`contrato_api_provision` estão em `crates/wyd-db/Cargo.toml` desde 2026-11-26.

**Quatro níveis de afirmação convivem aqui:**

- **`P`** — tudo o que a página diz sobre a suíte: as 8 regras valem para dois alvos que ainda não existem
  em `crates/wyd-db/Cargo.toml`.
- **`H`** — dúvida aberta, dita e localizada em §4, com o dono que a fecha.
- **`V`** — o que foi medido nesta árvore em 2026-11-24, com o arquivo ao lado.
- **sem marca** — regra de projeto (`../../docs/convencoes.md`) e comportamento do PostgreSQL
  (`../../padrao-banco.md` §13).

Regras que valem para `contrato_migrations` e `contrato_catalogo`. A estratégia é do guia; esta página
fixa o que **a suíte** não pode violar, e cada regra diz como a violação aparece.

**Alvos escritos onde roda:** `contrato_migrations` (`CAP-01`–`CAP-07`) e `contrato_catalogo`
(`CAP-08`), os dois sem banco ([../index.md](../index.md) §2).

## 1. As regras
**Sem arquivo no repositório:** `serie.rs` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| # | Regra | Violação aparece como |
|---|---|---|
| 1 | **não abre conexão** — nem para "conferir rápido" | o alvo roda sem banco; conexão é sinal de que o teste é de sistema |
| 2 | **usa o mesmo parser do runner** (`serie.rs`), nunca um segundo | contrato e runner discordando sobre o mesmo arquivo |
| 3 | **lê a série como texto**, arquivo por arquivo, na ordem do número | ordem dependente do sistema de arquivos |
| 4 | **o caso que passa e o que falha rodam juntos**: série real + fixture com um defeito | fixture ruim passando = invariante não coberto |
| 5 | **um defeito por fixture** | falha que não diz qual invariante quebrou |
| 6 | **mensagem esperada é parte do teste**: o erro nomeia arquivo, linha e regra | asserção frouxa que aceita qualquer erro |
| 7 | **determinístico**: sem relógio, sem `env`, sem rede, sem aleatório | passa hoje e falha amanhã sem mudança de código |
| 8 | **não escreve no repositório** nem em `/tmp` compartilhado | efeito colateral entre alvos paralelos |

## 2. O que conta como "contrato"

| É contrato | Não é contrato |
|---|---|
| contiguidade, `down`, FK, ordem de seed, nome de constraint, tipo | `CHECK` que dispara, transação, índice em uso |
| paridade catálogo × série | privilégio de papel |
| forma do nome de arquivo e marca `_seed_` | mensagem de erro do PostgreSQL |

O critério: se dá para decidir **lendo texto**, é contrato; se precisa de servidor, é sistema.

## 3. Falha de contrato é falha de migration

Invariante quebrado corrige-se no arquivo da série, não no teste. As duas únicas exceções, e ambas
exigem justificativa no PR:

- o invariante está mais estrito que a série real e a série está certa — então muda a regra, com o
  contrato atualizado primeiro;
- a fixture deixou de representar o defeito — então a fixture se corrige e o cenário continua provando o
  mesmo invariante.

## 4. Dúvidas abertas

| Dúvida | O que a resolve | Dono |
|---|---|---|
| a mensagem de erro comparada pela regra 6 é texto, código ou os dois? | o formato não está fixado; a regra exige apenas que ela nomeie arquivo, linha e invariante | autor do alvo |
| como o alvo escolhe a fixture: `--dir`, variável de ambiente ou argumento? | `../resources/README.md` §3 cita `--dir` como exemplo; o nome do parâmetro não está decidido | autor do alvo |

## 5. O que esta página não decide

- **Não lista capacidades:** é `../specs/capacidades.md`.
- **Não lista cenários:** são `../alvos/contrato-*.md`.
- **Não define o parser:** é `../../src/migrate/spec.md` §2.
- **Não define o leitor do catálogo:** o `banco-parse` em uso não enxerga `CREATE TABLE esquema.tabela`;
  a limitação está declarada em `../alvos/contrato-catalogo.md` §3.
