> Pergunta: "quem pede o quê, e quem aprova mudança de requisito?"
> Público: Product owner, líder de sistema consumidor, staff e administrador
> Dono: equipe do `wyd-db`
> Base: `../arquitetura/contexto-c4.md` §2; `../arquitetura/seguranca.md`; `../guia-administrador/configuracao.md` §3; `requisitos-funcionais.md`; `regras-de-negocio.md`; `index.md`
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-24)

Esta página responde "quem pede o quê?" para os **5** atores do banco: **4** credenciais de produto
(`GAME`, `PORTAL`, `ADMIN`, `READ`) e **3** valores de `person.role` (`member`, `moderator`, `admin`), que
não formam conjuntos disjuntos — staff e administrador compartilham a credencial `ADMIN` e se distinguem
pelo `role`. A decisão `Q-BLK-09` = A (2026-11-24) fixou esse recorte: **5** linhas, com o jogador dentro
e o runner e o provisionamento fora.

## 1. Os cinco atores

| # | Ator | Quem é | Credencial | `person.role` | O que pode no banco |
|---:|---|---|---|---|---|
| 1 | **jogador** | quem joga; entra pelo cliente e pelo servidor de jogo | nenhuma própria — quem conecta é `WYD_DB_URL_GAME` | — (não tem pessoa) | nada direto: o estado dele é escrito pelo servidor de jogo |
| 2 | **titular do portal** | pessoa com conta no portal; compra, resgata e recupera acesso | `WYD_DB_URL_PORTAL` | `member` | o próprio perfil, consentimento, token e resgate; nunca o saldo de jogo |
| 3 | **staff** | quem modera: bloqueia conta, lê trilha | `WYD_DB_URL_ADMIN` | `moderator` | bloqueio, desbloqueio e leitura de auditoria |
| 4 | **administrador** | quem concede item e papel, e responde pelo conteúdo | `WYD_DB_URL_ADMIN` | `admin` | concessão, papel de jogo e configuração |
| 5 | **operador** | quem opera o banco: backup, restauração e leitura de indicador | `WYD_DB_URL_READ` | — (não é `person.role`) | leitura operacional; nenhuma escrita de produto |

O operador não tem linha em `person.role` porque não é pessoa do jogo: ele lê por privilégio do papel
`wyd_read`, e escrever não é o trabalho dele. O jogador não tem credencial porque nenhum processo do
cliente fala com o banco: o caminho é cliente → `wyd-server` → `WYD_DB_URL_GAME`.

## 2. Ator → interesse → requisito que o atende

| Ator | Interesse | Requisito que o atende | Por quê |
|---|---|---|---|
| jogador | não perder progresso ao reconectar | `RF-DB-006`, `RN-DB-002` | uma sessão viva por conta e por personagem |
| jogador | saldo e cofre não somem | `RF-DB-003`, `RF-DB-004`, `RN-DB-005` | saldo em inteiro de 32 bits, cofre preso à conta |
| jogador | cinco contas da mesma casa, online juntas | `RF-DB-014`, `RF-DB-015`, `RF-DB-016`, `RN-DB-003` | o limite é por conta, não por pessoa ou IP |
| titular do portal | recuperar acesso sem expor segredo | `RF-DB-010`, `RF-DB-013` | token só como hash; senha e PIN só como argon2id |
| titular do portal | sair e não deixar rastro pessoal | `RN-DB-004` | fechar pessoa anonimiza o e-mail |
| titular do portal | comprar e receber o que comprou | `RF-DB-008`, `RF-DB-009` | donate idempotente; grant pendente não se perde |
| titular do portal | resgatar uma vez por dia | `RF-DB-005` | resgate diário único por conta e dia |
| staff | bloquear com motivo e responsável | `RF-DB-007`, `RN-DB-006` | bloqueio é moderação, com autor, motivo e instante |
| staff | provar quem fez o quê | `RF-DB-011` | auditoria append-only, sem FK |
| administrador | conceder item e papel sem furar o jogo | `RF-DB-008`, `RF-DB-009`, `RF-DB-012` | concessão idempotente; papel de jogo é fechado |
| administrador | conta nasce na vaga certa | `RF-DB-001`, `RN-DB-001` | login de 1 a 16 caracteres; vaga 1..5 |
| administrador | PIN não vira oráculo | `RF-DB-002`, `RN-DB-007` | falha acumula; limiar é política, estado é do banco |
| operador | diagnosticar sem derrubar o jogo | `RNF-DB-006`, `RNF-DB-007` | escrita concorrente sem `40P01`; espera por conexão finita em `30 s` e reuso |
| operador | ler trilha confiável | `RF-DB-011` | a trilha não é reescrita nem apagada |
| todos | instalar e atualizar sem surpresa | `RF-DB-017` | o banco admite uma única versão corrente de conteúdo |

Os três `UC` não aparecem nesta tabela porque são a soma de requisitos já listados: `UC-DB-001`
(`RF-DB-010`, `RF-DB-013`, `RN-DB-004`), `UC-DB-002` (`RF-DB-001`, `RF-DB-003`, `RF-DB-012`,
`RN-DB-001`, `RN-DB-005`) e `UC-DB-003` (`RF-DB-002`, `RF-DB-006`, `RF-DB-014`, `RN-DB-002`,
`RN-DB-003`).

## 3. Quem aprova mudança de requisito

A regra é a de [`index.md`](index.md) §"Como um requisito nasce, muda e é retirado": **muda por decisão do
dono**, aparece na página dona e nas que a citam **na mesma entrega**, e mudança que altera contrato não é
edição silenciosa.

| Tipo de mudança | Quem decide | Onde fica registrado | Quem é citado na mesma entrega |
|---|---|---|---|
| enunciado, limiar ou critério de aceitação | dono da página que enuncia (equipe do `wyd-db`) | a própria página dona | as páginas que a citam |
| contrato de porta (`accounts`, `wallet`, `session`, …) | o mesmo dono, com a decisão registrada | página dona + `../arquitetura/componentes-c4.md` | o sistema consumidor da porta |
| forma de tabela, `CHECK` ou índice | dono do DDL | `../../ddl-confirmado.md` + migration | dicionário e `rastreabilidade.md` |
| papel, credencial ou privilégio | decisão com ADR | `../arquitetura/adr/` (`M27`, `adr-db-0001`) | `../guia-administrador/configuracao.md`; esta página |
| retirada de requisito | dono da página dona | página com estado `retirado`, motivo e data | `rastreabilidade.md` (a linha sai) |

**O que "aprovar" não significa aqui:** não existe comitê, votação nem prazo de silêncio. Aprovar é o dono
da página dona registrar a decisão com data; o que a tabela acima acrescenta é **quem precisa ser citado**
para a mudança não chegar quebrada ao consumidor.

## 4. Quem está fora da matriz

| Processo | Por que não é ator desta página |
|---|---|
| runner de migrations | one-shot; aplica a série com `WYD_DB_URL_MIGRATOR` e não atende requisito de usuário |
| provisionamento | one-shot; cria papel e estado de `public` com `WYD_PROVISION_DSN`, fora da série |
| BI e relatório futuro | lê pelo papel `wyd_read`, como o operador; ganha linha própria no dia em que tiver requisito |

Os dois primeiros estão no C4 como containers (`../arquitetura/contexto-c4.md` §3) e não como atores: eles
não pedem nada, executam o que a série e o provisionamento mandam. Colocá-los na matriz faria a página
afirmar que eles atendem requisito de jogador — foi por isso que a alternativa C de `Q-BLK-09` foi
recusada.

## 5. O que esta página não decide

- **Não enuncia requisito:** só liga ator a requisito existente; o texto e o limiar ficam na página dona.
- **Não decide permissão de banco:** `GRANT` e `REVOKE` são de [`../arquitetura/seguranca.md`](../arquitetura/seguranca.md)
  e do provisionamento.
- **Não decide papel de produto:** `member`, `moderator` e `admin` são valores de `person.role` no DDL; o
  que cada um pode no portal é do `wyd-portal`.
- **Não cria ator novo:** ator entra quando tiver requisito nomeado, pela regra de §3.
- **Não substitui o C4:** container não é ator, e a lista de containers é de
  [`../arquitetura/contexto-c4.md`](../arquitetura/contexto-c4.md).
