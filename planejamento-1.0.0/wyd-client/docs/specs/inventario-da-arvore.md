# Inventário da árvore do cliente — metadados e SHA-256 (escopo B)

> Pergunta: "quais arquivos existem na árvore inteira do cliente, com que tamanho e hash, sem copiar conteúdo?"
> Público: dono do produto, `wyd-client`, `wyd-launcher`, `wyd-ops` e revisão jurídica
> Dono: equipe do `wyd-client`
> Base: `Q-IP-1`/`Q-IP-2` (escopo B, 2026-11-23); `docs/specs/propriedade-intelectual-e-distribuicao.md`; `registro-do-cliente.md`
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-23) — inventário medido (`V`, 2026-11-23). Só metadados: caminho relativo, tamanho e `sha256`. Nenhum
conteúdo, screenshot, logo ou binário do cliente é copiado, embutido ou publicado. A distribuição segue
**congelada**; `LCLI-05` continua `proposta` e `LCLI-02` fechou em 2026-11-25 como decisão de conteúdo,
sem liberar distribuição até a evidência do titular.

## 1. Totais

| Medida | Valor |
|---|---:|
| Itens (arquivos regulares) | **7.147** |
| Tamanho lógico agregado | **576.124.039 bytes** |
| Equivalente decimal | 576,124 MB |
| Equivalente binário | 549,43 MiB |
| `du -sh` observado em 2026-11-23 | 567 MB |
| Classes (extensão técnica) | **30** |
| Subdiretórios | 25 |
| Pacote `client.zip` incluído no total | 192.652.447 bytes |
| Data da medição | 2026-11-23 |

O total lógico é a soma dos tamanhos dos arquivos; `du -sh` mede espaço em disco e por isso aparece como
567 MB. O registro anterior (`registro-do-cliente.md` §3) permanece correto para contagem e para a leitura
de disco; este inventário fixa o valor lógico exato.

## 2. Quebra por classe

A classe é a **extensão técnica** do nome do arquivo, não uma conclusão sobre o conteúdo. A coluna de
bytes é a soma dos tamanhos dos itens daquela classe. Nenhum item foi aberto para classificar.

| Classe | Itens | Bytes |
|---|---:|---:|
| `.ani` | 776 | 32.853.576 |
| `.bak` | 1 | 2.674.752 |
| `.bak-precaptura` | 1 | 7.040 |
| `.bak-qa` | 1 | 218 |
| `.bin` | 95 | 6.543.311 |
| `.bmp` | 7 | 12.183.114 |
| `.bon` | 81 | 19.616.094 |
| `.csv` | 1 | 2.014 |
| `.cur` | 3 | 4.802 |
| `.dat` | 117 | 7.109.101 |
| `.dll` | 9 | 4.786.936 |
| `.exe` | 3 | 15.794.742 |
| `.hat` | 1 | 268 |
| `.log` | 2 | 142.602 |
| `.map` | 4 | 212.228 |
| `.mp3` | 13 | 26.272.662 |
| `.msa` | 1.745 | 41.371.870 |
| `.msh` | 903 | 20.704.778 |
| `.mshb` | 2 | 44.488 |
| `.scc` | 4 | 400 |
| `.sys` | 3 | 90.520 |
| `.trn` | 104 | 5.112.620 |
| `.txt` | 54 | 1.126.361 |
| `.vxd` | 2 | 34.853 |
| `.wav` | 350 | 38.087.546 |
| `.wys` | 2.601 | 71.887.113 |
| `.wyt` | 261 | 74.950.739 |
| `.xt` | 1 | 1.856.536 |
| `.xtp` | 1 | 308 |
| `.zip` | 1 | 192.652.447 |
| **Total** | **7.147** | **576.124.039** |

A soma das 30 classes confere com os totais de §1; o checklist de verificação está em §5.

## 3. Artefato item a item

O inventário completo, com uma linha por arquivo e as colunas `caminho_relativo`, `tamanho_bytes`,
`sha256` e `classe`, está em:

```text
planejamento-1.0.0/wyd-client/inventario-arvore-cliente.tsv
```

O arquivo começa com cabeçalho de escopo, data, total, tamanho agregado e a quebra por classe; depois
vêm as 7.147 linhas. Ele é **dado**, não página publicada: o site não copia o TSV e nenhum conteúdo do
cliente entra nele.

Caminhos são relativos a `wyd-client/files/` e usam `/`. O hash é SHA-256 calculado por fluxo; a
ferramenta não interpreta o arquivo. A árvore inclui o pacote `client.zip` e a pasta `client/` já
extraída; ambos são itens do escopo, não uma duplicação a ser distribuída.

## 4. O que este inventário prova — e o que não prova

| Prova | Não prova |
|---|---|
| que a árvore tinha 7.147 arquivos regulares em 2026-11-23 | que algum item pode ser publicado, modificado ou redistribuído |
| o tamanho e o SHA-256 de cada item no momento da leitura | que o item é o build-alvo ou que seu conteúdo é o esperado |
| a quebra técnica por extensão | origem, titularidade, licença ou autorização de uso |
| que a árvore foi enumerada sem copiar conteúdo | comportamento do jogo, protocolo ou regra de negócio |
| que o pacote `client.zip` e sua extração estão no escopo | que `client.zip` e a extração são idênticos |

A presença local não é evidência de direito. A classificação de uso e distribuição continua em
`docs/specs/propriedade-intelectual-e-distribuicao.md` (`LCLI-05`, `proposta`). A lista do patch é outro
artefato: ela foi montada item a item e medida em `registro-do-cliente.md` §5.1 (2026-11-25) e não é
inferida deste inventário.

## 5. Como reproduzir e conferir

O inventário foi gerado por enumeração recursiva de `wyd-client/files/`, com `lstat` para tamanho e
verificação de alteração durante a leitura, e SHA-256 por fluxo. Não há script executável versionado
em `planejamento-1.0.0/`; o artefato é o TSV. A conferência mínima:

1. contar as linhas de itens e comparar com `# Total de itens: 7147`;
2. somar `tamanho_bytes` e comparar com `576124039`;
3. validar que toda linha tem caminho, tamanho inteiro e `sha256` com 64 caracteres hexadecimais;
4. comparar a soma da quebra por classe com o total;
5. confirmar que `git check-ignore` continua ignorando `wyd-client/files/`.

Em 2026-11-23 os cinco pontos fecharam: 7.147 linhas, 576.124.039 bytes, 30 classes e nenhuma linha
inválida. O comando `du -sh` devolveu 567 MB; a diferença para o total lógico é a conhecida diferença
entre soma de arquivos e espaço em disco.

## 6. Pedido ao titular e limite de uso

O [pacote do pedido](pedido-ao-titular.md) no escopo B está em `docs/specs/pedido-ao-titular.md`. Este inventário é anexo
técnico do pedido, não autorização: o destinatário, o objeto, o que se pede e o que não se pede estão
naquele pacote. A resposta será registrada por item, com origem, autorização, escopo, prazo, aprovador e
destino, e passará pelo gate de release (`Q-IP-6`).

Até a resposta e o registro, **nada** deste inventário autoriza patch, release, captura, publicação ou
evidência de direitos. `LCLI-05` continua `proposta`; `LCLI-02` fechou em 2026-11-25 como decisão de
conteúdo (`registro-do-cliente.md` §5.1), sem liberar distribuição. Nenhuma linha deste inventário
altera as contagens de lacunas.
