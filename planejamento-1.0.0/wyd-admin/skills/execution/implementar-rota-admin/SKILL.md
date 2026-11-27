---
name: implementar-rota-admin
description: Implementa rota do backoffice WYD com autenticação, autorização por papel, auditoria e resposta sem PII. Use ao criar painel, ação administrativa, relatório operacional ou ao corrigir rota que expõe dado demais.
---

# Implementar rota admin

## Quando usar

- Rota nova no `wyd-admin` ou ajuste de uma existente.
- Suspeita de dado sensível em resposta administrativa.

## Pré-requisitos

- `specs/` do admin e a página de segurança do `wyd-db` (`docs/arquitetura/seguranca.md`).
- Papel que executa a ação definido (não inventado na rota).

## Passos

1. **Autenticação primeiro**: rota sem sessão válida devolve `401`, sem tocar dado.
2. **Autorização por papel**: ação exige papel nomeado; ausência devolve `403` — nunca "esconde" a rota.
3. **Auditoria**: toda escrita registra quem, o quê, quando e o alvo, em `ops` — sem corpo sensível.
4. **Sem PII na resposta**: e-mail, IP, hash e token não saem; quando inevitável, mascare e registre o
   motivo.
5. **Idempotência** em ação repetível (chave de requisição), como no `wyd-db`.
6. **`/health` e `/ready`** continuam respondendo sem autenticação.

## Como verificar

- `401` sem sessão, `403` com papel errado, `200` com papel certo — os três casos testados.
- Log de auditoria gerado na escrita, com alvo e autor.
- `cargo test --manifest-path crates/wyd-admin/Cargo.toml` verde (não existe `Cargo.toml` de workspace na
  raiz; `cargo test -p wyd-admin` falha).

## Quando parar e perguntar

- A ação exige novo papel ou novo GRANT no banco.
- O painel precisa expor dado sensível por exigência de operação.

## Armadilhas

- **Checar papel no front** e confiar nisso.
- **Copiar DTO do portal** com campo a mais.
- **Logar payload completo** para "facilitar o suporte".
