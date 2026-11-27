# Skills do `wyd-ops` — roteador

O `wyd-ops` opera todos os sistemas: CI, gate, imagem, board e entrega. As skills daqui são de
**processo**, não de domínio — o domínio de cada sistema tem a skill dele.

| Tarefa | Skill | Quando não usar |
|---|---|---|
| entregar mudança por issue → PR → checks → merge → gate | [`execution/entregar-mudanca`](execution/entregar-mudanca/SKILL.md) | mudança é só de página de planejamento e não toca código |
| mexer em workflow, action composta, imagem ou rollback | [`execution/operar-ci-e-imagem`](execution/operar-ci-e-imagem/SKILL.md) | o job já está verde e só falta documentar |
| revisar PR que altera `.github/workflows` ou `docker-compose*` | [`review/revisar-workflow`](review/revisar-workflow/SKILL.md) | revisão de DDL ou de página |

## Fronteiras

- **Gate em um ponto só**: `make gate-fast` no ciclo, `make gate` no fim. A decisão é do
  `ADR-OPS-0010`; não crie script de verificação paralelo.
- **Segredo nunca aparece**: workflow, log, documento e issue citam tipo e lugar, nunca valor.
- **Board não se atualiza sozinho**: sem `PROJECT_TOKEN`, o cartão é movido por API.
