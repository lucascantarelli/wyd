---
name: planejar-sistema
description: Abre o planejamento de um sistema do WYD que ainda está vazio, definindo fronteira, contrato com outros sistemas, requisitos, arquitetura, plano de teste e roadmap antes de qualquer código. Use quando a pasta em `planejamento-1.0.0/` só tiver `.gitkeep`, quando o time pedir para iniciar um sistema novo ou quando for preciso decidir o que cada sistema consome e fornece.
---

# Planejar sistema

## Quando usar

- O sistema tem pasta em `planejamento-1.0.0/` mas nenhum documento.
- O time pediu para iniciar `wyd-protocol`, `wyd-core`, `wyd-launcher` etc.
- É preciso definir o contrato entre dois sistemas antes de planejar o interior deles.

Não use para escrever página dentro de sistema já aberto: siga o molde e o dono que já existem.

## Pré-requisitos

- `AGENTS.md` lido, em especial §5 e §6.
- Escopo e dono aprovados pelo time. Sem “sim”, o máximo que se produz é **proposta**.
- Sistema consumidor e fornecedor identificados. Se não estiver claro, pare aqui.
- Molde de referência: `planejamento-1.0.0/wyd-db/` (estrutura, não conteúdo).

## Passos

### 1. Escrever a fronteira em uma página

Responda, em no máximo uma tela:

| Pergunta | Saída |
|---|---|
| O que este sistema faz? | 3–5 linhas, sem adjetivo |
| O que ele **não** faz? | lista, com o dono de cada item |
| Quem consome? | sistema + o que consome |
| O que ele consome? | sistema + contrato (porta, mensagem, arquivo) |
| O que é estado dele? | o que persiste, o que é efêmero |
| O que quebra se ele cair? | efeito no jogador e nos outros sistemas |

Se a resposta depender de decisão não tomada, vira lacuna com ID e dono — não vira suposição no
texto.

### 2. Definir o contrato antes do interior

Para cada troca com outro sistema, registre: mensagem/porta, direção, garantia (ordem, idempotência,
timeout), erro esperado e dono do contrato. Protocolo de fio não se descreve por “o cliente manda”:
o servidor é autoridade.

Contrato sem dono é o defeito que produz duas implementações incompatíveis.

### 3. Criar a árvore mínima

Espelhe o molde aprovado, sem copiar texto:

```text
planejamento-1.0.0/<sistema>/
├── <sistema>-1.0.0.md      # dashboard: escopo, estado, métricas, checklists
├── docs/
│   ├── index.md            # porta de entrada
│   ├── convencoes.md       # só se o sistema tiver regra própria; senão cita wyd-db
│   ├── arquitetura/
│   ├── specs/
│   ├── planos/
│   ├── roadmap/
│   ├── guia-desenvolvedor/
│   ├── guia-teste/
│   └── guia-administrador/ # só se o sistema for operado
├── src/                    # por unidade de responsabilidade
├── tests/                  # por alvo de teste
├── prompts/
└── skills/
```

Pasta que não tem pergunta própria não é criada. Estrutura vazia por simetria é enfeite.

### 4. Escrever requisitos com ID

Um bloco por requisito, com: enunciado, entrada, saída, critério de aceitação verificável, contrato
envolvido e issue que entrega. Regra de cálculo leva exemplo numérico. Sem critério de aceitação não
é requisito, é intenção.

Prefixos: use o do sistema (`RF-<SYS>-nnn`, `RN-<SYS>-nnn`, `RNF-<SYS>-nnn`). Não renumere o que já
existe.

### 5. Planejar arquitetura e código

- Uma página por unidade de responsabilidade, com “responsabilidade”, “API pública”, “invariantes”,
  “quem depende”, “testes que cobrem” e “risco de alteração”.
- Nada de espelho 1:1 de arquivo `.rs` — exceto módulo que o próprio plano declare implementável
  isoladamente (como `migrate/` e `provision/` no `wyd-db`).
- Árvore de código e de teste vem antes do primeiro arquivo.

### 6. Planejar teste

Defina alvos, o que cada um prova, dado necessário, isolamento e comando. Pirâmide: unitário →
contrato → sistema → E2E. E2E de `portal`/`client` é Playwright; o resto é Rust. Cobertura mínima 85%.
Nenhum `SKIP` silencioso.

### 7. Fatiar o roadmap

Uma página por issue, com: título, fase, prioridade, contexto, entregável, DoD verificável (comando
ou arquivo), fora de escopo, risco, dependência e evidência ao fechar. Issue sem DoD executável não
entra.

### 8. Fechar o dashboard

`<sistema>-1.0.0.md` mede, não narra: quantas páginas por pasta, quantas escritas, quantos requisitos,
quantas issues, qual gate. Número com fonte e data.

## Como verificar

```bash
# toda página tem cabeçalho
grep -rL '^> Pergunta:' planejamento-1.0.0/<sistema>/docs --include='*.md'

# não há código dentro do planejamento
find planejamento-1.0.0/<sistema> -type f \( -name '*.rs' -o -name '*.sql' -o -name '*.ts' \)

# não há ADR sem decisão
ls planejamento-1.0.0/<sistema>/docs/arquitetura/adr/ 2>/dev/null
```

Depois, se o sistema publica no MkDocs, `mkdocs build --strict` sem link quebrado.

## Quando parar e perguntar

- Escopo, dono ou contrato não estão aprovados.
- O sistema precisa de tabela nova no `wyd-db` (muda contagem e exige decisão).
- Aparece regra de jogo que pertence a `wyd-sim`/`wyd-core` e não ao sistema em planejamento.
- A estrutura do `wyd-db` não serve e você quer propor outra: proponha, não crie em paralelo.
