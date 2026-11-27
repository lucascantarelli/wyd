# `wyd-client` — registro medido do cliente local

> Pergunta: "o que já foi medido do cliente real, com que comando e o que essa medição **não** prova?"
> Público: pesquisa (RE), arquitetura e QA
> Dono: equipe do `wyd-client`
> Base: `LPRT-10` (processo de captura), `goldens/como-capturar.md` (procedimento)
> Revisado em: 2026-11-27

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
| Versionado no git | 0 arquivo — `git ls-files wyd-client` devolve só `wyd-client/tools/.gitkeep` |
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

## 6.1 Evidência estática local e auditoria de `deprecated` (2026-11-26)

**Escopo medido:** análise offline do `WYD.exe` local e leitura de amostras de assets. Isso não executa
o cliente, não captura tráfego e não valida comportamento em runtime. A pasta `deprecated/` é ignorada
pelo git e continua sendo referência de investigação, não fonte normativa (`AGENTS.md` §3).

**Reprodução da análise do executável** (Node v26.13.0 e `objdump` disponível nesta estação):

```bash
cd deprecated
node tools/client/analyse.mjs ../wyd-client/files/client/WYD.exe
```

O comando terminou com exit 0 e conferiu o executável contra o build-alvo 7662 já registrado em §4.
Gerou localmente `deprecated/docs/cliente/gerado/{pe,strings,evidencia,formatos}.json` e
`ancoras.txt`; esses arquivos ficam sob uma árvore ignorada, não são evidência versionada e não
substituem este registro.

| Medição reproduzida | Resultado | Limite da conclusão |
|---|---|---|
| Formato PE | PE32 / i386; 4 seções; 18 DLLs importadas e 268 funções | inventário estático do executável, não comportamento em runtime |
| Endereços herdados no pipeline | 6/6 endereços são início de instrução no disassemble; apenas 1/6 coincide com prólogo detectado | os nomes herdados não provam entradas de função; cinco endereços são pontos internos, não funções confirmadas |
| Sites de checksum | 3/3 contêm instrução `jmp rel8` (`EB`) nos endereços herdados | prova os bytes/opcodes no arquivo local; não prova o efeito dinâmico de cada fluxo |
| CPSock | bloco de tabela de 512 bytes idêntico à tabela usada pelo analisador; constante `INITCODE` localizada como dado | a conclusão do pipeline para o header de 12 bytes é **indireta**; offsets/campos continuam sem trace dinâmico |
| Candidatos de limite | padrão imediato `0x2000` encontrado em seis posições | são candidatos estáticos, não prova isolada de regra de enquadramento |

**Amostras de assets**, lidas sem exportar ou versionar conteúdo. Comando reproduzível, a partir de
`deprecated/`:

```bash
node --input-type=module -e "import { analyseFormats } from './tools/client/formats.mjs'; console.log(JSON.stringify(analyseFormats({root:'../wyd-client/files/client'}), null, 2));"
```

| Amostra | Resultado observado | Estado da interpretação |
|---|---|---|
| `client/NUI/ServerList.wyt` | magic `WT10`, 287×280, 32 bpp | estrutura raster decodificada nesta amostra |
| `client/NUI/ccmode.wyt` | magic `WT10`, 64×64, 24 bpp | estrutura raster decodificada nesta amostra |
| `client/UI/AutoTrade.wyt` | magic `WT10`, 220×199, 24 bpp | estrutura raster decodificada nesta amostra |
| `client/UI/Combine102.wyt` | magic `WT10`, 228×313, 32 bpp | estrutura raster decodificada nesta amostra |
| `client/Mesh/Bownew1.msa`, `client/Mesh/CharmPowder.msa` | em ambas: `u32 @0 = 274`, `u32 @4 = 32`, `u32 @8 = 1`; caminho de textura lido em `@34` | analisador mantém geometria, esqueleto e animação opacos; os valores repetidos são candidatos, não semântica aprovada |
| `client/sn.bin` | leitura por NUL encontra rótulos; primeiro byte `0xd6` não explicado | primeiro byte continua não verificado |
| `client/serverlist.bin` | 7.040 bytes; tentativas do analisador não produziram estrutura textual coerente | layout permanece opaco; grupo, índice, IP e porta não foram provados |

**Auditoria dos artefatos legados de captura:** `deprecated/tools/client/captura.mjs` declara que gera
fixtures sintéticas; a validação `node tools/client/captura.mjs --check` terminou 0 e aceitou 5/5 desses
fixtures como válidos para o codec. Isso não é prova de comportamento do cliente. O arquivo legado
`deprecated/crates/wyd-protocol/tests/fixtures/cliente/captura-handshake.json` tem `conexoes: []`, não
contém `frames` nem bytes capturados, e registra endereço `0.0.0.0:8281`; portanto não comprova handshake
nem é golden. O conversor `deprecated/tools/client/captura-live.mjs` também oferece modos de servidor
falso e proxy para um alvo configurável, com armazenamento de fluxo bruto; ele não implementa o
contrato ECAP aprovado para A1 e **não foi executado** nesta medição.

**Conclusão e rastreabilidade:** há evidência estática reproduzida sobre a identidade do executável,
alguns endereços/instruções e amostras de formatos; não há evidência dinâmica de comunicação cliente–servidor.
Os resultados que sugerem layout ou semântica permanecem `H` vinculadas a `LPRT-03`/`LPRT-06`, e dados
que aparecem só no legado não são promovidos a regra. A captura completa continua dependendo do aceite
técnico do ECAP, da implementação do peer A + ECAP + CLI, do operador nomeado em `LCLI-03` e da segunda
leitura prevista em `goldens/como-capturar.md`. Esta seção não aceita o mapa funcional nem fecha qualquer
lacuna.

## 7. Plano de captura — o que falta para a base real

O procedimento é do `wyd-protocol` (`goldens/como-capturar.md`) e não é replicado aqui. O que este
registro fixa é o **estado**:

| Passo | Situação | Depende de |
|---|---|---|
| identidade do build-alvo | medida e promovida a alvo em 2026-11-23 | `Q-PRT-1`/`LPRT-09` (decidida); `LCLI-01` fecha nela |
| ferramenta de gravação de tráfego | não existe; pasta e escopo aprovados | `LCLI-04` (D3–D6 decididos; implementação pendente) |
| endpoint que aceita a conexão do cliente | não existe — nenhum servidor roda esta versão | `wyd-server` / listener sob nosso controle |
| ficha de origem + anonimização | processo decidido (`LPRT-10`); QA designado: **Lucas Cantarelli** (dono do produto), que **não é o operador**; quem captura não assina; o nome do operador entra na ficha quando ela existir | `LCLI-03` |
| primeira captura aceita | **0 capturas** (tentativa medida em 2026-11-26 sem conexão CPSock; §7.1) | os quatro itens acima |

Enquanto o passo 3 não existir, não há como produzir tráfego legítimo do cliente: gravar bytes exige
algo do outro lado do socket. Por isso `LPRT-03` e `LPRT-06` seguem abertas — e por isso nenhuma delas
foi respondida por estimativa. `LPRT-09` não esperou captura: fechou por decisão de alvo em 2026-11-23.

### 7.1 Tentativa de captura real (2026-11-26, medida e sem captura)

Execução local nesta estação, para medir até onde o cliente chega sem peer. Não gerou `capture.wydcap`,
não gerou golden e não alterou o cliente de forma permanente.

| Passo | Execução | Resultado observado |
|---|---|---|
| 1. apontar o cliente para loopback | `serverlist.bin` e `local-serverlist.bin` reescritos no formato 10×11×64 (grupo 1, canal 1 = `127.0.0.1`) | `--print` devolveu `1 1 127.0.0.1` nos dois; originais restaurados depois (`serverlist.bin` com hash `d835aef985d7256683f5370a526bf783c72a611cccb62eba714e483370e8cec0`) |
| 2. responder o status | listener HTTP em `0.0.0.0:80` devolvendo o `serv00.htm` mínimo | `GET /serv00.htm HTTP/1.1` recebido do cliente — tráfego real do binário, mas não é handshake |
| 3. abrir o CPSock | listener TCP em `127.0.0.1:8281`, janela de 240 s | nenhuma conexão; `netstat -ano` listou apenas o listener |
| 4. dirigir o cliente | `WYD.exe` elevado (`requireAdministrator`) e cliques por `SendInput` `MOVE\\|ABSOLUTE` em servidor (330,318), canal (465,318) e Conectar (330,504) | o cliente chegou à tela de Login sem abrir socket |
| 5. enviar o login | não executado | exigiria credencial, e [`como-capturar.md`](../wyd-protocol/docs/goldens/como-capturar.md) §8 proíbe capturar senha, PIN ou token |

**O que a tentativa prova.** O cliente não abre o CPSock antes do envio do login: chegou ao Login com o
listener ativo e nenhuma conexão em 240 s. O `GET /serv00.htm` é tráfego real e foi atendido; a coluna
Canal depende dele, mas ele não é handshake e não vira golden.

**Entrada aberta (a única).** Para existir sessão A1 falta **conta de teste em um peer que responda ao
CPSock** ou **aprovação escrita para uma sessão com credencial sintética**, descartando os frames de
login antes de qualquer artefato. Nem a conta nem o peer existem no repositório; o nome do operador na
ficha continua pendente em `LCLI-03`. Sem uma das duas, `LPRT-03`/`LPRT-06` não têm bruto.

**Evidência datada deste estado (2026-11-26):**
`planejamento-1.0.0/wyd-ops/docs/devops/evidencias-de-depreciacao.md` §4 e
`planejamento-1.0.0/wyd-ops/inventario-depreciacao.tsv` (CAP-01 e CAP-02).

## 8. Como re-medir

```bash
find wyd-client/files -type f | wc -l
du -sh wyd-client/files
sha256sum wyd-client/files/client.zip wyd-client/files/client/WYD.exe
```

Se qualquer número divergir do §3–§5, este registro está vencido: a atualização é de quem mediu, com
data nova — não se corrige número sem rodar o comando.

## 9. Layout de `ItemList.bin` e `Itemname.bin` (medido em 2026-11-27)

Bloco medido por comando, em 2026-11-27, sobre `wyd-client/files/client/`. A hipótese de partida veio
da análise do `deprecated` (`…/análises/wyd-db/database/05-conteudo-e-procedencia.md` §2–§3 e do
script `verificar-itemname.mjs`); **o que está nesta seção é o que a medição confirmou**, não o que o
documento antigo afirmava.

### 9.1 `ItemList.bin` — 6.500 slots, registro de 140 B, XOR `0x5A`

`910.004 = 0 + 6.500 × 140 + 4` (trailer `ef c7 81 03` no fim; **não há cabeçalho**). O arquivo é
codificado por **XOR `0x5A` em cada byte**: é por isso que ele "abre com preenchimento `0x5A`" — o
preenchimento é o zero codificado.

| Offset | Bytes | Campo | Medição (`V`, 2026-11-27) |
|---:|---:|---|---|
| 0 | 64 | `name`, ASCII terminado em NUL | 3.216 slots com nome; maior nome 40 B; decodifica com `byte ^ 0x5A` |
| 64 | 76 | carga restante do slot | depois de decodificar o nome, o resto do registro é zero em 6 de 6.500 slots — logo há dado aqui, não espaço morto |

Contagens que fecham: **6.500 slots**, **3.216 com nome**, **3.284 vazios**, nome máximo de 40 B.
A contagem de itens do catálogo é do `wyd-db` (`plano-migrations.md` §4); esta seção mede o arquivo.

### 9.2 `Itemname.bin` — 3.163 registros de 68 B, máscara `byte − posição`

`215.084 = 3.163 × 68`, sem cabeçalho. Registro: `u32 LE` (índice do item) + 62 B de nome + `u16`
trailer (= 0). O nome **não** é texto direto: `plain[p] = (byte[4 + p] − p) & 0xFF`, terminado em NUL.
É esse deslocamento por posição que produz a rampa crescente (`0x0B, 0x0C, … 0x3D`) depois do nome.

### 9.3 O que os dois arquivos dizem um do outro

| Medição | Valor |
|---|---:|
| registros de `Itemname.bin` | 3.163 |
| nomes idênticos aos do `ItemList.bin` | **3.109** |
| divergentes | **54** |

Exemplos medidos de divergência: slot 473 (`Caixa_Misteriosa` no `Itemname`, `Presente_de_Natal` no
`ItemList`); slot 1731 (`Carta_de_Duelo(A)` × `Carta_de_Duelo_(A)`); slots 649 e 650
(`not_used` no `Itemname`, vazio no `ItemList`). Divergência é dado, não erro de leitura: a autoridade
do nome é decisão de curadoria, e a lista completa das 54 é insumo dela.

### 9.4 O que fica em `H`

- Semântica de `IndexMesh`/`IndexTexture` (`@64`, `@66`) e a ordem dos 12 pares de efeito (`@80`):
  os offsets estão no documento antigo, mas **não** foram reconferidos aqui.
- Preço, `unique_id`, `pos`, `extra` e `grade` (`@128`…`@138`): também não reconferidos nesta rodada.

### 9.5 O que isso destrava

O nome e a contagem do catálogo deixam de ser bloqueio de dado para o seed `0017`: os 3.216 nomes são
legíveis por esquema provado. O que continua faltando para o `0017` é a curadoria das colunas
numéricas (`Req*`, preço, `grade`, efeitos), não a decodificação do arquivo.
