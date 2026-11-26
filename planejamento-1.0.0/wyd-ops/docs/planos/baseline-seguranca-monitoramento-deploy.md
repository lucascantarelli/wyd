# Baseline de segurança, monitoramento e deploy — planejamento

> Pergunta: "qual é o baseline mínimo de segurança, observabilidade e deploy para o projeto, sem afirmar ambiente publicado?"
> Público: Desenvolvimento, operação, arquitetura e agentes
> Dono: frente `wyd-ops`
> Base: `AGENTS.md` §5, §7 e §9; `wyd-ops/docs/specs/requisitos.md` (`RF-OPS-004`, `RF-OPS-005`, `RF-OPS-006`, `RNF-OPS-003`, `RNF-OPS-004`); `wyd-ops/docs/arquitetura/componentes.md` (`monitoramento`, `segredos`, `deploy`, `ambiente`, `imagem`); `wyd-ops/docs/prioridade-do-projeto.md` (`D-05`, `D-06`, `Q-OPS-2`); `wyd-ops/auditoria-de-organizacao.md` §2–§3
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Planejamento; nenhum ambiente, segredo, deploy, monitoramento ou código foi criado por este documento.

## 1. Escopo e dono

Este documento define o baseline de segurança, monitoramento e deploy. Ele não implementa nada, não cria ambiente, não configura segredo e não publica. O dono é `wyd-ops`; sistemas consumidores continuam donos de suas regras e de seus dados.

## 2. Segurança (baseline)

| Área | Regra mínima | Evidência proposta |
|---|---|---|
| Segredos | nunca em código, documento, log, issue ou chat; só secret do GitHub ou `.env` local ignorado | varredura de segredo no repositório |
| DSN/token/hash/senha/PIN | descritos por tipo e lugar, nunca por valor | revisão de documento e código |
| Dependências Rust | `cargo audit` e `cargo deny` bloqueantes | `code.yml` / `security` |
| Dependências Python do site | `pip-audit` e hash para `mkdocs==2.0.dev6` em item separado | workflow ou gate a definir |
| Supply chain | actions pinadas por SHA; permissões mínimas; runner GitHub-hosted em PR | revisão de workflow |
| Logs | sem payload, PII, token ou DSN | varredura de log e revisão |
| Branch e release | `develop` padrão, `main` release; deploy/release/push exigem pedido explícito | `github-mapping.md` e política do projeto |
| Imagem | sem segredo na imagem; build reproduzível | `docs-image.yml` e fumaça |

## 3. Monitoramento (baseline)

| Área | Regra mínima | O que medir |
|---|---|---|
| Métricas | Prometheus ou equivalente, fonte externa | conexões, erros, bytes, latência |
| Logs | Loki ou equivalente, metadados apenas | erros, eventos de operação, sem payload/PII |
| Alertas | a definir por severidade e dono | falha de deploy, erro elevado, indisponibilidade |
| Dashboards | a criar por sistema quando houver operação própria | saúde do serviço e do banco |
| Retenção | política de retenção documentada antes de produção | log e métrica |
| Prova | métricas presentes e log sem payload/PII | teste de monitoramento |

Fonte: `wyd-ops/docs/specs/requisitos.md` (`RF-OPS-004`, `RNF-OPS-004`).

## 4. Deploy (baseline)

| Área | Regra mínima | Estado medido |
|---|---|---|
| Hospedagem de produção | VPS paga com processo persistente e Compose (`Q-OPS-2 = A`) | decisão registrada; contrato, VPS, backup e operação não executados |
| Imagem do site | construída pelo CI e consumida pelo Compose | publicada em 2026-11-26 (run `45827609542`): tags SHA/develop/latest, provenance/SBOM e smoke 200 |
| Deploy | permissões mínimas e rollback validado | planejado; nenhum deploy existe |
| Ambiente | provisionamento efêmero em CI; sem estado global | planejado |
| Segredo em deploy | rotação documentada; valor nunca em chat | planejado |
| Aprovação | deploy/release/push exigem pedido explícito | política vigente |

Fonte: `wyd-ops/docs/index.md` (decisão de hospedagem); `wyd-ops/docs/prioridade-do-projeto.md` (`D-05`, `D-06`).

## 5. Gates e verificação

| Gate | Comando/evidência | Bloqueante? |
|---|---|---|
| gate de documentação | `docker compose --profile gate run --rm gate` | sim |
| segurança de dependências | `cargo audit`, `cargo deny` | sim |
| varredura de segredo | a definir | sim |
| build da imagem | `docker build` | sim quando o caminho for tocado |
| fumaça HTTP | resposta 200 em tempo limitado | sim em PR |
| deploy/rollback | staging + rollback validado | sim antes de produção |
| monitoramento | métricas presentes; log sem payload/PII | sim quando houver operação |

## 6. Skills que o agente deve carregar

| Trabalho | Skill | Quando |
|---|---|---|
| Planejar sistema, frente ou decisão | `planejar-sistema` | antes de escrever ou alterar planejamento |
| Auditar página, decisão ou workflow antes de aceitar | `auditar-planejamento` | antes de declarar `aceito` |
| Redação técnica | `documentation-writer` | ao redigir ou revisar docs |

Fonte: `wyd-ops/docs/arquitetura/ia-e-skills.md`.

## 7. Cruzamento com auditoria e especificação

| Referência | Relação com este documento |
|---|---|
| `wyd-ops/auditoria-de-organizacao.md` §2–§3 | `wyd-ops` está organizado e tem os contratos O1–O5 aprovados; este baseline fecha parte do checklist de operação, mas não declara a implementação por si só |
| `wyd-ops/docs/specs/requisitos.md` (`RF-OPS-004`, `RF-OPS-005`, `RF-OPS-006`, `RNF-OPS-003`, `RNF-OPS-004`) | monitoramento, segredos, deploy, ausência de segredo e ausência de payload/PII |
| `wyd-ops/docs/arquitetura/componentes.md` (`monitoramento`, `segredos`, `deploy`, `ambiente`, `imagem`) | unidades e invariantes de operação |
| `wyd-ops/docs/planos/plano-de-testes.md` | alvos de teste de monitoramento, segredos e deploy |
| `wyd-ops/docs/prioridade-do-projeto.md` (`D-05`, `D-06`, `Q-OPS-2`) | workspace, frente de execução e hospedagem |
| `wyd-ops/docs/github-mapping.md` | convenções de issue, branch, PR e milestone usadas no deploy |

## 8. Decisões abertas (não decididas por este documento)

1. **Ferramenta de monitoramento** — Prometheus/Loki ou equivalente.
   - Opções: A) Prometheus + Loki; B) stack gerenciada; C) apenas logs estruturados no curto prazo.
   - Recomendação: A para produção paga; C até existir operação real.
2. **Ambiente de staging** — quando criar.
   - Opções: A) antes do primeiro deploy; B) junto com produção; C) apenas efêmero em CI.
   - Recomendação: C primeiro, A quando houver release candidata.
3. **Runtime da imagem do site** — a publicação já foi provada em `develop` em 2026-11-26; o runtime nginx unprivileged da F2 continua pendente.
   - Opções: A) manter o servidor Python atual até o deploy; B) migrar para nginx unprivileged antes do deploy; C) decidir no release 1.0.0.
   - Recomendação: B, com smoke 200 antes de promover.
4. **Rotação de segredos** — frequência e dono.
   - Opções: A) rotina documentada por segredo; B) rotação manual por incidente; C) automação futura.
   - Recomendação: A para segredos de produção; B para desenvolvimento.
5. **Alertas** — severidade e canal.
   - Opções: A) alertas bloqueantes para indisponibilidade; B) apenas informativos; C) sem alerta até staging.
   - Recomendação: C até existir ambiente real, depois A.

## 9. O que esta página não cobre

- Implementação de monitoramento, segurança ou deploy.
- Criação de ambiente, segredo, release ou deploy.
- Valores de segredo ou DSN.
- Pipeline de CI/CD detalhado: ver `pipeline-ci-cd.md`.
- Estratégia de teste transversal: ver `estrategia-de-testes.md`.