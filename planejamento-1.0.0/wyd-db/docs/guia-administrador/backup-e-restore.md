> Pergunta: "como protejo o banco e como recupero o dado, com que frequência e com que prazo?"
> Público: Operação e DBA do `wyd-db`
> Dono: equipe do `wyd-db` com a operação
> Base: `proposta-retencao-pendente.md` §9 (`Q-RET-7 = A`); `pendencias-de-fechamento.md` §6.6 (`Q-BLK-03`, aprovada em 2026-11-24); `ADR-DB-0001`; `docs/arquitetura/seguranca.md`; `src/provision/spec.md` §3; `configuracao.md`
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-24)

A política já estava decidida (`Q-RET-7 = A`); faltavam **onde** a cópia mora e **qual prazo** ela promete.
As duas coisas foram respondidas em 2026-11-24 (`Q-BLK-03`). Esta página é a dona do procedimento de
cópia e restauração; a rotina que a agenda é de [`plano-de-manutencao.md`](../planos/plano-de-manutencao.md)
e o cenário de perda total é de
[`recuperacao-de-desastre.md`](recuperacao-de-desastre.md).

## 1. Política

| Item | Valor | Fonte |
|---|---|---|
| frequência | **1 cópia por dia** | `Q-RET-7 = A` |
| retenção | **30 dias**, com rotação | `Q-RET-7 = A` |
| destino | **storage de objeto do mesmo provedor da VPS**, fora do host | `Q-BLK-03`, 2026-11-24 |
| criptografia | em repouso no destino, **fora do host**; a chave não mora no mesmo host do banco | `Q-RET-7 = A` |
| RPO | **24 h** — perda máxima aceita é o último dia | `Q-BLK-03`, 2026-11-24 |
| RTO | **4 h** — do incidente ao banco validado | `Q-BLK-03`, 2026-11-24 |
| ensaio de restore | **trimestral**, com RTO medido | `Q-RET-7 = A` |
| credencial | administrativa do ambiente, **não fixada** no contrato da árvore | `guia-administrador/index.md` §3 |

> [!NOTE] Por que o RPO é 24 h e não menos
> A política decidida é de uma cópia diária. Prometer RPO menor exigiria arquivamento contínuo de WAL, que
> não foi decidido e não existe. O número declara o que a política entrega; não é meta aspiracional.

## 2. O que entra na cópia

| Objeto | Como | Por quê |
|---|---|---|
| o banco `wyd` (schema `game`, `portal`, `ops`, `content`, views e dados) | `pg_dump --format=custom` | é o dado que a operação recupera |
| **papéis do cluster** | `pg_dumpall --globals-only` | os papéis **não nascem da série**: são provisionados fora dela (`ADR-DB-0001`), e sem eles o restore não consegue reatribuir dono |
| a série aplicada | já vem dentro do banco, em `public.schema_migrations (version, slug, checksum)` | é como se prova **qual** versão foi restaurada |
| segredo, `.env` e chave de criptografia | **nunca** no dump | `AGENTS.md` §5, item 7 |

Fora da cópia: binário do runner, imagem do container e configuração do host — esses são reconstruídos pelo
Compose, não restaurados de backup.

## 3. Comando de backup

```bash
# 1. estrutura e dados
pg_dump --format=custom --no-password \
  --file="backup-$(date +%F).dump" "$DSN_ADMIN"

# 2. papéis do cluster (globals), que a série não cria
pg_dumpall --globals-only --no-password \
  --file="roles-$(date +%F).sql"
```

`DSN_ADMIN` é rótulo do DSN administrativo do ambiente: `pg_dump` precisa ler tabelas que `wyd_read` não
alcança (ele só tem `SELECT` nas views) e o papel de migração é de DDL, não de cópia. O nome da variável é
de `configuracao.md`; a credencial de backup continua **não fixada** (§1).

| Passo | Verificação |
|---|---|
| gerar os dois arquivos | ambos existem, com tamanho maior que zero |
| cifrar e enviar | o objeto no destino existe com a data do dia e o checksum registrado |
| rotação | remover o objeto com mais de 30 dias **depois** de confirmar o do dia |
| falha | código de saída ≠ 0 aborta a rotação: sem cópia nova, a antiga fica |

## 4. Ensaio de restore (trimestral)

1. Subir um ambiente **separado** com o Compose, sem tocar no banco em uso.
2. Aplicar os papéis: `wyd-db provision plan` e `apply` com `WYD_PROVISION_DSN` (`src/provision/spec.md` §3),
   ou restaurar o `roles-AAAA-MM-DD.sql`. Sem os papéis, o passo 3 falha por dono ausente.
3. Restaurar o banco: `pg_restore --exit-on-error --dbname="$DSN_ADMIN" backup-AAAA-MM-DD.dump`.
4. Validar com a tabela de §5.
5. Registrar o **RTO real** no log do ensaio; se passar de 4 h, o desvio vira item de
   [`riscos.md`](../planos/riscos.md), não um número novo.

## 5. Como medir sucesso

| Prova | Comando | Critério |
|---|---|---|
| versão restaurada | `select count(*) from public.schema_migrations;` | igual ao da origem no momento da cópia |
| dado por tabela | `select count(*)` nas tabelas de controle (as que a rotina de expurgo toca, mais `game.account`, `game.character`, `portal.person`) | igual à origem; diferença é falha de restore, não “arredondamento” |
| papéis | `\du` no cluster restaurado | os papéis de `seguranca.md` §1 existem |
| privilégio | `select has_table_privilege('wyd_admin','game.account','INSERT');` | `true` — prova que o restore não perdeu `GRANT` de `M27` |

Contagem de tabela **não** prova conteúdo: prova que a cópia abriu. A prova de conteúdo é a mesma consulta
na origem e no restaurado, no mesmo momento.

## 6. O que não fazer

- Copiar o arquivo do volume com o banco em execução: o `.dump` é feito pela ferramenta, não pelo
  sistema de arquivos.
- Guardar a cópia no mesmo host ou no mesmo volume do banco: não sobrevive à perda do host.
- Passar senha em `--dbname` com valor, em script ou em log: o DSN vem de variável de ambiente.
- Restaurar sobre o banco em uso sem janela e sem parar os processos que escrevem.
- Usar dado real em ensaio de restore fora do ambiente do dono: o ensaio roda com a cópia de produção e é
  ato de operação, não de teste de página (`M32`).
- Rodar a rotação antes de confirmar a cópia do dia.

## 7. O que esta página não decide

- **O agendamento:** é de [`plano-de-manutencao.md`](../planos/plano-de-manutencao.md) (`cron` do host,
  `Q-BLK-02`).
- **O cenário de perda total, corrupção e migration destrutiva:** é de
  [`recuperacao-de-desastre.md`](recuperacao-de-desastre.md).
- **A credencial nominal e o inventário de papéis:** `guia-administrador/index.md` §3 e `Q-BLK-04`
  (pendente).
- **RPO/RTO de outros componentes** (portal, launcher, site): não são do `wyd-db`.
- **A rotina de expurgo:** é de `plano-de-manutencao.md`; backup não expurga tabela.
