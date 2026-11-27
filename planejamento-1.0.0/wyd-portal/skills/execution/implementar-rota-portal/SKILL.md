---
name: implementar-rota-portal
description: Implementa página ou rota do portal WYD em Next.js com sessão, DTO mínimo e chamada server-side ao banco, sem expor credencial nem PII ao navegador. Use ao criar tela de conta, personagem, mercado ou ao corrigir rota que vaza dado.
---

# Implementar rota portal

## Quando usar

- Tela, rota ou server action nova no `wyd-portal`.
- Rota existente devolve campo a mais ou chama o banco pelo cliente.

## Pré-requisitos

- Página de spec do portal e a de segurança do `wyd-db`.
- Saber qual papel do banco atende a rota (`wyd_portal`, só leitura onde couber).

## Passos

1. **Sessão no servidor**: middleware/rota valida a sessão; sem sessão, redireciona ou `401`.
2. **Acesso ao banco só no servidor**: nenhuma DSN, chave ou query no bundle do cliente.
3. **DTO explícito**: selecione coluna por coluna; `SELECT *` e repasse do objeto do banco são proibidos.
4. **Sem PII desnecessária**: e-mail, IP e hash não vão para a tela nem para o log.
5. **Mutação**: valide origem e idempotência; ação repetida não duplica efeito.
6. **Estados de erro**: carregando, vazio, erro e sem permissão têm tela — não só o caminho feliz.

## Como verificar

- `npm run build` (ou o script do projeto) sem erro e sem chave no bundle (`grep` no `.next`).
- Rota com sessão inválida devolve `401`/redirect; com sessão válida, o DTO esperado.
- Teste de componente ou de rota verde.

## Quando parar e perguntar

- A tela exige dado novo do banco (é `wyd-db`, com migration e página).
- A funcionalidade muda fronteira portal × admin.

## Armadilhas

- **`fetch` do cliente direto no banco** ou em URL interna sem autenticação.
- **Repassar o objeto do banco** para o componente.
- **Confiar em checagem feita só na UI.**
