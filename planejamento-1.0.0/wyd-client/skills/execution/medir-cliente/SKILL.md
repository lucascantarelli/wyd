---
name: medir-cliente
description: Inventaria e mede um arquivo do cliente 7662 do WYD — tamanho, sha256, contagem de registros e papel — registrando a procedência. Use ao precisar de dado do cliente para seed ou protocolo, ao reconferir contagem declarada, ou ao atualizar o registro do cliente.
---

# Medir cliente

## Quando usar

- Um seed ou uma spec precisa de contagem/valor do cliente 7662.
- Uma contagem declarada no planejamento precisa ser reconferida.

## Pré-requisitos

- Cliente presente em `wyd-client/files/client/` (ignorado pelo git).
- `planejamento-1.0.0/wyd-client/registro-do-cliente.md` aberto.

## Passos

1. **Meça sem abrir o executável em produção**: tamanho e hash do arquivo alvo.

   ```bash
   stat -c '%n %s' wyd-client/files/client/ItemList.bin
   sha256sum wyd-client/files/client/ItemList.bin
   ```

2. **Registre a identidade** no `registro-do-cliente.md`: caminho relativo, papel, bytes e `sha256`.
3. **Conte registros** pela estrutura do arquivo (tamanho ÷ tamanho de registro quando o layout é
   conhecido) e diga qual divisor foi usado.
4. **Não confunda arquivo com tabela**: contagem do arquivo e contagem da entidade podem divergir
   (ex.: `Itemname.bin` tem 3.163 nomes para 3.220 itens). Divergência é dado, não erro a esconder.
5. **Procedência** de conteúdo segue o `pedido-de-curadoria-dos-seeds.md` §4.

## Como verificar

| Alegação | Prova |
|---|---|
| identidade do arquivo | `sha256sum` igual ao registrado |
| contagem | divisor inteiro e resto declarado |
| origem | `path=` + `sha256=` na entrega |

## Quando parar e perguntar

- O arquivo muda entre medições (cliente patchado): pare e registre a nova identidade antes de usar.
- A medição exige rodar o cliente (login, servidor): é teste do dono, não do agente.

## Armadilhas

- **Versionar arquivo do cliente** ou print no lugar de tabela.
- **Contagem declarada "confirmada"** sem divisão inteira que a sustente.
- **Usar `deprecated/` como atalho** — proibido, mesmo para conferir.
