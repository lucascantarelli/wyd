> Pergunta: "o que há de requisito no `wyd-db` e em que estado cada área está?"
> Público: Produto / gestão e quem procura um requisito
> Dono: equipe do `wyd-db`
> Base: `wyd-db-1.0.0.md` §6.5 e §9; `proposta-paginas.md` §4–§5; `lacunas.md` §1
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

Porta de entrada das especificações. Um requisito nasce com número nesta pasta, é citado pelo
dicionário e pela página de `migrations/`, e só morre com estado `retirado` e motivo — nunca por
apagamento. A convenção de prefixo e de estado é de `wyd-db-1.0.0.md` §9.1–§9.3; esta página só
mapeia o que existe.

## As 8 páginas irmãs

| Página | Pergunta | Estado |
|---|---|---|
| [requisitos-funcionais.md](requisitos-funcionais.md) | o que o banco precisa fazer? | **escrita** (2026-11-22): 16 `RF-DB` do bloco de identidade |
| [regras-de-negocio.md](regras-de-negocio.md) | que regra vale sempre? | **escrita** (2026-11-22): 7 `RN-DB` |
| [casos-de-uso.md](casos-de-uso.md) | quem faz o quê, passo a passo? | **escrita** (2026-11-22): 3 `UC-DB` |
| [restricoes-e-premissas.md](restricoes-e-premissas.md) | o que assumimos e o que nos limita? | **escrita** (2026-11-22): restrições, premissas `H` e 3 lacunas externas |
| `visao-e-escolpo.md` | o que o `wyd-db` resolve? | recorte pendente (`proposta-paginas.md` §5) |
| `stakeholders.md` | quem pede o quê? | bloqueada: ator → requisito exige `RF-DB` de outros domínios |
| `requisitos-nao-funcionais.md` | com que qualidade? | bloqueada: nenhum limiar medido |
| `rastreabilidade.md` | quem atende este requisito? | bloqueada: porta, teste e issue ainda não existem |

## Fluxos e estados

| Pasta | Páginas escritas | Restantes |
|---|---|---|
| [`fluxos/`](fluxos/login-e-sessao.md) | `login-e-sessao.md` (2026-11-22) | 5: personagem, carga e save, grant, compra e crédito, carga de conteúdo |
| [`estados/`](estados/conta-e-moderacao.md) | `conta-e-moderacao.md`, `sessao.md` (2026-11-22) | 3: personagem, grant, item e refino |

As páginas restantes dependem de fluxo ou DDL de outro domínio (`lacunas.md` §3) e não entram por
conveniência: página que só existiria para preencher a pasta não é escrita.

## Convenção de ID

O prefixo e o formato são de `wyd-db-1.0.0.md` §9.1. Nesta pasta valem:

| Prefixo | Uso | Primeiro número | Último em uso |
|---|---|---|---|
| `RF-DB-nnn` | requisito funcional | `RF-DB-001` | `RF-DB-016` |
| `RN-DB-nnn` | regra de negócio | `RN-DB-001` | `RN-DB-007` |
| `UC-DB-nnn` | caso de uso | `UC-DB-001` | `UC-DB-003` |

`RNF-DB` ainda não tem número: a página de requisito não funcional está bloqueada. O próximo número
livre de cada série é o último mais um; número não é reaproveitado (`wyd-db-1.0.0.md` §9.2).

## Estado por área

| Área | O que existe | O que falta |
|---|---|---|
| identidade (pessoa, conta, credencial, sessão) | `RF-DB-001`–`RF-DB-016`, `RN-DB-001`–`RN-DB-007`, `UC-DB-001`–`UC-DB-003`, fluxo de login, estados de conta e de sessão | recuperação de senha, MFA do staff e bloqueio pelo `wyd-admin` |
| personagem e inventário | dicionário e DDL | requisito, regra, caso de uso, fluxo e estados |
| economia, conteúdo e entrega | dicionário e DDL | idem |
| qualidade (RNF, rastreabilidade) | — | limiar medido, porta, teste e issue |

## Como um requisito nasce, muda e é retirado

1. **Nasce** com o próximo número livre da série, na página dona, com critério de aceitação
   verificável (`wyd-db-1.0.0.md` §7.2).
2. **Muda** por decisão do dono; a alteração aparece na página dona e nas que a citam na mesma
   entrega. Mudança que altera contrato não é edição silenciosa.
3. **É retirado** com estado `retirado`, motivo e data; o número não volta a ser usado
   (`wyd-db-1.0.0.md` §9.3). `substituído` aponta o ID novo.

## O que esta página não faz

- Não repete o texto do requisito: aponta a página dona.
- Não numera requisito novo: a numeração nasce na página que o enuncia.
- Não mede cobertura: a matriz é de `rastreabilidade.md`, ainda bloqueada.
