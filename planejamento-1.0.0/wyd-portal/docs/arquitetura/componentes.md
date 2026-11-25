> Pergunta: "quais unidades de código o `wyd-portal` tem, qual é a API de cada uma e o que ela não pode violar?"
> Público: Desenvolvedor do `wyd-portal`
> Dono: equipe do wyd-portal
> Base: [fronteira.md](fronteira.md); [rotas-e-permissoes.md](rotas-e-permissoes.md); [consumo-do-banco.md](consumo-do-banco.md); [monitoramento.md](monitoramento.md); `wyd-admin/docs/arquitetura/componentes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum arquivo de código existe. O portal é Next.js (TypeScript); a árvore abaixo descreve unidades de responsabilidade, não arquivos 1:1.

## `auth`

- **Responsabilidade:** autenticar pessoa no portal (e-mail/senha, sessão revogável, token de cerimônia) e manter a conta de jogo ligada à pessoa; aplicar segundo fator TOTP para staff.
- **API:** `login(email, senha)`, `logout(session_id)`, `verify_token(token, purpose)`, `require_role(role, mínimo)`, `require_mfa(person)`.
- **Invariantes:** senha nunca aparece em log; sessão só nasce depois do segundo passo para `admin`/`moderator`; `X-Staff-Person-Id` sem token válido é `401`; papel nunca vem do corpo; recuperação de segundo fator é ato de staff, nunca autoatendimento.
- **Depende de:** `banco` (`portal.person`, `person_session`, `person_token`, `person_authenticator`).
- **Testes:** token inválido, papel `member` em rota staff, reuso de token consumido, segundo fator ausente.
- **Risco de alteração:** alto — é a única barreira entre o navegador e o estado.

## `banco`

- **Responsabilidade:** pool, transação, `search_path` fechado e SQL nomeado para o adapter do `wyd-db`.
- **API:** `tx(|conn| …)`; consultas por nome; nenhum SQL montado por string de entrada.
- **Invariantes:** `wyd_portal` não tem `USAGE` em `game` (`P-17`); uma transação por requisição; timeout de 2 s; papel próprio por superfície.
- **Depende de:** `wyd-db` (papel e `GRANT`).
- **Testes:** papel sem `INSERT` falha; transação não vaza conexão; leitura de `game` só pela view de `0029`.
- **Risco:** médio — pool mal dimensionado derruba o portal por timeout.

## `bff`

- **Responsabilidade:** rotas de servidor (BFF) que validam sessão, permissão e entrada antes de tocar o banco ou o `wyd-admin`.
- **API:** uma função por rota; `RequestContext { person_id, role, trace_id, idempotency_key }`.
- **Invariantes:** nenhuma rota toca o banco antes de validar cabeçalho e corpo; toda mutação grava `actor_kind`/`actor_id` em `audit_log`; mutação de estado de jogo **não** é SQL do portal — chama o `wyd-admin` (`LAC-POR-01`).
- **Depende de:** `auth`, `banco`, `admin`, `observabilidade`.
- **Testes:** contrato de cada rota (200/4xx), cabeçalho ausente, corpo inválido, papel insuficiente.
- **Risco de alteração:** alto — mudar rota ou envelope quebra o front-end e o cliente do mercado.

## `admin`

- **Responsabilidade:** cliente do `wyd-admin` (HTTP/JSON interno, contrato C4): kick, broadcast, ajuste de saldo, custódia de item.
- **API:** `kick(session_id)`, `broadcast(scope, message)`, `escrow(character_id, item)`, `deliver(escrow_id)`, `return(escrow_id)`, `cancel(escrow_id)`, com prazo de 1,5 s.
- **Invariantes:** sem ack não há escrita em `game.session`; timeout não é sucesso; toda chamada leva `Idempotency-Key` e `X-Actor-Account-Id`.
- **Depende de:** contrato C4 do `wyd-admin` (ainda proposto).
- **Testes:** dublê que não responde, ack atrasado, ack duplicado, erro tipado `409`.
- **Risco:** alto — o endpoint do `wyd-admin` ainda não tem plano; o cliente é escrito contra o contrato decidido e testado com dublê, nunca “aproximado”.

## `cache`

- **Responsabilidade:** cache de página e de consulta (ISR, cache de 60 s nos rankings, janela declarada na armaria).
- **API:** `revalidate(tag)`, `get_or_set(key, ttl, loader)`.
- **Invariantes:** ranking informa o instante de leitura/atualização e o limite aplicado; armaria declara a janela de cache; nenhum cache esconde escrita — escrita aparece no carregamento seguinte.
- **Depende de:** `banco`.
- **Testes:** TTL expirado recarrega; escrita aparece no próximo carregamento; cache não vaza entre contas.
- **Risco:** médio — cache stale dá número velho sem aviso.

## `monitoramento`

- **Responsabilidade:** cliente das fontes externas (Prometheus, Loki, Grafana) e agregação para o NOC.
- **API:** `query(promql)`, `query_range(...)`, `logs(service, level, text)`, `dashboards()`.
- **Invariantes:** fonte fora deixa **só** os cartões dela em “não medido”, sem zerar os demais; credencial e URL interna nunca chegam ao navegador (`RNF-POR-007`); nenhum card de CPU/RAM/latência vem de tabela de `portal`/`ops`.
- **Depende de:** `wyd-ops` (URL, credencial, intervalo, retenção).
- **Testes:** fonte fora → “não medido”; `limit` fora de 1–1.000 → `422`; metacaractere de LogQL → `422`.
- **Risco:** médio — fonte externa é ponto único de falha do NOC.

## `observabilidade`

- **Responsabilidade:** log estruturado, `trace_id` e métricas de rota.
- **API:** `log_action(ctx, action, outcome, duration_ms)`.
- **Invariantes:** sem corpo sensível, sem PII, sem segredo; `trace_id` sempre presente; varredura de log sem `password_hash`/`pin_hash`/`token_hash`.
- **Depende de:** nenhum módulo de negócio.
- **Testes:** captura de log em teste de contrato; varredura de segredo.
- **Risco:** baixo — mas log com PII é incidente, não bug cosmético.

## Árvore de código e de teste

```text
src/                      # planejamento; código real mora em wyd-portal/ (raiz)
├── auth/                 # autenticação de portal e MFA
├── banco/                # adapter do wyd-db
├── bff/                  # rotas de servidor (BFF)
├── admin/                # cliente do wyd-admin (C4)
├── cache/                # ISR e cache de consulta
├── monitoramento/        # clientes Prometheus/Loki/Grafana
└── observabilidade/      # log estruturado e trace_id
tests/
├── contrato/            # HTTP por rota (Jest/Vitest)
├── integracao/          # banco efêmero + adapter
├── e2e/                 # Playwright (portal e client)
└── acessibilidade/      # contraste e foco (AA)
```

*Página de planejamento. Nenhuma unidade foi implementada.*