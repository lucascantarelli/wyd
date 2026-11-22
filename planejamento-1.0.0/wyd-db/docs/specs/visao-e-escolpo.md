> Pergunta: "o que o `wyd-db` resolve, o que fica fora e o que a `1.0.0` entrega?"
> Público: Produto, arquitetura e revisor de requisito
> Dono: equipe do `wyd-db`
> Base: `wyd-db-1.0.0.md` §1.2; `plano-migrations.md` §2–§4; `docs/arquitetura/visao-geral.md`; `lacunas.md`
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

O `wyd-db` resolve um problema de **fronteira de escrita**: jogo, portal e operação precisam do mesmo
dado sem compartilhar credencial, e o banco precisa recusar a escrita que o código não deveria fazer.
Ele entrega o schema, a série de migrations, o runner, o provisionamento de papéis, o pool e as portas
de acesso. Ele não entrega a regra de jogo, a tela do portal, o deploy do servidor nem a operação do
ambiente.

## 1. Problema

| Sem o `wyd-db` | Com o `wyd-db` |
|---|---|
| cada serviço guarda uma cópia da identidade ou do estado | `portal.person` é a identidade; `game.account` referencia `person_id` |
| a separação de poder depende de disciplina do código | `GRANT`/`REVOKE` e views aplicam a fronteira no banco |
| a série de schema não tem ordem nem rollback | arquivos `up`/`down` contíguos, ledger e checksum |
| o boot lê um catálogo misturado | `content_version` fixa o corte de conteúdo lido |
| a operação não sabe quem escreveu o quê | `ops.audit_log`, `ops.login_attempt` e fila de entrega |

## 2. Escopo

| Dentro da `1.0.0` | Fora da `1.0.0` |
|---|---|
| PostgreSQL 17, quatro schemas e `public` vazio | topologia de réplica, cache ou event bus |
| 34 arquivos de migration e `SCHEMA_VERSION = 34` | carga/ETL da base legada |
| 109 tabelas e 2 views de leitura cruzada | relatório de BI, dashboard e tela de operação |
| runner `status`/`apply`/`down`, ledger, checksum e lock | interface de linha de comando além dos subcomandos documentados |
| provisionamento de seis papéis e `public` | rotação de senha e `pg_hba.conf` do ambiente |
| portas de `migrate`/`provision` e contratos de `repo` já desenhados | contrato de `repo/` que ainda não existe |
| testes de contrato e sistema descritos | suíte completa, cobertura e CI real |
| dicionário de dados e páginas de domínio | página de retenção sem volume medido |

A tabela de escopo é a leitura de `wyd-db-1.0.0.md` §1.2; se as duas páginas divergirem, o plano
diretor é o dono.

## 3. Premissas de negócio

| Premissa | O que a invalida |
|---|---|
| o jogo lê o catálogo no boot e usa um corte de `content_version` | necessidade de recarga a quente sem reinício, que não foi desenhada |
| o portal é a origem da identidade e da PII | o jogo passar a criar identidade própria |
| a operação lê por view e não escreve no quente | uma tarefa operacional exigir escrita direta em `game` |
| o cliente 7662 é a fonte de login, senha e PIN do jogo | mudança de protocolo ou de cliente que altere o formato |
| a série é de instalação, não de evolução incremental | necessidade de `ALTER` em produção antes da troca de série |

As premissas de produto ainda abertas ficam em
[`restricoes-e-premissas.md`](restricoes-e-premissas.md); esta página não as fecha.

## 4. O que a `1.0.0` entrega

1. **Schema instalável:** a série contígua de 34 arquivos, com `down` por arquivo e ledger.
2. **Papéis provisionáveis:** seis papéis, `public` sem escrita pública e uma DSN por papel.
3. **Fronteira aplicada:** escrita separada por schema e leitura cruzada por duas views.
4. **Runner verificável:** `status`, `apply`, `down` e `--check`, com drift e lock explícitos.
5. **Documentação operável:** dicionário, guia do desenvolvedor, guia do administrador e guia de teste
   nos recortes que têm fonte; o que depende de medição fica declarado.

Os números 34, 109 e 2 são **planejados** (`P`) até a série ser aplicada e o catálogo medido; a fonte é
`plano-migrations.md` §3, não esta página. O mapa de dados começa em
[`../dicionario-dados/index.md`](../dicionario-dados/index.md); a ordem de implantação é de
[`../planos/plano-de-implantacao.md`](../planos/plano-de-implantacao.md).

## 5. O que esta página não decide

- **Regra de jogo, balanceamento e economia:** são do `wyd-server`/`wyd-sim`; o banco só persiste.
- **Preço, retenção final e catálogo comercial:** são decisões de produto registradas em `lacunas.md`.
- **Números medidos de volume, latência e capacidade:** não existem antes da série.
- **Prazo e custo:** não há estimativa aprovada; a fila de trabalho é de [`../roadmap/fases.md`](../roadmap/fases.md).
