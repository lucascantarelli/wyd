# Decisões do `wyd-launcher` — manifesto, canais, assinatura e distribuição

> Pergunta: "qual é o formato do manifesto, de onde vem a lista de canais, o manifesto é assinado e
> como o launcher é distribuído?"
> Público: arquitetura, `wyd-launcher`, `wyd-client`, `wyd-server` e `wyd-ops`
> Dono: equipe do `wyd-launcher`
> Base: `lacunas.md` (`LLCH-01`–`LLCH-04`); `docs/index.md`; `wyd-launcher-1.0.0.md` §1–§6;
> `wyd-1.0.0.md` §7.1, §7.3 (X11) e §7.6; `AGENTS.md` §5.7 e §8
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23). Quatro perguntas, uma por lacuna aberta. Nenhuma está respondida e
nada foi aplicado. Não há patch, release nem manifesto; nenhum número desta página é medição.

## 1. Fronteira

**Fecha:** formato do manifesto, origem da lista de canais, assinatura e empacotamento/distribuição —
`LLCH-01` a `LLCH-04`.

**Não decide:** conteúdo do patch (`wyd-client`), hospedagem (`wyd-ops`), topologia de canais
(`wyd-server`), login (`wyd-net`/`wyd-protocol`).

## 2. O que o planejamento permite medir (`V`, 2026-11-23)

| Fato | Medição | Fonte |
|---|---|---|
| lacunas do sistema | **4 abertas, 0 propostas, 0 fechadas** | `lacunas.md` §1 |
| arquivos de código | **0** (só `.gitkeep`) | `wyd-launcher-1.0.0.md` §4 |
| manifesto, patch ou release | **0** | idem |
| contrato X11 | manifesto com hash por arquivo: **lacuna** | `wyd-1.0.0.md` §7.3 |
| testes | **0**; alvo nativo/script de fumaça | `wyd-1.0.0.md` §7.6 |

## 3. Já decidido — não reabrir

| Item | Decisão vigente | Fonte | Efeito |
|---|---|---|---|
| captura e golden | processo `LPRT-10` decidido; captura é do `wyd-client` | `LPRT-10`; X12 | `LLCH-01` consome o resultado, não o redefine |
| fronteira do conteúdo | conteúdo e versão do cliente são do `wyd-client`; hospedagem é do `wyd-ops` | `docs/index.md` | a lacuna não decide conteúdo nem hosting |

## 4. Decisões

### Q-LCH-1 — Qual é o formato do manifesto?

**Medido.** Não há manifesto, schema nem versão; `X11` diz "manifesto com hash por arquivo" e está em
lacuna. O launcher e o cliente ainda não têm contrato fechado.

**O que está em jogo.** Um formato legível e versionado permite verificar arquivo a arquivo; um
formato binário é compacto, mas difícil de auditar; reutilizar o legado não tem prova.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | JSON versionado: lista de arquivos, `sha256` por arquivo, versão do patch e compatibilidade | auditável e simples; exige schema e canonização para assinar |
| B | Formato binário compacto | menor e mais rápido; difícil de revisar e de evoluir sem ferramenta |
| C | Reutilizar o manifesto do legado | começa com formato conhecido; não há prova de que cobre hash/versão/compatibilidade |

**Recomendação: A.** É o menor formato que cumpre X11 e pode ser assinado por inteiro. **Custo:** um
schema e uma canonização. **Fontes:** `lacunas.md` `LLCH-01`; `wyd-1.0.0.md` X11;
`wyd-launcher-1.0.0.md` §3. **Recusa B** por auditabilidade; **C** por falta de prova.

### Q-LCH-2 — De onde vem a lista de canais e o estado de cada um?

**Medido.** Não há fonte decidida. O portal tem `/admin/canais` no planejamento, mas a topologia é do
`wyd-server`; o `wyd-launcher` só consome. A lacuna está aberta.

**O que está em jogo.** Lista no portal é uma fonte editorial; arquivo publicado funciona offline;
o servidor no login é a verdade operacional, mas exige conexão antes de mostrar a lista.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Portal/publicação como fonte, com cache local do launcher e estado por canal | uma fonte editorial e offline; exige atualização e cache |
| B | Arquivo estático dentro da release | simples e offline; muda com a release, não com a operação |
| C | O próprio servidor no login | estado real; o jogador só vê a lista depois de conectar |

**Recomendação: A.** O portal já tem a tela `/admin/canais`; usá-la como fonte exige uma rota pública
ou contrato de leitura, mas evita uma lista presa à release. **Custo:** cache, invalidação e contrato
com o portal. **Fontes:** `lacunas.md` `LLCH-02`; `docs/index.md`;
`wyd-portal/docs/arquitetura/consumo-do-banco.md` §2 e `wyd-portal/docs/arquitetura/rotas-e-permissoes.md` rota 37; `LLCH-02`.
**Recusa B** por desatualizar; **C** por inverter a ordem.

### Q-LCH-3 — O manifesto é assinado e o launcher verifica?

**Medido.** Não há assinatura, chave nem verificação; o risco é execução de código arbitrário por
patch adulterado. A lacuna está aberta.

**O que está em jogo.** Sem assinatura, o hash só protege contra corrupção, não contra troca; uma
assinatura exige gestão de chave e rotação.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Assinatura destacada do manifesto (Ed25519) verificada antes de aplicar, com chave pública no launcher | integridade autenticada; exige chave, rotação e cerimônia de assinatura |
| B | Só `sha256` no manifesto | detecta corrupção; não autentica a origem |
| C | Sem assinatura | mínimo; patch adulterado passa |

**Recomendação: A.** O risco declarado é execução arbitrária; hash sem assinatura não fecha esse
risco. **Custo:** gestão de chave. **Fontes:** `lacunas.md` `LLCH-03`; `AGENTS.md` §5.7;
`wyd-launcher-1.0.0.md` §3. **Recusa B** por não autenticar; **C** por risco aceito sem decisão.

### Q-LCH-4 — Como o aplicativo é empacotado e distribuído?

**Medido.** Não há instalador, release nem atualização; `wyd-ops` publica release e a lacuna está
aberta. Nada foi publicado.

**O que está em jogo.** Instalador assinado com atualização é o fluxo completo, mas exige infra; zip
manual é simples e não atualiza; sem pacote não há release.

| Opção | O que é | Consequência |
|---|---|---|
| **A (recomendada)** | Instalador assinado + atualização do launcher, publicados por `wyd-ops` | fluxo completo; exige assinatura, hosting e rollback |
| B | Zip/portable manual | simples; sem atualização e sem verificação de versão |
| C | Sem empacotamento nesta versão | não bloqueia código; não há release |

**Recomendação: A.** É o único formato que entrega atualização e verificação; a assinatura de Q-LCH-3
se aplica ao pacote. **Custo:** pipeline e hosting. **Fontes:** `lacunas.md` `LLCH-04`;
`wyd-launcher-1.0.0.md` §2; `AGENTS.md` §9. **Recusa B** por não atualizar; **C** por não entregar.

## 5. Cobertura das lacunas

| Lacuna | Pergunta | Situação |
|---|---|---|
| `LLCH-01` | `Q-LCH-1` | aberta |
| `LLCH-02` | `Q-LCH-2` | aberta |
| `LLCH-03` | `Q-LCH-3` | aberta |
| `LLCH-04` | `Q-LCH-4` | aberta |

## 6. Registro da decisão

**Aguardando resposta do dono.** As quatro lacunas continuam `aberta`; nada foi aplicado.

| Pergunta | Lacuna | Resposta |
|---|---|---|
| `Q-LCH-1` | `LLCH-01` | — |
| `Q-LCH-2` | `LLCH-02` | — |
| `Q-LCH-3` | `LLCH-03` | — |
| `Q-LCH-4` | `LLCH-04` | — |
