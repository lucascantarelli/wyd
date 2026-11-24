# Imagem do site e `docker compose` do projeto — plano e decisão

> Pergunta: "quem constrói a imagem do site, quem a publica e como o Compose a consome?"
> Público: quem opera, arquitetura e quem edita o site
> Dono: frente `wyd-ops`
> Base: `plano-ci-real.md` §3 (runtime é Compose) e §4.1 (só `run:` e action própria); `wyd-docs-1.0.0.md` §9.1
> Revisado em: 2026-11-23

**Estado:** decidido pelo dono em **2026-11-22** (pedido de parar de reconstruir o MkDocs à mão e ter
o Compose consumindo a imagem), revisto em **2026-11-23** no ponto 6. A implementação acompanha esta
página: `wyd-ops/wyd-docs.Dockerfile`, `docker-compose.yml`, `docker-compose.override.yml`,
`docker-compose.dev.yml`, `.dockerignore` e `.github/workflows/docs-image.yml`.

## 1. Problema

O site era construído por comando manual em container a cada verificação, e o repositório não tinha
Compose nenhum. Duas consequências: o build dependia da máquina de quem opera, e não existia artefato
reproduzível — `docker compose up` não subia nada.

## 2. Decisão

1. **O CI constrói a imagem, não a estação.** Um workflow dedicado (`docs-image`) roda o mesmo build
   do gate e publica a imagem.
2. **O registro é o GHCR**, imagem `ghcr.io/lucascantarelli/wyd-docs`, com duas marcas: `latest` e o
   SHA do commit.
3. **O Compose consome a imagem publicada.** O serviço `docs` referencia a imagem e mantém `build:`
   como fallback local — quem não tem a imagem constrói, quem tem faz `pull`.
4. **Sem action de terceiro.** Login e push usam `docker login` e `docker push` em `run:` com
   `GITHUB_TOKEN`, conforme `plano-ci-real.md` §4.1. Só `actions/checkout` entra, fixado por SHA.
5. **O servidor é o do Python**, `http.server`, sem nginx: o site é estático e não precisa de mais
   uma peça para manter.
6. **O caminho padrão consome a imagem publicada; o build da árvore é explícito.** O Compose base
   referencia `${WYD_DOCS_TAG:-latest}` — a tag que o CI empurra — e mantém `build:` apenas como
   fallback de quem não tem a imagem. O build da árvore de trabalho tem camada própria,
   `docker-compose.dev.yml`, carregada só por `-f`: usa a tag `:dev`, constrói no `up`
   (`pull_policy: build`) e não reinicia sozinho (`restart: "no"`), então **não sobrescreve** a
   imagem publicada. Decisão do dono em 2026-11-23, revista na mesma data: a versão anterior punha
   esses ajustes no override automático, e o resultado medido foi `docker compose pull docs` mirando
   `:dev` — tag que o CI nunca publica — e `up` sempre construindo, ou seja, a imagem publicada nunca
   era consumida. Produção não roda na máquina de desenvolvimento; `:latest` é consumida onde o site
   é servido. Estado do CI: o workflow existe desde `8b23cc2`, mas não há execução nem imagem
   publicada confirmada (2026-11-23) — a medição está em §5.1.
7. **O gate de documentação roda pelo Compose, atrás de perfil e sem montar o site durante o build
   da imagem gate.** O override acrescenta `gate` (perfil `gate`), que executa `preparar.py`,
   `mkdocs build`, `verificar.py`, `verificar-contagens.py` e `verificar-censos.py` em um comando. Seu alvo Dockerfile `gate`
   instala dependências e copia fontes, mas não faz a montagem; assim o comando a executa uma única
   vez. `docker compose --profile gate run --rm gate` valida a árvore de trabalho e sai com o código
   do último verificador; sem
   `--profile gate` o serviço não entra no `up` nem em `docker compose config`. A comparação medida do
   protótipo está em §6; gate verde e contagem permanecem em §5 (dono da contagem:
   `wyd-docs-1.0.0.md` §9.1).

## 3. O que esta decisão **não** faz

| Fora | Dono |
|---|---|
| publicar o site em host de produção | deploy real é decisão separada (`pg-lake`, `plano-ci-real.md` §3) |
| servir o portal Next.js pelo Compose | `wyd-portal` |
| publicar o site no GitHub Pages | `docs.yml`, inalterado por esta entrega |
| fixar `latest` por digest no Compose | fica `WYD_DOCS_TAG` como override; digest é próximo passo se houver necessidade |

## 4. Arquivos e responsabilidade

| Arquivo | Responsabilidade |
|---|---|
| `wyd-ops/wyd-docs.Dockerfile` | estágio `gate` instala dependências e copia fontes; `build` deriva de `gate` e executa `preparar.py` + `mkdocs build`; runtime só com `site/` e `python -m http.server` |
| `docker-compose.yml` | serviço `docs`: porta `${WYD_DOCS_PORT:-8000}`, imagem `${WYD_DOCS_TAG:-latest}` — o que o CI publica —, `build:` só como fallback de quem não tem a imagem |
| `docker-compose.override.yml` | carregado automaticamente; traz **só** o `gate` sob perfil `gate`, que roda os três passos uma vez no estágio `gate` do Dockerfile. Não toca `docs`, para o caminho padrão continuar consumindo a imagem publicada |
| `docker-compose.dev.yml` | **opt-in** (`-f`), não mesclado automaticamente: `docs` com tag `:dev`, `pull_policy: build` e `restart: "no"`; porta, `build:`, `container_name` e contexto vêm do base |
| `.dockerignore` | mantém fora do contexto `.git`, `deprecated/`, `wyd-client/` (567 MB), `**/site`, `**/.build`, `.venv`, `.env` |
| `.github/workflows/docs-image.yml` | build, fumaça HTTP 200 e push no GHCR (push só fora de PR) |

O **dono do layout** é `planejamento-1.0.0/wyd-1.0.0.md` §8: ele diz, para cada tipo de artefato,
onde mora, qual sistema responde e por que esses arquivos ficam na raiz (Compose, override,
`.dockerignore` e `.github/workflows/`). Esta página não repete a tabela — só lista os arquivos desta entrega. Nenhum
deles pode ser copiado para `planejamento-1.0.0/wyd-ops/`: aquela árvore é documentação
(`AGENTS.md` §5, item 6).

## 5. Como verificar (DoD)

```bash
# 1. build igual ao do CI, com as mesmas tags (SHA do commit e latest)
SHA=$(git rev-parse HEAD)
MSYS_NO_PATHCONV=1 docker build -f wyd-ops/wyd-docs.Dockerfile \
  -t ghcr.io/lucascantarelli/wyd-docs:$SHA \
  -t ghcr.io/lucascantarelli/wyd-docs:latest .

# 2. o Compose padrão sobe a partir dessa imagem, sem construir
MSYS_NO_PATHCONV=1 docker compose up -d docs
curl -fsS -o /dev/null -w '%{http_code}\n' http://127.0.0.1:8000/
curl -fsS http://127.0.0.1:8000/ | grep -o '<title>[^<]*</title>'

# 3. gate de documentação em um comando (montagem, build e verificação)
MSYS_NO_PATHCONV=1 docker compose --profile gate config  # gate.build.target deve ser gate
MSYS_NO_PATHCONV=1 docker compose --profile gate run --rm -T gate
# esperado: exit 0 e a linha "verificação: … ok (N fontes verificadas)" — contagem em
# wyd-docs-1.0.0.md §9.1
```

### 5.1 Medição de 2026-11-23 (`V`)

Host de desenvolvimento, Docker 29.9.2, árvore sem alteração durante as medições. Cada linha é o
comando executado, não a expectativa.

| Passo | Resultado medido |
|---|---|
| build com as duas tags (passo 1) | exit **0**, **17 s**, imagem `sha256:f3b15499d90f…`, **194 329 433 bytes**; `:latest` e `:<sha>` apontam para o mesmo id |
| fumaça do workflow, passo por passo: `docker run -d --name docs-smoke -p 8123:8000`, laço de `curl` e `docker rm -f` | `run` exit 0; **HTTP 200 na 2ª tentativa** de 30 — a 1ª devolveu `Empty reply from server` porque o servidor ainda subia, que é justamente o que o laço do job cobre; `rm` exit 0 |
| páginas servidas pelo container | `find /site -name '*.html' \| wc -l` = **269**, igual ao "269 páginas" do gate |
| `docker compose config --services` sem perfil | `docs` — o `gate` não vaza para o `up` |
| `docker compose up -d docs` | exit 0, **sem construir**; a imagem do container é `sha256:f3b15499d90f…` (a do passo 1); HTTP **200**; `<title>WYD 1.0.0 — documentação oficial · WYD 1.0.0</title>`; 269 HTML |
| `docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d docs --build` | exit 0, imagem `:dev` (`sha256:027efb023d40…`), `restart: no`, HTTP 200; `down` exit 0 |
| `docker compose pull docs` | exit **1**: `Image ghcr.io/lucascantarelli/wyd-docs:latest error from registry: denied` — o pacote não existe; só fica verde depois da primeira publicação |
| `docker compose --profile gate run --rm -T gate` | exit **0**: `verificação: 269 páginas, links, âncoras, navegação e sintaxe ok (267 fontes verificadas)` |

**DoD do workflow, ainda não cumprido:** a primeira execução em `develop` termina verde, empurra
`:<sha>` e `:latest`, e `docker compose pull docs` baixa a imagem sem construir. Hoje o build, a
fumaça e o consumo da imagem local estão provados; o `pull` reprova porque não há pacote publicado —
causa e o que falta em `wyd-1.0.0.md` §8.3. O job não tem passo manual: build, fumaça e push são
`run:` com `docker` e `curl`.

## 6. Risco

| Risco | Efeito | Mitigação |
|---|---|---|
| imagem grande por copiar o repositório no estágio de build | pull lento | estágio de runtime recebe só `site/`; `.dockerignore` corta cliente e legado |
| `latest` mudando sozinho | site servido diverge do commit | `WYD_DOCS_TAG:<sha>` no ambiente que precisa de reprodutibilidade; em desenvolvimento, `docker-compose.dev.yml` usa `:dev` com `pull_policy: build` |
| build local gravar `:latest` | a imagem baixada é substituída pela da árvore | o build de dev usa `docker-compose.dev.yml` (`:dev`); quem roda `docker compose up --build docs` sem ele aceita a tag da árvore local |
| pacote GHCR privado | `pull` sem `docker login` falha | imagem de documentação pública; se virar privada, o host precisa de credencial |
| workflow duplicar o gate | dois builds do mesmo commit | o gate continua em `docs.yml`; a imagem não reexecuta `verificar.py` |
| build de gate voltar a montar o site antes do comando | custo extra, duplicando MkDocs | eliminado: `target: gate` termina após copiar fontes; montagem, build e verificação só rodam no comando |

### 6.1 Medição da otimização do gate (`V`, 2026-11-23)

Comparação controlada no mesmo host, com a mesma árvore sem alterações e caches aquecidos. O cenário
anterior usou temporariamente o Dockerfile de `HEAD` com alvo `build`; o novo usou o Dockerfile atual
com alvo `gate`. Ambos executaram `docker compose run` e os mesmos três passos. O tempo inclui
resolução/build da imagem, criação do contêiner, montagem, MkDocs e verificação.

| Execução | Antes (alvo `build`, monta durante o build da imagem e de novo no comando) | Depois (alvo `gate`, monta só no comando) |
|---|---:|---:|
| 1 | 60 s | 40 s |
| 2 | 59 s | 51 s |
| Média das duas execuções | 59,4 s | 45,5 s |

Redução média observada: **13,9 s (23%)**. As duas execuções de cada cenário terminaram em exit 0,
com `verificação: 267 páginas, links, âncoras, navegação e sintaxe ok (265 fontes verificadas)` — a
contagem da árvore de então; a de hoje está em §5.1, porque páginas nasceram depois.
A variação entre execuções mostra que este número não é promessa de velocidade nem orçamento de CI.

Prova de que a saída não mudou: hash agregado dos arquivos `/site` da imagem construída com o
Dockerfile anterior e da nova `:dev` — `e37cb201…` em ambas, `diff -qr` sem diferenças. A primeira
execução integrada após a edição do Dockerfile, com cache parcialmente quente, levou 82 s; registrei-a
como observação de validação, mas não a incluí na comparação controlada acima.
