---
name: auditar-planejamento
description: Audita documento de planejamento do WYD atrás de afirmação sem evidência, contagem sem fonte, ADR inventado, cópia do projeto legado e dono duplicado. Use antes de aceitar uma página, depois de um lote de documentos, quando um número parecer herdado do `deprecated` ou quando o número de ADRs crescer sem decisão registrada.
---

# Auditar planejamento

## Quando usar

- Antes de marcar uma página como `aceito`.
- Depois de um lote de documentos escritos por agente.
- Quando uma contagem não bate com o código ou com a pasta.
- Quando aparece ADR, requisito ou regra que ninguém lembra de ter aprovado.

Não use para revisar código Rust: ainda não há código de produção.

## Pré-requisitos

- Ler `AGENTS.md` §3 e §5.
- Saber qual é o sistema e qual é o arquivo dono do assunto.
- **Auditoria é leitura.** Não corrija, não renomeie, não feche lacuna durante a auditoria.

## Passos

### 1. Inventariar o que existe

```bash
find planejamento-1.0.0 -type f | sort
grep -rl '^> Pergunta:' planejamento-1.0.0 --include='*.md'
grep -rlE '\*\*Estado:\*\*' planejamento-1.0.0 --include='*.md'
```

Conte por pasta e compare com o que o dashboard do sistema declara. Divergência entre dashboard e
arquivo é achado, não detalhe.

### 2. Conferir ADR e decisão

```bash
ls planejamento-1.0.0/*/docs/arquitetura/adr/ 2>/dev/null
grep -rhoE 'ADR-[A-Z]+-[0-9]{4}' planejamento-1.0.0 --include='*.md' | sort -u
grep -rhoE '^\| \*\*[A-Z][0-9]+\*\*' planejamento-1.0.0/wyd-db/lacunas.md | sort -u
```

Regra: ADR só existe se houver decisão tomada, com estado, data e “como verificar”. Hoje o esperado
é **dois** ADRs (`ADR-DB-0001`, `ADR-DB-0002`). ADR a mais é achado **crítico** até prova em
contrário; ADR citado e inexistente também.

### 3. Conferir evidência

Procure afirmação de comportamento no presente sem `V`:

```bash
grep -rnE '\b(faz|garante|impede|executa|retorna|grava)\b' planejamento-1.0.0/wyd-db/docs --include='*.md'
```

Para cada trecho, verifique se há `V` com arquivo/comando e data, `H` com ID de lacuna, ou `P`. `H`
sem ID é omissão, não hipótese.

### 4. Conferir dono único do número

```bash
grep -rnE '\b(29|91|53|87|146|40)\b' planejamento-1.0.0 --include='*.md' | grep -v 'plano-migrations.md'
```

Número repetido em duas páginas sem citação do dono é achado. Os números da 1.0.0 são **29
migrations, 91 tabelas, 6 seeds**; os do legado são **53 migrations, 87 tabelas** e não podem ser
apresentados como estado atual.

### 5. Procurar contaminação do legado

Sinais típicos:

- tabela no plural (`accounts`, `characters`) como se fosse o desenho novo;
- `public` como schema de domínio;
- `SERIAL`, `timestamp` sem fuso, `money`, `ENUM` nativo;
- contagem de issues/crates do README antigo;
- migration numerada fora da série de 29;
- texto que descreve “como funciona hoje” sem `V` no código novo.

```bash
grep -rniE 'serial|timestamp[^t]|\bmoney\b|\benum\b' planejamento-1.0.0/wyd-db --include='*.md'
```

O achado não é a palavra: é a palavra usada como decisão da 1.0.0 sem decisão registrada.

### 6. Procurar placeholder e enfeite

```bash
grep -rniE 'TODO|TBD|em constru|a preencher|robusto|escal[aá]vel|r[aá]pido' planejamento-1.0.0 --include='*.md'
```

Também reprova: seção “Introdução” que repete o título, índice manual no topo, tabela de conteúdo
escrita à mão, captura sem texto alternativo.

### 7. Conferir cabeçalho e link

Toda página tem cinco linhas (`Pergunta`, `Público`, `Dono`, `Base`, `Revisado em`) e linha de estado
com data. Link relativo para fora de `docs/` reprova; use caminho em `code` com a marca
“planejamento”.

## Como relatar

Uma linha por achado, ordenada por severidade:

| Severidade | Arquivo:linha | O que está escrito | Evidência contrária | Ação |
|---|---|---|---|---|
| crítica | `…:12` | “ADR-DB-0005 decide…” | não há arquivo nem decisão | remover ou abrir decisão com o dono |
| alta | `…:40` | “53 migrations” como estado atual | `plano-migrations.md` §3: 29 | citar o dono |
| média | `…:8` | afirmação sem marca | — | marcar `H` com ID |

Termine com: **o que foi medido**, **o que não foi possível medir** e **quantos achados por
severidade**. Não proponha correção de conteúdo que dependa de decisão do time; registre e pare.

## Quando parar e perguntar

- O achado exige fechar lacuna, renomear tabela ou mudar decisão.
- O documento e o dono divergem e não está claro qual vale.
- Aparece ADR, requisito ou regra sem dono identificável.
- A correção mudaria contagem de tabela, migration ou issue.
