> Pergunta: "onde um defeito de página ou um risco do `wyd-db` é registrado, e quem o mantém?"
> Público: equipe do `wyd-db` e quem revisa página do sistema
> Dono: equipe do `wyd-db`
> Base: `docs/convencoes.md` §"Quando a página está errada"; `plano-migrations.md` §8; `lacunas.md` §1
> Revisado em: 2026-11-20

**Estado:** proposto (2026-11-20)

Este é o **dono do registro de defeito de página e de risco** do `wyd-db`. A convenção manda o defeito
para cá; antes desta página ele ia para a issue do roadmap que cobria a página, e o risco não tinha
onde morar. Criada por decisão do dono em 2026-11-20, que fecha `P3` (`lacunas.md` §3).

Cada linha responde quatro coisas: qual é o risco, o que ele impede, quem age e **como verificar** que
deixou de existir. Risco não é lacuna: lacuna é decisão pendente com dono e prazo em `lacunas.md`;
risco é exposição conhecida, com mitigação em andamento. Os dois não se substituem.

## 1. Como registrar

| Tipo | Quando | Formato da linha |
|---|---|---|
| Defeito de página | página afirma o que a fonte não sustenta, link quebrado, placeholder, segredo | página, trecho, fonte correta, quem encontrou, data, estado (`aberto`/`corrigido`) |
| Risco | exposição que pode impedir o plano de fechar | `R-n`, risco, efeito, exposição, dono, mitigação, como verificar |

Defeito corrigido não some do registro: a linha muda para `corrigido` com a data. Risco encerrado sai
da tabela de §2 e fica em §3, com a evidência que o encerrou.

## 2. Riscos abertos

| ID | Risco | O que impede | Exposição | Dono | Mitigação | Como verificar |
|---|---|---|---|---|---|---|
| `R-2` | ferramentas de catálogo não conhecem schema qualificado, `INET` nem view | tabela some do catálogo sem erro e o gate dá verde falso (`plano-migrations.md` §1) | alta | arquitetura | PR de ferramentas liberado em 2026-11-20, mas **não é pré-requisito do DDL**; até ele rodar, os gates de §7 são conferência manual | `banco-parse.mjs` lê `game.account` e o catálogo conta a tabela |
| `R-3` | CI de código é stub: publica `version`/`commit` vazios e não roda build nem teste | nenhum gate de código existe hoje; verde não prova nada (`wyd-1.0.0.md` §3, `D3`) | alta | operação | substituir o stub por workflows reutilizáveis na Fase 4 | run passa a publicar `version`/`commit` e a falhar quando o teste falha |
| `R-4` | captura do cliente 7662 não existe para `LPRT-03` e `LPRT-06` (`LPRT-09` fechou por decisão em 2026-11-23) | contrato de handshake e encoding fica em `H` (`wyd-protocol/lacunas.md` §1) | média | frente `wyd-client` | procedimento de captura em `wyd-protocol/docs/goldens/como-capturar.md` | as duas linhas passam a `fechada` com captura datada |
| `R-5` | board existe, mas nenhuma issue foi criada | roadmap de 40 issues do `wyd-db` não é executável (`wyd-1.0.0.md` §3, `D5`) | média | operação | criar issue só depois de a página dona existir, com DoD | board mostra o mesmo número de itens do roadmap |
| `R-6` | o dado de origem dos 6 seeds não está verificado | `0016`–`0018`, `0020`, `0023` e `0027` ficam bloqueados (`migrations-checklist.md` §3) | média | arquitetura + curadoria de conteúdo | conferir contagem e valor na fonte; os arquivos são escritos à mão, sem gerador (decisão de 2026-11-21, `plano-migrations.md` §4) | cada seed cita a fonte conferida e a contagem medida antes de a página fechar |
| `R-7` | catálogo funcional não existe | a `1.0.0` não tem como ser medida por cobertura (`wyd-1.0.0.md` §1.5) | alta | produto + arquitetura | divisão `wyd-core`/`wyd-sim`/`wyd-db` e molde de entrada propostos em `wyd-ops/proposta-versoes-e-cobertura-funcional.md` §7 | §1.5 do panorama deixa de dizer "não medido" |

## 3. Riscos encerrados

`R-1` **encerrado em 2026-11-21 e recontado em 2026-11-22:** as 109 tabelas da série têm coluna desenhada
e aprovada — **109 de 109** com DDL fechado em `ddl-confirmado.md` §1, depois do "sim" do dono aos lotes
§5.23/§5.25/§5.26/§5.28, à §5.29 e às §5.33/§5.34 (2026-11-22). Os 6 arquivos que seguem `bloqueada` são seeds, presos no dado de origem
(`plano-migrations.md` §4), não desenho de coluna.

Risco encerrado entra aqui com a data e a evidência que o encerrou, em vez de ser apagado.

## 4. Defeitos de página

Nenhum registrado desde a criação desta página (2026-11-20). Defeito encontrado em revisão entra aqui
antes de virar issue.

## 5. O que esta página não decide

- Não decide lacuna: estado, dono e prazo continuam em `lacunas.md`.
- Não decide ordem de trabalho: a fila é de `proximos-passos.md` e a ordem da série é de
  `plano-migrations.md` §3.
- Não mede nada: contagem de tabela é de `plano-migrations.md` §3, e DDL fechado é de
  `ddl-confirmado.md` §1.
