# ADR-SIM-0001 — Aritmética inteira em bps, arredondamento único, `f64` só em exibição

> Pergunta: "com que números o `wyd-sim` calcula, e como o resultado é reproduzível entre plataformas?"
> Público: Arquitetura, desenvolvimento do sim e do core, QA
> Dono: equipe do `wyd-sim` (decisão do dono em 2026-11-23)
> Base: `docs/proposta-decisoes-sim.md` §4 (`Q-SIM-4`); `lacunas.md` `LSIM-04`; `wyd-db/padrao-banco.md` `P-07`; `wyd-sim-1.0.0.md` §2
> Revisado em: 2026-11-23

**Estado:** aceito (2026-11-23) — aprovado pelo dono em `Q-SIM-4` = A.

## Contexto

`LSIM-04` estava aberta: não havia política aritmética, e `f64` diverge entre plataformas, quebrando o
determinismo que é a garantia central do sistema ("mesma entrada, mesma saída"). O banco já fixa
dinheiro inteiro (`P-07`).

**Medido em 2026-11-23:** vida, AC e dano são `INTEGER`/`SMALLINT` em `game.character_scores`
(`wyd-db/migrations/0006_game_character.md`), o dano de arena é `BIGINT` (`0028`), dinheiro é inteiro na
menor unidade e percentual é bps inteiro (`0011`, `0021`, `0034`); **nenhuma** migration escrita tem
coluna `NUMERIC`, `REAL`, `DOUBLE` ou `FLOAT` — a busca só encontra essas palavras em prosa.

## Decisão

A aritmética é **inteira** em tudo que afeta estado e economia, com uma regra numérica única:

| Regra | Valor |
|---|---|
| unidade | inteiro de 64 bits na unidade do domínio, **sem escala global** |
| percentual | **bps (1/10000)**, o mesmo tipo do banco (`rate_bps`, `tax_rate_bps`, `fee_bps`) |
| arredondamento | **meio-para-longe-do-zero**, aplicado **uma única vez** no fim de cada fórmula |
| divisão | inteira pelo denominador da unidade (10000 para bps), com o mesmo arredondamento |
| overflow | verificado, nunca silencioso |
| dinheiro | unidades menores inteiras (`P-07`) |
| `f64` | proibido em estado e economia; permitido só na formatação de exibição |

A escala global de 1/1000 foi dispensada por não ter consumidor: obrigaria a converter bps → milésimos e
a arredondar duas vezes, criando divergência sem ganho. O "fixed-point" fica reservado à fórmula
específica que comprovadamente precise de sub-unidade, com a escala declarada na spec daquela fórmula
(`docs/specs/formulas/`), nunca como regra global.

## Consequências

- **Positivas:** resultado reproduzível entre plataformas; borda numérica única entre banco, fio e tela;
  chance é comparação inteira (sorteio em `[0, 10000)` contra o `rate_bps`), sem divisão.
- **Negativas/custos:** toda fórmula nova precisa declarar unidade e ponto de arredondamento; fração só
  existe onde a spec da fórmula declarar escala.
- **Contratos afetados:** `wyd-core` (estado), `wyd-protocol` (inteiro do fio) e `wyd-portal` (exibição)
  seguem o mesmo critério; `LSIM-03` fixou os vetores em `tests/fixtures/sim/<formula>.json` com
  `formula_version` e `content_version` no cabeçalho.

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| `f64` em toda fórmula | divergência entre plataformas e quebra do determinismo |
| Tudo inteiro sem regra de arredondamento | não representa fração e deixa cada fórmula livre para divergir |
| Escala global de 1/1000 | sem consumidor no domínio; obriga arredondamento duplo |

## Como verificar

A decisão está em vigor quando as fixtures de `tests/fixtures/sim/<formula>.json` trouxerem entrada,
saída esperada e o cabeçalho `formula_version`/`content_version`, e quando o gate do sistema recusar
`f64` em caminho de estado. **Hoje nenhuma fórmula, vetor ou fixture existe** — a decisão fixa a
aritmética, não o cálculo.

## Referências

- Decisão: `Q-SIM-4` = A (2026-11-23), em `docs/proposta-decisoes-sim.md` §4.
- Lacuna fechada: `LSIM-04`.
- Especificação: `wyd-sim-1.0.0.md` §2 e §6 (itens 2 e 3 do gate).
- Fonte do dinheiro inteiro: `wyd-db/padrao-banco.md` `P-07`.
