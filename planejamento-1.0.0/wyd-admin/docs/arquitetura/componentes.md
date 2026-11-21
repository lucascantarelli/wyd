> Pergunta: "quais unidades de código o `wyd-admin` tem, qual é a API de cada uma e o que ela não pode violar?"
> Público: Desenvolvedor do `wyd-admin`
> Dono: equipe do wyd-admin
> Base: [contrato C4](contrato-c4.md); [ações](../specs/acoes.md); `src/migrate/spec.md` (padrão de módulo do `wyd-db`)
> Revisado em: 2026-11-21

**Estado:** proposto (2026-11-21). Nenhum arquivo `.rs` existe.

## `http`

- **Responsabilidade:** rotear, ler cabeçalhos, validar corpo, mapear resultado e erro para HTTP.
- **API:** `Router` com uma função por rota; `RequestContext { staff_person_id, trace_id, idempotency_key }`.
- **Invariantes:** nenhuma rota toca o banco antes de validar cabeçalho e corpo; todo erro sai no
  envelope de §3 do contrato; nenhum `SQLSTATE` cru na resposta.
- **Depende de:** `auth`, `acoes`.
- **Testes:** contrato de cada rota (200/4xx), cabeçalho ausente, corpo inválido.
- **Risco de alteração:** alto — mudar rota ou envelope quebra o BFF.

## `auth`

- **Responsabilidade:** validar token de serviço e extrair o staff; checar papel mínimo por rota.
- **API:** `authenticate(headers) -> Staff { person_id, role }`; `require_role(role, mínimo)`.
- **Invariantes:** `X-Staff-Person-Id` sem token válido é `401`; papel insuficiente é `403`; o papel
  nunca vem do corpo.
- **Depende de:** configuração de segredo (`LAC-ADM-04`).
- **Testes:** token inválido, papel `member`, persona de outro ambiente.
- **Risco:** alto — é a única barreira entre o portal e a escrita em `game`.

## `idempotencia`

- **Responsabilidade:** registrar a chave, detectar replay/conflito e devolver a resposta gravada.
- **API:** `begin(key, caller, route, body_hash)`, `complete(status, body)`, `fail(code)`.
- **Invariantes:** a chave é única por `(chamador, método, rota-modelo, chave)`; corpo diferente é
  `409`; `completed` devolve exatamente o corpo original; registro e ação commitam juntos.
- **Depende de:** `banco` e do armazenamento de `LAC-ADM-02`.
- **Testes:** repetição, concorrência de duas requisições com a mesma chave, corpo divergente.
- **Risco:** alto — sem isso, retry do portal duplica lançamento.

## `acoes`

- **Responsabilidade:** executar as sete ações de §4 do contrato, cada uma em uma transação.
- **API:** um executor por ação, todos com `execute(ctx, body) -> Response`.
- **Invariantes:** conferência otimista obrigatória em carteira e preço; `account_ledger` sempre que
  a carteira muda; auditoria na mesma transação; `DELETE` proibido.
- **Depende de:** `banco`, `idempotencia`, `server` (kick/broadcast).
- **Testes:** integração por ação, rollback forçado, concorrência.
- **Risco:** alto — é onde a regra de escrita vive.

## `banco`

- **Responsabilidade:** pool, transação, `search_path` fechado e SQL nomeado.
- **API:** `tx(|conn| …)`; consultas por nome; nenhum SQL montado por string de entrada.
- **Invariantes:** uma transação por requisição; papel próprio (`LAC-ADM-01`); sem `USAGE` em
  `portal`; timeout de 2 s.
- **Depende de:** `wyd-db` (papel e `GRANT`).
- **Testes:** papel sem `INSERT` falha; transação não vaza conexão.
- **Risco:** médio — pool mal dimensionado derruba o portal por timeout.

## `server`

- **Responsabilidade:** cliente do canal de controle do `wyd-server` (kick, broadcast).
- **API:** `kick(session_id)`, `broadcast(scope, message)`, com prazo de 1,5 s.
- **Invariantes:** sem ack não há escrita em `game.session`; timeout não é sucesso.
- **Depende de:** `LAC-ADM-03`.
- **Testes:** dublê que não responde, ack atrasado, ack duplicado.
- **Risco:** alto — contrato inexistente; nada aqui pode ser implementado antes da decisão.

## `observabilidade`

- **Responsabilidade:** log estruturado, `trace_id` e métricas de ação.
- **API:** `log_action(ctx, action, outcome, duration_ms)`.
- **Invariantes:** sem corpo sensível, sem PII, sem segredo; `trace_id` sempre presente.
- **Depende de:** nenhum módulo de negócio.
- **Testes:** captura de log em teste de contrato.
- **Risco:** baixo — mas log com PII é incidente, não bug cosmético.

## Árvore de código e de teste

```text
src/
├── main.rs          # composition root: config, pool, router
├── http/            # rotas, validação, erro
├── auth/            # token de serviço e papel
├── idempotencia/    # chave, replay, conflito
├── acoes/           # um executor por ação
├── banco/           # pool, transação, SQL
└── server/          # cliente do wyd-server
tests/
├── contrato.rs      # HTTP por rota
├── acoes.rs         # integração por ação e rollback
├── idempotencia.rs  # replay e concorrência
└── papel.rs         # 401/403 e GRANT
```
