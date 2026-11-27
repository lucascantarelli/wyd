# Estratégia de testes — planejamento transversal

> Pergunta: "quais padrões de teste, fixtures, mocks, stubs, cobertura e integração com CI o projeto precisa para provar capacidade, não só linha?"
> Público: Desenvolvimento, QA, arquitetura e agentes
> Dono: frente `wyd-ops` (padrão transversal); cada sistema é dono de seu `plano-de-testes.md`
> Base: `AGENTS.md` §5 e §8; `wyd-ops/docs/specs/requisitos.md` (`RF-OPS-001`, `RF-OPS-003`, `RF-OPS-004`, `RF-OPS-007`, `RNF-OPS-005`); `wyd-ops/docs/arquitetura/componentes.md`; `wyd-ops/docs/planos/plano-de-testes.md`; `wyd-ops/auditoria-de-organizacao.md` §2–§3
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Planejamento; nenhum teste, fixture, mock, stub ou código foi criado por este documento.

## 1. Escopo e dono

Este documento define o padrão transversal de testes. Ele não substitui o `plano-de-testes.md` de cada sistema, que continua dono de alvos, dado, isolamento e comando. Ele também não implementa teste, não cria pasta de teste e não fecha lacuna de organização por si só.

## 2. Pirâmide e níveis

| Nível | O que prova | Dono típico | Dado |
|---|---|---|---|
| Unitário | função pura, regra, parsing, validação | sistema dono | dado construído em memória |
| Contrato | fronteira entre sistemas, schema, pacote, API | sistema dono + consumidor | golden, fixture ou contrato versionado |
| Integração | colaboração entre componentes e banco/ambiente | sistema dono | ambiente efêmero e seed controlado |
| Sistema | fluxo ponta a ponta, deploy, rollback, carga | `wyd-ops` + sistema dono | ambiente descartável por execução |
| E2E | portal e cliente em navegador real | `wyd-portal`, `wyd-client` | Playwright |
| Propriedade | invariantes e casos aleatórios quando houver regra determinística | sistema dono | gerador reproduzível |

Regra: teste de linha não substitui capacidade provada. A matriz de rastreabilidade do `wyd-db` é o modelo de como ligar requisito, página, capacidade e teste.

## 3. Alvos por sistema (referência)

| Sistema | Plano de teste | Estado medido |
|---|---|---|
| `wyd-db` | `wyd-db/docs/guia-teste/` e `wyd-db/docs/planos/plano-de-testes.md` | 20 testes de sistema (13 schema + 7 porta) e 9 de contrato sem banco; **20 de 39** capacidades cobertas (2026-11-27) |
| `wyd-ops` | `wyd-ops/docs/planos/plano-de-testes.md` | planejado; nenhum teste escrito |
| demais 11 sistemas | `docs/planos/plano-de-testes.md` por sistema | planejado; 0 testes implementados |

Fonte: `wyd-ops/auditoria-de-organizacao.md` §2–§3.

## 4. Centralização de fixtures, mocks e stubs

- Fixtures e factories moram em um ponto central por sistema (`tests/common/` ou equivalente), não espalhadas em cada teste.
- Dados de teste têm factory; proibido ID mágico, `sleep` como sincronização e estado global entre testes.
- Mocks/stubs são usados para isolar fronteira, não para esconder regra de negócio.
- Contratos entre sistemas usam golden ou fixture versionada; mudança de contrato exige atualização explícita do golden.
- Dado externo (captura, cliente, seed) não entra no repositório como segredo; entra como artefato documentado e curado.
- Cada teste declara o que prova, o dado usado e o comando que o executa.

## 5. Isolamento e dados

- Ambiente efêmero por execução em CI.
- Banco de teste com seed aplicado e descartado ao fim.
- Proibido dependência de ordem entre testes.
- Proibido estado global mutável compartilhado.
- Teste que falha por ambiente instável deve ser corrigido ou removido; não pode virar `SKIP` silencioso.
- Evidência de teste é saída de comando, não afirmação de intenção.

## 6. Cobertura e gates

- Cobertura mínima de 85% por sistema, medida no gate do sistema.
- No `code.yml`, a cobertura de `wyd-db` é informativa até o sistema declarar o gate fechado (`D-02 = B`); o `wyd-launcher` já aplica o piso de 85% de linha do plano dele (`wyd-launcher/docs/planos/plano-de-testes.md` §Cobertura) desde 2026-11-27.
- `cargo audit` e `cargo deny` são bloqueantes desde já.
- Para Python do site (`mkdocs==2.0.dev6`), usar `pip-audit`/hash em item separado; não entra em `cargo audit`.
- Gate de documentação (`docker compose --profile gate run --rm gate`) continua obrigatório e independente de cobertura.
- Cobertura de linha não substitui capacidade provada: 19 de 39 capacidades do `wyd-db` ainda não têm teste executado (2026-11-27).

## 7. Integração com CI

| Etapa | Onde | O que roda |
|---|---|---|
| PR | `docs.yml` | gate editorial e censos |
| PR | `code.yml` | fmt e `clippy+unit` por crate, cobertura (`wyd-db` informativa, `wyd-launcher` com o piso do plano do crate), audit/deny |
| PR | `docs-image.yml` | build e fumaça da imagem do site quando o caminho for tocado |
| sistema | gate do sistema (a fechar) | cobertura bloqueante, capacidades e rastreabilidade |
| release | a definir | deploy em staging, rollback e smoke |

Fonte: `pipeline-ci-cd.md` §3–§4.

## 8. Anti-padrões proibidos

- Teste que apenas aumenta porcentagem sem provar capacidade.
- `sleep` como sincronização.
- ID mágico sem factory.
- Estado global entre testes.
- `SKIP` silencioso.
- Mock de regra de negócio para fazer teste passar.
- Segredo em fixture, log ou artefato de teste.
- Teste que depende de dado externo não curado.

## 9. Skills que o agente deve carregar

| Trabalho | Skill | Quando |
|---|---|---|
| Planejar sistema ou frente | `planejar-sistema` | antes de escrever teste ou plano |
| Auditar página ou plano antes de aceitar | `auditar-planejamento` | antes de declarar `aceito` |
| Redação de documentação de teste | `documentation-writer` | ao redigir guias e planos |

Fonte: `wyd-ops/docs/arquitetura/ia-e-skills.md`.

## 10. Cruzamento com auditoria e especificação

| Referência | Relação com este documento |
|---|---|
| `wyd-ops/auditoria-de-organizacao.md` §2–§3 | 13 sistemas organizados; `wyd-ops` tem os contratos O1–O5 aprovados; este padrão não fecha a organização por si só |
| `wyd-ops/docs/specs/requisitos.md` (`RF-OPS-001`, `RF-OPS-003`, `RF-OPS-004`, `RF-OPS-007`, `RNF-OPS-005`) | CI, ambiente efêmero, monitoramento, gate e cobertura |
| `wyd-ops/docs/arquitetura/componentes.md` (`ci`, `ambiente`, `monitoramento`, `gate`) | unidades e invariantes de teste de infraestrutura |
| `wyd-ops/docs/planos/plano-de-testes.md` | alvos de teste por nível no `wyd-ops` |
| `wyd-db/docs/guia-teste/` | padrão de teste do piloto (parcial) |
| `AGENTS.md` §8 | cobertura mínima, pirâmide, proibições e E2E |

## 11. Decisões abertas (não decididas por este documento)

1. **Escopo da cobertura bloqueante** — quando o `code.yml` deixa de ser informativo.
   - Opções: A) por sistema, quando declarar gate fechado; B) global, quando todos os sistemas tiverem gate; C) por crate, no primeiro crate.
   - Recomendação: A, conforme `D-02 = B`; exceção já em vigor: `wyd-launcher` fecha o piso no próprio crate desde 2026-11-27.
2. **Dono da fixture central** — quem mantém `tests/common/`.
   - Opções: A) sistema dono do domínio; B) `wyd-ops` como padrão transversal; C) biblioteca compartilhada em crate comum.
   - Recomendação: A para dado de domínio; B para padrão e convenção.
3. **E2E** — ferramenta e dono.
   - Opções: A) Playwright para portal/cliente e Rust para os demais; B) tudo em Rust; C) tudo em Playwright.
   - Recomendação: A, conforme `AGENTS.md` §8.
4. **Teste de propriedade** — quando introduzir.
   - Opções: A) apenas fórmulas determinísticas do `wyd-sim`; B) também parser/protocolo; C) todos os sistemas.
   - Recomendação: A primeiro, depois B.

## 12. O que esta página não cobre

- Plano de teste específico de cada sistema.
- Implementação de teste, fixture, mock ou stub.
- Gate de documentação em si.
- Baseline de segurança, monitoramento e deploy: ver `baseline-seguranca-monitoramento-deploy.md`.
- Pipeline de CI/CD detalhado: ver `pipeline-ci-cd.md`.
