> Pergunta: "como o `wyd-ops` prova CI, imagem, ambiente e deploy sem inventar métrica?"
> Público: QA e desenvolvedor do `wyd-ops`
> Dono: equipe do wyd-ops
> Base: `AGENTS.md` §8; `wyd-ops/docs/index.md`; `wyd-protocol/docs/planos/plano-de-testes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Um teste unitário escrito em 2026-11-26 (`sync-issue-status`); os demais alvos continuam sem teste.

## Pirâmide e alvos

| Nível | Alvo | O que prova | Dado |
|---|---|---|---|
| Unitário | `gate`, `config`, `sync-issue-status` | comando, configuração e transição de estado da issue | funções puras |
| Contrato | `tests/contrato/` | (n/a) | — |
| Integração | `tests/integracao/` | smoke de ambiente | ambiente efêmero |
| Sistema | `tests/sistema/` | deploy, rollback | GitHub Pages/servidor |
| E2E | (n/a) | — | — |
| Propriedade | (n/a) | — | — |

## O que cada teste precisa provar

1. **CI:** workflow verde em PR; permissões mínimas; pin por SHA.
2. **Imagem:** build reproduzível; sem segredo.
3. **Ambiente:** up/down; seed aplicado; sem estado global.
4. **Monitoramento:** métricas presentes; log sem payload/PII.
5. **Segredos:** varredura de segredo no repo.
6. **Deploy:** deploy em staging; rollback validado.
7. **Gate:** `docker compose --profile gate run --rm gate` verde; censos coincidem.
8. **Transição de issue:** `estado/*` trocada em uma requisição, `unlabeled` sem outra label e eco do
   próprio bot ignorado (`wyd-ops/tests/unitario/sync-issue-status.test.mjs`).

## Isolamento

- Testes em ambiente efêmero por execução.
- Proibido `sleep` como sincronização.
- Cobertura mínima de 85% onde aplicável.

## Comandos

```bash
node --test wyd-ops/tests/unitario/
docker compose --profile gate run --rm gate
```

## CI, cobertura e monitoramento

- **CI:** `.github/workflows/code.yml` valida Compose e o job de documentação roda o gate; o deploy continua manual e fora do CI até a implementação de F3/F4 e a aplicação de `OPS-04`.
- **Cobertura:** A cobertura é de gate e de artefato: pipeline, imagem, ambiente, monitoramento, segredo e rollback precisam de execução registrada; não há percentual de linha sem crate.
- **Monitoramento:** Status do gate, build da imagem, deploy, `ready` do servidor, Prometheus/Loki e alertas do baseline. A página `baseline-seguranca-monitoramento-deploy.md` é o dono dos limiares e do que ainda não foi executado.
- **Critério verificável:** o comando de teste e o gate de documentação exit 0; nenhuma falha é mascarada por `| tail`, `--no-fail-fast` ou `SKIP` silencioso.

## Critério de saída da `1.0.0`

CI, imagem, ambiente, monitoramento, segredos e deploy verdes; gate verde; `RF-OPS` numerados; nenhuma lacuna de organização aberta. O dashboard existe e o registro de lacunas vive em `docs/lacunas.md`.