> Pergunta: "o que assumimos e o que nos limita antes de escrever o resto do `wyd-db`?"
> Público: Produto / gestão e arquitetura
> Dono: equipe do `wyd-db`
> Base: `lacunas.md` §0.2, §1 e §2; `rascunho-accounts.md` §7; `src/provision/spec.md` §2–§3; `AGENTS.md` §5 e §9
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-22)

Esta é a página que **recebe o que está em aberto**. Ela não decide lacuna nem substitui `lacunas.md`
— apenas reúne, em um lugar legível, as restrições que valem para qualquer página do `wyd-db` e as
premissas que ainda não viraram fato. Quando uma premissa cair, a linha muda aqui e a origem continua
sendo o registro de lacuna.

## 1. Restrições técnicas

| Restrição | Efeito no que se pode escrever | Fonte |
|---|---|---|
| PostgreSQL 17, quatro schemas (`content`, `game`, `portal`, `ops`) | nenhuma página promete outro SGBD, schema `public` de domínio ou tabela sem schema | `padrao-banco.md` §1 (`P-01`, `P-02`) |
| seis papéis de cluster, criados **fora** da série | página de migration não cria papel nem senha; o `GRANT` de objeto é da série, o papel é do provisionamento | `ADR-DB-0001`; `src/provision/spec.md` §2 |
| a série da `1.0.0` ainda **não foi aplicada** | todo DDL é `P`; a contagem que vale é a planejada (34 arquivos, 109 tabelas, `SCHEMA_VERSION = 34` na troca), e o número medido só entra depois da medição | `plano-migrations.md` §2–§3; `lacunas.md` `M20` (fechada) |
| enquanto a série herdada roda, `SCHEMA_VERSION` continua `53` | nenhuma página afirma que a troca já aconteceu | `plano-migrations.md` §6 |
| catálogo de banco não existe | cobertura e contagem de coluna não são medidas por catálogo; o que há é o DDL confirmado | `wyd-db-1.0.0.md` §1.2; `lacunas.md` `M20` |
| `planejamento-1.0.0/` é documentação | nenhum `.sql`, `.py`, `.rs` ou artefato executável nasce nesta árvore; script de conferência mora em `wyd-docs/` ou `tools/` | `AGENTS.md` §5.6 |
| build do site só em Linux/CI | página não promete comando de build no Windows | `AGENTS.md` §9 |
| CI de código é stub | nenhuma página trata "verde de CI" como prova de build ou teste de código | `AGENTS.md` §4; `docs/planos/riscos.md` `R-3` |
| seed sem gerador, escrito à mão quando o dado de origem for conferido | nenhuma página promete `make` ou gerador de seed | `plano-migrations.md` §4 |

## 2. Restrições legais e de dado pessoal

| Restrição | Efeito | Fonte |
|---|---|---|
| PII de identidade mora só em `portal` | e-mail, hash de senha e token ficam em `portal.person`/`person_session`/`person_token`; `game` referencia a pessoa por FK e não copia e-mail | `padrao-banco.md` `P-22`; `ddl-confirmado.md` §5.1–§5.2 |
| fechar a pessoa **anonimiza** o e-mail | a linha fechada guarda `closed-<id>@removido.invalid` e o índice de e-mail é total, sem janela de reuso | `lacunas.md` `A12` (fechada); `ddl-confirmado.md` §5.1 |
| tentativa de login não guarda identificador em claro | `ops.login_attempt.identifier_hash` é `sha256`; `ops` é lido por `wyd_read` | `lacunas.md` `M6` (fechada); `ddl-confirmado.md` §5.4 |
| consentimento é versionado | guarda-se qual texto foi aceito e quando; revogar preenche `revoked_at`, sem apagar histórico | `lacunas.md` `M7` (fechada); `ddl-confirmado.md` §5.7 |
| retenção por tabela foi decidida | `person_session` 30 dias após `expires_at`; `game.session` 365 dias após `released_at`; `ops.audit_log` 730 dias, append-only | `lacunas.md` `M22` (fechada) |
| senha e PIN só como hash resistente | `argon2id`; a senha do portal tem 15–128 e o PIN 4–6, com 5 falhas/10 min | `lacunas.md` `M9`; `B3` (fechadas) |
| finalidade declarada por grupo de campo, sem base legal afirmada | nenhuma tela coleta campo sem finalidade; `person_profile.birth_date` e `country` ficam fora de coleta até existir consumidor | `M32` (`Q-DP-2`, 2026-11-23); `proposta-dados-pessoais-e-retencao.md` §5.4 |
| coleta real bloqueada até validação jurídica | cadastro e login de pessoa real não começam antes da revisão; nenhuma página declara conformidade | `M32` (`Q-DP-1`, `Q-DP-2`); `proposta-dados-pessoais-e-retencao.md` §5.1 e §5.4 |
| `identifier_hash` é identificador indireto reidentificável | `sha256` sem segredo confirma um endereço candidato; sem export da coluna e sem `SELECT` para runtime de jogo | `M32` (`Q-DP-3`); `M6`; `docs/arquitetura/seguranca.md` |
| tabela crescente sem prazo declarado não vai a produção | as categorias sem prazo ficam `pendente`, com dono `operação` + `produto`; planejamento e DDL seguem livres | `M32` (`Q-DP-4`); `padrao-banco.md` `P-21` |
| exclusão do titular é fechamento com anonimização | nada promete apagamento físico; auditoria (730 dias) e histórico de `game` ficam preservados | `M32` (`Q-DP-5`); `A12`; `M22` |
| log e telemetria não carregam identificador em claro | usar `person_id`, `account_id` ou hash; retenção de log é de `wyd-ops` | `M32` (`Q-DP-6`); `P-22` |
| fixture, golden e captura só com dado sintético declarado | captura de sessão real exige decisão nova, com higienização verificável | `M32` (`Q-DP-7`); `LCLI-04`; `LPRT-10` |
| nenhum fornecedor que receba PII está autorizado | cada candidato entra no registro antes do uso, com região, contrato e instrução de exclusão | `M32` (`Q-DP-8`); `B1` |

## 3. Restrições operacionais

| Restrição | Efeito | Fonte |
|---|---|---|
| nada está em produção e nenhum serviço está no ar | nenhuma página descreve incidente, backup executado ou medição de carga | `AGENTS.md` §4 |
| free tier não serve para produção de jogo | capacidade e disponibilidade não são prometidas com base em plano gratuito | `AGENTS.md` §9 |
| seis seeds esperam dado de origem verificado | `0016`, `0017`, `0018`, `0020`, `0023` e `0027` seguem `bloqueada`; o dado não é inventado para destravar | `plano-migrations.md` §4; `migrations-checklist.md` §3 |
| o site publica apenas `docs/` | link para `migrations/`, `src/`, `tests/` é caminho em `code` com a marca "planejamento", nunca link relativo | `docs/convencoes.md` §Links e citações |

## 4. Premissas (`H`)

Premissa é o que a página pode afirmar **como hipótese**, com o que a invalida declarado. Nenhuma
delas vira regra sem a captura ou a decisão que a fecha.

| Premissa | O que a invalida | Onde está registrada |
|---|---|---|
| o `login` de jogo aceita 1–16 caracteres, sem espaço nas bordas, e o conjunto de caracteres é permissivo | captura do cliente 7662 mostrar charset ou tamanho diferente | `rascunho-accounts.md` §7; `lacunas.md` `A11` (externa) |
| o handshake entrega `account_id`, não `login` | captura do cliente mostrar que o fio carrega o login | `rascunho-accounts.md` §7 |
| o cliente envia o login na mesma caixa do cadastro; `login_fold` existe para que a diferença não crie duas contas | captura mostrar que a caixa é significativa no fio | `rascunho-accounts.md` §7; `ddl-confirmado.md` §5.2 |
| cinco contas da mesma pessoa podem ficar online ao mesmo tempo, em qualquer canal | decisão de produto em contrário; o schema não tem barreira por pessoa nem por IP | `rascunho-accounts.md` §3 |
| o limite de conexões por IP precisa comportar cinco clientes por pessoa mais margem | medição de campo mostrar outro perfil de uso | `rascunho-accounts.md` §3 (`D10`) |
| o cliente alvo é o 7662, com senha de 12 bytes e PIN de 6 no fio | troca de cliente ou de protocolo | `padrao-banco.md` §4; `rascunho-accounts.md` §2 |

## 5. Pendências externas (com dono e prazo)

O registro tem **0 `proposta`, 0 `aberta`** e **3 `externa`** (`lacunas.md` §1, 2026-11-23). As três não
bloqueiam as páginas desta onda: cada uma é execução de frente fora do `wyd-db`.

| Lacuna | Assunto | Dono | Estado | Marco |
|---|---|---|---|---|
| `A11` | charset do `login` é `UNVERIFIED` | frente `wyd-server` (cliente 7662) | externa | `EXT` |
| `A20` | segundo fator do staff: escopo decidido, execução no portal | produto + arquitetura / frente `wyd-portal` | externa | `EXT` |
| `M21` | telas do portal: escopo completo decidido, lista por tela no planejamento do portal | frente `wyd-portal` | externa | `EXT` |

`M8` (MFA na `1.0.0`) está **deferida** e vive dentro de `A20`: o DDL de
`portal.person_authenticator` existe (`ddl-confirmado.md` §5.20), o uso é do portal.

## 6. O que esta página não decide

- Não fecha lacuna: estado, dono e prazo continuam em `lacunas.md`.
- Não decide desenho: coluna, constraint e índice são de `ddl-confirmado.md` e da página de
  `migrations/` do arquivo dono.
- Não mede: contagem de tabela é de `plano-migrations.md` §3; número medido só depois da série.
- Não substitui `docs/planos/riscos.md`: risco é exposição conhecida; aqui é limite e premissa.
