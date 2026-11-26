> Pergunta: "que capacidade observável do banco existe, qual alvo a prova e qual requisito ela atende?"
> Público: QA / testador
> Dono: equipe do `wyd-db`
> Base: `../../docs/planos/plano-de-testes.md` §2–§5; `../../plano-migrations.md` §7; `../../docs/specs/rastreabilidade.md`; `../../docs/guia-teste/gates-e-anti-skip.md` §3
> Revisado em: 2026-11-25

**Estado:** aceito (2026-11-24) — matriz escrita; **14 de 39** capacidades cobertas por teste executado (2026-11-25).

**Quatro níveis de afirmação convivem aqui:**

- **`P`** — tudo o que a página diz sobre a suíte: os 39 `CAP` são matriz, cinco alvos estão declarados
  em `crates/wyd-db/Cargo.toml` (`sistema_pg`, `reversibilidade_pg` e `runner_pg` entre eles) e 14 dos 39
  cenários rodaram.
- **`H`** — dúvida aberta, dita e localizada em §9, com o dono que a fecha.
- **`V`** — o que foi medido nesta árvore em 2026-11-24, com a fonte ao lado.
- **sem marca** — regra de projeto (`../../docs/convencoes.md`) e comportamento do PostgreSQL
  (`../../padrao-banco.md` §13).

Esta é a matriz `CAP-nn → alvo → cenário → requisito` que o plano de testes declarava ausente. Ela é o
**dono único** da capacidade de teste: os alvos de `../alvos/` citam os IDs daqui, e
`../../docs/specs/rastreabilidade.md` continua dono do requisito. **39 capacidades**, **7 alvos**, **14**
executadas.

## 1. Como ler

| Coluna | O que é | Dono |
|---|---|---|
| `CAP-nn` | propriedade observável do banco, com resultado esperado | esta página |
| alvo | o `Cargo.toml` de teste que a executa | `../index.md` §2 |
| cenário | caso nomeado dentro do alvo, com entrada e resultado | `../alvos/` |
| requisito | `RF`/`RN`/`RNF-DB` que a capacidade atende; a propriedade da série é enunciada por `RNF-DB-008` a `RNF-DB-013`, um por gate de `../../docs/planos/plano-de-qualidade-e-gates.md` §1 | `../../docs/specs/` (texto) e `../../docs/planos/plano-de-qualidade-e-gates.md` §1 (prova) |
| **onde é provada** | a **página** que descreve o cenário e o alvo escrito que o executa | coluna desta matriz |

**Duas regras, e nenhuma delas é preferência:**

1. **Capacidade sem cenário é falha**, não pendência: o gate de §2 de
   `../../docs/guia-teste/gates-e-anti-skip.md` reprova.
2. **Cenário sem capacidade é falha** também: teste que não prova `CAP-nn` não entra no caminho de
   aceite (`RNF-DB-003`).

**O que "coberto" quer dizer:** capacidade com cenário **executado e verde**. Como `sistema_pg` rodou
contra a base migrada e `runner_pg` rodou contra PG17 em 2026-11-25, esta página declara **14 de 39
cobertas** — e as outras 25 seguem planejadas, sem trocar "planejado" por "coberto".

## 2. Contrato — `contrato_migrations` (7)

| CAP | Capacidade observável | Cenário | Requisito ou gate | Onde é provada |
|---|---|---|---|---|
| `CAP-01` | a série tem 34 arquivos, `0001`–`0034`, sem furo nem repetição | `serie_contigua` | `RNF-DB-008` | `alvos/contrato-migrations.md` · `contrato_migrations` |
| `CAP-02` | o `down` de cada arquivo derruba exatamente o que o seu `up` criou | `down_completo` | `RNF-DB-008` | `alvos/contrato-migrations.md` · `contrato_migrations` |
| `CAP-03` | nenhuma `REFERENCES` aponta para tabela criada em arquivo posterior | `fk_sem_futuro` | `RNF-DB-008` | `alvos/contrato-migrations.md` · `contrato_migrations` |
| `CAP-04` | `INSERT` de seed só aparece em arquivo posterior ao `CREATE TABLE` da tabela | `seed_depois_da_tabela` | `RNF-DB-008` | `alvos/contrato-migrations.md` · `contrato_migrations` |
| `CAP-05` | todo `INSERT` de catálogo vive em arquivo cujo nome tem `_seed_` | `seed_em_arquivo_proprio` | `RNF-DB-008` | `alvos/contrato-migrations.md` · `contrato_migrations` |
| `CAP-06` | toda `PRIMARY KEY`, `UNIQUE`, `CHECK` e `FOREIGN KEY` tem `CONSTRAINT nome` | `constraint_nomeada` | `RNF-DB-009` | `alvos/contrato-migrations.md` · `contrato_migrations` |
| `CAP-07` | nenhum `serial`, `timestamp` sem fuso, `time`, `money`, `varchar` ou `enum` na série | `tipo_permitido` | `RNF-DB-010` | `alvos/contrato-migrations.md` · `contrato_migrations` |

## 3. Contrato — `contrato_catalogo` (1)

| CAP | Capacidade observável | Cenário | Requisito ou gate | Onde é provada |
|---|---|---|---|---|
| `CAP-08` | o documento de catálogo e a série descrevem as mesmas **109** tabelas e **2** views, com schema | `catalogo_igual_a_serie` | `RNF-DB-011` | `alvos/contrato-catalogo.md` · `contrato_catalogo` (bloqueada: catálogo ausente, §7.1) |

## 4. Sistema — `sistema_pg` (27)

| CAP | Capacidade observável | Cenário | Requisito ou gate | Onde é provada |
|---|---|---|---|---|
| `CAP-09` | aplicar a série, reaplicar sem efeito e detectar drift no ledger | `apply_reapply_drift` | `RNF-DB-008` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-10` | vaga de conta é 1..5 e a fechada é reutilizada | `vaga_de_conta` | `RN-DB-001` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-11` | uma sessão viva por conta e por personagem | `sessao_unica` | `RF-DB-006`, `RN-DB-002` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-12` | saldo e moeda recusam valor fora do inteiro de 32 bits | `moeda_int32` | `RF-DB-003`, `RN-DB-005` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-13` | fechar pessoa anonimiza o e-mail e mantém a unicidade | `anonimizacao` | `RN-DB-004` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-14` | cinco contas por pessoa **sem** barreira por pessoa ou IP | `cinco_contas_sem_barreira` | `RF-DB-014`, `RN-DB-003` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-15` | grant pendente impede apagar conta e personagem | `grant_pendente_impede` | `RF-DB-009` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-16` | um resgate diário por conta e dia | `resgate_diario` | `RF-DB-005` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-17` | quatro processos concorrentes terminam sem deadlock | `concorrencia_4` | `RNF-DB-006` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-18` | a versão de conteúdo admite **uma** linha corrente e recusa a segunda | `boot_uma_versao` | `RF-DB-017` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-19` | o pool tem espera finita e reuso observável | `pool_espera_reuso` | `RNF-DB-007` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-24` | login de conta aceita 1 a 16 caracteres e `login_fold` dobra caixa e borda | `login_tamanho_e_fold` | `RF-DB-001` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-25` | contador de falha de PIN e instante de bloqueio ficam na linha da conta e sobrevivem a conexão nova | `pin_falha_persistida` | `RF-DB-002` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-26` | cofre é da conta: FK recusa conta inexistente e apagar personagem não apaga o cofre | `cofre_da_conta` | `RF-DB-004` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-27` | bloqueio exige motivo não vazio, autor e instante, os três por `CHECK` | `bloqueio_com_motivo` | `RF-DB-007` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-28` | chave de idempotência repetida no donate não duplica o lançamento, e o valor é positivo | `donate_idempotente` | `RF-DB-008` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-29` | token de reset só entra como sha256 hexadecimal de 64 caracteres | `token_como_hash` | `RF-DB-010` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-30` | papel de jogo é `player`, `moderator` ou `admin`, e nada mais | `papel_de_jogo_fechado` | `RF-DB-012` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-31` | senha de portal, senha de jogo e PIN só entram com o prefixo `$argon2id$` | `hash_argon2id` | `RF-DB-013` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-32` | bloquear conta não muda `game.account.status`, e `suspended` não existe no schema | `bloqueio_nao_muda_status` | `RN-DB-006` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-33` | o limiar de PIN não vive no schema: contador negativo é recusado e seis falhas passam | `limiar_de_pin_e_politica` | `RN-DB-007` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-34` | autenticar no portal deixa sessões revogáveis em `portal.person_session` e tentativa em `ops.login_attempt` | `autenticar_portal` | `UC-DB-001` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-35` | criar conta na vaga grava conta e carteira, e a sexta vaga é recusada pelo índice único | `criar_conta_na_vaga` | `UC-DB-002` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-36` | abrir sessão de jogo grava `character_id` nulo e recusa canal inexistente por FK | `abrir_sessao_de_jogo` | `UC-DB-003` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-37` | a segunda execução da série espera no advisory lock no máximo **10 s** e falha nomeando a espera | `cap37_segunda_execucao_espera_no_advisory_lock` | `RNF-DB-002` | `alvos/runner-pg.md` · `runner_pg` (verde em 2026-11-25) |
| `CAP-38` | o p95 da leitura de personagem é medido e registrado, sem limiar afirmado | `p95_leitura_personagem` | `RNF-DB-004` | `alvos/sistema-pg.md` · `sistema_pg` |
| `CAP-39` | o tempo de boot até a primeira conexão aceita é medido e registrado, sem prazo afirmado | `tempo_de_boot` | `RNF-DB-005` | `alvos/sistema-pg.md` · `sistema_pg` |

**O mapeamento de `CAP-17`–`CAP-19` foi fixado em 2026-11-24** por `Q-BLK-24` = A
(`../../pendencias-de-fechamento.md` §6.6.26): cada capacidade aponta o requisito que a enuncia —
`RNF-DB-006` (concorrência de escrita), `RF-DB-017` (versão única de conteúdo) e `RNF-DB-007` (espera por
conexão e reuso). Os três `RNF` que estavam aqui antes (`002`, `004`, `005`) medem outra coisa — espera de
DDL, p95 e tempo de boot — e ficaram sem capacidade naquele dia; voltaram em 2026-11-24 como `CAP-37`–`CAP-39`.
A entrada de `CAP-18` também mudou: `0001` fecha `content_version` em uma linha, então o cenário recusa
`id = 2` em vez de montar duas correntes.

**As 16 capacidades de `CAP-24` a `CAP-39` nasceram em 2026-11-24** com `Q-BLK-23` = A: eram as **16** linhas
da rastreabilidade com critério de aceitação na página dona e nenhum cenário escrito. Cada uma cita o
requisito da própria linha e nenhuma enuncia regra nova. `CAP-37` é a espera de DDL com limiar **10 s**
(`RNF-DB-002`); `CAP-38` e `CAP-39` medem p95 e tempo de boot e **não** fixam teto, porque limiar de latência
e de boot não existe (`../../docs/specs/requisitos-nao-funcionais.md` §4–§5).

## 5. Sistema — `sistema_sessoes` (2)

| CAP | Capacidade observável | Cenário | Requisito ou gate | Onde é provada |
|---|---|---|---|---|
| `CAP-20` | portal recebe `permission denied` ao escrever em `game` | `fronteira_por_papel` | `RNF-DB-012` | `alvos/sistema-sessoes.md` · `sistema_sessoes` |
| `CAP-21` | leitura cruzada só por view, e auditoria recusa `UPDATE`/`DELETE` | `leitura_por_view` | `RF-DB-011` + gate `fronteira` | `alvos/sistema-sessoes.md` · `sistema_sessoes` |

## 6. A suíte sobre a suíte — `sistema_stack_caps` (2)

| CAP | Capacidade observável | Cenário | Requisito ou gate | Onde é provada |
|---|---|---|---|---|
| `CAP-22` | DSN ausente falha nomeando a variável e nenhum binário lê `DATABASE_URL` | `dsn_nomeada` | `RNF-DB-013` | `alvos/sistema-stack-caps.md` · `sistema_stack_caps` |
| `CAP-23` | nenhum `SKIP` ou `#[ignore]` no caminho de aceite | `anti_skip` | `RNF-DB-003` | `alvos/sistema-stack-caps.md` · `sistema_stack_caps` |

## 7. Censo e dívida

**Medido em 2026-11-24**, com a fonte de cada linha:

| Medida | Número | Fonte |
|---|---:|---|
| capacidades na matriz | **39** | §2–§6 |
| cobertas por teste executado | **13** | `crates/wyd-db/tests/sistema_pg.rs`: 13 verdes em 2026-11-24 |
| com requisito nomeado (`RF`/`RN`/`RNF`) | **39** | `CAP-01`–`CAP-39` |
| governadas por gate solto | **0** | as 11 que citavam gate passaram a citar `RNF-DB-008`–`013` (`Q-BLK-19` = A) |
| alvos | **5** | `../index.md` §2 |
| domínio sem capacidade no banco | `RF-DB-015` e `RF-DB-016` | são regra de `wyd-server`, não do banco (`../../docs/specs/rastreabilidade.md` §2.1) |

**A dívida de gate solto foi fechada em 2026-11-24** (`Q-BLK-19` = A): as 11 capacidades que provavam
propriedade da série sem requisito numerado passaram a citar `RNF-DB-008` a `RNF-DB-013`, um por gate.
Nenhuma capacidade ficou sem requisito, e nenhum limiar novo foi inventado — o limiar é o do gate.

### 7.1 Capacidade com lugar escrito — e a que não tem ambiente

**Medido em 2026-11-24.** Nenhuma das 39 ficou sem lugar: cada linha de §2–§6 nomeia a **página** que
descreve o cenário e o **alvo** que o executa. O que uma capacidade pode não ter é **ambiente para rodar**, e isso é declarado,
não silenciado.

| Situação | Quantas | Quais |
|---|---:|---|
| com página e alvo escritos | **39** | todas: `CAP-01`–`CAP-07` em `alvos/contrato-migrations.md`; `CAP-08` em `alvos/contrato-catalogo.md`; `CAP-09`–`CAP-19` e `CAP-24`–`CAP-39` em `alvos/sistema-pg.md`; `CAP-20`–`CAP-21` em `alvos/sistema-sessoes.md`; `CAP-22`–`CAP-23` em `alvos/sistema-stack-caps.md` |
| com alvo escrito e **sem ambiente hoje** | **1** | `CAP-08`: o documento `docs/dicionario-dados/banco-catalogo.md` não existe e o leitor atual não enxerga `CREATE TABLE esquema.tabela` (`../alvos/contrato-catalogo.md` §3); é **declarada não executável na `1.0.0`** por `Q-BLK-20` = A (2026-11-24) |
| com cenário executado e verde | **14** | `CAP-24`–`CAP-36` em `sistema_pg` (13 verdes em 2026-11-24) e `CAP-37` em `runner_pg` (verde em 2026-11-25) |
| com cenário escrito e **sem código que o execute** | **2** | `CAP-38` (`RNF-DB-004`, consulta da porta C4) e `CAP-39` (`RNF-DB-005`, boot do binário) |

**O que isso não muda nas outras 25:** a cobertura é **14 de 39**. Página e alvo escritos são o
*onde*, não o *resultado*; declarar uma capacidade coberta sem cenário executado é o defeito que a regra 2
de §1 existe para impedir. As 13 verdes de `sistema_pg` mediram o **schema** — o harness chama `psql` no
container (`WYD_DB_TEST_CONTAINER`); `CAP-37` mediu a **porta** pelo `PgAdapter` real em `runner_pg`.
`CAP-38` e `CAP-39` seguem sem código que as execute.

**O aceite é a outra metade:** as 12 páginas foram aceitas em 2026-11-24 (`Q-BLK-21` = A), e o código dos
cinco alvos foi liberado por `Q-BLK-18` = A (2026-11-24); `crates/wyd-db/` tem `Cargo.toml`, `src/dsn.rs`,
`tests/sistema_pg.rs`, `tests/reversibilidade_pg.rs` e `tests/runner_pg.rs` — **3 dos 7 alvos** declarados,
e a matriz está **14 de 39** cobertas.

## 8. Como uma capacidade entra ou muda

1. a propriedade é observável e tem resultado esperado verificável;
2. ganha `CAP-nn` novo — número não se reutiliza, nem depois de retirada;
3. ganha cenário em um alvo e a linha em `../alvos/` daquele alvo;
4. aponta requisito (`RF`/`RN`/`RNF`) — propriedade da série usa `RNF-DB-008`–`013`, não o nome do gate;
5. só é declarada coberta quando o cenário roda e passa, com a evidência registrada na issue.

## 9. Dúvidas abertas

| Dúvida | O que a resolve | Dono |
|---|---|---|
| o dicionário `banco-catalogo.md` entra na `1.0.0`? | **decidida em 2026-11-24 (`A`)**: pós-`1.0.0`; `CAP-08` fica **declarada não executável na `1.0.0`**, com o gate `catálogo` como conferência manual até o PR de ferramentas | dono do `wyd-db` |
| quando `CAP-38` e `CAP-39` rodam? | código que ainda não existe: consulta da porta C4 (`RNF-DB-004`) e boot do binário (`RNF-DB-005`) | dono do `wyd-db` |

## 10. O que esta página não decide

- **Não executa teste:** capacidade verde é a que rodou; aqui só existe a matriz.
- **Não enuncia requisito:** `RF`/`RN`/`RNF` são das páginas de `../../docs/specs/`; esta matriz cita.
- **Não define limiar de cobertura:** a regra é por capacidade, e percentual de linha não substitui
  (`../../docs/guia-teste/gates-e-anti-skip.md` §4).
- **Não cria alvo novo:** alvo é do `Cargo.toml` do crate e da página de `../alvos/`.
