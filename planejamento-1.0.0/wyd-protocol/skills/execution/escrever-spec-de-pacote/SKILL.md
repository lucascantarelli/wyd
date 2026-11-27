---
name: escrever-spec-de-pacote
description: Escreve a spec de um pacote do protocolo WYD com opcode, layout de campos, endereçamento e evidência da captura do cliente. Use ao implementar handshake, opcode novo, ou ao corrigir spec que não bate com o tráfego real.
---

# Escrever spec de pacote

## Quando usar

- Opcode novo entra no protocolo.
- Spec existente divergiu do tráfego medido.

## Pré-requisitos

- Captura de tráfego do cliente 7662 (arquivo e `sha256` registrados).
- `wyd-protocol-1.0.0.md` §4 (teto) e `lacunas.md` (ID `LPRT-*`).

## Passos

1. **Identifique o opcode** no cabeçalho da captura e o sentido (cliente → servidor ou o inverso).
2. **Descreva o layout** campo a campo: offset, tipo, tamanho, endianness, domínio de valor.
3. **Tamanho**: declare se o pacote é fixo ou tem contador/terminador; diga como o tamanho é calculado.
4. **Amostra real**: cole um pacote decodificado com origem (captura, offset, data).
5. **Nenhum campo sem evidência**: o que não fecha fica `H` com o ID de lacuna que o fecha.
6. **Registre a página** no mapa de `docs/index.md` do sistema.

## Como verificar

- O layout reproduz o tamanho do pacote capturado, byte a byte.
- Decodificar a amostra devolve valores plausíveis para o domínio.
- Página citada no índice e sem link quebrado (`make gate-fast`).

## Quando parar e perguntar

- Falta captura para o opcode.
- O campo depende de decisão de arquitetura (ex.: versionamento de protocolo).

## Armadilhas

- **Inventar campo** para o layout fechar.
- **Copiar spec do legado** — nível 4, proibido.
- **Esquecer endianness** e "funcionar" só no exemplo escolhido.
