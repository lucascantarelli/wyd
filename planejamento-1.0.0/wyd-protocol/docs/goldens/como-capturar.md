> Pergunta: "como capturo a prova que fecha `LPRT-03`, `LPRT-06` e `LPRT-09`?"
> Público: frente `wyd-client`, pesquisa (RE) e QA
> Dono: equipe do wyd-protocol
> Base: `lacunas.md` §0.4, §2 e §3; `wyd-protocol-1.0.0.md` §6.7 e §9.1; processo de `LPRT-10`
> Revisado em: 2026-11-20

**Estado:** proposto (2026-11-20)

Este é o procedimento operacional das **três capturas que ainda faltam** no `wyd-protocol`:
`LPRT-03` (origem da semente de ofuscação no handshake), `LPRT-06` (encoding, tamanho e terminador
de string) e `LPRT-09` (build do cliente que serve de alvo). O processo de captura e aprovação já foi
decidido em `LPRT-10`; esta página diz **o que fazer em cada uma**, não o que foi medido.

As três não entram na mesma fatia: a captura de `LPRT-03` e os goldens do fio são o que destrava a
`0.1.0`, e a captura de `LPRT-06` fecha antes da família `personagem` — o nome já atravessa o fio
como string. A ordem é de `LPRT-10` e de §9.1 do plano diretor; esta página registra o efeito dela
para a captura e não a redefine.

**Esta página não afirma byte nenhum.** Nenhuma captura existe, nenhum golden foi aceito e nenhuma
destas três lacunas está fechada: as três seguem `aberta` em `lacunas.md`, e continuam `H` até a
ficha datada e a segunda leitura. Captura feita não é captura aceita.

## 1. O que a captura precisa entregar

Uma captura só vale como prova quando entrega as quatro partes juntas:

1. **o fluxo bruto dos dois sentidos**, com o instante relativo de cada bloco;
2. **a ficha de origem** de §7, preenchida antes da captura e completada depois;
3. **o texto ou o estado do cliente no mesmo instante**, quando o byte depende de algo visível na tela
   (nome, chat, versão);
4. **a segunda leitura**: outra pessoa refaz a medição a partir do bruto e da ficha, sem falar com
   quem capturou, e concorda com o que foi anotado.

Falta uma das quatro: a captura é registro, não prova, e a lacuna continua aberta.

## 2. Regras que valem para as três

1. **Pacote não é frame.** A segmentação do TCP e o tamanho que a pilha entrega não são o `Size` do
   fio nem a fronteira da mensagem. A captura guarda o fluxo contínuo por sentido, com marcas de
   tempo; a fronteira é assunto do codec e do golden, não do pacote.
2. **Os dois sentidos, sempre.** Medir só `client → game` esconde resposta do servidor, e o handshake
   é justamente pergunta e resposta.
3. **Um cenário por captura.** Uma sessão que faz login, escolhe personagem e fala no chat produz um
   bruto em que não dá para saber qual bloco é qual. Cada cenário tem a sua ficha.
4. **Texto e bytes no mesmo registro.** Quando o byte corresponde a algo digitado ou exibido, a ficha
   cita o texto literal e o instante, para que a segunda leitura compare os dois.
5. **A build entra na ficha.** Toda captura é amarrada a uma build identificada (arquivo e hash); sem
   isso o byte não pode ser atribuído a versão nenhuma, e `LPRT-09` continua aberta para ele.
6. **Bruto não entra no repositório.** O bruto pode conter conta, personagem e endereço; ele fica fora
   do repositório, no local registrado na ficha. O repositório recebe só o frame anonimizado e a ficha
   resumida.
7. **Medir não é concluir.** A ficha registra o observado ("o campo recebeu estes bytes", "o valor
   apareceu neste ponto do fluxo"), nunca a causa ("a semente vem do cliente"). A conclusão é escrita
   na página dona, depois da segunda leitura, com o ID do golden.
8. **Ambiente controlado.** A captura é feita contra servidor local ou proxy sob controle da própria
   equipe; captura de tráfego de produção é frente de `wyd-ops` e não está descrita aqui.

**Onde cada uma entra na série.** A ordem e o gate são de §9.1 do plano diretor, que é o dono deles;
aqui fica só o efeito para a captura.

| Captura | Precisa estar aceita | Por quê |
|---|---|---|
| `LPRT-03` e os goldens do fio | antes de a `0.1.0` fechar | a primeira fatia é o fio sem payload, e ela só fecha com o handshake medido e o byte provado |
| `LPRT-06` | antes de a família `personagem` entrar (`0.2.0`) | nome de personagem já atravessa o fio como string; encoding e terminador precisam estar fechados antes |
| `LPRT-09` | acompanha todas as capturas; fecha até a `1.0.0` | toda captura é amarrada a uma build; sem build identificada, o byte não pertence a versão nenhuma |

## 3. `LPRT-03` — handshake e origem da `KeyWord`

**Cenário.** Abrir o cliente contra o ambiente controlado e capturar do primeiro byte até o cliente
entrar no mundo, sem outra ação. Registrar a build usada.

**O que capturar.** O fluxo dos dois sentidos desde antes de qualquer resposta do servidor; a ordem e o
intervalo relativo entre o primeiro bloco do cliente e o primeiro bloco do servidor; o tamanho de cada
bloco; e o instante em que a tela muda de estado (conexão aceita, lista de personagens).

**O que medir, sem concluir.**

- se existe, no fluxo do cliente, um campo candidato a semente, e em que ponto do fluxo ele aparece;
- se o servidor devolve o mesmo valor ou um valor derivado, e em que ponto;
- se o candidato coincide, byte a byte, com algum par da tabela citada no legado (`lacunas.md` §0.4) —
  a comparação é por igualdade, sem supor que o cliente use a mesma tabela;
- se o candidato aparece antes ou depois da transformação de ofuscação, e o que permite afirmar isso;
- qual seria o offset do campo dentro do frame, **derivado da fronteira do frame**, não do pacote.

**Fecha quando.** A ficha datada identifica o candidato, o ponto do handshake, o offset derivado e a
origem da afirmação, a segunda leitura confere no bruto e `keyword-e-checksum.md` passa a citar o
golden — é esta captura, junto dos goldens do fio, que destrava a `0.1.0` (§9.1). Enquanto isso, a
origem da semente continua `H`.

**Inconclusiva se.** O fluxo não foi capturado desde o primeiro byte, veio só um sentido, ou a build
não está identificada — mesmo que o valor "pareça" reconhecível.

## 4. `LPRT-06` — texto acentuado

**Cenário.** Com um personagem de teste criado para a captura, enviar texto conhecido: um nome com
acentuação, uma linha de chat com acentuação e uma linha que preencha o campo até o limite aceito pelo
cliente. Se o cliente recusar ou cortar o excesso, registrar o que aconteceu na tela.

**O que capturar.** O fluxo dos dois sentidos durante cada envio; o texto exato digitado, copiado para
a ficha; a captura de tela do mesmo instante em que o texto aparece; e a build usada.

**O que medir, sem concluir.**

- a sequência exata de bytes do campo, transcrita como está no bruto;
- a comparação dessa sequência com candidatos de encoding (por exemplo, tabela de um byte, UTF-8,
  UTF-16), apresentada como comparação, sem declarar qual é;
- se há terminador depois do texto, de que largura, e se ele também aparece quando o campo está cheio;
- se o tamanho declarado para a string conta o terminador ou só o texto;
- o que o cliente faz quando o texto excede o campo: recusa, corta ou envia assim mesmo.

**Fecha quando.** Bytes e texto visível estão no mesmo registro datado, a segunda leitura reproduz a
transcrição e `payloads/tipos-e-encoding.md` cita o golden. Isso acontece **antes** de a família
`personagem` entrar (`0.2.0`), porque o nome já atravessa o fio como string. Até lá, encoding e
terminador seguem `H`.

**Inconclusiva se.** O texto digitado não foi registrado literalmente, a tela não foi capturada no
mesmo instante, ou o frame não pôde ser isolado do resto do fluxo.

## 5. `LPRT-09` — build do cliente

**Cenário.** Duas vias, e cada uma vale como registro próprio: (a) leitura do binário — nome do
arquivo, data e hash, e o ponto onde a versão aparece; (b) captura do pacote de boas-vindas ou do
handshake em que a build é declarada, se ela for declarada no fio.

**O que capturar.** O identificador do binário usado em **todas** as outras capturas; o trecho do
fluxo em que a versão aparece, se aparecer; e a build anterior, se houver, para comparação.

**O que medir, sem concluir.**

- qual versão o cliente declara, e por qual das duas vias isso foi lido;
- se o valor declarado coincide com as duas referências já registradas (`APP_VERSION` do legado e a
  versão provável do repositório), ou diverge das duas;
- o que muda entre as builds que toque cabeçalho, flags ou catálogo — só o que a comparação mostrar,
  sem completar por dedução.

**Fecha quando.** Há registro datado que identifica a build e a fonte da leitura; a segunda leitura
confere; e `versionamento-e-compatibilidade.md` e o catálogo citam o registro. Até lá o alvo é
**provável**, não confirmado.

**Inconclusiva se.** A build não pôde ser identificada ou o trecho lido não declara versão alguma.

## 6. Anonimização

| Dado | Tratamento | Onde pode aparecer |
|---|---|---|
| login ou conta | apelido `ACC-01`, `ACC-02`… | ficha; nunca o login real |
| nome de personagem | personagem criado para a captura; se o nome real aparecer no frame, substituir e registrar a substituição | ficha e frame anonimizado |
| endereço IP e porta | `IP-01` e `PORT-01` | ficha; no frame, se o endereço aparecer em payload, mascarar |
| fala de outras pessoas | não capturar: janela de chat limpa, sem terceiros | — |
| data e hora | manter em UTC na ficha | ficha |
| tela | capturar só a janela que prova o texto, sem abas, sobreposições ou dados pessoais | ficha |
| binário do cliente | não versionar nem redistribuir; guardar só o identificador e o trecho lido | ficha |

Anonimizar não é apagar o que prova: o frame entregue ao repositório mantém os bytes que sustentam a
medição e perde só o que identifica pessoa, conta ou máquina.

## 7. Ficha de origem

A ficha é escrita **antes** da captura (campos 1–7) e completada depois (8–12). Ela acompanha o bruto
fora do repositório e uma cópia resumida acompanha o frame anonimizado.

1. identificação provisória da captura: data, cenário e lacuna que ela ataca (`LPRT-03`, `06` ou `09`);
2. data e hora em UTC;
3. operador que capturou;
4. build do cliente: nome do arquivo, data e hash;
5. ambiente: servidor local ou proxy, com a versão usada;
6. ferramenta de captura e versão;
7. passos executados no cliente, na ordem, com o texto digitado quando houver;
8. arquivos brutos e o caminho onde estão guardados, fora do repositório;
9. arquivos anonimizados e o destino em `tests/golden/` (planejamento);
10. o que foi medido e o que **não** foi medido, item por item;
11. segunda leitura: quem conferiu, quando e o que refez;
12. ID do golden e a linha do catálogo, preenchidos só na aceitação.

**Onde a origem fica registrada, por parte:**

| Parte | Onde | Dono |
|---|---|---|
| bruto e ficha completa | fora do repositório, caminho no campo 8 | frente `wyd-client` |
| frame anonimizado e ficha resumida | `tests/golden/` (planejamento) | frente `wyd-client` |
| entrada pública: ID, família, opcode, origem, data e o que prova | `goldens/catalogo.md` (planejamento) | equipe do wyd-protocol |
| execução pendente e estado | ficha de captura e `lacunas.md` §1 | `lacunas.md` |

Enquanto `goldens/catalogo.md` não existir, nenhum ID é alocado: a captura fica identificada pela
ficha e a lacuna continua `aberta`. O ID `G-<familia>-<nn>` nasce na aceitação, no catálogo.

## 8. O que nunca capturar

- senha, PIN, token de sessão ou qualquer credencial, mesmo que apareçam dentro de um frame;
- dado de pagamento, documento, e-mail ou telefone;
- conversa de outras pessoas: o cenário usa conta e personagem de teste, e a janela de chat fica limpa;
- tráfego de jogador real em produção — captura de produção é frente de `wyd-ops`, com outro processo;
- o binário do cliente como artefato versionado: a captura é de bytes do fio, não do executável.

Credencial que aparecer no bruto descarta a captura inteira: não se "remove o campo e segue".

## 9. Quando a captura não fecha

Captura que não responde a pergunta é resultado, não fracasso: registra-se o que ela mostrou e o que
ficou sem resposta, com o motivo, e a lacuna continua aberta. Nada do que ficou parcial entra em
página de `wire/`, `payloads/` ou no catálogo — valor parcial em página é defeito bloqueante.

Se o observado contrariar uma decisão de 2026-11-20, o caminho não é ajustar a página: a discussão
reabre em `lacunas.md`, com a evidência nova, e o dono decide de novo.

## 10. O que esta página não faz

- não mede, não conclui e não afirma byte: descreve o procedimento;
- não está `aceita`: o estado é `proposto` até a revisão de quem não escreveu;
- não aloca golden nem ID: o catálogo é o dono;
- não substitui o processo de `LPRT-10`: é a aplicação dele às três lacunas abertas;
- não decide encoding, origem da semente nem build-alvo;
- não descreve captura em produção, que é de `wyd-ops`.
