# Pacote de aprovação — decisões fora dos contratos O1–O5

> Pergunta: "o que falta aprovar além de O1–O5, quem responde cada item e o que continua bloqueado sem resposta?"
> Público: dono do produto, arquitetura, `wyd-db`, QA, `wyd-ops` e revisão jurídica
> Dono: equipe do `wyd-ops`; cada aprovação pertence ao dono listado na seção
> Base: `docs/lacunas.md` §4 (`OPS-04`, `OPS-05`, `D-06`, `Q-SERIE-1`, `Q-SEP-1`, `LNET-01`/`LBOT-03`, `LCLI-02`/`LCLI-05`); `docs/planos/medicoes-lnet-01-lbot-03.md`; `docs/proposta-decisoes-ops.md`; `ADR-OPS-0002`
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Instrumento de leitura e aprovação das decisões que o pacote O1–O5 declara fora de escopo; **não decide, não aprova e não altera** `docs/lacunas.md` §4. Em 2026-11-25 o dono respondeu às duas decisões de §4 (medições) e às duas de §5 (direitos), aceitando a recomendação; `OPS-04`, `OPS-05`, `D-06`, `Q-SERIE-1` e `Q-SEP-1` continuam sem resposta, e nenhuma mensagem foi enviada.

## 1. Como responder

Cada seção tem uma pergunta fechada e um conjunto de donos. **Sim** aprova a decisão na forma descrita e autoriza registrar a resposta em `docs/lacunas.md` §4; **não** devolve a objeção e mantém o item bloqueado; ausência de resposta mantém bloqueado. A pergunta canônica continua sendo a de `docs/lacunas.md` §4 — este pacote só organiza a leitura e dá o texto do pedido.

## 2. Produto — `OPS-04`, `OPS-05` e `D-06`

| Item | Pergunta (sim/não) | O que aprova | ADR |
|---|---|---|---|
| `OPS-04` | O produto e o `wyd-ops` aprovam o provedor de VPS, o contrato, a região, a rotina de backup e o RPO/RTO da hospedagem `Q-OPS-2` = A? | aplicação de uma decisão já tomada (`ADR-OPS-0002`), com custo e credencial | não |
| `OPS-05` | O produto autoriza criar o board e as novas issues a partir de `docs/github-mapping.md`? | ato de gestão no GitHub, não arquitetura | não |
| `D-06` | O produto confirma a próxima frente depois de `D-05`? | prioridade entre sistemas | não |

**Se ficar aberto:** sem `OPS-04` a produção continua inexistente e o backup e o RPO/RTO seguem indefinidos; sem `OPS-05` o repositório fica com as 40 issues atuais e sem board; sem `D-06` a ordem das frentes fica implícita e cada agente escolhe a sua.

## 3. Arquitetura e `wyd-db` — `Q-SERIE-1` e `Q-SEP-1`

**Pergunta (sim/não):** a arquitetura e o `wyd-db` aprovam a política de `down` da série de 28 migrations (`Q-SERIE-1`: A, só onde há reversão provada; B, nenhum `down`; C, todos com `down`) e o separador dos nomes (`Q-SEP-1`: A, `_`, já usado; B, `-`)?

**O que aprova:** duas decisões de forma da série de migrations; `Q-SERIE-1` é estrutural (política de reversão) e `Q-SEP-1` é nomenclatura. **ADR:** sim para `Q-SERIE-1`, depois do "sim"; não para `Q-SEP-1`.

**Se ficar aberto:** a série segue com 28 arquivos sem política de reversão escrita; qualquer `down` novo é decisão ad hoc, e uma mudança de separador depois da primeira migration publicada quebra o histórico.

## 4. QA e `wyd-ops` — `LNET-01` e `LBOT-03`

**Pergunta (sim/não):** QA e `wyd-ops` autorizam a implementação mínima de servidor e bot e a execução de `docs/planos/medicoes-lnet-01-lbot-03.md` no CI efêmero, para congelar os valores de `LNET-01` e `LBOT-03`?

**O que aprova:** a execução da medição, não a política — `Q-NET-1` = A e `Q-BOT-3` = A já foram decididas. **ADR:** não; falta medição, não decisão estrutural.

**Resposta do dono (2026-11-25): sim** — a implementação mínima e a execução da medição estão autorizadas; falta executar no CI efêmero de `OPS-03.1`. **Enquanto não executada:** os dois números continuam sem valor medido; limites de rede e carga de bots ficam sem base para o plano de teste, e nenhum P50/P99 pode ser citado.

## 5. Jurídico — `LCLI-02` e `LCLI-05`

**Pergunta (sim/não):** o produto e a revisão jurídica confirmam que há liberação do titular para distribuir o cliente e para a classificação jurídica de `LCLI-02` e `LCLI-05`?

**O que aprova:** a liberação de distribuição e a classificação jurídica; a distribuição continua congelada até a resposta. **ADR:** não — direito e contrato, não arquitetura. **Resposta do dono (2026-11-25):** opção A de `LCLI-02` (dados + launcher próprio) e de `LCLI-05` (só dados e peças próprias, distribuição congelada), com o pedido `Q-IP-1`/`Q-IP-2` aprovado para envio; a resposta do titular continua pendente e é ela que fecha as duas.

**Se ficar aberto:** o cliente segue congelado, sem patch publicado nem arquivo redistribuído; o pedido ao titular está redigido e não enviado, e nenhuma decisão técnica substitui essa resposta.

## 6. Resumo por entrada

| Entrada em `docs/lacunas.md` §4 | Dono | Mensagem |
|---|---|---|
| `OPS-04` | produto + `wyd-ops` | §7.1 |
| `OPS-05` | produto | §7.1 |
| `D-06` | produto | §7.1 |
| `Q-SERIE-1` | arquitetura + `wyd-db` | §7.2 |
| `Q-SEP-1` | arquitetura + `wyd-db` | §7.2 |
| `LNET-01`/`LBOT-03` | QA + `wyd-ops` | §7.3 |
| `LCLI-02`/`LCLI-05` | produto + revisão jurídica | §7.4 |

## 7. Mensagens prontas para envio (não enviadas)

Textos curtos derivados de §2–§5 e de `docs/lacunas.md` §4, para copiar e enviar ao dono indicado.
Cada item é o desbloqueio citado em `docs/lacunas.md` §4, que aponta `§7.1`–`§7.4` e não repete a
pergunta. A visão consolidada de todas as decisões pendentes do projeto é a
[pauta de decisões](pauta-de-decisoes-pendentes.md); ela aponta de volta para cada item desta seção. **Nenhuma foi enviada**: não houve issue, e-mail, Slack ou qualquer outro canal. A repetição
da pergunta e do impacto é intencional, porque a mensagem precisa se sustentar fora do repositório; o
registro continua sendo `docs/lacunas.md` §4 e um "sim" dito na conversa só vale depois de registrado lá.

### 7.1 Produto — OPS-04, OPS-05 e D-06

**Para:** dono do produto · **Assunto:** Três aprovações fora dos contratos O1–O5 (sim/não)

*Registro consolidado: [pauta de decisões pendentes](pauta-de-decisoes-pendentes.md).*

> Preciso do seu sim/não em três itens que ficaram fora do pacote O1–O5. **OPS-04** — provedor de VPS,
> contrato, região, rotina de backup e RPO/RTO da hospedagem `Q-OPS-2` = A: a decisão já existe em
> `ADR-OPS-0002`, falta aplicação, custo e credencial. **OPS-05** — autorização para criar o board e as
> novas issues a partir do `github-mapping.md`; o repositório já tem 40 issues e milestones, e nada foi
> criado. **D-06** — qual frente vem depois de `D-05`; a recomendação é manter `wyd-db` até o gate e
> então `wyd-server`/`wyd-net`.
>
> Se ficar aberto: sem `OPS-04` a produção continua inexistente, sem backup nem RPO/RTO; sem `OPS-05`
> o board não existe e as fatias restantes não têm onde morar; sem `D-06` a ordem das frentes fica
> implícita. Pode responder por item — um "sim" parcial também serve.

### 7.2 Arquitetura e wyd-db — Q-SERIE-1 e Q-SEP-1

**Para:** arquitetura + equipe do `wyd-db` · **Assunto:** Política de `down` e separador das migrations (sim/não)

*Registro consolidado: [pauta de decisões pendentes](pauta-de-decisoes-pendentes.md).*

> A arquitetura e o `wyd-db` aprovam duas decisões de forma da série de 28 migrations? **Q-SERIE-1** —
> a política de `down`: A) só onde há reversão provada, B) nenhum `down`, C) todos com `down`; a
> recomendação é A, e isso exige ADR depois do sim, porque é política de reversão. **Q-SEP-1** — o
> separador dos nomes: A) `_`, já usado, ou B) `-`; sem impacto estrutural e sem ADR.
>
> Se ficar aberto: a série continua com 28 arquivos sem política de reversão escrita, cada `down` novo
> vira decisão ad hoc, e trocar o separador depois da primeira migration publicada quebra o histórico.

### 7.3 QA e wyd-ops — LNET-01 e LBOT-03

**Para:** QA + equipe do `wyd-ops` · **Assunto:** Autorização de execução das medições `LNET-01` e `LBOT-03` (sim/não)

*Registro consolidado: [pauta de decisões pendentes](pauta-de-decisoes-pendentes.md).*

> QA e `wyd-ops` autorizam a implementação mínima de servidor e bot e a execução de
> `docs/planos/medicoes-lnet-01-lbot-03.md` no CI efêmero, para congelar os valores de `LNET-01` e
> `LBOT-03`? A política já está decidida (`Q-NET-1` = A, `Q-BOT-3` = A); falta a medição, não ADR.
>
> Se ficar aberto: os dois números continuam sem valor medido, limites de rede e carga de bots ficam sem
> base para o plano de teste, e nenhum P50/P99 pode ser citado.

### 7.4 Jurídico — LCLI-02 e LCLI-05

**Para:** produto + revisão jurídica · **Assunto:** Liberação de distribuição e classificação jurídica do cliente (sim/não)

*Registro consolidado: [pauta de decisões pendentes](pauta-de-decisoes-pendentes.md).*

> O produto e a revisão jurídica confirmam que há liberação do titular para distribuir o cliente e para
> a classificação jurídica de `LCLI-02` e `LCLI-05`? É direito e contrato, não arquitetura, então não
> gera ADR. O pedido ao titular está redigido e não enviado.
>
> Se ficar aberto: o cliente continua congelado, sem patch publicado nem arquivo redistribuído, e
> nenhuma decisão técnica substitui essa resposta.

## 8. Limites

- Não cria ADR, issue, board, commit, push ou deploy, e não envia mensagem.
- `docs/lacunas.md` §4 continua sendo o registro único; este pacote só organiza a leitura por dono.
- Os contratos O1–O5 continuam no [pacote O1–O5](pacote-aprovacao-o1-o5.md); este documento não os reabre.
