# `wyd-launcher` — lacunas e decisões pendentes (registro único)

> **Estado: registro vivo (2026-11-22).** Único lugar onde uma lacuna do `wyd-launcher` tem dono,
> impacto e estado. Nada aqui está decidido.

## 0. Como ler

Estados: **aberta**, **proposta**, **fechada** (com evidência). `arquitetura` propõe; o dono do
sistema aprova.

## 1. Situação em uma linha

| Grupo | Lacunas | Abertas | Propostas | Fechadas |
|---|---:|---:|---:|---:|
| Patch e manifesto | 2 | 2 | 0 | 0 |
| Servidor e conteúdo | 1 | 1 | 0 | 0 |
| Distribuição | 1 | 1 | 0 | 0 |
| **Total** | **4** | **4** | **0** | **0** |

## 2. Lacunas

| ID | Pergunta | Por que importa | Dono | Estado |
|---|---|---|---|---|
| `LLCH-01` | Qual é o formato do manifesto: campos, hash por arquivo, versão e compatibilidade? | é o contrato do patch; sem ele, launcher e `wyd-client` divergem silenciosamente | arquitetura + `wyd-client` | aberta |
| `LLCH-02` | De onde vem a lista de canais e o estado de cada um: portal, arquivo publicado ou o próprio servidor? | lista errada manda o jogador para canal morto; o portal já publica a página de canais no planejamento | arquitetura + `wyd-server` | aberta |
| `LLCH-03` | O manifesto é assinado, e o launcher verifica a assinatura? | patch adulterado é execução de código arbitrário na máquina do jogador | arquitetura + segurança | aberta |
| `LLCH-04` | Como o aplicativo é empacotado e distribuído (instalador, atualização do próprio launcher)? | `wyd-ops` publica release; sem formato, a release não tem artefato | `wyd-ops` | aberta |

## 3. O que não é lacuna deste sistema

- Conteúdo do patch: dono é `wyd-client`.
- Hospedagem e imagem: dono é `wyd-ops`.
