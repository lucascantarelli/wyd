> Pergunta: "como o `wyd-ops` prova CI, imagem, ambiente e deploy sem inventar métrica?"
> Público: QA e desenvolvedor do `wyd-ops`
> Dono: equipe do wyd-ops
> Base: `AGENTS.md` §8; `wyd-ops/docs/index.md`; `wyd-protocol/docs/planos/plano-de-testes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum teste escrito.

## Pirâmide e alvos

| Nível | Alvo | O que prova | Dado |
|---|---|---|---|
| Unitário | `gate`, `config` | comando, configuração | funções puras |
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

## Isolamento

- Testes em ambiente efêmero por execução.
- Proibido `sleep` como sincronização.
- Cobertura mínima de 85% onde aplicável.

## Comandos

```bash
docker compose --profile gate run --rm gate
```

## Critério de saída da `1.0.0`

CI, imagem, ambiente, monitoramento, segredos e deploy verdes; gate verde; `RF-OPS` numerados; nenhuma lacuna de organização aberta. `wyd-ops` sem dashboard próprio até esta rodada.