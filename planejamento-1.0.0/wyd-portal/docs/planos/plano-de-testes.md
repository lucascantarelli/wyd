> Pergunta: "como o `wyd-portal` prova que lê certo, não vaza segredo e não escreve em `game` direto?"
> Público: QA e desenvolvedor do `wyd-portal`
> Dono: equipe do wyd-portal
> Base: `AGENTS.md` §8; [fronteira.md](../arquitetura/fronteira.md); [rotas-e-permissoes.md](../arquitetura/rotas-e-permissoes.md); [componentes.md](../arquitetura/componentes.md); `wyd-admin/docs/planos/plano-de-testes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum teste escrito.

## Pirâmide e alvos

| Nível | Alvo | O que prova | Dado |
|---|---|---|---|
| Unitário | `auth`, `cache`, `monitoramento` | validação, papel, hash, TTL, allowlist de LogQL | funções puras, sem banco |
| Contrato | `tests/contrato/` | cada rota devolve o envelope e o status de `rotas-e-permissoes.md` | banco efêmero por teste |
| Integração | `tests/integracao/` | leitura pelo adapter, transação por requisição, cache de 60 s | banco efêmero + fixture de conta/personagem |
| Sistema | `tests/integracao/` (fronteira) | `wyd_portal` sem `USAGE` em `game`; mutação administrativa chama o `wyd-admin` e grava `audit_log` | banco efêmero com o papel de `P-17` |
| E2E | `tests/e2e/` (Playwright) | tela → BFF → banco/admin; `member` recebe `403` em `/admin/*`; P95 < 1 s | ambiente efêmero com dados sintéticos |
| Acessibilidade | `tests/acessibilidade/` | contraste ≥ 4,5:1 e foco visível em todos os controles, nos dois temas | varredura automatizada |

E2E do portal é Playwright, coerente com `AGENTS.md` §8: quem tem navegador é o portal.

## O que cada teste precisa provar

1. **Nenhuma escrita em `game` pelo BFF:** toda mutação de estado de jogo chama o `wyd-admin` e não executa SQL em `game` (contador de transações + inspeção de `audit_log`).
2. **Nenhum segredo em resposta ou log:** varredura de resposta HTTP e de log sem `password_hash`, `pin_hash`, `token_hash`, URL interna de métrica ou segredo.
3. **Nenhuma rota administrativa sem papel:** `member` recebe `403` em `/admin/*`; `staff` sem TOTP não recebe sessão quando for `admin`.
4. **Nenhuma escrita duplicada no mercado:** duas requisições com a mesma chave não criam segundo anúncio nem segunda custódia.
5. **Nenhuma escrita parcial:** falha injetada no `audit_log` desfaz a mutação administrativa.
6. **Fronteira de papel:** `wyd_portal` sem `USAGE` em `game` falha com `42501`; leitura de personagem só pela view de `0029`.
7. **Telemetria honesta:** fonte fora deixa só os cartões dela em “não medido”; nenhum card de CPU/RAM/latência vem de tabela de `portal`/`ops`.

## Isolamento

- Um banco efêmero por teste, migrações aplicadas até a série vigente (`0001`–`0034`).
- Fixtures por factory: pessoa, conta, personagem, canal, item; nenhum id mágico.
- Proibido `sleep` como sincronização; concorrência usa barreira de transação.
- Nenhum estado global entre testes; nenhum `SKIP` silencioso.
- Cobertura mínima de 85% (`RNF-POR-006`); cobertura de linha não substitui os sete comportamentos acima.

## Comandos

```bash
# unitário + contrato + integração
npm test
# E2E
npx playwright test
# cobertura
npm run test:coverage
```

O `--fail-under-lines 85` (ou equivalente do workspace) é o gate de `AGENTS.md` §8.

## CI, cobertura e monitoramento

- **CI:** `.github/workflows/code.yml` roda `npm ci`, `npm test`, `npx playwright test` e `npm run test:coverage`; o PR para `develop` e `main` só passa com os quatro verdes.
- **Cobertura:** `cargo llvm-cov -p  --fail-under-lines 85` é o piso; a cobertura de linha não substitui os comportamentos listados acima e `#[ignore]` exige issue dona e prazo.
- **Monitoramento:** Latência e status por rota, `trace_id`, erros de BFF, cache hit/miss e falha das fontes Prometheus/Loki. A página `arquitetura/monitoramento.md` é o dono; fonte fora aparece como “não medido”, nunca como zero.
- **Critério verificável:** o comando de teste e o gate de documentação exit 0; nenhuma falha é mascarada por `| tail`, `--no-fail-fast` ou `SKIP` silencioso.

## Critério de saída da `1.0.0`

Todos os testes de contrato, integração, fronteira e E2E verdes; nenhum `skip`/`ignore` sem issue dona; os comandos acima exit 0. As rotas de monitoramento (38–43) entram no critério com dublê das fontes externas, porque o contrato é de leitura HTTP.
