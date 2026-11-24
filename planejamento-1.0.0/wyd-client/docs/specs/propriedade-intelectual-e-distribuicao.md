> Pergunta: "quais arquivos, marcas e materiais do cliente podem ser modificados, publicados e distribuídos pelo WYD 1.0.0?"
> Público: dono do produto, `wyd-client`, `wyd-launcher`, `wyd-ops` e revisão jurídica
> Dono: equipe do `wyd-client` (inventário); aprovação de direitos por Lucas Cantarelli (dono do produto), responsável nominal desde 2026-11-23
> Base: `wyd-client/lacunas.md` (`LCLI-02`); `wyd-launcher/lacunas.md` (`LLCH-01`, `LLCH-03`, `LLCH-04`); `wyd-client/registro-do-cliente.md`; `docs/specs/inventario-da-arvore.md`; `docs/specs/pedido-ao-titular.md`; `AGENTS.md` §§1, 3, 5.7–5.9
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23) — proposta (`P`, 2026-11-23). Em 2026-11-23 o dono decidiu as políticas de `Q-IP-3`–`Q-IP-6` e iniciou o pedido externo de `Q-IP-1`/`Q-IP-2`; o inventário item a item e o pacote do pedido foram preparados, mas o pedido **não foi enviado**. A proposta **continua aberta**, a distribuição segue **congelada** e nenhuma decisão aqui autoriza patch, release, captura ou evidência de direitos. Esta página não conclui titularidade, licença, permissão ou legalidade de uso, modificação, publicação ou redistribuição. Não é parecer jurídico nem autorização para distribuir arquivo.

## 1. Objetivo e limite

Esta proposta busca impedir que o projeto confunda **ter acesso local a um arquivo** com **ter direito de publicá-lo ou distribuí-lo**. O inventário medido em `registro-do-cliente.md` descreve arquivos presentes na máquina; não prova origem, titularidade, licença nem permissão de redistribuição (`V` quanto ao inventário; direitos permanecem `P`, sem evidência registrada). O inventário item a item está em [`inventario-da-arvore.md`](inventario-da-arvore.md) e o pacote do pedido em [`pedido-ao-titular.md`](pedido-ao-titular.md); ambos são metadados, não prova de direito.

A política futura precisa classificar separadamente: (a) código e documentação escritos para a reimplementação; (b) binários e recursos do cliente existente; (c) configurações e arquivos modificados para o patch; (d) capturas, imagens, nomes e marcas usados em documentação, site ou launcher. Uma decisão sobre uma classe não se estende automaticamente às demais.

## 2. Perguntas e estado

| ID | Pergunta | Dono que precisa responder | O que bloqueia |
|---|---|---|---|
| Q-IP-1 | Qual é a proveniência verificável de cada classe de arquivo e quem detém os direitos aplicáveis? | `wyd-client` inventaria; dono do produto obtém evidência do titular ou de representante autorizado | Classificar itens sem origem comprovada como distribuíveis; aprovar `LCLI-02` |
| Q-IP-2 | Há autorização expressa para reproduzir, modificar ou redistribuir executável, assets, sons, fontes, textos, mapas, imagens ou arquivos como `strdef.bin`? Quais itens, operações, canais, territórios e prazo ela cobre? | dono do produto + titular dos direitos; revisão jurídica qualificada | Incluir ou modificar esses itens em patch, instalador, site, CDN, release ou pacote público |
| Q-IP-3 | O que pode entrar no repositório público: conteúdo, trecho, screenshot, hash, metadado técnico ou somente referência externa? | dono do produto + revisão jurídica; `wyd-client` aplica a classificação | Versionar ou publicar conteúdo do cliente; qualquer conteúdo fica fora até a decisão documentada |
| Q-IP-4 | Como se separam a licença do código novo do projeto e a situação jurídica dos arquivos preexistentes ou derivados? | dono do produto + revisão jurídica; `wyd-ops` mantém os metadados de release | Escolher uma licença que pareça abranger material de terceiros ou publicar release misto sem classificação |
| Q-IP-5 | Quais nomes, logos, marcas, artes e capturas de tela podem aparecer no portal, documentação e launcher, e com que atribuição ou aviso? | dono do produto + titular dos direitos; `wyd-portal`/`wyd-launcher` listam usos pretendidos | Divulgar página promocional, screenshot, identidade visual ou nome como se houvesse autorização |
| Q-IP-6 | Que evidência documenta cada autorização, suas condições, revogação e revisão antes de uma release? | dono do produto define aprovador; `wyd-ops` liga evidência à publicação; revisão jurídica valida a forma | Aprovar release, espelho, atualização ou pacote sem prova rastreável de escopo autorizado |

**Decisões de 2026-11-23:** `Q-IP-3`–`Q-IP-6` têm política registrada em §8; `Q-IP-1`/`Q-IP-2` são pedido externo iniciado, sem resposta do titular; `Q-IP-2` tem escopo **B (árvore inteira)** decidido em §8.1. Nenhuma das seis está fechada como evidência.

## 3. Donos e fronteiras

| Responsabilidade | Dono proposto | Limite |
|---|---|---|
| Inventariar os itens e suas proveniências, sem publicar o conteúdo | `wyd-client` | Não pode converter presença local em prova de direito |
| Aprovar o escopo de uso/distribuição e obter autorização documentada | Lucas Cantarelli (dono do produto), responsável nominal por direitos desde 2026-11-23 | Decisão de negócio e permissão; não substitui análise jurídica |
| Avaliar licenças, titularidade e condições de uso | titular ou representante autorizado, com revisão jurídica qualificada | Não presumir autorização nem emitir conclusão pela equipe técnica |
| Definir conteúdo do patch | `wyd-client`, com a lacuna `LCLI-02` | Só após classes e itens distribuíveis estarem liberados |
| Definir manifesto, integridade e assinatura | `wyd-launcher` (`LLCH-01`, `LLCH-03`) | Restrição técnica não concede direito de redistribuição |
| Empacotar, hospedar e publicar release | `wyd-ops` / `wyd-launcher` (`LLCH-04`) | Só consome itens liberados e evidência aprovada |

O responsável nominal por direitos é **Lucas Cantarelli** (dono do produto), definido em 2026-11-23; quem assina a revisão jurídica segue em aberto. A distribuição está **congelada**: a nomeação não aprova esta proposta nem autoriza publicar. Não se atribui ao `wyd-ops` o poder de conceder direitos, nem ao launcher o de decidir conteúdo do cliente.

## 4. Registro mínimo para uma futura decisão

Antes de liberar qualquer item, a decisão deverá registrar: identificador/classe do item (não necessariamente o conteúdo); origem e evidência; titular ou parte autorizadora; usos autorizados (copiar, modificar, distribuir); canais e território; prazo e condições; restrições de atribuição; aprovador e data; e destino operacional (repositório, pacote privado/público, launcher, site ou nenhum). Se a permissão não puder ser demonstrada, o estado permanece **não autorizado para publicação/distribuição** até decisão — não se infere permissão por silêncio.

A lista de itens e o detalhe da evidência deverão ter acesso e armazenamento apropriados; esta página não manda versionar contrato, arquivo binário ou dado confidencial no repositório público.

## 5. Bloqueios e não-bloqueios

**Bloqueia até evidência:** conteúdo de `LCLI-02`; composição final de artefatos do manifesto (`LLCH-01`); empacotamento e distribuição de launcher/cliente (`LLCH-04`); publicação de recursos, screenshots, marcas ou imagens do cliente no site; e qualquer release que inclua material cuja autorização não esteja comprovada. A assinatura técnica de `LLCH-03` não resolve essa autorização. As políticas de `Q-IP-3`–`Q-IP-6` não mudam essa lista: elas definem o que entra no repositório público, o registro de prova, a licença e o uso de marcas; **nada disso libera um item**.

**Não bloqueia por si só:** inventariar localmente arquivos sem expor conteúdo, registrar hashes/metadados já permitidos pelas regras do repositório, planejar ou implementar código original sem incorporar material não liberado, nem capturar tráfego sintético conforme `LPRT-10` e `LCLI-04`. Capturas que reproduzam conteúdo protegido continuam sujeitas à classificação desta proposta antes de publicação.

## 6. Como esta proposta pode fechar

O responsável por direitos está nomeado; as políticas de `Q-IP-3`–`Q-IP-6` estão decididas; falta **concluir** o pedido externo de `Q-IP-1`/`Q-IP-2`, a evidência por classe e a validação jurídica. `wyd-client`, `wyd-launcher` e `wyd-ops` refletem a decisão nos respectivos artefatos e lacunas, sem duplicar o registro de direitos. Até lá, esta página permanece `proposta`, `LCLI-05` continua `proposta`, `LCLI-02` continua aberta, a distribuição segue congelada e nada autoriza redistribuição.

## 7. Nomeação de 2026-11-23

**Decisão do dono:** Lucas Cantarelli (dono do produto) assume a responsabilidade nominal por direitos e mantém a **distribuição congelada**. A nomeação não responde `Q-IP-1`/`Q-IP-2`, não aprova esta proposta e não autoriza publicação; o inventário técnico pode começar sem expor conteúdo. Quem assina a revisão jurídica segue em aberto.

## 8. Decisões do dono em 2026-11-23

**Decisor:** Lucas Cantarelli (dono do produto), responsável nominal por direitos (`LCLI-05`). As decisões abaixo são de **política**; não são evidência de titularidade nem autorização de distribuição.

| ID | Decisão | O que autoriza | O que não autoriza |
|---|---|---|---|
| `Q-IP-1` | Pedido de proveniência ao titular/representante **iniciado**; inventário técnico em paralelo | inventariar classes/itens com caminho, tamanho e `sha256`, sem expor conteúdo; formular o pedido | classificar item como distribuível; publicar conteúdo; fechar `LCLI-02` |
| `Q-IP-2` | Pedido de autorização escrita ao titular **iniciado**; escopo **B = árvore inteira** (7.147 arquivos/567 MB, incluindo assets e screenshots) decidido em 2026-11-23; inventário e pacote preparados, pedido não enviado | manter o pacote pronto para envio e responder ao titular com a lista item a item | incluir, copiar, modificar ou redistribuir qualquer item; liberar patch ou release; tratar a resposta como evidência sem registro por item |
| `Q-IP-3` | No repositório público, **só hash, metadado e referência externa** | registrar metadados e referências; avançar o inventário documentado | conteúdo, trecho, screenshot, logo ou binário do cliente no git |
| `Q-IP-4` | **GPL v2** para código original; binários, assets e configurações do cliente **explicitamente excluídos**; exclusões registradas no release | licenciar o código original sob GPL v2; declarar a exclusão | aplicar a GPL a material de terceiros; distribuir ativo do cliente |
| `Q-IP-5` | **Nenhum logo, arte ou screenshot do cliente** até autorização escrita; identidade original e referência nominativa | identidade visual original; citar o nome de forma nominativa | usar marca, arte ou screenshot do cliente |
| `Q-IP-6` | Release **bloqueada** sem registro por item com origem, autorização, escopo, prazo, aprovador e destino; validação jurídica posterior da forma | construir o registro e o gate de release | aprovar release sem registro completo ou sem escopo comprovado |

### 8.1 Escopo do pedido ao titular (`Q-IP-2`) — 2026-11-23

**Decisor:** Lucas Cantarelli (dono do produto). **Decisão:** **B — árvore inteira**. O pedido ao titular passa a cobrir os 7.147 arquivos/567 MB do cliente, incluindo assets e screenshots, itemizado por arquivo/classe e com as operações de reproduzir, modificar e redistribuir. **Custo estimado:** 1–2 sessões de inventário e revisão jurídica ampla (semanas), com maior chance de recusa ou resposta genérica. **Alternativas recusadas:** A (pacote mínimo), C (só dados/config) e D (adiar).

**Canal de envio (2026-11-23):** suporte da Raid Hut, com pedido explícito de encaminhamento ao jurídico; o dono envia, o agente não. O envio ainda não ocorreu e o pacote está pronto.

**Efeito:** apenas o **escopo do pedido** muda; nada é liberado até a resposta do titular e o registro por item de `Q-IP-6`. `Q-IP-3` continua proibindo conteúdo no repositório público, `Q-IP-5` continua proibindo marca/arte/screenshot até autorização escrita, `LCLI-05` continua `proposta` e `LCLI-02` aberta. Patch, release, captura e evidência de direitos não estão autorizados; nenhuma linha acima altera as contagens.

## 9. Trabalho técnico preparatório (não é pergunta)

| Passo | Dono | Saída | Limite |
|---|---|---|---|
| Inventário item a item com caminho, tamanho e `sha256` | `wyd-client` | [`inventario-da-arvore.md`](inventario-da-arvore.md) + `inventario-arvore-cliente.tsv`; 7.147 itens, 576.124.039 bytes, 30 classes; **feito em 2026-11-23** | não expõe arquivo nem publica material do cliente |
| Pacote do pedido ao titular no escopo B | `wyd-client` | [`pedido-ao-titular.md`](pedido-ao-titular.md), pronto para envio, **não enviado** | não envia, não presume resposta e não anexa conteúdo do cliente |
| Lista candidata do patch com caminhos, hashes e versão | `wyd-client` + `wyd-launcher` | lista candidata, ainda não autorizada | inclusão só depois de `Q-IP-1`/`Q-IP-2` com evidência; o inventário da árvore não é esta lista |
| Lista de usos pretendidos de marca/screenshot no portal, launcher e docs | `wyd-portal` + `wyd-launcher` | inventário de usos, sem publicar | nenhum logo/arte/screenshot até `Q-IP-5` |
| Template do registro de evidência ligado ao gate de release | `wyd-client` + `wyd-ops` | campos de §4 + bloqueio de release | revisão jurídica valida a forma; sem registro não há release |

Esses passos não são lacunas nem perguntas; não fecham `LCLI-05` e não autorizam distribuição. O inventário e o pacote do pedido existem desde 2026-11-23; os demais passos seguem pendentes.
