> Pergunta: "quando uma mudança no `wyd-db` é aceita?"
> Público: Desenvolvedor, revisor e operação
> Dono: equipe do `wyd-db`
> Base: `padrao-banco.md` §10; `plano-migrations.md` §1.3, §7 e §9; `migrations-checklist.md` §5; `wyd-db-1.0.0.md` §6.11
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

Um gate só vale se **reprova** alguma coisa. A regra da casa é curta: *constraint sem teste é
comentário* (`padrao-banco.md` §10). O aceite da `1.0.0` é a soma de oito gates de série, dez itens
obrigatórios de página de migration, o contrato e o sistema, a fronteira entre papéis e a revisão de
PR. Alguns têm comando; os que ainda não têm são conferência manual declarada, nunca “verde” por
ausência.

## 1. Gates de série (oito)

| Gate | O que prova | Comando ou prova | Reprova quando |
|---|---|---|---|
| estrutura | 34 arquivos contíguos; **0 tabela** ao fim do `down`, somando os 4 schemas | `psql` no PG17 efêmero: aplicar, `down` de todos, contar `pg_tables`/views | falta arquivo, há lacuna, sobra tabela ou view |
| catálogo | 109 tabelas + 2 views no documento gerado, com schema, nota e fonte | `node tools/content/check-migrations.mjs` (previsto) e `contrato_catalogo` | tabela ausente, sem schema ou sem nota |
| tipos | nenhum `serial`, `timestamp` sem fuso, `money`, `varchar` ou `enum` | conferência manual hoje; o gate automático é do PR de ferramentas | qualquer tipo proibido em coluna |
| constraints | 100% nomeadas; toda constraint tem teste que a viola e recebe o erro esperado | `contrato_migrations` + `sistema_pg` | constraint anônima ou invariante sem teste |
| fronteira | `permission denied` para portal escrevendo em `game`; jogo não lê `portal.person` cru | `sistema_sessoes` e `psql` com a DSN de cada papel | a escrita passa ou a leitura crua funciona |
| conteúdo | contagens de `plano-migrations.md` §4 idênticas às medidas; `content_version = 7` | consultas de contagem por dataset | qualquer contagem diverge ou a versão não fecha |
| vertical | o corte de `plano-migrations.md` §3.2 passa com o cliente 7662: login, PIN, seleção, 5 contas | execução manual com o cliente e o PG17 efêmero | qualquer passo falha |
| ambiente | DSN ausente falha **nomeando a variável**; nenhum binário lê `DATABASE_URL` | executar sem a variável e conferir a mensagem; `rg DATABASE_URL` | erro genérico, default silencioso ou leitura de `DATABASE_URL` |

Os oito são de `plano-migrations.md` §7 e valem para a série inteira, não para um arquivo isolado.

## 2. Página de migration (dez itens)

Toda página de `migrations/` só fecha com os **dez itens** de `migrations-checklist.md` §5:
cabeçalho de cinco campos; identificação e dependências; objetivo em uma frase; DDL do `up` completo;
DDL do `down` na ordem inversa; tabela de colunas; contratos e invariantes com o código de erro;
permissões com o efeito por papel; prova de execução com `psql`; e o que o arquivo não decide. Falta
um item, a linha fica `bloqueada` — não há “quase pronta”.

## 3. Contrato e sistema

| Camada | Prova | Alvo previsto | Reprova quando |
|---|---|---|---|
| contrato | série contígua, `down` derruba o que o `up` criou, nenhuma FK para arquivo posterior, seed só em tabela criada, constraint nomeada, tipo proibido | `contrato_migrations` | qualquer invariante estrutural quebra |
| contrato de catálogo | documento gerado ↔ migrations, com schema, view, função e trigger | `contrato_catalogo` | documento e série divergem |
| sistema | `migrate-cycle` up → down → up com contagens idênticas e 0 tabela; `verify-schema` por schema; cada `CHECK`/único violado com o erro esperado | `sistema_pg` | ciclo sujo, contagem diferente ou violação não reprovada |
| fronteira e sessão | privilégio por papel e sessão única por conta | `sistema_sessoes` | escrita cruzada passa ou a segunda sessão é aceita |

Os cinco alvos são os de `wyd-db-1.0.0.md` §6.17. **Nenhum deles existe hoje**: a suíte é `P` e o
leitor atual do catálogo não entende `CREATE TABLE game.account (` — enquanto isso, o contrato da
série nova é conferência manual (`plano-migrations.md` §1.3).

## 4. Documentação e PR

| Área | O que é aceito | Prova | Reprova quando |
|---|---|---|---|
| documentação | link e âncora resolvem; build do site passa | `make docs-check` (previsto) e o build do site no CI | link quebrado, âncora inexistente, build falha |
| página | cabeçalho, `Base:` com fonte, número com data, estado no cabeçalho | revisão contra `docs/convencoes.md` | fonte ausente, número sem dono, placeholder |
| PR | fmt, lint, teste, catálogo regenerado, issue vinculada, doc atualizada, sem segredo, sem `unwrap` novo em produção | revisão do checklist de `wyd-db-1.0.0.md` §6.11 | qualquer item aberto ou `unwrap` novo sem justificativa |

## 5. Ordem e o que não é gate hoje

A ordem é a de `plano-migrations.md` §1: `provision render` → `provision apply` → `0001`…`0034` →
catálogo → gates. O **corte vertical** de `0001`–`0007` é o primeiro gate que autoriza continuar; os
demais não substituem esse.

O que **não** é gate hoje, e a página não finge que é:

- o PR de ferramentas está em espera; `check-migrations` tem a contagem antiga cravada
  (`53 | 87`) e reprova a série nova por contagem, não por defeito;
- `banco-parse` não lê schema qualificado, view nem `INET`, então o catálogo da série nova não é
  gerado por ele;
- o CI de código é stub: verde de CI não prova build nem teste (`riscos.md` `R-3`);
- a suíte de contrato/sistema não tem alvo no `Cargo.toml` — os nomes são desenho.

Enquanto isso, os gates de §1 são **conferência manual** registrada na revisão; quem assina a
conferência responde por ela.

## 6. Exceção e quem aprova

Não existe exceção silenciosa. Um gate que não pode rodar é **falha declarada com o motivo**, não
resultado neutro. Abrir exceção exige decisão do dono do `wyd-db`, com data, motivo e o que a
substitui até o gate voltar; a decisão entra no registro de risco ou na issue que a motivou. Sem esse
registro, o gate reprova.

## 7. O que esta página não cobre

- **Cobertura funcional:** a matriz `CAP-nn` está em `tests/specs/capacidades.md` (**23** capacidades,
  **0** cobertas); a estratégia é de `plano-de-testes.md`.
- **Desempenho e carga:** não há medição nem limiar decidido.
- **Backup, restore e retenção executada:** são de
  [`plano-de-implantacao.md`](plano-de-implantacao.md) e da operação.
- **Risco:** a exposição e o antídoto ficam em [`riscos.md`](riscos.md); esta página só diz quando
  uma mudança é aceita.
