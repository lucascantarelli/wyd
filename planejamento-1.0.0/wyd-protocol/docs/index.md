> Pergunta: "por onde começo a ler a documentação do wyd-protocol?"
> Público: Desenvolvedor novo
> Dono: equipe do wyd-protocol
> Base: —
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-19)

O `wyd-protocol` é o crate que transforma bytes do CPSock em tipos e tipos em bytes: cabeçalho de
12 bytes, enquadramento por `Size`, ofuscação por `pKeyWord`, checksum e catálogo de `Type`. Ele não
abre socket, não guarda sessão e não decide regra de jogo.

> [!ABSTRACT] Regra que vale para toda esta documentação
> O layout do fio é imposto pelo cliente 7662, que é binário externo e proprietário. Byte sem
> prova (golden, captura ou leitura do cliente) é hipótese marcada `H` com o ID da lacuna que a
> fecha — nunca afirmação.
## O que existe aqui

Esta árvore está em **planejamento**: cinco páginas escritas, o restante é teto declarado no
`wyd-protocol-1.0.0.md` (planejamento) e nas lacunas do sistema (planejamento).

| Pasta | Responde | Estado |
|---|---|---|
| `docs/arquitetura/` | por que o codec é assim, onde termina o protocolo e começa a rede | planejada |
| `docs/specs/` | requisitos, regras, fluxos e estados com ID | planejada |
| `docs/wire/` | cabeçalho, enquadramento, transformação, catálogo e limites | planejada |
| `docs/payloads/` | campos de cada mensagem, com offset, tipo e proveniência | planejada |
| `docs/goldens/` | que bytes provam o layout e de onde vieram | parcial — 1 escrita (procedimento) |
| `docs/planos/` | plano, métrica e risco do sistema | planejada |
| `docs/roadmap/` | issues `WYDPRT-nnn` com DoD verificável | parcial — 2 escritas (índice e fases) |
| `docs/guia-desenvolvedor/` | como mexer no crate sem quebrar o fio | planejada |
| `docs/guia-teste/` | como provar o layout | planejada |

## Trilhas de leitura

#### Rede e servidor


1. `docs/arquitetura/camadas-de-codec.md` — o pipeline de bytes.
2. `docs/arquitetura/fronteira-protocolo-rede.md` — o que o codec devolve e o que a conexão decide.
3. `docs/guia-desenvolvedor/api-e-garantias.md` — que função chamar.
#### Regra de jogo


1. `docs/payloads/index.md` — qual família carrega o dado.
2. `docs/payloads/<familia>.md` — campos, faixa e o que é opaco.
3. `docs/specs/regras-de-negocio.md` — o que o crate garante sobre o valor.
#### Pesquisa do cliente


1. `docs/goldens/como-capturar.md` — como produzir prova.
2. `docs/goldens/catalogo.md` — o que já está provado.
3. `lacunas.md` do sistema (planejamento) — o que falta capturar.
#### QA


1. `docs/guia-teste/piramide-e-alvos.md` — que tipo de teste escrever.
2. `docs/guia-teste/goldens-e-fixtures.md` — que byte usar.
3. `docs/guia-teste/gates-e-anti-skip.md` — como o gate detecta teste que não roda.
## Antes de escrever qualquer página

Leia **[como ler uma página de fio](convencoes.md)**: marcações de evidência, proveniência de byte,
regra de direção e o que reprova uma página.

## Limite desta documentação

- Não descreve regra de jogo: dono é `wyd-core` e `wyd-sim`.
- Não descreve socket, timeout ou rate-limit: dono é `wyd-net`.
- Não descreve captura em produção: dono é `wyd-ops`.
- Não promete opcode, offset ou campo sem prova: o que falta está em `lacunas.md` do sistema
  (planejamento).
## Decisões

O build-alvo foi **decidido em 2026-11-23** (`Q-PRT-1`, opção A): cliente **7662**, identificado por versão
e SHA-256 completos; qualquer outro build é não suportado. O registro está em
[Decisões do `wyd-protocol`](proposta-decisoes-protocol.md). `LPRT-03` e `LPRT-06` são medição/execução e
seguem cobertas pelo procedimento de captura; o estado de cada lacuna continua em `lacunas.md`. **A1 foi
autorizada em 2026-11-23 (resposta A):** peer da opção A, `ECAP` e CLI autorizados; captura não executada;
`LPRT-03` só fecha após a segunda leitura e `LPRT-06` continua nas capturas B/C.
