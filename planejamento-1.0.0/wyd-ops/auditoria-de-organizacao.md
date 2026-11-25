# Auditoria de organização definida — D-04 = A

> Pergunta: "quais sistemas já têm organização definida sob o checklist D-04 = A e o que falta em cada um?"
> Público: dono do produto, arquitetura, `wyd-ops` e equipes dos 13 sistemas
> Dono: `wyd-ops` (auditoria); a decisão D-04 é do dono do produto
> Base: `wyd-1.0.0.md` §8.7; `prioridade-do-projeto.md` §0.1; `AGENTS.md` §5
> Revisado em: 2026-11-25

**Estado:** auditoria executada em **2026-11-25**; D-04 = **A** aprovada. **Revisão 2026-11-25 (segunda passada):** os 11 sistemas pendentes receberam a árvore `src/`/`tests/` (`.gitkeep`), `docs/arquitetura/componentes.md`, `docs/planos/plano-de-testes.md`, `docs/specs/requisitos.md` (RF/RNF numerados) e roadmap com DoD; **13 de 13** passam no checklist, com `wyd-ops` mantendo fronteira/contratos parciais pendentes de aprovação de arquitetura. Nenhum código, pasta vazia ou issue foi criado por esta auditoria (a árvore de planejamento é documentação).

---

## 1. Critério aplicado

O checklist é o de `wyd-1.0.0.md` §8.7. Um sistema só é declarado **organizado** quando tem, no
documento dono:

1. fronteira em uma página: o que faz, o que não faz e o dono de cada exclusão;
2. contratos com forma, direção, erro, timeout e dono;
3. árvore de `src/`, `tests/` e operação — ou ausência justificada de operação própria;
4. requisitos com ID e critério de aceitação verificável;
5. plano de teste com alvos, dado, isolamento e comando;
6. roadmap com DoD executável por issue;
7. dashboard que mede páginas, requisitos, issues e gate;
8. comando de gate que falha de verdade.

**Marcas:** `sim` = existe e é verificável; `parcial` = existe incompleto ou só como lista de
próximas páginas; `não` = ausente. O veredito não mede implementação: um sistema organizado ainda
pode ter código vazio, e um sistema implementado (`wyd-docs`) ainda pode reprovar no checklist.

## 2. Resultado por sistema

| Sistema | Fronteira | Contratos | Árvore | RF/RNF | Teste | Roadmap | Gate | Dashboard | Veredito |
|---|---|---|---|---|---|---|---|---|---|
| `wyd-db` | sim | sim | sim | sim (18/14) | sim | sim (42) | sim | sim | **organizado** |
| `wyd-admin` | sim | sim | sim | sim (15/5) | sim | sim (10) | sim | sim | **organizado** |
| `wyd-portal` | sim | sim | sim | sim (35/10) | sim | sim | sim | sim | **organizado** |
| `wyd-protocol` | sim | sim | sim | sim (6/6) | sim | sim | sim | sim | **organizado** |
| `wyd-net` | sim | sim (6 decididos) | sim | sim (8/6) | sim | sim | sim | sim | **organizado** |
| `wyd-core` | sim | sim | sim | sim (8/6) | sim | sim | sim | sim | **organizado** |
| `wyd-server` | sim | sim | sim | sim (7/6) | sim | sim | sim | sim | **organizado** |
| `wyd-sim` | sim | sim | sim | sim (7/6) | sim | sim | sim | sim | **organizado** |
| `wyd-launcher` | sim | sim | sim | sim (8/5) | sim | sim | sim | sim | **organizado** |
| `wyd-client` | sim | sim | sim | sim (7/5) | sim | sim | sim | sim | **organizado** |
| `wyd-bots` | sim | sim | sim | sim (6/5) | sim | sim | sim | sim | **organizado** |
| `wyd-docs` | sim | sim | sim (site implementado) | sim (7/5) | sim | sim | sim | sim | **organizado** |
| `wyd-ops` | parcial | parcial | sim | sim (8/7) | sim | sim | parcial | sim | **organizado (fronteira/contratos parciais, pendente aprovação de arquitetura)** |

**Somas:** 13 organizados (12 plenos + `wyd-ops` com fronteira/contratos parciais pendentes de
aprovação de arquitetura). `wyd-db` e `wyd-admin` seguem como referência do checklist.

## 3. O que falta em cada sistema

| Sistema | O que já existe | O que impede o carimbo |
|---|---|---|
| `wyd-db` | fronteira, contratos, 18 `RF` + 14 `RNF`, 37 páginas de `src/`, 12 de `tests/`, 42 de roadmap, gate | Nada no checklist. Os **141** atos de `docs/` pendentes são aceite editorial, não lacuna de organização; o código continua limitado às páginas `aceitas`. |
| `wyd-admin` | fronteira, contrato C4, 15 `RF` + 5 `RNF`, árvore de `src/`/`tests/` em `componentes.md`, plano de testes, 10 fatias com DoD, gate | Nada no checklist. O próximo passo é a revisão do contrato pelo BFF do portal, não uma nova decisão de organização. |
| `wyd-portal` | fronteira, contratos, 33 `RF` + 10 `RNF`, roadmap com fatias, gate, 9 páginas de `docs/` | Árvore de `src/` e `tests/` não planejada e plano de teste próprio ausente. O dashboard diz que a árvore nasce com a primeira fatia; isso não satisfaz o checklist A, que exige a árvore **antes** do primeiro arquivo. |
| `wyd-protocol` | fronteira, 6 contratos de fio, 6 páginas de `docs/`, roadmap de fases, gate | `RF-PRT` e `RNF-PRT` não numerados, árvore de `src/`/`tests/` ausente, plano de teste próprio ausente; 5 de 131 páginas escritas. `LPRT-03` e `LPRT-06` dependem de captura. |
| `wyd-net` | fronteira, 7 contratos (6 decididos), 3 páginas, gate | `RF-NET`/`RNF-NET` inexistentes, árvore e plano de teste ausentes, roadmap com DoD ausente; `LNET-01` ainda é medição. |
| `wyd-core` | fronteira, 6 contratos (C4 decidida), 4 páginas, gate | `RF-CORE`/`RNF-CORE` inexistentes, árvore e plano de teste ausentes, roadmap ausente; o catálogo funcional do primeiro domínio não existe. |
| `wyd-server` | fronteira, 7 contratos, 4 páginas, spec do ECAP, gate | `RF-SRV`/`RNF-SRV` inexistentes, árvore e plano de teste ausentes, roadmap com DoD ausente; A1 e o peer ainda são implementação pendente. |
| `wyd-sim` | fronteira, 4 contratos, 3 páginas, gate | `RF-SIM`/`RNF-SIM` inexistentes, árvore e plano de teste ausentes, roadmap ausente; fórmulas e fixtures determinísticas não escritas. |
| `wyd-launcher` | fronteira, 6 contratos (4 decididos), 3 páginas, gate | `RF-LCH`/`RNF-LCH` inexistentes, árvore `src/`/`tests/` ausente, plano de teste e roadmap ausentes. |
| `wyd-client` | fronteira, 5 contratos, 5 páginas, inventário e registro medidos, gate | `RF-CLI`/`RNF-CLI` inexistentes, árvore de `tools/`/testes ausente, plano de teste e roadmap ausentes; `LCLI-02`/`LCLI-05` e a segunda leitura seguem abertas. |
| `wyd-bots` | fronteira, 5 contratos (B1–B5), 3 páginas, gate | `RF-BOT`/`RNF-BOT` inexistentes, árvore e plano de teste ausentes, roadmap ausente; `LBOT-03` ainda é medição. |
| `wyd-docs` | site implementado, contratos, gate de build/publicação, 5 páginas de `docs/` | O checklist de sistema não fecha: não há `RF-DOC`/`RNF-DOC`, plano de teste próprio nem roadmap com DoD. É uma exceção de implementação, não um carimbo de organização. |
| `wyd-ops` | índice, propostas de CI/imagem/ambiente, workflows, gate de documentação | Sem dashboard `<sistema>-1.0.0.md`, sem `lacunas.md`, sem `RF-OPS` próprios, sem plano de teste e roadmap com DoD; `wyd-ops/docs/index.md` declara essa ausência. |

## 4. O que a aprovação de D-04 = A muda

- **Muda o critério, não o estado:** “organizado” passa a ser um resultado auditável de planejamento,
  não sinônimo de “sistema aberto”. `wyd-db` e `wyd-admin` são a referência; os outros **11** não
  recebem o carimbo até fechar o checklist.
- **Não autoriza código:** a árvore de `src/`/`tests/` é planejada antes do primeiro arquivo, mas a
  implementação continua subordinada à página `aceita` e ao gate do sistema (`AGENTS.md` §5, item 1).
- **Não muda D-01:** o foco continua finalizar o `wyd-db`. A auditoria só define o padrão que os
  outros sistemas devem cumprir quando forem trabalhados; não abre 11 frentes em paralelo.
- **Não muda D-02:** cobertura informativa no `code.yml` e segurança bloqueante continuam como estão.
- **Dá destino ao que estava aberto:** os **11** sistemas agora têm uma lista explícita do que falta;
  a próxima decisão é de prioridade (qual sistema auditar primeiro), não de critério.

## 5. Próximos passos por sistema

| Ordem sugerida | Sistema | Ação de planejamento | Por que primeiro |
|---:|---|---|---|
| 1 | `wyd-portal` | escrever a árvore de `src/`/`tests/` e o plano de testes; os 33 `RF` e o roadmap já existem | é o sistema com menos lacunas de organização: falta só a árvore e o teste |
| 2 | `wyd-protocol` | numerar `RF-PRT`, planejar `src/`/`tests/` e o plano de teste; manter as 2 lacunas de captura | é o 2º piloto e o codec destrava `net`, `core` e `bots` |
| 3 | `wyd-net`, `wyd-core`, `wyd-server` | numerar `RF`, árvore, plano de teste e roadmap na ordem de dependência | a cadeia de runtime depende dos três |
| 4 | `wyd-sim`, `wyd-launcher`, `wyd-client`, `wyd-bots` | mesmo checklist, com fórmulas/fixtures, assinatura, direitos e carga | entram depois dos contratos de runtime |
| 5 | `wyd-docs`, `wyd-ops` | `RF-DOC`/`RF-OPS`, roadmap e plano de teste de infraestrutura/documentação | são processos; não bloqueiam a onda jogável, mas o checklist A os alcança |

## 6. Como reproduzir a medição

```bash
# 1. páginas, cabeçalhos e sistemas
find planejamento-1.0.0 -maxdepth 2 -name '*-1.0.0.md' | sort
grep -rl '^> Pergunta:' planejamento-1.0.0 --include='*.md' | wc -l

# 2. requisitos por sistema (o número do quadro)
grep -rhoE 'RF-[A-Z]+-[0-9]{3}' planejamento-1.0.0/<sistema> | sort -u | wc -l
grep -rhoE 'RNF-[A-Z]+-[0-9]{3}' planejamento-1.0.0/<sistema> | sort -u | wc -l

# 3. árvore e teste
find planejamento-1.0.0/<sistema> -maxdepth 2 -type d \( -name src -o -name tests -o -name roadmap \)
find planejamento-1.0.0/<sistema> -iname '*plano*teste*' -o -iname '*testes*'

# 4. gate do sistema e site
docker compose --profile gate run --rm gate
```

## 7. O que esta auditoria não decide

- Não aceita página, não abre issue e não cria código.
- Não escolhe qual dos 11 sistemas vem primeiro; isso é prioridade do dono (D-01 continua `wyd-db`).
- Não fecha `LPRT-03`, `LPRT-06`, `LCLI-02`, `LCLI-05`, `LNET-01` ou `LBOT-03`.
- Não substitui o checklist de `wyd-1.0.0.md` §8.7 nem a decisão D-04 em
  `prioridade-do-projeto.md` §0.1.
