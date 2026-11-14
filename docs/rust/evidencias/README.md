# Índice de evidências técnicas

> Provas registradas no repositório que sustentam issues concluídas. Cada arquivo registra o
> que foi medido, como foi verificado e o que permanece como hipótese ou pendência. O status
> canônico das issues continua em [`roadmap-rust/index.md`](../../../roadmap-rust/index.md),
> o painel em [`docs/rust/03-progresso.md`](../03-progresso.md) e o fechamento em
> [`roadmap-rust/changelog.md`](../../../roadmap-rust/changelog.md).

| Evidência | Issue | Data | Resultado |
|---|---|---|---|
| [`AUDIT-ISSUES-2026-11-12.md`](AUDIT-ISSUES-2026-11-12.md) | fechamento de `GAME-19`/`WEB-01` | 2026-11-12 | 29 issues conferidas; 2 fechadas (`GAME-19`, `WEB-01`) e 27 mantidas com critério real nomeado. |
| [`PROC-02-limpeza-ambiente-2026-11-12.md`](PROC-02-limpeza-ambiente-2026-11-12.md) | `PROC-02` | 2026-11-12 | Go 1.27.3 desinstalado; `where go` não resolve; caches, ferramentas, `GOPATH`/`GOROOT` e extensão Go removidos. |
| [`PROC-23-pocs-2026-11-12.md`](PROC-23-pocs-2026-11-12.md) | `PROC-23` | 2026-11-12 | PoCs medidas: SoA vence `hecs`, parse manual mantido, `io_uring`/`wasmtime` não adotados; nenhuma dependência nova. |
| [`AUDIT-IA-2026-11-12.md`](AUDIT-IA-2026-11-12.md) | `PROC-18` | 2026-11-12 | Auditoria das instruções de IA; skills reduzidas a 23 e referências/gates íntegros. |
| [`AUDIT-SKILLS-2026-11-12.md`](AUDIT-SKILLS-2026-11-12.md) | `PROC-18` (2ª/3ª passadas) | 2026-11-12 | 23→18 skills; 2 duplicadas fundidas/removidas, 3 genéricas removidas; dedup de 18 skills contra AGENTS/docs; gate `skills-check` valida doc paths. |
| [`PROC-03-rust-analyzer.md`](PROC-03-rust-analyzer.md) | `PROC-03` | 2026-11-12 | Workspace analisado pelo rust-analyzer, VS Code em Rust e sem extensão Go. |
| [`PROC-05-auditoria-2026-11-12.md`](PROC-05-auditoria-2026-11-12.md) | `PROC-05` | 2026-11-12 | **36 issues do baseline:** 21 abertas, 8 em andamento, 2 parciais, 1 pronta, 1 bloqueada e 3 concluídas; 33 ativas + 3 fechadas, com próximo passo por linha. |
| [`PROC-11-revisao-fria-2026-11-12.md`](PROC-11-revisao-fria-2026-11-12.md) | `PROC-11` | 2026-11-12 | Árvore real, DAG e módulos confrontados com o documento normativo; divergências corrigidas. |
| [`PROC-13-gate-docs.md`](PROC-13-gate-docs.md) | `PROC-13` | 2026-11-12 | Gate de links e consistência de docs incorporado ao `make check`. |
| [`QA-08-prova-viva-2026-11-12.md`](QA-08-prova-viva-2026-11-12.md) | `QA-08` | 2026-11-12 | Dashboards com série real dos dois canais; 10 regras e 12 painéis. |
| [`WEB-02-painel-vivo.md`](WEB-02-painel-vivo.md) | `WEB-02` | 2026-11-12 | Quatro fluxos do painel do jogador provados no portal/gateway vivos. |
| [`WEB-04-e2e-2026-11-12.md`](WEB-04-e2e-2026-11-12.md) | `WEB-04` | 2026-11-12 | E2E Playwright: 6 specs, três execuções verdes. |
| [`personagens-soft-delete-2026-11-14.md`](personagens-soft-delete-2026-11-14.md) | `GAME-08` (identificação do personagem) | 2026-11-14 | Seis linhas `Cantarelli` = uma viva (id 164) + cinco soft-deleted; o cliente carrega a 164 pelo slot 0; limite de 4 slots por conta e dependência de slot livre para Mortal + Arch documentados. |
| [`AUD-02-createitem-2026-11-14.md`](AUD-02-createitem-2026-11-14.md) | `AUD-02` / `DIV-02` | 2026-11-14 | Direção do `CreateItem` (0x026E) corrigida para S→C no catálogo congelado, valor de fio intacto; teste com mutação RED→GREEN e `DIRECTION_OVERRIDES` no gerador. |
| [`auditoria-cobertura-roadmap-2026-11-14.md`](auditoria-cobertura-roadmap-2026-11-14.md) | `AUD-19`…`AUD-22` | 2026-11-14 | Cobertura fechada: 37 `UNK`, 14 `DIV`, 10 `N-*` e a decisão 4.5 ganharam issue com fase e aceite; tabela entrega→issue do §17 (33 linhas) e checker `check-audit-coverage` no `make check`; 6 defeitos documentais corrigidos (incl. teste citado que não existia); 851 testes verdes. |
| [`roteiro-manual-verificacao-2026-11-14.md`](roteiro-manual-verificacao-2026-11-14.md) | `QA-12` / `QA-13` | 2026-11-14 | Cada comando e consulta do roteiro manual executado contra a stack real: 6 erros de documento corrigidos (compose `--wait-timeout`, `serv00.htm`, CLI do launcher, colunas de itens/quest/sessão, `:9898` não publicado, grafia de item); teste de lista de contas corrigido para paginar; 851 workspace + 300 sistema verdes. |
| [`auditoria-status-honestidade-2026-11-14.md`](auditoria-status-honestidade-2026-11-14.md) | honestidade dos 50 status ✅ | 2026-11-14 | Inventário das 50 issues resolvidas contra teste citado → existe → executado → assere; 6 mutações RED→GREEN; 2 testes decorativos corrigidos (`INITCODE`, regra de gema duplicada); 20 issues rebaixadas a 🟢 com motivo. |
| [`combate-e-loja-vivo-2026-11-14.md`](combate-e-loja-vivo-2026-11-14.md) | dano (`BASE_GetDamage`), quest do Campo e loja de NPC | 2026-11-14 | Dano 1 → 5 (AC 40) / 12 (AC 2); `GremlinDecoy` morto com `hp_left=0`, `exp_granted=1313` e drop 451 no inventário, quest em `step=2`; `Runas_Joias` devolveu catálogo e a compra de 25 500 debitou o ouro (`5 000 000 → 4 974 500`). Respawn e equipamento ficam declarados como abertos. |
| [`nivel-wire-2026-11-14.md`](nivel-wire-2026-11-14.md) | nível no fio (cliente × banco) | 2026-11-14 | Proposição do cliente 0-based confirmada (`CMob.cpp` 354/369, `MAX_LEVEL 399`); banco 1→selchar 0/login 0/createmob 0, 200→199, 399→398 medidos no canal vivo; teste com mutação RED→GREEN. Level-up ao vivo e tela do cliente declarados como não verificados. |
| [`auditoria-roadmap-2026-11-14.md`](auditoria-roadmap-2026-11-14.md) | abertura de `AUD-01`…`AUD-17` | 2026-11-14 | Suíte do workspace medida (847 passed, 0 failed, 1 ignored, 76 binários); 17 issues da auditoria abertas e mapeadas no índice/painel; eixo cliente/live declarado como não provado. |

## Regras

- Evidência nova entra com issue, data, comando/medição e resultado; hipótese é marcada como
  hipótese, nunca como prova.
- Issue concluída sai de `roadmap-rust/issues/` e entra no changelog; esta tabela guarda o
  caminho da prova.
- Arquivo de evidência não substitui o critério de aceite da issue nem promove dependências
  externas (cliente 7662/VM) sem a prova correspondente.
