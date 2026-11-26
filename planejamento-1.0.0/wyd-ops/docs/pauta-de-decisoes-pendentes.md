# Pauta de decisões pendentes — registro canônico

> Pergunta: "quais são as 11 pendências que ainda exigem decisão ou evidência externa, o que cada uma bloqueia, quais opções existem e o que a aprovação libera?"
> Público: dono do produto, arquitetura, `wyd-ops`, QA, `wyd-db`, `wyd-client`, `wyd-protocol`, `wyd-net`, `wyd-bots`, `wyd-portal`, revisão jurídica e agentes
> Dono: equipe do `wyd-ops` (consolidação); cada decisão pertence ao dono listado no item
> Base: `docs/lacunas.md` §4; `docs/pacote-aprovacao-o1-o5.md` §7; `docs/pacote-aprovacao-decisoes-fora-o1-o5.md` §7; ledgers `lacunas.md` dos sistemas; `docs/github-mapping.md` §7; `docs/arquitetura/adr/index.md` §2
> Revisado em: 2026-11-26

**Estado:** proposto (2026-11-26). Esta é a **página canônica** das decisões e pendências abertas. Ela não decide, não aprova e não envia mensagem: reúne o que falta, as opções e o critério de pronto, e aponta para o ledger e o documento dono de cada item. O estado de cada lacuna continua em `docs/lacunas.md` §4 e nos `lacunas.md` dos sistemas; os pacotes O1–O5 guardam os textos de origem. Não existe uma segunda pauta.

**Escopo:** 11 itens abertos. Decisões já respondidas e formalizadas — `OPS-03.1`–`OPS-03.5`, `D-06`, `Q-SERIE-1`, `Q-SEP-1`, `LCLI-02` — não são reabertas aqui; o histórico está em `docs/lacunas.md` §4 e nos pacotes.

## 1. Como usar

| Elemento | Onde vive |
|---|---|
| Estado, dono e evidência da lacuna | `docs/lacunas.md` §4 e o `lacunas.md` do sistema |
| Pergunta completa e mensagem de origem | §7 do pacote O1–O5 ou do pacote fora de O1–O5 |
| Opções, recomendação e critério de pronto | item correspondente nesta página |
| Board e inventário de issues | `docs/github-mapping.md` §7 |

Status possíveis: **bloqueada** (sem decisão, evidência ou execução externa) ou **fechada** com a evidência registrada no ledger. Um "sim" dito no chat não fecha a linha: a resposta precisa ser registrada no ledger com data, dono e artefato.

## 2. Índice

| # | Item | Pergunta curta | Responde | Estado |
|---:|---|---|---|---|
| 1 | [`OPS-04`](#ops-04) | Provedor, VPS, backup e RPO/RTO | infra/owner | bloqueada |
| 2 | [`OPS-05`](#ops-05) | Board do GitHub Project | infra/owner | bloqueada |
| 3 | [`LPRT-03`](#lprt-03) | Origem da `KeyWord` e prova da captura A1 | runtime | bloqueada como prova |
| 4 | [`LPRT-06`](#lprt-06) | Encoding, tamanho e terminador de string | runtime | bloqueada como prova |
| 5 | [`LCLI-03`](#lcli-03) | Operador e primeira ficha de captura | infra/owner | bloqueada como evidência |
| 6 | [`LCLI-05`](#lcli-05) | Evidência do titular para distribuir o cliente | dono + titular | bloqueada como evidência |
| 7 | [`LNET-01`](#lnet-01) | Limites de conexão, timeout e rate-limit medidos | runtime | bloqueada como medição |
| 8 | [`LBOT-03`](#lbot-03) | Limiar de carga dos bots medido | runtime | bloqueada como medição |
| 9 | [`A11`](#a11) | Charset do `GameLogin` | runtime | bloqueada como medição |
| 10 | [`A20`](#a20) | MFA de staff antes do portal público | infra/owner | bloqueada por execução |
| 11 | [`M21`](#m21) | Telas e rotas do portal | infra/owner | bloqueada por execução |

## 3. Itens

### OPS-04

**O que é.** Aplicação da decisão `Q-OPS-2` = A: provedor de VPS, contrato, região, backup e RPO/RTO da hospedagem de produção. A forma já está em `ADR-OPS-0002`; falta o provisionamento e os valores operacionais.

**Por que está bloqueado e por quem.** O dono do produto adiou provedor, região, backup e RPO/RTO para depois do primeiro deploy interno e informou que não há valores definidos. Depende de produto + `wyd-ops`; não é decisão de arquitetura.

**Evidência/artefato que falta.** Custo mensal cotado, provedor e região escolhidos, contrato/credencial, destino de backup, RPO/RTO numéricos e um ensaio de restauração. Nada foi provisionado; não há custo, VPS ou backup medidos (`V`, 2026-11-25).

**Opções.**

| Opção | O que é | Custo/risco/esforço | Recomendação |
|---|---|---|---|
| **A** | Medir 2–3 alternativas de VPS com Compose, cotação, backup e RPO/RTO; decidir com esses dados e manter produção desligada até a aprovação | custo de cotação e tempo; risco de atraso; esforço médio | **recomendada** |
| B | Escolher um provedor agora, sem cotação, backup ou RPO/RTO definidos | custo desconhecido; risco de retrabalho e backup frágil; esforço baixo no papel, alto depois | não recomendada |
| C | Manter adiado até depois do primeiro deploy interno | custo zero agora; produção e backup continuam inexistentes; esforço zero | aceitável como estado atual, mas não fecha |

**Por que A.** É a única opção que produz os números que o dono pediu para decidir; evita escolher provedor por impulso e deixa o critério de backup verificável. O adiamento do dono continua respeitado: A prepara a decisão, não provisiona.

**Pronto quando.** O dono aprovar provedor, região, contrato, destino de backup e RPO/RTO numéricos; o ensaio de restauração passar; `ADR-OPS-0002` e o ledger registrarem data e evidência; a issue [#198](https://github.com/lucascantarelli/wyd/issues/198) for atualizada.

**Origem.** [`docs/lacunas.md` §4](lacunas.md); [`ADR-OPS-0002`](arquitetura/adr/adr-ops-0002-hospedagem-producao.md); [`pacote fora de O1–O5` §7.1](pacote-aprovacao-decisoes-fora-o1-o5.md#71-produto-ops-04-ops-05-e-d-06); issue [#198](https://github.com/lucascantarelli/wyd/issues/198).

### OPS-05

**O que é.** Montagem do board do GitHub Project para as 111 páginas/issues planejadas. As 61 fatias (#129–#189) e 10 lacunas (#190–#199) já foram criadas com labels `area/*`; o board não existe.

**Por que está bloqueado e por quem.** O dono do produto autorizou as issues, mas a criação do board é ato de gestão no GitHub e continua pendente. Depende do produto; não é arquitetura.

**Evidência/artefato que falta.** Projeto `WYD 1.0.0` criado, campos e views de `wyd-ops/proposta-gestao-github-e-ambientes.md` §10.3, dependências `blocked by` e as 111 issues importadas.

**Opções.**

| Opção | O que é | Custo/risco/esforço | Recomendação |
|---|---|---|---|
| **A** | Criar um Project com os campos/views já propostos e importar as 111 issues, mantendo labels e milestones como estão | custo zero; risco baixo de duplicidade se o import for conferido; esforço baixo | **recomendada** |
| B | Adiar o board e operar só por labels/milestones | custo zero; perde views, dependências e leitura de portfólio; esforço zero | aceitável temporariamente |
| C | Criar vários boards por sistema | custo de manutenção; fragmenta a visão e duplica configuração; esforço médio | não recomendada |

**Por que A.** O inventário e os campos já estão definidos; um único board preserva a dependência entre sistemas e evita uma segunda fonte de status. B não quebra o repositório, mas não cumpre o mapeamento pedido.

**Pronto quando.** O board existir com os campos/views acordados, as 111 issues estiverem associadas e as dependências registradas; `docs/github-mapping.md` §7 apontar para ele.

**Origem.** [`docs/lacunas.md` §4](lacunas.md); [`github-mapping.md` §5 e §7](github-mapping.md).

### LPRT-03

**O que é.** A origem da `KeyWord` usada na ofuscação do handshake. A política já foi decidida: o crate recebe a `KeyWord` do chamador e não assume origem; falta provar o byte no cliente 7662.

**Por que está bloqueado e por quem.** A captura A1 não foi executada: não há `ECAP`/`PEER-A1` operado, `crates/wyd-server`/`wyd-net`/`wyd-protocol` não têm código e `wyd-client/tools/` não existe. Depende de runtime/captura e da segunda leitura de QA; não é uma nova escolha de desenho.

**Evidência/artefato que falta.** `ECAP` e CLI implementados, captura A1 datada, frame anonimizado, golden com origem e segunda leitura assinada. Até lá a afirmação de byte fica `H`.

**Opções.**

| Opção | O que é | Custo/risco/esforço | Recomendação |
|---|---|---|---|
| **A** | Implementar o `ECAP` + CLI já autorizados, capturar A1, fazer a segunda leitura e registrar o golden | custo de implementação e operação; risco de a captura revelar comportamento diferente do legado; esforço médio | **recomendada** |
| B | Capturar por ferramenta externa/proxy sem implementar o `ECAP` operado | custo menor inicial; risco de captura não reproduzível e fora do processo `LPRT-10`; esforço baixo/médio | não recomendada |
| C | Manter a validação permissiva e a prova como `H` até o loop jogável | custo zero agora; o handshake fica sem prova e bloqueia a onda C; esforço zero | não fecha a lacuna |

**Por que A.** É a execução já autorizada em 2026-11-23; produz golden reproduzível e satisfaz `LPRT-10`. B economiza implementação, mas não resolve o endpoint operado nem a segunda leitura.

**Pronto quando.** A captura A1 tiver ficha datada, frame anonimizado e golden; a segunda leitura independente estiver assinada; o comportamento da `KeyWord` estiver escrito com `V`; `LPRT-03` fechar no ledger do `wyd-protocol`.

**Origem.** [`wyd-protocol/lacunas.md` §2.2](https://github.com/lucascantarelli/wyd/blob/develop/planejamento-1.0.0/wyd-protocol/lacunas.md); [`endpoint-de-captura.md`](../wyd-server/specs/endpoint-de-captura.md); [`como-capturar.md`](../wyd-protocol/goldens/como-capturar.md); [`proposta-decisoes-protocol.md` §6](../wyd-protocol/proposta-decisoes-protocol.md).

### LPRT-06

**O que é.** Encoding, tamanho e terminador das strings do fio. A política já foi decidida: o codec não transcodifica e a string é bytes de tamanho fixo declarado; falta medir encoding e terminador com texto acentuado.

**Por que está bloqueado e por quem.** Depende das capturas B/C com personagem sintético criado no servidor; o mesmo `ECAP`/`PEER-A1` de `LPRT-03` não existe e não há runtime operado. Depende de runtime/captura e QA.

**Evidência/artefato que falta.** Capturas B/C com texto acentuado, seed sintético, frame anonimizado, golden e segunda leitura. Não há golden de string registrado no ledger.

**Opções.**

| Opção | O que é | Custo/risco/esforço | Recomendação |
|---|---|---|---|
| **A** | Depois de A1, executar B/C com personagem sintético e seed, registrar golden e segunda leitura | custo de mais capturas; risco de revelar encoding diferente do esperado; esforço médio | **recomendada** |
| B | Inferir encoding/terminador do legado ou de uma única captura | custo baixo; risco de generalizar sem `V` e corromper nome/chat; esforço baixo | não recomendada |
| C | Adiar até o loop jogável e manter `H` | custo zero agora; texto acentuado fica sem prova; esforço zero | não fecha |

**Por que A.** É a única opção que cobre texto real e o processo de golden; B repete a contaminação do legado que `AGENTS.md` §3 proíbe como fato.

**Pronto quando.** As capturas B/C tiverem ficha, frame e golden; encoding/tamanho/terminador estiverem provados com `V`; `LPRT-06` fechar no ledger.

**Origem.** [`wyd-protocol/lacunas.md` §2.2](https://github.com/lucascantarelli/wyd/blob/develop/planejamento-1.0.0/wyd-protocol/lacunas.md); [`como-capturar.md`](../wyd-protocol/goldens/como-capturar.md); [`proposta-decisoes-protocol.md` §6](../wyd-protocol/proposta-decisoes-protocol.md).

### LCLI-03

**O que é.** O nome do operador da primeira ficha de captura e a assinatura da segunda leitura. Lucas Cantarelli (dono do produto) é o QA designado e não pode ser o operador; o nome do operador ainda não existe.

**Por que está bloqueado e por quem.** Falta a primeira ficha assinada; a captura A1 também não foi executada. Depende do dono do produto/QA e do operador nomeado.

**Evidência/artefato que falta.** Ficha com data, cenário, operador nomeado e assinatura da segunda leitura por pessoa diferente de quem capturou (D6).

**Opções.**

| Opção | O que é | Custo/risco/esforço | Recomendação |
|---|---|---|---|
| **A** | Nomear um operador distinto do QA e assinar a ficha na primeira captura A1 | custo de coordenação; risco baixo; esforço baixo | **recomendada** |
| B | Usar o próprio QA como operador e revisor | custo zero; viola D6 e a independência da segunda leitura; esforço baixo | não recomendada |
| C | Deixar a ficha para depois da captura | custo zero agora; mantém LPRT-03/06 e A1 bloqueados; esforço zero | não fecha |

**Por que A.** Preserva a separação operador/revisor já decidida e é pré-requisito da prova de captura. B economiza uma pessoa, mas torna a aceitação inválida.

**Pronto quando.** A ficha da primeira captura tiver operador nomeado, data e assinatura de segunda leitura independente; `LCLI-03` fechar no ledger do `wyd-client`.

**Origem.** [`wyd-client/lacunas.md` §2 e §2.2](https://github.com/lucascantarelli/wyd/blob/develop/planejamento-1.0.0/wyd-client/lacunas.md); [`como-capturar.md`](../wyd-protocol/goldens/como-capturar.md).

### LCLI-05

**O que é.** A evidência do titular sobre qual conteúdo do cliente e quais marcas podem ser copiados, modificados, publicados ou redistribuídos. A política e o responsável nominal estão definidos; a distribuição segue congelada.

**Por que está bloqueado e por quem.** O pedido `Q-IP-1`/`Q-IP-2` está redigido e **não enviado**; falta o dono confirmar signatário/dados, abrir o pedido e obter a resposta do titular, com revisão jurídica. Depende de produto + revisão jurídica + terceiro.

**Evidência/artefato que falta.** Pedido enviado e protocolado, resposta escrita do titular por classe de conteúdo, registro de evidência de `Q-IP-6` e atualização do inventário. Nenhum arquivo é liberado por presença local.

**Opções.**

| Opção | O que é | Custo/risco/esforço | Recomendação |
|---|---|---|---|
| **A** | Enviar o pedido já redigido, manter tudo congelado e liberar somente o que a resposta escrita autorizar, item a item | custo de revisão jurídica e espera; risco de resposta parcial; esforço médio | **recomendada** |
| B | Manter congelado sem enviar o pedido | custo zero; nenhum avanço e nenhuma evidência; esforço zero | não recomendada |
| C | Distribuir ou publicar por conta própria, sem evidência | custo zero aparente; risco jurídico e de retirada; esforço baixo | proibida |

**Por que A.** É o único caminho que produz evidência verificável sem presumir titularidade; B apenas posterga e C troca risco técnico por risco jurídico. A distribuição continua congelada até a resposta.

**Pronto quando.** O pedido estiver enviado e protocolado; a resposta do titular cobrir as classes inventariadas; `Q-IP-6` registrar a evidência por item; o ledger e a issue [#199](https://github.com/lucascantarelli/wyd/issues/199) refletirem o escopo liberado — e somente ele.

**Origem.** [`wyd-client/lacunas.md` §2](https://github.com/lucascantarelli/wyd/blob/develop/planejamento-1.0.0/wyd-client/lacunas.md); [`pedido-ao-titular.md`](../wyd-client/specs/pedido-ao-titular.md); [`propriedade-intelectual-e-distribuicao.md`](../wyd-client/specs/propriedade-intelectual-e-distribuicao.md); issue [#199](https://github.com/lucascantarelli/wyd/issues/199).

### LNET-01

**O que é.** Os valores medidos de limite por conexão, teto por IP, teto por shard, timeout de handshake, timeout ocioso e rate-limit. A política estrita (`Q-NET-1` = A) já está formalizada em `ADR-NET-0001`; falta o número.

**Por que está bloqueado e por quem.** A execução de prontidão encontrou 0 `Cargo.toml` em `crates/wyd-net`, 0 listeners e 0 cenário de carga. A medição foi autorizada, mas depende do `ECAP`/servidor mínimo e do CI efêmero; dono QA + `wyd-ops`, com arquitetura revisando.

**Evidência/artefato que falta.** Relatório datado da escada de carga com conexões aceitas/recusadas, recusas por motivo, timeouts e ambiente; valores aplicados no transporte e teste de recusa.

**Opções.**

| Opção | O que é | Custo/risco/esforço | Recomendação |
|---|---|---|---|
| **A** | Executar a escada de carga e congelar valores absolutos após o relatório | custo de runtime/CI; risco de revelar limite abaixo do esperado; esforço médio | **recomendada** |
| B | Adotar política adaptativa por taxa de recusa | custo de implementação; comportamento imprevisível para jogador/teste; esforço alto | não recomendada |
| C | Fixar valores provisórios agora | custo zero; número sem fonte vira promessa; esforço baixo | não recomendada |

**Por que A.** `ADR-NET-0001` já fixou política estrita; o que falta é o valor que o teste de recusa consegue provar. B e C trocam medição por estimativa.

**Pronto quando.** O relatório de `medicoes-lnet-01-lbot-03.md` §4.6 estiver anexado com ambiente/data/commit; os valores estiverem aplicados e o teste de recusa por motivo passar; `LNET-01` fechar no ledger do `wyd-net`.

**Origem.** [`wyd-net/lacunas.md` §4](https://github.com/lucascantarelli/wyd/blob/develop/planejamento-1.0.0/wyd-net/lacunas.md); [`medicoes-lnet-01-lbot-03.md`](planos/medicoes-lnet-01-lbot-03.md); [`ADR-NET-0001`](../wyd-net/arquitetura/adr/adr-net-0001-politica-de-limites.md).

### LBOT-03

**O que é.** O limiar de carga dos bots: conexões, duração, P50/P99, taxa de sucesso e o que reprova. A política (`Q-BOT-3` = A: medir, congelar, parar/drenar/reportar) já foi decidida; falta o número.

**Por que está bloqueado e por quem.** Não há `crates/wyd-bots`, cenário nem listener; a primeira medição não foi executada. Depende do servidor mínimo, do seed sintético e do CI efêmero; dono QA + `wyd-ops`.

**Evidência/artefato que falta.** Cenário em degraus, relatório com P50/P99, taxa de sucesso e recusas por motivo; limiar congelado com ambiente/data e job que reprova abaixo dele.

**Opções.**

| Opção | O que é | Custo/risco/esforço | Recomendação |
|---|---|---|---|
| **A** | Medir em degraus, congelar o primeiro degrau de saturação e transformá-lo em gate | custo de runtime/CI; risco de o limite ser baixo; esforço médio | **recomendada** |
| B | Usar um teto provisório de infraestrutura | custo baixo; mede o processo, não o jogo; esforço baixo | não recomendada |
| C | Fixar conexões/latência alvo agora | custo zero; número sem fonte; esforço baixo | não recomendada |

**Por que A.** É a política já aprovada e a única que produz um gate com causa medida. B pode bloquear jogador legítimo; C inventa promessa.

**Pronto quando.** O cenário e o relatório de `medicoes-lnet-01-lbot-03.md` §4.6 existirem; o limiar estiver congelado com data/ambiente; o job reprovar abaixo dele; `LBOT-03` fechar no ledger do `wyd-bots`.

**Origem.** [`wyd-bots/lacunas.md` §2](https://github.com/lucascantarelli/wyd/blob/develop/planejamento-1.0.0/wyd-bots/lacunas.md); [`proposta-decisoes-bots.md` §4](../wyd-bots/proposta-decisoes-bots.md); [`medicoes-lnet-01-lbot-03.md`](planos/medicoes-lnet-01-lbot-03.md).

### A11

**O que é.** O charset aceito no `GameLogin` do cliente 7662. Até a captura, o campo fica permissivo (1..16, sem espaço nas bordas) e marcado `H`; a validação estrita não tem fonte.

**Por que está bloqueado e por quem.** Depende da captura do `GameLogin` e da leitura do cliente 7662; a frente `wyd-server` não tem runtime operado. Dono: frente `wyd-server`/`wyd-client` + QA.

**Evidência/artefato que falta.** Frame `GameLogin` capturado, charset e limites provados, golden e segunda leitura; depois, validação e testes no servidor.

**Opções.**

| Opção | O que é | Custo/risco/esforço | Recomendação |
|---|---|---|---|
| **A** | Capturar o `GameLogin` com o mesmo `ECAP`/A1 e então restringir a validação ao charset provado | custo de captura; risco de rejeitar nome legítimo se a prova for incompleta; esforço médio | **recomendada** |
| B | Manter 1..16 permissivo indefinidamente | custo zero; aceita entrada não provada e mantém `H`; esforço zero | aceitável só como fallback |
| C | Restringir a ASCII agora, sem captura | custo baixo; pode recusar nome válido do cliente; esforço baixo | não recomendada |

**Por que A.** Troca a hipótese permissiva por uma regra com `V`; B não fecha e C arrisca compatibilidade. O fallback B continua válido até A produzir o golden.

**Pronto quando.** O frame e o golden estiverem registrados; charset/limites estiverem provados; a validação e os testes de aceitação/recusa passarem; `A11` fechar no ledger do `wyd-db`.

**Origem.** [`wyd-db/lacunas.md` §2.2](https://github.com/lucascantarelli/wyd/blob/develop/planejamento-1.0.0/wyd-db/lacunas.md); [`como-capturar.md`](../wyd-protocol/goldens/como-capturar.md); [`endpoint-de-captura.md`](../wyd-server/specs/endpoint-de-captura.md).

### A20

**O que é.** O segundo fator obrigatório para staff (`admin`/`moderator`) antes de o portal ir a público. O escopo e o DDL de `portal.person_authenticator` no lote `0031` já estão decididos; falta implementar as telas/rotas e os testes.

**Por que está bloqueado e por quem.** A execução é da frente `wyd-portal`: sessão em `WYDPOR-F0`, middleware de papel em `F4` e a tela `/admin/seguranca` com recuperação em `F5`; o titular manteve a classificação `externa` até a execução. Depende do dono do portal + produto.

**Evidência/artefato que falta.** `/admin/seguranca` e o fluxo TOTP de staff implementados, recuperação de staff testada, testes de admissão/recusa e a condição de “portal público” ligada a esse gate.

**Opções.**

| Opção | O que é | Custo/risco/esforço | Recomendação |
|---|---|---|---|
| **A** | Implementar o MFA de staff em `WYDPOR-F0`/`F4`/`F5` e só abrir o portal público depois dos testes | custo de implementação; risco de atrasar o portal; esforço médio | **recomendada** |
| B | Abrir o portal público sem MFA de staff e adicionar depois | custo baixo agora; risco de conta privilegiada sem segundo fator; esforço baixo | não recomendada |
| C | Adiar o portal público até a fatia de segurança | custo de espera; risco de bloquear outras telas; esforço baixo | aceitável, mas A entrega o mesmo gate com escopo claro |

**Por que A.** A política e o DDL já existem; implementar antes da exposição é o único caminho que satisfaz a decisão sem aceitar risco de staff. B viola a condição registrada.

**Pronto quando.** A rota `/admin/seguranca` e o fluxo TOTP estiverem implementados; o ADR de MFA de staff estiver aceito; os testes de staff (adesão, login, recuperação e recusa) passarem; o portal público estiver condicionado a esse gate; `A20` fechar no ledger do `wyd-db`.

**Origem.** [`wyd-db/lacunas.md` §2.2](https://github.com/lucascantarelli/wyd/blob/develop/planejamento-1.0.0/wyd-db/lacunas.md); [`rotas-e-permissoes.md` §3](../wyd-portal/arquitetura/rotas-e-permissoes.md); [`ddl-confirmado.md` §5.20](https://github.com/lucascantarelli/wyd/blob/develop/planejamento-1.0.0/wyd-db/ddl-confirmado.md).

### M21

**O que é.** A implementação das telas e rotas do portal: área pública e administrativa (NOC, jogadores/contas, economia, conteúdo/CMS, launcher, logs e segurança), com configuração do servidor, monitoramento e dashboards. O escopo e as rotas já estão planejados; falta executar as fatias `WYDPOR-F0`–`F11`.

**Por que está bloqueado e por quem.** A execução é da frente `wyd-portal`; a classificação `externa` foi mantida pelo titular até a execução. Depende do dono do portal + produto.

**Evidência/artefato que falta.** Rotas e telas do §3 implementadas, testes de contrato/permissão, estados de erro e carregamento, e as fatias do roadmap com DoD fechado.

**Opções.**

| Opção | O que é | Custo/risco/esforço | Recomendação |
|---|---|---|---|
| **A** | Executar o escopo planejado de `WYDPOR-F0`–`F11`, priorizando segurança e operação | custo de implementação; risco de prazo; esforço alto | **recomendada** |
| B | Entregar um MVP com poucas telas e reclassificar o restante | custo menor; risco de mudar escopo já decidido; esforço médio | só com decisão explícita do dono |
| C | Manter apenas o planejamento e não implementar o portal | custo zero agora; portal e administração continuam inexistentes; esforço zero | não fecha |

**Por que A.** O escopo e as rotas já foram decididos e mapeados; executar as fatias evita reabrir a decisão. B é uma mudança de escopo, não uma opção de execução.

**Pronto quando.** As rotas/telas do §3 estiverem implementadas e testadas; as fatias `WYDPOR-F0`–`F11` tiverem evidência de DoD; `M21` fechar no ledger do `wyd-db`.

**Origem.** [`wyd-db/lacunas.md` §2.3](https://github.com/lucascantarelli/wyd/blob/develop/planejamento-1.0.0/wyd-db/lacunas.md); [`rotas-e-permissoes.md`](../wyd-portal/arquitetura/rotas-e-permissoes.md); [`roadmap do portal`](../wyd-portal/roadmap/roadmap.md).

## 4. Perguntas numeradas por respondedor

As perguntas abaixo são a forma curta de responder. Cada uma aponta para o item detalhado acima e para o ledger; a resposta só vale depois de registrada. São **11 itens**; o item `LCLI-05` tem **duas respostas separadas** porque a autorização de envio e a evidência do titular pertencem a pessoas diferentes: **6a** (dono) e **6b** (titular).

### Infra/owner

1. **`OPS-04`** — Você aprova medir 2–3 alternativas de VPS com cotação, backup e RPO/RTO antes de provisionar, em vez de escolher provedor sem esses dados? **Default recomendado: A (medir e decidir com dados; produção desligada até a aprovação).**
2. **`OPS-05`** — Você autoriza criar o board único do Project com os campos/views propostos e importar as 111 issues? **Default: A (criar agora).**
3. **`LCLI-03`** — Você nomeia um operador distinto do QA para a primeira captura e assina a segunda leitura? **Default: A (nomear operador e assinar a ficha).**
4. **`A20`** — Você confirma o MFA de staff como condição para o portal público e autoriza implementá-lo em `WYDPOR-F0`/`F4`/`F5`? **Default: A (implementar antes de abrir).**
5. **`M21`** — Você mantém o escopo completo das telas/rotas do portal e autoriza executar `WYDPOR-F0`–`F11`? **Default: A (executar o escopo planejado).**
6a. **`LCLI-05` — autorização do dono** — Você confirma o signatário e os dados do pedido, autoriza o envio ao titular e mantém a distribuição congelada até a resposta escrita? **Default: A (enviar e manter congelado; liberar somente o escopo escrito).**

### Terceiro

6b. **`LCLI-05` — resposta do titular** — O titular confirma por escrito, por classe de conteúdo, o que pode ser copiado, modificado, publicado ou redistribuído, com signatário e data? **Default: aguardar a resposta escrita e não liberar nada sem ela.**

### Runtime

7. **`LPRT-03`** — Você confirma a execução da captura A1 via `ECAP` + CLI, com segunda leitura e golden, para provar a origem da `KeyWord`? **Default: A (executar A1).**
8. **`LPRT-06`** — Você confirma as capturas B/C com personagem sintético e seed para provar encoding e terminador? **Default: A (executar B/C e registrar golden).**
9. **`LNET-01`** — Você autoriza executar a escada de carga e congelar os limites absolutos no relatório, sem valor provisório? **Default: A (medir e congelar).**
10. **`LBOT-03`** — Você autoriza medir o limiar de carga dos bots e transformar o primeiro degrau de saturação no gate? **Default: A (medir e congelar).**
11. **`A11`** — Você autoriza capturar o `GameLogin` do cliente 7662 e só então restringir o charset, mantendo o fallback permissivo até a prova? **Default: A (capturar e provar).**

## 5. Manutenção

1. O dono responde na pergunta numerada ou na mensagem de origem.
2. A resposta é registrada no `lacunas.md` do sistema e em `docs/lacunas.md` §4, com data, dono e artefato.
3. Só então o estado muda nesta pauta; uma resposta verbal não altera o ledger.
4. Se a resposta for "não", a objeção volta ao dono e o item continua bloqueado.
5. Se um item fechar, ele sai do índice e permanece no histórico do ledger; esta página não mantém uma segunda cópia do estado.

## 6. Limites

- Não cria issue, board, commit, push, deploy, ADR ou mensagem; nada foi enviado.
- Não substitui o ledger nem os documentos donos: consolida a leitura e o critério de pronto.
- Não reabre decisão já fechada (`OPS-03.1`–`OPS-03.5`, `D-06`, `Q-SERIE-1`, `Q-SEP-1`, `LCLI-02`).
- Não inventa provedor, custo, RPO/RTO, limite, charset ou data; onde o dado não existe, o critério de pronto exige a medição.
