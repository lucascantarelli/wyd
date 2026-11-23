> Pergunta: "quais arquivos, marcas e materiais do cliente podem ser modificados, publicados e distribuídos pelo WYD 1.0.0?"
> Público: dono do produto, `wyd-client`, `wyd-launcher`, `wyd-ops` e revisão jurídica
> Dono: equipe do `wyd-client` (inventário); aprovação de direitos pelo dono do produto, ainda sem responsável nominal
> Base: `wyd-client/lacunas.md` (`LCLI-02`); `wyd-launcher/lacunas.md` (`LLCH-01`, `LLCH-03`, `LLCH-04`); `wyd-client/registro-do-cliente.md`; `AGENTS.md` §§1, 3, 5.7–5.9
> Revisado em: 2026-11-23

**Estado:** proposta (`P`, 2026-11-23). Esta página abre perguntas; não conclui titularidade, licença, permissão ou legalidade de uso, modificação, publicação ou redistribuição. Não é parecer jurídico nem autorização para distribuir arquivo.

## 1. Objetivo e limite

Esta proposta busca impedir que o projeto confunda **ter acesso local a um arquivo** com **ter direito de publicá-lo ou distribuí-lo**. O inventário medido em `registro-do-cliente.md` descreve arquivos presentes na máquina; não prova origem, titularidade, licença nem permissão de redistribuição (`V` quanto ao inventário; direitos permanecem `P`, sem evidência registrada).

A política futura precisa classificar separadamente: (a) código e documentação escritos para a reimplementação; (b) binários e recursos do cliente existente; (c) configurações e arquivos modificados para o patch; (d) capturas, imagens, nomes e marcas usados em documentação, site ou launcher. Uma decisão sobre uma classe não se estende automaticamente às demais.

## 2. Perguntas em aberto

| ID | Pergunta | Dono que precisa responder | O que bloqueia |
|---|---|---|---|
| Q-IP-1 | Qual é a proveniência verificável de cada classe de arquivo e quem detém os direitos aplicáveis? | `wyd-client` inventaria; dono do produto obtém evidência do titular ou de representante autorizado | Classificar itens sem origem comprovada como distribuíveis; aprovar `LCLI-02` |
| Q-IP-2 | Há autorização expressa para reproduzir, modificar ou redistribuir executável, assets, sons, fontes, textos, mapas, imagens ou arquivos como `strdef.bin`? Quais itens, operações, canais, territórios e prazo ela cobre? | dono do produto + titular dos direitos; revisão jurídica qualificada | Incluir ou modificar esses itens em patch, instalador, site, CDN, release ou pacote público |
| Q-IP-3 | O que pode entrar no repositório público: conteúdo, trecho, screenshot, hash, metadado técnico ou somente referência externa? | dono do produto + revisão jurídica; `wyd-client` aplica a classificação | Versionar ou publicar conteúdo do cliente; qualquer conteúdo fica fora até a decisão documentada |
| Q-IP-4 | Como se separam a licença do código novo do projeto e a situação jurídica dos arquivos preexistentes ou derivados? | dono do produto + revisão jurídica; `wyd-ops` mantém os metadados de release | Escolher uma licença que pareça abranger material de terceiros ou publicar release misto sem classificação |
| Q-IP-5 | Quais nomes, logos, marcas, artes e capturas de tela podem aparecer no portal, documentação e launcher, e com que atribuição ou aviso? | dono do produto + titular dos direitos; `wyd-portal`/`wyd-launcher` listam usos pretendidos | Divulgar página promocional, screenshot, identidade visual ou nome como se houvesse autorização |
| Q-IP-6 | Que evidência documenta cada autorização, suas condições, revogação e revisão antes de uma release? | dono do produto define aprovador; `wyd-ops` liga evidência à publicação; revisão jurídica valida a forma | Aprovar release, espelho, atualização ou pacote sem prova rastreável de escopo autorizado |

## 3. Donos e fronteiras

| Responsabilidade | Dono proposto | Limite |
|---|---|---|
| Inventariar os itens e suas proveniências, sem publicar o conteúdo | `wyd-client` | Não pode converter presença local em prova de direito |
| Aprovar o escopo de uso/distribuição e obter autorização documentada | dono do produto / responsável por direitos (nome ainda não identificado) | Decisão de negócio e permissão; não substitui análise jurídica |
| Avaliar licenças, titularidade e condições de uso | titular ou representante autorizado, com revisão jurídica qualificada | Não presumir autorização nem emitir conclusão pela equipe técnica |
| Definir conteúdo do patch | `wyd-client`, com a lacuna `LCLI-02` | Só após classes e itens distribuíveis estarem liberados |
| Definir manifesto, integridade e assinatura | `wyd-launcher` (`LLCH-01`, `LLCH-03`) | Restrição técnica não concede direito de redistribuição |
| Empacotar, hospedar e publicar release | `wyd-ops` / `wyd-launcher` (`LLCH-04`) | Só consome itens liberados e evidência aprovada |

O responsável nominal por direitos e quem assina a revisão jurídica são perguntas em aberto; não há nome registrado nesta proposta. Não se atribui ao `wyd-ops` o poder de conceder direitos, nem ao launcher o de decidir conteúdo do cliente.

## 4. Registro mínimo para uma futura decisão

Antes de liberar qualquer item, a decisão deverá registrar: identificador/classe do item (não necessariamente o conteúdo); origem e evidência; titular ou parte autorizadora; usos autorizados (copiar, modificar, distribuir); canais e território; prazo e condições; restrições de atribuição; aprovador e data; e destino operacional (repositório, pacote privado/público, launcher, site ou nenhum). Se a permissão não puder ser demonstrada, o estado permanece **não autorizado para publicação/distribuição** até decisão — não se infere permissão por silêncio.

A lista de itens e o detalhe da evidência deverão ter acesso e armazenamento apropriados; esta página não manda versionar contrato, arquivo binário ou dado confidencial no repositório público.

## 5. Bloqueios e não-bloqueios

**Bloqueia até decisão/evidência:** conteúdo de `LCLI-02`; composição final de artefatos do manifesto (`LLCH-01`); empacotamento e distribuição de launcher/cliente (`LLCH-04`); publicação de recursos, screenshots, marcas ou imagens do cliente no site; e qualquer release que inclua material cuja autorização não esteja comprovada. A assinatura técnica de `LLCH-03` não resolve essa autorização.

**Não bloqueia por si só:** inventariar localmente arquivos sem expor conteúdo, registrar hashes/metadados já permitidos pelas regras do repositório, planejar ou implementar código original sem incorporar material não liberado, nem capturar tráfego sintético conforme `LPRT-10` e `LCLI-04`. Capturas que reproduzam conteúdo protegido continuam sujeitas à classificação desta proposta antes de publicação.

## 6. Como esta proposta pode fechar

O dono do produto nomeia o responsável por direitos e define o processo de aprovação; as partes reúnem evidência para cada classe e respondem Q-IP-1 a Q-IP-6. A revisão jurídica valida as permissões e seus limites. `wyd-client`, `wyd-launcher` e `wyd-ops` refletem a decisão nos respectivos artefatos e lacunas, sem duplicar o registro de direitos. Até lá, esta página permanece `proposta` e não autoriza redistribuição.
