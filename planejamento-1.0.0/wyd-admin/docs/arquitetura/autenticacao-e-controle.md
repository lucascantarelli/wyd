> Pergunta: "como o BFF do portal prova ao `wyd-admin` que o chamador é ele, e como o admin pede kick e broadcast ao `wyd-server`?"
> Público: Arquitetura, `wyd-ops`, `wyd-admin` e quem for planejar o `wyd-server`
> Dono: arquitetura (decisão); `wyd-admin` (forma do elo portal → admin)
> Base: `contrato-c4.md` §1, §4.4, §4.5 e §7; `lacunas.md` `LAC-ADM-03` e `LAC-ADM-04`; `RNF-ADM-001`/`RNF-ADM-002`; `AGENTS.md` §5.8
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22) — **decisão aguardando o dono**. Duas lacunas, uma por seção:
`LAC-ADM-04` em §2 e `LAC-ADM-03` em §3. Nada nesta página está decidido; a recomendação é minha e o
"sim" é do dono (`AGENTS.md` §5.8). Enquanto não fechar, nenhum `GRANT`, tabela ou migration depende
dela, e `ADM-02` (autenticação) e `ADM-08` (kick/broadcast) seguem bloqueadas por decisão, não por
schema.

## 1. Os dois elos, e o que eles **não** são

| Elo | Quem chama | O que precisa provar | Onde a forma mora |
|---|---|---|---|
| portal → admin | BFF do portal | que **o chamador é o BFF daquele ambiente**; a pessoa é afirmada no cabeçalho e o portal é a autoridade dela | esta página (§2) + `contrato-c4.md` §1 |
| admin → `wyd-server` | `wyd-admin` | que **o comando vem do admin** e vale para o processo daquele canal | esta página (§3); o endpoint nasce no plano do `wyd-server` |

Os dois elos **não** autenticam jogador, não emitem sessão e não carregam segundo fator: isso é do
portal (`0003`/`0004`, `LAC-POR-05`). E nenhum dos dois é rota pública — o admin não é exposto à
internet e o endpoint de controle mora na rede interna (`wyd-ops`).

Fronteira de confiança, explicitada porque é o que a decisão compra: **o admin não autentica pessoa**. Ele
confia em `X-Staff-Person-Id` porque quem o assina é o único detentor do token de serviço. Se o token
vazar, o atacante não ganha "ser o staff 42": ganha "ser o BFF", que é mais amplo. É esse o risco que a
escolha precisa dimensionar, não a troca de um cabeçalho.

## 2. `LAC-ADM-04` — autenticação portal → admin

### 2.1 O que a decisão precisa resolver

1. Recusar chamada de fora da rede interna (inclusive de outro ambiente de deploy).
2. Impedir que um token de outro serviço (`wyd_game`, `wyd_portal` do banco) sirva como credencial de API.
3. Permitir rotação de segredo **sem janela de erro** — portal e admin não sobem no mesmo instante.
4. Não guardar segredo em tabela, log, resposta ou repositório (`AGENTS.md` §5.7).
5. Caber no que já está fixo no contrato: `Authorization: Bearer`, `401` sem token válido, `403` por
   papel, `400` sem `X-Trace-Id` (`RN-ADM-002`, `RNF-ADM-002`).

### 2.2 Opções

| # | Forma | Custo de 1.0.0 | Impede | Não impede | Veredito |
|---|---|---|---|---|---|
| **A** | **Token de serviço estático por ambiente** em `Authorization: Bearer`, sobre TLS interno, com **dois valores aceitos** durante a rotação | um segredo por ambiente no `wyd-ops`; nenhuma infra nova; ~20 linhas no componente `auth` | chamada sem token; chamada de outro ambiente (token diferente); token de banco usado como API | nada contra token vazado, exceto a rotação; não amarra o corpo ao chamador | **recomendada** |
| B | mTLS: o BFF apresenta certificado de cliente emitido por CA interna | PKI interna (emissão, distribuição, expiração e revogação) + validação no admin | tudo o que A impede, **e** não depende de segredo copiável | o mesmo: vazamento do certificado continua sendo "ser o BFF" | melhor tecnicamente, caro demais para o único chamador de 1.0.0 |
| C | HMAC por requisição (`X-Signature` sobre método + rota + `sha256(corpo)` + `X-Staff-Person-Id` + timestamp) | código nos dois lados + janela de relógio; para anti-replay de `GET`, cache com estado | token vazado sozinho não basta (falta a chave de assinatura); corpo e ator amarrados | ainda é um segredo compartilhado; acrescenta o cache de nonce que a 1.0.0 não tem | evolução de A, não substituto |
| D | OIDC client credentials com JWT verificado por JWKS | um provedor de identidade de serviço (novo componente e novo custo) | tudo de B, com rotação automática | — | fora de escopo: o stack é Postgres + 1 serviço por canal |

### 2.3 Recomendação (opção A), com o que a torna defensável

- **Um token, um detentor:** `WYD_ADMIN_SERVICE_TOKEN` existe só no BFF e no admin, por ambiente
  (`dev`, `staging`, `prod`), com valor distinto em cada um. Ambiente trocado é `401`, não "funciona".
- **Rotação em dois passos:** o admin aceita `WYD_ADMIN_SERVICE_TOKEN` e `WYD_ADMIN_SERVICE_TOKEN_NEXT`;
  o portal passa a enviar o novo; no deploy seguinte o antigo vira o atual e o `NEXT` fica vazio. Nenhuma
  janela com `401`.
- **TLS obrigatório no elo**, mesmo em rede interna: sem ele o token trafega em claro e a opção A deixa
  de valer.
- **Comparação em tempo constante** e token de 32 bytes ou mais (base64url), gerado pelo `wyd-ops`; o
  valor nunca entra em log, resposta, issue ou documento (`AGENTS.md` §5.7).
- **O que o token autoriza é o papel, não a pessoa:** `RN-ADM-001` continua decidindo `moderator` ×
  `admin`, e nas rotas de custódia o ator é `X-Actor-Account-Id` com conferência de titular no banco.

### 2.4 O que muda se o dono aprovar

`ADM-02` destrava (autenticação, papel mínimo e `trace_id`), `RNF-ADM-002` ganha o mecanismo, o
componente `auth` de `componentes.md` deixa de depender de lacuna e o `wyd-ops` recebe duas variáveis por
ambiente e um passo de rotação. **Nenhuma tabela, `GRANT` ou contagem de série muda** — a decisão é de
transporte, não de schema.

## 3. `LAC-ADM-03` — canal de controle com o `wyd-server`

### 3.1 O que a decisão precisa resolver

1. `POST /sessions/{id}/kick` só grava `released_at`/`released_reason = 'admin'` **depois do ack**
   (`contrato-c4.md` §4.4) — sem canal, essa promessa não existe.
2. `POST /broadcasts` precisa de contagem de destinos (§4.5) e não escreve tabela de jogo.
3. Prazo de 1,5 s no elo admin → server (`RNF-ADM-001`); timeout é estado **incerto** e reconcilia por
   `GET /actions/{key}` (§6).
4. O destinatário é o **processo de um canal**, que pode estar fora do ar enquanto o resto do servidor
   responde.
5. O lado de lá não tem plano: `planejamento-1.0.0/wyd-server/` e `wyd-net/` estão **vazios** (verificado
   em 2026-11-22). Quem fecha a forma do endpoint é o plano do `wyd-server`; esta decisão define só o
   **modelo de transporte** que ele terá de cumprir.

### 3.2 Opções

| # | Forma | Ack | Custo | O que muda no banco | Veredito |
|---|---|---|---|---|---|
| **A** | **HTTP interno síncrono**: admin chama `POST /internal/v1/control` no processo do canal; `200` devolve `{delivered_to}`; o mapa canal → URL vem de **configuração do `wyd-ops`**, não de tabela | imediato, dentro de 1,5 s | um endpoint novo no `wyd-server`; nada de infra | **nada** — 34 arquivos / 109 tabelas continuam | **recomendada** |
| B | **Fila de comando em `ops`** (`ops.server_command`), dreno com `SELECT … FOR UPDATE SKIP LOCKED` como `0012`; o processo confirma por `UPDATE` | no próximo tick (assíncrono) | tabela, dreno, expurgo e reconciliação | **+1 tabela e +1 arquivo** (110 / 35) e a promessa do C4 §4.4 vira "assíncrono com prazo" | reserva para quando o processo precisar receber comando com o servidor HTTP fora |
| C | Broker externo (NATS/Redis pub-sub) | imediato | serviço novo, com portas, credencial e operação próprias | nada | recusada: o stack é Postgres + processos; free tier não paga mais uma dependência |

### 3.3 Recomendação (opção A), com o que a torna defensável

- **"Sem ack, sem escrita"** continua literal: o admin chama, espera `200`, e só então abre a transação
  que grava `released_at` e a auditoria. Processo fora do ar é `503 ADM_SERVER_UNAVAILABLE` **sem
  escrita** — o jogador não fica preso em estado meio-liberado.
- **Destino é configuração, não dado:** o mapa canal → URL pertence ao deploy (`WYD_SERVER_CONTROL_URLS`,
  por grupo/canal), porque muda com a topologia, não com o jogador. Guardá-lo em `game.server_channel`
  faria o banco de jogo carregar endereço de processo.
- **Autenticação do elo, mesmo padrão de §2:** segredo próprio (`WYD_SERVER_CONTROL_TOKEN`), distinto do
  token portal → admin — audiências diferentes não compartilham segredo. O `wyd-server` recusa comando
  sem ele; o admin nunca manda `X-Staff-Person-Id` para o processo do jogo, só o resumo da ação.
- **Broadcast idempotente pelo lado do admin:** repetir a chave não reenvia; o `200` com
  `delivered_to` é o resultado gravado no registro de idempotência (`0033`).
- **Timeout é incerto, não sucesso:** `504` deixa a sessão como estava e a reconciliação é
  `GET /actions/{key}`, nunca chave nova.

### 3.4 O que muda se o dono aprovar

`ADM-08` destrava, `RF-ADM-004`/`RF-ADM-005` ganham o elo, `componentes.md` (`acoes` → `server`) deixa de
depender de lacuna, e o plano do `wyd-server` nasce com uma obrigação nomeada: endpoint de controle,
autenticação pelo segredo, ack com contagem e erro tipado. **Nenhuma tabela, `GRANT` ou contagem muda**
na opção A; a opção B acrescenta `ops.server_command` e passa a exigir decisão no `wyd-db` (uma lacuna
nova lá, não uma tabela escrita por conta própria).

## 4. O que precisa do "sim"

| Lacuna | Aprovar | Consequência imediata |
|---|---|---|
| `LAC-ADM-04` | **opção A** (token de serviço com dois valores na rotação) — ou indicar B, C ou D | `ADM-02` destrava; `contrato-c4.md` §1 deixa de apontar lacuna; `wyd-ops` ganha as duas variáveis e o passo de rotação |
| `LAC-ADM-03` | **opção A** (HTTP interno síncrono com ack) — ou indicar B ou C | `ADM-08` destrava; o plano do `wyd-server` recebe a obrigação do endpoint; opção B abre lacuna de schema no `wyd-db` |

Aprovar B em `LAC-ADM-03` **não** autoriza escrever `ops.server_command` aqui: a tabela nasce no `wyd-db`,
com decisão registrada em `lacunas.md` de lá, como as anteriores (`M30`, `M31`).

## 5. Como se verifica depois de aceito

| Elo | Prova | Esperado |
|---|---|---|
| portal → admin | chamada sem `Authorization` | `401`, nenhuma linha em `audit_log` |
| portal → admin | token do ambiente errado | `401` |
| portal → admin | token válido com `X-Trace-Id` ausente | `400` |
| portal → admin | rotação: portal no token novo, admin aceitando os dois | `200` nos dois lados da janela |
| admin → server | processo do canal parado | `503 ADM_SERVER_UNAVAILABLE` e **nenhuma** escrita de `released_at` |
| admin → server | ack recebido | `released_at` e `released_reason = 'admin'` gravados, `audit_id` no `200` |
| admin → server | timeout | `504`; `GET /actions/{key}` devolve o desfecho real; sessão inalterada |
| admin → server | comando sem o segredo do elo | `401` no processo do jogo |

## 6. O que esta página não decide

| Assunto | Onde fecha |
|---|---|
| Papel mínimo por rota e autoria na auditoria | `RN-ADM-001`/`RN-ADM-002` e `M18-A`, já decididos |
| Formato do pacote de broadcast no fio do jogo (opcode, encode, limites) | `wyd-protocol` (2º piloto) e o plano do `wyd-server` |
| Porta, bind e topologia dos processos de canal | `wyd-server` + `wyd-ops` |
| Emissão, guarda e rotação dos segredos (cofre, `.env`, secret do runner) | `wyd-ops`; esta página só nomeia as variáveis |
| Forma de `ops.server_command`, se a opção B for escolhida | `wyd-db` (`ddl-confirmado.md` + nova lacuna em `lacunas.md`) |
