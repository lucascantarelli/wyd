# Dossiê D-05 — Workspace Cargo único, toolchain e versões

> Pergunta: "D-05 deve adotar um workspace Cargo único na raiz, com toolchain e Makefile, ou manter os crates independentes, ou um híbrido?"
> Público: dono do produto, arquitetura, `wyd-ops` e equipes dos crates Rust
> Dono: `wyd-ops` (dossiê); a decisão é do dono do produto
> Base: `wyd-1.0.0.md` §8.7; `prioridade-do-projeto.md` §0.1; árvore viva do repositório em 2026-11-25
> Revisado em: 2026-11-25

**Estado: APROVADA em A em 2026-11-25.** A decisão foi registrada pelo dono do produto. O plano de
migração está em `plano-migracao-d-05.md`; ele **não foi executado** e não cria código/configuração
nesta rodada. As evidências de §2 continuam sendo a base da decisão.

---

## 1. A pergunta única

**D-05:** o projeto deve ter **um workspace Cargo único na raiz**, com `rust-toolchain.toml`,
`Makefile`, um `Cargo.lock` e imagens fixadas por digest — ou deve manter **crates independentes**, ou
ainda um **híbrido**?

A pergunta é decidível em A/B/C, e a resposta registrada é **A**. Ela não decide qual crate nasce
primeiro, não muda o foco D-01 (`wyd-db`) e não autoriza código.

## 2. Evidência conferida no repositório

Escopo da medição: árvore viva, excluindo `deprecated/`, `wyd-client/files/`, `.build/` e `.git/`.
O legado foi medido à parte e não é fonte de decisão (`AGENTS.md` §3).

| Fato | Medida em 2026-11-25 | Como foi medido |
|---|---:|---|
| diretórios em `crates/` | **9** | `find crates -maxdepth 1 -mindepth 1 -type d` |
| crates com `Cargo.toml` | **1** (`crates/wyd-db`) | `git ls-files` + leitura do arquivo |
| crates só com `.gitkeep` | **8** (`admin`, `bots`, `core`, `launcher`, `net`, `protocol`, `server`, `sim`) | conteúdo de cada diretório |
| `Cargo.toml` na raiz | **0** | `ls Cargo.toml`; `git ls-files` |
| `Cargo.lock` na raiz | **0** | `ls Cargo.lock`; `git ls-files` |
| `Cargo.lock` no crate real | **1** (`crates/wyd-db/Cargo.lock`, versão 4) | cabeçalho do arquivo |
| `rust-toolchain.toml` na árvore viva | **0** | `git ls-files` e busca na árvore viva |
| `Makefile` na árvore viva | **0** | `git ls-files` e busca na árvore viva |
| toolchain Rust fixada no repositório | **0 versões** | nenhum `rust-toolchain.toml`; `Cargo.toml` não declara `rust-version` |
| versão local observada | Rust **1.100.0** | `rustc --version` / `cargo --version` (2026-11-25) |
| canal do CI | **`stable` implícito**, sem pin | `code.yml` usa `rustup component add`, não instala toolchain; `runs-on: ubuntu-latest` |
| comandos `cargo` com `--manifest-path` no `code.yml` | **6** | fmt, clippy, teste de lib, cobertura (2 comandos) e teste de schema |
| imagens externas sem digest | **2 nomes distintos** (`python:3.15-slim`, `postgres:17`) | Dockerfile do site e `docker run` do `code.yml` |
| referências de imagem sem digest | **6** | 2 estágios Python, 1 PostgreSQL e 3 tags GHCR (`latest`, `dev`, `gate`) |
| imagens Rust no Compose/gate | **0** | `gate` usa somente Python; nenhum `Dockerfile` Rust |
| manifesto Python | **1** (`wyd-docs/requirements.txt`) | `git ls-files` |
| `deprecated/` (legado ignorado) | 12 `Cargo.toml`, 2 `Cargo.lock`, 1 `rust-toolchain.toml`, 2 `Makefile` | busca separada com `-maxdepth 5`; **não** é árvore viva |

### 2.1 Correção de números anteriores

A frase “até nove locks” **não descreve o presente**: hoje existe **um** `Cargo.lock`, no crate
`wyd-db`. O número **9** é apenas o teto se os **8 crates vazios** virarem crates independentes e cada
um ganhar seu próprio lock. O dossiê usa “1 hoje; até 9 se B for seguido”, não “9 hoje”.

### 2.2 O que o gate e o Compose fazem hoje

- `docker compose --profile gate run --rm gate` constrói apenas o estágio Python do site e roda
  `preparar.py`, MkDocs, `verificar.py`, `verificar-contagens.py` e `verificar-censos.py`. **Não
  compila Rust** e não depende de `Cargo.toml`.
- O Rust é compilado diretamente no runner do `code.yml`, com `cargo ... --manifest-path
  crates/wyd-db/Cargo.toml`. O `security` procura **o primeiro** `Cargo.toml`/`Cargo.lock` com
  `find ... -print -quit`.
- O `code.yml` só dispara em `crates/**`, `deny.toml` e no próprio workflow. Um `Cargo.toml`,
  `Cargo.lock`, `rust-toolchain.toml` ou `Makefile` **na raiz** não está na lista de gatilhos.
- `python:3.15-slim` aparece em dois estágios do Dockerfile e `postgres:17` no `code.yml`; nenhum dos
  dois está fixado por digest. As três tags GHCR (`latest`, `dev`, `gate`) também não usam digest.

## 3. Opções

| Opção | O que é | Custo agora | O que trava depois |
|---|---|---|---|
| **A — workspace único + toolchain + Makefile + digests** | `Cargo.toml` de workspace na raiz com membros explícitos; um `Cargo.lock` na raiz; `rust-toolchain.toml` com versão declarada; `Makefile` com alvos únicos; ações por SHA e imagens por digest | Criar o workspace, mover/gerar **1** lock para a raiz, criar toolchain e Makefile, atualizar os **6** comandos `--manifest-path` e os gatilhos do `code.yml`, fixar **2** imagens externas por digest. Ainda não cria crate novo nem código de produto. | Nada para o segundo crate por falta de workspace; a migração é única. O custo é concentrado agora, com **1** crate real. |
| **B — crates independentes** | Cada crate com seu `Cargo.toml`, seu `Cargo.lock` e sua configuração; sem workspace, toolchain ou Makefile na raiz | **0** arquivos novos agora; o primeiro crate já funciona assim | Até **9** locks independentes quando os 8 vazios virarem crates; o `security` do CI continua auditando só o primeiro lock; cada crate pode usar toolchain diferente; comandos locais e de CI se repetem. |
| **C — híbrido** | Workspace + lock na raiz, mas sem toolchain/Makefile; ou toolchain/Makefile sem workspace; digests podem ficar para depois | Custo intermediário entre A e B | O comando local não prova o mesmo que o CI; a reprodutibilidade fica parcial; a dívida de versões continua em algum ponto. |

## 4. Recomendação

**A — workspace único + toolchain + Makefile + digests.**

A justificativa é a evidência, não a simetria:

1. **O custo de A é pequeno porque só há 1 crate real.** Hoje são 9 diretórios, mas 8 só têm
   `.gitkeep`; o workspace nasce com **1** membro e cresce por adição explícita.
2. **O custo de B cresce exatamente onde o projeto vai entrar.** O próximo crate (`wyd-protocol` ou
   outro Rust) repetiria lock, toolchain e configuração; com os 8 vazios, o teto é **9** locks.
3. **O `security` atual não sobrevive a B sem mudança.** Ele escolhe o primeiro `Cargo.lock`; com
   vários crates, os demais não seriam auditados. A não é só organização: evita um gate cego.
4. **O CI já está no limite de comandos repetidos.** São **6** `--manifest-path` apontando para
   `crates/wyd-db`; um Makefile/workspace pode reduzir isso a comandos únicos, mas os gatilhos
   precisam incluir a raiz.
5. **As imagens não estão fixadas.** São **2** nomes externos e **3** tags GHCR sem digest; A inclui
   fixá-las, alinhado ao princípio de reprodutibilidade já aceito em D-02 para ações por SHA.

**A não fecha** as 6 lacunas abertas + 1 proposta (captura/direitos/medição), não muda D-01 e não
autoriza código. Ela apenas define a estrutura que o segundo crate e o gate vão usar.

## 5. Impacto em bloqueios, lacunas e frentes

| Item | Se A | Se B | Se C |
|---|---|---|---|
| workspace | 1 workspace, 1 lock, 1 toolchain | 9 configurações possíveis; 1 lock hoje | parcial; lock/toolchain podem divergir |
| segundo crate Rust | destravado com o workspace pronto | pode nascer, mas repete lock/toolchain | depende do que o híbrido escolher |
| gate `code.yml` | atualizar 6 comandos e gatilhos da raiz; audit/deny no lock único | corrigir `security` para varrer todos os locks | atualização parcial, com risco de comando local ≠ CI |
| imagens por digest | 2 imagens externas fixadas; 3 tags GHCR passam a ter política | tags continuam | tags podem continuar |
| D-04 / organização | não reabre a auditoria; `wyd-db` e `wyd-admin` seguem os únicos organizados; `wyd-portal` continua independente de D-05 | igual | igual |
| D-01 / `wyd-db` | **não bloqueia** o passo 2 (`migrations/serie-e-regras.md`); o foco de execução continua no `wyd-db` | igual | igual |
| lacunas abertas | **0** fechadas; as 6 + 1 dependem de captura/direitos/medição | 0 | 0 |
| frentes liberadas | segundo crate e todos os crates Rust; `wyd-protocol` pode ser implementado depois do planejamento | cada crate isolado, sem padrão comum | liberação parcial, com dívida de versões |

**Impacto no gate de documentação:** **nenhum**. O `gate` continua Python e as contagens publicadas
seguem 309 páginas / 307 fontes / 141 atos / 79 aceitas / 64 células / 0 Q-BLK. A mudança de A é no
`code.yml` e na raiz, não no site.

## 6. O que esta decisão não resolve

- Não fecha `LPRT-03`, `LPRT-06`, `LCLI-02`, `LCLI-05`, `LNET-01` ou `LBOT-03`; todas dependem de
  captura, direitos ou medição.
- Não substitui D-04: a auditoria de organização continua válida; `wyd-portal` não é crate Rust e
  precisa da árvore `src/`/`tests/` e do plano de teste, não do workspace Cargo.
- Não escolhe o próximo crate; apenas remove a repetição de lock/toolchain quando ele nascer.
- A decisão não implementa workspace, Makefile, toolchain ou digest; a execução é o plano em
  `plano-migracao-d-05.md`.

## 7. Decisão registrada

**Resposta do dono do produto em 2026-11-25: A.** Adotar o workspace Cargo único na raiz, com
`rust-toolchain.toml`, `Makefile`, **um** `Cargo.lock`, ações por SHA e imagens por digest.

A execução está descrita em `plano-migracao-d-05.md`; este dossiê não implementa os arquivos e não
altera o CI. O próximo crate Rust herda a política; o foco D-01 continua no passo 2 do `wyd-db`.
