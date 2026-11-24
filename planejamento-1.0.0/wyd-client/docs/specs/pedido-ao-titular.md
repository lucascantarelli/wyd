# Pedido ao titular — proveniência e autorização escrita (escopo B)

> Pergunta: "quem pode autorizar a proveniência e o uso da árvore inteira do cliente, e o que exatamente estamos pedindo?"
> Público: dono do produto, revisão jurídica, `wyd-client` e `wyd-ops`
> Dono: equipe do `wyd-client`; aprovação de direitos por Lucas Cantarelli (dono do produto)
> Base: `Q-IP-1`/`Q-IP-2`/`Q-IP-6`; `docs/specs/inventario-da-arvore.md`; `docs/specs/propriedade-intelectual-e-distribuicao.md` §4 e §8
> Revisado em: 2026-11-23

**Estado:** pacote em `P`, pronto para revisão e envio pelo dono, **não enviado** em 2026-11-23. Nenhuma
resposta foi recebida ou presumida. A distribuição segue **congelada**; `LCLI-05` continua `proposta`;
`LCLI-02` continua aberta até a evidência do titular, e nada aqui autoriza patch, release, captura ou
evidência de direitos.

## 1. Destinatário

O pedido se dirige ao **titular ou representante autorizado** dos direitos aplicáveis ao cliente
`WYD2 / With Your Destiny`, não a um usuário, servidor privado ou fórum.

| Item | Definição |
|---|---|
| Destinatário | titular ou representante legal com poderes para falar sobre proveniência, reprodução, modificação e redistribuição do cliente |
| Rota provisória | **Raid Hut Licenciamento de Jogos LTDA.**, que se apresenta publicamente como licenciante global exclusivo de publicação de *With Your Destiny* (`https://wydglobal.raidhut.com/en/support/about`) |
| Por que provisória | a declaração pública é **indício de rota**, não prova de titularidade do cliente local nem de que a empresa possa autorizar modificação ou redistribuição; o dono e a revisão jurídica confirmam antes do envio |
| Canal público | central de suporte/tickets em `https://status.raidhut.com/`; **canal escolhido em 2026-11-23**, com pedido explícito de encaminhamento ao jurídico; se a revisão jurídica indicar canal contratual ou jurídico, ele substitui o formulário |
| Canal a evitar | `denuncia@raidhut.com`, descrito pelo próprio site como canal de denúncia de pirataria; não é canal adequado para este pedido |
| Dados que faltam | nome do signatário, endereço jurídico e prova de poderes; não inventar. Se não forem confirmáveis, o pedido fica retido |

A escolha do destinatário não conclui que ele seja o titular. Se ele responder que não é o titular ou que
não pode autorizar a totalidade, o pedido será redirecionado ou registrado como recusa/limitação, sem
transformar a resposta em permissão.

## 2. Objeto declarado

O pedido tem por objeto a **árvore inteira** do cliente local, na forma do
[inventário técnico de metadados](inventario-da-arvore.md):

- **7.147 arquivos regulares**;
- **576.124.039 bytes** de tamanho lógico agregado;
- **30 classes** por extensão técnica;
- raiz local `wyd-client/files/`, com `client.zip` e `client/` já extraído;
- inventário item a item em `inventario-arvore-cliente.tsv` (caminho relativo, tamanho e `sha256`).

Este pacote declara que o objeto é a **ampliação da escolha B — árvore inteira**, registrada em
`propriedade-intelectual-e-distribuicao.md` §8.1. A árvore inclui assets e screenshots; **nenhum
conteúdo, screenshot, logo ou binário é anexado a este pacote**. O anexo técnico é somente o inventário
de metadados.

A declaração de escopo não é pedido de publicação imediata e não afirma que a árvore seja toda de
titularidade do destinatário. Ela serve para que o titular responda de forma itemizada, sem ambiguidade
sobre o que está sendo considerado.

## 3. O que se pede

1. **Proveniência verificável** de cada classe/item: origem, titular ou titulares, e se o destinatário
   age em nome próprio ou de terceiro.
2. **Autorização escrita** para reproduzir, modificar e redistribuir os itens indicados, no contexto do
   patch e do release do projeto, com a lista de itens e operações coberta.
3. **Delimitação da autorização**: canais, territórios, prazo, possibilidade de revogação, condições de
   atribuição/aviso e necessidade de autorização de terceiros.
4. **Resposta itemizada**: para cada classe ou item, indicar **autorizado**, **não autorizado** ou
   **condicionado**, com a condição correspondente. Autorização genérica sem lista não será tratada como
   cobertura de item não identificado.
5. **Forma da evidência**: documento escrito com identificação do signatário, data, poderes ou vínculo
   com o titular, e canal de envio verificável. Se a resposta vier por ticket, preservar o protocolo e o
   conteúdo integral.
6. **Indicação de terceiros**: se houver desenvolvedora, licenciante ou outro titular cuja autorização
   também seja necessária, informar quem é e como contatá-lo.

## 4. O que NÃO se pede

- **Não** se pede o envio de arquivos, binários, assets, screenshots, logos ou conteúdo do cliente.
- **Não** se pede aprovação de patch, release, publicação, captura ou operação do jogo; a resposta, se
  vier, será apenas evidência para o registro e para o gate de release.
- **Não** se pede cessão de direitos, exclusividade, renúncia, pagamento ou alteração de licença do
  projeto.
- **Não** se pede conta, personagem, IP, chat, credencial, token, senha, PIN ou qualquer dado de jogador.
- **Não** se pede que o destinatário confirme a identidade do build-alvo, o protocolo ou o comportamento
  do cliente; isso é medição técnica separada.
- **Não** se presume resposta, prazo ou autorização por silêncio.

Se o titular oferecer condições que exijam decisão de negócio ou jurídica, elas não serão aceitas
automaticamente: voltam ao dono como nova decisão registrada.

## 5. Como a resposta será registrada

Cada linha autorizada, negada ou condicionada terá registro próprio, com os campos de
`propriedade-intelectual-e-distribuicao.md` §4. O registro mínimo por item/classe é:

| Campo | Conteúdo exigido |
|---|---|
| Item/classe | caminho ou conjunto de caminhos do inventário |
| Origem | titular, representante e documento de origem |
| Autorização | sim, não ou condicional, com a condição |
| Operações | reproduzir, modificar, redistribuir, empacotar ou outra, delimitada |
| Escopo | canais, territórios, prazo e revogação |
| Aprovador | nome, cargo/função, poderes e data |
| Evidência | documento, protocolo do ticket, data e arquivo de referência |
| Destino | patch, release, site ou nenhum |
| Revisão jurídica | quem validou a forma e quando |
| Estado | liberado, bloqueado ou pendente |

O registro fica ligado ao gate de release (`Q-IP-6`): **sem registro completo, não há release**. Uma
resposta favorável para uma classe não libera outra. Uma resposta negativa ou parcial mantém os itens
não cobertos fora do patch e da distribuição, com `LCLI-05` ainda `proposta` e `LCLI-02` aberta.

## 6. Minuta para revisão e envio

**Assunto:** Pedido de proveniência e autorização escrita — cliente WYD2 / With Your Destiny

> Prezados,  
>  
> O projeto WYD 1.0.0 é uma reimplementação independente e está em fase de planejamento. Temos uma
> cópia local do cliente WYD2 e precisamos confirmar a proveniência e obter autorização escrita antes de
> qualquer uso, modificação ou redistribuição.  
>  
> O pedido cobre a árvore inteira do cliente, com **7.147 arquivos** e **576.124.039 bytes**, inventariados
> apenas por caminho relativo, tamanho e SHA-256. Nenhum conteúdo do cliente segue anexado.  
>  
> Pedimos: (a) a proveniência verificável de cada classe/item; (b) a autorização escrita para reproduzir,
> modificar e redistribuir os itens indicados; (c) a delimitação de canais, territórios, prazo, revogação
> e condições; e (d) a indicação de terceiros cuja autorização também seja necessária.  
>  
> Não pedimos o envio de arquivos, não pedimos aprovação de release ou publicação e não pedimos cessão,
> exclusividade ou renúncia. A resposta pode ser itemizada por classe ou arquivo, indicando autorizado,
> não autorizado ou condicionado.  
>  
> Podem responder por este canal ou indicar o contato jurídico competente. Obrigado.

A minuta é um rascunho de revisão. O envio depende de o dono confirmar destinatário, signatário e canal;
o agente não envia, não abre ticket e não registra resposta presumida.

## 7. Estado e próximo ato

- **Decisão de canal (2026-11-23):** **A — enviar pelo suporte da Raid Hut**, com pedido explícito de
  encaminhamento ao jurídico. O **dono** envia; o agente não abre ticket nem envia mensagem.
- **Pronto para envio:** sim; falta o dono confirmar signatário/dados e abrir o ticket.
- **Enviado:** não.
- **Resposta:** nenhuma.
- **Paralelo:** enquanto aguarda, priorizar `wyd-db` (decisão do dono; não altera escopo nem contagens).
- **Anexo técnico:** inventário de metadados; nenhum conteúdo do cliente.
- **Efeito:** nenhum. A distribuição segue congelada; `LCLI-05` continua `proposta`; `LCLI-02` continua
  aberta; nenhuma decisão autoriza patch, release, captura ou evidência de direitos.
