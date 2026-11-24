> Pergunta: "perdi o banco ou o dado saiu errado — o que faço, em que ordem e em quanto tempo?"
> Público: Operação e DBA do `wyd-db`
> Dono: equipe do `wyd-db` com a operação
> Base: `backup-e-restore.md`; `proposta-retencao-pendente.md` §9; `pendencias-de-fechamento.md` §6.6 (`Q-BLK-03`); `ADR-DB-0001`; `docs/arquitetura/migracoes.md`; `wyd-db-1.0.0.md` §6.15
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-24)

Recuperação de desastre é a decisão **antes** do incidente: qual cenário é qual, o que se restaura e o que se
corrige, e em quanto tempo. O procedimento de cópia e de restore não se repete aqui — ele é de
[`backup-e-restore.md`](backup-e-restore.md) §3–§5; esta página decide **quando** usá-lo.

## 1. Prazo alvo

| Medida | Valor | O que significa na prática |
|---|---|---|
| RPO | **24 h** | o que foi escrito depois da última cópia diária se perde; aceitar isso é parte da decisão |
| RTO | **4 h** | do reconhecimento do incidente ao banco validado e liberado para os processos |

Os dois números vêm de `Q-BLK-03` (2026-11-24) e são os mesmos de `backup-e-restore.md` §1 — esta página
não recalcula. Ultrapassar 4 h no ensaio trimestral é desvio a registrar em
[`riscos.md`](../planos/riscos.md), não um número novo.

## 2. Cenários

| Cenário | Sinal | O que se perde | Caminho |
|---|---|---|---|
| **corrupção lógica** (dado errado, `UPDATE`/`DELETE` indevido, script aplicado na base errada) | relato de jogador, divergência de saldo ou de item, auditoria apontando linha estranha | só o que a cópia não tem; o banco continua no ar | avaliar §3; quase sempre corrigir por script novo, não restaurar |
| **perda de volume ou do host** | banco não abre, `pg_ctl` recusa, volume vazio | tudo desde a última cópia (até 24 h) | reconstruir host pelo Compose e restaurar (§4) |
| **migration destrutiva** (`down` aplicado ou arquivo errado em produção) | `schema_migrations` com versão inesperada, tabela ausente, erro `42P01` no runtime | depende do que caiu | §3; **regra de produção ainda não publicada** (§6) |
| **indisponibilidade sem perda** (host cai, disco cheio, processo morto) | serviço fora, banco íntegro | nada | reiniciar o processo; restaurar só se a integridade falhar |

## 3. Restaurar × corrigir

| Pergunta | Restaurar a cópia | Corrigir com script |
|---|---|---|
| O dano é de **estrutura** (tabela, coluna, índice)? | sim, se não houver correção segura | só com DDL novo e revisão |
| O dano é de **dado** em poucas linhas e o valor correto é conhecido? | não | sim |
| O dano é de **dado** em volume e a origem é desconhecida? | sim | não |
| O que se perde ao restaurar é maior que o dano? | não restaurar | sim |
| Há janela para parar os processos que escrevem? | restaurar exige janela | corrigir pode rodar a quente |

Regra de ouro: **restaurar é perder tudo o que veio depois da cópia.** Se o dano é menor que 24 h de
escrita, corrigir ganha; se o dano é estrutural ou de origem desconhecida, restaurar é a única saída
honesta.

## 4. Passos do cenário de perda total

1. **Conter:** parar os processos que escrevem (jogo, portal, admin) para não gravar sobre um banco em
   recuperação.
2. **Reconstruir:** subir o ambiente pelo Compose da operação (`Q-OPS-2 = A`) com volume vazio.
3. **Papéis primeiro:** `wyd-db provision plan` e `apply` (`WYD_PROVISION_DSN`), porque os papéis não vêm da
   série (`ADR-DB-0001`). Sem eles, o restore falha por dono ausente.
4. **Restaurar:** `pg_restore --exit-on-error` da última cópia, conforme
   [`backup-e-restore.md`](backup-e-restore.md) §4.
5. **Validar:** as quatro provas de `backup-e-restore.md` §5 (ledger, contagem por tabela, papéis,
   privilégio de `wyd_admin`).
6. **Liberar:** só então religar os processos e conferir o primeiro login e o primeiro save.
7. **Registrar:** hora do reconhecimento, hora da validação e RTO real.

## 5. Comunicação

| Quem | O quê | Quando |
|---|---|---|
| operador do banco | assume o incidente e abre o registro com hora | no reconhecimento |
| dono do produto | perda estimada (até 24 h) e previsão de retorno | em até 1 h do reconhecimento |
| suporte ao jogador | aviso de manutenção e do que pode ter sido perdido | quando a janela é confirmada |
| jogador | aviso pelo canal do portal | é do `wyd-portal`/`wyd-ops`, não desta página |

Nenhum aviso cita DSN, host, versão de migration ou nome de tabela: o jogador recebe o que muda para ele,
não o diagnóstico interno (`AGENTS.md` §5, item 7).

## 6. O que esta página não decide

- **O que fazer quando uma migration já rodou em produção:** a regra é da
  `migrations/serie-e-regras.md`, que **não existe** no worktree (é página da 5ª onda,
  `pendencias-de-fechamento.md` §4.3). Até ela existir, migration destrutiva em produção é decisão do dono,
  caso a caso, com o `down` do próprio arquivo como insumo (`wyd-db-1.0.0.md` §6.15).
- **A cópia, a retenção e o ensaio:** são de [`backup-e-restore.md`](backup-e-restore.md).
- **O agendamento da cópia:** é de [`plano-de-manutencao.md`](../planos/plano-de-manutencao.md).
- **Fornecedor, contrato e região do destino:** `Q-BLK-03` fixou “mesmo provedor da VPS”; contrato e região
  são de `wyd-ops`.
- **A credencial nominal:** `guia-administrador/index.md` §3.
- **O aviso ao jogador:** é do `wyd-portal`/`wyd-ops`.
