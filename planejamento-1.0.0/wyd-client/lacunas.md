# `wyd-client` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-22).** Único lugar onde uma lacuna do `wyd-client` tem dono,
> impacto e estado. Nada aqui está decidido por esta abertura.

## 0. Como ler

Estados: **aberta**, **proposta**, **fechada** (com evidência). `arquitetura` propõe; o dono do
sistema aprova. Lacuna de outro sistema que este consome é citada pelo ID de origem.

## 1. Situação em uma linha

| Grupo | Lacunas | Abertas | Propostas | Fechadas |
|---|---:|---:|---:|---:|
| Medição do cliente | 2 | 2 | 0 | 0 |
| Distribuição | 1 | 1 | 0 | 0 |
| Processo | 1 | 1 | 0 | 0 |
| **Total** | **4** | **4** | **0** | **0** |

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `LCLI-01` | Qual build do cliente é o alvo, e qual é a identidade verificável dele (versão declarada e hash do binário)? | é `LPRT-09` vista do lado do cliente: sem build-alvo, offset e opcode ficam ambíguos | frente `wyd-client` + pesquisa | aberta |
| `LCLI-02` | O que exatamente o patch entrega (`strdef.bin`, assets, executável) e com que versionamento? | é a entrada do manifesto do `wyd-launcher`; sem lista, o patch vira cópia da árvore inteira | `wyd-client` + `wyd-launcher` | aberta |
| `LCLI-03` | Onde mora o registro de captura — ficha, data e quem aprova a segunda leitura? | o processo de `LPRT-10` está decidido, mas o lugar do registro e o aprovador não estão nomeados | QA + arquitetura | aberta |
| `LCLI-04` | Qual ferramenta faz a captura, com que dono e em qual pasta do repositório? | captura sem ferramenta aprovada vira script solto; `AGENTS.md` §5.6 proíbe executável em `planejamento-1.0.0/` | `wyd-ops` + pesquisa | aberta |

## 3. Medições herdadas (não são lacunas deste registro)

| ID de origem | O que falta medir | Estado lá |
|---|---|---|
| `LPRT-03` | origem da `KeyWord` no handshake | aberta (`wyd-protocol/lacunas.md`) |
| `LPRT-06` | encoding e terminador de string | aberta (idem) |
| `LPRT-09` | build-alvo e compatibilidade | aberta (idem) |

Estas três fecham no `wyd-protocol`; o `wyd-client` fornece a captura. Duplicar o registro aqui
criaria dois donos para a mesma lacuna.
