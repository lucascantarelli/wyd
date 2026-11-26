# ADR-OPS-0011 — Inventário e ciclo de vida de segredos (O4)

> Pergunta: "quais segredos existem, quem emite, quem consome, onde ficam e como rotacionam?"
> Público: Arquitetura, operação, segurança e desenvolvimento
> Dono: `wyd-ops` (decisão aprovada pelo dono em 2026-11-25)
> Base: `docs/lacunas.md` §4 (`OPS-03.4`); `LAC-ADM-03` §3.3; `LAC-ADM-04`; `ADR-LCH-0001`; `.github/workflows/release.yml`; `docs/planos/baseline-seguranca-monitoramento-deploy.md`
> Revisado em: 2026-11-26

**Estado:** aceito (2026-11-26).

## Contexto

O contrato O4 foi aprovado na forma em 2026-11-25: inventário, emissor, consumidor, armazenamento e
rotação, com segredo de controle, DSN, chave de assinatura e token de serviço separados. O que faltava
era o registro da decisão. `LAC-ADM-03` §3.3 cobre só o token admin → servidor; `LAC-ADM-04` cobre o
token portal → admin; a chave de assinatura e os tokens de automação vivem nos workflows, sem inventário
comum.

Sem esse registro, o deploy de servidor e admin continuaria bloqueado e um segredo poderia ser
reutilizado por audiências diferentes. A produção não existe: provedor, VPS e secret store de produção
dependem de `OPS-04`, adiado pelo dono. Esta ADR não cria valor, não escolhe provedor e não implementa
rotação.

## Decisão

1. **Quatro classes separadas, sem reutilização:** controle e serviço (audiências diferentes), DSN de
   banco, chave de assinatura do launcher e tokens de automação. Um segredo de uma classe nunca
   autentica outra.
2. **O inventário abaixo é a fonte única.** Segredo novo só entra em uso depois de ganhar uma linha com
   classe, emissor, consumidor, armazenamento e gatilho de rotação.
3. **Armazenamento:** GitHub Actions/environment secret para CI; `.env` local ignorado pelo git; secret
   store do ambiente escolhido em `OPS-04` para produção. Valor nunca aparece em código, documento, log,
   issue, chat ou imagem.
4. **Rotação por gatilho, sem periodicidade inventada:** compromisso, troca de detentor ou de
   consumidor, mudança de escopo/ambiente e antes do primeiro deploy de produção. Tokens de serviço usam
   dois valores durante a troca (`..._NEXT`), como já decidido em `LAC-ADM-04`.
5. **Ausência de token opcional não bloqueia o gate:** `PROJECT_TOKEN` desliga só o espelho do Project;
   `SLACK_WEBHOOK_URL` desliga só o aviso. O gate continua valendo.
6. **`GITHUB_TOKEN` não é inventário:** é efêmero por job, emitido pelo GitHub, e não é armazenado.

| Classe | Nome hoje | Emissor | Consumidor | Armazenamento | Gatilho de rotação |
|---|---|---|---|---|---|
| Controle admin → servidor | `WYD_SERVER_CONTROL_TOKEN` | `wyd-admin` (operador) | `wyd-server` | secret de ambiente/GitHub; `.env` local ignorado | troca de operador, compromisso ou mudança de topologia |
| Serviço portal → admin | `WYD_ADMIN_SERVICE_TOKEN` (+ `_NEXT` na troca) | `wyd-admin` | BFF `wyd-portal` | secret de ambiente/GitHub; `.env` local ignorado | compromisso, mudança de escopo ou rotação com dois valores |
| Assinatura do launcher | `WYD_LAUNCHER_SIGNING_KEY` | `wyd-launcher` | `release.yml` | GitHub Actions secret | compromisso ou troca do detentor; chave pública no manifesto |
| Automação GitHub (opcional) | `PROJECT_TOKEN` | GitHub (PAT com escopo `project`) | `issue-status.yml` | GitHub Actions secret | mudança de escopo ou compromisso; ausência só desliga o espelho |
| Notificação (opcional) | `SLACK_CI_WEBHOOK` (exposto como env `SLACK_WEBHOOK_URL`) | Slack | workflows de aviso | GitHub Actions secret | compromisso ou troca de canal; ausência só desliga o aviso |
| Banco | DSN de produção (nome definido no deploy de `OPS-04`) | `wyd-db` (role de provisionamento) | `wyd-server`, `wyd-admin` | secret de ambiente da VPS | troca de senha do role ou compromisso |
| Efêmero | `GITHUB_TOKEN` | GitHub | workflows | não armazenado | não se aplica |

## Consequências

- Positivas: cada segredo tem classe, dono e consumidor; o deploy pode exigir o inventário sem valor em
  documento; a auditoria de segurança ganha um ponto de conferência; segredos de audiências diferentes
  deixam de se confundir.
- Negativas/custos: o secret store de produção continua dependente de `OPS-04`; a rotação é manual até
  existir automação; a troca de token de serviço exige coordenar os dois valores no deploy.
- Contratos afetados: `wyd-admin` (`LAC-ADM-03`, `LAC-ADM-04`), `wyd-portal` (BFF), `wyd-server`
  (controle), `wyd-launcher` (`release.yml`) e `wyd-ops` (CI e deploy).

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Um token genérico para todos os elos | mistura audiências; vazamento em um elo autentica os outros |
| Escolher um Vault agora | o provedor e o secret store de produção são de `OPS-04`, ainda adiado; cria dependência sem alvo |
| Rotação por calendário fixo | não há medição de necessidade nem cadência de release; o gatilho cobre compromisso e troca de dono |
| Guardar segredo cifrado no repositório | mantém o material no mesmo lugar do código e não substitui secret store |
| Inventar nomes para o DSN e para o ambiente | o nome real só existe quando `OPS-04` definir o provisionamento |

## Como verificar

- `grep -R "WYD_SERVER_CONTROL_TOKEN\|WYD_ADMIN_SERVICE_TOKEN\|WYD_LAUNCHER_SIGNING_KEY\|PROJECT_TOKEN\|SLACK_CI_WEBHOOK" .github/workflows` mostra somente referências `${{ secrets.* }}`, nunca valores; o env dos workflows se chama `SLACK_WEBHOOK_URL` por compatibilidade da action.
- `git grep -nE 'postgres://[^ ]+:[^ ]+@'` não encontra DSN com credencial.
- `release.yml` restringe `WYD_LAUNCHER_SIGNING_KEY` ao job de release; `issue-status.yml` e os avisos
  continuam funcionando sem `PROJECT_TOKEN`/`SLACK_WEBHOOK_URL`.
- `docs/lacunas.md` §4 registra `OPS-03.4` fechada com esta ADR; nenhum segredo de produção existe
  enquanto `OPS-04` estiver adiada.

## Referências

- Lacuna fechada: `OPS-03.4` (`planejamento-1.0.0/wyd-ops/docs/lacunas.md` §4)
- Decisão do dono: aprovação de O4 na forma em 2026-11-25
- ADRs relacionadas: `ADR-OPS-0007` (deploy), `ADR-LCH-0001` (assinatura do launcher)
- Documentos donos: `planejamento-1.0.0/wyd-admin/docs/arquitetura/autenticacao-e-controle.md` §2 e §3.3;
  `planejamento-1.0.0/wyd-launcher/docs/arquitetura/adr/adr-lch-0001-manifesto-canonico-assinado.md`;
  `planejamento-1.0.0/wyd-ops/docs/planos/baseline-seguranca-monitoramento-deploy.md`
