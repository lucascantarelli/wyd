# Cobertura do `wyd-portal` — dono do fechamento de leilão

> Pergunta: "onde roda o job que fecha leilão vencido, expira anúncio sem lance e devolve custódia, e
> quem o agenda?"
> Público: arquitetura, `wyd-portal`, `wyd-admin` e `wyd-ops`
> Dono: arquitetura + `wyd-ops`
> Base: `lacunas.md` (`LAC-POR-09`); `docs/arquitetura/consumo-do-banco.md`;
> `docs/specs/requisitos.md` (`RF-POR-025`, `RNF-POR-009`); `armaria-e-mercado.md` (D4, `M25`);
> `AGENTS.md` §5.8
> Revisado em: 2026-11-23

**Estado:** proposta (2026-11-23). **Nenhuma pergunta de desenho**: A é a única opção compatível com
`LAC-POR-01`; B exigiria reabrir a fronteira e criar um segundo escritor em `portal`, e C/D já foram
recusadas em D4. Falta o "sim" do dono de operação, que é aprovação de execução, não escolha entre
alternativas. Nada foi aplicado. Não há job, métrica nem agendamento; nenhum número desta página é
medição.

## 1. Fronteira

**Cobre:** o dono do fechamento de leilão/anúncio vencido e o agendador — `LAC-POR-09`.

**Não decide:** a regra do fechamento (idempotente, `SKIP LOCKED`, uma transação por anúncio, D4), a
custódia (`M31`/`M25`) nem o `GRANT` do `wyd-db`; o `LAC-POR-01` (só o BFF escreve `portal`) não é
reaberto.

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| lacunas do portal | **1 proposta, 0 abertas, 8 fechadas** | `lacunas.md` §1 |
| job de fechamento | **não existe** | `lacunas.md` `LAC-POR-09` |
| agendador e alerta | **não decididos** | idem |
| fechamento idempotente | **decidido**: `SKIP LOCKED`, uma transação por anúncio, `uq_market_transaction_listing` | `lacunas.md`; `M25` |
| regra de leilão sem lance | **decidida** (D4): expira e devolve custódia | `armaria-e-mercado.md` D4 |

## 3. Já decidido — não reabrir

| Item | Decisão vigente | Fonte | Efeito |
|---|---|---|---|
| escrita em `portal` | só o BFF do portal escreve; `wyd_portal` não escreve `game` | `LAC-POR-01`; `P-17` | a opção não pode criar worker com escrita em `portal` |
| dreno concorrente | `FOR UPDATE SKIP LOCKED` em transação curta | `M25` | o job usa o mesmo padrão |
| fechamento | idempotente, uma transação por anúncio, `uq_market_transaction_listing` | `LAC-POR-09`; `M25` | não se rediscute a regra |
| leilão sem lance | expira e devolve a custódia; vencedor recebe item, perdedor recebe reserva | D4; `RF-POR-025` | não se rediscute o resultado |

## 4. Cobertura da lacuna

`LAC-POR-09` continua `proposta`: falta o "sim" do dono de operação para A. B existe na fonte, mas
cria worker com escrita em `portal` e exigiria reabrir `LAC-POR-01`; por isso não é alternativa válida
nesta página. C e D foram recusadas em D4. O status continua `proposta`, não `fechada`.

| Lacuna | Pergunta | Por que não há pergunta |
|---|---|---|
| `LAC-POR-09` | — | A é a única opção que preserva `LAC-POR-01`/`P-17`, usa o BFF que já escreve `portal` e não cria `GRANT`; B exigiria reabrir a fronteira e criar segundo escritor; C/D recusadas em D4. Falta aprovação de execução. |

## 5. Registro da decisão

**Aguardando aprovação do dono de operação.** `LAC-POR-09` continua `proposta`; nada foi aplicado.

| Lacuna | Aprovação | Resposta |
|---|---|---|
| `LAC-POR-09` | A | — |
