# ADR-OPS-0007 — Deploy sobre Compose e contrato O5 (provedor de VPS a definir)

> Pergunta: "como planejar deploy sem escolher um provedor e sem simular ambiente publicado?"
> Público: Arquitetura, operação e produto
> Dono: `wyd-ops` (aprovado pelo dono em 2026-11-26)
> Base: `ADR-OPS-0002` (hospedagem em VPS paga com Compose); decisão Q13 do plano de DevOps
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-26).

## Contexto

`ADR-OPS-0002` já decidiu que a produção será uma VPS paga com Docker Compose — não Kubernetes. O **fornecedor** da VPS, a conta, o domínio e os segredos de deploy continuam indefinidos. Criar workflow de deploy agora produziria um job que falha de propósito ou um deploy fictício.

## Decisão

Não criar workflow de deploy nesta fase. Fixar o contrato: imagem imutável por digest/SHA, `docker compose` como runtime, smoke HTTP, rollback por tag anterior, OIDC em vez de chave de longa duração quando o provedor for escolhido. A implementação fica na F3, bloqueada pela escolha do fornecedor.

## Contrato O5 — servidor e launcher (2026-11-26)

O contrato O5 foi aprovado na forma em 2026-11-25; este ADR o registra sem criar workflow novo e sem
contrariar a decisão acima.

1. **Servidor:** runtime `docker compose` na VPS escolhida em `OPS-04`; artefato imutável por digest/SHA,
   nunca `latest`; deploy manual com `environment` e aprovação explícita, nunca automático em push.
2. **Launcher:** a release publica manifesto assinado (`ADR-LCH-0001`); o rollback restaura o manifesto e
   a versão anteriores; a distribuição continua congelada por `LCLI-05`.
3. **Rollback:** comando e dono são do `wyd-ops`; o registro do deploy guarda o digest/manifesto atual e
   o anterior, e a volta usa o anterior. O rollback só é exigível depois de ensaiado em staging.
4. **Staging, provedor, janela, credenciais e backup** continuam em `OPS-04`/F3–F4; esta ADR não escolhe
   fornecedor nem inventa ambiente. Sem `OPS-04`, não há deploy de produção.

## Consequências

- Positivas: nenhum segredo de produção inventado; nenhum ambiente declarado publicado; o contrato serve a qualquer VPS.
- Negativas/custos: a entrega fica incompleta até a decisão de negócio; o rollback só será testado de ponta a ponta depois.
- Contratos afetados: `release.yml` continua publicando manifesto assinado, não binários; `docs-image.yml` publica imagem, não ambiente.

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Workflow manual que falha sem alvo | confunde o time e não prova nada |
| Deploy automático em `main` | sem staging, smoke e rollback validados |
| Escolher provedor agora | decisão de custo/conta do dono, fora do escopo técnico |

## Como verificar

Não existe workflow de deploy com `environment: production`; `release.yml` só assina/publica manifesto; a página de DevOps declara a F3 bloqueada e o critério de saída.

## Referências

- Decisão do dono: Q13 do plano de DevOps (2026-11-26); O5 aprovado na forma em 2026-11-25
- Lacuna fechada: `OPS-03.5` (`planejamento-1.0.0/wyd-ops/docs/lacunas.md` §4)
- ADRs relacionadas: `adr-ops-0002-hospedagem-producao.md`, `adr-lch-0001-manifesto-canonico-assinado.md`
- Página: `docs/devops/index.md` §"F3 — deploy" e §"F4 — segredos e deploy"
