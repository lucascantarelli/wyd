> Pergunta: "como o `wyd-docs` prova montagem, gate e publicação sem inventar página?"
> Público: QA e desenvolvedor do `wyd-docs`
> Dono: equipe do wyd-docs
> Base: `AGENTS.md` §8; `wyd-docs-1.0.0.md` §7; `wyd-protocol/docs/planos/plano-de-testes.md` (molde, não conteúdo)
> Revisado em: 2026-11-27

**Estado:** proposto (2026-11-25; testes de fixture da camada de gate implementados em 2026-11-26).

## Pirâmide e alvos

| Nível | Alvo | O que prova | Dado |
|---|---|---|---|
| Unitário | `preparar`, `gate` | montagem, link/âncora, regras | funções puras e fixtures |
| Contrato | `tests/fixtures.py` | alvos, capacidades, contagens, censos, hoje, lacunas, skills | árvore mínima |
| Integração | `make gate` | build completo + links | container Linux |
| Sistema | `tests/sistema/` | publicação, rollback | GitHub Pages |
| E2E | (n/a) | — | — |
| Propriedade | (n/a) | — | — |

## O que cada teste precisa provar

1. **Montagem:** preserva caminho relativo; não publica conteúdo interno.
2. **Gate:** link quebrado, âncora inexistente, nav ausente, título ausente, redirect inválido, sintaxe 1.x reprova.
3. **Contagens:** o gate mede `Tab.`/`CREATE TABLE`, arquivos da série, páginas do site, páginas de issue e fatias, planos com CI e censo por estado (`aceito`, `proposto`, sem linha, fontes) em `wyd-docs-1.0.0.md` §9.1, e confere as citações dos dashboards: `N tabelas`/`N arquivos` da série (número não classificado em `OUTROS_DONOS` reprova, e o excluído sai em `--details`), `N páginas` só na frase do gate do site, `N páginas montadas`, contagem por pasta, `(N sistemas)`, `docs/modulos.md em N sistemas` e inventários (que exigem `planejadas`/`previsto`). "0 órfãos" reprova porque nenhum gate publica alcançabilidade, e número do board do GitHub não é medido localmente. Soma divergente reprova.
4. **Censos:** citação divergente de `N atos`, `N páginas aceitas`, `N células pendentes` e `N Q-BLK abertas` reprova. Em `N atos` entra também `**N** atos` sem “de aceite” e o mesmo número em texto corrido, sem negrito; o negrito não muda a regra. Recorte de pasta (N atos de `docs/`) é contagem da pasta, não do total. `12`, `27` e `37` ficam fora só quando a leva qualifica o próprio número, colada a ele e sem outra oração no meio — `leva` em outra parte da frase não isenta. Seção datada antes da medição, data na própria frase e marca de registro histórico isentam a citação, com o motivo impresso em `--details`.
5. **Capacidades:** citação de censo (`X de 39`, `N capacidades`, `N capacidades sem execução`) divergente de `capacidades.md` §7 reprova; recorte de `CAP-nn`, seção datada antes da medição e data na própria frase ficam fora; data em outra frase do mesmo parágrafo não isenta.
6. **Build:** roda em Linux com Python 3.15; MkDocs 2.0.dev6.
7. **Publicação:** workflow com permissões mínimas; rollback validado.
8. **Redirect:** origem duplicada reprova; destino inexistente reprova.
9. **Hoje:** número colado ao advérbio que declara a unidade do censo (`N atos`, `N páginas`, `N células`, `N lacunas`, `N capacidades`, `N Q-BLK` ou `N de 39`) é conferido contra `censos`, `lacunas` §1 e `capacidades` §7, e a unidade da `contagens` (`N arquivos`, `N tabelas`, `N fontes`, `N páginas do site`) contra a série, o censo de fontes e as páginas do site — `arquivos`/`tabelas` ficam fora quando a cláusula mede outra coisa (contagem vizinha na mesma enumeração, lista `content`/`game`/… em crase, caminho `deprecated/`/`target/`) e exigem a marca da série documentada (`Tab.`, `DDL`, `checklist`, `plano-migrations.md` ou `migrations-checklist.md`), porque `série` e `migrations` também nomeiam a série materializada (28 dos 34 arquivos) e `a série materializada tem hoje 28 arquivos` mede outro conjunto; `fontes` exige `censo` na cláusula e `páginas do site` é medida própria do site, sem guarda de enumeração; seção datada antes da medição, data na frase e marca de registro isentam com o motivo em `--details`; número sem unidade colada (`hoje **138**`) entra quando a cláusula cita a unidade do censo ou o dono (`§6.6.22.1` para atos, `lacunas.md` §1, `capacidades.md` §7), e fica fora quando a cláusula cita outro dono, há mais de uma unidade candidata, o número é razão de outra medida (`N das 40`) ou é identificador em crase — a atribuição sai em `--details`. O check fica separado do `censos` (decisão de 2026-11-27): na medição daquele dia, das 16 citações conferidas, o `censos` sozinho avaliaria 3 (só 1 das 11 vigentes); 6 das 16 dependiam da atribuição contextual e 9 eram de unidades que o `censos` não lê (`contagens`, `lacunas`, `capacidades`) — 2 delas também contextuais —, então fundir exigiria importar os três donos e a atribuição contextual. Critério da medição: citação do `hoje` casada com citação do `censos` no mesmo arquivo, mesma linha física, mesma unidade e mesmo valor.
10. **Lacunas:** o placar é medido dos ledgers `planejamento-1.0.0/*/lacunas.md`, com id deduplicado entre registros e estado lido do marcador ou do título da seção; toda lacuna fechada precisa citar artefato que exista e trate do item; número em negrito na linha fechada com `Refletida em` confere com a medição da seção citada; o placar publicado (`lacunas medidas: N = F fechadas + B bloqueadas`), as somas `N = a + p + f + e` e as tabelas por sistema e `Total` têm de bater com a medição; o censo do `wyd-db` (39 = 36 fechadas + 3 externas) é conferido na prosa e no atalho `0/0/F/E`, com data na frase ou seção datada isentando.
11. **Alvos:** só o comando de teste é asserção dura — `cargo test --test <alvo>` sem seletor, com `-p wyd-db` ou `--manifest-path crates/wyd-db/Cargo.toml`, inclusive linha quebrada com `\`; tabela com cabeçalho `Alvo` e prosa na forma `alvo`/`alvos` saem como detalhe. O nome passa se existe `[[test]]` no `Cargo.toml` ou se `docs/guia-teste/index.md` §1 o declara `planejado` (lista derivada da página dona); comando que cita alvo planejado exige marca de futuro na própria linha (`- [ ]`, `planejad`, `previst`, `ainda não`); `--test` fora de comando, outro crate (por `-p` ou `--manifest-path`, mesmo com alvo declarado no `wyd-db`) e caminho dinâmico ficam fora, são impressos como `ignorado` em `--details` e não contam como citação conferida.

## Isolamento

- Testes Python com `unittest`; `make gate`/`gate.sh` os executa antes do `gate full`, e `make test-gate` é o atalho isolado. Container Linux por build.
- O teste paralelo-vs-sequencial exige `fork`; é executado no Linux e no macOS, e o skip é apenas do Windows (sem `fork`).
- Fixtures por montagem real; nenhum número inventado.
- Proibido `sleep` como sincronização.
- Cobertura mínima de 85% por capacidade (onde aplicável).

## Comandos

```bash
make gate-fast                 # ciclo curto: layout, alvos, skills, lacunas, contagens, censos, capacidades, hoje
make gate                      # testes dos verificadores + build, links e todos os domínios
make test-gate                 # atalho isolado para os testes de fixture
cd wyd-docs && python -m gate fast --json
```

## CI, cobertura e monitoramento

- **CI:** `make gate` e o job de documentação de `.github/workflows/docs.yml` rodam os testes dos verificadores, build, links, âncoras, contagem e censo; falha em qualquer um reprova o PR. O job chama `wyd-docs/gate.sh`, que é o mesmo ponto do container.
- **Cobertura:** A cobertura é de página, link, âncora e censo; o total e o censo vigentes são os de `wyd-docs-1.0.0.md` §9.1, não repetidos aqui. Não há percentual de linha para o site.
- **Monitoramento:** Duração do build, páginas publicadas, fontes fora do censo, link quebrado e status da publicação. O gate já falha em link, âncora, contagem e censo; não há SLO de disponibilidade antes de existir deploy.
- **Critério verificável:** o comando de teste e o gate de documentação exit 0; nenhuma falha é mascarada por `| tail`, `--no-fail-fast` ou `SKIP` silencioso.

## Critério de saída da `1.0.0`

Build e verificação verdes no CI; publicação automática em `develop`; URL estável; nenhum conteúdo interno no artefato; rollback validado. `LPDOC-01` fechada. `LPDOC-12` (Windows) segue aberto como decisão.
