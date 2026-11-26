> Pergunta: "como a suíte prova a si mesma — DSN nomeada, nenhum `SKIP` e a matriz consistente?"
> Público: QA / testador
> Dono: equipe do `wyd-db`
> Base: `../specs/capacidades.md` §1 e §6; `../../docs/guia-teste/gates-e-anti-skip.md`; `../../docs/planos/plano-de-testes.md` §5
> Revisado em: 2026-11-24

**Estado:** aceito (2026-11-24) — alvo planejado; nenhum cenário executado.

**Quatro níveis de afirmação convivem aqui:**

- **`P`** — tudo o que a página diz sobre o alvo: `sistema_stack_caps` não existe em `Cargo.toml` nem em
  `tests/`, e nenhum dos cenários rodou.
- **`H`** — dúvida aberta, dita e localizada em §7, com o dono que a fecha.
- **`V`** — o que foi medido nesta árvore em 2026-11-24, com o arquivo ao lado.
- **sem marca** — regra de projeto (`../../docs/convencoes.md`) e comportamento do PostgreSQL
  (`../../padrao-banco.md` §13).

**Alvo escrito:** `sistema_stack_caps` — arquivo `crates/wyd-db/tests/sistema_stack_caps.rs`, ainda fora
do `Cargo.toml`; o nome do alvo é o de [../index.md](../index.md) §2.

Este alvo não prova o banco: prova a **suíte**. Duas capacidades — o ambiente que falha nomeando a
variável e a ausência de `SKIP` — mais a conferência de que a matriz de capacidades continua ligada a
alvos e cenários.

## 1. Capacidades cobertas

| `CAP` | Capacidade | Cenário |
|---|---|---|
| `CAP-22` | DSN ausente falha nomeando a variável, e nenhum binário lê `DATABASE_URL` | `dsn_nomeada` |
| `CAP-23` | nenhum `SKIP`/`#[ignore]` no caminho de aceite | `anti_skip` |

## 2. Cenários

| Cenário | Entrada | Resultado esperado | Falha esperada |
|---|---|---|---|
| `dsn_nomeada` | alvo de sistema com o ambiente limpo | saída de erro citando `WYD_DB_URL_TEST`; código de saída diferente de zero | saída neutra, "skipped" ou exit 0 |
| `anti_skip` | varredura do código de teste do caminho de aceite | zero `#[ignore]`, zero `SKIP`, zero filtro silencioso | ocorrência nomeada com arquivo e linha |
| `env_sem_database_url` | varredura do crate e dos binários | nenhuma leitura de `DATABASE_URL` | leitura encontrada, com arquivo e linha |

Os dois primeiros são `CAP-22` e `CAP-23`; o terceiro é o complemento estático do mesmo gate `ambiente`
e roda sem banco.

## 3. Conferência da matriz

| Regra | O que o alvo verifica | Se falhar |
|---|---|---|
| toda `CAP-nn` tem cenário | cada linha de `../specs/capacidades.md` aparece em `alvos/` | capacidade órfã: ou ganha cenário, ou sai da matriz |
| todo alvo cobre ao menos uma `CAP` | as cinco páginas de `alvos/` citam `CAP-nn` | alvo sem propósito |
| todo cenário cita `CAP-nn` | nome do cenário aparece na matriz | teste sem capacidade |
| `CAP` não reutilizado | soma das linhas de `§2`–`§6` = **23** | numeração duplicada |

A conferência é do **planejamento**, não do banco: ela pode rodar como teste de leitura de texto, sem
conexão, e é o que impede a matriz de envelhecer calada.

## 4. Pré-requisito de ambiente

| Item | Regra |
|---|---|
| para `dsn_nomeada` | um alvo de sistema que falhe sem DSN; ambiente sem `WYD_DB_URL_TEST` |
| para `anti_skip` e `env_sem_database_url` | só o código de teste e o crate; sem banco |
| para a conferência da matriz | as páginas de `../specs/` e `../alvos/` legíveis como texto |

## 5. Como interpretar a falha

- **`skipped` em vez de erro:** o gate anti-skip está frouxo; corrigir o alvo, não o teste.
- **Exit 0 sem rodar:** mesmo defeito, no nível do runner.
- **`DATABASE_URL` lida em algum lugar:** quebra `RNF-DB-013` (gate `ambiente`) e vaza
  configuração para fora da DSN por papel.
- **Matriz com `CAP` órfã:** capacidade sem alvo — o gate de §1 de
  `../../docs/guia-teste/gates-e-anti-skip.md` reprova por desenho.

## 6. Tempo esperado

**Não há tempo medido**; os cenários estáticos são baratos e os de ambiente dependem de subir um alvo sem
DSN. Nenhum número entra aqui sem execução registrada.

## 7. Dúvidas abertas

| Dúvida | O que a resolve | Dono |
|---|---|---|
| onde a cobertura de 85% (`RNF-DB-001`) é medida? | não é alvo Rust: o gate de cobertura é do CI, que hoje é stub (`../../docs/planos/plano-de-qualidade-e-gates.md`) | `wyd-ops` |
| a conferência da matriz roda como teste deste alvo ou como script de gate? | a página diz que pode rodar como leitura de texto, sem banco; a forma final não está decidida | autor do alvo |

## 8. O que este alvo não prova

- **Comportamento do banco:** `sistema_pg` e `sistema_sessoes`.
- **Cobertura por linha:** a regra é por capacidade, e percentual não substitui.
- **CI:** não existe página de CI em `docs/planos/` do `wyd-db`; o plano do projeto é
  `../../../wyd-ops/docs/devops/index.md` (`aceito`, 2026-11-26), e `docs/planos/plano-de-testes.md` §81
  anuncia a página local. Aqui o comando local.
- **Que a matriz esteja correta:** ela prova que está **ligada**; o acerto de cada capacidade é revisão de
  quem escreve o cenário.
