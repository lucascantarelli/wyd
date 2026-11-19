> Pergunta: "por que a série e o runner têm estas regras — ledger por versão, checksum do arquivo inteiro, transação por arquivo, lock com chave própria e `down` obrigatório?"
> Público: Arquiteto de schema e desenvolvedor que for mexer em `crates/wyd-db/src/migrate/` ou na série
> Dono: equipe do `wyd-db`
> Base: `src/migrate/spec.md` §2–§6, `plano-migrations.md` §2, `adr/adr-db-0002-runner-de-migrations-proprio.md`
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18)

Esta página responde **por que** as regras existem. O **contrato** — assinatura, tipo, variante de
erro e código de saída — mora nas páginas de `src/migrate/` (planejamento), na
[`ADR-DB-0002`](adr/adr-db-0002-runner-de-migrations-proprio.md) e no `plano-migrations.md`, e esta
página só o cita ([`../convencoes.md`](../convencoes.md) §*Nota de enunciação*). Toda
afirmação desta página é `P`: nem a série nem o módulo `migrate/` existem no repositório ainda.

## 1. Duas perguntas que não se misturam

A maior parte da confusão sobre migrations vem de tratar "a série" e "o runner" como o mesmo assunto.
São dois desenhos com donos diferentes:

| Pergunta | Dono |
|---|---|
| que arquivos existem, em que ordem, e o que cada um cria? | `plano-migrations.md` §2–§3 |
| o que este arquivo faz e como eu reverto? | páginas `migrations/NNNN-<slug>.md` (`wyd-db-1.0.0.md` §6.15) |
| como o binário descobre, valida, aplica e registra? | `src/migrate/spec.md` + as páginas de `src/migrate/` |
| por que a decisão estrutural foi esta? | [`ADR-DB-0002`](adr/adr-db-0002-runner-de-migrations-proprio.md) e esta página |

"O arquivo não abre transação" é regra do **runner** sobre a série, não escolha editorial de quem
escreve o `.sql` — é por isso que ela aparece em `src/migrate/spec.md` §2 e não em
`plano-migrations.md` §2. Quando as duas listas divergem, o defeito é de fronteira: numeração e ordem
são do plano; forma e execução são do runner.

## 2. Por que cada regra

### 2.1 Ledger por versão

A pergunta operacional é uma só: **este banco já aplicou o arquivo N, e é o mesmo arquivo?** O ledger
existe para respondê-la sem olhar o schema.

- **`version` inteira como chave**, e não o nome do arquivo: renomear um slug é mudança de identidade
  (`DriftKind::Slug`), não versão nova; e inteiro é ordenável, comparável e imune a zero à esquerda.
- **`slug` guarda a identidade**, porque dois arquivos podem ter o mesmo conteúdo em versões
  diferentes e o operador precisa reconhecer o arquivo na mensagem de erro.
- **`applied_at` vem do banco (`now()`)**, não do processo: dois hosts não precisam de relógio
  sincronizado para a ordem ser verdadeira. Duração e `down` não entram — o primeiro é observabilidade,
  o segundo não roda em produção.
- **Ledger em `public`, criado pelo runner**: ele precisa existir **antes** do primeiro arquivo, e um
  schema de domínio criaria dependência circular. É a exceção declarada de `ddl-confirmado.md` §3, e a
  razão do `GRANT` de `0001` (`ADR-DB-0001`).
- **`prepare` compara a forma em vez de usar `IF NOT EXISTS`**: o ledger errado por baixo — coluna
  faltando, tipo trocado, `CHECK` removida — passaria calado e apareceria depois como drift de
  conteúdo, longe da causa.
- **Tabela ausente é ledger vazio, não erro**: `status` e `apply --dry-run` não criam tabela, e num
  banco virgem a resposta correta é "nada aplicado", não "forma inválida". Quem cria é `prepare`, sob
  lock.

Contrato em `src/migrate/spec.md` §3 e `src/migrate/port/ledger.md`.

### 2.2 Checksum do arquivo inteiro

Um arquivo aplicado é **imutável**, e o checksum é o que transforma essa frase em regra executável.

- **Cobre cabeçalho, `up` e `down`**, não só o `up`: o `down` também é código que roda — em
  desenvolvimento, no ciclo de teste e no rollback —, e alterá-lo depois de aplicar muda o
  comportamento sem deixar registro. Checksum só do `up` aceitaria essa mudança em silêncio.
- **EOL normalizado e uma quebra final descartada**: um clone no Windows com `autocrlf` produziria
  checksum diferente para o mesmo commit, e o runner acusaria drift que não existe. Dois `\n` no fim
  são conteúdo; um só é o que o editor acrescenta.
- **Consequência aceita**: corrigir um comentário num arquivo já aplicado é erro de checksum, e o
  caminho é arquivo novo. É desconfortável de propósito — reescrever o passado é o que o ledger existe
  para impedir. A alternativa (confiar em disciplina) já falhou na série herdada.

Contrato em `src/migrate/spec.md` §4 e `src/migrate/migration/checksum.md`; o fluxo de detecção está em
`src/migrate/migration/plan.md`.

### 2.3 Uma transação por arquivo

A escolha é entre dois riscos: desfazer trabalho válido ou não saber onde parou. O desenho escolhe o
segundo.

- **Arquivo e linha do ledger commitam juntos**: não existe "schema aplicado sem linha" nem "linha sem
  schema". Um estado desses é impossível de diagnosticar depois, porque o ledger é justamente a fonte
  que se consultaria.
- **Falha desfaz só aquele arquivo**, e a mensagem carrega a versão e a posição. O ponto de parada fica
  verdadeiro, e o boot pode decidir com informação em vez de adivinhar.
- **Uma transação para a série inteira foi descartada** porque um erro no arquivo 20 desfaria os 19
  anteriores — trabalho que já estava correto — e apagaria o "até onde chegou", que é a informação mais
  cara do incidente.
- **`search_path` vazio dentro da transação** é deliberado: a série escreve nome qualificado
  (`padrao-banco.md` §1, P-01 e P-02, e §3), e `CREATE TABLE account` sem schema deve **falhar**, não
  cair por acidente num schema do caminho. O erro cedo é mais barato que o objeto no lugar errado.
- **Quem emite `BEGIN`/`COMMIT` é o adapter**, não a orquestração: a atomicidade é do contrato do
  `MigrationTxn`, e o runner só chama `run`/`record`/`commit` — é o que permite provar o fluxo contra
  portas falsas.

Contrato em `src/migrate/spec.md` §6, `src/migrate/port/unit_of_work.md` e `src/migrate/adapter/pg.md`.

### 2.4 Lock de sessão, com chave própria

Duas versões do binário não podem aplicar a série ao mesmo tempo, e o lock precisa sobreviver ao
trabalho inteiro — não a uma instrução.

- **Lock de sessão (`pg_advisory_lock`), não de transação**: a sessão atravessa uma transação por
  arquivo; o lock `xact` seria liberado entre arquivos e outro processo entraria no meio.
- **Chave própria, `0x5759_4404`**, e não a herdada (`0x5759_4401`): binário antigo e novo não devem
  serializar entre si durante a troca de série — eles não aplicam o mesmo formato de ledger, e
  compartilhar o lock daria a falsa impressão de que um espera pelo outro com segurança. O custo é
  aceito e registrado na ADR: durante a janela de troca, os dois não se enxergam.
- **Espera limitada**: `lock_timeout` configurável, padrão curto; estourou, o erro diz "outro processo
  está migrando; verifique `pg_locks`". Boot pendurado é pior que boot que falha com instrução.
- **Sem `unlock` na porta**: liberar é devolver a conexão dedicada. Expor `unlock` permitiria liberar
  no meio da série — exatamente a serialização que o lock existe para garantir.

Contrato em `src/migrate/spec.md` §5, `src/migrate/port/lock.md` e `ADR-DB-0002`, Decisão item 4.

### 2.5 `down` obrigatório

Um `up` sem `down` é uma promessa que ninguém testou. Exigir o `down` no mesmo arquivo tem três
efeitos, e nenhum é burocracia:

- **Prova o `up`**: escrever a reversão revela dependência escondida — objeto criado fora do arquivo,
  FK que aponta para o lado errado, seed que não volta. O `down` é o primeiro revisor do `up`.
- **Mantém a reversão viva**: separado num diretório de rollback, o `down` apodrece sem que ninguém
  perceba, porque nada o executa. No mesmo arquivo, o ciclo `up → down → up` é testável desde o
  primeiro dia.
- **Delimita a responsabilidade**: cada arquivo derruba o que criou, sem `DROP SCHEMA` global e sem
  `CASCADE`. Em produção o `down` não é o plano A — a página de implantação diz isso —, mas ele é o
  que torna o rollback de desenvolvimento e o reset (via `down` até vazio) executáveis.

Regra de série em `plano-migrations.md` §2; forma e recusa do parser em `src/migrate/spec.md` §2 e
`src/migrate/migration/file.md`.

## 3. O que o desenho recusa

| Recusa | Por quê |
|---|---|
| ledger com `CREATE TABLE IF NOT EXISTS` | esconde ledger de forma diferente em vez de recusá-lo |
| checksum só do bloco `up` | deixaria o `down` alterado passar depois de aplicado |
| transação única para a série | desfaz trabalho válido e apaga o ponto de parada |
| lock de transação, ou espera infinita | solta entre arquivos / pendura o boot |
| `down` opcional | reversão não testada e arquivo que ninguém consegue desfazer |
| `BEGIN`/`COMMIT` dentro do `.sql` | a série fecharia o que o runner abriu e a atomicidade deixaria de ser do contrato |
| `CREATE INDEX CONCURRENTLY`, `VACUUM`, `ALTER TYPE … ADD VALUE` | não rodam dentro de transação; a mensagem precisa dizer isso, não falhar por acidente |
| baseline por schema pré-existente | "já tem tabela, então já está aplicado" é como a série herdada marcou tudo sem executar |
| `ALTER TABLE` corretivo no `up` | série de instalação, não de evolução: desenho errado se conserta na origem |

As três primeiras linhas são contrato do runner (`src/migrate/spec.md` §3–§6); a de baseline e a de
`ALTER` são da série e estão em `plano-migrations.md` §2 e `ADR-DB-0002`, Contexto.

## 4. Invariantes que o desenho protege

1. **Arquivo aplicado é imutável** — mudança é arquivo novo, detectada por checksum do arquivo inteiro.
2. **Nada entra no schema por fora da série** — nem `ALTER` manual, nem script de operação.
3. **Aplicado é aplicado com linha** — schema e ledger não divergem, nem por falha no meio.
4. **Um processo aplica por vez** — lock de sessão com chave da série nova, espera limitada.
5. **Toda reversão é executável** — todo arquivo tem `down` que derruba o que o `up` criou.
6. **Boot falha alto** — papel errado, servidor errado, ledger de forma diferente ou drift param o
   boot com mensagem acionável, em vez de subir com schema divergente.

São `P`, como o resto da página: nenhum deles foi exercitado contra um PostgreSQL da 1.0.0. A forma
curta é esta; a regra é da série (`plano-migrations.md` §2) e o contrato de ledger, checksum, lock e
transação é de `src/migrate/spec.md` §3–§6 — esta lista não é dona de nenhum dos dois.

## 5. Onde o contrato mora

Esta página não é fonte de assinatura. Para implementar, a ordem é: desenho do módulo, página do
arquivo, ADR.

| Preciso de… | Leia |
|---|---|
| formato do `.sql`, marcadores, o que o parser recusa | `src/migrate/spec.md` §2 + `src/migrate/migration/file.md` |
| forma do ledger e leitura | `src/migrate/spec.md` §3 + `src/migrate/port/ledger.md` |
| checksum e normalização | `src/migrate/spec.md` §4 + `src/migrate/migration/checksum.md` |
| lock, chave e espera | `src/migrate/spec.md` §5 + `src/migrate/port/lock.md` |
| transação, `search_path` e commit | `src/migrate/spec.md` §6 + `src/migrate/port/unit_of_work.md` e `src/migrate/adapter/pg.md` |
| comandos, `down --to` e códigos de saída | `src/migrate/spec.md` §8 e §11 + `src/migrate/error.md` |
| a decisão estrutural e o que ela custa | [`ADR-DB-0002`](adr/adr-db-0002-runner-de-migrations-proprio.md) |
| numeração, ordem, seeds e gates da série | `plano-migrations.md` §2–§4 e §7 |

## 6. O que reabre a decisão

A decisão não é para sempre. Ela volta à mesa quando: houver **mais de uma série** no mesmo banco
(exige coluna de série no ledger e muda a chave do lock); uma **réplica de leitura** passar a receber
migrations (o lock é por cluster, não por nó); ou algum processo **deixar de chamar** o runner no boot
— o que reabre a pergunta de quem aplica a série. Os três critérios estão na seção *Revisão* da
[`ADR-DB-0002`](adr/adr-db-0002-runner-de-migrations-proprio.md).
