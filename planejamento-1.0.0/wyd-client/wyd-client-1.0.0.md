# wyd-client 1.0.0 — abertura do planejamento

> Pergunta: "o que o `wyd-client` entrega, o que ele mede do cliente real e como essa medição vira prova sem inventar dado?"
> Público: Produto, arquitetura, pesquisa (RE) e equipe do `wyd-client`
> Dono: equipe do `wyd-client`
> Base: `wyd-protocol/lacunas.md` `LPRT-10`; `wyd-protocol/docs/goldens/como-capturar.md`; `LPRT-03`, `LPRT-06`, `LPRT-09`; `AGENTS.md` §3/§6
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-22). Abertura inicial: fronteira, contratos e a frente de captura
nomeada. **Nenhum código** e nenhum requisito numerado (`RF-CLI`). A política de uso e distribuição do
cliente está aberta como proposta em `docs/specs/propriedade-intelectual-e-distribuicao.md`
(`LCLI-05`); nenhum direito é presumido. O registro do que já foi medido no cliente local está em
`registro-do-cliente.md` e **não** afirma comportamento do jogo.

## 1. O que ele faz

Duas coisas, nesta ordem de maturidade:

1. **Frente de captura e registro** — mede o cliente real (build, arquivos, tráfego) e entrega
   prova datada e anonimizada, no processo de `LPRT-10`, para que `wyd-protocol` e `wyd-db` não
   afirmem byte por estimativa.
2. **Distribuição de cliente** — patch, arquivos de configuração e artefatos que o launcher aplica
   (`strdef.bin`, assets, patch), hoje inexistente.

O que ele **não** é: fonte de verdade. O `deprecated/` e a árvore local do cliente são referência
(nível 4 de `AGENTS.md` §3); a fonte é a captura datada.

## 2. O que ele **não** faz

| Fora | Dono |
|---|---|
| layout do fio, opcode e offset | `wyd-protocol` |
| formato do manifesto e aplicação do patch | `wyd-launcher` |
| hospedagem do artefato | `wyd-ops` |
| login e conta | `wyd-portal` / `wyd-net` |
| regra de jogo | `wyd-core` / `wyd-sim` |

## 3. Contratos

| # | Contrato | Direção | Forma | Garantia | Estado |
|---|---|---|---|---|---|
| K1 | procedimento de captura | cliente ← `wyd-protocol` | `goldens/como-capturar.md` (dono) | datada, anonimizada, com segunda leitura | **decidido** (`LPRT-10`) |
| K2 | ficha de origem de cada captura | cliente entrega | registro com data, build, cenário e o que mediu | sem ficha, a captura é registro, não prova | **decidido** (`LPRT-10`) |
| K3 | identidade do build-alvo | cliente mede | hash e versão do binário | `LPRT-09` decidido em 2026-11-23: 7662 + SHA-256 | **fechado** (`LCLI-01` com `LPRT-09`) |
| K4 | conteúdo do patch e `strdef.bin` | cliente → `wyd-launcher` | artefato versionado | patch descreve exatamente os arquivos | **lacuna** `LCLI-02` |
| K5 | anonimização de dado de jogador | cliente garante | conta, nome, IP e chat removidos antes de gravar | nenhum dado pessoal em artefato versionado | **decidido** (`LPRT-10`), operação pendente |

## 4. Estado medido (`V`, 2026-11-22; páginas recontadas em 2026-11-23)

| Medida | Valor | Fonte |
|---|---:|---|
| Arquivos do cliente na máquina | 7.147 | `find wyd-client -type f` |
| Tamanho da árvore local | 567 MB | `du -sh wyd-client/files` |
| Pacote compactado | 192.652.447 bytes | `ls -l wyd-client/files/client.zip` |
| Arquivos versionados no git | 0 | `git ls-files wyd-client` (só `.gitkeep`) |
| Capturas registradas | 0 | nenhuma ficha de origem existe |
| Páginas de planejamento | 5 | `find planejamento-1.0.0/wyd-client -name '*.md'` (2026-11-23, com a proposta `LCLI-05`) |

O inventário por extensão e os hashes medidos estão em `registro-do-cliente.md`; esta página não os
repete (um fato, um dono).

## 5. O que falta

`lacunas.md`: `LCLI-02` (conteúdo do patch e `strdef.bin`), `LCLI-03` (quem aprova a segunda leitura; o
registro já está fixado em `tests/golden/` por `LPRT-10`), `LCLI-05` (proposta de autorização e escopo de
uso/distribuição), mais as duas medições herdadas do protocolo (`LPRT-03`, `LPRT-06`) e a leitura que
confirma a build do alvo fixado (`LPRT-09`). `LCLI-01` fechou com `LPRT-09` em 2026-11-23.
`LCLI-04` foi fechada em 2026-11-23: ferramenta específica em
`wyd-client/tools/`; D3–D6 foram decididos em 2026-11-23 e a implementação está pendente.

## 6. Gate para sair do papel

1. Registro medido do cliente (feito em 2026-11-22) e primeira captura datada com ficha.
2. Ferramenta de captura planejada em `wyd-client/tools/`, com escopo e salvaguardas aprovados;
   D3–D6 estão decididos, e a implementação do `ECAP` e da CLI Rust é o próximo trabalho,
   sem código em `planejamento-1.0.0/` (`AGENTS.md` §5.6).
3. Nenhum conteúdo do cliente é versionado, publicado ou redistribuído sem autorização e escopo
   documentados (`LCLI-05`); ignorar a árvore local é uma salvaguarda, não uma conclusão sobre
   titularidade.
4. `RF-CLI` numerados com critério verificável.
5. Anonimização conferida antes de qualquer captura ser aceita.

## 7. Próximas páginas

`docs/arquitetura/fronteira.md`, `docs/specs/captura-e-registro.md`,
`docs/planos/plano-de-testes.md` (E2E do cliente é Playwright, `AGENTS.md` §8) e `docs/roadmap/`.
`docs/specs/propriedade-intelectual-e-distribuicao.md` já existe como proposta (`LCLI-05`).
