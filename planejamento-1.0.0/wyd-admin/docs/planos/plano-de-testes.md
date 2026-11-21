> Pergunta: "como o `wyd-admin` prova que escreve certo, não duplica e falha sem gravar?"
> Público: QA e desenvolvedor do `wyd-admin`
> Dono: equipe do wyd-admin
> Base: `AGENTS.md` §8; [contrato C4](../arquitetura/contrato-c4.md); [ações](../specs/acoes.md); [componentes](../arquitetura/componentes.md)
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21). Nenhum teste escrito.

## Pirâmide e alvos

| Nível | Alvo | O que prova | Dado |
|---|---|---|---|
| Unitário | `http`, `auth`, `idempotencia` | validação, papel, hash canônico, mapeamento de erro | funções puras, sem banco |
| Contrato | `tests/contrato.rs` | cada rota devolve o envelope e o status de §3/§4 | banco efêmero por teste |
| Integração | `tests/acoes.rs` | escrita, ledger, auditoria e rollback | banco efêmero + fixture de conta/canal |
| Integração | `tests/idempotencia.rs` | replay, conflito, duas requisições simultâneas | banco efêmero |
| Sistema | `tests/papel.rs` | `401`/`403` e `GRANT` real do papel | banco efêmero com o papel de `LAC-ADM-01` |
| E2E | portal (Playwright) | tela → BFF → admin → banco | fora deste sistema; contrato coberto aqui |

E2E do admin é Rust, coerente com `AGENTS.md` §8: quem tem navegador é o portal.

## O que cada teste precisa provar

1. **Nenhuma escrita sem validação:** corpo inválido não chega ao banco (contador de transações).
2. **Nenhuma escrita duplicada:** duas chamadas com a mesma chave produzem uma linha em
   `account_ledger` e um evento em `audit_log`.
3. **Nenhuma escrita parcial:** rollback forçado deixa carteira e ledger no estado anterior e sem
   linha de auditoria.
4. **Nenhuma escrita com papel errado:** `wyd_admin` sem `INSERT` falha com `503`/`500` tipado, nunca
   com sucesso.
5. **Concorrência:** dois ajustes simultâneos com o mesmo `expected_coin` — um `200`, um `409`.
6. **Timeout não é sucesso:** dublê do `wyd-server` que não responde gera `504` e sessão intacta.

## Isolamento

- Um banco efêmero por teste (`sqlx::test` ou container), migrações aplicadas até `0007` + `0011`.
- Fixtures por factory: conta com carteira, canal, cidade e item; nenhum id mágico.
- Proibido `sleep` como sincronização; concorrência usa barreira de transação.
- Nenhum estado global entre testes; nenhum `SKIP` silencioso.

## Comandos

```bash
cargo test -p wyd-admin --all-targets
cargo llvm-cov -p wyd-admin --fail-under-lines 85
```

O `--fail-under-lines 85` é o gate de `AGENTS.md` §8; cobertura de linha não substitui os seis
comportamentos acima.

## Critério de saída da `1.0.0`

Todos os testes de contrato, ação, idempotência e papel verdes; nenhum `#[ignore]` sem issue dona; os
dois comandos acima exit 0. `kick` e `broadcast` só entram no critério depois de `LAC-ADM-03`.
