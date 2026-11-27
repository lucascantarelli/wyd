---
name: planejar-onda
description: Monta uma onda de trabalho do wyd-db com arquivos, ordem, dependências e critério de pronto antes de abrir as issues. Use quando há várias issues para uma mesma frente, quando a ordem entre elas importa, ou quando um lote de documentos precisa entrar de uma vez.
---

# Planejar onda

## Quando usar

- Várias issues na mesma frente (série, dicionário, testes).
- A ordem de execução muda o resultado (dependência de DDL, de página, de dado medido).

Não use para uma issue isolada: ela já cabe em `execution/implementar-issue`.

## Pré-requisitos

- `planejamento-1.0.0/wyd-db/docs/roadmap/` e `docs/roadmap/fases.md` lidos (ordem, risco e regra de
  fatiamento são deles).
- Lista do que já existe: `ls` no diretório real, não a lembrança.

## Passos

1. **Liste os arquivos da onda** com caminho real e estado atual (`existe` / `falta`).
2. **Declare a dependência entre eles** na forma "A antes de B porque B referencia A"; dependência sem
   motivo não entra.
3. **Separe o que depende de dado medido** e ponha fora da onda, com o motivo — não empurre seed sem
   medição para "fechar a onda".
4. **Escreva o critério de pronto da onda**: comando que roda no fim e prova o conjunto (gate, série
   aplicando do zero, censo fechando).
5. **Dimensione**: onda grande demais se divide por arquivo, não por "fase 2".
6. **Só então abra as issues**, uma por arquivo, citando a onda e o critério.

## Como verificar

Rode o critério de pronto antes de começar: ele precisa falhar hoje pelo motivo declarado. Se já
passa, a onda não tem o que entregar.

## Quando parar e perguntar

- A onda exige mudar a ordem já aprovada em `roadmap/`: decisão do dono.
- O critério depende de tempo (prazo, calendário): o planejamento não fixa prazo.

## Armadilhas

- **Onda que inclui seed bloqueada** só para a contagem fechar.
- **Dependência circular** entre páginas (A cita B e B cita A como dono).
- **Critério de pronto que é a soma das issues** — precisa ser um comando único sobre o conjunto.
