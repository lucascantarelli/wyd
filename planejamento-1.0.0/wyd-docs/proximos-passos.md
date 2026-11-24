> Pergunta: "qual é o próximo passo do site, em ordem de desbloqueio ÷ custo?"
> Público: quem mantém o `wyd-docs`
> Dono: equipe do wyd-docs
> Base: `lacunas.md`; `wyd-docs-1.0.0.md` §9.1; `roadmap-fase-3-para-4.md` §2
> Revisado em: 2026-11-23

**Estado:** proposto (2026-11-19)

# wyd-docs — próximos passos

Esta é a **fonte única da fila** do site. O critério de saída da fase 4 é do
[roadmap-fase-3-para-4.md](roadmap-fase-3-para-4.md) §2, a ordem de produção é de
`wyd-docs-1.0.0.md` §9 e o estado de cada lacuna é de `lacunas.md` §1; nenhum dos três é
recontado aqui. As decisões dos sistemas que o site publica estão ordenadas no
`../briefing-de-decisoes-1.0.0.md` (2026-11-23); esta fila não as repete.

O total e o censo são de `wyd-docs-1.0.0.md` §9.1: **267 páginas** e **15 `aceito` · 245
`proposto` · 4 sem linha de estado** (2026-11-22). Item concluído não entra nesta fila: o que fechou em
2026-11-19 está em `lacunas.md` §1, `wyd-docs-1.0.0.md` §9/§10 e `docs/planos/metricas.md`.

A fila é ordenada por **ganho de desbloqueio × custo**, não por facilidade. **Dono** diz quem
age: `usuário` espera uma palavra, `agente` executa sem decisão. **Tamanho** é `rápido` (uma
sessão curta) ou `médio` (decisão + implementação). Onde depende do usuário, a recomendação já
vem escrita — basta aceitar ou recusar.

**Estado do pacote anterior (`V`, 2026-11-19):** PR #48 foi mesclado por squash em `e08f7d1`; sua
publicação e CI não são mais pendências. A reconciliação posterior da `LPDOC-09` de 91 para 92
páginas não pertence àquele PR e permanece nas alterações locais atuais.

## 1. Fila priorizada

| # | Item e o que destrava | Dono | Tamanho | Recomendação e impacto | Pronto quando |
|---:|---|---|---|---|---|
| 1 | **Nenhuma página por tabela pendente** — `M26` fechou em 2026-11-21; as **109 de 109** páginas foram escritas à mão, sem gerador | — | concluída | frente concluída; não há decisão nem ferramenta a autorizar | 109 páginas existentes e gate verde |
| 2 | **Nenhuma rodada de decisões pendente** — `A21` (troca de e-mail) e `P3` (`planos/riscos.md`) fecharam em 2026-11-20; a fila de decisões é do `briefing`, e não tem resposta pendente | — | concluída | frente concluída; a implementação seguinte é A1 (captura), autorizada em 2026-11-23 no `briefing` | — |
| 3 | **Frente de captura do cliente 7662** — medir `LPRT-03` (origem da `KeyWord`) e `LPRT-06` (encoding/terminador), registrando a build de `LPRT-09` (alvo decidido em 2026-11-23); **A1 autorizada em 2026-11-23**, implementação e captura pendentes | frente `wyd-client` + pesquisa | médio — sessão de captura | **não é decisão**: a parte decidível já foi decidida (`LPRT-03`/`LPRT-06` em 2026-11-20; `LPRT-09` em 2026-11-23; A1 em 2026-11-23) e o que resta é byte medido; sem captura a linha continua `aberta`, sem valor suposto | captura datada e anonimizada registrada, com o medido e o não medido escritos |

As linhas 1 e 2 estão concluídas: `M26`, `A21` e `P3` fecharam no `wyd-db`, e nenhuma delas espera
resposta. A 3 é a única frente do protocolo que não espera resposta do usuário: as duas `LPRT` que
sobraram não são perguntas em aberto, são medições que ninguém fez ainda — o que medir está em §6, e
nenhum resultado está registrado.

## 2. Pré-commit

| O que | Onde | Por que agora |
|---|---|---|
| Rodar o gate antes do próximo PR que alterar documentação publicável | `wyd-docs/` (comando de `wyd-docs-1.0.0.md` §7.1) | evita introduzir página quebrada em uma mudança futura |
| Conferir o total e o censo medidos | comando de §3; valor em `wyd-docs-1.0.0.md` §9.1 | número que não bate com a montagem é defeito, não estimativa |

## 3. Censo por estado (um comando, rodado pelo CI)

```bash
cd wyd-docs && ./.venv/Scripts/python.exe verificar-contagens.py
# Linux/macOS: cd wyd-docs && .venv/bin/python verificar-contagens.py
```

O comando lê as fontes de sistema que a montagem publica (`planejamento-1.0.0/<sistema>/docs/**/*.md`,
a mesma regra do `preparar.py`) e conta `aceito`, `proposto` e sem linha de estado, mais o total de
fontes censadas. Ele compara as quatro grandezas com `wyd-docs-1.0.0.md` §9.1 — o único dono do
número — e sai com `exit 1` quando qualquer uma diverge; `exit 2` é leitura impossível (arquivo,
pasta ou declaração ausente), não divergência. Fonte publicada com estado fora de `aceito`/`proposto`
reprova, salvo exceção declarada no próprio script (`CENSO_ESPECIAL`), que ele imprime com o motivo.

O resultado não se copia para cá: total e censo continuam só em §9.1, e este comando é como o número
se reproduz e se confere, sem leitura manual do artefato.

Quem impõe o número não é quem roda o comando à mão: o `docs-check` do CI
(`.github/actions/docs-check/`) roda `verificar-contagens.py` e `verificar-censos.py` logo depois do
`verificar.py`, em todo
PR e push que o workflow `docs` cobre — o filtro é a árvore `planejamento-1.0.0/**` mais
`wyd-docs/**`, sem lista de arquivos, então painel, `lacunas.md`, `migrations/` e página de sistema
disparam o mesmo gate.
Divergência de §9.1 derruba o check como um link quebrado; o comando local é a mesma medição,
antecipada.

## 4. O que esta página não decide

- Não decide lacuna: `lacunas.md` continua dono de estado, dono e evidência.
- Não decide fase nem critério de saída: isso é do
  [roadmap-fase-3-para-4.md](roadmap-fase-3-para-4.md).
- Não autoriza commit, push, merge nem mudança de ruleset.


## 5. Anexo — brainstorm do `wyd-db` (2026-11-19)

> **Escopo:** este anexo não altera a fila do site nem o dono de nenhuma lacuna. Ele lê o estado dos
> documentos do `wyd-db` e ordena o que já pode ser executado sem nova resposta do usuário. Estado,
> dono e evidência continuam em `lacunas.md`; DDL, em `ddl-confirmado.md`; ordem e contagem, em
> `plano-migrations.md` §3. Nenhum `.sql` foi escrito e nenhum Postgres rodou.

### 5.1 Frentes executáveis agora

Ordenadas por **retorno sobre esforço**: primeiro o que fecha mais tabelas ou remove mais bloqueio
com menos decisão nova.

| # | Frente | Artefato exato | Efeito técnico real | Por que é rápida | O que destrava |
|---:|---|---|---|---|---|
| 1 | **Página da identidade** — escrever a migration `0003` a partir do DDL já fechado | `planejamento-1.0.0/wyd-db/migrations/0003-portal-person.md` (hoje ausente; só existem `0001` e `0004`) | 3 tabelas passam de `planejado` a DDL executável: `portal.person`, `portal.person_profile`, `portal.person_consent`; entram `ux_person_email`, `ix_person_nao_ativas`, `ix_person_suspended_by`, FKs, CHECKs e `trg_person_set_updated_at` | O DDL inteiro já está em `ddl-confirmado.md` §5.1/§5.6/§5.7; a página só aplica o molde de 10 itens de `migrations-checklist.md` §5, sem modelagem nova | Fecha a primeira das três páginas `pronta`; `0005` depende de `portal.person`, então esta é a raiz do corte vertical `0003`–`0007` |
| 2 | **Página da conta e do ledger** — escrever `0005` com as 5 tabelas fechadas | `planejamento-1.0.0/wyd-db/migrations/0005-game-account.md` | 5 tabelas executáveis: `game.account`, `account_wallet`, `account_ledger` (`M16`), `account_moderation`, `pin_reset`; com `ux_account_login`, `ux_account_slot`, `ix_account_ledger_conta`, trigger e CHECKs de saldo/motivo | É o maior bloco já fechado (5 tabelas) e o DDL está literal em `ddl-confirmado.md` §5.2; `A17` não acrescentou objeto e `M16` já acrescentou a quinta tabela | Fecha `0005` e o domínio conta; libera `0006` (FK `account`) e o caminho de carteira/ledger do corte vertical |
| 3 | **Página da sessão** — escrever `0007` com o bloco `server_channel` + `session` | `planejamento-1.0.0/wyd-db/migrations/0007-game-session.md` | 2 tabelas executáveis; `game.session.released_reason` e `ck_session_motivo` tornam `A10`/`A14` verificáveis no DDL; `server_channel` leva `max_players`, `maintenance` e `role` (`M23`) | Só 2 tabelas e o DDL está fechado em `ddl-confirmado.md` §5.5; a página pode ser escrita agora, mesmo que a aplicação espere `0006` pela FK de `character` | Fecha a terceira página `pronta`; destrava a leitura de sessão/reaper e o teste de sessão viva do corte vertical |
| 4 | **Propagar as decisões no checklist** — frente concluída; usar os estados vigentes | `planejamento-1.0.0/wyd-db/migrations-checklist.md` §2/§3/§4 | Os bloqueios resolvidos e o estado de cada página estão registrados no checklist, dono do acompanhamento; esta frente não altera DDL | Conferência contra `lacunas.md` e `ddl-confirmado.md` | Evita reintroduzir estados históricos já superados |
| 5 | **Fechar a aritmética do DDL no plano** — conferir a fonte única | `planejamento-1.0.0/wyd-db/plano-migrations.md` §3; `migrations-checklist.md` §4 | Plano e checklist são os donos do escopo e do estado; consultar essas fontes, sem reproduzir aqui suas contagens | Conferência de consistência; nenhum DDL muda | Evita manter ou propagar somas históricas neste anexo |
| 6 | **Fechar a narrativa de `0005`** — retirar “o arquivo não fecha” e a tabela de decisões pendentes | `planejamento-1.0.0/wyd-db/ddl-confirmado.md` §5.2 | `0005` passa a ser lido como 5 tabelas fechadas; a tabela `A17`/`M16` deixa de dizer que o bloco ainda pode mudar; nenhum DDL muda | A decisão já está tomada e o DDL já está no mesmo §5.2; é remover a contradição local | Alinha o documento de DDL ao estado `pronta` que o checklist e o plano já usam |
| 7 | **Limpar as duas referências históricas** — `A10` sem motivo e `M16`/`0005` parcial | `planejamento-1.0.0/wyd-db/lacunas.md` §1 (linha histórica de `A10`) e §2.3 (nota de `M16`) | A linha de `A10` deixa de dizer “sem `released_reason`”; a nota de `M16` deixa de dizer que `0005` é `parcial`; a decisão revisada e a tabela de 5 tabelas passam a ser a única leitura | Duas correções pontuais, sem tocar em DDL nem em contagem | Fecha a consistência `A10`/`A14` e evita que uma página nova copie o estado antigo |
| 8 | **Política de retenção em página própria** — materializar `M22` | `planejamento-1.0.0/wyd-db/docs/arquitetura/retencao-e-capacidade.md` (novo), com as três referências a `M22` já corrigidas: `docs/arquitetura/integridade-e-constraints.md`, `docs/dicionario-dados/dominio-acesso.md` e `docs/guia-desenvolvedor/como-adicionar-tabela.md` | Política operacional explícita: `portal.person_session` 30 dias após `expires_at`; `game.session` 365 dias após `released_at`; `ops.audit_log` 730 dias, append-only. Não cria tabela, coluna ou índice; os índices de reaper já estão em `ddl-confirmado.md` §4/§5.5 | Os três prazos já foram decididos em `M22`; a página só reúne a decisão, os índices existentes e o efeito de backup/purge | Destrava a rotina de operação e backup; a leitura de `M22` como decisão aberta já foi removida |

### 5.2 Recomendação de primeira execução

**Primeira execução: frente 1, a página `0003-portal-person.md`.** É a menor das três páginas
`pronta` (3 tabelas), não depende de `0006` e é a raiz de `0005`: sem `portal.person`, a conta não
tem FK para onde apontar. Ela reduz o caminho crítico do corte vertical e pode ser escrita inteira
com o DDL de §5.1/§5.6/§5.7, sem uma única resposta nova.

Na mesma sessão, as frentes 2 e 3 podem ser escritas em paralelo, porque o DDL delas também está
fechado; a ordem de **aplicação** continua `0003` → `0005` → `0007` (e `0007` só roda depois de
`0006`). As frentes 4–7 são correções de estado de uma sessão curta e devem entrar antes de a
próxima página ser aberta, para não propagar bloqueio falso. A frente 8 é a única de escopo maior:
entrega uma página operacional nova, sem alterar schema.

### 5.3 Frentes que esperaram decisão do usuário (histórico)

**Nenhuma pendente:** `A21` e `P3` fecharam em 2026-11-20 (`wyd-db/lacunas.md` §2–§3); a lista abaixo é
histórico, não fila.

Nenhuma resposta abaixo foi inventada. Cada linha só registra **o que a resposta destrava**.

| ID | Pergunta ainda aberta | O que a resposta destrava |
|---|---|---|
| `A21` | A troca de e-mail entra na `1.0.0`? Quem pode pedir, como o endereço novo é verificado, o que o antigo recebe e se a troca encerra sessões? | Rota de troca e emissor do token `email_change` na frente `wyd-portal`, onda 6; o `purpose` já existe no CHECK de `0004`, então não há DDL nova a decidir aqui |
| `P3` | Qual é o registro de riscos do `wyd-db` e quem o mantém? | Criação de `planos/riscos.md` e fim do defeito sem dono; sem ele, a revisão do plano diretor não tem onde registrar risco |
| `A7` | Cadastro de pessoa: autocadastro ou criado por staff? `pending` exige clique no e-mail? | Rota de cadastro do portal e teste “cadastro cria pessoa + conta”, onda 6; é a lacuna aberta do `wyd-db`, sem DDL nova |
| `A11` | Qual é o charset real do login do cliente 7662? | Fecha o contrato de `GameLogin` na onda 5; é `externa`, dona `wyd-server`, e não bloqueia schema |
| `A20` | MFA de staff: quando e para quem o segundo fator existe? | Telas e rotas de staff do portal, onda 6; é `externa`, dona `wyd-portal` |
| `M21` | Quais telas do portal existem (cadastro, recuperação, lista de contas)? | Rotas do portal, onda 6; é `externa`, dona `wyd-portal` |

`LPRT-01`, `LPRT-18`, `LPRT-19` e as demais decididas em 2026-11-20 saíram desta lista; `LPRT-20`
fechou junto com `LPRT-01`. Restam abertas `LPRT-03` e `LPRT-06`, que não estão aqui porque não são
decisões pendentes: são medições do cliente, tratadas na frente de captura de §6. `LPRT-09` fechou em
2026-11-23 (`Q-PRT-1`: alvo 7662 + SHA-256).
Nenhuma delas muda o schema do `wyd-db`.

### 5.4 Contagens que este anexo usa

| Medida | Valor | Dono |
|---|---:|---|
| Arquivos da série / tabelas planejadas / seeds | consultar `plano-migrations.md` §3–§4; resumo em `wyd-1.0.0.md` §1 |
| Tabelas com DDL fechado | consultar `ddl-confirmado.md` §1 e `migrations-checklist.md` §4; resumo em `wyd-1.0.0.md` §1 |
| Estado das páginas de migration | consultar `migrations-checklist.md` §2/§4; resumo em `wyd-1.0.0.md` §1 |
| Lacunas `B`/`A`/`M` | **35 fechadas, 0 proposta, 0 aberta, 3 externas (`A11`, `A20`, `M21`)** | `lacunas.md` §1 |
| Divergências `P` / achados `D` | **11 fechadas** / **3 fechadas** | `lacunas.md` §1 |
| Estado atual das páginas | consultar `migrations-checklist.md` §2/§4 |

O número de tabelas não sobe ao escrever página: página é documentação; as **109 de 109** tabelas têm
DDL fechado, e **109** é o total planejado da série, não uma contagem medida do catálogo. Nenhum Postgres
rodou nesta rodada.

---

## 6. Frente de captura — o que medir (não é rodada de decisão)

> **Seção própria, fora do anexo de §5.** **Nenhuma linha desta seção registra resultado.**
> `LPRT-03` e `LPRT-06` continuam `aberta` em `lacunas.md`, que é o dono do estado; `LPRT-09` fechou por
> decisão em 2026-11-23. O que esta seção fixa é **o que uma captura precisa mostrar** para fechar cada
> uma, para que a próxima
> sessão não rediscuta a pergunta nem aceite um “parece que é”.

As duas não esperam palavra do usuário: em `LPRT-03` e `LPRT-06` a parte decidível já foi decidida em
2026-11-20, e o que resta é medir o que o cliente faz no fio. Uma única sessão de captura do cliente
7662 pode responder as duas, e nenhuma bloqueia a outra. Dono da execução: frente `wyd-client` +
pesquisa; o usuário fornece o cliente e o ambiente, não uma resposta. O processo de captura é o de
`LPRT-10` — datada, anonimizada, `G-<familia>-<nn>` em `tests/golden/`, origem registrada e segunda
leitura antes de aceitar. O procedimento operacional das duas — o que capturar, como anonimizar e a
ficha de origem — está em `planejamento-1.0.0/wyd-protocol/docs/goldens/como-capturar.md`.

| ID | O que já está decidido (não reabrir) | O que a captura precisa medir | Fecha quando | Enquanto não houver captura |
|---|---|---|---|---|
| `LPRT-03` | o crate **recebe** a `KeyWord` do chamador e não assume origem; escolher quem gera não é pergunta do crate | no handshake: se o cliente envia a semente ou só a recebe; qual dos 16 pares de `KEY_TABLE` aparece; em que campo/offset do frame ela viaja; se viaja antes ou depois da ofuscação | captura datada do handshake + segunda leitura, com o offset e o campo escritos em `keyword-e-checksum.md` | a origem da `KeyWord` fica `H`; `header.md`, `enquadramento.md` e `keyword-e-checksum.md` sem afirmação de byte |
| `LPRT-06` | o codec **não transcodifica**; string é bytes de tamanho fixo declarado | num frame com texto acentuado (nome e linha de chat): a sequência exata de bytes; a qual encoding ela corresponde, comparada contra candidatos sem presumir; se há terminador e de que largura; se o tamanho declarado conta o terminador; o que acontece quando o texto preenche o campo | captura com o mesmo texto visível na tela e os bytes no frame, datada e anonimizada | `payloads/*` sem afirmação de encoding; contrato de nome e chat sem fechar |
| `LPRT-09` | alvo **decidido** em 2026-11-23: cliente 7662 com versão e SHA-256 completos | a versão que o cliente reporta (welcome/handshake ou o próprio binário); se `APP_VERSION = 7640` do legado bate ou diverge; o que muda entre as builds que toque header ou opcode | leitura datada que confirme a versão declarada, com o arquivo de origem registrado | catálogo e versionamento citam a decisão de alvo, sem afirmar versão confirmada |

Uma captura parcial é resultado válido e não vira decisão pela metade: registra-se o que ela respondeu
e o que não respondeu, e a lacuna que ficou sem resposta continua `aberta`. Se a captura mostrar algo
diferente do decidido em 2026-11-20, o caminho não é ajustar a página: a `LPRT` reabre em `lacunas.md`
com a evidência nova e a decisão é revista.
