# ADR-0029 — `wyd-core` consome a matemática pura de `wyd-sim` (regra única de gema)

**Status:** aceita (2026-11-14)
**Contexto:** a regra da gema (quais itens são gema, piso de +10 para Anct, valor por refino)
existia duas vezes: em `crates/wyd-sim/src/formula/refine.rs` e em
`crates/wyd-core/src/ecs/components.rs` (`plan_gem`, `gem_value`, `gem_effect_code`). A
auditoria de status de 2026-11-14 (mutação 5 de
`docs/rust/evidencias/auditoria-status-honestidade-2026-11-14.md`) mostrou o custo: mutar a
cópia de `wyd-sim` **não** derrubava `integracao_gem::anct_e_estritamente_melhor_que_comum_no_efeito_da_gema`,
porque o teste passa pela cópia do core. Duas fontes da mesma regra podem divergir sem que
nenhum gate acuse — o risco é balanceamento valer no fio e não valer na simulação (ou o
contrário).

O DAG de `docs/arquitetura/estrutura-e-padroes.md` §1 dizia `wyd-core ► wyd-protocol` e
`wyd-sim ► (independente)`, com a nota de que a unificação exigiria contrato em
`wyd-protocol`. `wyd-protocol` não serve: a regra 2 do mesmo documento proíbe regra de jogo
na crate de bytes. Um crate novo só para ~70 linhas de regra pura não se justifica, e
`wyd-sim` já é a crate de matemática determinística, sem dependências internas e já presente
no binário do servidor (a raiz de composição `wyd-server` depende dela desde a ADR-0022).

## Decisão

1. **A regra da gema mora só em `wyd-sim`** (`formula::refine`): `GemEffect`, `gem_effect_value`,
   `apply_gem`, constantes de refino e percentuais, e o contrato numérico de estado
   (`GemEffect::state_code`/`from_state_code`, 1..=4).
2. **`wyd-core` passa a depender de `wyd-sim`** e consome essa regra. O core mantém apenas o
   que é estado e transação: `GEM_EFFECT_*` como códigos persistidos (re-exportando
   `GEM_MIN_REFINE`/`GEM_MAX_REFINE`), `plan_gem` (valida sem escrever, consome a gema e
   publica `DomainEvent::GemEffectChanged`) e a conversão `GemError → CoreError`.
3. A aresta é **estreita**: `wyd-core → wyd-sim` vale só para funções puras (sem I/O, sem
   relógio, sem estado, sem protocolo). `wyd-sim` continua sem depender de `wyd-core`,
   `wyd-protocol`, `wyd-db` ou `wyd-net`, então o DAG segue acíclico e o core segue sem saber
   o que é Postgres ou socket.
4. A **transação** continua exclusiva do core: de fora do crate não há como aplicar gema numa
   cópia solta (`plan_gem`/`commit_gem`/`consume_gem` são `pub(crate)`).

## Consequências

- Uma mutação na regra derruba **os dois** lados: o golden da simulação
  (`crates/wyd-sim/tests/golden/formulas.rs::ancient_gets_base_value_below_ten`) e a integração
  do core (`crates/wyd-core/tests/integracao/gem.rs::anct_e_estritamente_melhor_que_comum_no_efeito_da_gema`).
- Nenhum byte novo no binário: `wyd-sim` já era dependência de `wyd-server`; a aresta só
  formaliza o uso direto pelo core.
- O comentário de `crates/wyd-core/src/npc/state.rs` (que citava a proibição) passa a
  descrever o estado real: a aresta existe, e a **ameaça** continua como cópia presa por golden
  até a unificação da GAME-19.
- A dívida registrada em `docs/rust/06-matematica.md` §4 deixa de ser "regra duplicada" e passa
  a ser "regra única em `wyd-sim`, transação no core".

## Alternativas descartadas

| Alternativa | Por que não |
|---|---|
| Manter as duas cópias presas por golden | Foi exatamente o que a mutação expôs: o golden prende número, não impede divergência silenciosa entre caminhos |
| Mover a regra para `wyd-protocol` | Contraria a regra 2 do DAG (protocolo só conhece bytes) |
| Criar crate `wyd-rules` só para a gema | Mais uma fronteira, mais contagem de crates nos docs e um membro de workspace para ~70 linhas sem dependências |
| `wyd-sim` depender de `wyd-core` | Inverte a direção: laboratório passaria a carregar o estado do engine |
