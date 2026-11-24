> Pergunta: "o que eu, administrador, preciso saber e com qual credencial?"
> Público: Operação, DBA e quem assume o banco
> Dono: equipe do `wyd-db` com a operação
> Base: `wyd-db-1.0.0.md` §6.12; `docs/arquitetura/seguranca.md`; `padrao-banco.md` §8; `src/provision/spec.md` §3; `configuracao.md`; `pendencias-de-fechamento.md` §6.6
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-22)

O guia do administrador é para quem **instala, configura, protege e recupera** o banco — não para quem
modela tabela nem para quem escreve regra de jogo. Ele diz o que fazer, com qual credencial e como confirmar
que deu certo; o porquê do privilégio fica em [`seguranca.md`](../arquitetura/seguranca.md) e o modelo de
permissão, em `padrao-banco.md` §8.

## 1. Mapa dos sete guias

| Guia | Pergunta | Estado |
|---|---|---|
| [`instalacao.md`](instalacao.md) | como instalo do zero? | escrito |
| [`configuracao.md`](configuracao.md) | quais parâmetros existem? | escrito |
| [`backup-e-restore.md`](backup-e-restore.md) | como protejo e recupero o dado? | escrito (2026-11-24) |
| [`diagnostico.md`](diagnostico.md) | está lento ou travado, e agora? | escrito |
| [`manutencao-rotineira.md`](manutencao-rotineira.md) | o que fazer por rotina? | escrito (2026-11-24) |
| [`recuperacao-de-desastre.md`](recuperacao-de-desastre.md) | perdi o banco, e agora? | escrito (2026-11-24) |
| [`sustentacao.md`](sustentacao.md) | como assumo este projeto? | escrito (2026-11-24) |

As sete páginas existem: a pasta está no teto de 8 com o `index`. O estado de cada uma é o de
`proposta-paginas.md` §6, dono da fila; esta página só o reflete.

## 2. O que é responsabilidade do administrador

| É do administrador | Não é do administrador |
|---|---|
| provisionar papéis e `public`, uma vez por banco (`ADR-DB-0001`) | decidir schema, coluna ou constraint |
| aplicar a série com o papel de migração | editar migration aplicada ou `ALTER TABLE` corretivo |
| preencher DSNs e arquivo de ambiente, sem expor valor | escolher regra de negócio ou limite de jogo |
| backup, restore, vacuum, retenção executada e diagnóstico | escrever código de jogo, portal ou admin |
| confirmar que o ambiente está no estado esperado | aprovar página ou decisão de arquitetura |

## 3. Nível de acesso por tarefa

| Tarefa | Credencial ou papel | O que permite | Fonte |
|---|---|---|---|
| instalar e provisionar | `WYD_PROVISION_DSN` (admin do cluster **e** dono do banco) | criar papéis e o estado de `public` | `src/provision/spec.md` §3; `ADR-DB-0001` |
| aplicar a série | `WYD_DB_URL_MIGRATOR` (`wyd_migrator`) | DDL da série e escrita no ledger | `seguranca.md`; `src/migrate/spec.md` §7 |
| consultar e alimentar BI | `WYD_DB_URL_READ` (`wyd_read`) | `SELECT` nas views | `seguranca.md` |
| operar staff | `WYD_DB_URL_ADMIN` (`wyd_admin`) | tabelas do contrato C4 e `ops.audit_log` | `seguranca.md`; `M27` |
| configurar parâmetros | as DSNs, por nome | cada processo lê só a sua; não há `DATABASE_URL` | [`configuracao.md`](configuracao.md) |
| backup e restore | credencial administrativa do ambiente | cópia e restauração completas; `wyd_read` só lê views e `wyd_migrator` é de DDL | [`backup-e-restore.md`](backup-e-restore.md) §1 |
| diagnosticar lock, ledger e saturação | credencial administrativa do ambiente | leitura de catálogo e de sessão | [`diagnostico.md`](diagnostico.md) |
| vacuum e manutenção | credencial administrativa do ambiente; `wyd_owner` é `NOLOGIN` | `vacuum`, expurgo e análise | `seguranca.md`; `planos/plano-de-manutencao.md` §2 |

A credencial administrativa **não tem nome fixado** no contrato desta árvore: o DSN de cada tarefa
administrativa é ato do operador do ambiente até `configuracao.md` ou uma decisão de operação fixá-lo
(`Q-BLK-04` = A, 2026-11-24: inventário por função, credencial nominal fora). Ausência declarada, não
permissão implícita — quem opera está em [`sustentacao.md`](sustentacao.md) §3.

## 4. Ordem de leitura

1. **Instalar:** [`instalacao.md`](instalacao.md) → [`configuracao.md`](configuracao.md) →
   [`plano-de-implantacao.md`](../planos/plano-de-implantacao.md).
2. **Operar:** [`diagnostico.md`](diagnostico.md) → [`plano-de-manutencao.md`](../planos/plano-de-manutencao.md)
   → [`backup-e-restore.md`](backup-e-restore.md).
3. **Sobreviver:** [`recuperacao-de-desastre.md`](recuperacao-de-desastre.md) → `sustentacao.md` (planejado).

A ordem de **implantação** (banco → schema → seed → serviço) é de
[`plano-de-implantacao.md`](../planos/plano-de-implantacao.md); esta pasta é a operação do dia a dia.

## 5. O que esta página não cobre

- **O privilégio por tabela e a decisão `M27`:** são de [`seguranca.md`](../arquitetura/seguranca.md).
- **Os nomes e o efeito de cada variável:** são de [`configuracao.md`](configuracao.md).
- **A política de retenção:** é de `padrao-banco.md` `P-21`, `M22` e
  [`retencao-e-capacidade.md`](../arquitetura/retencao-e-capacidade.md).
- **A rotina que executa a retenção:** é de [`plano-de-manutencao.md`](../planos/plano-de-manutencao.md);
  o guia de manutenção rotineira ainda não existe e não é pré-requisito de nada.
- **O aceite de uma mudança:** é de
  [`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md).
