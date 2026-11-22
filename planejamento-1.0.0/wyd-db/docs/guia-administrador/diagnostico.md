> Pergunta: "a migration está travada ou o schema divergiu; como confirmo e o que faço?"
> Público: Operação e DBA de plantão
> Dono: equipe do `wyd-db` com a operação
> Base: `src/migrate/spec.md` §3–§7 e §11; `guia-administrador/configuracao.md`; `plano-de-qualidade-e-gates.md`
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

O diagnóstico deste recorte cobre **lock de migration e ledger divergente**. Pool saturado, disco,
conexões e latência não têm medição nem limiar aprovado; a página não inventa um número. O princípio é
não matar sessão nem editar ledger sem identificar o dono e o efeito.

## 1. Sintoma → causa provável → verificação

| Sintoma | Causa provável | Verificação | Ação |
|---|---|---|---|
| `apply` espera e sai com código 3 | outro processo segura o advisory lock | consultar `pg_locks` pela chave `MIGRATION_LOCK_KEY` e ver a sessão dona | esperar o processo terminar; não matar sessão alheia sem confirmar |
| `status --check` sai 4 | checksum, slug ou arquivo faltando | comparar versão/slug e o arquivo no diretório | restaurar o arquivo correto; não editar a migration aplicada |
| `apply` falha no preflight | conexão não é `wyd_migrator` ou servidor não é 17 | conferir `current_user` e `server_version` | usar a DSN correta; não desligar o preflight em produção |
| ledger tem coluna a menos | schema do ledger divergente | comparar a forma de `schema_migrations` com o contrato | parar e corrigir o ledger; não “recriar” por conveniência |
| `down` não chega a zero | `down` de arquivo incompleto | aplicar `down` e contar tabelas/views ao fim | corrigir o `down` no arquivo; não fazer `DROP SCHEMA` |
| boot recusa o schema | drift ou SQL falhou | rodar `status --check` e ler o código | não subir com divergência; corrigir a causa |

## 2. Lock

O runner usa **advisory lock de sessão**, não de transação, porque uma sessão atravessa uma transação
por arquivo. A chave é constante e documentada em `src/migrate/spec.md` §5
(`MIGRATION_LOCK_KEY = 0x5759_4404`); mudá-la é mudar o contrato entre versões do binário. O timeout
padrão é 10 s e o estouro vira `LockBusy`, com código 3.

Ao investigar:

1. descubra a sessão que segura o lock em `pg_locks`;
2. confirme se é um runner legítimo, qual banco e qual usuário;
3. espere o término ou peça ao dono para encerrar;
4. só então rode `apply` de novo.

Não use `pg_terminate_backend` em sessão desconhecida: o lock pode estar protegendo uma aplicação de
arquivo no meio. O lock não é reentrante e não é “liberado” por `unlock` exposto na porta.

## 3. Ledger e drift

| Achado | Significado | Não faça |
|---|---|---|
| `Checksum` | o conteúdo do disco difere do que foi aplicado | editar a linha do ledger para “igualar” |
| `Slug` | arquivo renomeado depois de aplicado | renomear de volta sem conferir a página |
| `MissingFile` | o ledger tem versão sem arquivo | apagar a linha; a versão existe no banco |
| série inválida | buraco, duplicata ou nome fora do padrão | renumerar em produção sem plano |
| ledger sem forma | coluna/tipo divergente | recriar a tabela sem backup |

O checksum é `sha256` dos bytes normalizados (`CRLF` → `LF`); um byte de diferença é drift. A correção
é na origem (arquivo/migration), não no teste nem no ledger.

## 4. O que ainda não há

- **Pool, conexões e saturação:** sem métrica, sem limiar e sem painel; não há ação prescrita.
- **Disco e vacuum:** dependem de volume medido; a página de capacidade está bloqueada.
- **Métrica de latência do boot:** não medida.
- **Runbook de incidente:** este recorte é técnico; comunicação e escalada são do `wyd-ops`.

## 5. Escalonamento

Se o lock for de um processo legítimo, espere. Se o ledger estiver divergente, pare a implantação e
preserve o banco. Se houver suspeita de corrupção ou perda de volume, a decisão é de recuperação de
desastre, não desta página; restaurar por cima de um ledger divergente pode piorar o estado.
