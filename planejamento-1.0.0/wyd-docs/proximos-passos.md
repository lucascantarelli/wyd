> Pergunta: "qual é o próximo passo do site, em ordem de desbloqueio ÷ custo?"
> Público: quem mantém o `wyd-docs`
> Dono: equipe do wyd-docs
> Base: `lacunas.md`; `wyd-docs-1.0.0.md` §7.2 e §9; `roadmap-fase-3-para-4.md` §2
> Revisado em: 2026-11-20

**Estado:** proposto (2026-11-19)

# wyd-docs — próximos passos

Esta é a **fonte única da fila** do site. O critério de saída da fase 4 é do
[roadmap-fase-3-para-4.md](roadmap-fase-3-para-4.md) §2, a ordem de produção é de
`wyd-docs-1.0.0.md` §9 e o estado de cada lacuna é de `lacunas.md` §1; nenhum dos três é
recontado aqui.

O total e o censo são de `wyd-docs-1.0.0.md` §7.2: **34 páginas, 12 `proposto` · 16 sem linha
de estado · 6 `aceito`** (2026-11-19). Item concluído não entra nesta fila: o que fechou em
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
| 1 | **97 páginas por tabela do `wyd-db`** — produção a decidir na lacuna `M26` (`planejamento-1.0.0/wyd-db/lacunas.md` §2.3); o gerador `wyd-db-pages` deixou de ser presumido e só existiria com aprovação explícita | arquitetura + documentação | bloqueado por decisão, não por projeto | nenhuma ferramenta é proposta por esta fila; as opções (à mão em lotes, gerador, adiar ou cancelar) estão em `M26` | decisão registrada e, se houver páginas, `gerado/index.md` + gate verde |
| 2 | **Próxima rodada de decisões** — `A21` (troca de e-mail) e `P3` (`planos/riscos.md`) | usuário (produto + arquitetura) | rápido — 1 resposta por ID | **nenhuma resposta inventada**; são as duas de maior desbloqueio entre as que ainda esperam palavra do dono — as demais de §5.3 são de outro dono ou de onda posterior | respostas registradas e linhas fechadas |
| 3 | **Frente de captura do cliente 7662** — medir `LPRT-03` (origem da `KeyWord`), `LPRT-06` (encoding/terminador) e `LPRT-09` (build-alvo); detalhe do que medir em §6 | frente `wyd-client` + pesquisa | médio — sessão de captura | **não é decisão**: a parte decidível já foi decidida em 2026-11-20 e o que resta é byte medido; sem captura a linha continua `aberta`, sem valor suposto | captura datada e anonimizada registrada, com o medido e o não medido escritos |

A linha 1 depende da decisão `M26` do `wyd-db` e não autoriza ferramenta nenhuma; a 2 não é lacuna deste site, mas reúne
o que ainda bloqueia o sistema vizinho que o site publica. A 3 é a única frente do protocolo que não
espera resposta do usuário: as três `LPRT` que sobraram não são perguntas em aberto, são medições que
ninguém fez ainda — o que medir está em §6, e nenhum resultado está registrado.

## 2. Pré-commit

| O que | Onde | Por que agora |
|---|---|---|
| Rodar o gate antes do próximo PR que alterar documentação publicável | `wyd-docs/` (comando de `wyd-docs-1.0.0.md` §7.1) | evita introduzir página quebrada em uma mudança futura |
| Conferir o total e o censo medidos | `wyd-docs-1.0.0.md` §7.2 | número que não bate com a montagem é defeito, não estimativa |

## 3. Recontar o censo (um comando)

```bash
for f in $(find wyd-docs/.build/docs -name '*.md'); do
  if estado=$(grep -m1 -oE '^\*\*Estado:\*\* *[^ ]+' "$f"); then
    printf '%s\n' "$estado" | sed -E 's/^\*\*Estado:\*\* *([^ ]+).*/\1/'
  else
    echo SEM
  fi
done | sort | uniq -c
```

O resultado não se copia: total e censo são de `wyd-docs-1.0.0.md` §7.2, o único dono do
número. Este comando reproduz o censo a qualquer momento, sem guardar cópia do resultado.

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
| 4 | **Propagar as decisões no checklist** — remover bloqueios que já fecharam e corrigir estados | `planejamento-1.0.0/wyd-db/migrations-checklist.md` §2/§3/§4 | `A17`, `M16`, `A19`, `A14` e `M19` deixam de aparecer como bloqueio; `0005` fica `pronta` com 5 tabelas; `0011` fica 6 em `portal` + 4 em `game`; a soma permanece **2 `concluída` + 3 `pronta` + 1 `parcial` + 23 `bloqueada` = 29** | É edição de estado, não de DDL: os valores já estão em `lacunas.md` e `ddl-confirmado.md`; não muda a coluna `Tab.` nem a contagem de 92 | Remove falso bloqueio de `0005`/`0011` e evita que a próxima página seja escrita contra um checklist mentiroso |
| 5 | **Fechar a aritmética do DDL no plano** — corrigir o resumo de arquivos fechados | `planejamento-1.0.0/wyd-db/plano-migrations.md` §3 | O resumo passa a dizer **5 arquivos 100%** (`0001`, `0003`, `0004`, `0005`, `0007` = 13 tabelas) + **2 tabelas** de `0002` = **15 de 92**; hoje a frase lista 4 arquivos e “mais 6” e não fecha a conta | Uma frase; o DDL e os números já estão fechados, falta só a soma explícita | Torna o número **15/92** auditável na página que é dona da contagem e evita repetir “14 de 92” |
| 6 | **Fechar a narrativa de `0005`** — retirar “o arquivo não fecha” e a tabela de decisões pendentes | `planejamento-1.0.0/wyd-db/ddl-confirmado.md` §5.2 | `0005` passa a ser lido como 5 tabelas fechadas; a tabela `A17`/`M16` deixa de dizer que o bloco ainda pode mudar; nenhum DDL muda | A decisão já está tomada e o DDL já está no mesmo §5.2; é remover a contradição local | Alinha o documento de DDL ao estado `pronta` que o checklist e o plano já usam |
| 7 | **Limpar as duas referências históricas** — `A10` sem motivo e `M16`/`0005` parcial | `planejamento-1.0.0/wyd-db/lacunas.md` §1 (linha histórica de `A10`) e §2.3 (nota de `M16`) | A linha de `A10` deixa de dizer “sem `released_reason`”; a nota de `M16` deixa de dizer que `0005` é `parcial`; a decisão revisada e a tabela de 5 tabelas passam a ser a única leitura | Duas correções pontuais, sem tocar em DDL nem em contagem | Fecha a consistência `A10`/`A14` e evita que uma página nova copie o estado antigo |
| 8 | **Política de retenção em página própria** — materializar `M22` | `planejamento-1.0.0/wyd-db/docs/arquitetura/retencao-e-capacidade.md` (novo), com correção das três referências que ainda dizem `M22` aberta: `docs/arquitetura/integridade-e-constraints.md`, `docs/dicionario-dados/dominio-acesso.md` e `docs/guia-desenvolvedor/como-adicionar-tabela.md` | Política operacional explícita: `portal.person_session` 30 dias após `expires_at`; `game.session` 365 dias após `released_at`; `ops.audit_log` 730 dias, append-only. Não cria tabela, coluna ou índice; os índices de reaper já estão em `ddl-confirmado.md` §4/§5.5 | Os três prazos já foram decididos em `M22`; a página só reúne a decisão, os índices existentes e o efeito de backup/purge | Destrava a rotina de operação e backup e remove a última leitura de `M22` como decisão aberta |

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

### 5.3 Frentes bloqueadas por decisão do usuário

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
fechou junto com `LPRT-01`. Restam abertas `LPRT-03`, `LPRT-06` e `LPRT-09`, que não estão aqui
porque não são decisões pendentes: são medições do cliente, tratadas na frente de captura de §6.
Nenhuma delas muda o schema do `wyd-db`.

### 5.4 Contagens que este anexo usa

| Medida | Valor | Dono |
|---|---:|---|
| Arquivos da série / tabelas planejadas / seeds | **29 / 92 / 6** | `plano-migrations.md` §3–§4 |
| Tabelas com DDL fechado | **15 de 92** | `ddl-confirmado.md` §1 |
| Arquivos com DDL 100% | **5** (`0001`, `0003`, `0004`, `0005`, `0007`) | `ddl-confirmado.md` §1 |
| Páginas de migration | **2 `concluída` + 3 `pronta` + 1 `parcial` + 23 `bloqueada` = 29** | `migrations-checklist.md` §2/§4 |
| Lacunas `B`/`A`/`M` | **24 fechadas, 2 abertas (`A7`, `A21`), 3 externas (`A11`, `A20`, `M21`)** | `lacunas.md` §1 |
| Divergências `P` / achados `D` | **10 fechadas + 1 aberta (`P3`)** / **3 fechadas** | `lacunas.md` §1 |
| Tabelas das três páginas `pronta` | **3** (`0003`) + **5** (`0005`) + **2** (`0007`) = **10** | `plano-migrations.md` §3 |
| Páginas `concluída` se as frentes 1–3 forem escritas | **2 → 5**; o DDL continua **15/92** | `migrations-checklist.md` §2/§4 |

O número de tabelas não sobe ao escrever página: página é documentação; **15/92** é DDL já desenhado,
e **92** é o planejado da série. Nenhum dos dois é contagem de catálogo, porque nenhum Postgres rodou
nesta rodada.

---

## 6. Frente de captura — o que medir (não é rodada de decisão)

> **Seção própria, fora do anexo de §5.** **Nenhuma linha desta seção registra resultado.**
> `LPRT-03`, `LPRT-06` e `LPRT-09` continuam `aberta` em `lacunas.md`, que é o dono do estado. O que
> esta seção fixa é **o que uma captura precisa mostrar** para fechar cada uma, para que a próxima
> sessão não rediscuta a pergunta nem aceite um “parece que é”.

As três não esperam palavra do usuário: em `LPRT-03` e `LPRT-06` a parte decidível já foi decidida em
2026-11-20, e o que resta é medir o que o cliente faz no fio. Uma única sessão de captura do cliente
7662 pode responder as três, e nenhuma bloqueia as outras. Dono da execução: frente `wyd-client` +
pesquisa; o usuário fornece o cliente e o ambiente, não uma resposta. O processo de captura é o de
`LPRT-10` — datada, anonimizada, `G-<familia>-<nn>` em `tests/golden/`, origem registrada e segunda
leitura antes de aceitar. O procedimento operacional das três — o que capturar, como anonimizar e a
ficha de origem — está em `planejamento-1.0.0/wyd-protocol/docs/goldens/como-capturar.md`.

| ID | O que já está decidido (não reabrir) | O que a captura precisa medir | Fecha quando | Enquanto não houver captura |
|---|---|---|---|---|
| `LPRT-03` | o crate **recebe** a `KeyWord` do chamador e não assume origem; escolher quem gera não é pergunta do crate | no handshake: se o cliente envia a semente ou só a recebe; qual dos 16 pares de `KEY_TABLE` aparece; em que campo/offset do frame ela viaja; se viaja antes ou depois da ofuscação | captura datada do handshake + segunda leitura, com o offset e o campo escritos em `keyword-e-checksum.md` | a origem da `KeyWord` fica `H`; `header.md`, `enquadramento.md` e `keyword-e-checksum.md` sem afirmação de byte |
| `LPRT-06` | o codec **não transcodifica**; string é bytes de tamanho fixo declarado | num frame com texto acentuado (nome e linha de chat): a sequência exata de bytes; a qual encoding ela corresponde, comparada contra candidatos sem presumir; se há terminador e de que largura; se o tamanho declarado conta o terminador; o que acontece quando o texto preenche o campo | captura com o mesmo texto visível na tela e os bytes no frame, datada e anonimizada | `payloads/*` sem afirmação de encoding; contrato de nome e chat sem fechar |
| `LPRT-09` | nada decidido além do alvo **provável** 7662, que não é confirmação | a build que o cliente reporta (welcome/handshake ou o próprio binário); se `APP_VERSION = 7640` do legado bate ou diverge; o que muda entre as duas builds que toque header ou opcode | captura ou leitura datada que identifique a build, com o arquivo de origem registrado | catálogo e versionamento sem afirmação de versão; `wire/opcodes/*` e `versionamento-e-compatibilidade.md` seguem bloqueados |

Uma captura parcial é resultado válido e não vira decisão pela metade: registra-se o que ela respondeu
e o que não respondeu, e a lacuna que ficou sem resposta continua `aberta`. Se a captura mostrar algo
diferente do decidido em 2026-11-20, o caminho não é ajustar a página: a `LPRT` reabre em `lacunas.md`
com a evidência nova e a decisão é revista.
