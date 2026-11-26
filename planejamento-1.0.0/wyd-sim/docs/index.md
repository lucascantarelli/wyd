# wyd-sim — visão geral

> Pergunta: "o que o `wyd-sim` calcula e por que ele é separado do `wyd-core`?"
> Público: Desenvolvedor novo, QA e produto
> Dono: equipe do wyd-sim
> Base: `wyd-ops/proposta-versoes-e-cobertura-funcional.md` §7.2; `wyd-protocol-1.0.0.md` §1.2
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-22)

O `wyd-sim` concentra o **cálculo** do jogo em funções puras e determinísticas: dano, chance, curva
de nível, experiência, drop e cooldown. Foi separado do `wyd-core` para que a regra (quando aplicar)
e a fórmula (quanto resulta) tenham donos distintos, e para que a fórmula seja provável por vetor de
entrada e saída, sem subir servidor.

## O que ele não faz

| Fora | Dono |
|---|---|
| decidir quando a fórmula é aplicada | `wyd-core` |
| carregar e versionar o dado de conteúdo | `wyd-db` (`content`) |
| equilibrar o jogo | produto |
| formatar ou arredondar para a tela | `wyd-portal` / `wyd-protocol` |

## Garantia central

Mesma entrada, mesma saída. Nenhuma fórmula lê banco, relógio, ambiente ou estado global — se
precisa de um valor do mundo, ele entra como parâmetro.

## O que falta

Nenhuma pergunta está aberta em `lacunas.md`: as quatro lacunas fecharam pelo dono em 2026-11-23.
A versão do cálculo (`LSIM-02`) foi decidida por `Q-SIM-2` = A — cada fórmula carrega `formula_version`
próprio, registrado no cabeçalho do fixture junto do `content_version` usado. A localização/formato dos
vetores (`LSIM-03`) foi decidida por `Q-SIM-3` = A: fixtures JSON versionadas em
`tests/fixtures/sim/<formula>.json`, com entrada, saída esperada e identificador da versão de fórmula; os
arquivos ainda não foram implementados. A lista de fórmulas (`LSIM-01`, fatia vertical com vetor por
fórmula, incluindo o deslocamento puro) e a política aritmética (`LSIM-04`: aritmética inteira, percentual
em bps, sem escala global, `f64` só em exibição) fecharam na mesma data. Resta implementar fórmulas,
vetores e fixtures. Nenhuma fórmula está escrita: esta página não afirma coeficiente, curva nem faixa de dano.

## Limite desta documentação

- Não afirma número de balanceamento: valor sem fonte é hipótese, não regra.
- Não mede desempenho de cálculo: nada foi executado.

## Decisões em aberto

Nenhuma: `LSIM-01`–`LSIM-04` foram decididas pelo dono em 2026-11-23, com o registro em
[Decisões do `wyd-sim`](proposta-decisoes-sim.md), e não são reabertas. A decisão de `LSIM-03` não
significa que fixtures já existam, e `LSIM-02` não escreve fórmula. O estado de cada lacuna continua em
`lacunas.md`.

## Mapa de páginas

Este mapa garante que toda página de `docs/` seja alcançável a partir deste índice; a pasta dona continua sendo a fonte do conteúdo.

### arquitetura

- [componentes](arquitetura/componentes.md)
- [adr/index](arquitetura/adr/index.md)

### planos

- [plano-de-testes](planos/plano-de-testes.md)

### roadmap

- [index](roadmap/index.md)
- [roadmap](roadmap/roadmap.md)

### specs

- [requisitos](specs/requisitos.md)
