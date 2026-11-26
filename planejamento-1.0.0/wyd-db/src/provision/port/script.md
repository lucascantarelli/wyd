> Pergunta: "como o texto gerado por `render()` chega ao cluster, e onde ele é validado?"
> Público: Desenvolvedor de `crates/wyd-db` que vai implementar `src/provision/port/script.rs`
> Dono: equipe do `wyd-db`
> Base: `src/provision/spec.md` §3, §4 e §6
> Revisado em: 2026-11-18
**Sem arquivo no repositório:** `src/provision/port/script.rs` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

**Estado:** aceito (2026-11-24) — arquivo inexistente; especificação a implementar.

# `src/provision/port/script.rs` — execução do script
**Sem arquivo no repositório:** `src/provision/port/script.rs` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

## 1. Responsabilidade

Executar **um** texto SQL multi-instrução numa sessão, sem transação explícita e sem `psql`. É a porta
que `apply` usa; `render --check` não passa por aqui (compara arquivo).

## 2. API pública
**Sem arquivo no repositório:** `db/roles.sql` — caminho planejado (ou nome do legado citado como referência); nenhum desses arquivos está versionado hoje.

```rust
pub trait Script: Send + Sync {
    /// Executa o texto inteiro; `sql` é o retorno de `render()` ou o conteúdo de `db/roles.sql`.
    async fn apply(&self, sql: &str) -> Result<(), ScriptError>;
}

#[non_exhaustive]
pub enum ScriptError {
    Connect { detail: String },                       // sem DSN
    Query { detail: String, position: Option<u32> },  // erro do servidor
}
```

## 3. Contrato

| Regra | Detalhe |
|---|---|
| texto inteiro em **uma** sessão | `CREATE ROLE` no `DO` e `GRANT` fora precisam do mesmo catálogo |
| sem transação explícita | o script usa `DO`/`GRANT` idempotentes; falha no meio deixa o que já passou aplicado, e rodar de novo converge |
| sem `psql` | nada de subprocesso; `sqlx::raw_sql` envia múltiplas instruções |
| texto vem do chamador | a porta não chama `render()`; é o runner que decide o que aplicar |
| `position` opcional | erro no meio do script aponta o offset para a mensagem |

## 4. Regras

| Regra | Por que |
|---|---|
| não valida o texto | validação de formato seria uma segunda fonte de verdade para o gerador |
| não filtra instrução | `apply` já só recebe `render()`; filtrar aqui esconderia bug de gerador |
| idempotência é do SQL gerado | a porta pode rodar o mesmo texto duas vezes com sucesso |
| erro sem DSN | mesma regra de redação do `Catalog` |
| falha parcial é recuperável | sem transação, o rerun é o caminho; a porta não tenta desfazer o que passou |

## 5. Dependências (camada)

`std`. Proibido: `sqlx`, `std::process`, `std::fs`, `std::env`, `domain`, `app`, `adapter`.

## 6. Testes

| Teste | O que prova |
|---|---|
| `fake_script_records_sql` | o runner entrega exatamente `render()` à porta |
| `apply_is_idempotent` (sistema) | rodar duas vezes não falha e o cluster fica igual |
| `error_position_is_preserved` (sistema) | script com erro sintático devolve `Query` com `position` |
| `connect_error_is_redacted` (sistema) | senha ausente do `Display` |

## 7. Pronto quando

- a porta não tem `apply_file(path)` nem `apply_render()`;
- o teste de sistema confirma que o `DO` e os `GRANT` rodaram na mesma sessão (membership aplicada).
