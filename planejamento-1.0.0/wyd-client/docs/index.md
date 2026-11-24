# wyd-client — visão geral

> Pergunta: "o que o `wyd-client` faz e como o cliente real se transforma em prova?"
> Público: Desenvolvedor novo, pesquisa (RE) e QA
> Dono: equipe do wyd-client
> Base: `wyd-protocol/lacunas.md` `LPRT-10`; `LPRT-03`, `LPRT-06`, `LPRT-09`
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-22)

O `wyd-client` tem duas responsabilidades: **medir o cliente real** — build, arquivos e tráfego —
para produzir prova datada e anonimizada, e **distribuir** o cliente (patch, configuração e
artefatos) quando houver launcher.

## O que ele não faz

| Fora | Dono |
|---|---|
| layout do fio e opcode | `wyd-protocol` |
| manifesto e aplicação de patch | `wyd-launcher` |
| hospedagem do artefato | `wyd-ops` |
| login e conta | `wyd-portal` / `wyd-net` |
| regra de jogo | `wyd-core` / `wyd-sim` |

## Como a captura funciona

O processo é **decidido** em `LPRT-10` e o procedimento operacional é do `wyd-protocol`
(`goldens/como-capturar.md`). O endpoint local é o `ECAP`, separado do endpoint de produção (`EPROD`);
o contrato está em `../../wyd-server/docs/specs/endpoint-de-captura.md`. O peer de A1 foi decidido em C,
mas a condição foi reprovada na medição de 2026-11-23: a captura usa o peer da opção A. Uma captura só vale como prova quando entrega, juntas: o fluxo de bytes por sentido,
a ficha de origem preenchida, a anonimização de conta/nome/IP e uma segunda leitura. Captura feita não
é captura aceita.

Duas medições do cliente continuam pendentes e são elas que destravam o fio: origem da `KeyWord`
(`LPRT-03`) e encoding/terminador de texto (`LPRT-06`). O build-alvo foi **decidido em 2026-11-23**
(`LPRT-09`, cliente 7662 + SHA-256). **Nenhuma captura existe hoje** (2026-11-22).

**A1 autorizada em 2026-11-23 (resposta A):** o dono autorizou implementar o peer da opção A, o `ECAP` e a
CLI, e executar a captura A1. A CLI e o peer ainda não existem; a captura não foi executada. A segunda
leitura será assinada por **Lucas Cantarelli** (dono do produto), que **não é o operador**; o dono do
`wyd-protocol` é suplente e o nome do operador entra na ficha. `LPRT-03` continua aberta até a segunda
leitura; `LPRT-06` continua nas capturas B/C.

## Uso e distribuição

Nenhum direito de redistribuição é inferido da cópia local. A proposta em
[`specs/propriedade-intelectual-e-distribuicao.md`](specs/propriedade-intelectual-e-distribuicao.md)
(`LCLI-05`) lista as perguntas, aprovações e bloqueios. Em 2026-11-23 o dono assumiu a responsabilidade
nominal por direitos (**Lucas Cantarelli**), congelou a distribuição e decidiu as políticas de
`Q-IP-3`–`Q-IP-6`; o pedido de proveniência/autorização (`Q-IP-1`/`Q-IP-2`) foi iniciado, com `Q-IP-2`
no escopo B (árvore inteira), e segue sem resposta do titular. **`LCLI-05` continua proposta**: nenhuma dessas decisões autoriza patch, release ou
conteúdo do cliente. Os passos técnicos com dono estão em
[`specs/propriedade-intelectual-e-distribuicao.md`](specs/propriedade-intelectual-e-distribuicao.md) §9;
o inventário item a item está em [`specs/inventario-da-arvore.md`](specs/inventario-da-arvore.md) e o
pacote do pedido em [`specs/pedido-ao-titular.md`](specs/pedido-ao-titular.md). O pedido **não foi
enviado** e nenhum dos dois artefatos libera uso. `LCLI-05` continua `proposta` e `LCLI-02` continua
aberta até a evidência do titular. Até lá, conteúdo do cliente não deve ser incluído em patch, release,
site ou outro artefato publicado. A árvore local ignorada pelo Git é uma salvaguarda; não prova
titularidade nem concede licença.

## O que já foi medido

A árvore local do cliente tem 7.147 arquivos e 567 MB, com um pacote de 192.652.447 bytes; nada
disso está versionado no git e nada disso é prova de protocolo. O inventário item a item, com 7.147
caminhos e hashes, está em [`specs/inventario-da-arvore.md`](specs/inventario-da-arvore.md); o pacote do
pedido ao titular está em [`specs/pedido-ao-titular.md`](specs/pedido-ao-titular.md) e **não foi enviado**.
O registro de identidade e totais permanece em `registro-do-cliente.md` — esta página não repete os números.

## Limite desta documentação

- Não afirma comportamento do jogo nem layout do fio: isso vem de captura, não de leitura de pasta.
- Não publica conteúdo do cliente sem autorização documentada; titularidade e escopo de uso
  permanecem perguntas de `LCLI-05`.
## Decisões em aberto

`LCLI-02` continua aberta e `Q-CLI-2` espera o dono; `LCLI-03` tem QA definido (**Lucas Cantarelli**, que não é o operador) e espera a primeira ficha assinada com o nome do operador; `LCLI-05` segue **proposta**, com responsável nominal, distribuição congelada, `Q-IP-3`–`Q-IP-6` decididas e `Q-IP-1`/`Q-IP-2` aguardando evidência do titular, com `Q-IP-2` no escopo B (árvore inteira). As perguntas estão em [Decisões do `wyd-client`](proposta-decisoes-client.md); `LCLI-01` fechou com `LPRT-09` em 2026-11-23 e não é duplicada. O estado de cada lacuna continua em `lacunas.md`.
