# Prompt — auditoria de cenário

Use para atualizar o panorama do projeto sem alterar código, documento de sistema ou GitHub.

## Entrada

- Data da medição (obrigatória).
- Escopo: repositório inteiro ou um sistema.
- O que mudou desde a última medição, se souber.

## O que pedir

> Meça o estado real do repositório WYD e compare com o que os documentos declaram. **Não edite
> nada** além do panorama e da memória. Não crie issue, label, milestone, commit ou deploy.
>
> 1. Rode os comandos de medição abaixo e guarde a saída.
> 2. Compare cada número declarado em `planejamento-1.0.0/wyd-1.0.0.md` §1 e no dashboard do
>    sistema com a medição.
> 3. Liste divergências entre documento e realidade, e entre dois documentos.
> 4. Verifique quantos ADRs existem e se cada um tem decisão, estado, data e “como verificar”.
> 5. Verifique se há afirmação de comportamento sem `V`/`H`/`P`.
> 6. Atualize **somente** `planejamento-1.0.0/wyd-1.0.0.md` §1 e
>    `.agents/memory/estado-do-projeto.md`, com a data nova.
> 7. Entregue a tabela de divergências e diga o que não conseguiu medir.

## Comandos de medição

```bash
git status --short --branch
git log -1 --format='%h %ad %s' --date=short

# planejamento por sistema
for d in planejamento-1.0.0/*/; do printf '%-45s %s\n' "$d" "$(find "$d" -type f | wc -l)"; done

# páginas com cabeçalho, por sistema
grep -rl '^> Pergunta:' planejamento-1.0.0 --include='*.md' | sed 's#/[^/]*$##' | sort | uniq -c

# ADRs e estados
find planejamento-1.0.0 -path '*/adr/*.md' -type f | sort
grep -rlE '^\*\*Estado:\*\*' planejamento-1.0.0 --include='*.md' | wc -l

# código e workspace
find crates -maxdepth 2 -type f | sort
ls Cargo.toml rust-toolchain.toml Makefile 2>/dev/null
git ls-files deprecated | wc -l

# CI
find .github -type f | sort
grep -nE 'runs-on|uses:|run:' .github/workflows/*.yml 2>/dev/null
```

Para GitHub, leia via integração (somente leitura): repositório, issues, milestones e projects.
Registre a data da consulta; a API muda sem commit.

## Saída obrigatória

| # | Divergência | Documento afirma | Medição | Efeito | Dono da decisão |
|---|---|---|---|---|---|

Depois:

- **Medido:** o que foi contado e com qual comando.
- **Não medido:** o que depende de acesso, ambiente ou decisão.
- **Mudanças aplicadas:** apenas os dois arquivos permitidos.

## Proibido

- Corrigir documento de sistema durante a auditoria.
- Fechar lacuna, renomear tabela, criar ADR ou “ajustar” contagem no texto.
- Usar número do legado como estado atual.
- Escrever no GitHub.
