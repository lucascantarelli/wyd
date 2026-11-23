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
o contrato está em `../../wyd-server/docs/specs/endpoint-de-captura.md`. O peer de A1 é a decisão
aberta `Q-SRV-5`. Uma captura só vale como prova quando entrega, juntas: o fluxo de bytes por sentido,
a ficha de origem preenchida, a anonimização de conta/nome/IP e uma segunda leitura. Captura feita não
é captura aceita.

Três medições do cliente estão pendentes e são elas que destravam o fio: origem da `KeyWord`
(`LPRT-03`), encoding e terminador de texto (`LPRT-06`) e o build-alvo (`LPRT-09`). **Nenhuma
captura existe hoje** (2026-11-22).

## Uso e distribuição

Nenhum direito de redistribuição é inferido da cópia local. A proposta em
[`specs/propriedade-intelectual-e-distribuicao.md`](specs/propriedade-intelectual-e-distribuicao.md)
(`LCLI-05`) lista as perguntas, aprovações e bloqueios. Até haver evidência e decisão, conteúdo do
cliente não deve ser incluído em patch, release, site ou outro artefato publicado. A árvore local
ignorada pelo Git é uma salvaguarda; não prova titularidade nem concede licença.

## O que já foi medido

A árvore local do cliente tem 7.147 arquivos e 567 MB, com um pacote de 192.652.447 bytes; nada
disso está versionado no git e nada disso é prova de protocolo. O inventário e os hashes estão em
`registro-do-cliente.md` (planejamento) — esta página não repete os números.

## Limite desta documentação

- Não afirma comportamento do jogo nem layout do fio: isso vem de captura, não de leitura de pasta.
- Não publica conteúdo do cliente sem autorização documentada; titularidade e escopo de uso
  permanecem perguntas de `LCLI-05`.
## Decisões em aberto

As perguntas que ainda esperam o dono — `LCLI-02` e `LCLI-03` — estão em [Decisões do `wyd-client`](proposta-decisoes-client.md); `LCLI-01` é a mesma decisão de `LPRT-09` e não é duplicada. O estado de cada lacuna continua em `lacunas.md`.
