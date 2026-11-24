> Pergunta: "o que o alvo `sistema_sessoes` prova sobre papel, fronteira e leitura por view?"
> Público: QA / testador
> Dono: equipe do `wyd-db`
> Base: `../specs/capacidades.md` §5; `../specs/sistema.md`; `../../docs/arquitetura/seguranca.md`; `../../plano-migrations.md` §7 (gate `fronteira`)
> Revisado em: 2026-11-24

**Estado:** aceito (2026-11-24) — alvo planejado; nenhum cenário executado.

**Quatro níveis de afirmação convivem aqui:**

- **`P`** — tudo o que a página diz sobre o alvo: `sistema_sessoes` não existe em `Cargo.toml` nem em
  `tests/`, e nenhum dos cenários rodou.
- **`H`** — dúvida aberta, dita e localizada em §6, com o dono que a fecha.
- **`V`** — o que foi medido nesta árvore em 2026-11-24, com o arquivo ao lado.
- **sem marca** — regra de projeto (`../../docs/convencoes.md`) e comportamento do PostgreSQL
  (`../../padrao-banco.md` §13).

**Alvo escrito:** `sistema_sessoes` — arquivo `crates/wyd-db/tests/sistema_sessoes.rs`, ainda fora do
`Cargo.toml`; o nome do alvo é o de [../index.md](../index.md) §2.

Prova o que depende de **privilégio de papel**: portal não escreve em `game`, jogo não lê `portal.person`
direto, leitura cruzada só por view e auditoria sem `UPDATE`/`DELETE`. É o alvo que transforma o desenho
de `seguranca.md` em recusa observada.

## 1. Capacidades cobertas

| `CAP` | Capacidade | Cenário |
|---|---|---|
| `CAP-20` | portal recebe `permission denied` ao escrever em `game` (e o jogo ao ler `portal.person` direto) | `fronteira_por_papel` |
| `CAP-21` | leitura cruzada só por view; auditoria recusa `UPDATE`/`DELETE` | `leitura_por_view` |

## 2. Cenários

| Cenário | Entrada | Resultado esperado | Falha esperada |
|---|---|---|---|
| `fronteira_por_papel` | `WYD_DB_URL_PORTAL` → `INSERT` em `game.account`; `WYD_DB_URL_GAME` → `SELECT` em `portal.person` | as duas operações recusadas | `permission denied for table` |
| `leitura_por_view` | `WYD_DB_URL_GAME` → `SELECT` na view de leitura cruzada | leitura permitida, com as colunas expostas pela view | `permission denied` na view, ou coluna a mais |

| Cenário (cont.) | Entrada | Resultado esperado | Falha esperada |
|---|---|---|---|
| `auditoria_append_only` | `WYD_DB_URL_MIGRATOR` e `WYD_DB_URL_ADMIN` → `UPDATE`/`DELETE` em `ops.audit_log` | `UPDATE`/`DELETE` recusados | `permission denied` ou ausência de privilégio |

A recusa é o **resultado esperado**, não um acidente do teste: quem espera sucesso é o cenário do papel
que tem o privilégio, e ele existe em `sistema_pg` (`CAP-09`).

## 3. Pré-requisito de ambiente

| Item | Regra |
|---|---|
| banco | PostgreSQL 17 descartável, provisionado e com a série aplicada |
| papéis | os **seis** de `../../docs/arquitetura/seguranca.md` §"Papéis do banco" — `wyd_owner`, `wyd_migrator`, `wyd_game`, `wyd_portal`, `wyd_admin`, `wyd_read` — existem; `provision apply` os cria ou ajusta (`../../docs/arquitetura/implantacao.md` §1, passo 2). Sem eles o cenário falha dizendo qual falta |
| DSN | a do **papel testado** em cada passo — usar `WYD_DB_URL_TEST` aqui esconde a fronteira |
| view | a de leitura cruzada existe em `0029-views.md`, aplicada sem `GRANT` novo |
| auditoria | `ops.audit_log` de `0002-ops-audit.md` com os privilégios do provisionamento e de `0032-admin-grants.md` |

## 4. Como interpretar a falha

- **`permission denied` onde se esperava sucesso:** `GRANT` faltando — defeito do provisionamento ou de
  `0032-admin-grants.md`.
- **Sucesso onde se esperava recusa:** privilégio a mais; é falha de segurança, não de teste.
- **View inexistente:** `0029-views.md` não aplicada, ou o nome mudou sem atualizar a página da view.
- **`UPDATE` na auditoria aceito:** privilégio de escrita sobrando — o cenário falha mesmo que o banco
  "esteja funcionando".

## 5. Tempo esperado

**Não há tempo medido**; o alvo não roda. Tempo entra quando houver execução registrada, com data.

## 6. Dúvidas abertas

| Dúvida | O que a resolve | Dono |
|---|---|---|
| quem concede o privilégio de auditoria: o provisionamento ou `0032-admin-grants.md`? | o cenário precisa saber qual concessão revogar para provar a recusa; a página lista os dois | autor do alvo |
| qual coluna a view de leitura cruzada expõe? | `0029-views.md` fixa a view; a lista de colunas não está copiada aqui | `0029-views.md` |

## 7. O que este alvo não prova

- **Que a escrita legítima funciona:** `sistema_pg` (`CAP-09`–`CAP-19` e `CAP-24`–`CAP-39`).
- **Forma da série e catálogo:** `contrato_migrations` e `contrato_catalogo`.
- **Anti-skip:** `sistema_stack_caps`.
- **Segredo em log:** é regra de código e de `../../docs/guia-administrador/configuracao.md` §4, não de
  teste de banco.
