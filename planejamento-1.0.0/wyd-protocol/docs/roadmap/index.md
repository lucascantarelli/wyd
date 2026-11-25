> Pergunta: "por onde começo o roadmap do `wyd-protocol` e o que já existe dele?"
> Público: quem mantém o roadmap, escreve issue e opera o board
> Dono: equipe do wyd-protocol
> Base: `wyd-protocol-1.0.0.md` §4, §5.8, §7 e §9; `lacunas.md` §1
> Revisado em: 2026-11-20

**Estado:** proposto (2026-11-20)

Esta é a porta de entrada do roadmap. Ela diz **o que existe, o que ainda é teto e onde está o
fatiamento** — não decide escopo, não escreve issue e não conta issue: o escopo e o gate de cada
versão são de `wyd-protocol-1.0.0.md` §9.1, e o total de issues é de §4 do plano diretor.

## 1. Mapa das páginas

| Página | Responde | Estado |
|---|---|---|
| `index.md` (esta) | por onde começo e o que já existe | escrita, 2026-11-20 |
| [fases](fases.md) | em que versão entra cada issue e o que a fatia precisa provar | escrita, 2026-11-20 |
| [`WYDPRT-0.1.0`](WYDPRT-0.1.0.md) a [`WYDPRT-1.0.0`](WYDPRT-1.0.0.md) | o que cada fatia entrega, do que depende e como prova que terminou | escritas, uma por versão (2026-11-25) |

As páginas de fatia existem, uma por versão, com DoD verificável na ordem de §5.8 e §9 do plano
diretor. Faixa de número de issue continua não reservada aqui.

## 2. O que existe hoje

- este índice e [fases](fases.md), escritos em 2026-11-20;
- a série de versões `0.1.0` → `1.0.0`, com escopo e gate em §9.1 do plano e o recorte das issues em
  `fases.md` §3;
- a regra de numeração `WYDPRT-nnn`, em §7 do plano, aplicada quando a página é escrita.

Também existe uma página por fatia (`WYDPRT-<versão>.md`), com o detalhe que `fases.md` cita e não
repete. Não existe: issue criada no GitHub, número `WYDPRT-nnn` alocado e data de entrega.

## 3. O que ainda é teto

| O que | Onde está o dono | Por que ainda não existe |
|---|---|---|
| a issue `WYDPRT-nnn` criada a partir da página da fatia, com DoD executável | §4 e §5.8 do plano diretor | a fatia está escrita; o número e a issue nascem na criação |
| a distribuição numérica das issues por versão | `fases.md` §4 e §9.1 do plano | nasce quando as páginas forem escritas; nenhum número é fixado antes |
| o total de issues do roadmap | §4 do plano diretor | esta página não reconta nem redistribui |

## 4. Como uma issue nasce

1. a fatia é a de `fases.md`: uma issue pertence a exatamente uma versão, a que serve o gate dela;
2. o DoD é escrito com critério verificável — o gate da versão está em §9.1, o da issue não existe
   sem ele;
3. o número `WYDPRT-nnn` é alocado na escrita da página, sequencial, na ordem de §7 — nunca antes;
4. a página só entra na fila quando o DoD é executável (§5.8 do plano);
5. criar a issue no GitHub é ato à parte, que exige pedido explícito (`AGENTS.md` §5, item 9).

## 5. Onde está o fatiamento por versão

O recorte das issues por versão, com as regras que ligam issue e fatia, está em
[fases](fases.md) §2–§3. O escopo e o gate de cada versão continuam em §9.1 do plano diretor, que é
o dono deles; este índice não repete nenhum dos dois.

## 6. O que esta página não faz

- não cria issue, label, milestone nem sprint;
- não aloca `WYDPRT-nnn` nem reserva faixa;
- não escreve DoD nem define prioridade;
- não conta issue, não muda o total do roadmap e não fixa data;
- não substitui §9.1 (escopo e gate) nem [fases](fases.md) (fatia por versão).

## Páginas de issue por fatia

Medido em 2026-11-25: **10** páginas, uma por fatia, no formato de `WYDDB-nnn`
(estado, dependências, DoD observável e bloco de skills). O número sequencial de cada issue é
atribuído na criação pelo `github-mapping.md`; a chave aqui é a fatia.

| Issue | Título | Estado |
|---|---|---|
| [`WYDPRT-0.1.0`](WYDPRT-0.1.0.md) | fatia 0.1.0 do fio | proposto |
| [`WYDPRT-0.2.0`](WYDPRT-0.2.0.md) | fatia 0.2.0 do fio | proposto |
| [`WYDPRT-0.3.0`](WYDPRT-0.3.0.md) | fatia 0.3.0 do fio | proposto |
| [`WYDPRT-0.4.0`](WYDPRT-0.4.0.md) | fatia 0.4.0 do fio | proposto |
| [`WYDPRT-0.5.0`](WYDPRT-0.5.0.md) | fatia 0.5.0 do fio | proposto |
| [`WYDPRT-0.6.0`](WYDPRT-0.6.0.md) | fatia 0.6.0 do fio | proposto |
| [`WYDPRT-0.7.0`](WYDPRT-0.7.0.md) | fatia 0.7.0 do fio | proposto |
| [`WYDPRT-0.8.0`](WYDPRT-0.8.0.md) | fatia 0.8.0 do fio | proposto |
| [`WYDPRT-0.9.0`](WYDPRT-0.9.0.md) | fatia 0.9.0 do fio | proposto |
| [`WYDPRT-1.0.0`](WYDPRT-1.0.0.md) | fatia 1.0.0 do fio | proposto |
