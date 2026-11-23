# `wyd-client` — registro medido do cliente local

> Pergunta: "o que já foi medido do cliente real, com que comando e o que essa medição **não** prova?"
> Público: pesquisa (RE), arquitetura e QA
> Dono: equipe do `wyd-client`
> Base: `LPRT-10` (processo de captura), `goldens/como-capturar.md` (procedimento)
> Revisado em: 2026-11-22

**Estado:** registro medido (`V`), 2026-11-22. Substitui qualquer estimativa anterior de tamanho,
contagem ou build. **Não** é captura de protocolo: não há um byte de tráfego aqui.

## 1. Regras deste registro

1. Só entra número que sai de comando, com data. Número sem comando é `H` e vira lacuna.
2. **Nenhum dado de jogador**: conta, personagem, nome, IP, chat ou configuração pessoal não são
   lidos nem gravados.
3. **Nenhum conteúdo do cliente no git**: a árvore é ignorada e este registro guarda metadados
   (contagem, tamanho, hash), não arquivo. Ignorar é salvaguarda, não conclusão sobre titularidade:
   o que pode ser publicado ou redistribuído está em proposta na `LCLI-05`
   (`docs/specs/propriedade-intelectual-e-distribuicao.md`).
4. Não se afirma comportamento do jogo a partir de nome de arquivo. Extensão e pasta são indício,
   não prova (`AGENTS.md` §3).

## 2. Ambiente medido

| Item | Valor |
|---|---|
| Sistema | Windows, estação de desenvolvimento do dono |
| Raiz local | `wyd-client/files/` (ignorada pelo git) |
| Versionado no git | 0 arquivo — `git ls-files wyd-client` devolve só `wyd-client/.gitkeep` |
| Data da medição | 2026-11-22 |

## 3. Totais

| Medida | Valor | Comando |
|---|---:|---|
| Arquivos | 7.147 | `find wyd-client/files -type f \| wc -l` |
| Tamanho da árvore | 567 MB | `du -sh wyd-client/files` |
| Pacote compactado | 192.652.447 bytes | `ls -l wyd-client/files/client.zip` |

## 4. Identidade dos binários

| Arquivo | Bytes | Data de modificação | SHA-256 |
|---|---:|---|---|
| `client/WYD.exe` | 2.347.008 | 2026-06-22 09:18 | `04c9d2a1235cba57f7b588347fae935a2466824ddf9d7d92e541e0cc8a27ddf8` |
| `client/WYDLauncher.exe` | 13.271.552 | 2026-11-11 22:26 | `4c1d3ff59d5b080eb4759daa0141b3e8ce13fde5798876a4c7d00a86d4a2cd3e` |
| `client/Mesh/halfframe.exe` | 176.182 | 2026-06-22 09:18 | não medido (auxiliar de malha) |
| `client.zip` | 192.652.447 | 2026-11-11 20:55 | `cd06047525b1f9b3cbcafb4d15f970a4ab2f5e0136c64f93f4373749c3975471` |

Comando: `sha256sum wyd-client/files/client.zip wyd-client/files/client/WYD.exe`.
O hash de `WYD.exe` é a identidade do build-alvo desde 2026-11-23 (`Q-PRT-1`/`LPRT-09`, opção A: cliente
7662 com versão e SHA-256 completos). Era candidato até a decisão; a medição não escolhe o alvo.

## 5. Distribuição por extensão (top 20)

| Extensão | Arquivos | Extensão | Arquivos |
|---|---:|---|---:|
| `wys` | 2.601 | `dat` | 117 |
| `msa` | 1.745 | `trn` | 104 |
| `msh` | 903 | `bin` | 95 |
| `ani` | 776 | `bon` | 81 |
| `wav` | 350 | `txt` | 54 |
| `wyt` | 261 | `mp3` | 13 |
| | | `dll` | 9 |
| | | `bmp` | 7 |
| | | `exe` | 3 |

Comando:

```bash
find wyd-client/files -type f | awk -F. '{n=NF>1?tolower($NF):"(sem)"; c[n]++} END{for(k in c) print c[k], k}' | sort -rn
```

## 6. O que esta medição prova — e o que não prova

| Prova | Não prova |
|---|---|
| que existe um cliente local com 7.147 arquivos e 567 MB, nesta máquina, nesta data | que o cliente fala o protocolo descrito em `wyd-protocol` |
| que `WYD.exe` tem 2.347.008 bytes e hash `04c9d2a1…` | que este deva ser o alvo — isso é decisão (`LPRT-09`, 2026-11-23), não medição |
| que o pacote `client.zip` é idêntico ao conteúdo extraído ou não | que `strdef.bin`, assets ou patch têm o conteúdo esperado |
| a distribuição de formatos presentes | qualquer regra de jogo, dano, drop ou progressão |

Qualquer afirmação das colunas da direita exige **captura datada** ou leitura de conteúdo com fonte
declarada, não inferência a partir desta tabela.

## 7. Plano de captura — o que falta para a base real

O procedimento é do `wyd-protocol` (`goldens/como-capturar.md`) e não é replicado aqui. O que este
registro fixa é o **estado**:

| Passo | Situação | Depende de |
|---|---|---|
| identidade do build-alvo | medida e promovida a alvo em 2026-11-23 | `Q-PRT-1`/`LPRT-09` (decidida); `LCLI-01` fecha nela |
| ferramenta de gravação de tráfego | não existe; pasta e escopo aprovados | `LCLI-04` (D3–D6 decididos; implementação pendente) |
| endpoint que aceita a conexão do cliente | não existe — nenhum servidor roda esta versão | `wyd-server` / listener sob nosso controle |
| ficha de origem + anonimização | processo decidido (`LPRT-10`), aprovador não nomeado | `LCLI-03` |
| primeira captura aceita | **0 capturas** (2026-11-22) | os quatro itens acima |

Enquanto o passo 3 não existir, não há como produzir tráfego legítimo do cliente: gravar bytes exige
algo do outro lado do socket. Por isso `LPRT-03` e `LPRT-06` seguem abertas — e por isso nenhuma delas
foi respondida por estimativa. `LPRT-09` não esperou captura: fechou por decisão de alvo em 2026-11-23.

## 8. Como re-medir

```bash
find wyd-client/files -type f | wc -l
du -sh wyd-client/files
sha256sum wyd-client/files/client.zip wyd-client/files/client/WYD.exe
```

Se qualquer número divergir do §3–§5, este registro está vencido: a atualização é de quem mediu, com
data nova — não se corrige número sem rodar o comando.
