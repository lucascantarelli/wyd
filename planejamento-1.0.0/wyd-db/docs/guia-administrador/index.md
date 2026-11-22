> Pergunta: "o que eu, administrador, preciso saber e com qual credencial?"
> Público: Operação, DBA e quem assume o banco
> Dono: equipe do `wyd-db` com a operação
> Base: `wyd-db-1.0.0.md` §6.12; `docs/arquitetura/seguranca.md`; `padrao-banco.md` §8; `src/provision/spec.md` §3; `configuracao.md`
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

O guia do administrador é para quem **instala, configura, protege e recupera** o banco — não para
quem modela tabela nem para quem escreve regra de jogo. Ele diz o que fazer, com qual credencial e
como confirmar que deu certo; o porquê do privilégio fica em
[`seguranca.md`](../arquitetura/seguranca.md) e o modelo de permissão, em `padrao-banco.md` §8.

## 1. Mapa dos sete guias

| Guia | Pergunta | Estado |
|---|---|---|
| `instalacao.md` | como instalo do zero? | não escrito |
| [`configuracao.md`](configuracao.md) | quais parâmetros existem? | escrito |
| `backup-e-restore.md` | como protejo e recupero o dado? | não escrito |
| `diagnostico.md` | está lento ou travado, e agora? | não escrito |
| `manutencao-rotineira.md` | o que fazer por rotina? | não escrito |
| `recuperacao-de-desastre.md` | perdi o banco, e agora? | não escrito |
| `sustentacao.md` | como assumo este projeto? | não escrito |

Caminho em `code` não é link: a página ainda não existe. O estado de recorte ou de não escrita de
cada uma é o de `proposta-paginas.md` §5–§6, dono da fila; esta página não o reabre.

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
| backup e restore | credencial administrativa do ambiente | **não definida no contrato** desta árvore | — |
| diagnosticar lock, ledger e saturação | credencial administrativa do ambiente | **não definida**; a página de diagnóstico dirá qual | — |
| vacuum e manutenção | `wyd_owner` é `NOLOGIN`; operação por credencial administrativa | **não definida** na árvore | `seguranca.md` |

As três linhas “não definida” são ausência declarada, não permissão implícita: enquanto a página do
guia não fixar a credencial, a tarefa é ato do operador do ambiente, fora do contrato do `wyd-db`.

## 4. Ordem de leitura

1. **Instalar:** `instalacao.md` → `configuracao.md` → `plano-de-implantacao.md` (planejamento).
2. **Operar:** `diagnostico.md` → `manutencao-rotineira.md` → `backup-e-restore.md`.
3. **Sobreviver:** `recuperacao-de-desastre.md` → `sustentacao.md`.

A ordem de **implantação** (banco → schema → seed → serviço) é de
[`plano-de-implantacao.md`](../planos/plano-de-implantacao.md); esta pasta é a operação do dia a dia.

## 5. O que esta página não cobre

- **O privilégio por tabela e a decisão `M27`:** são de
  [`seguranca.md`](../arquitetura/seguranca.md).
- **Os nomes e o efeito de cada variável:** são de [`configuracao.md`](configuracao.md).
- **A política de retenção:** é de `padrao-banco.md` `P-21` e `M22`; a rotina que a executa é de
  `manutencao-rotineira.md` (planejado).
- **O aceite de uma mudança:** é de
  [`plano-de-qualidade-e-gates.md`](../planos/plano-de-qualidade-e-gates.md).
