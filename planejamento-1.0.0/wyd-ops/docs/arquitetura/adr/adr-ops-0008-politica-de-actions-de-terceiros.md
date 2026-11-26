# ADR-OPS-0008 — Política de actions de terceiros

> Pergunta: "quando uma action de terceiro pode entrar no pipeline e como ela é mantida?"
> Público: Arquitetura, desenvolvimento e segurança
> Dono: `wyd-ops` (aprovado pelo dono em 2026-11-26)
> Base: `docs/planos/pipeline-ci-cd.md` §5; configuração do repositório no GitHub
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-26).

## Contexto

O repositório já usava `taiki-e/install-action` e passou a usar `Swatinem/rust-cache` e `dtolnay/rust-toolchain`; `sha_pinning_required` estava desligado e não havia Dependabot. Pin por SHA protege contra tag móvel, mas congela o runtime (foi a causa do warning de Node 20) e esconde atualizações de ações compostas.

## Decisão

1. Toda action de terceiro entra por SHA completo, com versão em comentário.
2. A allowlist do repositório é restrita a GitHub + as actions auditadas: `Swatinem/rust-cache`, `taiki-e/install-action`, `dtolnay/rust-toolchain`, `docker/*` quando usadas.
3. Dependabot `github-actions` semanal é obrigatório; atualização de SHA passa por PR e pelo gate.
4. Preferir `run:` a uma action nova quando o comando é simples (login, curl, jq).
5. `sha_pinning_required` fica ligado.

## Consequências

- Positivas: supply chain verificável; runtime acompanha o runner; ações não auditadas falham na allowlist.
- Negativas/custos: Dependabot abre PRs semanais; allowlist precisa ser atualizada a cada action nova; SHA de action composta pode esconder dependências internas.
- Contratos afetados: qualquer workflow novo; `dependabot.yml`; configuração de Actions do repositório.

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Tags móveis (`@v4`) | supply chain e runtime imprevisíveis |
| Só actions oficiais | `rust-cache` e `install-action` têm ganho real; proibir tudo custa mais |
| Dependabot sem pin | perde a garantia de SHA |

## Como verificar

`gh api repos/lucascantarelli/wyd/actions/permissions` mostra `sha_pinning_required: true`; todo `uses:` de terceiro tem SHA; Dependabot existe; o PR de atualização roda o gate.

## Referências

- Decisão do dono: Q9 do plano de DevOps (2026-11-26)
- Arquivos: `.github/dependabot.yml`, workflows, configuração do repositório
