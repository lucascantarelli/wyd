> Pergunta: "como instalo o `wyd-db` do zero e como confirmo cada passo?"
> Público: Operação e DBA que instalam o banco
> Dono: equipe do `wyd-db` com a operação
> Base: `ADR-DB-0001`; `src/provision/spec.md` §3–§4; `plano-migrations.md` §9; `plano-de-implantacao.md`
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

A instalação tem uma ordem que não pode ser trocada: **papéis → `public` → série → seed → serviço**. O
provisionamento é uma vez por banco e fica fora da série; a série não cria papel nem credencial. Esta
página cobre o procedimento do banco; porta de serviço, volume, imagem e tempo esperado não foram
decididos e não são inventados aqui.

## 1. Pré-requisitos

| Pré-requisito | Como confirmar | Se falhar |
|---|---|---|
| PostgreSQL 17 | `SELECT version()` mostra a versão 17 | não prosseguir; a série não é suportada em outra major |
| DSN administrativa | `WYD_PROVISION_DSN` existe e conecta como dono do banco/`CREATEROLE` | erro nomeia a variável; não usar `DATABASE_URL` |
| DSN do migrator | `WYD_DB_URL_MIGRATOR` existe para `wyd_migrator` | provisionar o papel antes de aplicar a série |
| diretório da série | `WYD_MIGRATIONS_DIR` aponta para os `.sql` (default `/migrations`) | conferir caminho; não editar migration por fora |
| cliente 7662 (opcional) | só para o corte vertical, não para instalar | o corte fica pendente, não vira “verde” |

## 2. Ordem

1. **Renderizar os papéis.** `provision render` gera o texto de `db/roles.sql`; `render --check`
   compara com o arquivo versionado. O render é determinístico e não lê ambiente.
2. **Aplicar os papéis.** `provision apply` cria/ajusta os seis papéis e o estado de `public`. É
   idempotente; rodar de novo converge. O comando não tem `down` e nunca remove papel.
3. **Conferir o plano.** `provision plan` ou `check` compara `pg_roles` com `ROLES`; drift de `login`
   é recusado, papel extra é ignorado.
4. **Aplicar a série.** `migrate apply` recebe a DSN `wyd_migrator`, pega o advisory lock, cria o
   ledger, aplica os arquivos em ordem e registra checksum.
5. **Conferir o ledger.** `migrate status` mostra cada versão como `applied`; `status --check` sai 4
   em drift, arquivo faltante ou série inválida.
6. **Semear.** Os seeds são arquivos da série; os seis bloqueados não entram até o dado de origem ser
   conferido. Seed não é passo separado do DDL quando está no mesmo arquivo.
7. **Subir o serviço.** O boot lê `content_version` e falha se o schema/versão não servir. Serviço não
   é instalado por esta página.

## 3. Como confirmar cada passo

| Passo | Confirmação | Evidência |
|---|---|---|
| papéis | `provision check` sem drift de `login` | saída do comando e `pg_roles` |
| `public` | `CREATE` público revogado; `wyd_migrator` pode criar | consulta de privilégio |
| série | `migrate status` lista 34 aplicadas | ledger e saída do runner |
| checksum | reaplicar não muda nada | `apply` seguinte sem escrita |
| seed | contagem do dataset confere com a fonte | contagem registrada na página do seed |
| serviço | boot lê o catálogo e abre sessão | log do serviço, sem DSN |

O número **34** é planejado (`P`); a série pode ter menos arquivos enquanto os seeds bloqueados não
forem escritos. A confirmação é a lista real do ledger, não o número desta página.

## 4. O que fazer se falhar

| Falha | Ação |
|---|---|
| `provision` não conecta | conferir `WYD_PROVISION_DSN`; a mensagem nomeia a variável, não o valor |
| `migrate` recusa papel | usar `wyd_migrator`; não desligar o preflight em produção |
| lock ocupado (código 3) | localizar o outro processo em `pg_locks`; não matar sessão sem confirmar o dono |
| drift (código 4) | comparar checksum/arquivo; não editar a migration aplicada |
| SQL falha (código 5) | corrigir o arquivo antes de reaplicar; o `down` do arquivo anterior não é automático |
| seed bloqueado | parar; o dado não foi conferido, e contar como concluído seria falso |

## 5. O que esta página não cobre

- **Instalação de serviço, porta, volume, imagem e tempo:** não há desenho aprovado.
- **Backup, restore e recuperação:** são de `backup-e-restore.md` e `recuperacao-de-desastre.md`,
  ainda bloqueados por política de operação.
- **Rotação de senha e `pg_hba.conf`:** são do ambiente; `render` não toca credencial.
- **Carga da base legada:** não decidida na `1.0.0`.
