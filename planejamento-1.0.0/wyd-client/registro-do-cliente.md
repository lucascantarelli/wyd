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

O inventário item a item, com caminho relativo, tamanho e `sha256`, está em
[`docs/specs/inventario-da-arvore.md`](docs/specs/inventario-da-arvore.md); o total lógico exato é
576.124.039 bytes e `du -sh` continua 567 MB.

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

## 5.1 Lista fechada do patch da 1.0.0 (`LCLI-02`, 2026-11-25)

`LCLI-02` fechou em 2026-11-25 pela opção A do bloco de decisão: **dados + launcher próprio**, sem
binário de terceiro. A tabela abaixo é a entrada do manifesto do patch
(`planejamento-1.0.0/wyd-launcher/docs/specs/patch-1.0.0.json`, formato do `ADR-LCH-0001`); cada linha
foi medida nesta máquina em 2026-11-25 com `sha256sum`. A tabela é declarada **antes** do bloco de
comando de §5 porque é ela que fixa o conteúdo do patch; o comando de §5 continua sendo o da
distribuição por extensão.

| # | Caminho no cliente | Papel | Origem | Bytes | `sha256` medido |
|---|---|---|---|---:|---|
| 1 | `client/UI/strdef.bin` | tabela de strings binária | terceiro (base medida) | 255.992 | `26a6140b9a31258b018e17c53ca47df17c532e3ae362d73ba6c764d38686a81f` |
| 2 | `client/UI/strdef.txt` | tabela de strings legível | terceiro (base medida) | 56.792 | `295cab75ab23f371d28d1bd2a2a462098d1cc8244e819b4e3f8fd684b31ba347` |
| 3 | `client/UI/UI_strdef.bin` | tabela de strings da UI | terceiro (base medida) | 56.324 | `af5f22b80ee4333dd52b603eb48cf84477e7e830b1a790727709e4e8fcdbdb89` |

**O que o patch entrega.** Os três arquivos byte a byte: o patch da `1.0.0` fixa a base de tabelas
contra a qual o servidor foi construído, e não uma tabela modificada. Tabela modificada — nome novo de
item, skill ou mob — só existe quando houver conteúdo novo na área de conteúdo (`M21`) e passa pela
mesma verificação antes de entrar. Nenhum executável de terceiro entra no pacote; o launcher é peça
própria.

**Versionamento.** `version` é a versão do patch (`1.0.0`), `min_version` é `1.0.0`, `channel` é
`stable` e `client_build` é o `sha256` do `WYD.exe` alvo (`Q-PRT-1`/`LPRT-09`): patch aplicado sobre
outro build é recusado pelo manifesto. O campo `url` fica ausente enquanto não houver host de release
(`OPS-04`) e o manifesto não é assinado até `LLCH-03`; nem um nem outro impedem a verificação de
integridade da lista.

**Verificação.** `crates/wyd-launcher` lê o manifesto (`src/manifest.rs`) e compara tamanho e `sha256`
de cada arquivo (`src/verify.rs`); qualquer divergência recusa o patch inteiro e o relatório diz qual
arquivo e por quê. Os testes estão em `tests/manifesto.rs` (fixtures centralizados em `tests/fixtures/`,
com hash divergente, tamanho divergente e arquivo ausente) e `tests/patch_1_0_0.rs`, que verifica este
manifesto contra a árvore local quando ela existe.

**O que isso não libera.** A lista é metadado; a distribuição segue congelada por `LCLI-05`, que espera
a evidência do titular para `Q-IP-1`/`Q-IP-2`. Fechar `LCLI-02` decide **o que** o patch entrega e como
verifica; não autoriza publicá-lo.

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
| ficha de origem + anonimização | processo decidido (`LPRT-10`); QA designado: **Lucas Cantarelli** (dono do produto), que **não é o operador**; quem captura não assina; o nome do operador entra na ficha quando ela existir | `LCLI-03` |
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
