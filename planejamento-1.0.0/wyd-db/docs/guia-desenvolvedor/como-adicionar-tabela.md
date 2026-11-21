> Pergunta: "como modelo e adiciono uma tabela nova sem quebrar o padrão nem a série?"
> Público: Desenvolvedor novo
> Dono: equipe do `wyd-db`
> Base: `padrao-banco.md` §1–§6, §10–§11; `plano-migrations.md` §1–§4; `migrations-checklist.md` §5; `lacunas.md` §1 (`M7`, `M23`)
> Revisado em: 2026-11-18

**Estado:** proposto (2026-11-18)

Quatro níveis de afirmação convivem aqui:

- **`P`** — o procedimento descreve a série da `1.0.0`, que não foi escrita; nenhum comando desta
  página foi executado.
- **`H`** — ausência de decisão, dita na seção em que o assunto aparece.
- **`V`** — o estado do gate, verificável no documento de planejamento citado.
- **sem marca** — comportamento do PostgreSQL em si (o índice não cobre o lado filho da FK).

Esta página é o **procedimento**. A regra de cada escolha (que tipo, que índice, por quê) mora em
[`padroes-postgres.md`](../arquitetura/padroes-postgres.md) e em
[`integridade-e-constraints.md`](../arquitetura/integridade-e-constraints.md); o texto do DDL de cada
tabela mora na página de `migrations/` do arquivo que a cria. Aqui está a ordem em que se decide, para
não decidir duas vezes nem no lugar errado.

## 1. Checklist de modelagem

| # | Passo | O que fazer | Por quê | Regra em |
|---:|---|---|---|---|
| 1 | **Nome** | entidade no singular, `snake_case`, no schema do domínio; nada de prefixo com o nome do schema | o schema já diz a coleção; nome repetido gera `game.game_account` | `padrao-banco.md` §1 (`P-01`, `P-02`), §3 |
| 2 | **Chave** | PK identity nomeada; PK composta quando a chave natural já é a identidade da linha | `id` sintético em tabela filha paga índice redundante e permite linha duplicada | §1 (`P-03`, `P-04`) |
| 3 | **Nulidade** | `NOT NULL` por padrão; nulidade só com motivo escrito na nota da coluna | nulo sem motivo é regra de negócio escondida em três estados | §1 (`P-12`) |
| 4 | **CHECK** | aplicar a família que o caso exigir — faixa, conjunto, par estado × carimbo, forma de segredo | é o que impede o impossível inclusive em escrita manual e concorrente | quais famílias existem e quando cada uma é devida: [`integridade-e-constraints.md`](../arquitetura/integridade-e-constraints.md) §3; o predicado, na página de migration |
| 5 | **Índice** | um no lado filho de cada FK; parcial para "um vivo por chave"; nenhum preventivo | o Postgres não indexa o filho, e índice sem consulta só encarece a escrita | §6 (`P-14`, `P-15`) |
| 6 | **FK** | `ON DELETE` escolhido explicitamente: `RESTRICT` para dinheiro e auditoria, `CASCADE` só para filho possuído | cascade em dinheiro apaga a razão junto com a conta | §5, item 7 (`P-15`) |
| 7 | **Retenção** | declarar se a tabela cresce para sempre e qual é a política | crescimento sem política é descoberto tarde, no disco | §1 (`P-21`) |
| 8 | **Permissão** | conferir se o `ALTER DEFAULT PRIVILEGES` do schema já dá o efeito certo aos papéis | a fronteira de escrita é do banco, não da revisão de código | §8; [`seguranca.md`](../arquitetura/seguranca.md) |

> [!TIP] A ordem importa
> Chave antes de nulidade, nulidade antes de `CHECK`, `CHECK` antes de índice. Cada passo depende do
> anterior: um `CHECK` escrito antes da nulidade acaba com `COALESCE` dentro do predicado, que é o
> cheiro de que a nulidade foi decidida no lugar errado.

## 2. Coluna, tabela 1:1 ou tabela filha

| Situação | Forma | Precedente |
|---|---|---|
| o dado é obrigatório, escalar, e sempre existe junto com a linha | **coluna** | `padrao-banco.md` §1 (`P-04`) |
| é 1:1, tem **ciclo de vida próprio** (nasce, muda e morre por conta própria) | **tabela 1:1** | `M23` |
| é 1:1, mas a **base legal, o consentimento ou a auditoria** têm regra própria | **tabela 1:1 separada** | `M7`, `ddl-confirmado.md` §5.6 e §5.7 |
| cresce em cardinalidade (N por linha pai) | **tabela filha** com PK composta | `P-04` |
| existe só para "não misturar" dois assuntos que sempre andam juntos | **não criar** | `M23` |

Dois casos reais, em direções opostas, e a diferença entre eles é a pergunta certa:

- **`M23` — fundir.** Duas tabelas 1:1 com a tabela dona, sem ciclo de vida próprio e sem escrita
  independente, viraram colunas da tabela dona. Separar sem vida própria só paga junção e mais um
  ponto de divergência; o DDL resultante está em `ddl-confirmado.md` §5.5.
- **`M7` — separar.** Perfil e consentimento da pessoa ficaram em tabelas próprias mesmo sendo 1:1,
  porque a prova de consentimento precisa de versão do documento aceito e retenção própria — e PII
  mora só em `portal` (`padrao-banco.md` §1, `P-22`). O DDL está em `ddl-confirmado.md` §5.6 e §5.7, e
  o significado em [`dominio-acesso.md`](../dicionario-dados/dominio-acesso.md).

> [!WARNING] Sem soft-delete genérico
> `deleted_at` universal contamina todo `WHERE`, todo índice e toda FK. Estado é explícito: `status`
> com carimbo coerente, e `deleted_at` só onde o jogo exige (`padrao-banco.md` §1, `P-20`). Se a
> tabela nova "precisa" de soft-delete, a pergunta é outra: que estado ela tem?

## 3. Onde a tabela nasce

1. **No arquivo do domínio**, nunca em arquivo de outro assunto — um domínio por arquivo, e nenhuma
   tabela partida entre dois (`plano-migrations.md` §2).
2. **Sem `ALTER TABLE` no `up`.** Coluna nova nasce na criação da tabela; corrigir desenho é reescrever
   o arquivo (série nova) ou criar arquivo novo (série aplicada) (`plano-migrations.md` §2).
3. **Sem FK para tabela criada em arquivo posterior.** A ordem é de `plano-migrations.md` §3, e
   dependência invertida exige mudar a ordem, não forçar a FK (`plano-migrations.md` §2).
4. **Seed em arquivo próprio**, imediatamente depois do DDL do domínio — nunca dentro do arquivo que
   cria a tabela (`plano-migrations.md` §4).
5. **O `down` derruba o que o `up` criou**, no mesmo arquivo e na ordem inversa
   (`migrations-checklist.md` §5, item 5).

## 4. Como documentar a tabela nova

| O quê | Onde | Regra |
|---|---|---|
| por que a tabela existe, ciclo de vida e armadilhas | página de domínio do dicionário, seção "tabela de tabelas" | uma linha por tabela; o domínio é a porta de entrada (`wyd-db-1.0.0.md` §6.8) |
| coluna, tipo, nulidade, PK, FK, `CHECK` e índice | DDL e tabela de colunas da página de `migrations/` (dona); a página por tabela, escrita à mão (`M26`) | a forma não se digita em texto solto (`wyd-db-1.0.0.md` §6.8) |
| intenção e `down` do arquivo | página de `migrations/` | molde de 10 itens em `migrations-checklist.md` §5 |
| estado da tabela | derivado do estado do arquivo | `migrations-checklist.md` §1.2; a página de domínio **cita**, não digita (`D-C`) |

**Nunca** copie a forma para um segundo lugar: se a coluna muda, a cópia diverge e a divergência vira
defeito registrado — a migration prevalece e a origem é corrigida, não o texto (`wyd-db-1.0.0.md` §8.2).

## 5. Revisão exigida

Quem revisa confere, nesta ordem, e reprova no primeiro item que falhar:

1. a tabela pertence ao domínio daquele arquivo, e o nome segue `padrao-banco.md` §2–§3;
2. toda constraint está nomeada, com o prefixo do tipo de objeto (`P-13`);
3. nenhum tipo proibido — lista fechada em `padrao-banco.md` §11;
4. `ON DELETE` de cada FK é explícito e justificável (§5, item 7);
5. há índice no lado filho de toda FK e índice parcial onde a regra é "um vivo" (`P-14`, `P-15`);
6. toda `CHECK` nova tem teste que a viola e recebe o erro esperado (`padrao-banco.md` §10);
7. retenção declarada, ou a frase que diz por que a tabela não cresce (`P-21`);
8. a página de domínio ganhou a linha, e a página de migration foi escrita com o `down`
   (`migrations-checklist.md` §5).

Regra da casa: **constraint sem teste é comentário** (`padrao-banco.md` §10). Tabela que só o autor
sabe violar não está pronta.

## 6. Gate

O gate automático da série **não existe hoje** (`V` — `plano-migrations.md` §1): o PR de ferramentas
está adiado por decisão do dono, e `banco-parse`/`check-migrations` ainda não leem nome qualificado nem
`INET`. Enquanto isso, a conferência dos 8 itens de §5 é **manual**, e nenhum dos comandos abaixo foi
executado (`P`):

```bash
# planejado — não executado nesta árvore
node tools/content/check-migrations.mjs
make migrations-apply        # up → down → up
# catálogo e páginas por tabela não entram: ferramenta não aprovada e produção em `M26`
```

Quando o PR for autorizado, o critério de pronto dele é o de `plano-migrations.md` §1.3, e a partir daí
os três comandos viram gate de PR em `checklist-de-pr.md` (planejado).

## 7. O que esta página não decide

1. **Não decide a forma da tabela** — tipo, predicado e nome de constraint são de `ddl-confirmado.md`
   §3–§5 e da página de `migrations/` do arquivo dono.
2. **Não decide a ordem da série**: arquivo e posição são de `plano-migrations.md` §3.
3. **Não decide permissão.** O efeito por papel é de [`seguranca.md`](../arquitetura/seguranca.md) e o
   texto do artefato de papel é de `src/provision/spec.md` §4 (planejamento).
4. **Não decide retenção por tabela** — a política é de `plano-de-manutencao` (planejado) e a decisão
   de operação está aberta em `lacunas.md` §2.3 (`M22`).
5. **Não autoriza tabela nova fora da série.** Tabela que não está em `plano-migrations.md` §3 entra
   por decisão registrada, não por este procedimento (`migrations-checklist.md` §6).
