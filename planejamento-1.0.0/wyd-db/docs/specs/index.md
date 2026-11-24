> Pergunta: "o que há de requisito no `wyd-db` e em que estado cada área está?"
> Público: Produto / gestão e quem procura um requisito
> Dono: equipe do `wyd-db`
> Base: `wyd-db-1.0.0.md` §6.5 e §9; `proposta-paginas.md` §4–§5; `lacunas.md` §1
> Revisado em: 2026-11-24

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
| `visao-e-escolpo.md` | o que o `wyd-db` resolve? | **escrita** (recorte, 2026-11-22): problema, escopo, premissas, o que a `1.0.0` entrega e o que a página não decide |
| [`stakeholders.md`](stakeholders.md) | quem pede o quê? | **escrita** (2026-11-24): 5 atores, credencial e requisito que cada um pede |
| [`requisitos-nao-funcionais.md`](requisitos-nao-funcionais.md) | com que qualidade? | **escrita** (2026-11-24): 5 `RNF-DB`, 3 com limiar e 2 a medir |
| [`rastreabilidade.md`](rastreabilidade.md) | quem atende este requisito? | **escrita** (2026-11-24): matriz de 40 requisitos em 5 colunas |

## Fluxos e estados

| Pasta | Páginas escritas | Restantes |
|---|---|---|
| [`fluxos/`](fluxos/login-e-sessao.md) | `login-e-sessao.md` (2026-11-22), `carga-de-conteudo.md` (recorte, 2026-11-22), `criacao-e-selecao-de-personagem.md` e `carregamento-e-save.md` (2026-11-24), `concessao-de-grant.md` e `compra-e-credito.md` (2026-11-24) | 0 |
| [`estados/`](estados/conta-e-moderacao.md) | `conta-e-moderacao.md`, `sessao.md` (2026-11-22), `personagem.md`, `grant.md` e `item-e-refino.md` (2026-11-24) | 0 |

As seis páginas de fluxo e as cinco de estado planejadas em `wyd-db-1.0.0.md` §4 estão escritas: o
lote de entrada na cena e o de economia e custódia fecharam as sete que faltavam em 2026-11-24, e
`requisitos-nao-funcionais.md` saiu do bloqueio na mesma data (`Q-BLK-07` = A). Nenhuma página de
`specs/` está na fila. `stakeholders.md` e `rastreabilidade.md` foram escritas em 2026-11-24 (`Q-BLK-09` e
`Q-BLK-08` = A) e a pasta fecha as **20** do teto. As lacunas de save, posição e refino continuam declaradas nas próprias
páginas de fluxo e estado.

## Convenção de ID

O prefixo e o formato são de `wyd-db-1.0.0.md` §9.1. Nesta pasta valem:

| Prefixo | Uso | Primeiro número | Último em uso |
|---|---|---|---|
| `RF-DB-nnn` | requisito funcional | `RF-DB-001` | `RF-DB-017` |
| `RN-DB-nnn` | regra de negócio | `RN-DB-001` | `RN-DB-007` |
| `UC-DB-nnn` | caso de uso | `UC-DB-001` | `UC-DB-003` |
| `RNF-DB-nnn` | requisito não funcional | `RNF-DB-001` | `RNF-DB-007` |

`RNF-DB` nasceu em 2026-11-24 com [requisitos-nao-funcionais.md](requisitos-nao-funcionais.md):
`RNF-DB-001` a `RNF-DB-003` têm limiar com fonte, `RNF-DB-004` e `RNF-DB-005` têm métrica e
**sem limiar até `M20`**, e `Q-BLK-24` = A (mesma data) acrescentou `RNF-DB-006` — limiar **0** de
`40P01`, declarado **sem página de fonte** — e `RNF-DB-007` — espera de `30 s` e reuso, com fonte em
`configuracao.md` §3. O último `RF` nasceu na mesma decisão: `RF-DB-017`, versão única de conteúdo. O próximo número livre de cada série é o último mais um; número não é
reaproveitado (`wyd-db-1.0.0.md` §9.2).

## Estado por área

| Área | O que existe | O que falta |
|---|---|---|
| identidade (pessoa, conta, credencial, sessão) | `RF-DB-001`–`RF-DB-016`, `RN-DB-001`–`RN-DB-007`, `UC-DB-001`–`UC-DB-003`, fluxo de login, estados de conta e de sessão | recuperação de senha, MFA do staff e bloqueio pelo `wyd-admin` |
| personagem e inventário | dicionário, DDL, fluxo de criação/seleção, carga/save, estado do personagem e estado de item/refino | requisito, regra e caso de uso |
| economia, conteúdo e entrega | dicionário, DDL, fluxo de grant, fluxo de compra/crédito e estado de grant | requisito, regra e caso de uso; implementação do cash não tem issue |
| qualidade (RNF, rastreabilidade) | 7 `RNF-DB` (5 com limiar, 2 a medir) | limiares medidos, porta, teste e issue |

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
- Não mede cobertura: a matriz é de [`rastreabilidade.md`](rastreabilidade.md).
