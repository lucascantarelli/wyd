# Skills do `wyd-client` — roteador

O `wyd-client` mede o cliente 7662 e é a fonte de dado de conteúdo do banco novo. O executável e os
arquivos do cliente vivem em `wyd-client/files/`, **ignorados pelo git**: o que se versiona é a
medição, com `sha256` e contagem.

| Tarefa | Skill | Quando não usar |
|---|---|---|
| inventariar arquivo do cliente (tamanho, hash, papel) | [`execution/medir-cliente`](execution/medir-cliente/SKILL.md) | o arquivo já está no `registro-do-cliente.md` e nada mudou |
| decodificar binário sem spec (layout de registro) | [`execution/decodificar-binario`](execution/decodificar-binario/SKILL.md) | o dado já está em arquivo texto do cliente |

## Fronteiras

- Medição é **dado**, não código: entra como `V`, com `path`, `sha256`, `size_bytes` e `row_count`.
- Nada do `deprecated/` é reutilizado — nem decodificador, nem dump, nem planilha.
- Saída de medição de conteúdo segue `pedido-de-curadoria-dos-seeds.md` §4 (`.tsv` + `source.txt` +
  `ausentes.tsv`).
