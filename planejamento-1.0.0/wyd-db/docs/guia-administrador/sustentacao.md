> Pergunta: "como assumo este projeto sem o dono anterior e sem descobrir tudo por leitura?"
> Público: Operação, quem entra no projeto e revisor
> Dono: equipe do `wyd-db` com a operação
> Base: `wyd-db-1.0.0.md` §6.12; `pendencias-de-fechamento.md` §6.6.4 (`Q-BLK-04` = A, 2026-11-24); `lacunas.md` `M32`; `../../../wyd-client/lacunas.md` `LCLI-03`
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-24)

O projeto é experimental, de uma pessoa, sem serviço no ar: não há plantão, não há escala e não há
segundo nome registrado. Esta página não monta organograma — nomeia **quatro funções**, dá nome real só
a quem o registro já nomeia, e lista o que um sucessor não descobre lendo, porque quase não há código
para ler.

## 1. Inventário do que existe

| Bloco | Onde | O que responde |
|---|---|---|
| plano diretor | `wyd-db-1.0.0.md` | escopo, censo e critério de aceite de página |
| lacunas | `lacunas.md` | dono, estado e prazo de cada lacuna |
| fila de páginas | `proposta-paginas.md` | quantas páginas autorais existem, escritas e bloqueadas |
| fechamento | `pendencias-de-fechamento.md` | o que falta, quem decide e o pacote `Q-BLK` |
| dicionário | [`dicionario-dados/`](../dicionario-dados/index.md) | 109 tabelas em 19 domínios |
| série e seeds | code `migrations/` e code `migrations-checklist.md` | 34 arquivos, 28 com página e 6 seeds bloqueados |
| arquitetura | [`arquitetura/`](../arquitetura/index.md) | fronteira, modelo, fluxo do dado, retenção e segurança |
| guias | [`guia-desenvolvedor/`](../guia-desenvolvedor/index.md), [`guia-teste/`](../guia-teste/index.md), [`guia-administrador/`](index.md) | como fazer, por público |
| roadmap | [`roadmap/`](../roadmap/index.md) | as 40 `WYDDB-nnn`, uma por issue, com DoD |
| código | code `crates/` | vazio: só `.gitkeep`; não há binário nem migration aplicada |

Contagens não se repetem aqui: o censo e a data de cada número são de `wyd-db-1.0.0.md` §4, e o estado
de cada página é de `docs/convencoes.md` §Estados.

## 2. Onde está cada fato (e o dono dele)

| Assunto | Dono único | Não procure em |
|---|---|---|
| quantos arquivos, tabelas e seeds existem | `plano-migrations.md` §3 | página de domínio ou dashboard |
| estado de uma página | `docs/convencoes.md` §Estados | `migrations-checklist.md` |
| retenção por tabela | [`retencao-e-capacidade.md`](../arquitetura/retencao-e-capacidade.md) | dicionário gerado |
| privilégio por papel | [`seguranca.md`](../arquitetura/seguranca.md) | guia do desenvolvedor |
| parâmetro e DSN por nome | [`configuracao.md`](configuracao.md) | esta página |
| rotina, calendário e registro | [`manutencao-rotineira.md`](manutencao-rotineira.md) | plano diretor |
| decisão pendente | `lacunas.md` §1 e `pendencias-de-fechamento.md` §6.6 | issue do GitHub |

## 3. Contatos e papéis

| Função | Quem | Canal | Decide |
|---|---|---|---|
| dono do produto | **Lucas Cantarelli**, nomeado em `lacunas.md` `M32` | Slack do projeto | escopo, prioridade e aprovação de decisão |
| operador do banco | função, **sem nome público** | Slack do projeto | executa a rotina, faz backup/restore e assume incidente ([`recuperacao-de-desastre.md`](recuperacao-de-desastre.md) §5) |
| revisor de página | quem **não** escreveu a página (`AGENTS.md` §7, etapa 2) | revisão no repositório | aceite da página (`lacunas.md` `M33`) |
| suporte ao jogador | função, sem nome público | Slack do projeto | primeiro atendimento; não altera dado sem `wyd_admin` |

O `LCLI-03`, no planejamento do cliente, registra que existe um operador distinto do QA e que o nome
dele entra na ficha de captura. É **precedente de vocabulário**, não prova de que exista um operador de
banco nomeado: as três funções sem nome continuam sem nome até alguém assumi-las por escrito.

## 4. O que só o dono anterior sabia

| Item | Por que não se descobre lendo | Onde está a prova |
|---|---|---|
| por que 6 seeds estão presos | dependem de valor medido no cliente 7662, não de decisão de schema | `pendencias-de-fechamento.md` §4.2 |
| por que A1 não vira `cargo test` | o cliente não é automatizável na `1.0.0` | [`piramide-e-alvos.md`](../guia-teste/piramide-e-alvos.md) §1 |
| por que a credencial administrativa não tem nome | falta decisão de operação | [`index.md`](index.md) §3 |
| por que `ops.audit_log` guarda 730 dias e não é expurgada | retenção decidida e `DELETE` fora do runtime | [`retencao-e-capacidade.md`](../arquitetura/retencao-e-capacidade.md) |
| por que `ops.delivery_queue` com `pending` nunca é expurgada | fila parada é entrega não perdida | [`fluxo-de-dados.md`](../arquitetura/fluxo-de-dados.md) §3, linha 6 |
| por que a rotina roda em `cron` do host | a operação já vive no host com Compose; agendador dentro do banco exigiria extensão | `pendencias-de-fechamento.md` §6.6.2 |
| por que o teto autoral é 144 e não 147 | três páginas saíram por decisão, não por corte de escopo | `proposta-paginas.md` §2 |
| por que `bloqueada` existe como estado | página com condição declarada não é página escrita | `docs/convencoes.md` §Estados |
| o que é “7662” | cliente de origem do jogo; nunca fonte de dado para a árvore nova | `../../../wyd-client/registro-do-cliente.md` |

## 5. Checklist de transferência

1. **Acesso:** repositório, Slack do projeto e o host da operação — sem isso, nada do resto importa.
2. **Credencial administrativa:** qual é o nome da variável e onde ela mora ([`configuracao.md`](configuracao.md)); esta página não inventa o nome, e [`index.md`](index.md) §3 registra a ausência.
3. **Backup:** o último arquivo gerado e o último ensaio de restore ([`backup-e-restore.md`](backup-e-restore.md)).
4. **Rotina:** o último registro de execução ([`manutencao-rotineira.md`](manutencao-rotineira.md) §7).
5. **Decisões abertas:** `lacunas.md` §1 e o pacote `Q-BLK` de `pendencias-de-fechamento.md` §6.6.
6. **O que não está pronto:** as páginas bloqueadas, os 6 seeds, as aplicações de retenção pendentes, a suíte e o CI de código — a lista viva está em `pendencias-de-fechamento.md` §1.

## 6. O que esta página não decide

- **A credencial nominal** de backup, vacuum e diagnóstico: é de [`configuracao.md`](configuracao.md) e da decisão de operação registrada em [`index.md`](index.md) §3.
- **Os nomes das outras três funções**, a escala e o plantão: não há registro, e inventar seria hipótese disfarçada de fato.
- **Contrato, salário e propriedade do projeto:** estão fora do repositório e não são decisão desta árvore.
