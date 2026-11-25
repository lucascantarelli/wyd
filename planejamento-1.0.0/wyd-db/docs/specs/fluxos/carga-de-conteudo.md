> Pergunta: "como o jogo lê o catálogo no boot e como a versão de conteúdo limita o que ele enxerga?"
> Público: Desenvolvedor do servidor, arquiteto e revisor de seed
> Dono: equipe do `wyd-db`
> Base: `plano-migrations.md` §4; `ddl-confirmado.md` §3 e §5.17–§5.28; `wyd-db-1.0.0.md` §6.16
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

O conteúdo é **frio e versionado**: migrations e seeds escrevem `content.*`; o jogo lê um corte no
boot. O corte é a linha de `content_version` criada por `0001-bootstrap.sql` e preenchida pelos seeds.
Esta página cobre a leitura inicial e a falha por versão; recarga a quente e ETL da base legada não
fazem parte do desenho.

## 1. Atores e gatilho

| Ator | Gatilho | Resultado |
|---|---|---|
| `wyd_migrator` | aplica `0001` e os seeds de conteúdo | `content_version` e catálogo disponíveis |
| `wyd-server` | boot do processo | lê a versão e o catálogo em uma transação de leitura |
| operação | troca de série ou seed | nova versão; o próximo boot lê o corte novo |
| jogador | login/seleção | usa o catálogo já carregado; não escreve conteúdo |

## 2. Sequência de boot

```mermaid
sequenceDiagram
    participant S as wyd-server
    participant R as repo/content.rs
    participant D as PostgreSQL
    S->>R: carregar catálogo
    R->>D: SELECT version, applied_at FROM content.content_version
    alt versão presente
        R->>D: SELECT ... FROM content.* WHERE version = ?
        D-->>R: itens, skills, mapas, regras, quests
        R-->>S: catálogo imutável em memória
    else versão ausente ou múltipla
        R-->>S: erro de boot; servidor não atende
    end
```

O nome `repo/content.rs` é a página de módulo planejada em `wyd-db-1.0.0.md` §6.16; o arquivo ainda
não existe. A consulta concreta pertence ao contrato dessa página, não a este fluxo.

## 3. Passo a passo e dado que muda

1. A série aplica `0001` e cria `content.content_version`; o seed de cada dataset grava a versão e as
   linhas do catálogo.
2. No boot, o servidor pede ao repositório o corte vigente — uma versão, não “a última linha”.
3. O repositório valida que a versão é única e que os datasets esperados existem; ausência é erro de
   boot, não catálogo vazio.
4. O servidor materializa o catálogo em memória e não escreve em `content.*`.
5. Um novo seed ou uma nova série cria outra versão; o processo em execução continua no corte que
   carregou até reiniciar. Não há promessa de recarga a quente.

## 4. Transação e fronteira

| Operação | Transação | Por quê |
|---|---|---|
| aplicação de seed | uma transação por arquivo, com o ledger | conteúdo e versão não podem ficar pela metade |
| leitura de boot | uma transação de leitura (snapshot) | todas as tabelas vêm do mesmo corte |
| recarga | não desenhada | não há contrato de troca atômica em processo vivo |

O servidor lê `content` com o papel `wyd_game`; ele não tem `INSERT`/`UPDATE` nesse schema. A escrita é
do `wyd_migrator` e acontece antes de o serviço atender.

## 5. Falhas previstas

| Falha | Efeito | Correção |
|---|---|---|
| `content_version` ausente | boot falha; não serve catálogo vazio | aplicar o seed de versão; conferir o ledger |
| mais de uma versão vigente | boot falha por ambiguidade | corrigir o seed; a versão é um corte, não um histórico de linhas ativas |
| dataset ausente para a versão | boot falha nomeando o dataset | completar o seed do dataset ou reverter a versão |
| seed aplicado sem DDL | migration falha antes do commit | mover o seed para depois do DDL, como exige a série |
| versão antiga em processo vivo | leitura continua no corte antigo | reiniciar o serviço; recarga a quente não existe |

## 6. Dependências e limite do recorte

- **Depende de** `content.*` (arquivos `0013`–`0027`) e de `repo/content.rs`; nenhum dos dois tem
  implementação publicada.
- **Depende de** contagens de seed conferidas em `plano-migrations.md` §4; os seis seeds bloqueados
  (`0016`–`0018`, `0020`, `0023`, `0027`) não podem ser tratados como dados reais.
- **Fora do recorte:** recarga a quente, cache distribuído, ETL da base legada e formato do catálogo
  legado. Não há número de itens, tempo de boot ou TTL a inventar.
