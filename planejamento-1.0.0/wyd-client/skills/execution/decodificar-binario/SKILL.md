---
name: decodificar-binario
description: Deduz e documenta o layout de um arquivo binário do cliente WYD (registro, campo, offset) sem reutilizar código do legado, provando o layout pela estrutura do arquivo. Use quando o dado não existe em arquivo texto e é preciso medi-lo, como em ItemList.bin, SkillData.bin ou InitItem.bin.
---

# Decodificar binário

## Quando usar

- O dado necessário só existe em `.bin` do cliente.
- Uma spec de protocolo ou um seed depende de campo que ninguém documentou.

## Pré-requisitos

- Identidade do arquivo medida (`execution/medir-cliente`).
- Página de planejamento do sistema que vai consumir o dado (o layout não nasce no `wyd-db`).

## Passos

1. **Hipótese de registro**: divida o tamanho por candidatos plausíveis (contagem declarada, múltiplos
   de 2/4/8). Divisão inteira é indício, não prova.
2. **Procure periodicidade**: bloco repetido com o mesmo padrão de bytes no mesmo offset confirma
   tamanho de registro.
3. **Identifique campos por evidência**: valor que só cresce (índice), faixa pequena (0–100), texto
   com preenchimento (`0x5A`, `0x00`), bit alto usado como flag.
4. **Teste o layout na prática**: com o mapa de campos, os valores têm de fazer sentido para o domínio
   (preço monotônico, nível dentro da faixa, nome decodificável).
5. **Documente o layout** na página do sistema dono: arquivo, tamanho do registro, tabela
   offset × tipo × campo × evidência (`V` com comando/data).
6. **Só então meça** a entidade e entregue no formato de curadoria (§4).

## Como verificar

- O layout prevê exatamente o tamanho do arquivo: `registros × tamanho + cabeçalho = bytes`.
- Amostra decodificada é conferível contra o cliente (nome existe, item existe).
- Nenhum campo sem evidência: o que não fecha fica marcado `H`.

## Quando parar e perguntar

- O layout não fecha por nenhuma hipótese: falta dado (versão do cliente, patch) — pergunte antes de
  estimar.
- A decodificação exige construir ferramenta nova: ela nasce com issue e página própria, não dentro
  de uma issue de seed.

## Armadilhas

- **Reaproveitar decodificador do `deprecated/`** — proibido: o legado é nível 4.
- **Preencher campo sem evidência** para o layout "fechar".
- **Confundir arquivo com tabela** e declarar contagem que não existe.
- **Guardar binário ou print** no repositório como se fosse medição.
