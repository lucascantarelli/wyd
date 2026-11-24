> Pergunta: "como crio dado de teste sem estado global e sem id mágico?"
> Público: Desenvolvedor e QA
> Dono: equipe do `wyd-db`
> Base: `AGENTS.md` §8; `pendencias-de-fechamento.md` §6.6.5 (`Q-BLK-05` = A, 2026-11-24); `docs/planos/plano-de-testes.md`; [`piramide-e-alvos.md`](piramide-e-alvos.md)
> Revisado em: 2026-11-24

**Estado:** proposto (2026-11-24)

Factory é código de teste que cria o agregado inteiro e **devolve a chave**; fixture é o dado fixo que o
teste não cria. A forma está definida aqui; nenhuma factory existe em `crates/`, porque a suíte ainda não tem
alvo declarado no `Cargo.toml` (o arquivo existe desde 2026-11-24, com `src/dsn.rs`).

## 1. Factory por agregado

Uma factory por agregado, não por tabela: o teste pede “uma conta com carteira”, não cinco `INSERT`
soltos.

| Agregado | Raiz | O que a factory cria | O que fica fixo |
|---|---|---|---|
| conta | `portal.person` | `person`, `game.account` e `game.account_wallet` | papel `member` |
| personagem | `game.character` | `character` e as linhas `character_*` que o teste usa | conta devolvida pela factory de conta |
| clã | `game.guild` | `guild` e `guild_member` | cargo de membro comum |
| conteúdo | `content.content_version` | versão base e as linhas `content_*` do cenário | versão marcada como fixture |
| mercado | `portal.market_listing` | anúncio e, quando o teste pede, lance e transação | estado fechado só quando o cenário exige |
| operação | `ops.delivery_queue` | linha `pending` para o dreno consumir | `attempt_count = 0` |

Agregado sem factory é agregado sem teste de sistema: quem precisar de outro (por exemplo `guild_tower`)
cria a factory junto com o primeiro teste que o exigir, não antes.

## 2. Fixture de conteúdo

- A base vem da **série** (`content_version` + `content_*`), não de cópia do legado.
- As **6 seeds presas ao cliente 7662 não são fixture**: carregam dado de origem e seguem bloqueadas
  (`pendencias-de-fechamento.md` §4.2). Teste que depende delas não roda hoje.
- Override é fixture de cenário: `content.*_override` entra com `COALESCE(override, base)` no loader, e o
  teste que cria override **apaga o override**, não a base.
- Nenhuma fixture depende da ordem de aplicação: se o teste precisa de versão, ele cria a sua.

## 3. Regra de id

- A factory **devolve** o identificador; teste não escreve literal numérico.
- Chave natural só quando o DDL manda: `portal.person` por e-mail único, `person_token` por
  `(person, purpose)`, `content_version` por versão.
- Proibido: sequência fixa, “id 1”, `lastval()` reaproveitado entre arquivos, chave copiada de exemplo.
- Dois registros iguais no mesmo teste recebem sufixo explícito da factory — nunca `sleep` para “esperar”
  o segundo ficar visível.

## 4. Limpeza por chave

- O teste guarda a **chave do agregado raiz** que criou e limpa a partir dela.
- A ordem de exclusão é a inversa da FK; quando o teste é de contrato ou de sistema, a limpeza roda na
  mesma transação do cenário.
- A limpeza usa `WYD_DB_URL_TEST` (`wyd_migrator`, banco descartável — [`testes-de-sistema.md`](testes-de-sistema.md), seção “DSN de teste”).
  Os papéis de runtime **não recebem `DELETE`**: `0032` nega ao `wyd_admin`, e `market_ledger` tem
  `UPDATE`/`DELETE` revogados. Limpar com DSN de runtime não funciona — e é bom que não funcione.
- Limpeza que falha não vira “banco sujo”: o banco é descartável e recriado. Recriar é mais barato que
  caçar resíduo.

## 5. Proibição de estado global

- Nenhum teste depende da ordem dos outros, de dado deixado por execução anterior ou de fixture
  compartilhada entre arquivos.
- Sem `sleep` como sincronização: espera por condição (`SELECT` até o estado) com limite explícito.
- Paralelismo não pode estourar o pool do banco de teste; o limite é declarado no alvo, não herdado do
  default do driver (o número de pool é `Q-BLK-06`).
- Teste que precisa de duas contas cria duas; não reusa a conta de outro arquivo.

## 6. O que esta página não decide

- **Quais contas e qual seed existem:** é [`dados-de-teste.md`](dados-de-teste.md).
- **Quantos testes, cobertura e regra de `SKIP`:** é [`gates-e-anti-skip.md`](gates-e-anti-skip.md).
- **Driver, pool e timeout:** `Q-BLK-06`; a página não fixa número.
- **Ferramenta de factory:** o crate nasce na onda de código; aqui está a forma, não a API.
