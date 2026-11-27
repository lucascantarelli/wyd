> Pergunta: "que contas e dados a suíte usa, e o que nunca entra em teste?"
> Público: QA / testador
> Dono: equipe do `wyd-db`
> Base: `../docs/guia-teste/dados-de-teste.md` (política); `resources/README.md` (formato do JSON); `../docs/guia-administrador/configuracao.md` §3
> Revisado em: 2026-11-24

**Estado:** aceito (2026-11-24) — catálogo planejado; nenhuma conta criada.

**Quatro níveis de afirmação convivem aqui:**

- **`P`** — tudo o que a página diz sobre as contas: nenhuma existe; a factory e `resources/accounts.json`
  também não.
- **`H`** — dúvida aberta, dita e localizada em §5, com o dono que a fecha.
- **`V`** — o que foi medido nesta árvore em 2026-11-24, com o arquivo ao lado.
- **sem marca** — regra de projeto (`../docs/convencoes.md`) ou comportamento do PostgreSQL
  (`../padrao-banco.md` §13).

O catálogo concreto de contas de teste. A política — o que pode e o que não pode ser dado de teste — é de
`../docs/guia-teste/dados-de-teste.md`; aqui ficam as contas, o papel de cada uma e o que ela prova.

**Alvo escrito onde roda:** `sistema_pg` (as cinco contas de `CAP-14`, a anonimização de `CAP-13`) e
`sistema_sessoes` (`CAP-20`, com a DSN do papel testado). Os dois alvos de contrato não usam conta: não
abrem conexão.

## 1. As contas da suíte

| Conta | `person.role` | Credencial usada | O que prova | Como nasce |
|---|---|---|---|---|
| `qa_jogador` | `member` | `WYD_DB_URL_PORTAL` | escrita do titular no próprio perfil, sessão de portal | `factory::pessoa` |
| `qa_staff` | `moderator` | `WYD_DB_URL_ADMIN` | bloqueio com motivo, autor e instante (`CAP-20`) | `factory::pessoa` |
| `qa_admin` | `admin` | `WYD_DB_URL_ADMIN` | concessão e papel de jogo, quando o alvo precisar | `factory::pessoa` |
| `qa_leitura` | — (não é pessoa do jogo) | `WYD_DB_URL_READ` | leitura operacional e recusa de escrita | papel do provisionamento |
| `qa_migrator` | — | `WYD_DB_TEST_URL` | aplicação da série no banco descartável | provisionamento |

As três primeiras são **pessoas** e têm `person.role`; as duas últimas são **papéis de banco** e não têm
linha em `portal.person`. Quem entra sem credencial própria é o jogador de jogo
(`WYD_DB_URL_GAME`), e ele não aparece aqui porque nenhum alvo do crate conecta como `wyd_game`: a
fronteira de escrita do jogo é provada por `CAP-20`, com a DSN do papel testado.

## 2. As cinco contas da mesma pessoa

`CAP-14` precisa de **uma pessoa com cinco contas** e de uma sexta tentativa recusada por outro motivo
que não pessoa ou IP. A factory cria as cinco com a mesma chave de pessoa, em vagas 1 a 5, e tenta a
sexta na vaga **6**, que o DDL não admite (`ck_account_slot_range`) — o cenário prova que a barreira é a
vaga (`RN-DB-001`), não a pessoa (`RN-DB-003`). A vaga fechada e reutilizada é outro cenário (`CAP-10`).

| Vaga | Estado | O que o cenário faz |
|---|---|---|
| 1..5 | `active` | cria e mantém |
| 1 | `closed` | fecha e reabre com conta nova — vaga reutilizada |
| 6 | — | tenta criar: falha por vaga, com erro nomeado |

## 3. Relação com `seeds/dev.sql`
**Sem arquivo no repositório:** `resources/accounts.json` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

| Fonte | O que é | A suíte usa? |
|---|---|---|
| série de conteúdo (`0016`–`0027`) | catálogo, mundo, regras, quests | **sim**: o alvo aplica a série real e lê o que ela semeou |
| `seeds/dev.sql` | dado de desenvolvimento para abrir o jogo à mão | **não**: não é fixture nem conta de teste |
| `resources/accounts.json` | contas de teste por papel ([resources/README.md](resources/README.md)) | sim, como **ficha**, não como linha já no banco |

Seed de desenvolvimento nunca é dado de produção e nunca é dependência de teste: teste que só passa com
`seeds/dev.sql` aplicado depende de um ambiente que o CI não tem.

## 4. O que nunca entra
**Sem arquivo no repositório:** `resources/accounts.json` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

- **Dado de jogador real**, conta, personagem, saldo ou e-mail de pessoa real.
- **Segredo real:** senha ou token de qualquer ambiente; a única senha de teste será a fictícia
  declarada em `resources/accounts.json`.
- **Dado do cliente 7662:** nome de item, preço ou contagem de conteúdo — isso é seed, não teste
  (`../pedido-de-curadoria-dos-seeds.md`).
- **Id mágico:** `1`, `42` e `7662` fixos no teste; a factory devolve o id.
- **`sleep` como sincronização:** espera por tempo esconde corrida; o cenário espera por estado.

## 5. Dúvidas abertas

| Dúvida | O que a resolve | Dono |
|---|---|---|
| o que o cenário de `CAP-14` compara quando a 6ª conta é recusada: nome da constraint, `SQLSTATE` ou mensagem? | o DDL fixa `ck_account_slot_range` (`ddl-confirmado.md`); a mensagem é do PostgreSQL e não é contrato | autor do alvo |
| a senha fictícia é uma só para todas as contas e ambientes? | `resources/README.md` §2 diz "fictícia e igual em todo ambiente"; não há decisão que fixe o valor nem o hash | autor do alvo, revisado no PR |

## 6. O que esta página não decide

- **Não define política de dado:** é de `../docs/guia-teste/dados-de-teste.md`.
- **Não cria conta:** a criação é da factory ([fixtures-e-factories.md](fixtures-e-factories.md) §3).
- **Não define credencial de produção:** os papéis e as DSNs são de
  `../docs/guia-administrador/configuracao.md` §3.
- **Não decide o seed de conteúdo:** os seis seeds são de `../docs/planos/plano-de-dados-e-seeds.md` e
  seguem `bloqueada` até a curadoria.
