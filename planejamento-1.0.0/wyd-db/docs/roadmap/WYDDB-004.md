# WYDDB-004 — documentar invariantes dos modelos de identidade

> Pergunta: "o que esta issue entrega e como sei que terminou?"
> Público: Desenvolvedor, QA e revisor
> Dono: equipe do `wyd-db`
> Base: `fases.md`; `wyd-db-1.0.0.md` §6.10; `D-7` aprovada em 2026-11-22
> Revisado em: 2026-11-22

**Estado:** proposto (2026-11-22)

**Gate de verificação:** comandos do `## DoD` desta página + `make gate` na raiz; o PR só fecha com os dois verdes.

**Fase:** 1 — higiene e baseline


## Contexto

`GameLogin`, `GamePassword` e `NumericPin` são tipos de borda do cliente 7662; sem invariante escrita, o construtor pode aceitar 16, 12 ou 6 bytes por acidente e o banco recebe valor fora do contrato.

## Entregável

Documentar e implementar os construtores dos três tipos com as formas de 16 B, 12 B e 6 B, `Debug` sem valor sensível e testes de limite; a página `modelos-e-invariantes.md` é a referência.

## DoD

- [ ] Teste aceita o tamanho exato e recusa um byte a menos/mais para cada tipo.
- [ ] `Debug` não imprime login, senha ou PIN.
- [ ] Nenhum valor de teste é segredo real ou PII.
- [ ] `guia-desenvolvedor/modelos-e-invariantes.md` §1 coincide com o construtor.

## Fora de escopo

Não definir hash, salt, algoritmo de senha ou formato completo do protocolo; a captura do 7662 continua fora desta issue.

## Risco

O formato do cliente pode ter exceção não capturada; a issue deve citar a fonte e bloquear se o dado não existir.

## Depende de

WYDDB-001.

## Skills

| Campo | Valor |
|---|---|
| `skill_principal` | `documentation-writer` |
| `skills_complementares` | `writing-plans` |

A lista fechada e a distinção entre skill em disco e cloud estão em
`planejamento-1.0.0/wyd-ops/docs/planos/template-de-issue.md` §3.

**Dono do valor:** esta página. O roadmap cita o link para este bloco e não repete o nome da skill.

## Evidência ao fechar

Testes unitários verdes e diff da página de modelos.
