# Decisões do `wyd-client` — patch e aprovação da captura

> Pergunta: "o que o patch entrega e quem aprova a segunda leitura da captura?"
> Público: frente `wyd-client`, `wyd-protocol`, QA e `wyd-launcher`
> Dono: equipe do `wyd-client`
> Base: `lacunas.md` (`LCLI-01`–`LCLI-05`); `registro-do-cliente.md`; `docs/index.md`;
> `wyd-1.0.0.md` §7.1, §7.3 (X11, X12) e §7.6; `AGENTS.md` §5.6, §5.7 e §8
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). **Duas perguntas** para as lacunas abertas; `LCLI-01` é a mesma
decisão de `LPRT-09` e não é duplicada; `LCLI-04` já foi decidida e `LCLI-05` tem página própria, que
esta rodada não reabre. Nada foi aplicado. O hash e o tamanho citados são **medições** do registro;
nenhum outro número é medição.

## 1. Fronteira

**Fecha:** `LCLI-02` (conteúdo e versionamento do patch) e `LCLI-03` (aprovador da segunda leitura;
o local do registro já está fixado por `LPRT-10`).
**Não duplica:** `LCLI-01` (build-alvo e identidade) é `LPRT-09` vista do lado do cliente; a pergunta
`Q-PRT-1` é o dono único.

**Não decide:** a ferramenta de captura (`LCLI-04`, decidida), a titularidade e a distribuição
(`LCLI-05`, página própria), o layout do fio (`wyd-protocol`), o manifesto (`wyd-launcher`) e a
hospedagem (`wyd-ops`).

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| lacunas do sistema | **5**: 3 abertas, 1 proposta (`LCLI-05`), 1 fechada (`LCLI-04`) | `lacunas.md` §1 |
| build-alvo | **não decidido**; `WYD.exe` é candidato | `registro-do-cliente.md` §4 |
| tamanho de `WYD.exe` | **2.347.008 bytes** | `registro-do-cliente.md` |
| hash de `WYD.exe` | prefixo medido `04c9d2a1…`; a ficha deve completar o hash | idem |
| arquivos do cliente versionados no git | **0** — só `wyd-client/.gitkeep` | `registro-do-cliente.md` §3 |
| capturas existentes | **0** | `docs/index.md` |
| código do cliente | **0** | `lacunas.md`; `AGENTS.md` §5.6 |

## 3. Já decidido — não reabrir

| Item | Decisão vigente | Fonte | Efeito |
|---|---|---|---|
| ferramenta de captura | **opção A**: ferramenta específica em `wyd-client/tools/`, ainda não criada | `lacunas.md` `LCLI-04` | não se pergunta de novo |
| processo de captura | bytes + ficha + anonimização + segunda leitura, com registro em `tests/golden/` | `LPRT-10`; `goldens/como-capturar.md` | o local está fixado; `LCLI-03` só nomeia o aprovador |
| dado de jogador | captura só sintética, sem produção/terceiros | `LCLI-04`; `LPRT-10` | nenhuma opção pode usar conta real |

## 4. Decisões

### Q-CLI-2 — O que o patch entrega e como versiona?

**Medido.** Não há lista de arquivos do patch nem versão de patch; o `wyd-launcher` depende do
manifesto (X11). `LCLI-05` segue `proposta` e mantém a autorização de conteúdo em aberto. O registro
lista `WYD.exe`, mas não decide o que é distribuível.

**O que está em jogo.** "Cópia da árvore inteira" aumenta o patch e o risco jurídico; um conjunto
mínimo é verificável, mas precisa de lista e de versão.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Pacote mínimo declarado: `strdef.bin`, configurações e o executável **se liberados**, com versão de patch e lista por arquivo | patch pequeno e verificável; depende de `LCLI-05` para cada item |
| B | Árvore inteira do cliente | simples de montar; grande, difícil de versionar e inclui material não liberado |
| C | Só o executável | mínimo; não cobre `strdef.bin`/config e pode não rodar |

**Recomendação: A.** O manifesto precisa de lista e hash por arquivo (X11); a lista mínima é a única
que pode ser provada item a item contra `LCLI-05`. **Custo:** manter a lista e a versão. **Fontes:**
`lacunas.md` `LCLI-02`; `wyd-1.0.0.md` §7.3 X11; `LCLI-05`; `registro-do-cliente.md`. **Recusa B** por
risco e tamanho; **C** por não cobrir o patch.

### Q-CLI-3 — Quem aprova a segunda leitura da captura?

**Medido.** `LPRT-10` já fixa o registro em `tests/golden/`, a ficha e a segunda leitura; `LCLI-03`
registra que o **aprovador** da segunda leitura não está nomeado. Não há captura nem ficha preenchida.

**O que está em jogo.** Aprovador anônimo ou acumulado na mesma frente que captura reduz a
independência da segunda leitura; nomear uma pessoa ou uma dupla fecha a cadeia.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Pessoa de QA **ou do `wyd-protocol`**, independente de quem capturou, assina a ficha e a entrada do catálogo | segunda leitura independente e rastreável; exige disponibilidade e registro do nome |
| B | O dono do `wyd-client` aprova cada captura | autoridade centralizada; a mesma frente captura e aprova, então a independência é menor |
| C | Dupla revisão: QA + `wyd-protocol` assinam | mais forte para o fio; custa duas pessoas por captura |

**Recomendação: A.** `LPRT-10` exige uma segunda leitura por outra pessoa; nomear alguém de QA,
independente da captura, preserva essa separação sem transformar o dono do artefato em revisor único.
**Custo:** nomear a pessoa e manter a ficha. **Fontes:** `lacunas.md` `LCLI-03`; `LPRT-10`;
`wyd-protocol/docs/goldens/como-capturar.md` §7; `LCLI-04` D6. **Recusa B** por reduzir a independência;
**C** por custo duplo sem exigência da fonte.

**Resposta do dono em 2026-11-23: A — QA designado assina; dono do `wyd-protocol` é suplente.** O nome
do QA **não foi informado**; `LCLI-03` continua aberta até o nome entrar na ficha. O local já está
fixado: frame e ficha em `tests/golden/`, entrada pública em `docs/goldens/catalogo.md`, bruto fora do
repositório (D5).

### `LCLI-05` — página própria, não reaberta

A titularidade e a distribuição têm seis perguntas em
`docs/specs/propriedade-intelectual-e-distribuicao.md` (`Q-IP-1`–`Q-IP-6`). Esta página não as repete:
o dono do produto ainda não nomeou o responsável por direitos, e nenhuma resposta foi dada.

## 5. Cobertura das lacunas

| Lacuna | Pergunta | Situação |
|---|---|---|
| `LCLI-01` | — | aberta no `lacunas.md`, mas é a mesma decisão de `LPRT-09`/`Q-PRT-1`; a fonte diz que é "`LPRT-09` vista do lado do cliente"; não duplicada aqui |
| `LCLI-02` | `Q-CLI-2` | aberta |
| `LCLI-03` | `Q-CLI-3` | aberta; papel A decidido, **nome do QA pendente**; local fixado por `LPRT-10` |
| `LCLI-04` | — | fechada em 2026-11-23 (opção A); D3–D6 decididos; implementação pendente |
| `LCLI-05` | `Q-IP-1`–`Q-IP-6` | proposta, na página própria de propriedade intelectual |

## 6. Registro da decisão

**Parcialmente respondida em 2026-11-23.** `Q-CLI-3` escolheu A (QA designado; dono do `wyd-protocol`
suplente), mas o nome não foi informado; `LCLI-03` continua `aberta` e a captura pode ser executada, não
aceita. `LCLI-02` continua aberta; `LCLI-01` fecha em `LPRT-09`; `LCLI-05` continua `proposta` na página
própria.

| Pergunta | Lacuna | Resposta |
|---|---|---|
| `Q-CLI-2` | `LCLI-02` | — |
| `Q-CLI-3` | `LCLI-03` | A — QA designado; suplente `wyd-protocol`; **nome pendente** |
| `Q-IP-1`–`Q-IP-6` | `LCLI-05` | — (página própria) |
