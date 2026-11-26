# Primeira fatia de produção aceita — `WYDDB-001` + `WYDNET-001`

> Pergunta: "o que a primeira fatia de código entrega, como provamos que terminou e quem a executa?"
> Público: Produto, arquitetura, desenvolvimento e QA
> Dono: arquitetura (coordenação cross-sistema); `wyd-db` e `wyd-net` executam suas fatias
> Base: `wyd-db/docs/roadmap/WYDDB-001.md`; `wyd-net/docs/roadmap/roadmap.md` (F0); `wyd-db/docs/guia-desenvolvedor/index.md`; `wyd-net/docs/arquitetura/componentes.md`; `wyd-net/docs/specs/requisitos.md`; `wyd-ops/docs/planos/plano-de-testes.md`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Artefato de planejamento; nenhuma implementação, issue, commit, push, deploy ou segredo foi criado.

## 1. Objetivo da fatia

Abrir o primeiro código de produção do WYD 1.0.0 sem mudar regra de jogo, schema, fronteira ou operação. A fatia é deliberadamente pequena: ela prova que os dois crates nascem com o contrato mínimo de qualidade e que o gate de sistema consegue verificar esse nascimento.

O foco é baseline, não funcionalidade. Se esta fatia crescer para migration, sessão TCP, codec ou CI real, ela deixa de ser a primeira fatia e deve ser reaberta como outra issue.

## 2. Escopo da fatia

| Sistema | Issue planejada | Escopo aceito | Fora de escopo |
|---|---|---|---|
| `wyd-db` | `WYDDB-001` | habilitar `#![deny(missing_docs)]` em `crates/wyd-db/src/lib.rs`; documentar itens públicos; fixar baseline de `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` no guia do desenvolvedor | CI real, versão mínima de Rust, documentação de módulo sem contrato, migration, schema, teste de sistema |
| `wyd-net` | `WYDNET-001` (F0) | criar `Cargo.toml` e `lib.rs` com `#![deny(missing_docs)]`; fixar baseline de `cargo fmt` e `cargo clippy`; declarar a árvore mínima de código e teste do crate | sessão TCP, handshake, rate-limit, observabilidade, CI real, captura de protocolo |

A fatia é aceita apenas se as duas partes forem verificadas no mesmo ciclo de revisão. Uma parte verde e a outra vermelha não fecha a fatia.

## 3. Aceite da issue `WYDDB-001`

Fonte: `wyd-db/docs/roadmap/WYDDB-001.md`.

- **Entregável:** `crates/wyd-db/src/lib.rs` com `#![deny(missing_docs)]`; itens públicos documentados; baseline de formatação e lint registrado no guia do desenvolvedor.
- **Aceitação:**
  - `cargo doc -p wyd-db --no-deps` termina sem warning de documentação faltante.
  - `cargo fmt --check` passa no commit de referência.
  - `cargo clippy --all-targets -- -D warnings` passa no commit de referência.
  - `guia-desenvolvedor/index.md` deixa de afirmar que os três comandos não existem, se essa afirmação ainda estiver presente.
  - Nenhum `unsafe` novo entra sem bloco de segurança justificado.
- **Risco declarado:** o baseline pode virar gate decorativo se rodar só no PR inicial; o checklist de PR deve cobrar repetição.
- **Dependência:** nenhuma; é a primeira issue da fase 1 do `wyd-db`.

## 4. Aceite da issue `WYDNET-001`

Fonte: `wyd-net/docs/roadmap/roadmap.md`, fatia F0.

- **Entregável:** `Cargo.toml`, `lib.rs` com `#![deny(missing_docs)]`, baseline de `cargo fmt` e `cargo clippy`, e a árvore mínima de código/teste declarada.
- **Aceitação:**
  - `cargo doc -p wyd-net --no-deps` termina sem warning de documentação faltante.
  - `cargo fmt --check` passa no commit de referência.
  - `cargo clippy --all-targets -- -D warnings` passa no commit de referência.
  - A árvore de `src/` e `tests/` planejada em `wyd-net/docs/arquitetura/componentes.md` §"Árvore de código e de teste" é respeitada como planejamento; nenhum módulo fora dela entra sem decisão registrada.
  - Nenhum `unsafe` novo entra sem bloco de segurança justificado.
- **Bloqueios que não são desta fatia:** captura de protocolo (`LPRT-03`, `LPRT-06`), CI real, medição de limites (`LNET-01`) e versão mínima de Rust.
- **Dependência:** crate existente; não depende de captura de protocolo.

## 5. Aceite da fatia cruzada

A fatia cruzada é aceita somente quando todos os itens abaixo estiverem verdadeiros:

1. As duas issues individuais estão aceitas pelos seus donos.
2. Os comandos de documentação, formatação e lint passam nos dois crates.
3. Nenhum arquivo de código, migration, teste ou script foi criado fora do escopo de cada issue.
4. Nenhum segredo, DSN, token, hash, senha ou PIN aparece em código, log, documento, issue ou chat.
5. A documentação dona dos dois sistemas foi atualizada na mesma entrega.
6. A revisão foi feita por alguém que não escreveu o código da fatia.
7. O gate do sistema permanece verde: `docker compose --profile gate run --rm gate` → `censos: medidas e citações coincidem`.

Se qualquer item falhar, a fatia não é aceita e a falha é registrada como defeito ou lacuna, não como "parcial".

## 6. Checklist do gate de sistema

| Item | Condição de aprovação | Condição de reprovação |
|---|---|---|
| `WYDDB-001` | os três comandos passam e o guia foi atualizado | qualquer comando falha ou o guia contradiz o baseline |
| `WYDNET-001` | os três comandos passam e a árvore planejada é respeitada | qualquer comando falha ou módulo não planejado entra sem decisão |
| escopo | apenas arquivos da issue são alterados | refatoração, migration, teste ou CI real entra no mesmo PR |
| segurança | varredura de segredo limpa | qualquer segredo ou dado de jogador aparece |
| documentação | a página dona foi atualizada | documentação fica para "depois" |
| revisão | revisão de quem não escreveu registrada | autoaprovação sem registro |
| gate de censos | `censos: medidas e citações coincidem` | divergência de censo ou link quebrado |

## 7. Modelo de evidência ao fechar

Cada issue e a fatia cruzada devem produzir, no mesmo PR ou na evidência registrada:

- saída literal de `cargo doc -p <crate> --no-deps`;
- saída literal de `cargo fmt --check`;
- saída literal de `cargo clippy --all-targets -- -D warnings`;
- diff de `src/lib.rs` e do guia do desenvolvedor afetado;
- lista de arquivos alterados e confirmação de que todos pertencem ao escopo;
- resultado de `docker compose --profile gate run --rm gate`;
- nome ou papel de quem revisou, sem ser o autor.

Evidência sem comando e saída não fecha a issue. "Testei localmente" não é evidência.

## 8. Donos e skills a carregar

| Papel | Responsabilidade | Skills a carregar |
|---|---|---|
| equipe `wyd-db` | executar e provar `WYDDB-001` | `planejar-sistema`, `auditar-planejamento` |
| equipe `wyd-net` | executar e provar `WYDNET-001` | `planejar-sistema`, `auditar-planejamento` |
| arquitetura | aprovar baseline, escopo e fronteira | `auditar-planejamento` |
| QA | repetir comandos, verificar evidência e anti-skip | `auditar-planejamento` |
| frente `wyd-ops` | fornecer ambiente efêmero e gate quando a fatia exigir | `planejar-sistema` |
| agente de redação | atualizar guia e páginas donas sem duplicar fato | `documentation-writer` |

A issue criada no GitHub deve usar o template `tarefa.yml` e preencher o campo "Skills que o agente deve carregar" com os valores acima. A criação da issue, milestone ou board continua sendo ato explícito e não é feita por este artefato.

## 9. Referências cruzadas

- `wyd-db`: `wyd-db/docs/roadmap/WYDDB-001.md`, `wyd-db/docs/guia-desenvolvedor/index.md`, `wyd-db/docs/guia-desenvolvedor/checklist-de-pr.md`.
- `wyd-db` ADRs aceitos: `wyd-db/docs/arquitetura/adr/adr-db-0001-provisionamento-de-papel-fora-da-serie.md`, `wyd-db/docs/arquitetura/adr/adr-db-0002-runner-de-migrations-proprio.md`.
- `wyd-net`: `wyd-net/docs/roadmap/roadmap.md` (F0), `wyd-net/docs/arquitetura/componentes.md`, `wyd-net/docs/specs/requisitos.md` (`RF-NET-001`, `RF-NET-002`, `RNF-NET-005`), `wyd-net/lacunas.md` (`LNET-01` como medição, `LNET-02`, `LNET-03`, `LNET-04`).
- `wyd-net` ADR aceito: `wyd-net/docs/arquitetura/adr/adr-net-0001-politica-de-limites.md` (política de limites; valores continuam pendentes de medição).
- Operação e gate: `wyd-ops/docs/planos/plano-de-testes.md`, `wyd-ops/docs/roadmap/roadmap.md` (F0, F5), `wyd-ops/docs/specs/requisitos.md` (`RF-OPS-001`, `RF-OPS-007`).
- Regras do projeto: `AGENTS.md` §5 (invariantes), §7 (fluxo), §8 (testes), §9 (CI e operação).

## 10. O que permanece aberto

- A política de limites do `wyd-net` está decidida, mas `LNET-01` continua aberta como medição; esta fatia não fecha esse número.
- A captura de protocolo (`LPRT-03`, `LPRT-06`) não é pré-requisito desta fatia e não deve ser inventada para fazê-la passar.
- A criação de issues, milestones, boards, commits, push, deploy ou segredos continua exigindo autorização explícita.
- Esta fatia não declara versão completa, não fecha o `wyd-db` e não inicia sessão TCP.

## 11. Como esta página fecha

- O arquivo responde à pergunta do cabeçalho.
- O escopo, o aceite, o gate e a evidência são verificáveis por comando ou arquivo.
- Os donos e skills estão nomeados por papel.
- As referências cruzadas apontam para documentos donos, não para cópia de fato.
- Nenhum número de censo do `wyd-db` foi citado; portanto esta página não altera a medição vigente.