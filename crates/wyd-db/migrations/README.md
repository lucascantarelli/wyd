# Migrations (`crates/wyd-db`)

Série única, aplicada em ordem pelo nome do arquivo. Cada arquivo é um domínio e tem dois
blocos: `-- migrate:up` e `-- migrate:down`. O marcador só vale quando ocupa a linha inteira
(o cabeçalho cita os dois nomes na mesma frase e já enganou dois runners).

## Regra da sequência

**DDL primeiro, seed depois.** Um seed nunca roda antes de a tabela que ele preenche existir;
separar as fases deixa o `down` de seed (`DELETE`/`UPDATE` de volta ao estado herdado) e o de
DDL (`DROP TABLE`) em blocos previsíveis. DDL que entrou depois do bloco de seed (tabelas
decididas depois, como `0028` e `0053`) é exceção registrada: o que a regra proíbe de verdade é
seed inserindo em tabela inexistente, cobrado por `tests/contrato/catalogo.rs`.

| Faixa | Conteúdo |
|---|---|
| `0001`–`0006` | base de jogo: contas (login, senha, PIN, e-mail, papel), personagens, inventário (com refino/Anct/gema na criação), guildas, auditoria, delivery_queue |
| `0007`–`0008` | catálogo (item/skill/efeito, com `skill_catalog.notes`) e mundo (NPC, loja, região, settings) |
| `0009`–`0016` | refino, economia, rotas, combate, quests, eventos, arena, progressão por personagem |
| `0017`–`0027` | seeds: taxonomia, itens, skills, mundo, linhagem de skills, refino, mundo fixo, skills da Kersef, SancRate, composição |
| `0028`–`0037` | estado de jogo (buffs, capa, diária, clãs, GTorre), canal de servidor, sessão única, moderação, tabelas 1:1 de personagem (`character_scores`, `short_skill`, `learned_skill`, `special`, `wallet`), config editável |
| `0038`–`0043` | seed de refino de acessório, views de classificação/teto, capacidade e papel do canal, razão de crédito, piedade de refino |
| `0044`–`0052` | seed e correções de conteúdo (Campo de Treinamento, catálogo GAME-12/15, nomes de tela, sem duplicatas) |
| `0053` | token de reset da segunda senha (`pin_reset_token`); o e-mail da conta mora em `accounts` desde a criação |

Regras que o verificador cobra:

1. **Nenhum `ALTER TABLE` no `up`.** Coluna nova nasce na criação da tabela. `ALTER` corretivo
   esconde desenho errado — `0023_inventario_gema` e `0024_catalogo_skill_nota` foram
   consolidados em `0003`/`0007` (2026-11-08) e `0053_pin_reset_email` em `0001`/`0053`
   (2026-11-15). O teste do catálogo falha se um `ALTER` reaparecer.
2. **Nenhuma FK para tabela criada depois** — é o que obriga `0016_progressao` a vir depois de
   `0007`, já que referencia `skill_lineage`/`skill_catalog`.
3. **Toda tabela criada cai no `down` do próprio arquivo.**
4. **`down` de seed restaura o estado anterior, não o apaga** (ADR-0013): reverter uma migração
   de seed devolve as linhas herdadas ao valor da migração anterior (ex.: `0021` restaura as 6
   linhas da `0019`) e só zera o que ela própria classificou.
5. Numeração contígua a partir de `0001`, sem buraco.

## Verificação em três camadas

```bash
make migrations-check   # ordem/FK/down/INSERT, sem Postgres  (segundos)
make banco-catalogo     # regenera docs/arquitetura/banco-catalogo.md
make banco-verify       # banco vazio → up → compara com information_schema (Docker)
make migrations-apply   # up → down → up, contagens iguais (Docker)
```

| Camada | Comando | O que prova |
|---|---|---|
| estrutural | `node tools/content/check-migrations.mjs` | ordem, tabela duplicada, FK para tabela futura, `down` esquecido, `INSERT` órfão, contagem documentada |
| contrato | `cargo test -p wyd-db` (`tests/contrato/catalogo.rs`) | catálogo documentado == migrations; `SCHEMA_VERSION` == última; seed só depois do DDL; sem `ALTER TABLE` |
| banco real | `node tools/content/verify-schema.mjs` | `pg_attribute`/`information_schema` == catálogo (tabela, coluna, tipo, nulidade, PK), sem sobra nem falta |
| ciclo | `node tools/content/migrate-cycle.mjs` | `up` completo, `down` completo (0 tabelas), `up` de novo com contagens idênticas |

Medida de 2026-11-15 (`gen-catalogo-banco` + `check-migrations`): **53 migrations, 87 tabelas,
625 colunas, 161 `INSERT`s**. As medidas antigas deste README (24 e 29 migrations) descreviam a
série antes de `0028`–`0053` e ficam apenas como histórico no `roadmap-rust/changelog.md`.

## Lacunas declaradas (tabela criada ≠ conteúdo pronto)

| Tabela | Estado | Por quê |
|---|---|---|
| `quest`, `character_quest` | vazia | cadeia do 7662 não está fechada; seed seria invenção de quest |
| `mob_template_stat`, `mob_template_equip` | vazia | stats vêm do `STRUCT_MOB` (blob de `npc_template`); extração do struct ainda não foi feita |
| `character_status`, `character_lineage`, `character_skills` | vazia | schema pronto para o save; a fórmula de pontos está em `docs/wyd/progressao.md` e entra com a FASE-10 |
| `item_price`, `donate_shop_item`, `daily_reward_item` | vazia | preço é configuração do dono do servidor, não dado do legado |
| `item_effect_override`, `castle_quest_state`, `mob_carry_override` | fora do schema | pendências D-B05/D-B06/D-B07 em `docs/arquitetura/banco-catalogo.md` §3 |

## Contas de teste

A série não cria conta nenhuma. O banco de testes sobe **sem admin**: as contas padrão
(`teste01`..`teste04`, senha `teste1234`, PIN `1234`, um personagem por classe) vivem em
`crates/wyd-db/seeds/dev.sql` e são aplicadas uma vez, com o schema recém-criado. Staff nasce
de `wyd-gateway --seed-staff` ou do portal quando o teste exigir.

## Catálogo

`docs/arquitetura/banco-catalogo.md` é **gerado** (`make banco-catalogo`) a partir destas
migrations + `tools/content/banco-fontes.mjs`. Ele traz, por tabela, colunas/tipos/nulidade,
PK/FK/CHECK/índices, o arquivo que a cria, a fonte da decisão e o que é provisório. Não edite
à mão: rode o gerador.
